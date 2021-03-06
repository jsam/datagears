# datagears

[![image][]][1]

Framework for building high throughput feature, model and evaluation stores for machine learning teams. 
---

To avoid duplicating work and foster better collaboration inside ML teams and between data scientists and operations, we decided to build a scalable data-flow based feature builders which are backed by high troughput, centralized store which can be easily horizontally scaled - both in computation and storage, and easly reused in inference phases on deployed models. To achieve those goals we utilized the existing technologies and design principles from the Python ecosystem, so the ramp-up period, from data exploration to production, for ML engineers should be much faster. In our design we considered both perspectives - data scientists and devops, to enable ease of use during data exploration and model building phase and provide needed tools to productionize their results.

# Goals

* Build versioned and scalable ML features reusable in data explorations and production inferences

* Easily share prebuilt features in a team

* Enable easy continous deployment and evaluation of your models

* Enable reproducible evaluation stores for deployed models

* Track and monitoring models performance and degradation



# Getting started

```python
from datagears import Depends, Feature


def add(a, b) -> int:
    return a + b


def reduce(c: int, sum: int = Depends(add)) -> int:
    return sum - c


def my_out(reduced: int = Depends(reduce)) -> float:
    return reduced / 2


my_graph = Feature(name="mynet", outputs=[my_out]) 
my_graph.plot.view()
```


Which should produce following computational graph:

<p align="center">
    <img src="out.png" />
</p>


To inspect the `input_shape` we can check with:

```python
network.input_shape
> {'c': int, 'a': typing.Any, 'b': typing.Any}
```

To execute our newly composed computation, we can execute it with given parameters:
```python
my_graph.run(a=5, b=3, c=4, d=6)
```

Or register the graph with the backend and execute it from other
processes: 
```python
my_graph.register()
```

  [image]: https://badge.fury.io/py/datagears.png
  [1]: http://badge.fury.io/py/datagears
