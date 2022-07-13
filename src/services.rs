use std::{cmp, hash};

use crate::{
    communications::{DGRequest, DGResponse, MLModelRequest, MLModelResponse, PyModelRequest},
    errors::*,
};
use async_trait::async_trait;
use pyo3::ToPyObject;

pub(crate) trait Service {
    fn load(&mut self) -> Result<()>;
}

#[async_trait]
pub(crate) trait PyModelService {
    type FutType;

    async fn async_process<K: 'static, V: 'static, T: 'static>(
        &'static self,
        request: DGRequest<PyModelRequest<K, V, T>>,
    ) -> Self::FutType
    where
        K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
        V: Default + ToPyObject + Send,
        T: Default + ToPyObject + Send;
}

#[async_trait]
pub(crate) trait MLModelService {
    async fn async_process(
        &self,
        request: DGRequest<MLModelRequest>,
    ) -> Result<DGResponse<MLModelResponse>>;
}
