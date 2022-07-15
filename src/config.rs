use std::{env, path::PathBuf};

use pyo3::{
    ffi::{PyImport_AppendInittab, PyObject},
    types::{PyList, PyModule, PyString},
    Python,
};
use tract_onnx::tract_hir::infer::InferenceFact;

use crate::append_to_inittab;

#[derive(Debug, Clone)]
pub struct DGConfig {
    pub auto_load_input_facts: bool,
    pub default_input_fact_shape: Option<InferenceFact>,

    pub venv_path: PathBuf,
    pub gears_path: PathBuf,
}

impl Default for DGConfig {
    fn default() -> Self {
        let venv_packages_env =
            env::var("VENV_PACKAGES").expect("`VENV_PACKAGES` variable must be specified");
        let gears_packages_env =
            env::var("GEARS_PACKAGES").expect("`GEARS_PACKAGES` variable must be specified");

        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let sys = PyModule::import(py, "sys").unwrap();
            let sys_any = sys.getattr("path").unwrap();
            let syspath = sys_any.downcast::<PyList>().unwrap();

            let venv_entry = syspath
                .get_item(0)
                .unwrap()
                .downcast::<PyString>()
                .unwrap()
                .to_string_lossy();

            let gears_entry = syspath
                .get_item(1)
                .unwrap()
                .downcast::<PyString>()
                .unwrap()
                .to_string_lossy();

            if venv_entry.as_ref() != venv_packages_env
                && gears_entry.as_ref() != gears_packages_env
            {
                syspath.insert(0, venv_packages_env.clone()).unwrap();
                syspath.insert(1, gears_packages_env.clone()).unwrap();
            }

            //let module = PyModule::import(py, "requests").expect("requests is expected to be installed");
        });

        Self {
            auto_load_input_facts: Default::default(),
            default_input_fact_shape: Default::default(),
            venv_path: PathBuf::from(venv_packages_env),
            gears_path: PathBuf::from(gears_packages_env),
        }
    }
}

impl DGConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_auto_load_input_facts(mut self) -> Self {
        self.auto_load_input_facts = true;
        self
    }

    pub fn with_default_input_fact_shape(mut self, shape: InferenceFact) -> Self {
        self.default_input_fact_shape = Option::from(shape);
        self
    }
}
