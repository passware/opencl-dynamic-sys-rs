use std::sync::OnceLock;

use dlopen2::{wrapper::Container, Error};

use super::OpenCl;

static OPENCL_CONTAINER: OnceLock<Result<Container<OpenCl>, Error>> = OnceLock::new();

pub fn load_library() -> &'static Result<Container<OpenCl>, Error> {
    OPENCL_CONTAINER.get_or_init(|| {
        if let Ok(env_var) = std::env::var("OPENCL_DYLIB_PATH") {
            for library_path in env_var.split(';') {
                let library = unsafe { Container::load(library_path) };
                if library.is_ok() {
                    return library;
                }
            }
        }

        const LIBRARY_NAME: &'static str = if cfg!(windows) {
            "OpenCL.dll"
        } else if cfg!(darwin) {
            "/System/Library/Frameworks/OpenCL.framework/OpenCL"
        } else {
            "libOpenCL.so"
        };

        unsafe { Container::load(LIBRARY_NAME) }
    })
}