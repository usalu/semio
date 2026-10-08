//! 💾️ Mesh GLB, glTF and binary STL representation owners.
use crate::*;
use semio_framework_pack_json as json;

//#region Glb
pub fn mesh_to_glb(mesh: &MeshData) -> Vec<u8> {
    let positions = f32_slice_to_bytes(&mesh.positions);
    let normals = if mesh.normals.len() == mesh.positions.len() {
        f32_slice_to_bytes(&mesh.normals)
    } else {
        let mut copy = mesh.clone();
        copy.compute_normals();
        f32_slice_to_bytes(&copy.normals)
    };
    let indices = u32_slice_to_bytes(&mesh.indices);
    let bin = [positions.as_slice(), normals.as_slice(), indices.as_slice()].concat();
    let padded_bin = pad_to_4(bin);
    let positions_len = positions.len();
    let normals_len = normals.len();
    let indices_len = indices.len();
    let positions_offset = 0usize;
    let normals_offset = positions_offset + positions_len;
    let indices_offset = normals_offset + normals_len;
    let json = format!(
        r#"{{
  "asset": {{"version": "2.0"}},
  "scene": 0,
  "scenes": [{{"nodes": [0]}}],
  "nodes": [{{"mesh": 0}}],
  "meshes": [{{
    "primitives": [{{
      "attributes": {{"POSITION": 0, "NORMAL": 1}},
      "indices": 2,
      "mode": 4
    }}]
  }}],
  "accessors": [
    {{"bufferView": 0, "componentType": 5126, "count": {}, "type": "VEC3", "min": {}, "max": {}}},
    {{"bufferView": 1, "componentType": 5126, "count": {}, "type": "VEC3"}},
    {{"bufferView": 2, "componentType": 5125, "count": {}, "type": "SCALAR"}}
  ],
  "bufferViews": [
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}}
  ],
  "buffers": [{{"byteLength": {}}}]
}}"#,
        mesh.vertex_count(),
        json_vec3_min(&mesh.positions),
        json_vec3_max(&mesh.positions),
        mesh.vertex_count(),
        mesh.indices.len(),
        positions_offset,
        positions_len,
        normals_offset,
        normals_len,
        indices_offset,
        indices_len,
        padded_bin.len()
    );
    let mut json_bytes = json.into_bytes();
    while json_bytes.len() % 4 != 0 {
        json_bytes.push(b' ');
    }
    let total_len = 12 + 8 + json_bytes.len() + 8 + padded_bin.len();
    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&(2u32).to_le_bytes());
    out.extend_from_slice(&(total_len as u32).to_le_bytes());
    out.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(&json_bytes);
    out.extend_from_slice(&(padded_bin.len() as u32).to_le_bytes());
    out.extend_from_slice(b"BIN\x00");
    out.extend_from_slice(&padded_bin);
    out
}

pub(crate) type GlbMatrix = [[f32; 4]; 4];

