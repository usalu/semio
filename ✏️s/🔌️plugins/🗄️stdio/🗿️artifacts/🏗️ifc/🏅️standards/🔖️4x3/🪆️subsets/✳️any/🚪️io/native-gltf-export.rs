//! 🧊️ Owned tessellation projection borrows the original IFC graph, static declarations and funded identity locators.
use super::*;
use crate::validation::metadata;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcEntity;
type Matrix = [[f64; 4]; 4];
const IDENTITY: Matrix = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]];
#[derive(Default, RetireOwned)]
struct Receiving {
    index: Vec<(u64, usize)>,
    target: Option<GltfSnapshot>,
    primitives: usize,
    vertices: usize,
    triangles: usize,
    project: u64,
    scale: f64,
}
fn aggregate(v: Option<&IfcValue>) -> Result<&[IfcValue], ValueError> {
    v.and_then(IfcValue::as_aggregate).ok_or_else(|| invalid("selected IFC geometry requires an aggregate"))
}
fn reference(v: Option<&IfcValue>) -> Result<u64, ValueError> {
    v.and_then(IfcValue::as_reference).filter(|n| *n != 0).ok_or_else(|| invalid("selected IFC geometry requires a positive reference"))
}
fn number(v: &IfcValue, p: &mut Port<'_, '_, '_>) -> Result<f64, ValueError> {
    let value = p
        .effect(size_of::<IfcValue>(), || match v {
            IfcValue::Real(n) => Some(*n),
            IfcValue::Integer(n) if (-9007199254740992..=9007199254740992).contains(n) => Some(*n as f64),
            _ => None,
        })?
        .filter(|n| n.is_finite())
        .ok_or_else(|| invalid("selected IFC geometry requires an exact finite Binary64 measure"))?;
    Ok(value)
}
fn ordinal(v: &IfcValue, maximum: usize, p: &mut Port<'_, '_, '_>) -> Result<usize, ValueError> {
    let n = p.effect(size_of::<IfcValue>(), || if let IfcValue::Integer(n) = v { Some(*n) } else { None })?.filter(|n| *n > 0 && *n as u64 <= maximum as u64).ok_or_else(|| invalid("IFC tessellation index exceeds its declared source"))?;
    Ok(n as usize - 1)
}
fn sort(values: &mut [(u64, usize)], p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    fn sift(v: &mut [(u64, usize)], mut root: usize, end: usize, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
        loop {
            let left = root * 2 + 1;
            if left >= end {
                return Ok(());
            }
            let mut child = left;
            if left + 1 < end && p.effect(2 * size_of::<(u64, usize)>(), || v[left] < v[left + 1])? {
                child += 1;
            }
            if !p.effect(2 * size_of::<(u64, usize)>(), || v[root] < v[child])? {
                return Ok(());
            }
            p.effect(2 * size_of::<(u64, usize)>(), || v.swap(root, child))?;
            root = child;
        }
    }
    for root in (0..values.len() / 2).rev() {
        sift(values, root, values.len(), p)?;
    }
    for end in (1..values.len()).rev() {
        p.effect(2 * size_of::<(u64, usize)>(), || values.swap(0, end))?;
        sift(values, 0, end, p)?;
    }
    Ok(())
}
fn entity<'a>(source: &'a IfcSnapshot, state: &Receiving, id: u64, p: &mut Port<'_, '_, '_>) -> Result<&'a IfcEntity, ValueError> {
    let (mut low, mut high) = (0, state.index.len());
    while low < high {
        let mid = low + (high - low) / 2;
        let (key, index) = p.effect(size_of::<(u64, usize)>(), || state.index[mid])?;
        match key.cmp(&id) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Ok(&source.entities[index]),
        }
    }
    Err(invalid("selected IFC geometry reference is missing"))
}
fn multiply(a: Matrix, b: Matrix, p: &mut Port<'_, '_, '_>) -> Result<Matrix, ValueError> {
    let out = p.effect(3 * size_of::<Matrix>(), || std::array::from_fn::<[f64; 4], 4, _>(|r| std::array::from_fn(|c| (0..4).map(|k| a[r][k] * b[k][c]).sum())))?;
    if out.iter().flatten().any(|n| !n.is_finite()) {
        return Err(invalid("IFC geometry transform composition is nonfinite"));
    }
    Ok(out)
}
fn xyz(source: &IfcSnapshot, state: &Receiving, id: u64, kind: &str, p: &mut Port<'_, '_, '_>) -> Result<[f64; 3], ValueError> {
    let e = entity(source, state, id, p)?;
    if e.name != kind {
        return Err(invalid("IFC placement references the wrong coordinate kind"));
    }
    let row = aggregate(e.args.first())?;
    if row.len() != 3 {
        return Err(invalid("IFC geometry requires three-dimensional coordinates"));
    }
    Ok([number(&row[0], p)?, number(&row[1], p)?, number(&row[2], p)?])
}
fn normal(v: [f64; 3]) -> Result<[f64; 3], ValueError> {
    let scale = v.iter().fold(0f64, |a, v| a.max(v.abs()));
    if scale == 0. || !scale.is_finite() {
        return Err(invalid("IFC direction is zero or nonfinite"));
    }
    let q = v.map(|n| n / scale);
    let length = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt();
    Ok(q.map(|n| n / length))
}
fn axis(source: &IfcSnapshot, state: &Receiving, id: u64, p: &mut Port<'_, '_, '_>) -> Result<Matrix, ValueError> {
    let e = entity(source, state, id, p)?;
    if e.name != "IFCAXIS2PLACEMENT3D" || e.args.len() != 3 {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "IFC static triangles require explicit three-dimensional axis placements"));
    }
    if matches!(e.args.get(1), Some(IfcValue::Unset)) != matches!(e.args.get(2), Some(IfcValue::Unset)) {
        return Err(invalid("IFC Axis and RefDirection must both be supplied or both be unset"));
    }
    let origin = xyz(source, state, reference(e.args.first())?, "IFCCARTESIANPOINT", p)?;
    let z = if matches!(e.args.get(1), Some(IfcValue::Unset)) { [0., 0., 1.] } else { normal(xyz(source, state, reference(e.args.get(1))?, "IFCDIRECTION", p)?)? };
    let v = if matches!(e.args.get(2), Some(IfcValue::Unset)) { if z == [1., 0., 0.] { [0., 1., 0.] } else { [1., 0., 0.] } } else { normal(xyz(source, state, reference(e.args.get(2))?, "IFCDIRECTION", p)?)? };
    let dot = v[0] * z[0] + v[1] * z[1] + v[2] * z[2];
    let x = normal(std::array::from_fn(|i| v[i] - dot * z[i]))?;
    let y = normal([z[1] * x[2] - z[2] * x[1], z[2] * x[0] - z[0] * x[2], z[0] * x[1] - z[1] * x[0]])?;
    p.effect(size_of::<Matrix>(), || [[x[0], y[0], z[0], origin[0]], [x[1], y[1], z[1], origin[1]], [x[2], y[2], z[2], origin[2]], [0., 0., 0., 1.]])
}
fn placement(source: &IfcSnapshot, state: &Receiving, start: Option<&IfcValue>, p: &mut Port<'_, '_, '_>) -> Result<Matrix, ValueError> {
    if matches!(start, None | Some(IfcValue::Unset)) {
        return Ok(IDENTITY);
    }
    let mut id = reference(start)?;
    let mut result = IDENTITY;
    let mut seen = [0u64; 64];
    for depth in 0..64 {
        if seen[..depth].contains(&id) {
            return Err(invalid("IFC local placement chain is cyclic"));
        }
        seen[depth] = id;
        let e = entity(source, state, id, p)?;
        if e.name != "IFCLOCALPLACEMENT" || e.args.len() != 2 {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "IFC linear, grid and extended placements require their own retained evaluator"));
        }
        let local = axis(source, state, reference(e.args.get(1))?, p)?;
        result = multiply(local, result, p)?;
        if matches!(e.args.first(), Some(IfcValue::Unset)) {
            return Ok(result);
        }
        id = reference(e.args.first())?;
    }
    Err(ValueError::literal(ValueRefusalKind::DepthLimit, "IFC local placement chain exceeds 64"))
}
fn context(source: &IfcSnapshot, state: &Receiving, start: u64, p: &mut Port<'_, '_, '_>) -> Result<Matrix, ValueError> {
    let mut id = start;
    let mut seen = [0; 64];
    for depth in 0..64 {
        if seen[..depth].contains(&id) {
            return Err(invalid("IFC representation context chain is cyclic"));
        }
        seen[depth] = id;
        let e = entity(source, state, id, p)?;
        if e.name == "IFCGEOMETRICREPRESENTATIONSUBCONTEXT" {
            id = reference(e.args.get(6))?;
            continue;
        }
        if e.name != "IFCGEOMETRICREPRESENTATIONCONTEXT" || !matches!(e.args.get(2), Some(IfcValue::Integer(3))) || !matches!(e.args.get(5), Some(IfcValue::Unset)) {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "IFC geometry requires a three-dimensional context without additional geographic orientation"));
        }
        let project = entity(source, state, state.project, p)?;
        let mut belongs = false;
        for v in aggregate(project.args.get(7))? {
            let root = reference(Some(v))?;
            belongs |= p.effect(size_of::<u64>(), || root == id)?;
        }
        if !belongs {
            return Err(invalid("IFC representation context is not declared by its sole project"));
        }
        return axis(source, state, reference(e.args.get(4))?, p);
    }
    Err(ValueError::literal(ValueRefusalKind::DepthLimit, "IFC representation context chain exceeds 64"))
}
fn prepare(source: &IfcSnapshot, state: &mut Receiving, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    if source.schema != "stdio.ifc" || source.entities.len() > 1_000_000 || !matches!(source.header.file_schema.as_slice(),[IfcValue::Aggregate(v)]if matches!(v.as_slice(),[IfcValue::String(s)]if s=="IFC4X3_ADD2")) {
        return Err(invalid("IFC geometry source must be a bounded exact ADD2 graph"));
    }
    p.vector(source.entities.len(), &mut state.index)?;
    for (index, e) in source.entities.iter().enumerate() {
        p.effect(size_of::<(u64, usize)>(), || state.index.push((e.id, index)))?;
        if e.id == 0 || !e.complex.is_empty() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "IFC geometry requires positive simple entity records"));
        }
        if p.effect(128, || metadata::entity(&e.name).is_none())? {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "IFC geometry encountered an undeclared entity kind"));
        }
        if matches!(e.name.as_str(), "IFCMAPCONVERSION" | "IFCMAPCONVERSIONSCALED" | "IFCRIGIDOPERATION" | "IFCPROJECTEDCRS" | "IFCGEOGRAPHICCRS" | "IFCRELVOIDSELEMENT") {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "IFC georeferencing and relational void subtraction require their own retained evaluator"));
        }
        if e.name == "IFCPROJECT" {
            if state.project != 0 {
                return Err(invalid("IFC geometry project and length-unit authority is ambiguous"));
            }
            state.project = e.id;
        }
    }
    sort(&mut state.index, p)?;
    for pair in state.index.windows(2) {
        if p.effect(2 * size_of::<(u64, usize)>(), || pair[0].0 == pair[1].0)? {
            return Err(invalid("IFC geometry entity identities are repeated"));
        }
    }
    let project = entity(source, state, state.project, p)?;
    let units = entity(source, state, reference(project.args.get(8))?, p)?;
    if units.name != "IFCUNITASSIGNMENT" {
        return Err(invalid("IFC project unit authority is not a unit assignment"));
    }
    for v in aggregate(units.args.first())? {
        let unit = entity(source, state, reference(Some(v))?, p)?;
        p.effect(size_of::<IfcValue>(), || ())?;
        if !matches!(unit.args.get(1),Some(IfcValue::Enum(s))if s=="LENGTHUNIT") {
            continue;
        }
        if state.scale != 0. {
            return Err(invalid("IFC project has multiple length units"));
        }
        if unit.name != "IFCSIUNIT" || !matches!(unit.args.first(), Some(IfcValue::Derived)) || !matches!(unit.args.get(3),Some(IfcValue::Enum(s))if s=="METRE") {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "IFC geometry requires an explicit SI metre length unit"));
        }
        state.scale = match unit.args.get(2) {
            Some(IfcValue::Unset) => 1.,
            Some(IfcValue::Enum(s)) => match s.as_str() {
                "EXA" => 1e18,
                "PETA" => 1e15,
                "TERA" => 1e12,
                "GIGA" => 1e9,
                "MEGA" => 1e6,
                "KILO" => 1e3,
                "HECTO" => 1e2,
                "DECA" => 1e1,
                "DECI" => 1e-1,
                "CENTI" => 1e-2,
                "MILLI" => 1e-3,
                "MICRO" => 1e-6,
                "NANO" => 1e-9,
                "PICO" => 1e-12,
                "FEMTO" => 1e-15,
                "ATTO" => 1e-18,
                _ => return Err(invalid("IFC SI prefix is undeclared")),
            },
            _ => return Err(invalid("IFC SI prefix value is invalid")),
        };
    }
    if state.scale == 0. {
        return Err(invalid("IFC geometry project declares no length unit"));
    }
    Ok(())
}
fn product(source: &IfcSnapshot, state: &mut Receiving, product: &IfcEntity, emit: bool, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    if !p.effect(4096, || metadata::is_subtype(&product.name, "IFCPRODUCT"))? || matches!(product.args.get(6), None | Some(IfcValue::Unset)) {
        return Ok(());
    }
    let definition = entity(source, state, reference(product.args.get(6))?, p)?;
    if definition.name != "IFCPRODUCTDEFINITIONSHAPE" {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "IFC geometry requires a product definition shape"));
    }
    let place = placement(source, state, product.args.get(5), p)?;
    for value in aggregate(definition.args.get(2))? {
        let representation = entity(source, state, reference(Some(value))?, p)?;
        if representation.name != "IFCSHAPEREPRESENTATION" {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "IFC representation needs a retained geometric evaluator"));
        }
        if !matches!(representation.args.get(1),Some(IfcValue::String(s))if s=="Body") {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "IFC geometry-only export requires explicit Body representations"));
        }
        let world = multiply(context(source, state, reference(representation.args.first())?, p)?, place, p)?;
        for value in aggregate(representation.args.get(3))? {
            let item = entity(source, state, reference(Some(value))?, p)?;
            if item.name != "IFCTRIANGULATEDFACESET" {
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "owned IFC geometry export currently represents explicit triangulated face sets; mapped, swept, boolean and boundary geometry require retained evaluators"));
            }
            let points = entity(source, state, reference(item.args.first())?, p)?;
            if points.name != "IFCCARTESIANPOINTLIST3D" {
                return Err(invalid("IFC tessellation references the wrong point-list kind"));
            }
            let rows = aggregate(points.args.first())?;
            let faces = aggregate(item.args.get(3))?;
            if rows.is_empty() || faces.is_empty() {
                return Err(invalid("IFC tessellation is empty"));
            }
            if emit {
                emit_mesh(state.target.as_mut().unwrap(), item, rows, faces, world, state.scale, product.args.get(2).and_then(IfcValue::as_str).unwrap_or("IFC geometry"), p)?;
            } else {
                state.primitives = state.primitives.checked_add(1).filter(|n| *n <= 200_000).ok_or_else(|| limit("IFC triangle primitive budget exceeded"))?;
                state.vertices = state.vertices.checked_add(rows.len()).filter(|n| *n <= 200_000).ok_or_else(|| limit("IFC triangle vertex budget exceeded"))?;
                state.triangles = state.triangles.checked_add(faces.len()).filter(|n| *n <= 400_000).ok_or_else(|| limit("IFC triangle count budget exceeded"))?;
            }
        }
    }
    Ok(())
}
fn output(state: &mut Receiving, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    p.effect(size_of::<GltfSnapshot>(), || state.target = Some(super::receive::empty()))?;
    let out = state.target.as_mut().unwrap();
    p.text(10, &mut out.schema)?;
    p.effect(10, || out.schema.push_str("stdio.gltf"))?;
    p.text(3, &mut out.document.asset.version)?;
    p.effect(3, || out.document.asset.version.push_str("2.0"))?;
    out.source_form = GltfSourceForm::Glb;
    out.document.scene = Some(0);
    p.vector(1, &mut out.document.scenes)?;
    p.effect(size_of::<GltfScene>(), || out.document.scenes.push(GltfScene::default()))?;
    p.vector(state.primitives, &mut out.document.scenes[0].nodes)?;
    p.vector(state.primitives, &mut out.document.nodes)?;
    p.vector(state.primitives, &mut out.document.meshes)?;
    p.vector(2 * state.primitives, &mut out.document.accessors)?;
    p.vector(2 * state.primitives, &mut out.document.buffer_views)?;
    p.vector(1, &mut out.document.buffers)?;
    p.vector(1, &mut out.buffers)?;
    let length = state.vertices.checked_mul(12).and_then(|n| state.triangles.checked_mul(12).and_then(|m| n.checked_add(m))).filter(|n| *n <= MAX_BYTES).ok_or_else(|| limit("IFC geometry output exceeds byte admission"))?;
    p.effect(size_of::<GltfBuffer>(), || out.document.buffers.push(GltfBuffer { byte_length: length, uri: None, name: None, extensions: None, extras: None }))?;
    p.effect(size_of::<Vec<u8>>(), || out.buffers.push(Vec::new()))?;
    p.vector(length, &mut out.buffers[0])
}
fn accessor_row(view: usize, count: usize, position: bool) -> GltfAccessor {
    GltfAccessor {
        buffer_view: Some(view),
        byte_offset: 0,
        component_type: if position { GltfComponentType::Float } else { GltfComponentType::UnsignedInt },
        normalized: false,
        count,
        kind: if position { GltfAccessorType::Vec3 } else { GltfAccessorType::Scalar },
        max: None,
        min: None,
        sparse: None,
        name: None,
        extensions: None,
        extras: None,
    }
}
fn emit_mesh(out: &mut GltfSnapshot, item: &IfcEntity, points: &[IfcValue], faces: &[IfcValue], m: Matrix, scale: f64, name: &str, p: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let start = out.buffers[0].len();
    let mut low = [f64::INFINITY; 3];
    let mut high = [f64::NEG_INFINITY; 3];
    for row in points {
        let xyz = aggregate(Some(row))?;
        if xyz.len() != 3 {
            return Err(invalid("IFC tessellation point is not three-dimensional"));
        }
        let q = [number(&xyz[0], p)?, number(&xyz[1], p)?, number(&xyz[2], p)?];
        let world = p.effect(24, || std::array::from_fn::<f64, 3, _>(|r| (m[r][0] * q[0] + m[r][1] * q[1] + m[r][2] * q[2] + m[r][3]) * scale))?;
        let vertex = [world[0] as f32, world[2] as f32, -world[1] as f32];
        if vertex.iter().any(|n| !n.is_finite()) {
            return Err(invalid("IFC transformed coordinate exceeds finite glTF FLOAT storage"));
        }
        for k in 0..3 {
            low[k] = low[k].min(vertex[k] as f64);
            high[k] = high[k].max(vertex[k] as f64);
        }
        p.effect(12, || {
            for n in vertex {
                out.buffers[0].extend_from_slice(&n.to_le_bytes());
            }
        })?;
    }
    let indices_start = out.buffers[0].len();
    let map = match item.args.get(4) {
        Some(IfcValue::Unset) | None => None,
        Some(v) => Some(aggregate(Some(v))?),
    };
    for row in faces {
        let face = aggregate(Some(row))?;
        if face.len() != 3 {
            return Err(invalid("IFC triangulated face does not have three indices"));
        }
        let mut ids = [0u32; 3];
        for k in 0..3 {
            let index = if let Some(map) = map {
                let at = ordinal(&face[k], map.len(), p)?;
                ordinal(&map[at], points.len(), p)?
            } else {
                ordinal(&face[k], points.len(), p)?
            };
            ids[k] = index as u32;
        }
        p.effect(12, || {
            for n in ids {
                out.buffers[0].extend_from_slice(&n.to_le_bytes());
            }
        })?;
    }
    let d = &mut out.document;
    let view = d.buffer_views.len();
    p.effect(size_of::<GltfBufferView>(), || d.buffer_views.push(GltfBufferView { buffer: 0, byte_offset: start, byte_length: points.len() * 12, byte_stride: None, target: Some(34962), name: None, extensions: None, extras: None }))?;
    p.effect(size_of::<GltfBufferView>(), || d.buffer_views.push(GltfBufferView { buffer: 0, byte_offset: indices_start, byte_length: faces.len() * 12, byte_stride: None, target: Some(34963), name: None, extensions: None, extras: None }))?;
    let accessor = d.accessors.len();
    p.effect(size_of::<GltfAccessor>(), || d.accessors.push(accessor_row(view, points.len(), true)))?;
    let a = d.accessors.last_mut().unwrap();
    p.effect(2 * size_of::<Vec<f64>>(), || {
        a.min = Some(Vec::new());
        a.max = Some(Vec::new());
    })?;
    p.vector(3, a.min.as_mut().unwrap())?;
    p.vector(3, a.max.as_mut().unwrap())?;
    for k in 0..3 {
        p.effect(16, || {
            a.min.as_mut().unwrap().push(low[k]);
            a.max.as_mut().unwrap().push(high[k]);
        })?;
    }
    p.effect(size_of::<GltfAccessor>(), || d.accessors.push(accessor_row(view + 1, faces.len() * 3, false)))?;
    let mesh = d.meshes.len();
    p.effect(size_of::<GltfMesh>(), || d.meshes.push(GltfMesh { primitives: Vec::new(), weights: Vec::new(), name: Some(String::new()), extensions: None, extras: None }))?;
    let row = d.meshes.last_mut().unwrap();
    p.text(name.len(), row.name.as_mut().unwrap())?;
    let mut at = 0;
    while at < name.len() {
        let mut end = (at + 256).min(name.len());
        while !name.is_char_boundary(end) {
            end -= 1;
        }
        p.effect(end - at, || row.name.as_mut().unwrap().push_str(&name[at..end]))?;
        at = end;
    }
    p.vector(1, &mut row.primitives)?;
    p.effect(size_of::<GltfPrimitive>(), || row.primitives.push(GltfPrimitive { attributes: Vec::new(), indices: Some(accessor + 1), material: None, mode: Some(4), targets: Vec::new(), extensions: None, extras: None }))?;
    let primitive = &mut row.primitives[0];
    p.vector(1, &mut primitive.attributes)?;
    p.effect(size_of::<(String, usize)>(), || primitive.attributes.push((String::new(), accessor)))?;
    p.text(8, &mut primitive.attributes[0].0)?;
    p.effect(8, || primitive.attributes[0].0.push_str("POSITION"))?;
    p.effect(size_of::<GltfNode>(), || d.nodes.push(GltfNode { mesh: Some(mesh), ..GltfNode::default() }))?;
    p.effect(size_of::<usize>(), || d.scenes[0].nodes.push(mesh))?;
    Ok(())
}
pub(super) fn receive(source: &IfcSnapshot, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<GltfSnapshot, ValueError> {
    owner.receive::<Receiving, GltfSnapshot>(|slot, native, body| {
        native.scoped_stage(|native| {
            native.begin_stage(0)?;
            let mut p = Port { native, body };
            p.effect(size_of::<Receiving>(), || *slot = Some(Receiving::default()))?;
            let state = slot.as_mut().unwrap();
            prepare(source, state, &mut p)?;
            for row in &source.entities {
                product(source, state, row, false, &mut p)?;
            }
            if state.primitives == 0 {
                return Err(invalid("IFC graph has no supported Body triangle tessellation"));
            }
            output(state, &mut p)?;
            for row in &source.entities {
                product(source, state, row, true, &mut p)?;
            }
            p.effect(size_of::<GltfSnapshot>(), || ())?;
            Ok(state.target.take().unwrap())
        })
    })
}
