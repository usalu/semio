//! ↩️ `RemoveAccessor` semantic inverse.

use super::mutation::RemoveAccessor;
use crate::artifacts::gltf::schema::mutations::*;
use crate::artifacts::gltf::GltfSnapshot;

pub fn inverse(payload: &RemoveAccessor, base: &GltfSnapshot) -> Vec<GltfMutation> {
    base.document.accessors.get(payload.index).map(|accessor| vec![GltfMutation::InsertAccessor(InsertAccessor { index: payload.index, accessor: accessor.clone() })]).unwrap_or_default()
}
