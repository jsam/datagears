use pyo3::{FromPyObject, ToPyObject};
use smallvec::SmallVec;
use std::{cmp, collections::HashMap, hash, sync::Arc};
use tract_core::prelude::*;

pub enum Types {
    PyGear,
    MLGear,
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

#[derive(Default, Debug)]
pub struct PyGearRequest<K, V, T>
where
    K: hash::Hash + cmp::Eq + Default + ToPyObject,
    V: Default + ToPyObject,
    T: Default + ToPyObject,
{
    pub args: HashMap<K, V>,
    pub kwargs: HashMap<&'static str, T>,
}

impl<K, V, T> PyGearRequest<K, V, T>
where
    K: hash::Hash + cmp::Eq + Default + ToPyObject,
    V: Default + ToPyObject,
    T: Default + ToPyObject,
{
    pub fn new() -> Self {
        PyGearRequest {
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

#[derive(Default, Debug, FromPyObject)]
pub struct PyGearResponse {
    #[pyo3(item("response"))]
    response: String,
    #[pyo3(item("error"))]
    error: String,
}

impl PyGearResponse {
    pub fn new() -> Self {
        PyGearResponse {
            ..Default::default()
        }
    }
}

#[derive(Default, Debug)]
pub struct MLGearRequest {
    pub input: Tensor,
}

impl MLGearRequest {
    pub fn new() -> Self {
        MLGearRequest {
            ..Default::default()
        }
    }

    pub fn body(mut self, input: Tensor) -> Self {
        self.input = input;
        self
    }
}

#[derive(Default, Debug)]
pub struct MLGearResponse {
    pub output: SmallVec<[Arc<Tensor>; 4]>,
}

impl MLGearResponse {
    pub fn new() -> Self {
        MLGearResponse {
            ..Default::default()
        }
    }

    pub fn with_output(mut self, output: SmallVec<[Arc<Tensor>; 4]>) -> Self {
        self.output = output;
        self
    }
}