pub(crate) fn glb_identity() -> GlbMatrix {
    [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
}

pub(crate) fn glb_matrix_mul(left: GlbMatrix, right: GlbMatrix) -> GlbMatrix {
    let mut result = [[0.0; 4]; 4];
    for column in 0..4 {
        for row in 0..4 {
            result[column][row] = (0..4).map(|axis| left[axis][row] * right[column][axis]).sum();
        }
    }
    result
}

pub(crate) fn glb_transform_point(matrix: GlbMatrix, point: [f32; 3]) -> [f32; 3] {
    [
        matrix[0][0] * point[0] + matrix[1][0] * point[1] + matrix[2][0] * point[2] + matrix[3][0],
        matrix[0][1] * point[0] + matrix[1][1] * point[1] + matrix[2][1] * point[2] + matrix[3][1],
        matrix[0][2] * point[0] + matrix[1][2] * point[1] + matrix[2][2] * point[2] + matrix[3][2],
    ]
}

pub(crate) fn glb_transform_normal(matrix: GlbMatrix, normal: [f32; 3]) -> [f32; 3] {
    let (a00, a01, a02) = (matrix[0][0], matrix[1][0], matrix[2][0]);
    let (a10, a11, a12) = (matrix[0][1], matrix[1][1], matrix[2][1]);
    let (a20, a21, a22) = (matrix[0][2], matrix[1][2], matrix[2][2]);
    let det = a00 * (a11 * a22 - a12 * a21) - a01 * (a10 * a22 - a12 * a20) + a02 * (a10 * a21 - a11 * a20);
    if det.abs() <= f32::EPSILON {
        return normal;
    }
    let inverse_det = det.recip();
    let transformed = [
        ((a11 * a22 - a12 * a21) * normal[0] + (a12 * a20 - a10 * a22) * normal[1] + (a10 * a21 - a11 * a20) * normal[2]) * inverse_det,
        ((a02 * a21 - a01 * a22) * normal[0] + (a00 * a22 - a02 * a20) * normal[1] + (a01 * a20 - a00 * a21) * normal[2]) * inverse_det,
        ((a01 * a12 - a02 * a11) * normal[0] + (a02 * a10 - a00 * a12) * normal[1] + (a00 * a11 - a01 * a10) * normal[2]) * inverse_det,
    ];
    let length = transformed.iter().map(|value| value * value).sum::<f32>().sqrt();
    if length <= f32::EPSILON {
        normal
    } else {
        transformed.map(|value| value / length)
    }
}

//#region 🔖️GltfCodec
/// 🧊️ First-party glTF 2.0 container split: `.glb` binary (magic/version/chunk walk) or bare
/// `.gltf` JSON text (detected by a leading `{`) with no binary chunk. Replaces the `gltf` crate
/// per ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS — mirrors the
/// byte-for-byte semantics of the stdio gltf artifact's own `decode_glb`
/// (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🦀️.rs`),
/// re-expressed against `pack::json` instead of `serde_json` since this is the framework, not
/// that artifact's own mutation-schema codec.
fn gltf_split_container(bytes: &[u8]) -> Result<(Vec<u8>, Option<Vec<u8>>), String> {
    if bytes.first() == Some(&b'{') {
        return Ok((bytes.to_vec(), None));
    }
    if bytes.len() < 12 || &bytes[0..4] != b"glTF" {
        return Err("glb: bad magic, expected 'glTF' or '{'".into());
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    if version != 2 {
        return Err(format!("glb: unsupported version {version}, only 2 is supported"));
    }
    let total_len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let mut pos = 12usize;
    let mut json_chunk: Option<Vec<u8>> = None;
    let mut bin_chunk: Option<Vec<u8>> = None;
    while pos + 8 <= bytes.len() && pos < total_len {
        let chunk_len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
        let chunk_type = &bytes[pos + 4..pos + 8];
        let data_start = pos + 8;
        let data_end = data_start + chunk_len;
        if data_end > bytes.len() {
            return Err("glb: chunk length exceeds buffer".into());
        }
        if chunk_type == b"JSON" && json_chunk.is_none() {
            json_chunk = Some(bytes[data_start..data_end].to_vec());
        } else if chunk_type == b"BIN\0" && bin_chunk.is_none() {
            bin_chunk = Some(bytes[data_start..data_end].to_vec());
        }
        pos = data_end;
    }
    Ok((json_chunk.ok_or_else(|| "glb: missing JSON chunk".to_string())?, bin_chunk))
}

/// 🔓️ Decodes a `data:...;base64,...` uri through the framework's own base64 codec — external
/// (file-path) buffer uris are left unresolved (empty bytes), same contract as the stdio gltf
/// artifact's `resolve_document_buffers`: this engine has no filesystem/network access.
fn gltf_decode_data_uri(uri: &str) -> Result<Vec<u8>, String> {
    if !uri.starts_with("data:") {
        return Err("gltf: unsupported external buffer uri (no filesystem access)".into());
    }
    let marker = ";base64,";
    let idx = uri.find(marker).ok_or_else(|| "gltf: unsupported non-base64 data uri".to_string())?;
    semio_framework_io_base64::base64_standard_decode(&uri[idx + marker.len()..]).map_err(|error| error.to_string())
}

/// 📦️ Resolves `document.buffers[i]` to raw bytes, index-aligned with the JSON array. Only
/// `buffers[0]` may omit `uri` and be sourced from the `.glb` BIN chunk, per spec.
fn gltf_resolve_buffers(document: &json::Value, embedded_bin: Option<&[u8]>) -> Vec<Vec<u8>> {
    document
        .get("buffers")
        .and_then(json::Value::as_array)
        .map_or(&[][..], Vec::as_slice)
        .iter()
        .enumerate()
        .map(|(i, buffer)| match buffer.get("uri").and_then(json::Value::as_str) {
            Some(uri) => gltf_decode_data_uri(uri).unwrap_or_default(),
            None if i == 0 => embedded_bin.map(<[u8]>::to_vec).unwrap_or_default(),
            None => Vec::new(),
        })
        .collect()
}

fn gltf_component_byte_size(component_type: u64) -> Result<usize, String> {
    Ok(match component_type {
        5120 | 5121 => 1,
        5122 | 5123 => 2,
        5125 | 5126 => 4,
        other => return Err(format!("gltf: unsupported accessor.componentType {other}")),
    })
}

fn gltf_read_component(component_type: u64, bytes: &[u8], offset: usize) -> Result<f64, String> {
    let size = gltf_component_byte_size(component_type)?;
    if offset + size > bytes.len() {
        return Err("gltf: accessor component read out of buffer bounds".into());
    }
    Ok(match component_type {
        5120 => bytes[offset] as i8 as f64,
        5121 => bytes[offset] as f64,
        5122 => i16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as f64,
        5123 => u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as f64,
        5125 => u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as f64,
        5126 => f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as f64,
        other => return Err(format!("gltf: unsupported accessor.componentType {other}")),
    })
}

fn gltf_accessor_type_components(kind: &str) -> Result<usize, String> {
    Ok(match kind {
        "SCALAR" => 1,
        "VEC2" => 2,
        "VEC3" => 3,
        "VEC4" | "MAT2" => 4,
        "MAT3" => 9,
        "MAT4" => 16,
        other => return Err(format!("gltf: unsupported accessor.type {other:?}")),
    })
}

fn gltf_read_elements(bytes: &[u8], base_offset: usize, component_type: u64, accessor_type: &str, count: usize, byte_stride: Option<usize>) -> Result<Vec<f64>, String> {
    let nc = gltf_accessor_type_components(accessor_type)?;
    let component_size = gltf_component_byte_size(component_type)?;
    let stride = byte_stride.unwrap_or(component_size * nc);
    let mut out = Vec::with_capacity(count * nc);
    for i in 0..count {
        let elem_off = base_offset + i * stride;
        for c in 0..nc {
            out.push(gltf_read_component(component_type, bytes, elem_off + c * component_size)?);
        }
    }
    Ok(out)
}

/// 🎚️ Applies glTF 2.0 accessor normalization (§3.6.2.2) after dense/sparse values assembled.
fn gltf_normalize_components(component_type: u64, components: &mut [f64]) -> Result<(), String> {
    let (scale, signed) = match component_type {
        5120 => (127.0, true),
        5121 => (255.0, false),
        5122 => (32_767.0, true),
        5123 => (65_535.0, false),
        5125 => (4_294_967_295.0, false),
        5126 => return Err("gltf: normalized FLOAT accessor is invalid glTF 2.0".into()),
        other => return Err(format!("gltf: unsupported accessor.componentType {other}")),
    };
    for value in components {
        *value = if signed { (*value / scale).max(-1.0) } else { *value / scale };
    }
    Ok(())
}

fn gltf_read_bufferview_elements(document: &json::Value, buffers: &[Vec<u8>], bv_idx: usize, extra_offset: usize, component_type: u64, accessor_type: &str, count: usize) -> Result<Vec<f64>, String> {
    let bv = document.get("bufferViews").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice).get(bv_idx).ok_or_else(|| format!("gltf: bufferView index {bv_idx} out of range"))?;
    let buffer_index = bv.get("buffer").and_then(json::Value::as_u64).unwrap_or(0) as usize;
    let byte_offset = bv.get("byteOffset").and_then(json::Value::as_u64).unwrap_or(0) as usize;
    let byte_stride = bv.get("byteStride").and_then(json::Value::as_u64).map(|value| value as usize);
    let bytes = buffers.get(buffer_index).ok_or_else(|| format!("gltf: buffer index {buffer_index} out of range"))?;
    if bytes.is_empty() {
        return Err(format!("gltf: buffer {buffer_index} bytes unavailable (external uri not resolvable, or empty embedded buffer)"));
    }
    gltf_read_elements(bytes, byte_offset + extra_offset, component_type, accessor_type, count, byte_stride)
}

/// 🧩️ Decodes `document.accessors[accessor_index]` against `buffers` — dense `bufferView` read,
/// then `accessor.sparse` substitution (base is zero-filled when there's no `bufferView`).
fn gltf_decode_accessor(document: &json::Value, buffers: &[Vec<u8>], accessor_index: usize) -> Result<Vec<f64>, String> {
    let accessor = document.get("accessors").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice).get(accessor_index).ok_or_else(|| format!("gltf: accessor index {accessor_index} out of range"))?;
    let component_type = accessor.get("componentType").and_then(json::Value::as_u64).ok_or_else(|| "gltf: accessor missing componentType".to_string())?;
    let accessor_type = accessor.get("type").and_then(json::Value::as_str).ok_or_else(|| "gltf: accessor missing type".to_string())?;
    let count = accessor.get("count").and_then(json::Value::as_u64).unwrap_or(0) as usize;
    let normalized = accessor.get("normalized").and_then(json::Value::as_bool).unwrap_or(false);
    let nc = gltf_accessor_type_components(accessor_type)?;

    let mut components = vec![0.0f64; count * nc];
    if let Some(bv_idx) = accessor.get("bufferView").and_then(json::Value::as_u64) {
        let extra_offset = accessor.get("byteOffset").and_then(json::Value::as_u64).unwrap_or(0) as usize;
        components = gltf_read_bufferview_elements(document, buffers, bv_idx as usize, extra_offset, component_type, accessor_type, count)?;
    }

    if let Some(sparse) = accessor.get("sparse") {
        let sparse_count = sparse.get("count").and_then(json::Value::as_u64).unwrap_or(0) as usize;
        let indices = sparse.get("indices").ok_or_else(|| "gltf: sparse accessor missing indices".to_string())?;
        let values = sparse.get("values").ok_or_else(|| "gltf: sparse accessor missing values".to_string())?;
        let indices_bv = indices.get("bufferView").and_then(json::Value::as_u64).ok_or_else(|| "gltf: sparse indices missing bufferView".to_string())? as usize;
        let indices_offset = indices.get("byteOffset").and_then(json::Value::as_u64).unwrap_or(0) as usize;
        let indices_component = indices.get("componentType").and_then(json::Value::as_u64).ok_or_else(|| "gltf: sparse indices missing componentType".to_string())?;
        let values_bv = values.get("bufferView").and_then(json::Value::as_u64).ok_or_else(|| "gltf: sparse values missing bufferView".to_string())? as usize;
        let values_offset = values.get("byteOffset").and_then(json::Value::as_u64).unwrap_or(0) as usize;

        let idx_values = gltf_read_bufferview_elements(document, buffers, indices_bv, indices_offset, indices_component, "SCALAR", sparse_count)?;
        let val_values = gltf_read_bufferview_elements(document, buffers, values_bv, values_offset, component_type, accessor_type, sparse_count)?;
        for i in 0..sparse_count {
            let idx = idx_values[i] as usize;
            let dst = idx * nc;
            if dst + nc > components.len() {
                return Err(format!("gltf: sparse accessor index {idx} out of range for count {count}"));
            }
            components[dst..dst + nc].copy_from_slice(&val_values[i * nc..i * nc + nc]);
        }
    }

    if normalized {
        gltf_normalize_components(component_type, &mut components)?;
    }
    Ok(components)
}

