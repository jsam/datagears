use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

pub struct Gear {
    pycode: String,
}

impl Gear {
    pub fn init(&self) {
        pyo3::prepare_freethreaded_python()
    }

    pub fn run(&self) -> PyResult<()> {
        Python::with_gil(|py| {
            let sys = py.import("sys")?;
            let version: String = sys.getattr("version")?.extract()?;

            let locals = [("os", py.import("os")?)].into_py_dict(py);
            let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
            let user: String = py.eval(code, None, Some(&locals))?.extract()?;

            println!("Hello {}, I'm Python {}", user, version);
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_run() {
        let gear = Gear {
            pycode: "".to_string(),
        };

        gear.init();
        let _ = gear.run();
    }
}
