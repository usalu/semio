//! 🪟 Windows system-ICU temporal presentation.

use super::{PlatformTemporalAdapter, PlatformTemporalBatch};
use std::ffi::{c_char, c_void, CString};
use std::sync::OnceLock;
use ui_contract::HostTemporalFormatRequestV1;

#[path = "../🌐️icu/🦀️.rs"]
mod icu;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetSystemDirectoryW(buffer: *mut u16, capacity: u32) -> u32;
    fn LoadLibraryW(path: *const u16) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn FreeLibrary(module: *mut c_void) -> i32;
}

fn system_directory() -> Result<&'static [u16], String> {
    static DIRECTORY: OnceLock<Result<Vec<u16>, String>> = OnceLock::new();
    match DIRECTORY.get_or_init(|| {
        let mut buffer = vec![0u16; 32_768];
        let length = unsafe { GetSystemDirectoryW(buffer.as_mut_ptr(), buffer.len() as u32) } as usize;
        if length == 0 || length >= buffer.len() {
            return Err("host-temporal-format.windows-system-directory".to_string());
        }
        buffer.truncate(length);
        Ok(buffer)
    }) {
        Ok(directory) => Ok(directory),
        Err(fault) => Err(fault.clone()),
    }
}

fn load_system_library(name: &str) -> Result<*mut c_void, String> {
    let mut path = system_directory()?.to_vec();
    if path.last() != Some(&(b'\\' as u16)) {
        path.push(b'\\' as u16);
    }
    path.extend(name.encode_utf16());
    path.push(0);
    let handle = unsafe { LoadLibraryW(path.as_ptr()) };
    (!handle.is_null()).then_some(handle).ok_or_else(|| format!("host-temporal-format.windows-system-library-{name}"))
}

struct Library {
    handles: Vec<*mut c_void>,
    suffix: String,
}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}

impl Drop for Library {
    fn drop(&mut self) {
        for handle in self.handles.drain(..).rev() {
            unsafe { FreeLibrary(handle) };
        }
    }
}

impl Library {
    fn load(names: &[&str]) -> Result<Self, String> {
        let mut library = Self { handles: Vec::with_capacity(names.len()), suffix: String::new() };
        for name in names {
            library.handles.push(load_system_library(name)?);
        }
        Ok(library)
    }

    fn resolve(mut self) -> Option<Self> {
        if self.raw_symbol("udat_open").is_some() {
            return Some(self);
        }
        for major in (60..=99).rev() {
            self.suffix = format!("_{major}");
            if self.raw_symbol("udat_open").is_some() {
                return Some(self);
            }
        }
        None
    }

    fn open() -> Result<Self, String> {
        if let Ok(library) = Self::load(&["icu.dll"]) {
            if let Some(library) = library.resolve() {
                return Ok(library);
            }
        }
        if let Ok(library) = Self::load(&["icuin.dll", "icuuc.dll"]) {
            if let Some(library) = library.resolve() {
                return Ok(library);
            }
        }
        for major in (60..=99).rev() {
            let international = format!("icuin{major}.dll");
            let common = format!("icuuc{major}.dll");
            if let Ok(library) = Self::load(&[international.as_str(), common.as_str()]) {
                if let Some(library) = library.resolve() {
                    return Ok(library);
                }
            }
        }
        Err("host-temporal-format.windows-system-icu-unavailable".to_string())
    }

    fn raw_symbol(&self, name: &str) -> Option<*mut c_void> {
        let name = CString::new(format!("{name}{}", self.suffix)).ok()?;
        self.handles.iter().find_map(|handle| {
            let pointer = unsafe { GetProcAddress(*handle, name.as_ptr()) };
            (!pointer.is_null()).then_some(pointer)
        })
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
