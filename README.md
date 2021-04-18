# datagears

[![image][]][1]

Large-scale feature builder backed by high throughput feature store.
---

To avoid duplicating work and foster better collaboration inside ML teams and between data scientist and operations, we decided to build a scalable feature builder which is backed by high troughput, centralized feature store which can easily horizontally scale its computational and storage resources. To achieve those goals we utilized the existing technologies and design principles from the Python ecosystem, so the ramp-up period (from data exploration to production) for data scientists should be much faster. In our design we considered both perspectives - Data Scientists and DevOps, to enable ease of use during data exploration and model building phase and provide needed tools to productionize their results.

# Goals

* Build reproducible, reusable and versioned features using lightweight computation graph

* Easily share prebuilt features in a team

* Enable easy continous deployment of your models

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
