use cpython::{PyDict, PyResult, Python};

pub struct Gear {
    pycode: String,
}

impl Gear {
    pub fn run(&self, py: Python) -> PyResult<()> {
        println!("hello");
        let sys = py.import("sys")?;
        let version: String = sys.get(py, "version")?.extract(py)?;

        let locals = PyDict::new(py);
        locals.set_item(py, "os", py.import("os")?)?;
        let user: String = py
            .eval(
                "os.getenv('USER') or os.getenv('USERNAME')",
                None,
                Some(&locals),
            )?
            .extract(py)?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use cpython::Python;

    use super::Gear;


    #[test]
    fn test_gear_run() {
        let gil = Python::acquire_gil();
        let gear = Gear {
            pycode: "".to_string(),
        };

        let _ = gear.run(gil.python());
    }
}
