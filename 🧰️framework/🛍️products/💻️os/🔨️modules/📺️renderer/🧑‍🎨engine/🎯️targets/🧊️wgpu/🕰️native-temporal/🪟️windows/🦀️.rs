//! 🪟 Windows system-ICU temporal presentation.

use super::{PlatformTemporalAdapter, PlatformTemporalBatch};
use std::ffi::{c_char, c_void, CString};
use std::sync::OnceLock;
use ui_contract::HostTemporalFormatRequestV1;

#[path = "../🌐️icu/🦀️.rs"]
mod icu;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn LoadLibraryW(path: *const u16) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn FreeLibrary(module: *mut c_void) -> i32;
}

struct Library {
    handle: *mut c_void,
    suffix: String,
}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe { FreeLibrary(self.handle) };
    }
}

impl Library {
    fn open() -> Result<Self, String> {
        let path = "icu.dll\0".encode_utf16().collect::<Vec<_>>();
        let handle = unsafe { LoadLibraryW(path.as_ptr()) };
        if handle.is_null() {
            return Err("host-temporal-format.windows-icu-library-unavailable".to_string());
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
        Err("host-temporal-format.windows-icu-symbols-unavailable".to_string())
    }

    fn raw_symbol(&self, name: &str) -> Option<*mut c_void> {
        let name = CString::new(format!("{name}{}", self.suffix)).ok()?;
        let pointer = unsafe { GetProcAddress(self.handle, name.as_ptr()) };
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
