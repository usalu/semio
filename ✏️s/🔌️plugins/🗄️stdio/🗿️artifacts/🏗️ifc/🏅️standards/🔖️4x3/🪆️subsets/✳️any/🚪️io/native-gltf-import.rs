//! 📐️ Dense static glTF triangles bind final IFC coordinates and faces without decoded accessor or mesh mirrors.
use super::*;
use graph::V;
type Matrix = [[f64; 4]; 4];
const IDENTITY: Matrix = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]];
const NONE: usize = usize::MAX;
#[derive(Default, RetireOwned)]
struct Receiving {
    parents: Vec<usize>,
    seen: Vec<usize>,
    graph: Option<IfcSnapshot>,
    primitives: usize,
    vertices: usize,
    triangles: usize,
}
#[derive(Clone, Copy)]
struct Accessor {
    buffer: usize,
    start: usize,
    stride: usize,
    width: usize,
    count: usize,
}
fn accessor(source: &GltfSnapshot, index: usize, position: bool, p: &mut Port<'_, '_, '_>) -> Result<Accessor, ValueError> {
    let a = source.document.accessors.get(index).ok_or_else(|| invalid("static geometry references a missing accessor"))?;
    p.effect(size_of::<GltfAccessor>(), || ())?;
    let allowed = if position {
        a.component_type == GltfComponentType::Float && a.kind == GltfAccessorType::Vec3
    } else {
        a.kind == GltfAccessorType::Scalar && matches!(a.component_type, GltfComponentType::UnsignedByte | GltfComponentType::UnsignedShort | GltfComponentType::UnsignedInt)
    };
    if !allowed || a.normalized || a.sparse.is_some() || a.extensions.is_some() || a.count == 0 || a.count > if position { 200_000 } else { 1_200_000 } {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "static geometry requires bounded dense FLOAT VEC3 positions and unsigned scalar indices"));
    }
    for bounds in [&a.min, &a.max] {
        if let Some(bounds) = bounds {
            if bounds.len() != if position { 3 } else { 1 } {
                return Err(invalid("static geometry accessor bounds have wrong dimension"));
            }
            for n in bounds {
                if !p.effect(8, || n.is_finite())? {
                    return Err(invalid("static geometry accessor bounds are nonfinite"));
                }
            }
        }
    }
    let view = source.document.buffer_views.get(a.buffer_view.ok_or_else(|| invalid("dense geometry accessor has no buffer view"))?).ok_or_else(|| invalid("dense geometry accessor references a missing buffer view"))?;
    p.effect(size_of::<GltfBufferView>(), || ())?;
    let bytes = source.buffers.get(view.buffer).ok_or_else(|| invalid("dense geometry buffer is unresolved"))?;
    let declared = source.document.buffers.get(view.buffer).ok_or_else(|| invalid("dense geometry declared buffer is missing"))?;
    let component = if position {
        4
    } else {
        match a.component_type {
            GltfComponentType::UnsignedByte => 1,
            GltfComponentType::UnsignedShort => 2,
            _ => 4,
        }
    };
    let width = if position { 12 } else { component };
    let stride = view.byte_stride.unwrap_or(width);
    let extent = (a.count - 1).checked_mul(stride).and_then(|n| n.checked_add(width)).ok_or_else(|| invalid("dense geometry accessor extent overflows"))?;
    if view.extensions.is_some()
        || a.byte_offset % component != 0
        || view.byte_offset % component != 0
        || stride < width
        || view.target.is_some_and(|v| v != if position { 34962 } else { 34963 })
        || position && view.byte_stride.is_some_and(|v| v % 4 != 0 || v > 252)
        || !position && view.byte_stride.is_some()
        || a.byte_offset.checked_add(extent).is_none_or(|n| n > view.byte_length)
        || view.byte_offset.checked_add(view.byte_length).is_none_or(|n| n > declared.byte_length || n > bytes.len())
    {
        return Err(invalid("dense geometry accessor alignment, target, stride or original buffer extent is invalid"));
    }
    Ok(Accessor { buffer: view.buffer, start: view.byte_offset + a.byte_offset, stride, width: component, count: a.count })
}
fn matrix(node: &GltfNode, p: &mut Port<'_, '_, '_>) -> Result<Matrix, ValueError> {
    let (authored, translation, rotation, scale) = p.effect(size_of::<GltfNode>(), || (node.matrix, node.translation, node.rotation, node.scale))?;
    let m = if let Some(a) = authored {
        if translation.is_some() || rotation.is_some() || scale.is_some() {
            return Err(invalid("glTF matrix and TRS are mutually exclusive"));
        }
        std::array::from_fn(|r| std::array::from_fn(|c| a[c * 4 + r]))
    } else {
        let [x, y, z, w] = rotation.unwrap_or([0., 0., 0., 1.]);
        if ((x * x + y * y + z * z + w * w).sqrt() - 1.).abs() > 1e-5 {
            return Err(invalid("glTF rotation quaternion must be unit length"));
        }
        let [a, b, c] = scale.unwrap_or([1.; 3]);
        let [u, v, t] = translation.unwrap_or([0.; 3]);
        [
            [(1. - 2. * (y * y + z * z)) * a, 2. * (x * y - z * w) * b, 2. * (x * z + y * w) * c, u],
            [2. * (x * y + z * w) * a, (1. - 2. * (x * x + z * z)) * b, 2. * (y * z - x * w) * c, v],
            [2. * (x * z - y * w) * a, 2. * (y * z + x * w) * b, (1. - 2. * (x * x + y * y)) * c, t],
            [0., 0., 0., 1.],
        ]
    };
    if m.iter().flatten().any(|n| !n.is_finite()) || m[3] != [0., 0., 0., 1.] {
        return Err(invalid("glTF transform must be finite affine"));
    }
    Ok(m)
}
fn world(parent: Matrix, local: Matrix, p: &mut Port<'_, '_, '_>) -> Result<(Matrix, i8), ValueError> {
    let m = p.effect(3 * size_of::<Matrix>(), || std::array::from_fn(|r| std::array::from_fn(|c| (0..4).map(|k| parent[r][k] * local[k][c]).sum::<f64>())))?;
    if m.iter().flatten().any(|n| !n.is_finite()) {
        return Err(invalid("glTF composed world transform is nonfinite"));
    }
    let orientation = p.effect(32768, || super::super::projection::finite_orientation(m))?;
    if orientation == 0 {
        return Err(invalid("glTF world transform is exactly singular"));
    }
    Ok((m, orientation))
}
fn topology(source: &GltfSnapshot, state: &mut Receiving, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let d = &source.document;
    if source.schema != "stdio.gltf" || d.asset.version != "2.0" || d.asset.min_version.as_deref().is_some_and(|v| v != "2.0") || d.nodes.len() > 64_000 {
        return Err(invalid("bounded static glTF 2.0 native source required"));
    }
    if !d.animations.is_empty() || !d.skins.is_empty() || !d.extensions_used.is_empty() || !d.extensions_required.is_empty() || d.extensions.is_some() {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "static glTF geometry refuses extensions, animations and skins"));
    }
    p.vector(d.nodes.len(), &mut state.parents)?;
    p.vector(d.nodes.len(), &mut state.seen)?;
    for _ in &d.nodes {
        p.effect(2 * size_of::<usize>(), || {
            state.parents.push(NONE);
            state.seen.push(0);
        })?;
    }
    let mut bytes = 0usize;
    for buffer in &source.buffers {
        p.effect(size_of::<Vec<u8>>(), || ())?;
        bytes = bytes.checked_add(buffer.len()).filter(|n| *n <= MAX_BYTES).ok_or_else(|| limit("glTF original resolved buffers exceed 64 MiB"))?;
    }
    if d.buffers.len() != source.buffers.len() {
        return Err(invalid("glTF declared and resolved buffer counts differ"));
    }
    for (parent, node) in d.nodes.iter().enumerate() {
        p.effect(size_of::<GltfNode>(), || ())?;
        if node.skin.is_some() || !node.weights.is_empty() || node.extensions.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "static glTF geometry refuses node deformation and extensions"));
        }
        if node.mesh.is_some_and(|m| m >= d.meshes.len()) {
            return Err(invalid("glTF node references a missing mesh"));
        }
        for &child in &node.children {
            if child >= d.nodes.len() {
                return Err(invalid("glTF child references a missing node"));
            }
            if p.effect(size_of::<usize>(), || state.parents[child] != NONE)? {
                return Err(invalid("glTF node has repeated or multiple parents"));
            }
            p.effect(size_of::<usize>(), || state.parents[child] = parent)?;
        }
    }
    for root in 0..d.nodes.len() {
        let mut at = root;
        for depth in 0..=64 {
            if depth == 64 {
                return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "glTF node graph is cyclic or exceeds depth 64"));
            }
            let parent = p.effect(size_of::<usize>(), || state.parents[at])?;
            if parent == NONE {
                break;
            }
            if parent == root {
                return Err(invalid("glTF node graph is cyclic"));
            }
            at = parent;
        }
    }
    for (scene, row) in d.scenes.iter().enumerate() {
        p.effect(size_of::<GltfScene>(), || ())?;
        if row.extensions.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "static glTF scene extensions require evaluation"));
        }
        for &root in &row.nodes {
            if root >= d.nodes.len() || p.effect(size_of::<usize>(), || state.parents[root] != NONE || state.seen[root] == scene + 1)? {
                return Err(invalid("glTF scene roots must exist, be parentless and unique"));
            }
            p.effect(size_of::<usize>(), || state.seen[root] = scene + 1)?;
        }
    }
    Ok(())
}
fn visit(source: &GltfSnapshot, state: &mut Receiving, index: usize, parent: Matrix, depth: usize, context: Option<graph::Context>, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let context = p.effect(size_of::<Option<graph::Context>>(), || context)?;
    if depth >= 64 {
        return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "selected glTF hierarchy exceeds depth 64"));
    }
    let node = &source.document.nodes[index];
    let (transform, orientation) = world(parent, matrix(node, p)?, p)?;
    if let Some(index) = node.mesh {
        let mesh = &source.document.meshes[index];
        if !mesh.weights.is_empty() || mesh.extensions.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "static glTF geometry refuses mesh weights and extensions"));
        }
        for primitive in &mesh.primitives {
            p.effect(size_of::<GltfPrimitive>(), || ())?;
            if primitive.mode.unwrap_or(4) != 4 || !primitive.targets.is_empty() || primitive.extensions.is_some() {
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "static glTF geometry requires unextended TRIANGLES primitives"));
            }
            let mut position = None;
            for (key, index) in &primitive.attributes {
                if p.effect(key.len().min(9), || key == "POSITION")? {
                    if position.replace(*index).is_some() {
                        return Err(invalid("glTF primitive has duplicate POSITION semantics"));
                    }
                }
            }
            let position = accessor(source, position.ok_or_else(|| invalid("glTF primitive lacks POSITION"))?, true, p)?;
            let indices = primitive.indices.map(|i| accessor(source, i, false, p)).transpose()?;
            let count = indices.map_or(position.count, |a| a.count);
            if count % 3 != 0 || count == 0 {
                return Err(invalid("static glTF triangle index count is invalid"));
            }
            if let Some(context) = context {
                emit(source, state.graph.as_mut().unwrap(), context, position, indices, transform, orientation, node.name.as_deref().or(mesh.name.as_deref()).unwrap_or("glTF primitive"), p)?;
            } else {
                state.primitives = state.primitives.checked_add(1).filter(|n| *n <= 199_998).ok_or_else(|| limit("static glTF primitive budget exceeded"))?;
                state.vertices = state.vertices.checked_add(position.count).filter(|n| *n <= 200_000).ok_or_else(|| limit("static glTF vertex budget exceeded"))?;
                state.triangles = state.triangles.checked_add(count / 3).filter(|n| *n <= 400_000).ok_or_else(|| limit("static glTF triangle budget exceeded"))?;
            }
        }
    }
    for &child in &node.children {
        p.effect(size_of::<usize>(), || ())?;
        visit(source, state, child, transform, depth + 1, context, p)?;
    }
    Ok(())
}
fn roots(source: &GltfSnapshot, state: &mut Receiving, context: Option<graph::Context>, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let context = p.effect(size_of::<Option<graph::Context>>(), || context)?;
    let d = &source.document;
    let scene = if let Some(i) = d.scene {
        Some(d.scenes.get(i).ok_or_else(|| invalid("glTF selected scene is missing"))?)
    } else if d.scenes.len() == 1 {
        d.scenes.first()
    } else if d.scenes.is_empty() {
        None
    } else {
        return Err(invalid("glTF scene selection is ambiguous"));
    };
    if let Some(scene) = scene {
        for &root in &scene.nodes {
            visit(source, state, root, IDENTITY, 0, context, p)?;
        }
    } else {
        for root in 0..d.nodes.len() {
            if p.effect(size_of::<usize>(), || state.parents[root] == NONE)? {
                visit(source, state, root, IDENTITY, 0, context, p)?;
            }
        }
    }
    Ok(())
}
fn emit(source: &GltfSnapshot, out: &mut IfcSnapshot, context: graph::Context, position: Accessor, indices: Option<Accessor>, m: Matrix, orientation: i8, name: &str, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let context = p.effect(size_of::<graph::Context>(), || context)?;
    let points = graph::add(out, "IFCCARTESIANPOINTLIST3D", &[V::A(&[]), V::U], p)?;
    let IfcValue::Aggregate(rows) = &mut out.entities.last_mut().unwrap().args[0] else { unreachable!() };
    p.vector(position.count, rows)?;
    let bytes = &source.buffers[position.buffer];
    for i in 0..position.count {
        let at = position.start + i * position.stride;
        let xyz = p.effect(12, || std::array::from_fn::<f64, 3, _>(|k| f32::from_le_bytes(bytes[at + 4 * k..at + 4 * k + 4].try_into().unwrap()) as f64))?;
        if xyz.iter().any(|n| !n.is_finite()) {
            return Err(invalid("glTF selected position is nonfinite"));
        }
        let q = p.effect(24, || std::array::from_fn::<f64, 3, _>(|r| m[r][0] * xyz[0] + m[r][1] * xyz[1] + m[r][2] * xyz[2] + m[r][3]))?;
        if q.iter().any(|n| !n.is_finite()) {
            return Err(invalid("glTF transformed position is nonfinite"));
        }
        p.effect(size_of::<IfcValue>(), || rows.push(IfcValue::Aggregate(Vec::new())))?;
        graph::fill(&V::A(&[V::R(q[0]), V::R(-q[2]), V::R(q[1])]), rows.last_mut().unwrap(), p)?;
    }
    let faces = graph::add(out, "IFCTRIANGULATEDFACESET", &[V::Ref(points), V::U, V::E("F"), V::A(&[]), V::U], p)?;
    let IfcValue::Aggregate(rows) = &mut out.entities.last_mut().unwrap().args[3] else { unreachable!() };
    let count = indices.map_or(position.count, |a| a.count);
    p.vector(count / 3, rows)?;
    for triangle in 0..count / 3 {
        let mut ids = [0i64; 3];
        for (k, slot) in ids.iter_mut().enumerate() {
            let i = triangle * 3 + k;
            let index = if let Some(a) = indices {
                let at = a.start + i * a.stride;
                let bytes = &source.buffers[a.buffer];
                p.effect(a.width, || match a.width {
                    1 => bytes[at] as usize,
                    2 => u16::from_le_bytes(bytes[at..at + 2].try_into().unwrap()) as usize,
                    _ => u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap()) as usize,
                })?
            } else {
                i
            };
            if index >= position.count {
                return Err(invalid("glTF triangle index exceeds its positions"));
            }
            *slot = index as i64 + 1;
        }
        if orientation < 0 {
            ids.swap(1, 2);
        }
        p.effect(size_of::<IfcValue>(), || rows.push(IfcValue::Unset))?;
        graph::fill(&V::A(&[V::I(ids[0]), V::I(ids[1]), V::I(ids[2])]), rows.last_mut().unwrap(), p)?;
    }
    graph::proxy(out, context, faces, name, p)
}
pub(super) fn receive(source: &GltfSnapshot, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<IfcSnapshot, ValueError> {
    owner.receive::<Receiving, IfcSnapshot>(|slot, native, body| {
        native.scoped_stage(|native| {
            native.begin_stage(0)?;
            let mut p = Port { native, body };
            p.effect(size_of::<Receiving>(), || *slot = Some(Receiving::default()))?;
            let state = slot.as_mut().unwrap();
            topology(source, state, &mut p)?;
            roots(source, state, None, &mut p)?;
            if state.primitives == 0 {
                return Err(invalid("selected glTF scene has no static triangle geometry"));
            }
            p.effect(size_of::<IfcSnapshot>(), || state.graph = Some(graph::empty()))?;
            let context = graph::begin(state.graph.as_mut().unwrap(), state.primitives, &mut p)?;
            roots(source, state, Some(context), &mut p)?;
            p.effect(size_of::<IfcSnapshot>(), || ())?;
            Ok(state.graph.take().unwrap())
        })
    })
}
