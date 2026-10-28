//! 💾️ Natural GLB publication measures borrowed geometry fields and writes one final output buffer under the original encoder.
use super::*;
use semio_framework_os_kernel::NativeSnapshotEncodeOwner;
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonScalarBytes as Scalar};
use semio_framework_value::NativeEncodeControl;
struct Port<'a, 'b, 'c> {
    native: &'a mut NativeEncodeControl<'b>,
    body: &'c mut NativeSnapshotBodyWallet,
}
impl Port<'_, '_, '_> {
    fn effect<T: Copy>(&mut self, n: usize, f: impl FnOnce() -> T) -> Result<T, ValueError> {
        self.native.checkpoint()?;
        self.body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: n, maximum_depth: 2, ..Default::default() })?;
        let value = f();
        self.body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: n, ..Default::default() })?;
        self.native.step()?;
        Ok(value)
    }
}
#[derive(Default, RetireOwned)]
struct Writing {
    output: Vec<u8>,
    json: usize,
    counter: usize,
    writing: bool,
}
fn bytes(s: &[u8], state: &mut Writing, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    for chunk in s.chunks(256) {
        if state.writing && chunk.len() > state.output.capacity().saturating_sub(state.output.len()) {
            return Err(invalid("GLB output exceeded original admitted extent"));
        }
        let next = state.counter.checked_add(chunk.len()).filter(|n| *n <= MAX_BYTES).ok_or_else(|| limit("GLB output exceeds 64 MiB admission"))?;
        p.effect(chunk.len() + size_of::<usize>(), || {
            state.counter = next;
            if state.writing {
                state.output.extend_from_slice(chunk);
            }
        })?;
    }
    Ok(())
}
fn number(n: Node<'_>, state: &mut Writing, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    if matches!(n,Node::F64(n)if!n.is_finite()) {
        return Err(invalid("GLB JSON number must be finite"));
    }
    let formatted = p.effect(64, || Scalar::from_node(n).ok().map(|v| (v.bytes, v.length)))?.ok_or_else(|| invalid("GLB JSON scalar formatting refused"))?;
    bytes(&formatted.0[..formatted.1], state, p)
}
fn uint(n: usize, state: &mut Writing, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    number(Node::U64(n as u64), state, p)
}
fn string(s: &str, state: &mut Writing, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    bytes(b"\"", state, p)?;
    for &b in s.as_bytes() {
        let mut escape = [0; 6];
        let count = p.effect(7, || semio_framework_pack_json::canonical_escape(b, &mut escape))?;
        bytes(&escape[..count], state, p)?;
    }
    bytes(b"\"", state, p)
}
fn floats(values: &[f64], state: &mut Writing, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    bytes(b"[", state, p)?;
    for (i, &n) in values.iter().enumerate() {
        if i != 0 {
            bytes(b",", state, p)?;
        }
        number(Node::F64(n), state, p)?;
    }
    bytes(b"]", state, p)
}
fn document(source: &GltfSnapshot, state: &mut Writing, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let d = &source.document;
    bytes(b"{\"asset\":{\"version\":\"2.0\"},\"scene\":0,\"scenes\":[{\"nodes\":[", state, p)?;
    for (i, &node) in d.scenes[0].nodes.iter().enumerate() {
        if i != 0 {
            bytes(b",", state, p)?;
        }
        uint(node, state, p)?;
    }
    bytes(b"]}],\"nodes\":[", state, p)?;
    for (i, node) in d.nodes.iter().enumerate() {
        if i != 0 {
            bytes(b",", state, p)?;
        }
        bytes(b"{\"mesh\":", state, p)?;
        uint(node.mesh.unwrap(), state, p)?;
        bytes(b"}", state, p)?;
    }
    bytes(b"],\"meshes\":[", state, p)?;
    for (i, mesh) in d.meshes.iter().enumerate() {
        if i != 0 {
            bytes(b",", state, p)?;
        }
        bytes(b"{", state, p)?;
        if let Some(name) = &mesh.name {
            bytes(b"\"name\":", state, p)?;
            string(name, state, p)?;
            bytes(b",", state, p)?;
        }
        let primitive = &mesh.primitives[0];
        bytes(b"\"primitives\":[{\"attributes\":{\"POSITION\":", state, p)?;
        uint(primitive.attributes[0].1, state, p)?;
        bytes(b"},\"indices\":", state, p)?;
        uint(primitive.indices.unwrap(), state, p)?;
        bytes(b",\"mode\":4}]}", state, p)?;
    }
    bytes(b"],\"accessors\":[", state, p)?;
    for (i, a) in d.accessors.iter().enumerate() {
        if i != 0 {
            bytes(b",", state, p)?;
        }
        bytes(b"{\"bufferView\":", state, p)?;
        uint(a.buffer_view.unwrap(), state, p)?;
        bytes(b",\"componentType\":", state, p)?;
        number(
            Node::U64(match a.component_type {
                GltfComponentType::Float => 5126,
                GltfComponentType::UnsignedInt => 5125,
                _ => return Err(invalid("generated GLB accessor component type differs")),
            }),
            state,
            p,
        )?;
        bytes(b",\"count\":", state, p)?;
        uint(a.count, state, p)?;
        bytes(b",\"type\":", state, p)?;
        string(
            match a.kind {
                GltfAccessorType::Vec3 => "VEC3",
                GltfAccessorType::Scalar => "SCALAR",
                _ => return Err(invalid("generated GLB accessor shape differs")),
            },
            state,
            p,
        )?;
        if let Some(v) = &a.min {
            bytes(b",\"min\":", state, p)?;
            floats(v, state, p)?;
        }
        if let Some(v) = &a.max {
            bytes(b",\"max\":", state, p)?;
            floats(v, state, p)?;
        }
        bytes(b"}", state, p)?;
    }
    bytes(b"],\"bufferViews\":[", state, p)?;
    for (i, v) in d.buffer_views.iter().enumerate() {
        if i != 0 {
            bytes(b",", state, p)?;
        }
        bytes(b"{\"buffer\":0,\"byteOffset\":", state, p)?;
        uint(v.byte_offset, state, p)?;
        bytes(b",\"byteLength\":", state, p)?;
        uint(v.byte_length, state, p)?;
        if let Some(target) = v.target {
            bytes(b",\"target\":", state, p)?;
            number(Node::U64(target), state, p)?;
        }
        bytes(b"}", state, p)?;
    }
    bytes(b"],\"buffers\":[{\"byteLength\":", state, p)?;
    uint(source.buffers[0].len(), state, p)?;
    bytes(b"}]}", state, p)
}
fn validate(source: &GltfSnapshot, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let d = &source.document;
    p.effect(size_of::<GltfDocument>(), || ())?;
    if !d.materials.is_empty() || !d.textures.is_empty() || !d.images.is_empty() || !d.samplers.is_empty()
        || !d.skins.is_empty() || !d.animations.is_empty() || !d.cameras.is_empty()
        || !d.extensions_used.is_empty() || !d.extensions_required.is_empty() || d.extensions.is_some() || d.extras.is_some()
        || d.asset.generator.is_some() || d.asset.copyright.is_some() || d.asset.min_version.is_some()
        || d.asset.extensions.is_some() || d.asset.extras.is_some()
    {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural GLB publication requires the actual geometry-only IFC projection"));
    }
    if source.schema != "stdio.gltf"
        || d.asset.version != "2.0"
        || d.scene != Some(0)
        || d.scenes.len() != 1
        || d.buffers.len() != 1
        || source.buffers.len() != 1
        || d.buffers[0].byte_length != source.buffers[0].len()
        || d.buffers[0].uri.is_some()
        || d.nodes.len() != d.meshes.len()
        || d.scenes[0].nodes.len() != d.nodes.len()
    {
        return Err(invalid("GLB publication requires the actual baked geometry snapshot produced by IFC projection"));
    }
    if d.scenes[0].name.is_some() || d.scenes[0].extensions.is_some() || d.scenes[0].extras.is_some()
        || d.buffers[0].name.is_some() || d.buffers[0].extensions.is_some() || d.buffers[0].extras.is_some()
    {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural GLB publication cannot discard authored scene or buffer metadata"));
    }
    for (i, node) in d.nodes.iter().enumerate() {
        p.effect(size_of::<GltfNode>(), || ())?;
        if node.name.is_some() || node.camera.is_some() || node.skin.is_some() || !node.weights.is_empty() || node.extensions.is_some() || node.extras.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural GLB publication cannot discard authored node semantics"));
        }
        if node.mesh != Some(i) || !node.children.is_empty() || node.matrix.is_some() || node.translation.is_some() || node.rotation.is_some() || node.scale.is_some() || d.scenes[0].nodes[i] != i {
            return Err(invalid("GLB publication source is not the admitted baked geometry form"));
        }
    }
    for mesh in &d.meshes {
        p.effect(size_of::<GltfMesh>(), || ())?;
        if !mesh.weights.is_empty() || mesh.extensions.is_some() || mesh.extras.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural GLB publication cannot discard authored mesh semantics"));
        }
        if mesh.primitives.len() != 1 {
            return Err(invalid("GLB publication requires one admitted primitive per mesh"));
        }
        let primitive = &mesh.primitives[0];
        if primitive.material.is_some() || !primitive.targets.is_empty() || primitive.extensions.is_some() || primitive.extras.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural GLB publication cannot discard authored primitive semantics"));
        }
        if primitive.attributes.len() != 1 || primitive.attributes[0].0 != "POSITION" || primitive.indices.is_none() || primitive.mode != Some(4) {
            return Err(invalid("GLB publication primitive differs from admitted triangle form"));
        }
    }
    for a in &d.accessors {
        p.effect(size_of::<GltfAccessor>(), || ())?;
        if a.name.is_some() || a.extensions.is_some() || a.extras.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural GLB publication cannot discard authored accessor metadata"));
        }
        if a.buffer_view.is_none() || a.byte_offset != 0 || a.normalized || a.sparse.is_some() {
            return Err(invalid("GLB publication accessor differs from admitted dense form"));
        }
    }
    for v in &d.buffer_views {
        p.effect(size_of::<GltfBufferView>(), || ())?;
        if v.name.is_some() || v.extensions.is_some() || v.extras.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "natural GLB publication cannot discard authored buffer view metadata"));
        }
        if v.buffer != 0 || v.byte_stride.is_some() {
            return Err(invalid("GLB publication buffer view differs from admitted dense form"));
        }
    }
    Ok(())
}
pub(super) fn encode(source: &GltfSnapshot, owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<Vec<u8>, ValueError> {
    owner.receive::<Writing, Vec<u8>>(|slot, native, body| {
        native.scoped_stage(|native| {
            native.begin_stage(0)?;
            let mut p = Port { native, body };
            p.effect(size_of::<Writing>(), || *slot = Some(Writing::default()))?;
            let state = slot.as_mut().unwrap();
            validate(source, &mut p)?;
            document(source, state, &mut p)?;
            state.json = state.counter;
            let json = (state.json + 3) & !3;
            let bin = (source.buffers[0].len() + 3) & !3;
            let total = 28usize.checked_add(json).and_then(|n| n.checked_add(bin)).filter(|n| *n <= MAX_BYTES).ok_or_else(|| limit("GLB final output exceeds byte admission"))?;
            p.body.allocate_encode_vec_into(p.native, total, &mut state.output, 2)?;
            p.native.step()?;
            p.effect(size_of::<Writing>(), || {
                state.writing = true;
                state.counter = 0;
            })?;
            bytes(b"glTF", state, &mut p)?;
            for word in [2, total as u32, json as u32, 0x4e4f534a] {
                bytes(&word.to_le_bytes(), state, &mut p)?;
            }
            document(source, state, &mut p)?;
            for _ in state.json..json {
                bytes(b" ", state, &mut p)?;
            }
            bytes(&(bin as u32).to_le_bytes(), state, &mut p)?;
            bytes(&0x004e4942u32.to_le_bytes(), state, &mut p)?;
            bytes(&source.buffers[0], state, &mut p)?;
            for _ in source.buffers[0].len()..bin {
                bytes(&[0], state, &mut p)?;
            }
            if state.output.len() != total {
                return Err(invalid("GLB final output differs from its funded census"));
            }
            p.effect(size_of::<Vec<u8>>(), || ())?;
            Ok(std::mem::take(&mut state.output))
        })
    })
}
