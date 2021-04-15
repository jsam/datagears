import pytest

from datagears.core.engine import PoolEngine, SerialEngine
from datagears.core.network import Network
from datagears.core.nodes import GearNode, InvalidGraph, OutputNode
from tests.fixtures import Fixture


class TestSerialEngine:
    """Check all aspects of SerialEngine implementation."""

    def test_construction(self, myfeature: Fixture[Network]) -> None:
        """Test serial engine."""
        mynet: Network = myfeature
        engine = SerialEngine()

        assert engine.is_ready()
        engine.setup()
        assert engine.is_ready()

        new_net = engine.execute(mynet, a=5, b=20, c1=30)
        engine.teardown()

        assert new_net is not None
        assert new_net.outputs
        for output_node in new_net.outputs:
            assert output_node.value is not None

    def test_partial_construction(self, myfeature: Fixture[Network]) -> None:
        """Test SerialEngine partial construction."""
        mynet: Network = myfeature
        engine = SerialEngine()

        with pytest.raises(ValueError):
            _ = engine.execute(None, a=3, b=2, c=10)  # type: ignore

        dst: OutputNode = mynet.outputs[-1]
        another_gear = GearNode(lambda x: x ** 2)
        mynet.graph.add_edge(another_gear, dst)  # type: ignore

        with pytest.raises(InvalidGraph) as exp:
            _ = engine.execute(mynet, a=5, b=20, c1=30)

        assert exp.value.msg == "found a data node produced by multiple gears: [add_one, <lambda>]"
        assert "gears" in exp.value.params.keys()

    def test_submit_next_execution(self, myfeature: Fixture[Network]) -> None:
        """Check execution sequence."""
        mynet: Network = myfeature.copy()
        engine = SerialEngine()

        mynet.set_input({"a": 1, "b": 3, "c1": 10})

        engine._network = mynet  # type: ignore
        result = engine._submit_next()  # type: ignore
        assert result is True

        engine._network = None  # type: ignore
        with pytest.raises(ValueError):
            engine._submit_next()  # type: ignore


class TestPoolEngine:
    """Check all aspects of PoolEngine implementation."""

    def test_construction(self, myfeature: Fixture[Network]) -> None:
        """Test serial engine."""
        mynet: Network = myfeature
        engine = PoolEngine()

        assert engine.is_ready() is False
        engine.setup()
        assert engine.is_ready() is True

        new_net = engine.execute(mynet, a=5, b=20, c1=30)
        engine.teardown()

        assert new_net is not None
        assert new_net.outputs
        for output_node in new_net.outputs:
            assert output_node.value is not None

    def test_partial_construction(self, myfeature: Fixture[Network]) -> None:
        """Test SerialEngine partial construction."""
        mynet: Network = myfeature
        engine = PoolEngine()
        engine.setup()

        with pytest.raises(ValueError):
            _ = engine.execute(None, a=3, b=2, c=10)  # type: ignore

        dst: OutputNode = mynet.outputs[-1]
        another_gear = GearNode(lambda x: x ** 2)
        mynet.graph.add_edge(another_gear, dst)  # type: ignore

        with pytest.raises(InvalidGraph) as exp:
            _ = engine.execute(mynet, a=5, b=20, c1=30)

        assert exp.value.msg == "found a data node produced by multiple gears: [add_one, <lambda>]"
        assert "gears" in exp.value.params.keys()

    def test_submit_next_execution(self, myfeature: Fixture[Network]) -> None:
        """Check execution sequence."""
        mynet: Network = myfeature.copy()
        engine = PoolEngine()
        engine.setup()

        mynet.set_input({"a": 1, "b": 3, "c1": 10})

        engine._network = mynet  # type: ignore
        result = engine._submit_next()  # type: ignore
        assert set(result)

        engine._executor = None  # type: ignore
        with pytest.raises(ValueError):
            engine._submit_next()  # type: ignore

        engine._network = None  # type: ignore
        with pytest.raises(ValueError):
            engine._submit_next()  # type: ignore

    def test_teardown(self) -> None:
        """Check teardown step."""
        engine = PoolEngine()

        with pytest.raises(NotImplementedError):
            engine.register()

        with pytest.raises(ValueError):
            engine.teardown()

        engine.setup()
        engine.teardown()
