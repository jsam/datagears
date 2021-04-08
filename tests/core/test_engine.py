from test import add

from datagears.core.engine import LocalEngine
from datagears.core.network import Network


def test_local_engine():
    """Test local engine."""
    engine = LocalEngine(Network("my-net", outputs=[add]))
    engine.prepare()

    future = engine._executor.submit(add, 2, 3)

    assert future
