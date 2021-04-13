import pytest

from datagears.core.network import Network
from datagears.core.nodes import GearNode
from tests.fixtures import Fixture


def test_network_construction(myfeature: Fixture[Network]) -> None:
    """Test network construction."""

    network: Network = myfeature
    graph = network.graph

    assert graph
    assert list(network.graph.nodes)
    assert list(network.graph.edges)

    plot = network.plot
    assert plot
    assert plot.meta

    assert network.roots
    for gear in network.roots:
        assert isinstance(gear, GearNode)
        assert str(gear) in {"add", "add_one"}

    assert network.input_shape == {"c1": int, "b": int, "a": int}

    expected_inputs = [
        "reduce(c1[int] = None, ...)",
        "add(a[int] = None, ...)",
        "add(b[int] = 10, ...)",
    ]
    assert network.inputs
    for input_node in network.inputs:
        assert str(input_node) in expected_inputs

    expected_outputs = [
        "my-network(my_out[ndarray] = None)",
        "my_out(reduced[int] = None, ...)",
        "reduce(sum[int] = None, ...)",
        "my_out(add_one[int] = None, ...)",
    ]
    assert network.outputs
    for output_node in network.outputs:
        assert str(output_node) in expected_outputs


def test_network_set_input(myfeature: Fixture[Network]) -> None:
    """Test network set input."""
    network: Network = myfeature

    with pytest.raises(ValueError):
        network.set_input({})

    # TODO: Fix this.
    # default_values = {"a": None, "b": 10, "c": None}
    # assert network.inputs == default_values

    new_values = {"a": 1, "b": 2, "c1": 3}
    network.set_input(new_values)
    # assert network.inputs == new_values

    for input_node in network.inputs:
        assert new_values[input_node.name] == input_node.value