fn gltf_node_vec3(node: &json::Value, key: &str, default: [f32; 3]) -> [f32; 3] {
    node.get(key)
        .and_then(json::Value::as_array)
        .filter(|values| values.len() == 3)
        .map_or(default, |values| [values[0].as_f64().unwrap_or(default[0] as f64) as f32, values[1].as_f64().unwrap_or(default[1] as f64) as f32, values[2].as_f64().unwrap_or(default[2] as f64) as f32])
}

fn gltf_node_quat(node: &json::Value) -> [f32; 4] {
    node.get("rotation")
        .and_then(json::Value::as_array)
        .filter(|values| values.len() == 4)
        .map_or([0.0, 0.0, 0.0, 1.0], |values| [values[0].as_f64().unwrap_or(0.0) as f32, values[1].as_f64().unwrap_or(0.0) as f32, values[2].as_f64().unwrap_or(0.0) as f32, values[3].as_f64().unwrap_or(1.0) as f32])
}

/// 🧮️ `T * R * S` node transform per glTF 2.0 §5.25 — quaternion-to-rotation composed with
/// translation/scale, laid out column-major to match this file's `GlbMatrix` convention.
fn gltf_trs_matrix(t: [f32; 3], r: [f32; 4], s: [f32; 3]) -> GlbMatrix {
    let (x, y, z, w) = (r[0], r[1], r[2], r[3]);
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    [
        [(1.0 - (yy + zz)) * s[0], (xy + wz) * s[0], (xz - wy) * s[0], 0.0],
        [(xy - wz) * s[1], (1.0 - (xx + zz)) * s[1], (yz + wx) * s[1], 0.0],
        [(xz + wy) * s[2], (yz - wx) * s[2], (1.0 - (xx + yy)) * s[2], 0.0],
        [t[0], t[1], t[2], 1.0],
    ]
}

