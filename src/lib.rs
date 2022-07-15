pub mod communications;
pub mod config;
pub mod core;
pub mod errors;
pub mod gears;
pub mod macros;
pub mod services;

// struct _interpreter();

// impl _interpreter {
//     fn new() -> Self {
//         pyo3::prepare_freethreaded_python();
//         // append_to_inittab!("requests");
//         // Python::with_gil(|py| {

//         //     let pip_internal = py.import("pip._internal")?;
//         //     let main: String = pip_internal.getattr("main")?.extract()?;

//         //     let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
//         //     let user: String = py.eval(code, None, Some(&locals))?.extract()?;

//         //     println!("Hello {}, I'm Python {}", user, version);
//         //     Ok(())
//         // });

//         Self ()
//     }
// }

// //static mut DATAGEARS_PYTHON: _interpreter = _interpreter::new();
