use std::{cmp, fs, hash, path::PathBuf};

use pyo3::{
    types::{IntoPyDict, PyList, PyModule, PyString, PyTuple},
    PyObject, Python, ToPyObject,
};

use crate::{
    communications::{DGRequest, DGResponse, PyModelRequest},
    config::DGConfig,
    errors::{DGError, Result},
};

#[derive(Default)]
pub struct PyModel {
    pub name: &'static str,
    pub module_path: PathBuf,
    pub module: &'static str,
    pub requester_hook: &'static str,
    config: DGConfig,
}

impl PyModel {
    pub fn new(config: DGConfig) -> Self {
        PyModel {
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
        request: DGRequest<PyModelRequest<K, V, T>>,
    ) -> Result<DGResponse<PyObject>>
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

        // TODO: Please, fix this!
        let source = fs::read_to_string(module_file.as_str()).unwrap();

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

        datamod
            .call_method(self.requester_hook, args, Some(kwargs))
            .map_err(|e| {
                e.print(py);
                let err_msg: String = format!(
                    "Call failed over {:?}\n\
            \twith traceback",
                    self.requester_hook
                );
                DGError::PyModuleError(err_msg.to_owned())
            })
            .map(|resp| DGResponse::<PyObject> {
                body: resp.to_object(py),
            })
    }
}
