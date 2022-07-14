extern crate criterion;

use futures_util::future::join_all;
use tokio::runtime::Builder;

use criterion::*;
use criterion::{criterion_group, criterion_main, Criterion};

use datagears::communications::{DGRequest, DGResponse, PyGearRequest, PyGearResponse};
use datagears::config::DGConfig;
use datagears::core::DataGears;
use datagears::errors::Result;

pub fn datagears_benchmark(c: &mut Criterion) {
    pyo3::prepare_freethreaded_python();

    let dg: DataGears = DataGears::new()
        .config(DGConfig::default())
        .with_pymodel(
            "pymodel",
            "/Users/sam/Learn/rust/datagears-rs/fixtures",
            "pymodel",
            "model_hook",
        )
        .build();

    c.bench_function("py_run_sync", |b| {
        b.iter(|| {
            let result: Result<DGResponse<PyGearResponse>> = dg.clone().py_run(
                "pymodel",
                DGRequest::with_body(PyGearRequest::<&str, &str, &str>::new()),
            );
            println!("{:?}", result.unwrap());
        })
    });

    c.bench_function("py_run_sync_10k", |b| {
        b.iter(|| {
            for _ in 0..10000 {
                let result: Result<DGResponse<PyGearResponse>> = dg.clone().py_run(
                    "pymodel",
                    DGRequest::with_body(PyGearRequest::<&str, &str, &str>::new()),
                );
                println!("{:?}", result.unwrap());
            }
        })
    });

    c.bench_with_input(
        BenchmarkId::new("py_run_async", "1"),
        &dg,
        |b, _dg: &DataGears| {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();

            b.to_async(rt).iter(|| async {
                let mut vec = vec![];
                let result = _dg.py_run_async(
                    "pymodel",
                    DGRequest::with_body(PyGearRequest::<&str, &str, &str>::new()),
                );
                let mut _r = result.await;
                vec.push(_r);

                join_all(vec).await;
            })
        },
    );

    c.bench_with_input(
        BenchmarkId::new("py_run_async", "5k"),
        &dg,
        |b, _dg: &DataGears| {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();

            b.to_async(rt).iter(|| async {
                for _ in 0..5000 {
                    let mut vec = vec![];
                    //for _ in 0..5 {
                    let result = _dg.py_run_async(
                        "pymodel",
                        DGRequest::with_body(PyGearRequest::<&str, &str, &str>::new()),
                    );
                    let mut _r = result.await;
                    vec.push(_r);
                    //}

                    join_all(vec).await;
                }
            })
        },
    );

    c.bench_with_input(
        BenchmarkId::new("py_run_async", "10k"),
        &dg,
        |b, _dg: &DataGears| {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();

            b.to_async(rt).iter(|| async {
                for _ in 0..10000 {
                    let mut vec = vec![];
                    //for _ in 0..5 {
                    let result = _dg.py_run_async(
                        "pymodel",
                        DGRequest::with_body(PyGearRequest::<&str, &str, &str>::new()),
                    );
                    let mut _r = result.await;
                    vec.push(_r);
                    //}

                    join_all(vec).await;
                }
            })
        },
    );
    //
}

criterion_group!(benches, datagears_benchmark);
criterion_main!(benches);
