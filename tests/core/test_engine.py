from datagears.core.engine import LocalEngine
from datagears.core.network import Network
from tests.fixtures import Fixture


def test_local_engine(myfeature: Fixture[Network]) -> None:
    """Test local engine."""
    mynet: Network = myfeature
    engine = LocalEngine()

    new_net = engine.execute(mynet, a=5, b=20, c1=30)

    assert new_net
    assert new_net.outputs
    for output_node in new_net.outputs:
        assert output_node.value is not None
