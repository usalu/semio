//! 🪪️ Owned time-ordered identities and platform entropy behind a dependency-free interface.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// #region 🔖️Entropy
/// 🎲️ Failure to obtain entropy from the current platform boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntropyError;

impl fmt::Display for EntropyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("platform entropy source failed")
    }
}

impl std::error::Error for EntropyError {}

/// 🎲️ Fills `bytes` from the operating system or browser entropy boundary without exposing a
/// platform-specific type.
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn fill_entropy(bytes: &mut [u8]) -> Result<(), EntropyError> {
    unsafe extern "C" {
        fn arc4random_buf(buffer: *mut core::ffi::c_void, length: usize);
    }
    unsafe { arc4random_buf(bytes.as_mut_ptr().cast(), bytes.len()) };
    Ok(())
}

/// 🎲️ Linux and Android expose the kernel `getrandom(2)` service through libc without requiring
/// the external Rust `getrandom` crate.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn fill_entropy(bytes: &mut [u8]) -> Result<(), EntropyError> {
    unsafe extern "C" {
        fn getrandom(buffer: *mut core::ffi::c_void, length: usize, flags: u32) -> isize;
    }
    let mut cursor = 0;
    while cursor < bytes.len() {
        let written = unsafe { getrandom(bytes[cursor..].as_mut_ptr().cast(), bytes.len() - cursor, 0) };
        if written <= 0 {
            return Err(EntropyError);
        }
        cursor += written as usize;
    }
    Ok(())
}

/// 🎲️ Windows system-preferred CNG entropy.
#[cfg(target_os = "windows")]
pub fn fill_entropy(bytes: &mut [u8]) -> Result<(), EntropyError> {
    #[link(name = "bcrypt")]
    unsafe extern "system" {
        fn BCryptGenRandom(algorithm: *mut core::ffi::c_void, buffer: *mut u8, length: u32, flags: u32) -> i32;
    }
    const USE_SYSTEM_PREFERRED_RNG: u32 = 0x0000_0002;
    let length = u32::try_from(bytes.len()).map_err(|_| EntropyError)?;
    let status = unsafe { BCryptGenRandom(core::ptr::null_mut(), bytes.as_mut_ptr(), length, USE_SYSTEM_PREFERRED_RNG) };
    if status >= 0 {
        Ok(())
    } else {
        Err(EntropyError)
    }
}

/// 🎲️ Browser-hosted IDs call the platform `crypto.getRandomValues` boundary without exposing a JS
/// type through the owned interface. `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too, so
/// this is narrowed to the browser only — `js_sys`/`wasm_bindgen` have no meaning under WASI.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub fn fill_entropy(bytes: &mut [u8]) -> Result<(), EntropyError> {
    use wasm_bindgen::JsCast;
    let global = js_sys::global();
    let crypto = js_sys::Reflect::get(&global, &"crypto".into()).map_err(|_| EntropyError)?;
    let fill: js_sys::Function = js_sys::Reflect::get(&crypto, &"getRandomValues".into()).map_err(|_| EntropyError)?.dyn_into().map_err(|_| EntropyError)?;
    let array = js_sys::Uint8Array::new_with_length(u32::try_from(bytes.len()).map_err(|_| EntropyError)?);
    fill.call1(&crypto, &array).map_err(|_| EntropyError)?;
    array.copy_to(bytes);
    Ok(())
}

/// 🚧️ `wasm32-wasip2` has no arm of its own here: a correct `wasi:random/random` component import
/// needs a hand-rolled canonical-ABI binding (the same shape as `semio_browser_host` in
/// `ui-host`'s `🦀️window.rs`, but for the component model rather than a core-wasm import), which
/// is real implementation work beyond this slice — left unimplemented rather than stubbed with a
/// false success. wasip2 therefore falls into this catch-all `Err(EntropyError)` path deliberately
/// (widened from the pre-existing "every other platform" arm, not a new stub), which
/// `time_ordered_id` below already degrades from gracefully via a clock/pid-seeded `splitmix64` —
/// so nothing here panics or fabricates cryptographic strength it doesn't have.
#[cfg(any(not(any(target_os = "macos", target_os = "ios", target_os = "linux", target_os = "android", target_os = "windows", target_arch = "wasm32")), target_env = "p2"))]
pub fn fill_entropy(_bytes: &mut [u8]) -> Result<(), EntropyError> {
    Err(EntropyError)
}
// #endregion 🔖️Entropy

// #region 🔖️Identity
static ID_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// 🪪️ Returns a UUID-v7-shaped, lexically time-ordered identifier using owned formatting and the
/// platform entropy interface. The monotonic process sequence remains mixed in when the clock has
/// insufficient resolution or the entropy boundary is temporarily unavailable.
pub fn time_ordered_id() -> String {
    let millis = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| duration.as_millis().min((1u128 << 48) - 1) as u64);
    let sequence = ID_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let mut bytes = [0u8; 16];
    if fill_entropy(&mut bytes[6..]).is_err() {
        let mut state = splitmix64(millis ^ sequence.rotate_left(17) ^ u64::from(std::process::id()));
        for chunk in bytes[6..].chunks_mut(8) {
            state = splitmix64(state);
            chunk.copy_from_slice(&state.to_le_bytes()[..chunk.len()]);
        }
    }
    let timestamp = millis.to_be_bytes();
    bytes[..6].copy_from_slice(&timestamp[2..]);
    bytes[6] = 0x70 | (bytes[6] & 0x0f);
    bytes[8] = 0x80 | (bytes[8] & 0x3f);
    let sequence_bytes = sequence.to_be_bytes();
    bytes[14] ^= sequence_bytes[6];
    bytes[15] ^= sequence_bytes[7];
    format_uuid(bytes)
}

fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn format_uuid(bytes: [u8; 16]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = [0u8; 36];
    let mut source = 0;
    let mut target = 0;
    while source < bytes.len() {
        if matches!(source, 4 | 6 | 8 | 10) {
            output[target] = b'-';
            target += 1;
        }
        output[target] = HEX[(bytes[source] >> 4) as usize];
        output[target + 1] = HEX[(bytes[source] & 0x0f) as usize];
        source += 1;
        target += 2;
    }
    String::from_utf8(output.to_vec()).expect("UUID formatter emits ASCII")
}
// #endregion 🔖️Identity

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn time_ordered_ids_have_v7_shape_and_do_not_repeat() {
        let ids: Vec<_> = (0..10_000).map(|_| time_ordered_id()).collect();
        assert!(ids.iter().all(|id| id.len() == 36 && id.as_bytes()[14] == b'7' && matches!(id.as_bytes()[19], b'8' | b'9' | b'a' | b'b')));
        assert_eq!(ids.iter().collect::<BTreeSet<_>>().len(), ids.len());
    }

    #[test]
    fn entropy_changes_successive_buffers() {
        let mut first = [0u8; 32];
        let mut second = [0u8; 32];
        fill_entropy(&mut first).expect("platform entropy");
        fill_entropy(&mut second).expect("platform entropy");
        assert_ne!(first, second);
    }
}
// #endregion 🔖️Tests
