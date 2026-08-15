//! ↩️ `SetAccessor` semantic inverse.

use super::mutation::SetAccessor;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &SetAccessor, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.accessors.get(payload.index).map(|accessor| vec![GltfMutation::SetAccessor(SetAccessor { index: payload.index, accessor: accessor.clone() })]).unwrap_or_default()
}
