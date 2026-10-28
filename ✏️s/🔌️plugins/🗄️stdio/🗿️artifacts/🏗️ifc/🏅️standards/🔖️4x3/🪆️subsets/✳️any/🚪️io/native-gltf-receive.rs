//! 📥️ Geometry-only natural receiving writes actual glTF rows directly into retained typed storage.
use super::*;
use json::Catalog;
pub(super) fn empty() -> GltfSnapshot {
    GltfSnapshot {
        schema: String::new(),
        document: GltfDocument {
            asset: GltfAsset { version: String::new(), generator: None, copyright: None, min_version: None, extensions: None, extras: None },
            scene: None,
            scenes: Vec::new(),
            nodes: Vec::new(),
            meshes: Vec::new(),
            accessors: Vec::new(),
            buffer_views: Vec::new(),
            buffers: Vec::new(),
            materials: Vec::new(),
            textures: Vec::new(),
            images: Vec::new(),
            samplers: Vec::new(),
            skins: Vec::new(),
            animations: Vec::new(),
            cameras: Vec::new(),
            extensions_used: Vec::new(),
            extensions_required: Vec::new(),
            extensions: None,
            extras: None,
        },
        buffers: Vec::new(),
        source_form: GltfSourceForm::Json,
    }
}
fn required(c: &Catalog, s: &[u8], r: usize, k: &str, p: &mut Port<'_, '_, '_>) -> Result<usize, ValueError> {
    c.member(s, r, k, p)?.ok_or_else(|| invalid("natural glTF mandatory property is missing"))
}
fn optional_uint(c: &Catalog, s: &[u8], r: usize, k: &str, p: &mut Port<'_, '_, '_>) -> Result<Option<usize>, ValueError> {
    c.member(s, r, k, p)?.map(|i| c.usize(s, i, p)).transpose()
}
fn uint(c: &Catalog, s: &[u8], r: usize, k: &str, p: &mut Port<'_, '_, '_>) -> Result<usize, ValueError> {
    let i = required(c, s, r, k, p)?;
    c.usize(s, i, p)
}
fn text(c: &Catalog, s: &[u8], r: usize, k: &str, out: &mut Option<String>, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    if let Some(i) = c.member(s, r, k, p)? {
        p.effect(size_of::<String>(), || *out = Some(String::new()))?;
        c.text(s, i, out.as_mut().unwrap(), p)?;
    }
    Ok(())
}
fn boolean(c: &Catalog, s: &[u8], r: usize, k: &str, p: &mut Port<'_, '_, '_>) -> Result<bool, ValueError> {
    match c.member(s, r, k, p)?.map(|i| c.tokens[i].kind) {
        None | Some(6) => Ok(false),
        Some(5) => Ok(true),
        _ => Err(invalid("natural glTF property requires a Boolean")),
    }
}
fn fixed<const N: usize>(c: &Catalog, s: &[u8], r: usize, k: &str, p: &mut Port<'_, '_, '_>) -> Result<Option<[f64; N]>, ValueError> {
    let Some(i) = c.member(s, r, k, p)? else { return Ok(None) };
    if c.tokens[i].count != N {
        return Err(invalid("natural glTF transform vector has the wrong dimension"));
    }
    let mut values = [0.; N];
    for (at, child) in c.children(i)?.enumerate() {
        values[at] = c.number_value(s, child, p)?;
    }
    Ok(Some(values))
}
fn indices(c: &Catalog, s: &[u8], i: usize, out: &mut Vec<usize>, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    c.children(i)?;
    let count = c.tokens[i].count;
    p.vector(count, out)?;
    for child in c.children(i)? {
        let n = c.usize(s, child, p)?;
        p.effect(size_of::<usize>(), || out.push(n))?;
    }
    Ok(())
}
fn numbers(c: &Catalog, s: &[u8], r: usize, k: &str, out: &mut Option<Vec<f64>>, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    if let Some(i) = c.member(s, r, k, p)? {
        c.children(i)?;
        let count = c.tokens[i].count;
        if count > 16 {
            return Err(invalid("natural glTF accessor bounds exceed matrix dimensions"));
        }
        p.effect(size_of::<Vec<f64>>(), || *out = Some(Vec::new()))?;
        let values = out.as_mut().unwrap();
        p.vector(count, values)?;
        for child in c.children(i)? {
            let n = c.number_value(s, child, p)?;
            p.effect(size_of::<f64>(), || values.push(n))?;
        }
    }
    Ok(())
}
fn absent_or_empty(c: &Catalog, s: &[u8], r: usize, k: &str, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    if let Some(i) = c.member(s, r, k, p)? {
        if c.children(i)?.next().is_some() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural static geometry does not represent animation, skinning, morph targets or required extensions"));
        }
    }
    Ok(())
}
fn rows(c: &Catalog, s: &[u8], key: &str, p: &mut Port<'_, '_, '_>) -> Result<Option<usize>, ValueError> {
    let row = c.member(s, 0, key, p)?;
    if let Some(i) = row {
        c.children(i)?;
    }
    Ok(row)
}
fn unextended(c: &Catalog, s: &[u8], r: usize, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    if c.member(s, r, "extensions", p)?.is_some() {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural static geometry refuses selected extensions requiring evaluation"));
    }
    Ok(())
}
fn reserve_rows<T>(c: &Catalog, i: usize, out: &mut Vec<T>, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let count = c.tokens[i].count;
    if count > 200_000 {
        return Err(limit("natural glTF typed array exceeds row admission"));
    }
    p.vector(count, out)
}
pub(super) fn snapshot(source: &[u8], state: &mut NaturalReceiving, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let c = &state.catalog;
    let a = required(c, source, 0, "asset", port)?;
    unextended(c, source, 0, port)?;
    unextended(c, source, a, port)?;
    let version = required(c, source, a, "version", port)?;
    if !c.equal(source, version, "2.0", port)? {
        return Err(invalid("natural glTF asset version must be 2.0"));
    }
    if let Some(i) = c.member(source, a, "minVersion", port)? {
        if !c.equal(source, i, "2.0", port)? {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural glTF minimum version exceeds supported 2.0"));
        }
    }
    for k in ["animations", "skins", "extensionsRequired", "extensionsUsed"] {
        absent_or_empty(c, source, 0, k, port)?;
    }
    port.effect(size_of::<GltfSnapshot>(), || state.snapshot = Some(empty()))?;
    let snapshot = state.snapshot.as_mut().unwrap();
    port.text("stdio.gltf".len(), &mut snapshot.schema)?;
    port.effect("stdio.gltf".len(), || snapshot.schema.push_str("stdio.gltf"))?;
    port.text(3, &mut snapshot.document.asset.version)?;
    port.effect(3, || snapshot.document.asset.version.push_str("2.0"))?;
    snapshot.source_form = if state.framing.glb { GltfSourceForm::Glb } else { GltfSourceForm::Json };
    let d = &mut snapshot.document;
    d.scene = optional_uint(c, source, 0, "scene", port)?;
    text(c, source, a, "generator", &mut d.asset.generator, port)?;
    text(c, source, a, "copyright", &mut d.asset.copyright, port)?;
    text(c, source, a, "minVersion", &mut d.asset.min_version, port)?;
    if let Some(i) = rows(c, source, "scenes", port)? {
        reserve_rows(c, i, &mut d.scenes, port)?;
        for r in c.children(i)? {
            unextended(c, source, r, port)?;
            port.effect(size_of::<GltfScene>(), || d.scenes.push(GltfScene::default()))?;
            let row = d.scenes.last_mut().unwrap();
            text(c, source, r, "name", &mut row.name, port)?;
            if let Some(n) = c.member(source, r, "nodes", port)? {
                indices(c, source, n, &mut row.nodes, port)?;
            }
        }
    }
    if let Some(i) = rows(c, source, "nodes", port)? {
        reserve_rows(c, i, &mut d.nodes, port)?;
        for r in c.children(i)? {
            unextended(c, source, r, port)?;
            port.effect(size_of::<GltfNode>(), || d.nodes.push(GltfNode::default()))?;
            let row = d.nodes.last_mut().unwrap();
            text(c, source, r, "name", &mut row.name, port)?;
            row.mesh = optional_uint(c, source, r, "mesh", port)?;
            if c.member(source, r, "skin", port)?.is_some() {
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural static geometry refuses node skinning"));
            }
            absent_or_empty(c, source, r, "weights", port)?;
            row.matrix = fixed(c, source, r, "matrix", port)?;
            row.translation = fixed(c, source, r, "translation", port)?;
            row.rotation = fixed(c, source, r, "rotation", port)?;
            row.scale = fixed(c, source, r, "scale", port)?;
            if let Some(n) = c.member(source, r, "children", port)? {
                indices(c, source, n, &mut row.children, port)?;
            }
        }
    }
    if let Some(i) = rows(c, source, "meshes", port)? {
        reserve_rows(c, i, &mut d.meshes, port)?;
        for r in c.children(i)? {
            unextended(c, source, r, port)?;
            port.effect(size_of::<GltfMesh>(), || d.meshes.push(GltfMesh { primitives: Vec::new(), weights: Vec::new(), name: None, extensions: None, extras: None }))?;
            let row = d.meshes.last_mut().unwrap();
            text(c, source, r, "name", &mut row.name, port)?;
            absent_or_empty(c, source, r, "weights", port)?;
            let j = required(c, source, r, "primitives", port)?;
            reserve_rows(c, j, &mut row.primitives, port)?;
            for q in c.children(j)? {
                unextended(c, source, q, port)?;
                port.effect(size_of::<GltfPrimitive>(), || row.primitives.push(GltfPrimitive::default()))?;
                let primitive = row.primitives.last_mut().unwrap();
                primitive.indices = optional_uint(c, source, q, "indices", port)?;
                primitive.mode = optional_uint(c, source, q, "mode", port)?.map(|n| n as u64);
                absent_or_empty(c, source, q, "targets", port)?;
                let attributes = required(c, source, q, "attributes", port)?;
                let position = required(c, source, attributes, "POSITION", port)?;
                port.vector(1, &mut primitive.attributes)?;
                port.effect(size_of::<(String, usize)>(), || primitive.attributes.push((String::new(), 0)))?;
                let pair = &mut primitive.attributes[0];
                port.text(8, &mut pair.0)?;
                port.effect(8, || pair.0.push_str("POSITION"))?;
                pair.1 = c.usize(source, position, port)?;
            }
        }
    }
    if let Some(i) = rows(c, source, "accessors", port)? {
        reserve_rows(c, i, &mut d.accessors, port)?;
        for r in c.children(i)? {
            unextended(c, source, r, port)?;
            if c.member(source, r, "sparse", port)?.is_some() {
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural static geometry refuses sparse accessors"));
            }
            let component = match uint(c, source, r, "componentType", port)? {
                5120 => GltfComponentType::Byte,
                5121 => GltfComponentType::UnsignedByte,
                5122 => GltfComponentType::Short,
                5123 => GltfComponentType::UnsignedShort,
                5125 => GltfComponentType::UnsignedInt,
                5126 => GltfComponentType::Float,
                _ => return Err(invalid("natural glTF accessor component type is invalid")),
            };
            let kind = required(c, source, r, "type", port)?;
            let mut found = None;
            for (name, value) in [
                ("SCALAR", GltfAccessorType::Scalar),
                ("VEC2", GltfAccessorType::Vec2),
                ("VEC3", GltfAccessorType::Vec3),
                ("VEC4", GltfAccessorType::Vec4),
                ("MAT2", GltfAccessorType::Mat2),
                ("MAT3", GltfAccessorType::Mat3),
                ("MAT4", GltfAccessorType::Mat4),
            ] {
                if c.equal(source, kind, name, port)? {
                    found = Some(value);
                }
            }
            let kind = found.ok_or_else(|| invalid("natural glTF accessor type is invalid"))?;
            let count = uint(c, source, r, "count", port)?;
            port.effect(size_of::<GltfAccessor>(), || {
                d.accessors.push(GltfAccessor { buffer_view: None, byte_offset: 0, component_type: component, normalized: false, count, kind, max: None, min: None, sparse: None, name: None, extensions: None, extras: None })
            })?;
            let row = d.accessors.last_mut().unwrap();
            row.buffer_view = optional_uint(c, source, r, "bufferView", port)?;
            row.byte_offset = optional_uint(c, source, r, "byteOffset", port)?.unwrap_or(0);
            row.normalized = boolean(c, source, r, "normalized", port)?;
            text(c, source, r, "name", &mut row.name, port)?;
            numbers(c, source, r, "min", &mut row.min, port)?;
            numbers(c, source, r, "max", &mut row.max, port)?;
        }
    }
    if let Some(i) = rows(c, source, "bufferViews", port)? {
        reserve_rows(c, i, &mut d.buffer_views, port)?;
        for r in c.children(i)? {
            unextended(c, source, r, port)?;
            let buffer = uint(c, source, r, "buffer", port)?;
            let byte_length = uint(c, source, r, "byteLength", port)?;
            port.effect(size_of::<GltfBufferView>(), || d.buffer_views.push(GltfBufferView { buffer, byte_offset: 0, byte_length, byte_stride: None, target: None, name: None, extensions: None, extras: None }))?;
            let row = d.buffer_views.last_mut().unwrap();
            row.byte_offset = optional_uint(c, source, r, "byteOffset", port)?.unwrap_or(0);
            row.byte_stride = optional_uint(c, source, r, "byteStride", port)?;
            row.target = optional_uint(c, source, r, "target", port)?.map(|n| n as u64);
            text(c, source, r, "name", &mut row.name, port)?;
        }
    }
    if let Some(i) = rows(c, source, "buffers", port)? {
        reserve_rows(c, i, &mut d.buffers, port)?;
        reserve_rows(c, i, &mut snapshot.buffers, port)?;
        for r in c.children(i)? {
            unextended(c, source, r, port)?;
            let byte_length = uint(c, source, r, "byteLength", port)?;
            if byte_length > MAX_BYTES {
                return Err(limit("natural glTF resolved buffer exceeds byte admission"));
            }
            port.effect(size_of::<GltfBuffer>(), || d.buffers.push(GltfBuffer { byte_length, uri: None, name: None, extensions: None, extras: None }))?;
            let row = d.buffers.last_mut().unwrap();
            text(c, source, r, "uri", &mut row.uri, port)?;
            text(c, source, r, "name", &mut row.name, port)?;
            port.effect(size_of::<Vec<u8>>(), || snapshot.buffers.push(Vec::new()))?;
            let bytes = snapshot.buffers.last_mut().unwrap();
            port.vector(byte_length, bytes)?;
            match row.uri.as_deref() {
                Some(uri) => base64(uri, byte_length, bytes, port)?,
                None => {
                    if !state.framing.glb || d.buffers.len() != 1 {
                        return Err(invalid("natural glTF missing buffer URI requires the sole GLB BIN buffer"));
                    }
                    let (a, b) = state.framing.bin.ok_or_else(|| invalid("natural GLB declared buffer has no BIN chunk"))?;
                    if b - a < byte_length || b - a - byte_length > 3 {
                        return Err(invalid("natural GLB BIN declared extent or padding differs"));
                    }
                    for chunk in source[a..a + byte_length].chunks(256) {
                        port.effect(chunk.len(), || bytes.extend_from_slice(chunk))?;
                    }
                    for at in a + byte_length..b {
                        if !port.effect(1, || source[at] == 0)? {
                            return Err(invalid("natural GLB BIN padding is not zero"));
                        }
                    }
                }
            }
        }
    }
    if state.framing.bin.is_some() && (d.buffers.len() != 1 || d.buffers[0].uri.is_some()) {
        return Err(invalid("natural GLB BIN chunk does not correspond to one embedded buffer"));
    }
    Ok(())
}
fn base64(uri: &str, length: usize, out: &mut Vec<u8>, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let prefix = if uri.starts_with("data:application/octet-stream;base64,") {
        "data:application/octet-stream;base64,".len()
    } else if uri.starts_with("data:application/gltf-buffer;base64,") {
        "data:application/gltf-buffer;base64,".len()
    } else {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural glTF geometry requires embedded base64 buffer resources"));
    };
    let input = &uri.as_bytes()[prefix..];
    if input.len() % 4 != 0 || input.len() / 4 * 3 < length || input.len() / 4 * 3 - length > 2 {
        return Err(invalid("natural glTF base64 extent differs from declared buffer"));
    }
    fn digit(b: u8) -> Option<u32> {
        Some(match b {
            b'A'..=b'Z' => (b - b'A') as u32,
            b'a'..=b'z' => (b - b'a' + 26) as u32,
            b'0'..=b'9' => (b - b'0' + 52) as u32,
            b'+' => 62,
            b'/' => 63,
            _ => return None,
        })
    }
    for (i, chunk) in input.chunks_exact(4).enumerate() {
        let chunk = p.effect(4, || [chunk[0], chunk[1], chunk[2], chunk[3]])?;
        let remaining = length.saturating_sub(out.len());
        let take = remaining.min(3);
        let final_chunk = i + 1 == input.len() / 4;
        let a = digit(chunk[0]).ok_or_else(|| invalid("natural glTF base64 digit is invalid"))?;
        let b = digit(chunk[1]).ok_or_else(|| invalid("natural glTF base64 digit is invalid"))?;
        let c = if take >= 2 {
            digit(chunk[2]).ok_or_else(|| invalid("natural glTF base64 digit is invalid"))?
        } else {
            if chunk[2] != b'=' || !final_chunk || b & 15 != 0 {
                return Err(invalid("natural glTF base64 padding is invalid"));
            }
            0
        };
        let d = if take == 3 {
            digit(chunk[3]).ok_or_else(|| invalid("natural glTF base64 digit is invalid"))?
        } else {
            if chunk[3] != b'=' || !final_chunk || take == 2 && c & 3 != 0 {
                return Err(invalid("natural glTF base64 padding is invalid"));
            }
            0
        };
        let word = (a << 18) | (b << 12) | (c << 6) | d;
        let bytes = [(word >> 16) as u8, (word >> 8) as u8, word as u8];
        p.effect(4 + take, || out.extend_from_slice(&bytes[..take]))?;
    }
    if out.len() != length {
        return Err(invalid("natural glTF base64 output extent differs"));
    }
    Ok(())
}
