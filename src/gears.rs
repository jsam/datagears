use std::{cmp, fs, hash, path::PathBuf, sync::mpsc, thread};

use async_trait::async_trait;
use futures_util::future::FutureObj;
use pyo3::{
    types::{IntoPyDict, PyList, PyModule, PyString, PyTuple},
    Python, ToPyObject,
};

use crate::{
    communications::{DGRequest, DGResponse, PyGearRequest, PyGearResponse},
    config::DGConfig,
    errors::{DGError, Result},
    services::{PyGearService, Service},
};

#[derive(Debug, Default, Clone)]
pub struct PyGear {
    pub name: &'static str,
    pub module_path: PathBuf,
    pub module: &'static str,
    pub requester_hook: &'static str,
    config: DGConfig,
}

impl PyGear {
    pub fn new(config: DGConfig) -> Self {
        PyGear {
            config,
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }

    pub fn with_module_path(mut self, module_path: PathBuf) -> Self {
        self.module_path = module_path;
        self
    }

    pub fn with_module(mut self, module: &'static str) -> Self {
        self.module = module;
        self
    }

    pub fn with_requester_hook(mut self, requester_hook: &'static str) -> Self {
        self.requester_hook = requester_hook;
        self
    }

    pub fn process<K: 'static, V: 'static, T: 'static>(
        &mut self,
        request: DGRequest<PyGearRequest<K, V, T>>,
    ) -> Result<DGResponse<PyGearResponse>>
    where
        K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
        V: Default + ToPyObject + Send,
        T: Default + ToPyObject + Send,
    {
        let mut module_path = self.module_path.clone();
        let syspath_module_path = self
            .module_path
            .clone()
            .into_os_string()
            .into_string()
            .unwrap();
        let module = format!("{}.py", self.module);

        module_path.push(module);

        let module_file = module_path.clone().into_os_string().into_string().unwrap();
        println!("{:?}", module_file);
        // TODO: Please, fix this!
        let source = fs::read_to_string(module_file.as_str()).unwrap();

        // TODO: This will spawn interpreter per process.
        // NOTE: Both, this and above fs::read can be optimized away by keeping interpreters warm.
        // NOTE: Since, overhead of having an interpreter is already quite high we will not optimize this for now.
        let gilblock = Python::acquire_gil();
        let py = gilblock.python();

        // TODO: Screams for cleaner impl.
        let sys = PyModule::import(py, "sys").unwrap();
        let sys_any = sys.getattr("path").unwrap();
        let syspath = sys_any.downcast::<PyList>().unwrap();

        let syspath_entry = syspath
            .get_item(0)
            .unwrap()
            .downcast::<PyString>()
            .unwrap()
            .to_string_lossy();

        if syspath_entry.as_ref() != syspath_module_path {
            syspath.insert(0, syspath_module_path).unwrap();
        }

        let datamod = PyModule::from_code(py, source.as_str(), self.name, self.name)
            .map_err(|e| {
                e.print(py);
                let err_msg: String = format!(
                    "Import failed in {}\n\
                \twith traceback",
                    self.requester_hook
                );
                DGError::PyModuleError(err_msg.to_owned())
            })
            .unwrap();
        println!("SYS PATH => \n{:?}", syspath);

        let args_data = request.body.args.into_py_dict(py);
        let args = PyTuple::new(py, &[args_data]);
        let kwargs = request.body.kwargs.into_py_dict(py);

        let py_result = datamod
            .call_method(self.requester_hook, args, Some(kwargs))
            .map_err(|e| {
                // e.print(py);
                let err_msg: String = format!(
                    "Call failed over {:?}\n\
            \twith traceback",
                    self.requester_hook
                );
                DGError::PyModuleError(err_msg.to_owned())
            })
            .unwrap();

        let response: PyGearResponse = py_result.extract()?;
        Ok(DGResponse { body: response })
    }
}

impl Service for PyGear {
    fn load(&mut self) -> Result<()> {
        if !self.module_path.exists() {
            let _err = format!(
                "module doesn't exists: {}",
                self.module_path.to_str().unwrap()
            );
            return Err(DGError::PyModuleError(_err));
        }
        return Ok(());
    }
}

#[async_trait]
impl PyGearService for PyGear {
    type FutType = FutureObj<'static, Result<DGResponse<PyGearResponse>>>;

    async fn async_process<K: 'static, V: 'static, T: 'static>(
        &self,
        request: DGRequest<PyGearRequest<K, V, T>>,
    ) -> Self::FutType
    where
        K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
        V: Default + ToPyObject + Send,
        T: Default + ToPyObject + Send,
    {
        let mut _clone = self.clone();

        FutureObj::new(Box::new(async move {
            let (tx, rx) = mpsc::channel();
            let _ = thread::spawn(move || {
                let resp = _clone.process::<K, V, T>(request);

                let _ = tx.send(resp);
            });

            let result = rx.recv().unwrap();
            result
        }))
    }
}
