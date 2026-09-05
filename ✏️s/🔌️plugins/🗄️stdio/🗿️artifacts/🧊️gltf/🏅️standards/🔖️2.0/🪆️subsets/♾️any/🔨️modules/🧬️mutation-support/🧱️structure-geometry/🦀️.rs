//! 🔒 Local validation primitives for structure-and-geometry mutation leaves.
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn checked_position(position: usize, length: usize, path: &str) -> Result<(), GltfTopLevelMutationRejection> {
    if position <= length {
        Ok(())
    } else {
        Err(reject("gltf.mutation.insert-out-of-range", path, format!("position {position}, length {length}")))
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn checked_index(index: usize, length: usize, path: &str) -> Result<(), GltfTopLevelMutationRejection> {
    if index < length {
        Ok(())
    } else {
        Err(reject("gltf.mutation.index-out-of-range", path, format!("index {index}, length {length}")))
    }
}
