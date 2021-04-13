from typing import Any, Dict, List, Optional

from datagears.core.api import EngineAPI, NetworkAPI
from datagears.core.nodes import GearNode, OutputNode


class LocalEngine(EngineAPI):
    """Local engine executor."""

    def __init__(self) -> None:
        """Local engine constructor."""
        self._network: Optional[NetworkAPI] = None

    def _submit_next(self) -> bool:
        """Submit next batch of jobs to the pool."""
        if self._network is None:
            raise ValueError

        computed: Dict[GearNode, Any] = {}

        data_node: OutputNode
        for data_node in self._network.compute_next():

            predeccesors: List[GearNode] = list(self._network.graph.predecessors(data_node))  # type: ignore
            if len(predeccesors) != 1:
                raise NotImplementedError("found a compute node with multiple predecessors")

            gear = predeccesors[0]
            result = gear(gear.input_values)

            computed[gear] = result
            data_node.set_value(result)

        return bool(computed)

    def setup(self) -> None:
        """Prepare the given computation for executor."""
        pass

    def teardown(self) -> None:
        """Engine cleanup phase."""
        pass

    def is_ready(self) -> bool:
        """Check if engine is ready for computation."""
        return True

    def execute(self, network: NetworkAPI, **kwargs: Any) -> NetworkAPI:
        """Runs the computational network and returns the result object."""
        if network is None:
            raise ValueError("cannot execute empty network")

        self._network = network
        self._network.set_input(kwargs)

        while self._submit_next():
            pass

        return self._network


# class DaskEngine(EngineAPI):
#     """Dask engine executor."""

#     def __init__(self) -> None:
#         """Local engine constructor."""
#         from dask.distributed import Client

#         self._executor: Optional[Client] = None
#         self._network: Optional[NetworkAPI] = None

#     def _submit_next(self) -> bool:
#         """Submit next batch of jobs to the pool."""
#         if self._network is None:
#             raise ValueError("network not found")

#         if self._executor is None:
#             raise ValueError("engine not found")

#         futures = {}
#         gear: GearNode
#         data_node: OutputNode

#         for data_node in self._network.compute_next():

#             predeccesors: List[GearNode] = list(
#                 self._network.graph.predecessors(data_node)
#             )
#             if len(predeccesors) != 1:
#                 raise NotImplementedError(
#                     "found a compute node with multiple predecessors"
#                 )

#             gear = predeccesors[0]
#             data_node.set_value(gear(gear.input_values))

#             future = self._executor.submit(gear, kwargs=gear.input_values)  # type: ignore
#             futures[future] = (data_node, gear)

#         if not futures:
#             return False

#         for future in as_completed(futures):  # type: ignore
#             data_node, gear = futures[future]  # type: ignore
#             data_node.set_value(future.result())  # type: ignore

#         return True

#     def setup(self) -> None:
#         """Prepare the given computation for executor."""
#         from dask.distributed import Client

#         self._executor = Client("0.0.0.0:8786")

#     def is_ready(self) -> bool:
#         """Check if engine is ready for computation."""
#         return self._executor != None

#     def execute(self, network: NetworkAPI, **kwargs: Dict[str, Any]) -> NetworkAPI:
#         """Runs the computational network and returns the result object."""
#         if network is None:
#             raise ValueError("cannot execute empty network")

#         if self._executor is None:
#             raise ValueError

#         def _install_deps():
#             import os

#             os.system("pip install networkx numpy")

#         self._executor.upload_file("/Users/sam/Datagears/datagears/dist/datagears-0.1.0-py3.8.egg")  # type: ignore
#         self._executor.run(_install_deps)

#         self._network = network
#         self._network.set_input(kwargs)

#         while self._submit_next():
#             pass

#         return self._network

#     def teardown(self):
#         """Enging cleanup phase."""
#         if self._executor is None:
#             raise ValueError("engine not running")

#         self._executor.shutdown()
