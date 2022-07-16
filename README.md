# datagears

Is a runtime for native execution of machine-learning models and data-processing using Python. The goal of the project is to provide tools needed to speed up the development and optimization of machine-learning pipelines.

It's designed to run with a low memory footprint and latency, while providing all the benefits of using computational graphs and lambda functions.

# Getting started

Implement a data gear:

```python
def run(dg: "DataGears"):
    """"""
    iter = dg.iter("my-iter").read(10)  # Stateful iterators: Get 10 items from the last known state of `my-iter`
    iter = dg.iter().last(10) # Stateless iterator: Get last 10 items
    iter = dg.iter().last()  # Stateless iterator: Get last item
    log = dg.logging()  # Add a logger for the current execution.
    exec_id = dg.execution_id()  # Get the execution identifier for this run of the gear.
    
    try:
        result = df.map(lambda record: model_hook(record))
    except Exception as e:
        dg.error(e)
    
    log.info("reading 10 records from the stream")
    df = iter.read(10)
    
    fruits = df.sort("fruits").select(["fruits"])
    dg.info("writing result")
    dg.write("only-fruits", fruits)
```

Deploy a data gear and run it:
```rust
let dg: DataGears = DataGears::new()
    .config(DGConfig::default())
    .with_py_gear(
        "pygear",
        "/Users/sam/rust/datagears-rs/gears",
        "pygear",
        "model_hook",
    )
    .build();

let result: Result<DGResponse<PyGearResponse>> = dg.clone().py_run(
    "pygear",
    DGRequest::with_body(PyGearRequest::<&str, &str, &str>::new()),
);
println!("{:?}", result.unwrap());
```

Deploy a native ONNX model:
```rust
let dg: DataGears = DataGears::new()
    .config(DGConfig::default())
    .with_onnx_gear(
        "onnx_model",
        "/Users/sam/rust/datagears-rs/gears",
    )
    .build();

let result: Result<DGResponse<MLGearResponse>> = dg.clone().onnx_run(
    "onnx_model",
    DGRequest::with_body(MLGearRequest::<&str, &str, &str>::new()),
);

println!("{:?}", result.unwrap());
```

## Installing dependencies

```
poetry install
```

#### Install `scipy`

```
brew install openblas gfortran lapack
export OPENBLAS=/opt/homebrew/opt/openblas/lib
```

If in trouble, check `scripts/scipy-install.sh`.