/// 🧮️ A node's own local matrix: explicit `matrix` (already column-major, 16 floats) takes
/// precedence over `translation`/`rotation`/`scale` per spec.
fn gltf_node_local_matrix(node: &json::Value) -> GlbMatrix {
    if let Some(m) = node.get("matrix").and_then(json::Value::as_array).filter(|values| values.len() == 16) {
        let f: Vec<f32> = m.iter().map(|value| value.as_f64().unwrap_or(0.0) as f32).collect();
        return [[f[0], f[1], f[2], f[3]], [f[4], f[5], f[6], f[7]], [f[8], f[9], f[10], f[11]], [f[12], f[13], f[14], f[15]]];
    }
    gltf_trs_matrix(gltf_node_vec3(node, "translation", [0.0, 0.0, 0.0]), gltf_node_quat(node), gltf_node_vec3(node, "scale", [1.0, 1.0, 1.0]))
}

fn gltf_triangle_indices(mode: u64, source: Vec<u32>) -> Vec<u32> {
    match mode {
        4 => source,
        5 => source.windows(3).enumerate().flat_map(|(index, tri)| if index % 2 == 0 { [tri[0], tri[1], tri[2]] } else { [tri[1], tri[0], tri[2]] }).collect(),
        6 => source.first().map(|first| source[1..].windows(2).flat_map(|pair| [*first, pair[0], pair[1]]).collect()).unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn gltf_append_primitive(mesh: &mut MeshData, document: &json::Value, primitive: &json::Value, buffers: &[Vec<u8>], matrix: GlbMatrix) -> Result<(), String> {
    let mode = primitive.get("mode").and_then(json::Value::as_u64).unwrap_or(4);
    if !matches!(mode, 4..=6) {
        return Ok(());
    }
    let attributes = primitive.get("attributes").ok_or_else(|| "gltf: primitive missing attributes".to_string())?;
    let position_accessor = attributes.get("POSITION").and_then(json::Value::as_u64).ok_or_else(|| "glb triangle primitive missing POSITION".to_string())? as usize;
    let positions: Vec<[f32; 3]> = gltf_decode_accessor(document, buffers, position_accessor)?.as_chunks::<3>().0.iter().map(|c| [c[0] as f32, c[1] as f32, c[2] as f32]).collect();

    let source_indices: Vec<u32> = if let Some(indices_accessor) = primitive.get("indices").and_then(json::Value::as_u64) {
        gltf_decode_accessor(document, buffers, indices_accessor as usize)?.into_iter().map(|value| value as u32).collect()
    } else {
        (0..positions.len() as u32).collect()
    };
    let indices = gltf_triangle_indices(mode, source_indices);
    if indices.iter().any(|index| *index as usize >= positions.len()) {
        return Err("glb triangle index outside POSITION accessor".into());
    }
    let normals: Vec<[f32; 3]> = if let Some(normal_accessor) = attributes.get("NORMAL").and_then(json::Value::as_u64) {
        gltf_decode_accessor(document, buffers, normal_accessor as usize)?.as_chunks::<3>().0.iter().map(|c| [c[0] as f32, c[1] as f32, c[2] as f32]).collect()
    } else {
        let mut local = MeshData { positions: positions.iter().flatten().copied().collect(), indices: indices.clone(), ..Default::default() };
        local.compute_normals();
        local.normals.as_chunks::<3>().0.to_vec()
    };
    if normals.len() != positions.len() {
        return Err("glb NORMAL and POSITION accessor counts differ".into());
    }
    let vertex_offset = mesh.vertex_count() as u32;
    for position in positions {
        mesh.positions.extend(glb_transform_point(matrix, position));
    }
    for normal in normals {
        mesh.normals.extend(glb_transform_normal(matrix, normal));
    }
    mesh.indices.extend(indices.into_iter().map(|index| vertex_offset + index));
    Ok(())
}

fn gltf_append_mesh(mesh: &mut MeshData, document: &json::Value, mesh_index: usize, buffers: &[Vec<u8>], matrix: GlbMatrix) -> Result<(), String> {
    let source = document.get("meshes").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice).get(mesh_index).ok_or_else(|| format!("gltf: mesh index {mesh_index} out of range"))?;
    for primitive in source.get("primitives").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice) {
        gltf_append_primitive(mesh, document, primitive, buffers, matrix)?;
    }
    Ok(())
}

