//! 🌐️ Raw C-ABI for the browser (no wasm-bindgen): `alloc` an input buffer, write ModelScene JSON, `encode`, then read
//! `result_ptr`/`result_len` (DWG bytes when `status` is 0, a UTF-8 error message otherwise) and `free`.

use std::cell::RefCell;

thread_local! {
    static RESULT: RefCell<(i32, Vec<u8>)> = const { RefCell::new((0, Vec::new())) };
}

/// 📦️ Allocates `len` bytes for the caller to fill.
#[no_mangle]
pub extern "C" fn staffage_alloc(len: usize) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(len.max(1));
    let pointer = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    pointer
}

/// 🗑️ Releases a buffer from [`staffage_alloc`].
///
/// # Safety
/// `pointer`/`len` must be exactly an allocation made by [`staffage_alloc`].
#[no_mangle]
pub unsafe extern "C" fn staffage_free(pointer: *mut u8, len: usize) {
    drop(unsafe { Vec::from_raw_parts(pointer, 0, len.max(1)) });
}

/// 🧱️ Encodes the JSON at `pointer`; returns 0 on success, 1 on error.
///
/// # Safety
/// `pointer`/`len` must be a readable range of initialised bytes.
#[no_mangle]
pub unsafe extern "C" fn staffage_encode(pointer: *const u8, len: usize) -> i32 {
    let json = unsafe { std::slice::from_raw_parts(pointer, len) };
    let (status, bytes) = match crate::encode_json(json) {
        Ok(bytes) => (0, bytes),
        Err(message) => (1, message.into_bytes()),
    };
    RESULT.with(|cell| *cell.borrow_mut() = (status, bytes));
    status
}

/// 📍️ Address of the last result.
#[no_mangle]
pub extern "C" fn staffage_result_ptr() -> *const u8 {
    RESULT.with(|cell| cell.borrow().1.as_ptr())
}

/// 📏️ Length of the last result.
#[no_mangle]
pub extern "C" fn staffage_result_len() -> usize {
    RESULT.with(|cell| cell.borrow().1.len())
}
