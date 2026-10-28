//! 🏗️ Native analytical output receives each graph frontier through the unchanged original snapshot wallet.
use super::fem_owned::{axis, copy, invalid, unsupported, vector, work};
use super::*;
use semio_framework_os_kernel::NativeSnapshotDecodeOwner as Owner;
use semio_framework_value::{RetireOwned, ValueError};
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::{
    snapshot::{Fem3dSnapshot, FemIdentityIndex, FemIdentityKind},
    structure::{FemDof, FemElement},
};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcHeader, IfcTypedValue};
use std::mem::size_of;
#[path = "native_fem_load_build.rs"]
mod loads;
#[derive(Clone, Copy)]
enum V<'a> {
    U,
    D,
    I(i64),
    R(f64),
    Ref(u64),
    S(&'a str),
    E(&'a str),
    A(&'a [V<'a>]),
    T(&'a str, &'a V<'a>),
}
fn value(source: &V<'_>, owner: &mut Owner<'_, '_>) -> Result<IfcValue, ValueError> {
    owner.receive_nested::<IfcValue, IfcValue>(|slot, original| {
        work(original, size_of::<IfcValue>())?;
        *slot = Some(match source {
            V::U => IfcValue::Unset,
            V::D => IfcValue::Derived,
            V::I(v) => IfcValue::Integer(*v),
            V::R(v) if v.is_finite() => IfcValue::Real(*v),
            V::R(_) => return Err(invalid("finite analytical scalar required")),
            V::Ref(v) => IfcValue::Reference(*v),
            V::S(_) => IfcValue::String(String::new()),
            V::E(_) => IfcValue::Enum(String::new()),
            V::A(_) => IfcValue::Aggregate(Vec::new()),
            V::T(_, _) => IfcValue::TypedValue(IfcTypedValue { name: String::new(), items: Vec::new() }),
        });
        match (source, slot.as_mut().unwrap()) {
            (V::S(v), IfcValue::String(out)) | (V::E(v), IfcValue::Enum(out)) => *out = copy(v, original)?,
            (V::A(values), IfcValue::Aggregate(out)) => {
                *out = vector(values.len(), original)?;
                for v in *values {
                    work(original, size_of::<IfcValue>())?;
                    out.push(value(v, original)?);
                }
            }
            (V::T(name, v), IfcValue::TypedValue(out)) => {
                out.name = copy(name, original)?;
                out.items = vector(1, original)?;
                work(original, size_of::<IfcValue>())?;
                out.items.push(value(v, original)?);
            }
            _ => {}
        }
        Ok(slot.take().unwrap())
    })
}
fn values(source: &[V<'_>], owner: &mut Owner<'_, '_>) -> Result<Vec<IfcValue>, ValueError> {
    owner.receive_nested::<Vec<IfcValue>, Vec<IfcValue>>(|slot, original| {
        *slot = Some(vector(source.len(), original)?);
        for v in source {
            work(original, size_of::<IfcValue>())?;
            slot.as_mut().unwrap().push(value(v, original)?);
        }
        Ok(slot.take().unwrap())
    })
}
fn add(snapshot: &mut IfcSnapshot, name: &str, args: &[V<'_>], owner: &mut Owner<'_, '_>) -> Result<u64, ValueError> {
    if snapshot.entities.len() == snapshot.entities.capacity() {
        return Err(invalid("analytical graph exceeded its admitted backing"));
    }
    work(owner, size_of::<IfcEntity>())?;
    let id = snapshot.entities.len() as u64 + 1;
    let entity = owner.receive_nested::<IfcEntity, IfcEntity>(|slot, original| {
        work(original, size_of::<IfcEntity>())?;
        *slot = Some(IfcEntity { id, name: String::new(), args: Vec::new(), complex: Vec::new() });
        let row = slot.as_mut().unwrap();
        row.name = copy(name, original)?;
        row.args = values(args, original)?;
        Ok(slot.take().unwrap())
    })?;
    snapshot.entities.push(entity);
    Ok(id)
}
fn guid(id: u64, bytes: &mut [u8; 22]) -> &str {
    bytes.fill(b'0');
    let mut n = id;
    let mut at = 22;
    while n > 0 {
        at -= 1;
        bytes[at] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    std::str::from_utf8(bytes).expect("decimal IFC GUID")
}
fn root(snapshot: &mut IfcSnapshot, kind: &str, name: &str, tail: &[V<'_>], owner: &mut Owner<'_, '_>) -> Result<u64, ValueError> {
    owner.receive_nested::<Vec<IfcValue>, u64>(|slot, original| {
        let id = snapshot.entities.len() as u64 + 1;
        let mut bytes = [0; 22];
        *slot = Some(vector(4 + tail.len(), original)?);
        let args = slot.as_mut().unwrap();
        for v in &[V::S(guid(id, &mut bytes)), V::U, V::S(name), V::U] {
            work(original, size_of::<IfcValue>())?;
            args.push(value(v, original)?);
        }
        for v in tail {
            work(original, size_of::<IfcValue>())?;
            args.push(value(v, original)?);
        }
        if snapshot.entities.len() == snapshot.entities.capacity() {
            return Err(invalid("analytical graph exceeded its admitted backing"));
        }
        work(original, size_of::<IfcEntity>())?;
        let entity = original.receive_nested::<IfcEntity, IfcEntity>(|row, child| {
            work(child, size_of::<IfcEntity>())?;
            *row = Some(IfcEntity { id, name: String::new(), args: Vec::new(), complex: Vec::new() });
            row.as_mut().unwrap().name = copy(kind, child)?;
            row.as_mut().unwrap().args = slot.take().unwrap();
            Ok(row.take().unwrap())
        })?;
        snapshot.entities.push(entity);
        Ok(id)
    })
}
fn topology(snapshot: &mut IfcSnapshot, context: u64, item: u64, kind: &str, owner: &mut Owner<'_, '_>) -> Result<u64, ValueError> {
    let rep = add(snapshot, "IFCTOPOLOGYREPRESENTATION", &[V::Ref(context), V::S("Reference"), V::S(kind), V::A(&[V::Ref(item)])], owner)?;
    add(snapshot, "IFCPRODUCTDEFINITIONSHAPE", &[V::U, V::U, V::A(&[V::Ref(rep)])], owner)
}
fn property(snapshot: &mut IfcSnapshot, name: &str, kind: &str, scalar: f64, unit: Option<u64>, owner: &mut Owner<'_, '_>) -> Result<u64, ValueError> {
    add(snapshot, "IFCPROPERTYSINGLEVALUE", &[V::S(name), V::U, V::T(kind, &V::R(scalar)), unit.map(V::Ref).unwrap_or(V::U)], owner)
}
#[derive(Default, RetireOwned)]
struct Build {
    identity: Option<FemIdentityIndex>,
    support_nodes: Vec<(usize, usize)>,
    snapshot: Option<IfcSnapshot>,
    nodes: Vec<(u64, u64)>,
    materials: Vec<u64>,
    sections: Vec<u64>,
    members: Vec<u64>,
    incident: Vec<bool>,
    load_cases: Option<Vec<IfcValue>>,
}
fn node(source: &Fem3dSnapshot, index: &FemIdentityIndex, id: &str, owner: &mut Owner<'_, '_>) -> Result<usize, ValueError> {
    index.lookup(source, FemIdentityKind::Node, id, owner)
}
fn material(source: &Fem3dSnapshot, index: &FemIdentityIndex, id: &str, owner: &mut Owner<'_, '_>) -> Result<usize, ValueError> {
    index.lookup(source, FemIdentityKind::Material, id, owner)
}
fn section(source: &Fem3dSnapshot, index: &FemIdentityIndex, id: &str, owner: &mut Owner<'_, '_>) -> Result<usize, ValueError> {
    index.lookup(source, FemIdentityKind::Section, id, owner)
}
fn support(rows: &[(usize, usize)], node: usize, owner: &mut Owner<'_, '_>) -> Result<Option<usize>, ValueError> {
    let (mut lo, mut hi) = (0, rows.len());
    while lo < hi {
        work(owner, 2 * size_of::<(usize, usize)>())?;
        let m = lo + (hi - lo) / 2;
        match rows[m].0.cmp(&node) {
            std::cmp::Ordering::Less => lo = m + 1,
            std::cmp::Ordering::Greater => hi = m,
            std::cmp::Ordering::Equal => return Ok(Some(rows[m].1)),
        }
    }
    Ok(None)
}
fn admission(source: &Fem3dSnapshot, owner: &mut Owner<'_, '_>) -> Result<usize, ValueError> {
    if source.nodes.is_empty() || !source.solids.is_empty() || !source.combinations.is_empty() {
        return Err(unsupported("native FEM graph construction requires nonempty analytical topology without solids or combinations"));
    }
    let extent = 32usize
        .checked_add(source.nodes.len().checked_mul(6).ok_or_else(|| invalid("analytical row extent overflow"))?)
        .and_then(|v| v.checked_add(source.supports.len()))
        .and_then(|v| v.checked_add(source.materials.len().checked_mul(6)?))
        .and_then(|v| v.checked_add(source.sections.len().checked_mul(6)?))
        .and_then(|v| v.checked_add(source.elements.len().checked_mul(9)?))
        .ok_or_else(|| invalid("analytical graph extent overflow"))?;
    let extent = extent.checked_add(loads::extent(source, owner)?).ok_or_else(|| invalid("analytical graph extent overflow"))?;
    if extent > 64000 {
        return Err(unsupported("analytical output exceeds the owned graph entity admission"));
    }
    Ok(extent)
}

/// 📤️ Builds an ADD2 analytical graph with funded values, identities, collections and failure retirement.
pub(super) fn receive(source: &Fem3dSnapshot, owner: &mut Owner<'_, '_>) -> Result<IfcSnapshot, ValueError> {
    owner.scoped_stage(|owner| owner.receive_nested::<Build, IfcSnapshot>(|slot, original| {
        original.native().begin_stage(0)?;
        let extent = admission(source, original)?;
        work(original, size_of::<Build>())?;
        *slot = Some(Build::default());
        let frame = slot.as_mut().unwrap();
        frame.identity = Some(FemIdentityIndex::receive(source, original)?);
        frame.support_nodes = vector(source.supports.len(), original)?;
        let identity = frame.identity.as_ref().unwrap();
        frame.incident = vector(source.nodes.len(), original)?;
        for _ in 0..source.nodes.len() {
            work(original, size_of::<bool>())?;
            frame.incident.push(false);
        }
        for element in &source.elements {
            let (start, end) = match element {
                FemElement::Bar { start, end, .. } | FemElement::Frame { start, end, .. } => (start, end),
            };
            let start = node(source, identity, start, original)?;
            let end = node(source, identity, end, original)?;
            work(original, 2 * size_of::<bool>())?;
            frame.incident[start] = true;
            frame.incident[end] = true;
        }
        for incident in &frame.incident {
            work(original, size_of::<bool>())?;
            if !incident {
                return Err(unsupported("native analytical point connections require an incident member"));
            }
        }
        for (i, row) in source.supports.iter().enumerate() {
            let rank = node(source, identity, &row.node_id, original)?;
            if row.fixed.len() > 6 {
                return Err(invalid("analytical support has too many restraint degrees of freedom"));
            }
            for (j, d) in row.fixed.iter().enumerate() {
                work(original, 8)?;
                if row.fixed[..j].contains(d) {
                    return Err(invalid("analytical restraint degree of freedom repeats"));
                }
            }
            work(original, size_of::<(usize, usize)>())?;
            frame.support_nodes.push((rank, i));
        }
        super::fem_owned::sort(&mut frame.support_nodes, original, |a, b| a.0.cmp(&b.0))?;
        for pair in frame.support_nodes.windows(2) {
            work(original, 2 * size_of::<(usize, usize)>())?;
            if pair[0].0 == pair[1].0 {
                return Err(invalid("one analytical support per node is required"));
            }
        }
        work(original, size_of::<IfcSnapshot>())?;
        frame.snapshot = Some(IfcSnapshot { schema: String::new(), header: IfcHeader { file_description: Vec::new(), file_name: Vec::new(), file_schema: Vec::new() }, entities: Vec::new() });
        let graph = frame.snapshot.as_mut().unwrap();
        graph.schema = copy("stdio.ifc", original)?;
        graph.entities = vector(extent, original)?;
        graph.header.file_description = values(&[V::A(&[V::S("ViewDefinition [notYetAssigned]")]), V::S("2;1")], original)?;
        graph.header.file_name = values(&[V::S("FEM3d.ifc"), V::S(""), V::A(&[V::S("")]), V::A(&[V::S("")]), V::S("semio"), V::S("semio"), V::S("")], original)?;
        graph.header.file_schema = values(&[V::A(&[V::S("IFC4X3_ADD2")])], original)?;
        let point = add(graph, "IFCCARTESIANPOINT", &[V::A(&[V::R(0.), V::R(0.), V::R(0.)])], original)?;
        let basis = add(graph, "IFCAXIS2PLACEMENT3D", &[V::Ref(point), V::U, V::U], original)?;
        let context = add(graph, "IFCGEOMETRICREPRESENTATIONCONTEXT", &[V::U, V::S("Model"), V::I(3), V::R(1e-6), V::Ref(basis), V::U], original)?;
        let metre = add(graph, "IFCSIUNIT", &[V::D, V::E("LENGTHUNIT"), V::U, V::E("METRE")], original)?;
        let area = add(graph, "IFCSIUNIT", &[V::D, V::E("AREAUNIT"), V::U, V::E("SQUARE_METRE")], original)?;
        let pressure = add(graph, "IFCSIUNIT", &[V::D, V::E("PRESSUREUNIT"), V::U, V::E("PASCAL")], original)?;
        let mass = add(graph, "IFCSIUNIT", &[V::D, V::E("MASSUNIT"), V::E("KILO"), V::E("GRAM")], original)?;
        let kg = add(graph, "IFCDERIVEDUNITELEMENT", &[V::Ref(mass), V::I(1)], original)?;
        let cubic = add(graph, "IFCDERIVEDUNITELEMENT", &[V::Ref(metre), V::I(-3)], original)?;
        let density = add(graph, "IFCDERIVEDUNIT", &[V::A(&[V::Ref(kg), V::Ref(cubic)]), V::E("MASSDENSITYUNIT"), V::U, V::U], original)?;
        let quartic = add(graph, "IFCDERIVEDUNITELEMENT", &[V::Ref(metre), V::I(4)], original)?;
        let inertia = add(graph, "IFCDERIVEDUNIT", &[V::A(&[V::Ref(quartic)]), V::E("MOMENTOFINERTIAUNIT"), V::U, V::U], original)?;
        let force = add(graph, "IFCSIUNIT", &[V::D, V::E("FORCEUNIT"), V::U, V::E("NEWTON")], original)?;
        let torque_force = add(graph, "IFCDERIVEDUNITELEMENT", &[V::Ref(force), V::I(1)], original)?;
        let torque_length = add(graph, "IFCDERIVEDUNITELEMENT", &[V::Ref(metre), V::I(1)], original)?;
        let torque = add(graph, "IFCDERIVEDUNIT", &[V::A(&[V::Ref(torque_force), V::Ref(torque_length)]), V::E("TORQUEUNIT"), V::U, V::U], original)?;
        let units = add(graph, "IFCUNITASSIGNMENT", &[V::A(&[V::Ref(metre), V::Ref(area), V::Ref(pressure), V::Ref(mass), V::Ref(density), V::Ref(inertia), V::Ref(force), V::Ref(torque)])], original)?;
        let project = root(graph, "IFCPROJECT", "FEM3d analytical projection", &[V::U, V::U, V::U, V::A(&[V::Ref(context)]), V::Ref(units)], original)?;
        frame.nodes = vector(source.nodes.len(), original)?;
        frame.materials = vector(source.materials.len(), original)?;
        frame.sections = vector(source.sections.len(), original)?;
        frame.members = vector(source.elements.len(), original)?;
        for m in source.materials.iter() {
            work(original, 48)?;
            if !m.e.is_finite() || !m.g.is_finite() || !m.nu.is_finite() || !m.rho.is_finite() || m.e <= 0. || m.g <= 0. || m.rho <= 0. || m.nu <= -1. || m.nu >= 0.5 {
                return Err(invalid("analytical isotropic material is outside physical bounds"));
            }
            let material = add(graph, "IFCMATERIAL", &[V::S(m.name.as_str()), V::U, V::U], original)?;
            let e = property(graph, "YoungModulus", "IFCMODULUSOFELASTICITYMEASURE", m.e, Some(pressure), original)?;
            let g = property(graph, "ShearModulus", "IFCMODULUSOFELASTICITYMEASURE", m.g, Some(pressure), original)?;
            let nu = property(graph, "PoissonRatio", "IFCRATIOMEASURE", m.nu, None, original)?;
            let rho = property(graph, "MassDensity", "IFCMASSDENSITYMEASURE", m.rho, Some(density), original)?;
            add(graph, "IFCMATERIALPROPERTIES", &[V::S("semio_FemMaterial"), V::U, V::A(&[V::Ref(e), V::Ref(g), V::Ref(nu), V::Ref(rho)]), V::Ref(material)], original)?;
            work(original, 8)?;
            frame.materials.push(material);
        }
        for s in &source.sections {
            work(original, 32)?;
            if [s.area, s.iy, s.iz, s.j].iter().any(|v| !v.is_finite() || *v <= 0.) {
                return Err(invalid("analytical section properties must be finite and positive"));
            }
            let a = property(graph, "CrossSectionArea", "IFCAREAMEASURE", s.area, Some(area), original)?;
            let iy = property(graph, "MomentOfInertiaY", "IFCMOMENTOFINERTIAMEASURE", s.iy, Some(inertia), original)?;
            let iz = property(graph, "MomentOfInertiaZ", "IFCMOMENTOFINERTIAMEASURE", s.iz, Some(inertia), original)?;
            let j = property(graph, "TorsionalConstant", "IFCMOMENTOFINERTIAMEASURE", s.j, Some(inertia), original)?;
            let name = add(graph, "IFCPROPERTYSINGLEVALUE", &[V::S("SectionName"), V::U, V::T("IFCLABEL", &V::S(&s.name)), V::U], original)?;
            let pset = root(graph, "IFCPROPERTYSET", "semio_FemSection", &[V::A(&[V::Ref(a), V::Ref(iy), V::Ref(iz), V::Ref(j), V::Ref(name)])], original)?;
            work(original, 8)?;
            frame.sections.push(pset);
        }
        for (node_index, n) in source.nodes.iter().enumerate() {
            work(original, 24)?;
            if [n.x, n.y, n.z].iter().any(|v| !v.is_finite()) {
                return Err(invalid("analytical node coordinates must be finite"));
            }
            let p = add(graph, "IFCCARTESIANPOINT", &[V::A(&[V::R(n.x), V::R(n.y), V::R(n.z)])], original)?;
            let v = add(graph, "IFCVERTEXPOINT", &[V::Ref(p)], original)?;
            let shape = topology(graph, context, v, "Vertex", original)?;
            let support = match support(&frame.support_nodes, node_index, original)? {
                Some(row) => Some(&source.supports[row]),
                None => None,
            };
            let condition = if let Some(s) = support {
                let fixed = |d| if s.fixed.contains(&d) { "T" } else { "F" };
                Some(add(
                    graph,
                    "IFCBOUNDARYNODECONDITION",
                    &[
                        V::S(&s.id),
                        V::T("IFCBOOLEAN", &V::E(fixed(FemDof::Tx))),
                        V::T("IFCBOOLEAN", &V::E(fixed(FemDof::Ty))),
                        V::T("IFCBOOLEAN", &V::E(fixed(FemDof::Tz))),
                        V::T("IFCBOOLEAN", &V::E(fixed(FemDof::Rx))),
                        V::T("IFCBOOLEAN", &V::E(fixed(FemDof::Ry))),
                        V::T("IFCBOOLEAN", &V::E(fixed(FemDof::Rz))),
                    ],
                    original,
                )?)
            } else {
                None
            };
            let connection = root(graph, "IFCSTRUCTURALPOINTCONNECTION", n.id.as_str(), &[V::U, V::U, V::Ref(shape), condition.map(V::Ref).unwrap_or(V::U), V::U], original)?;
            work(original, 16)?;
            frame.nodes.push((connection, v));
        }
        for e in &source.elements {
            let (id, start, end, mat, sec, pinned) = match e {
                FemElement::Bar { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id, true),
                FemElement::Frame { id, start, end, material_id, section_id, roll } if *roll == 0. => (id, start, end, material_id, section_id, false),
                _ => return Err(unsupported("native analytical export requires zero frame roll")),
            };
            let si = node(source, identity, start, original)?;
            let ei = node(source, identity, end, original)?;
            let mi = material(source, identity, mat, original)?;
            let pi = section(source, identity, sec, original)?;
            let s = source.nodes.get(si).ok_or_else(|| invalid("analytical start row is absent"))?;
            let t = source.nodes.get(ei).ok_or_else(|| invalid("analytical end row is absent"))?;
            let z = axis([s.x, s.y, s.z], [t.x, t.y, t.z])?;
            let direction = add(graph, "IFCDIRECTION", &[V::A(&[V::R(z[0]), V::R(z[1]), V::R(z[2])])], original)?;
            let edge = add(graph, "IFCEDGE", &[V::Ref(frame.nodes[si].1), V::Ref(frame.nodes[ei].1)], original)?;
            let shape = topology(graph, context, edge, "Edge", original)?;
            let member = root(graph, "IFCSTRUCTURALCURVEMEMBER", id, &[V::U, V::U, V::Ref(shape), V::E(if pinned { "PIN_JOINED_MEMBER" } else { "RIGID_JOINED_MEMBER" }), V::Ref(direction)], original)?;
            root(graph, "IFCRELASSOCIATESMATERIAL", "", &[V::A(&[V::Ref(member)]), V::Ref(frame.materials[mi])], original)?;
            root(graph, "IFCRELDEFINESBYPROPERTIES", "", &[V::A(&[V::Ref(member)]), V::Ref(frame.sections[pi])], original)?;
            for index in [si, ei] {
                root(graph, "IFCRELCONNECTSSTRUCTURALMEMBER", "", &[V::Ref(member), V::Ref(frame.nodes[index].0), V::U, V::U, V::U, V::U], original)?;
            }
            work(original, 8)?;
            frame.members.push(member);
        }
        frame.load_cases = Some(loads::receive(source, identity, &frame.nodes, graph, original)?);
        let analysis = root(graph, "IFCSTRUCTURALANALYSISMODEL", "FEM3d", &[V::U, V::E("LOADING_3D"), V::U, V::U, V::U, V::U], original)?;
        if !frame.load_cases.as_ref().unwrap().is_empty() {
            work(original, size_of::<IfcValue>() + size_of::<Vec<IfcValue>>())?;
            graph.entities.last_mut().unwrap().args[7] = IfcValue::Aggregate(frame.load_cases.take().unwrap());
        }
        original.receive_nested::<Vec<IfcValue>, ()>(|objects, child| {
            *objects = Some(vector(frame.nodes.len() + frame.members.len(), child)?);
            for id in frame.nodes.iter().map(|v| v.0).chain(frame.members.iter().copied()) {
                work(child, size_of::<IfcValue>())?;
                objects.as_mut().unwrap().push(IfcValue::Reference(id));
            }
            root_with_objects(graph, analysis, objects, child)
        })?;
        root(graph, "IFCRELDECLARES", "", &[V::Ref(project), V::A(&[V::Ref(analysis)])], original)?;
        work(original, size_of::<IfcSnapshot>())?;
        Ok(frame.snapshot.take().unwrap())
    }))
}
fn root_with_objects(graph: &mut IfcSnapshot, analysis: u64, objects: &mut Option<Vec<IfcValue>>, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
    if graph.entities.len() == graph.entities.capacity() {
        return Err(invalid("analytical graph exceeded its admitted backing"));
    }
    owner.receive_nested::<IfcEntity, ()>(|slot, original| {
        let id = graph.entities.len() as u64 + 1;
        work(original, size_of::<IfcEntity>())?;
        *slot = Some(IfcEntity { id, name: String::new(), args: Vec::new(), complex: Vec::new() });
        let row = slot.as_mut().unwrap();
        row.name = copy("IFCRELASSIGNSTOGROUP", original)?;
        row.args = vector(7, original)?;
        let mut bytes = [0; 22];
        for v in [V::S(guid(id, &mut bytes)), V::U, V::S(""), V::U] {
            work(original, size_of::<IfcValue>())?;
            row.args.push(value(&v, original)?);
        }
        work(original, 3 * size_of::<IfcValue>() + size_of::<IfcEntity>())?;
        row.args.push(IfcValue::Aggregate(objects.take().unwrap()));
        row.args.push(IfcValue::Unset);
        row.args.push(IfcValue::Reference(analysis));
        graph.entities.push(slot.take().unwrap());
        Ok(())
    })
}