fn gltf_append_node(mesh: &mut MeshData, document: &json::Value, node_index: usize, parent: GlbMatrix, buffers: &[Vec<u8>]) -> Result<(), String> {
    let node = document.get("nodes").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice).get(node_index).ok_or_else(|| format!("gltf: node index {node_index} out of range"))?;
    let matrix = glb_matrix_mul(parent, gltf_node_local_matrix(node));
    if let Some(mesh_index) = node.get("mesh").and_then(json::Value::as_u64) {
        gltf_append_mesh(mesh, document, mesh_index as usize, buffers, matrix)?;
    }
    for child in node.get("children").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice) {
        if let Some(child_index) = child.as_u64() {
            gltf_append_node(mesh, document, child_index as usize, matrix, buffers)?;
        }
    }
    Ok(())
}

/// 🧊️ Decodes every triangle primitive in the active GLB/glTF scene into one renderer-neutral
/// mesh, via this crate's own glTF 2.0 codec (`pack::json` + `semio_framework_io_base64`) —
/// never the `gltf` crate, which survives only as a `[dev-dependencies]` differential-test oracle.
pub fn mesh_from_glb(bytes: &[u8]) -> Result<MeshData, String> {
    let (json_bytes, bin) = gltf_split_container(bytes)?;
    let text = std::str::from_utf8(&json_bytes).map_err(|error| format!("gltf json is not valid utf-8: {error}"))?;
    let document = json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("gltf json parse error: {error}"))?;
    let buffers = gltf_resolve_buffers(&document, bin.as_deref());
    let mut mesh = MeshData::default();

    let scene_index = document.get("scene").and_then(json::Value::as_u64).map(|value| value as usize);
    let scenes = document.get("scenes").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice);
    let scene = scene_index.and_then(|index| scenes.get(index)).or_else(|| scenes.first());

    if let Some(scene) = scene {
        for node in scene.get("nodes").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice) {
            if let Some(node_index) = node.as_u64() {
                gltf_append_node(&mut mesh, &document, node_index as usize, glb_identity(), &buffers)?;
            }
        }
    } else {
        let mesh_count = document.get("meshes").and_then(json::Value::as_array).map_or(0, Vec::len);
        for mesh_index in 0..mesh_count {
            gltf_append_mesh(&mut mesh, &document, mesh_index, &buffers, glb_identity())?;
        }
    }

    if mesh.indices.is_empty() {
        return Err("glb contains no triangle primitives".into());
    }
    Ok(mesh)
}
//#endregion 🔖️GltfCodec

