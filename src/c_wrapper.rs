use super::*;
use libc::c_char;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::panic;
use types::*;

/// On unix, Vim loads and unloads the library for every call. On Mac, and
/// possibly other unices, each load creates a new tlv key, and there is a
/// maximum number allowed per process.  When we run out, dlopen() aborts
/// our process.
///
/// Here we reference ourselves and throw the handle away to prevent
/// ourselves from being unloaded (and also set RTLD_NODELETE and
/// RTLD_GLOBAL to make extra sure).
#[cfg(unix)]
mod reference_hack {
    use libc::Dl_info;
    use libc::{c_void, dladdr, dlerror, dlopen};
    use std::ffi::CStr;
    use std::ptr;

    pub static mut INITIALIZED: bool = false;

    cfg_if::cfg_if! {
        if #[cfg(any(target_os = "netbsd", target_os = "openbsd"))] {
            use libc::{RTLD_GLOBAL, RTLD_LAZY};

            pub fn first_attempt_flags() -> i32 {
                RTLD_LAZY | RTLD_GLOBAL
            }
            pub fn second_attempt_flags() -> i32 {
                RTLD_LAZY | RTLD_GLOBAL
            }
        } else if #[cfg(target_os = "android")] {
            use libc::{RTLD_GLOBAL, RTLD_LAZY, RTLD_NOLOAD};

            pub fn first_attempt_flags() -> i32 {
                RTLD_LAZY | RTLD_NOLOAD | RTLD_GLOBAL
            }
            pub fn second_attempt_flags() -> i32 {
                RTLD_LAZY | RTLD_GLOBAL
            }
        } else {
            use libc::{RTLD_GLOBAL, RTLD_LAZY, RTLD_NODELETE, RTLD_NOLOAD};

            pub fn first_attempt_flags() -> i32 {
                RTLD_LAZY | RTLD_NOLOAD | RTLD_GLOBAL | RTLD_NODELETE
            }
            pub fn second_attempt_flags() -> i32 {
                RTLD_LAZY | RTLD_GLOBAL | RTLD_NODELETE
            }
        }
    }

    pub unsafe fn initialize() {
        if INITIALIZED {
            return;
        }

        let mut info: Dl_info = Dl_info {
            dli_fname: ptr::null(),
            dli_fbase: ptr::null_mut(),
            dli_sname: ptr::null(),
            dli_saddr: ptr::null_mut(),
        };
        let initialize_ptr: *const c_void = initialize as *const c_void;
        if dladdr(initialize_ptr, &mut info) == 0 {
            panic!("Could not get parinfer library path.");
        }
        // First, try to use RTLD_NOLOAD to promote the existing object.  If
        // this fails, it could be because we don't think we are already
        // loaded (this happens when running the tests under Linux, but not
        // Mac).  dlerror() is unhelfully NULL at that point, so try to
        // *really* load ourselves, then report if that fails.
        let handle = dlopen(info.dli_fname, first_attempt_flags());
        if handle.is_null() {
            let handle = dlopen(info.dli_fname, second_attempt_flags());
            if handle.is_null() {
                let error = dlerror();
                if error.is_null() {
                    panic!(
                        "Could not reference parinfer_rust library {:?}.",
                        CStr::from_ptr(info.dli_fname)
                    );
                } else {
                    panic!(
                        "Could not reference parinfer_rust library {:?}: {:?}.",
                        CStr::from_ptr(info.dli_fname),
                        CStr::from_ptr(error)
                    );
                }
            }
        }
        INITIALIZED = true;
    }
}

#[cfg(windows)]
mod reference_hack {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::ptr;
    use winapi::um::libloaderapi::{
        GetModuleFileNameW, GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
        GET_MODULE_HANDLE_EX_FLAG_PIN,
    };
    use winapi::um::winnt::LPCWSTR;

    pub static mut INITIALIZED: bool = false;

    pub unsafe fn initialize() {
        let mut out = ptr::null_mut();
        if GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_PIN,
            initialize as LPCWSTR,
            &mut out,
        ) == 0
        {
            panic!("Could not pin parinfer_rust DLL.")
        } else {
            let mut buf = Vec::with_capacity(512);
            let len = GetModuleFileNameW(out, buf.as_mut_ptr(), 512 as u32) as usize;
            if len > 0 {
                buf.set_len(len);
                let filename = OsString::from_wide(&buf)
                    .into_string()
                    .expect("expect a string");
                if filename.ends_with(".dll") {
                } else {
                    panic!("parinfer_rust: reference_hack failed to find DLL.");
                }
            } else {
                panic!("parinfer_rust: could not get DLL filename");
            }
        }
    }
}

#[cfg(all(not(windows), not(unix)))]
mod reference_hack {
    pub static mut INITIALIZED: bool = true;

    pub fn initialize() {}
}

pub use self::reference_hack::INITIALIZED;

unsafe fn unwrap_c_pointers(json: *const c_char) -> Result<CString, Error> {
    let json_str = CStr::from_ptr(json).to_str()?;
    let response = common_wrapper::internal_run(json_str)?;
    Ok(CString::new(response)?)
}

thread_local!(static BUFFER: RefCell<Option<CString>> = const { RefCell::new(None) });

#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn run_parinfer(json: *const c_char) -> *const c_char {
    reference_hack::initialize();
    let output = match panic::catch_unwind(|| unwrap_c_pointers(json)) {
        Ok(Ok(result)) => result,
        Ok(Err(e)) => {
            let out = serde_json::to_string(&Answer::from(e)).unwrap();
            CString::new(out).unwrap()
        }
        Err(_) => {
            let out = common_wrapper::panic_result();
            CString::new(out).unwrap()
        }
    };

    BUFFER.with(|buffer| {
        buffer.replace(Some(output));
        buffer.borrow().as_ref().unwrap().as_ptr()
    })
}
