import pytest
import cloudpickle

from datagears.core.network import Network

from . import *


def test_network_construction():
    """Test network construction."""
    network = Network("my-network", outputs=[my_out])
    plot = network.plot
    assert plot
    assert network

    assert all([True for n in network.graph.nodes if n])


def test_network_set_input():
    """Test network set input."""
    network = Network("my-network", outputs=[my_out])

    with pytest.raises(ValueError):
        network._set_input({})

    default_values = {"a": None, "b": 10, "c": None}
    assert network.inputs == default_values

    new_values = {"a": 1, "b": 2, "c": 3}
    network._set_input(new_values)
    assert network.inputs == new_values


def test_network_serialization():
    """Serialize network for remote execution."""
    network = Network("my-network", outputs=[my_out])

    result = network.run(a=4, b=10, c=3).result
    assert result == {"my_out": 5.5}

    engine = network._engine
    network._engine = None
    pickled_network = cloudpickle.dumps(network)
    assert pickled_network

    import pickle
    original = pickle.loads(pickled_network)
    original._engine = engine
    result = original.run(a=4, b=10, c=3).result
    assert result == {"my_out": 5.5}


