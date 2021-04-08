import inspect
from typing import Callable, Dict, Type


class Signature:
    """Analyze function signature."""

    def __init__(self, func: Callable) -> None:
        """Signature constructor."""
        self._func = func
        self._name = func.__name__
        self._signature = inspect.signature(func)
        self._params = dict(self._signature.parameters)
        self._return_type = self._signature.return_annotation

    @property
    def name(self) -> str:
        """Returns the name of the wrapped object."""
        return self._name

    @property
    def output_type(self) -> Type:
        """Get output type."""
        return self._return_type

    @property
    def params(self) -> Dict:
        """Get all function input parameters."""
        return self._params
