#[macro_export]
macro_rules! append_to_inittab {
    ($module:ident) => {
        unsafe {
            if $crate::ffi::Py_IsInitialized() != 0 {
                ::std::panic!(
                    "called `append_to_inittab` but a Python interpreter is already running."
                );
            }
            $crate::ffi::PyImport_AppendInittab(
                $module::NAME.as_ptr() as *const ::std::os::raw::c_char,
                ::std::option::Option::Some($module::init),
            );
        }
    };
}
