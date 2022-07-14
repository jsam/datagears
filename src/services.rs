use std::{cmp, hash};

use crate::{
    communications::{DGRequest, DGResponse, MLGearRequest, MLGearResponse, PyGearRequest},
    errors::*,
};
use async_trait::async_trait;
use pyo3::ToPyObject;

pub(crate) trait Service {
    fn load(&mut self) -> Result<()>;
}

#[async_trait]
pub(crate) trait PyGearService {
    type FutType;

    async fn async_process<K: 'static, V: 'static, T: 'static>(
        &self,
        request: DGRequest<PyGearRequest<K, V, T>>,
    ) -> Self::FutType
    where
        K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
        V: Default + ToPyObject + Send,
        T: Default + ToPyObject + Send;
}

#[async_trait]
pub(crate) trait MLGearService {
    async fn async_process(
        &self,
        request: DGRequest<MLGearRequest>,
    ) -> Result<DGResponse<MLGearResponse>>;
}
