use pyo3::{types::PyDict, ToPyObject};
use smallvec::SmallVec;
use std::{cmp, collections::HashMap, hash, sync::Arc};

use tract_core::prelude::*;

pub enum Types {
    PyModel,
    MLModel,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct DGRequest<T> {
    pub body: T,
}

impl<T> DGRequest<T> {
    pub fn with_body(body: T) -> Self {
        DGRequest { body }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct DGResponse<T> {
    pub body: T,
}

impl<T> DGResponse<T> {
    pub fn with_body(body: T) -> Self {
        DGResponse { body }
    }
}

pub(crate) trait DGRequestBase<T> {}
pub(crate) trait DGResponseBase<T> {}

impl<T> DGRequestBase<T> for T {}
impl<T> DGResponseBase<T> for T {}

#[derive(Default)]
pub struct PyModelRequest<K, V, T>
where
    K: hash::Hash + cmp::Eq + Default + ToPyObject,
    V: Default + ToPyObject,
    T: Default + ToPyObject,
{
    pub args: HashMap<K, V>,
    pub kwargs: HashMap<&'static str, T>,
}

impl<K, V, T> PyModelRequest<K, V, T>
where
    K: hash::Hash + cmp::Eq + Default + ToPyObject,
    V: Default + ToPyObject,
    T: Default + ToPyObject,
{
    pub fn new() -> Self {
        PyModelRequest {
            ..Default::default()
        }
    }

    pub fn with_args(mut self, args: HashMap<K, V>) -> Self {
        self.args = args;
        self
    }

    pub fn with_kwargs(mut self, kwargs: HashMap<&'static str, T>) -> Self {
        self.kwargs = kwargs;
        self
    }
}

pub struct PyModelResponse {
    response: Option<PyDict>,
}

impl PyModelResponse {
    pub fn new() -> Self {
        PyModelResponse { response: None }
    }
}

#[derive(Default, Debug)]
pub struct MLModelRequest {
    pub input: Tensor,
}

impl MLModelRequest {
    pub fn new() -> Self {
        MLModelRequest {
            ..Default::default()
        }
    }

    pub fn body(mut self, input: Tensor) -> Self {
        self.input = input;
        self
    }
}

#[derive(Default, Debug)]
pub struct MLModelResponse {
    pub output: SmallVec<[Arc<Tensor>; 4]>,
}

impl MLModelResponse {
    pub fn new() -> Self {
        MLModelResponse {
            ..Default::default()
        }
    }

    pub fn with_output(mut self, output: SmallVec<[Arc<Tensor>; 4]>) -> Self {
        self.output = output;
        self
    }
}
