//! 🐧 Linux system-ICU temporal presentation.

use super::{PlatformTemporalAdapter, PlatformTemporalBatch};
use std::ffi::{c_char, c_int, c_void, CString};
use std::sync::OnceLock;
use ui_contract::HostTemporalFormatRequestV1;

const RTLD_LOCAL: c_int = 0;
const RTLD_NOW: c_int = 2;

#[path = "../🌐️icu/🦀️.rs"]
mod icu;

#[link(name = "dl")]
unsafe extern "C" {
    fn dlopen(path: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
}

struct Library {
    handle: *mut c_void,
    suffix: String,
}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe { dlclose(self.handle) };
    }
}

impl Library {
    fn open() -> Result<Self, String> {
        let candidates = std::iter::once("libicui18n.so".to_string()).chain((60..=99).rev().map(|major| format!("libicui18n.so.{major}")));
        for candidate in candidates {
            let path = CString::new(candidate).map_err(|_| "host-temporal-format.icu-library-name".to_string())?;
            let handle = unsafe { dlopen(path.as_ptr(), RTLD_NOW | RTLD_LOCAL) };
            if handle.is_null() {
                continue;
            }
            let mut library = Self { handle, suffix: String::new() };
            if library.raw_symbol("udat_open").is_some() {
                return Ok(library);
            }
            for major in (60..=99).rev() {
                library.suffix = format!("_{major}");
                if library.raw_symbol("udat_open").is_some() {
                    return Ok(library);
                }
            }
        }
        Err("host-temporal-format.icu-library-unavailable".to_string())
    }

    fn raw_symbol(&self, name: &str) -> Option<*mut c_void> {
        let name = CString::new(format!("{name}{}", self.suffix)).ok()?;
        let pointer = unsafe { dlsym(self.handle, name.as_ptr()) };
        (!pointer.is_null()).then_some(pointer)
    }
}

impl icu::IcuLibrary for Library {
    fn symbol(&self, name: &str) -> Option<*mut c_void> {
        self.raw_symbol(name)
    }
}

fn system_icu() -> Result<&'static icu::IcuApi, String> {
    static SYSTEM: OnceLock<Result<(Library, icu::IcuApi), String>> = OnceLock::new();
    match SYSTEM.get_or_init(|| {
        let library = Library::open()?;
        let api = icu::IcuApi::load(&library)?;
        Ok((library, api))
    }) {
        Ok((_, api)) => Ok(api),
        Err(fault) => Err(fault.clone()),
    }
}

pub(super) struct SystemPlatformAdapter;

impl PlatformTemporalAdapter for SystemPlatformAdapter {
    fn format(&self, request: &HostTemporalFormatRequestV1) -> Result<PlatformTemporalBatch, String> {
        icu::format(system_icu()?, request)
    }
}
