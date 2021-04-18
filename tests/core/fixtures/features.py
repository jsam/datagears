import pytest

from datagears.core.network import Network


@pytest.fixture
def myfeature() -> Network:
    """Testing fixture for a feature."""
    from datagears.core.network import Network
    from datagears.features.dummy import my_out

    network = Network("my-network", outputs=[my_out])
    return network
