import numpy
import redisai as rai

from datagears.core.api import FeatureStoreAPI, NetworkAPI


class FeatureStore(FeatureStoreAPI):
    """Feature store."""

    def __init__(self) -> None:
        """Feature store constructor."""
        self._conn = rai.Client(host="0.0.0.0", port=6379)  # type: ignore

        super().__init__()

    def set(self, tensor: numpy.ndarray, network: NetworkAPI) -> str:
        """Store tensor to the store."""
        key: str = f"{network.tag}-{}"
        return key

    def get(self, key: str) -> numpy.ndarray:
        """Get tensor from the store."""
        return numpy.array([2])
