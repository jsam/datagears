from concurrent.futures import ProcessPoolExecutor, as_completed

from datagears.core.api import EngineAPI, NetworkAPI


class LocalEngine(EngineAPI):
    """Local engine executor."""

    def __init__(self, compute: NetworkAPI, max_workers=4) -> None:
        """Local engine constructor."""
        from datagears.engine.network import Gear, Network

        if isinstance(compute, Gear):
            compute = Network(compute.name, outputs=[compute])

        self._network: Network = compute
        self._max_workers: int = max_workers
        self._executor: ProcessPoolExecutor = None

    def _submit_next(self) -> dict:
        """Submit next batch of jobs to the pool."""
        results = {}
        futures = {}

        for data_node in self._network._compute_next():
            predeccesors = list(self._network.graph.predecessors(data_node))
            if len(predeccesors) != 1:
                raise NotImplementedError(
                    "found a compute node with multiple predecessors"
                )

            gear = predeccesors[0]
            future = self._executor.submit(gear, kwargs=gear.input_values)
            futures[future] = (data_node, gear)

        for future in as_completed(futures):
            data_node, gear = futures[future]
            value = future.result()

            data_node.set_value(value)
            results[gear.name] = value

        return results

    def is_ready(self):
        """Check if engine is ready for computation."""
        return self._executor != None

    def prepare(self) -> None:
        """Prepare the given computation for executor."""
        self._executor = ProcessPoolExecutor(max_workers=self._max_workers)

    def run(
        self, on_network: NetworkAPI = None, output_all: bool = False, **kwargs
    ) -> dict:
        """Runs the computational network and returns the result object."""
        if on_network:
            self._network = on_network

        self._network._set_input(kwargs)

        results = {}
        while True:
            last_results = self._submit_next()
            if not last_results:
                break

            results.update(last_results)

        if output_all:
            return results

        results_filter = {
            fn.__name__: results[fn.__name__] for fn in self._network._outputting_nodes
        }
        return results_filter

    def register(self):
        """Registers the computational network with external executor."""
        raise NotImplementedError

    def teardown(self):
        """Cleanup phase."""
        self._executor.shutdown(wait=True)

