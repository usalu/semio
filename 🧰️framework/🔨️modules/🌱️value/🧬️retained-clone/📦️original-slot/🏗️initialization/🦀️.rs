//! 🏗️ Initializes absent owner storage through bounded copies from immutable language-owned templates.
use crate::{ValueError, ValueRefusalKind};
use std::mem::{size_of, MaybeUninit};

fn absent_template<T: 'static>() -> &'static Option<T> { &None }
fn absent_array_template<T: 'static, const N: usize>() -> &'static [Option<T>; N] { &[const { None }; N] }

/// 📐️ The full destination extent includes its language-defined discriminant, payload and padding.
pub fn absent_field_bytes<T>() -> usize { size_of::<Option<T>>() }

/// 📏️ Every absent array element retains its actual destination extent.
pub fn absent_array_bytes<T, const N: usize>() -> usize { size_of::<[Option<T>; N]>() }

/// 🪜️ Selects the next copy from the caller's remaining byte allowance.
pub fn absent_chunk_bytes(extent: usize, offset: usize, maximum: usize) -> Result<usize, ValueError> {
    selected_end(extent, offset, 0)?;
    Ok((extent - offset).min(maximum))
}

fn selected_end(extent: usize, offset: usize, count: usize) -> Result<usize, ValueError> {
    if offset > extent || count > extent - offset {
        return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "absent initialization exceeds its retained destination"));
    }
    Ok(offset + count)
}

/// 🧱️ Copies exactly the selected absent-field bytes without creating a temporary owned value.
///
/// # Safety
/// The destination must be exclusively owned uninitialized storage. The caller must track complete prior coverage; the completion result describes only this range. The full extent must be initialized before any typed borrow or drop.
pub unsafe fn initialize_absent_field_at<T: 'static>(target: *mut Option<T>, offset: usize, count: usize) -> Result<bool, ValueError> {
    let extent = absent_field_bytes::<T>();
    let end = selected_end(extent, offset, count)?;
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(std::ptr::from_ref(absent_template::<T>()).cast::<MaybeUninit<u8>>().add(offset), target.cast::<MaybeUninit<u8>>().add(offset), count);
        }
    }
    Ok(end == extent)
}

/// 🧩️ Copies a selected slice of an absent array without constructing an aggregate array temporary.
///
/// # Safety
/// The destination must be exclusively owned uninitialized storage. The caller must track complete prior coverage; the completion result describes only this range. The full extent must be initialized before any typed borrow or drop.
pub unsafe fn initialize_absent_array_at<T: 'static, const N: usize>(target: *mut [Option<T>; N], offset: usize, count: usize) -> Result<bool, ValueError> {
    let extent = absent_array_bytes::<T, N>();
    let end = selected_end(extent, offset, count)?;
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(std::ptr::from_ref(absent_array_template::<T, N>()).cast::<MaybeUninit<u8>>().add(offset), target.cast::<MaybeUninit<u8>>().add(offset), count);
        }
    }
    Ok(end == extent)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
