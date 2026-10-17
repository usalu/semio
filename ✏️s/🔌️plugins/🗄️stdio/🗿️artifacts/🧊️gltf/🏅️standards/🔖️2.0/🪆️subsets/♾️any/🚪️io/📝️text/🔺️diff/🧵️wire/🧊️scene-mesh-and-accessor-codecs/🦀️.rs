//! 🧊️ Text codecs of the asset, scene, node, mesh, sparse and accessor elements and their diffs.

use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfAccessor, standards::v2_0::subsets::any::schema::snapshot::GltfMesh, standards::v2_0::subsets::any::schema::snapshot::GltfMorphTarget, standards::v2_0::subsets::any::schema::snapshot::GltfNode, standards::v2_0::subsets::any::schema::snapshot::GltfPrimitive, standards::v2_0::subsets::any::schema::snapshot::GltfScene, standards::v2_0::subsets::any::schema::snapshot::GltfSparseAccessor, standards::v2_0::subsets::any::schema::snapshot::GltfSparseIndices, standards::v2_0::subsets::any::schema::snapshot::GltfSparseValues};
use crate::standards::v2_0::subsets::any::io::text::diff::wire::*;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_asset_diff(d: &GltfAssetDiff) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        encode_option(&d.version, |v| enc_str(v)),
        encode_option_option(&d.generator, |v| enc_str(v)),
        encode_option_option(&d.copyright, |v| enc_str(v)),
        encode_option_option(&d.min_version, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_asset_diff(s: &str) -> Result<GltfAssetDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [version, generator, copyright, min_version, extensions, extras] = parts.as_slice() else {
        return Err(format!("asset diff: expected 6 fields, got {}", parts.len()));
    };
    Ok(GltfAssetDiff {
        version: decode_option(version, dec_str)?,
        generator: decode_option_option(generator, dec_str)?,
        copyright: decode_option_option(copyright, dec_str)?,
        min_version: decode_option_option(min_version, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_scene(sc: &GltfScene) -> String {
    format!("[{},{},{},{}]", enc_usize_vec(&sc.nodes), encode_option(&sc.name, |v| enc_str(v)), encode_option(&sc.extensions, enc_json), encode_option(&sc.extras, enc_json),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_scene(s: &str) -> Result<GltfScene, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [nodes, name, extensions, extras] = parts.as_slice() else { return Err(format!("scene: expected 4 fields, got {}", parts.len())) };
    Ok(GltfScene { nodes: dec_usize_vec(nodes)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_scene_diff(d: &GltfSceneDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.nodes, enc_refs_delta), encode_option_option(&d.name, |v| enc_str(v)), encode_option_option(&d.extensions, enc_json), encode_option_option(&d.extras, enc_json),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_scene_diff(s: &str) -> Result<GltfSceneDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [nodes, name, extensions, extras] = parts.as_slice() else { return Err(format!("scene diff: expected 4 fields, got {}", parts.len())) };
    Ok(GltfSceneDiff { nodes: decode_option(nodes, dec_refs_delta)?, name: decode_option_option(name, dec_str)?, extensions: decode_option_option(extensions, dec_json)?, extras: decode_option_option(extras, dec_json)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node(n: &GltfNode) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        enc_usize_vec(&n.children),
        encode_option(&n.mesh, |v| v.to_string()),
        encode_option(&n.camera, |v| v.to_string()),
        encode_option(&n.skin, |v| v.to_string()),
        encode_option(&n.matrix, |v| enc_f64_slice(v)),
        encode_option(&n.translation, |v| enc_f64_slice(v)),
        encode_option(&n.rotation, |v| enc_f64_slice(v)),
        encode_option(&n.scale, |v| enc_f64_slice(v)),
        enc_f64_slice(&n.weights),
        encode_option(&n.name, |v| enc_str(v)),
        encode_option(&n.extensions, enc_json),
        encode_option(&n.extras, enc_json),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node(s: &str) -> Result<GltfNode, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [children, mesh, camera, skin, matrix, translation, rotation, scale, weights, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("node: expected 12 fields, got {}", parts.len()));
    };
    Ok(GltfNode {
        children: dec_usize_vec(children)?,
        mesh: decode_option(mesh, parse_usize)?,
        camera: decode_option(camera, parse_usize)?,
        skin: decode_option(skin, parse_usize)?,
        matrix: decode_option(matrix, dec_f64_array::<16>)?,
        translation: decode_option(translation, dec_f64_array::<3>)?,
        rotation: decode_option(rotation, dec_f64_array::<4>)?,
        scale: decode_option(scale, dec_f64_array::<3>)?,
        weights: dec_f64_vec(weights)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_diff(d: &GltfNodeDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option(&d.children, enc_refs_delta),
        encode_option_option(&d.mesh, |v| v.to_string()),
        encode_option_option(&d.camera, |v| v.to_string()),
        encode_option_option(&d.skin, |v| v.to_string()),
        encode_option_option(&d.matrix, |v| enc_f64_slice(v)),
        encode_option_option(&d.translation, |v| enc_f64_slice(v)),
        encode_option_option(&d.rotation, |v| enc_f64_slice(v)),
        encode_option_option(&d.scale, |v| enc_f64_slice(v)),
        encode_option(&d.weights, |v| enc_f64_slice(v)),
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_diff(s: &str) -> Result<GltfNodeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [children, mesh, camera, skin, matrix, translation, rotation, scale, weights, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("node diff: expected 12 fields, got {}", parts.len()));
    };
    Ok(GltfNodeDiff {
        children: decode_option(children, dec_refs_delta)?,
        mesh: decode_option_option(mesh, parse_usize)?,
        camera: decode_option_option(camera, parse_usize)?,
        skin: decode_option_option(skin, parse_usize)?,
        matrix: decode_option_option(matrix, dec_f64_array::<16>)?,
        translation: decode_option_option(translation, dec_f64_array::<3>)?,
        rotation: decode_option_option(rotation, dec_f64_array::<4>)?,
        scale: decode_option_option(scale, dec_f64_array::<3>)?,
        weights: decode_option(weights, dec_f64_vec)?,
        name: decode_option_option(name, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_primitive(p: &GltfPrimitive) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        enc_attr_pairs(&p.attributes),
        encode_option(&p.indices, |v| v.to_string()),
        encode_option(&p.material, |v| v.to_string()),
        encode_option(&p.mode, |v| enc_u64(*v)),
        format_args!("[{}]", p.targets.iter().map(|target| enc_attr_pairs(&target.0)).collect::<Vec<_>>().join(",")),
        encode_option(&p.extensions, enc_json),
        encode_option(&p.extras, enc_json),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_primitive(s: &str) -> Result<GltfPrimitive, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [attributes, indices, material, mode, targets, extensions, extras] = parts.as_slice() else {
        return Err(format!("primitive: expected 7 fields, got {}", parts.len()));
    };
    Ok(GltfPrimitive {
        attributes: dec_attr_pairs(attributes)?,
        indices: decode_option(indices, parse_usize)?,
        material: decode_option(material, parse_usize)?,
        mode: decode_option(mode, dec_u64)?,
        targets: split_top_level(strip_brackets(targets)?, ',').into_iter().filter(|value| !value.is_empty()).map(|value| dec_attr_pairs(value).map(GltfMorphTarget)).collect::<Result<Vec<_>, _>>()?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_primitive_vec(v: &[GltfPrimitive]) -> String {
    format!("[{}]", v.iter().map(enc_primitive).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_primitive_vec(s: &str) -> Result<Vec<GltfPrimitive>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_primitive).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mesh(m: &GltfMesh) -> String {
    format!("[{},{},{},{},{}]", enc_primitive_vec(&m.primitives), enc_f64_slice(&m.weights), encode_option(&m.name, |v| enc_str(v)), encode_option(&m.extensions, enc_json), encode_option(&m.extras, enc_json),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mesh(s: &str) -> Result<GltfMesh, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [primitives, weights, name, extensions, extras] = parts.as_slice() else { return Err(format!("mesh: expected 5 fields, got {}", parts.len())) };
    Ok(GltfMesh { primitives: dec_primitive_vec(primitives)?, weights: dec_f64_vec(weights)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mesh_diff(d: &GltfMeshDiff) -> String {
    format!(
        "[{},{},{},{},{}]",
        encode_option(&d.primitives, |v| enc_collection(v, enc_primitive, enc_primitive_diff)),
        encode_option(&d.weights, |v| enc_f64_slice(v)),
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mesh_diff(s: &str) -> Result<GltfMeshDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [primitives, weights, name, extensions, extras] = parts.as_slice() else { return Err(format!("mesh diff: expected 5 fields, got {}", parts.len())) };
    Ok(GltfMeshDiff {
        primitives: decode_option(primitives, |s| dec_collection(s, dec_primitive, dec_primitive_diff))?,
        weights: decode_option(weights, dec_f64_vec)?,
        name: decode_option_option(name, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sparse_indices(v: &GltfSparseIndices) -> String {
    format!("[{},{},{}]", v.buffer_view, v.byte_offset, enc_component_type(v.component_type))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sparse_indices(s: &str) -> Result<GltfSparseIndices, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer_view, byte_offset, component_type] = parts.as_slice() else { return Err(format!("sparse indices: expected 3 fields, got {}", parts.len())) };
    Ok(GltfSparseIndices { buffer_view: parse_usize(buffer_view)?, byte_offset: parse_usize(byte_offset)?, component_type: dec_component_type(component_type)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sparse_values(v: &GltfSparseValues) -> String {
    format!("[{},{}]", v.buffer_view, v.byte_offset)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sparse_values(s: &str) -> Result<GltfSparseValues, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer_view, byte_offset] = parts.as_slice() else { return Err(format!("sparse values: expected 2 fields, got {}", parts.len())) };
    Ok(GltfSparseValues { buffer_view: parse_usize(buffer_view)?, byte_offset: parse_usize(byte_offset)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sparse_accessor(v: &GltfSparseAccessor) -> String {
    format!("[{},{},{}]", v.count, enc_sparse_indices(&v.indices), enc_sparse_values(&v.values))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sparse_accessor(s: &str) -> Result<GltfSparseAccessor, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [count, indices, values] = parts.as_slice() else { return Err(format!("sparse accessor: expected 3 fields, got {}", parts.len())) };
    Ok(GltfSparseAccessor { count: parse_usize(count)?, indices: dec_sparse_indices(indices)?, values: dec_sparse_values(values)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_accessor(a: &GltfAccessor) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option(&a.buffer_view, |v| v.to_string()),
        a.byte_offset,
        enc_component_type(a.component_type),
        enc_bool(a.normalized),
        a.count,
        enc_accessor_type(a.kind),
        encode_option(&a.max, |v| enc_f64_slice(v)),
        encode_option(&a.min, |v| enc_f64_slice(v)),
        encode_option(&a.sparse, enc_sparse_accessor),
        encode_option(&a.name, |v| enc_str(v)),
        encode_option(&a.extensions, enc_json),
        encode_option(&a.extras, enc_json),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_accessor(s: &str) -> Result<GltfAccessor, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer_view, byte_offset, component_type, normalized, count, kind, max, min, sparse, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("accessor: expected 12 fields, got {}", parts.len()));
    };
    Ok(GltfAccessor {
        buffer_view: decode_option(buffer_view, parse_usize)?,
        byte_offset: parse_usize(byte_offset)?,
        component_type: dec_component_type(component_type)?,
        normalized: dec_bool(normalized)?,
        count: parse_usize(count)?,
        kind: dec_accessor_type(kind)?,
        max: decode_option(max, dec_f64_vec)?,
        min: decode_option(min, dec_f64_vec)?,
        sparse: decode_option(sparse, dec_sparse_accessor)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_accessor_diff(d: &GltfAccessorDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option_option(&d.buffer_view, |v| v.to_string()),
        encode_option(&d.byte_offset, |v| v.to_string()),
        encode_option(&d.component_type, |v| enc_component_type(*v)),
        encode_option(&d.normalized, |v| enc_bool(*v)),
        encode_option(&d.count, |v| v.to_string()),
        encode_option(&d.kind, |v| enc_accessor_type(*v)),
        encode_option_option(&d.max, |v| enc_f64_slice(v)),
        encode_option_option(&d.min, |v| enc_f64_slice(v)),
        encode_option_option(&d.sparse, enc_sparse_accessor),
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_accessor_diff(s: &str) -> Result<GltfAccessorDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer_view, byte_offset, component_type, normalized, count, kind, max, min, sparse, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("accessor diff: expected 12 fields, got {}", parts.len()));
    };
    Ok(GltfAccessorDiff {
        buffer_view: decode_option_option(buffer_view, parse_usize)?,
        byte_offset: decode_option(byte_offset, parse_usize)?,
        component_type: decode_option(component_type, dec_component_type)?,
        normalized: decode_option(normalized, dec_bool)?,
        count: decode_option(count, parse_usize)?,
        kind: decode_option(kind, dec_accessor_type)?,
        max: decode_option_option(max, dec_f64_vec)?,
        min: decode_option_option(min, dec_f64_vec)?,
        sparse: decode_option_option(sparse, dec_sparse_accessor)?,
        name: decode_option_option(name, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}