fn f32_slice_to_bytes(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn u32_slice_to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn pad_to_4(mut data: Vec<u8>) -> Vec<u8> {
    while !data.len().is_multiple_of(4) {
        data.push(0);
    }
    data
}

fn json_vec3_min(positions: &[f32]) -> String {
    let (min, _) = MeshData { positions: positions.to_vec(), ..Default::default() }.aabb();
    format!("[{}, {}, {}]", min[0], min[1], min[2])
}

fn json_vec3_max(positions: &[f32]) -> String {
    let (_, max) = MeshData { positions: positions.to_vec(), ..Default::default() }.aabb();
    format!("[{}, {}, {}]", max[0], max[1], max[2])
}
//#endregion Glb

//#region Stl
/// 🧱️ Hand-rolled binary STL: 80-byte header, `u32` little-endian triangle count, then per triangle a `f32x3` facet normal, three `f32x3` vertices, and a `u16` attribute-byte-count (written as 0). No vertex dedupe, matching the binary STL convention of one independent triangle per record.
pub fn mesh_to_stl(mesh: &MeshData) -> Vec<u8> {
    let triangle_count = mesh.triangle_count() as u32;
    let mut out = Vec::with_capacity(80 + 4 + triangle_count as usize * 50);
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&triangle_count.to_le_bytes());
    for tri in mesh.indices.as_chunks::<3>().0 {
        let p0 = stl_vertex(&mesh.positions, tri[0]);
        let p1 = stl_vertex(&mesh.positions, tri[1]);
        let p2 = stl_vertex(&mesh.positions, tri[2]);
        let normal = stl_face_normal(p0, p1, p2);
        for component in normal {
            out.extend_from_slice(&component.to_le_bytes());
        }
        for vertex in [p0, p1, p2] {
            for component in vertex {
                out.extend_from_slice(&component.to_le_bytes());
            }
        }
        out.extend_from_slice(&0u16.to_le_bytes());
    }
    out
}

