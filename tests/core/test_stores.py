from numpy import array, ndarray

from datagears.core.network import Network


class TestFeatureStore:
    def test_feature_store(self, myfeature: Network) -> None:
        """Test feature store get and set."""
        from datagears.core.stores import FeatureStore

        store = FeatureStore()

        tensor: ndarray = array([1, 2, 3])
        key: str = store.set(tensor, myfeature)

        assert key
        assert len(key.split("-")) == 3

        assert all(store.get(key) == tensor)  # type: ignore
