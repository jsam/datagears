import networkx


class NetworkPlot:
    """Network plotting utility."""

    def __init__(self, graph: networkx.DiGraph) -> None:
        """Network plot constructor."""
        import pydot

        self._graph: networkx.DiGraph = graph

        g = pydot.Dot(graph_type="digraph", rank="same")

        for nx_node in self._graph.nodes:
            node = pydot.Node(
                name=nx_node.name_uniq, label=str(nx_node), shape=nx_node.shape
            )
            g.add_node(node)

        for src, dst, param in self._graph.edges(data=True):
            edge = pydot.Edge(src=src.name_uniq, dst=dst.name_uniq)
            g.add_edge(edge)

        self._pydot_graph = g

    @property
    def meta(self):
        """Return metadata of network plot."""
        return self._pydot_graph.obj_dict

    @property
    def show(self):
        """Render pydot for viewing in Jupyter notebook."""
        from IPython.display import Image, display

        _png = self._pydot_graph.create_png()
        display(Image(_png))

    def to_file(self, filename):
        """Write plot to a file."""
        self._pydot_graph.write_png(filename)
