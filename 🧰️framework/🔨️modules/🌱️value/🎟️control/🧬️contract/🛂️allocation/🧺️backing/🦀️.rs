//! 🧺️ Exact native layouts allocate empty vector and valid UTF-8 ownership without implicit growth.
use crate::{ValueError,ValueRefusalKind};
use std::alloc::{alloc,Layout};
fn layout<T>(count:usize)->Result<Layout,ValueError>{Layout::array::<T>(count).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"exact native backing layout exceeds the addressable extent"))}
/// 📏️ Computes the same checked physical layout used by the raw allocation primitive.
pub fn exact_vector_bytes<T>(count:usize)->Result<usize,ValueError>{Ok(layout::<T>(count)?.size())}
/// 🧺️ Allocates only the requested empty backing; the caller owns admission, work and the actual allocation marker.
pub fn allocate_exact_vector<T>(count:usize)->Result<Vec<T>,ValueError>{
 let layout=layout::<T>(count)?;if layout.size()==0{return Ok(Vec::new())}
 let pointer=unsafe{alloc(layout)}.cast::<T>();if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"exact native backing allocation refused"))}
 Ok(unsafe{Vec::from_raw_parts(pointer,0,count)})
}
/// 🔤️ Empty initialized content is valid UTF-8 while preserving the exact byte allocation.
pub fn allocate_exact_string(bytes:usize)->Result<String,ValueError>{allocate_exact_vector::<u8>(bytes).map(|value|unsafe{String::from_utf8_unchecked(value)})}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
