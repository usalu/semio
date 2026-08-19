//! 🔒 Local validation primitives for structure-and-geometry mutation leaves.
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
pub async fn checked_position(position: usize, length: usize, path: &str) -> Result<(), GltfTopLevelMutationRejection> { if position <= length { Ok(()) } else { Err(reject("gltf.mutation.insert-out-of-range", path, format!("position {position}, length {length}"))) } }
pub async fn checked_index(index: usize, length: usize, path: &str) -> Result<(), GltfTopLevelMutationRejection> { if index < length { Ok(()) } else { Err(reject("gltf.mutation.index-out-of-range", path, format!("index {index}, length {length}"))) } }