pub fn mesh_from_stl(bytes: &[u8]) -> Result<MeshData, String> {
    if bytes.len() < 84 {
        return Err("stl: truncated header".into());
    }
    let triangle_count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    let expected_len = 84 + triangle_count * 50;
    if bytes.len() < expected_len {
        return Err("stl: truncated triangle data".into());
    }
    let mut mesh = MeshData::default();
    for triangle in 0..triangle_count {
        let base = 84 + triangle * 50;
        let mut normal = [0f32; 3];
        for axis in 0..3 {
            normal[axis] = f32::from_le_bytes(bytes[base + axis * 4..base + axis * 4 + 4].try_into().unwrap());
        }
        let vertex_base = base + 12;
        for corner in 0..3 {
            let corner_base = vertex_base + corner * 12;
            let mut position = [0f32; 3];
            for axis in 0..3 {
                position[axis] = f32::from_le_bytes(bytes[corner_base + axis * 4..corner_base + axis * 4 + 4].try_into().unwrap());
            }
            let index = (mesh.positions.len() / 3) as u32;
            mesh.positions.extend_from_slice(&position);
            mesh.normals.extend_from_slice(&normal);
            mesh.indices.push(index);
        }
    }
    Ok(mesh)
}

fn stl_vertex(positions: &[f32], index: u32) -> [f32; 3] {
    let base = index as usize * 3;
    [positions[base], positions[base + 1], positions[base + 2]]
}

fn stl_face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let n = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > 1e-8 {
        [n[0] / len, n[1] / len, n[2] / len]
    } else {
        [0.0, 0.0, 0.0]
    }
}
//#endregion Stl

