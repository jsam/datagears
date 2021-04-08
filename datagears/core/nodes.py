from dataclasses import dataclass
from typing import Any, Callable, Optional, Type

from networkx.classes.multidigraph import MultiDiGraph

from datagears.core.analysis import Signature


class GearException(Exception):
    """Gear exception."""

    def __init__(self, **kwargs: Optional[Any]) -> None:
        """"Gear exception constructor."""
        self.gear = kwargs["gear"]
        self.params = kwargs["params"]
        self.raised_exception = kwargs["raised_exception"]

        super().__init__(self.raised_exception)


class Gear(Signature):
    """Node representing data transformation."""

    shape = "circle"

    def __init__(self, func: Callable, graph: MultiDiGraph = None) -> None:
        """Gear constructor."""
        self._graph: MultiDiGraph = graph
        super().__init__(func)

    def __call__(self, *args: Any, **kwds: Any) -> Any:
        """Execute the given callable with in going nodes as parameters."""
        params = self.input_values

        try:
            result = self._func(**params)
        except (Exception, BaseException) as e:
            raise GearException(gear=self, params=params, raised_exception=e)

        return result

    def __repr__(self) -> str:
        """String representation of a gear."""
        return self.name

    @property
    def name_uniq(self) -> str:
        return f"gear_{self._name}"

    @property
    def input_values(self) -> dict:
        """Input values for the gear computation."""
        params = {p.name: p.value for p in self._graph.predecessors(self)}
        return params

    def set_graph(self, graph):
        """Associate gear with a graph."""
        self._graph = graph


class Data:
    """Common operations for data nodes."""

    def __init__(
        self,
        name: str,
        value: Optional[Any],
        annotation: Type = Any,
        graph: MultiDiGraph = None,
    ):
        """Gear input constructor."""
        self._name: str = name
        self._value: Optional[Any] = value
        self._annotation: Type = annotation
        self._graph: MultiDiGraph = graph

    def __repr__(self) -> str:
        """String representation."""
        suffix = ")"
        annotation = self._annotation

        if hasattr(self._annotation, "__name__"):
            annotation = self._annotation.__name__

        successors = list(self._graph.successors(self))
        if not successors:
            child_out = self._graph.name
        else:
            child = successors[0]
            child_out = child.name
            if len(child.params) > 1:
                suffix = ", ...)"

        return f"{child_out}({self._name}[{annotation}] = {self._value}{suffix}"

    @property
    def name(self) -> str:
        """Node name."""
        return self._name

    @property
    def name_uniq(self) -> str:
        return f"data_{self._name}"

    @property
    def value(self) -> Any:
        """Returns wrapped data."""
        return self._value

    @property
    def annotation(self) -> str:
        """Node annotation."""
        return self._annotation

    @property
    def is_empty(self) -> bool:
        """Check if the node is empty."""
        return self._value is None

    def set_value(self, value) -> None:
        """Sets node value."""
        self._value = value


class GearInput(Data):
    """Input to the gear."""

    shape = "invhouse"


class GearOutput(Data):
    """Output of a gear without additional depedency."""

    shape = "house"


class GearInputOutput(Data):
    """Gear input and output node."""

    shape = "note"
