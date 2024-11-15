use std::sync::OnceLock;

use dlopen2::{wrapper::Container, Error};

use super::OpenCl;

/// `dlopen2` container with all loaded API functions.
pub type OpenClRuntime = Container<OpenCl>;

static OPENCL_RUNTIME: OnceLock<Result<OpenClRuntime, Error>> = OnceLock::new();

/// Utility function to load the OpenCL shared library (actual load will be performed only once).
/// Returns an error if the library is not found.
pub fn load_library() -> &'static Result<OpenClRuntime, Error> {
    OPENCL_RUNTIME.get_or_init(|| {
        if let Ok(env_var) = std::env::var("OPENCL_DYLIB_PATH") {
            for library_path in env_var.split(';') {
                let library = unsafe { Container::load(library_path) };
                if library.is_ok() {
                    return library;
                }
            }
        }

        const LIBRARY_NAME: &'static str = if cfg!(target_os = "windows") {
            "OpenCL.dll"
        } else if cfg!(target_os = "macos") {
            "/System/Library/Frameworks/OpenCL.framework/OpenCL"
        } else {
            "libOpenCL.so"
        };

        unsafe { Container::load(LIBRARY_NAME) }
    })
}

/// Utility function to check if the OpenCL shared library is loaded successfully.
pub fn is_opencl_runtime_available() -> bool {
    load_library().is_ok()
}
