use std::{cmp, collections::HashMap, fmt::Display, hash, path::PathBuf, sync::Arc};

use futures_util::future::FutureObj;
use pyo3::ToPyObject;

use crate::{
    communications::{DGRequest, DGResponse, PyGearRequest, PyGearResponse},
    config::DGConfig,
    errors::{DGError, Result},
    gears::PyGear,
    services::{PyGearService, Service},
};
/* API Usage:

let dg = Datagears::new()
    .config(&config)
    .with_pymodel("name1", "./pymodels/name1/main.py", "./pymodels/name1", "run")
    .with_pymodel("name1", "./pymodels/name1/main.py", "./pymodels/name1", "run")
    .with_pymodel("name1", "./pymodels/name1/main.py", "./pymodels/name1", "run")
    .with_pymodel("name1", "./pymodels/name1/main.py", "./pymodels/name1", "run")
    .build()


let result_py: Result<PyObject> = dg.py_run_async("name1", args, kwargs);
let result_onnx: Result<Tensor> = dg.onnx_run("name1", tensor);

 */

#[derive(Debug, Clone)]
pub struct DataGears {
    config: DGConfig,
    py_services: HashMap<String, PyGear>,
    //ml_services: HashMap<String, MLModel>,
}

impl Display for DataGears {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataGears()")
    }
}

impl Default for DataGears {
    fn default() -> Self {
        Self {
            config: DGConfig::default(),
            py_services: Default::default(),
        }
    }
}

impl DataGears {
    pub fn new() -> Self {
        DataGears {
            ..Default::default()
        }
    }

    pub fn config(mut self, config: DGConfig) -> Self {
        self.config = config;
        self
    }

    pub fn build(mut self) -> Self {
        for (name, service) in &mut self.py_services {
            println!("loading python model: {}", name);
            let _ = service.load().unwrap(); // TODO: Build better handling support.
        }

        self
    }

    pub fn frozen_state(&self) -> DataGears {
        self.clone()
    }

    pub fn shareable(self) -> Arc<Self> {
        Arc::new(self.build())
    }

    pub fn with_pymodel(
        mut self,
        name: &'static str,
        module_path: &'static str,
        module: &'static str,
        hook: &'static str,
    ) -> Self {
        // TODO: Cloning the config object should be avoided here.
        let spec = PyGear::new(self.config.clone())
            .with_name(name)
            .with_module_path(PathBuf::from(module_path))
            .with_module(module)
            .with_requester_hook(hook);

        let name = name.to_string();
        self.py_services.insert(name, spec);

        self
    }

    pub fn py_run<K: 'static + Send, V: 'static + Send, T: 'static + Send>(
        mut self,
        name: &str,
        request: DGRequest<PyGearRequest<K, V, T>>,
    ) -> Result<DGResponse<PyGearResponse>>
    where
        K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
        V: Default + ToPyObject + Send,
        T: Default + ToPyObject + Send,
    {
        if let Some(modelbox) = self.py_services.get_mut(name) {
            modelbox.process::<K, V, T>(request)
        } else {
            Err(DGError::ModelNotFound("model not found".to_string()))
        }
    }

    pub async fn py_run_async<K: 'static + Send, V: 'static + Send, T: 'static + Send>(
        &self,
        name: &str,
        request: DGRequest<PyGearRequest<K, V, T>>,
    ) -> FutureObj<Result<DGResponse<PyGearResponse>>>
    where
        K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
        V: Default + ToPyObject + Send,
        T: Default + ToPyObject + Send,
    {
        if let Some(modelbox) = self.py_services.get(name) {
            modelbox.async_process(request).await
        } else {
            FutureObj::new(Box::new(async move {
                let err = DGError::ModelNotFound("wtf".to_string());
                Err(err)
            }))
        }
    }
}
