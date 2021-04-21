import os
import time
import uuid

import numpy
import redisai as rai

from datagears.core.api import FeatureStoreAPI, NetworkAPI


class FeatureStore(FeatureStoreAPI):
    """Feature store."""

    def __init__(self) -> None:
        """Feature store constructor."""
        server: str = os.getenv("REDISAI_URI", "0.0.0.0:6379")
        host, port = server.split(":")
        self._conn = rai.Client(host=host, port=int(port))  # type: ignore

        super().__init__()

    def set(self, tensor: numpy.ndarray, network: NetworkAPI) -> str:
        """Store tensor to the feature store."""
        timestamp: str = str(int(time.time() * 1e3))  # NOTE: Milliseconds
        key: str = f"{network.identifier}-{timestamp}-{uuid.uuid4().hex}"
        self._conn.tensorset(key, tensor)  # type: ignore
        return key

    def get(self, key: str) -> numpy.ndarray:
        """Get tensor from the store."""
        return self._conn.tensorget(key, as_numpy=True)  # type: ignore
