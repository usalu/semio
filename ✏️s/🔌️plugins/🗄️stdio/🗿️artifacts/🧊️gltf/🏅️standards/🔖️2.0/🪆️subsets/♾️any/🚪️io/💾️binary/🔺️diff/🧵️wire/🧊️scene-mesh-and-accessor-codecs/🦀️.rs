//! 🧊️ Binary codecs of the asset, scene, node, mesh, sparse and accessor elements and their diffs.

use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfAccessor, standards::v2_0::subsets::any::schema::snapshot::GltfMesh, standards::v2_0::subsets::any::schema::snapshot::GltfMorphTarget, standards::v2_0::subsets::any::schema::snapshot::GltfNode, standards::v2_0::subsets::any::schema::snapshot::GltfPrimitive, standards::v2_0::subsets::any::schema::snapshot::GltfScene, standards::v2_0::subsets::any::schema::snapshot::GltfSparseAccessor, standards::v2_0::subsets::any::schema::snapshot::GltfSparseIndices, standards::v2_0::subsets::any::schema::snapshot::GltfSparseValues};
use crate::standards::v2_0::subsets::any::io::binary::diff::wire::*;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_asset_diff(w: &mut dsl::ByteWriter, d: &GltfAssetDiff) {
    write_bin_option(w, &d.version, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.generator, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.copyright, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.min_version, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_asset_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfAssetDiff, dsl::PackRefusal> {
    Ok(GltfAssetDiff {
        version: read_bin_option(r, read_bin_str)?,
        generator: read_bin_tri(r, read_bin_str)?,
        copyright: read_bin_tri(r, read_bin_str)?,
        min_version: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_scene(w: &mut dsl::ByteWriter, sc: &GltfScene) {
    write_bin_usize_vec(w, &sc.nodes);
    write_bin_option(w, &sc.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &sc.extensions);
    write_bin_json_opt(w, &sc.extras);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_scene(r: &mut dsl::ByteReader<'_>) -> Result<GltfScene, dsl::PackRefusal> {
    Ok(GltfScene { nodes: read_bin_usize_vec(r)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_scene_diff(w: &mut dsl::ByteWriter, d: &GltfSceneDiff) {
    write_bin_option(w, &d.nodes, write_bin_refs_delta);
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_scene_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfSceneDiff, dsl::PackRefusal> {
    Ok(GltfSceneDiff { nodes: read_bin_option(r, read_bin_refs_delta)?, name: read_bin_tri(r, read_bin_str)?, extensions: read_bin_tri(r, read_bin_json)?, extras: read_bin_tri(r, read_bin_json)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_node(w: &mut dsl::ByteWriter, n: &GltfNode) {
    write_bin_usize_vec(w, &n.children);
    write_bin_option(w, &n.mesh, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &n.camera, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &n.skin, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &n.matrix, write_bin_f64_array::<16>);
    write_bin_option(w, &n.translation, write_bin_f64_array::<3>);
    write_bin_option(w, &n.rotation, write_bin_f64_array::<4>);
    write_bin_option(w, &n.scale, write_bin_f64_array::<3>);
    write_bin_f64_vec(w, &n.weights);
    write_bin_option(w, &n.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &n.extensions);
    write_bin_json_opt(w, &n.extras);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_node(r: &mut dsl::ByteReader<'_>) -> Result<GltfNode, dsl::PackRefusal> {
    Ok(GltfNode {
        children: read_bin_usize_vec(r)?,
        mesh: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        camera: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        skin: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        matrix: read_bin_option(r, read_bin_f64_array::<16>)?,
        translation: read_bin_option(r, read_bin_f64_array::<3>)?,
        rotation: read_bin_option(r, read_bin_f64_array::<4>)?,
        scale: read_bin_option(r, read_bin_f64_array::<3>)?,
        weights: read_bin_f64_vec(r)?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_node_diff(w: &mut dsl::ByteWriter, d: &GltfNodeDiff) {
    write_bin_option(w, &d.children, write_bin_refs_delta);
    write_bin_tri(w, &d.mesh, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.camera, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.skin, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.matrix, write_bin_f64_array::<16>);
    write_bin_tri(w, &d.translation, write_bin_f64_array::<3>);
    write_bin_tri(w, &d.rotation, write_bin_f64_array::<4>);
    write_bin_tri(w, &d.scale, write_bin_f64_array::<3>);
    write_bin_option(w, &d.weights, |w, v| write_bin_f64_vec(w, v));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_node_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfNodeDiff, dsl::PackRefusal> {
    Ok(GltfNodeDiff {
        children: read_bin_option(r, read_bin_refs_delta)?,
        mesh: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        camera: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        skin: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        matrix: read_bin_tri(r, read_bin_f64_array::<16>)?,
        translation: read_bin_tri(r, read_bin_f64_array::<3>)?,
        rotation: read_bin_tri(r, read_bin_f64_array::<4>)?,
        scale: read_bin_tri(r, read_bin_f64_array::<3>)?,
        weights: read_bin_option(r, read_bin_f64_vec)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_primitive(w: &mut dsl::ByteWriter, p: &GltfPrimitive) {
    write_bin_attr_pairs(w, &p.attributes);
    write_bin_option(w, &p.indices, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &p.material, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &p.mode, |w, v| w.write_varint_u64(*v));
    write_bin_vec(w, &p.targets, |w, target| write_bin_attr_pairs(w, &target.0));
    write_bin_json_opt(w, &p.extensions);
    write_bin_json_opt(w, &p.extras);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_primitive(r: &mut dsl::ByteReader<'_>) -> Result<GltfPrimitive, dsl::PackRefusal> {
    Ok(GltfPrimitive {
        attributes: read_bin_attr_pairs(r)?,
        indices: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        material: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        mode: read_bin_option(r, |r| r.read_varint_u64())?,
        targets: read_bin_vec(r, |r| read_bin_attr_pairs(r).map(GltfMorphTarget))?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_primitive_vec(w: &mut dsl::ByteWriter, v: &[GltfPrimitive]) {
    write_bin_vec(w, v, write_bin_primitive);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_primitive_vec(r: &mut dsl::ByteReader<'_>) -> Result<Vec<GltfPrimitive>, dsl::PackRefusal> {
    read_bin_vec(r, read_bin_primitive)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_mesh(w: &mut dsl::ByteWriter, m: &GltfMesh) {
    write_bin_primitive_vec(w, &m.primitives);
    write_bin_f64_vec(w, &m.weights);
    write_bin_option(w, &m.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &m.extensions);
    write_bin_json_opt(w, &m.extras);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_mesh(r: &mut dsl::ByteReader<'_>) -> Result<GltfMesh, dsl::PackRefusal> {
    Ok(GltfMesh { primitives: read_bin_primitive_vec(r)?, weights: read_bin_f64_vec(r)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_mesh_diff(w: &mut dsl::ByteWriter, d: &GltfMeshDiff) {
    write_bin_option(w, &d.primitives, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_primitive, write_bin_primitive_diff)));
    write_bin_option(w, &d.weights, |w, v| write_bin_f64_vec(w, v));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_mesh_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfMeshDiff, dsl::PackRefusal> {
    Ok(GltfMeshDiff {
        primitives: read_bin_option(r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_primitive, read_bin_primitive_diff)
        })?,
        weights: read_bin_option(r, read_bin_f64_vec)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_sparse_indices(w: &mut dsl::ByteWriter, v: &GltfSparseIndices) {
    w.write_varint_u64(v.buffer_view as u64);
    w.write_varint_u64(v.byte_offset as u64);
    write_bin_component_type(w, v.component_type);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_sparse_indices(r: &mut dsl::ByteReader<'_>) -> Result<GltfSparseIndices, dsl::PackRefusal> {
    Ok(GltfSparseIndices { buffer_view: r.read_varint_u64()? as usize, byte_offset: r.read_varint_u64()? as usize, component_type: read_bin_component_type(r)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_sparse_values(w: &mut dsl::ByteWriter, v: &GltfSparseValues) {
    w.write_varint_u64(v.buffer_view as u64);
    w.write_varint_u64(v.byte_offset as u64);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_sparse_values(r: &mut dsl::ByteReader<'_>) -> Result<GltfSparseValues, dsl::PackRefusal> {
    Ok(GltfSparseValues { buffer_view: r.read_varint_u64()? as usize, byte_offset: r.read_varint_u64()? as usize })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_sparse_accessor(w: &mut dsl::ByteWriter, v: &GltfSparseAccessor) {
    w.write_varint_u64(v.count as u64);
    write_bin_sparse_indices(w, &v.indices);
    write_bin_sparse_values(w, &v.values);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_sparse_accessor(r: &mut dsl::ByteReader<'_>) -> Result<GltfSparseAccessor, dsl::PackRefusal> {
    Ok(GltfSparseAccessor { count: r.read_varint_u64()? as usize, indices: read_bin_sparse_indices(r)?, values: read_bin_sparse_values(r)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_accessor(w: &mut dsl::ByteWriter, a: &GltfAccessor) {
    write_bin_option(w, &a.buffer_view, |w, v| w.write_varint_u64(*v as u64));
    w.write_varint_u64(a.byte_offset as u64);
    write_bin_component_type(w, a.component_type);
    w.write_u8(if a.normalized { 1 } else { 0 });
    w.write_varint_u64(a.count as u64);
    write_bin_accessor_type(w, a.kind);
    write_bin_option(w, &a.max, |w, v| write_bin_f64_vec(w, v));
    write_bin_option(w, &a.min, |w, v| write_bin_f64_vec(w, v));
    write_bin_option(w, &a.sparse, write_bin_sparse_accessor);
    write_bin_option(w, &a.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &a.extensions);
    write_bin_json_opt(w, &a.extras);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_accessor(r: &mut dsl::ByteReader<'_>) -> Result<GltfAccessor, dsl::PackRefusal> {
    Ok(GltfAccessor {
        buffer_view: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        byte_offset: r.read_varint_u64()? as usize,
        component_type: read_bin_component_type(r)?,
        normalized: r.read_u8()? != 0,
        count: r.read_varint_u64()? as usize,
        kind: read_bin_accessor_type(r)?,
        max: read_bin_option(r, read_bin_f64_vec)?,
        min: read_bin_option(r, read_bin_f64_vec)?,
        sparse: read_bin_option(r, read_bin_sparse_accessor)?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bin_accessor_diff(w: &mut dsl::ByteWriter, d: &GltfAccessorDiff) {
    write_bin_tri(w, &d.buffer_view, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &d.byte_offset, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &d.component_type, |w, v| write_bin_component_type(w, *v));
    write_bin_option(w, &d.normalized, |w, v| w.write_u8(if *v { 1 } else { 0 }));
    write_bin_option(w, &d.count, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &d.kind, |w, v| write_bin_accessor_type(w, *v));
    write_bin_tri(w, &d.max, |w, v| write_bin_f64_vec(w, v));
    write_bin_tri(w, &d.min, |w, v| write_bin_f64_vec(w, v));
    write_bin_tri(w, &d.sparse, write_bin_sparse_accessor);
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bin_accessor_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfAccessorDiff, dsl::PackRefusal> {
    Ok(GltfAccessorDiff {
        buffer_view: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        byte_offset: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        component_type: read_bin_option(r, read_bin_component_type)?,
        normalized: read_bin_option(r, |r| Ok(r.read_u8()? != 0))?,
        count: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        kind: read_bin_option(r, read_bin_accessor_type)?,
        max: read_bin_tri(r, read_bin_f64_vec)?,
        min: read_bin_tri(r, read_bin_f64_vec)?,
        sparse: read_bin_tri(r, read_bin_sparse_accessor)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
