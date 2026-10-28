//! 🫴️ Analytical projection keeps lookup scratch and every authored destination in the original native receiving frame.
use super::*;
use semio_framework_os_kernel::NativeSnapshotDecodeOwner as Owner;
use semio_framework_value::{RetireOwned, ValueError, ValueRefusalKind, retained_clone::RetainedCloneProgress};
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::{
    FemMaterialValue, FemNodeValue,
    snapshot::{Fem3dSnapshot, Fem3dSnapshotValue},
    structure::{FemDof, FemElement, FemSection, FemSupport},
};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcEntity;
use std::mem::size_of;
#[path = "native_fem_load_query.rs"]
mod load_query;
#[path = "native_fem_load_import.rs"]
mod load_import;
pub(super) fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
pub(super) fn unsupported(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, message)
}
pub(super) fn work(owner: &mut Owner<'_, '_>, bytes: usize) -> Result<(), ValueError> {
    owner.native().checkpoint()?;
    let g = owner.remaining_grant();
    if g.maximum_items == 0 || g.maximum_depth == 0 || g.maximum_copy_bytes < bytes {
        return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "original IFC analytical projection wallet cannot admit its next borrowed work span"));
    }
    owner.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
    owner.native().step()
}
pub(super) fn vector<T: RetireOwned>(count: usize, owner: &mut Owner<'_, '_>) -> Result<Vec<T>, ValueError> {
    owner.receive::<Vec<T>, Vec<T>>(|slot, native, body| {
        *slot = Some(Vec::new());
        body.allocate_vec_into(native, count, slot.as_mut().unwrap(), 1)?;
        Ok(slot.take().unwrap())
    })
}
pub(super) fn copy(text: &str, owner: &mut Owner<'_, '_>) -> Result<String, ValueError> {
    owner.receive::<String, String>(|slot, native, body| {
        *slot = Some(String::new());
        body.copy_text_into(native, text, slot.as_mut().unwrap(), 1)?;
        Ok(slot.take().unwrap())
    })
}
fn key(id: u64, buffer: &mut [u8; 23]) -> &str {
    buffer[..3].copy_from_slice(b"ifc");
    let mut value = id;
    let mut end = 23;
    loop {
        end -= 1;
        buffer[end] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    let length = 23 - end;
    buffer.copy_within(end..23, 3);
    std::str::from_utf8(&buffer[..3 + length]).expect("decimal IFC identity")
}
fn list(value: Option<&IfcValue>) -> Result<&[IfcValue], ValueError> {
    value.and_then(IfcValue::as_aggregate).ok_or_else(|| invalid("explicit IFC analytical aggregate required"))
}
fn reference(value: Option<&IfcValue>) -> Result<u64, ValueError> {
    value.and_then(IfcValue::as_reference).filter(|id| *id != 0).ok_or_else(|| invalid("positive IFC analytical entity reference required"))
}
fn number(value: Option<&IfcValue>) -> Result<f64, ValueError> {
    let n = match value {
        Some(IfcValue::Real(n)) => *n,
        Some(IfcValue::Integer(n)) if (-9007199254740992..=9007199254740992).contains(n) => *n as f64,
        _ => return Err(invalid("exact finite analytical binary64 measure required")),
    };
    if n.is_finite() { Ok(n) } else { Err(invalid("finite analytical measure required")) }
}
#[derive(Clone, Copy, RetireOwned)]
struct Relation {
    kind: u8,
    target: u64,
    index: usize,
}
#[derive(Default, RetireOwned)]
struct Projection {
    index: Vec<(u64, usize)>,
    relations: Vec<Relation>,
    vertices: Vec<(u64, u64, [f64; 3])>,
    memberships: Vec<(u64, usize)>,
    material_keys: Vec<u64>,
    section_keys: Vec<u64>,
    value: Option<Fem3dSnapshotValue>,
    node: Option<FemNodeValue>,
    material: Option<FemMaterialValue>,
    section: Option<FemSection>,
    support: Option<FemSupport>,
    element: Option<FemElement>,
    load_state: Option<load_import::State>,
    unit_assignment: u64,
}
pub(super) fn sort<T: Copy>(values: &mut [T], owner: &mut Owner<'_, '_>, compare: impl Fn(&T, &T) -> std::cmp::Ordering + Copy) -> Result<(), ValueError> {
    fn sift<T: Copy>(values: &mut [T], mut root: usize, end: usize, owner: &mut Owner<'_, '_>, compare: impl Fn(&T, &T) -> std::cmp::Ordering + Copy) -> Result<(), ValueError> {
        loop {
            let left = root.saturating_mul(2).saturating_add(1);
            if left >= end {
                return Ok(());
            }
            let mut child = left;
            if left + 1 < end {
                work(owner, 2 * size_of::<T>())?;
                if compare(&values[left], &values[left + 1]).is_lt() {
                    child = left + 1;
                }
            }
            work(owner, 2 * size_of::<T>())?;
            if !compare(&values[root], &values[child]).is_lt() {
                return Ok(());
            }
            work(owner, 2 * size_of::<T>())?;
            values.swap(root, child);
            root = child;
        }
    }
    let length = values.len();
    for root in (0..length / 2).rev() {
        sift(values, root, length, owner, compare)?;
    }
    for end in (1..values.len()).rev() {
        work(owner, 2 * size_of::<T>())?;
        values.swap(0, end);
        sift(values, 0, end, owner, compare)?;
    }
    Ok(())
}
struct Graph<'a> {
    source: &'a IfcSnapshot,
    index: &'a [(u64, usize)],
    relations: &'a [Relation],
}
#[derive(Clone, Copy)]
enum LoadUnit {
    Force,
    Torque,
}
impl LoadUnit {
    fn tag(self) -> &'static str {
        match self { Self::Force => "FORCEUNIT", Self::Torque => "TORQUEUNIT" }
    }
    fn dimensions(self) -> [i32; 7] {
        match self { Self::Force => [1, 1, -2, 0, 0, 0, 0], Self::Torque => [2, 1, -2, 0, 0, 0, 0] }
    }
}
impl<'a> Graph<'a> {
    fn ordinal(&self, id: u64, owner: &mut Owner<'_, '_>) -> Result<usize, ValueError> {
        let (mut lo, mut hi) = (0, self.index.len());
        while lo < hi {
            work(owner, size_of::<(u64, usize)>())?;
            let m = lo + (hi - lo) / 2;
            match self.index[m].0.cmp(&id) {
                std::cmp::Ordering::Less => lo = m + 1,
                std::cmp::Ordering::Greater => hi = m,
                std::cmp::Ordering::Equal => return Ok(self.index[m].1),
            }
        }
        Err(invalid("analytical reference points to an absent IFC entity"))
    }
    fn entity(&self, id: u64, owner: &mut Owner<'_, '_>) -> Result<&'a IfcEntity, ValueError> {
        Ok(&self.source.entities[self.ordinal(id, owner)?])
    }
    fn at(&self, value: Option<&IfcValue>, owner: &mut Owner<'_, '_>) -> Result<&'a IfcEntity, ValueError> {
        self.entity(reference(value)?, owner)
    }
    fn rows(&self, kind: u8, target: u64) -> &'a [Relation] {
        let start = self.relations.partition_point(|r| (r.kind, r.target) < (kind, target));
        let end = start + self.relations[start..].partition_point(|r| r.kind == kind && r.target == target);
        &self.relations[start..end]
    }
    fn xyz(&self, value: Option<&IfcValue>, kind: &str, owner: &mut Owner<'_, '_>) -> Result<[f64; 3], ValueError> {
        let entity = self.at(value, owner)?;
        if entity.name != kind {
            return Err(invalid("unexpected IFC analytical coordinate kind"));
        }
        let v = list(entity.args.first())?;
        if v.len() != 3 {
            return Err(invalid("analytical coordinates require three scalars"));
        }
        Ok([number(v.first())?, number(v.get(1))?, number(v.get(2))?])
    }
    fn identity_axis(&self, value: Option<&IfcValue>, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
        let axis = self.at(value, owner)?;
        if axis.name != "IFCAXIS2PLACEMENT3D" || axis.args.len()!=3 || self.xyz(axis.args.first(), "IFCCARTESIANPOINT", owner)? != [0.; 3] {
            return Err(unsupported("analytical context requires identity global coordinates"));
        }
        if matches!(axis.args.get(1),Some(IfcValue::Unset))!=matches!(axis.args.get(2),Some(IfcValue::Unset)){return Err(invalid("analytical 3D Axis and RefDirection must be supplied together or both omitted"));}
        if axis.args.get(1) != Some(&IfcValue::Unset) {
            let z = self.xyz(axis.args.get(1), "IFCDIRECTION", owner)?;
            if z[0] != 0. || z[1] != 0. || z[2] <= 0. {
                return Err(unsupported("analytical context requires identity global axes"));
            }
        }
        if axis.args.get(2) != Some(&IfcValue::Unset) {
            let x = self.xyz(axis.args.get(2), "IFCDIRECTION", owner)?;
            if x[0] <= 0. || x[1] != 0. || x[2] != 0. {
                return Err(unsupported("analytical context requires identity global axes"));
            }
        }
        Ok(())
    }
    fn item(&self, entity: &IfcEntity, owner: &mut Owner<'_, '_>) -> Result<&'a IfcEntity, ValueError> {
        if entity.args.get(5) != Some(&IfcValue::Unset) {
            return Err(unsupported("analytical object placement requires an additional transform mapping"));
        }
        let shape = self.at(entity.args.get(6), owner)?;
        let reps = list(shape.args.get(2))?;
        if shape.name != "IFCPRODUCTDEFINITIONSHAPE" || reps.len() != 1 {
            return Err(unsupported("one analytical product shape is required"));
        }
        let rep = self.at(reps.first(), owner)?;
        let items = list(rep.args.get(3))?;
        if rep.name != "IFCTOPOLOGYREPRESENTATION" || items.len() != 1 {
            return Err(unsupported("one analytical topology item is required"));
        }
        let context = self.at(rep.args.first(), owner)?;
        if context.name != "IFCGEOMETRICREPRESENTATIONCONTEXT" || context.args.get(2) != Some(&IfcValue::Integer(3)) {
            return Err(unsupported("direct 3D analytical context is required"));
        }
        self.identity_axis(context.args.get(4), owner)?;
        self.at(items.first(), owner)
    }
    fn unit(&self, id: u64, stack: &mut [u64; 64], depth: usize, owner: &mut Owner<'_, '_>) -> Result<([i32; 7], f64), ValueError> {
        work(owner, 8)?;
        if depth == 64 || stack[..depth].contains(&id) {
            return Err(invalid("cyclic or overdeep analytical unit graph"));
        }
        stack[depth] = id;
        let unit = self.entity(id, owner)?;
        if unit.name == "IFCSIUNIT" {
            if unit.args.first() != Some(&IfcValue::Derived) {
                return Err(invalid("SI analytical dimensions must be derived"));
            }
            let (tag, dimensions, scale) = match unit.args.get(3).and_then(IfcValue::as_enum) {
                Some("METRE") => ("LENGTHUNIT", [1, 0, 0, 0, 0, 0, 0], 1.),
                Some("SQUARE_METRE") => ("AREAUNIT", [2, 0, 0, 0, 0, 0, 0], 1.),
                Some("PASCAL") => ("PRESSUREUNIT", [-1, 1, -2, 0, 0, 0, 0], 1.),
                Some("GRAM") => ("MASSUNIT", [0, 1, 0, 0, 0, 0, 0], 0.001),
                Some("NEWTON") => ("FORCEUNIT", [1, 1, -2, 0, 0, 0, 0], 1.),
                _ => return Err(unsupported("SI analytical unit is outside native admission")),
            };
            if unit.args.get(1).and_then(IfcValue::as_enum) != Some(tag) {
                return Err(invalid("analytical SI unit tag contradicts dimensions"));
            }
            let prefix = match unit.args.get(2) {
                Some(IfcValue::Unset) => 1.,
                Some(IfcValue::Enum(v)) => match v.as_str() {
                    "KILO" => 1000.,
                    "MILLI" => 0.001,
                    "CENTI" => 0.01,
                    _ => return Err(unsupported("SI analytical prefix is outside native admission")),
                },
                _ => return Err(invalid("invalid SI analytical prefix")),
            };
            return Ok((dimensions, scale * if tag == "AREAUNIT" { prefix * prefix } else { prefix }));
        }
        if unit.name != "IFCDERIVEDUNIT" {
            return Err(unsupported("native analytical engineering units require SI and derived dimensions"));
        }
        let mut dimensions = [0i32; 7];
        let mut scale = 1.0f64;
        let elements = list(unit.args.first())?;
        if elements.is_empty() {
            return Err(invalid("derived analytical unit is empty"));
        }
        for v in elements {
            work(owner, 8)?;
            let e = self.at(Some(v), owner)?;
            let Some(IfcValue::Integer(power)) = e.args.get(1) else {
                return Err(invalid("integral analytical unit exponent required"));
            };
            if e.name != "IFCDERIVEDUNITELEMENT" || !(-16..=16).contains(power) {
                return Err(invalid("analytical derived unit component is outside bounds"));
            }
            let id=reference(e.args.first())?;
            let child=self.entity(id,owner)?;
            if child.name!="IFCSIUNIT" {
                return Err(unsupported("derived analytical elements require supported named SI units"));
            }
            let (d, s) = self.unit(id, stack, depth + 1, owner)?;
            for i in 0..7 {
                dimensions[i] = dimensions[i].checked_add(d[i] * (*power as i32)).ok_or_else(|| invalid("analytical unit dimension overflow"))?;
            }
            scale *= s.powi(*power as i32);
        }
        let expected = match unit.args.get(1).and_then(IfcValue::as_enum) {
            Some("MASSDENSITYUNIT") => [-3, 1, 0, 0, 0, 0, 0],
            Some("MOMENTOFINERTIAUNIT") => [4, 0, 0, 0, 0, 0, 0],
            Some("TORQUEUNIT") => [2, 1, -2, 0, 0, 0, 0],
            _ => return Err(unsupported("derived analytical unit tag is outside native admission")),
        };
        if dimensions != expected || !scale.is_finite() || scale <= 0. {
            return Err(invalid("analytical derived unit declaration contradicts its dimensions"));
        }
        Ok((dimensions, scale))
    }
    fn require_load_unit(&self, assignment: &IfcEntity, role: LoadUnit, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
        owner.scoped_stage(|original| {
            original.native().begin_stage(0)?;
            work(original, size_of::<u64>())?;
            if assignment.name != "IFCUNITASSIGNMENT" {
                return Err(invalid("structural load requires a declared project unit assignment"));
            }
            let mut selected = None;
            for value in list(assignment.args.first())? {
                work(original, size_of::<u64>())?;
                let unit = self.at(Some(value), original)?;
                if unit.args.get(1).and_then(IfcValue::as_enum) == Some(role.tag()) {
                    if selected.replace(unit.id).is_some() {
                        return Err(invalid("structural load project unit role is ambiguous"));
                    }
                }
            }
            let id = selected.ok_or_else(|| invalid("explicit structural load project unit role is absent"))?;
            if self.unit(id, &mut [0; 64], 0, original)? != (role.dimensions(), 1.) {
                return Err(unsupported("structural load projection requires exact unscaled SI dimensions"));
            }
            Ok(())
        })
    }
    fn property(&self, definition: &IfcEntity, index: usize, name: &str, owner: &mut Owner<'_, '_>) -> Result<&'a IfcEntity, ValueError> {
        let mut found = None;
        for value in list(definition.args.get(index))? {
            work(owner, name.len())?;
            let property = self.at(Some(value), owner)?;
            if property.name != "IFCPROPERTYSINGLEVALUE" {
                return Err(unsupported("canonical analytical definitions require single values"));
            }
            if property.args.first().and_then(IfcValue::as_str) == Some(name) {
                if found.is_some() {
                    return Err(invalid("analytical property is repeated"));
                }
                found = Some(property);
            }
        }
        found.ok_or_else(|| invalid("necessary analytical property is absent"))
    }
    fn measure(&self, property: &IfcEntity, kind: &str, dimensions: [i32; 7], owner: &mut Owner<'_, '_>) -> Result<f64, ValueError> {
        let Some(IfcValue::TypedValue(v)) = property.args.get(2) else {
            return Err(invalid("typed analytical engineering property required"));
        };
        if v.name != kind || v.items.len() != 1 {
            return Err(invalid("analytical engineering selector differs from declaration"));
        }
        if dimensions == [0; 7] {
            if property.args.get(3) != Some(&IfcValue::Unset) {
                return Err(invalid("dimensionless engineering ratio cannot carry a physical unit"));
            }
        } else if self.unit(reference(property.args.get(3))?, &mut [0; 64], 0, owner)? != (dimensions, 1.) {
            return Err(invalid("analytical engineering unit requires exact SI dimensions and scale"));
        }
        number(v.items.first())
    }
}

fn definition<'a>(graph: &Graph<'a>, kind: u8, target: u64, name: &str, owner: &mut Owner<'_, '_>) -> Result<&'a IfcEntity, ValueError> {
    let mut found = None;
    for entry in graph.rows(kind, target) {
        work(owner, name.len())?;
        let relation = &graph.source.entities[entry.index];
        let value = if kind == 4 { relation } else { graph.at(relation.args.get(5), owner)? };
        if value.args.first().and_then(IfcValue::as_str) != Some(name) && value.args.get(2).and_then(IfcValue::as_str) != Some(name) {
            continue;
        }
        if let Some(previous) = found {
            if previous != value.id {
                return Err(invalid("analytical canonical definition is ambiguous"));
            }
        }
        found = Some(value.id);
    }
    graph.entity(found.ok_or_else(|| invalid("necessary analytical canonical definition is absent"))?, owner)
}
fn engineering(graph: &Graph<'_>, definition: &IfcEntity, index: usize, name: &str, kind: &str, dimensions: [i32; 7], owner: &mut Owner<'_, '_>) -> Result<f64, ValueError> {
    let p = graph.property(definition, index, name, owner)?;
    graph.measure(p, kind, dimensions, owner)
}
fn build_index(source: &IfcSnapshot, frame: &mut Projection, owner: &mut Owner<'_, '_>) -> Result<(usize, usize, u64), ValueError> {
    if source.schema != "stdio.ifc" || !matches!(source.header.file_schema.as_slice(),[IfcValue::Aggregate(names)]if matches!(names.as_slice(),[IfcValue::String(name)]if name=="IFC4X3_ADD2")) {
        return Err(invalid("native analytical projection requires the exact ADD2 graph identity"));
    }
    if source.entities.len() > 64000 {
        return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "native owned analytical source exceeds entity admission"));
    }
    frame.index = vector(source.entities.len(), owner)?;
    let (mut nodes, mut members, mut relation_count, mut analysis) = (0, 0, 0usize, None);
    for (index, entity) in source.entities.iter().enumerate() {
        work(owner, size_of::<(u64, usize)>())?;
        if !entity.complex.is_empty() {
            return Err(unsupported("native analytical projection requires simple entity instances"));
        }
        if entity.id == 0 {
            return Err(invalid("IFC analytical identity must be positive"));
        }
        frame.index.push((entity.id, index));
        match entity.name.as_str() {
            "IFCSTRUCTURALPOINTCONNECTION" => nodes += 1,
            "IFCSTRUCTURALCURVEMEMBER" => members += 1,
            "IFCSTRUCTURALANALYSISMODEL" => {
                if analysis.replace(entity.id).is_some() {
                    return Err(unsupported("one direct structural analysis model is required"));
                }
            }
            "IFCRELDEFINESBYPROPERTIES" | "IFCRELASSOCIATESMATERIAL" => {
                relation_count = relation_count.checked_add(list(entity.args.get(4))?.len()).ok_or_else(|| invalid("analytical relationship extent overflow"))?;
            }
            "IFCRELCONNECTSSTRUCTURALMEMBER" | "IFCMATERIALPROPERTIES" | "IFCRELCONNECTSSTRUCTURALACTIVITY" => relation_count = relation_count.checked_add(1).ok_or_else(|| invalid("analytical relationship extent overflow"))?,
            _ if entity.name.starts_with("IFCRELASSIGNSTOGROUP") => relation_count = relation_count.checked_add(1).ok_or_else(|| invalid("analytical relationship extent overflow"))?,
            _ => {}
        }
        if entity.name.starts_with("IFCSTRUCTURAL") && !matches!(entity.name.as_str(), "IFCSTRUCTURALANALYSISMODEL" | "IFCSTRUCTURALPOINTCONNECTION" | "IFCSTRUCTURALCURVEMEMBER" | "IFCSTRUCTURALLOADCASE" | "IFCSTRUCTURALPOINTACTION" | "IFCSTRUCTURALLOADSINGLEFORCE") {
            return Err(unsupported("loaded or specialized structural semantics require additional mappings"));
        }
        if entity.name.starts_with("IFCRELCONNECTSSTRUCTURAL") && !matches!(entity.name.as_str(),"IFCRELCONNECTSSTRUCTURALMEMBER"|"IFCRELCONNECTSSTRUCTURALACTIVITY") {
            return Err(unsupported("specialized structural connection semantics require additional mappings"));
        }
        if matches!(entity.name.as_str(), "IFCMAPCONVERSION" | "IFCMAPCONVERSIONSCALED" | "IFCPROJECTEDCRS" | "IFCRIGIDOPERATION") {
            return Err(unsupported("analytical georeferencing requires an explicit coordinate mapping"));
        }
    }
    if nodes == 0 || nodes + members > 64000 || relation_count > 2_000_000 {
        return Err(unsupported("bounded nonempty analytical topology is required"));
    }
    sort(&mut frame.index, owner, |a, b| a.0.cmp(&b.0))?;
    for pair in frame.index.windows(2) {
        work(owner, 16)?;
        if pair[0].0 == pair[1].0 {
            return Err(invalid("IFC analytical identity is repeated"));
        }
    }
    frame.relations = vector(relation_count, owner)?;
    for (index, entity) in source.entities.iter().enumerate() {
        work(owner, 8)?;
        let kind = match entity.name.as_str() {
            "IFCRELDEFINESBYPROPERTIES" => 1,
            "IFCRELASSOCIATESMATERIAL" => 2,
            "IFCRELCONNECTSSTRUCTURALMEMBER" => 3,
            "IFCMATERIALPROPERTIES" => 4,
            "IFCRELCONNECTSSTRUCTURALACTIVITY" => 6,
            _ if entity.name.starts_with("IFCRELASSIGNSTOGROUP") => 5,
            _ => continue,
        };
        if kind < 3 {
            for target in list(entity.args.get(4))? {
                work(owner, size_of::<Relation>())?;
                frame.relations.push(Relation { kind, target: reference(Some(target))?, index });
            }
        } else if kind==5 || kind==6 {
            let (name,declared)=if kind==5{("RelatingGroup","IFCRELASSIGNSTOGROUP")}else{("RelatedStructuralActivity","IFCRELCONNECTSSTRUCTURALACTIVITY")};
            let target=load_import::reference_at(source,index,name,declared,owner)?;
            work(owner,size_of::<Relation>())?;
            frame.relations.push(Relation{kind,target,index});
        } else {
            work(owner, size_of::<Relation>())?;
            frame.relations.push(Relation { kind, target: reference(entity.args.get(if kind == 3 { 4 } else { 3 }))?, index });
        }
    }
    sort(&mut frame.relations, owner, |a, b| (a.kind, a.target, a.index).cmp(&(b.kind, b.target, b.index)))?;
    Ok((nodes, members, analysis.ok_or_else(|| unsupported("one direct structural analysis model is required"))?))
}
fn graph_values(source: &IfcSnapshot, frame: &Projection, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
    fn identifier(name: &str, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
        if name.is_empty() || !name.as_bytes()[0].is_ascii_uppercase() {
            return Err(invalid("analytical graph type identifier is invalid"));
        }
        for bytes in name.as_bytes().chunks(256) {
            work(owner, bytes.len())?;
            if bytes.iter().any(|v| !v.is_ascii_uppercase() && !v.is_ascii_digit() && *v != b'_') {
                return Err(invalid("analytical graph type identifier is invalid"));
            }
        }
        Ok(())
    }
    fn visit(value: &IfcValue, graph: &Graph<'_>, depth: usize, count: &mut usize, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
        if depth > 64 || *count >= 2_000_000 {
            return Err(unsupported("analytical graph value traversal exceeds explicit admission"));
        }
        *count += 1;
        work(owner, size_of::<IfcValue>())?;
        match value {
            IfcValue::Reference(id) => {
                graph.entity(*id, owner)?;
            }
            IfcValue::Real(n) if !n.is_finite() => return Err(invalid("analytical graph contains a nonfinite binary64 value")),
            IfcValue::String(s) => {
                work(owner, s.len())?;
            }
            IfcValue::Enum(s) => identifier(s, owner)?,
            IfcValue::Aggregate(values) => {
                for v in values {
                    visit(v, graph, depth + 1, count, owner)?;
                }
            }
            IfcValue::TypedValue(v) => {
                identifier(&v.name, owner)?;
                for v in &v.items {
                    visit(v, graph, depth + 1, count, owner)?;
                }
            }
            IfcValue::Binary(_) => return Err(unsupported("binary graph properties require an additional native analytical mapping")),
            _ => {}
        }
        Ok(())
    }
    fn strings(v: &IfcValue, o: &mut Owner<'_, '_>) -> Result<(), ValueError> {
        let values = list(Some(v))?;
        if values.is_empty() {
            return Err(invalid("analytical graph header string list is empty"));
        }
        for v in values {
            work(o, size_of::<IfcValue>())?;
            if !matches!(v, IfcValue::String(_)) {
                return Err(invalid("analytical graph header requires string lists"));
            }
        }
        Ok(())
    }
    let [description, IfcValue::String(_)] = source.header.file_description.as_slice() else {
        return Err(invalid("analytical graph FILE_DESCRIPTION has an invalid Part21 record shape"));
    };
    strings(description, owner)?;
    let [IfcValue::String(_), IfcValue::String(_), authors, organizations, IfcValue::String(_), IfcValue::String(_), IfcValue::String(_)] = source.header.file_name.as_slice() else {
        return Err(invalid("analytical graph FILE_NAME has an invalid Part21 record shape"));
    };
    strings(authors, owner)?;
    strings(organizations, owner)?;
    let graph = Graph { source, index: &frame.index, relations: &frame.relations };
    let mut count = 0;
    for values in [&source.header.file_description, &source.header.file_name, &source.header.file_schema] {
        for v in values {
            visit(v, &graph, 1, &mut count, owner)?;
        }
    }
    for e in &source.entities {
        identifier(&e.name, owner)?;
        for v in &e.args {
            visit(v, &graph, 1, &mut count, owner)?;
        }
    }
    Ok(())
}
fn admit(source: &IfcSnapshot, frame: &mut Projection, analysis: u64, count: usize, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
    let graph = Graph { source, index: &frame.index, relations: &frame.relations };
    let a = graph.entity(analysis, owner)?;
    if a.args.get(5).and_then(IfcValue::as_enum) != Some("LOADING_3D") || [6,8,9].into_iter().any(|i| a.args.get(i) != Some(&IfcValue::Unset)) {
        return Err(unsupported("native analytical projection requires global 3D analysis without results or shared placement"));
    }
    let mut project = None;
    for entity in &source.entities {
        work(owner, 8)?;
        if entity.name == "IFCPROJECT" && project.replace(entity).is_some() {
            return Err(unsupported("one project unit assignment is required"));
        }
        if entity.name == "IFCGEOMETRICREPRESENTATIONCONTEXT" {
            if entity.args.get(2) != Some(&IfcValue::Integer(3)) {
                return Err(unsupported("analytical context requires three dimensions"));
            }
            graph.identity_axis(entity.args.get(4), owner)?;
        }
    }
    let project = project.ok_or_else(|| invalid("analytical project is absent"))?;
    let units = graph.at(project.args.get(8), owner)?;
    if units.name != "IFCUNITASSIGNMENT" {
        return Err(invalid("project unit assignment has the wrong kind"));
    }
    work(owner,size_of::<u64>())?;
    frame.unit_assignment=units.id;
    let mut lengths = 0;
    for unit in list(units.args.first())? {
        work(owner, 8)?;
        let unit = graph.at(Some(unit), owner)?;
        if unit.args.get(1).and_then(IfcValue::as_enum) == Some("LENGTHUNIT") {
            lengths += 1;
            if graph.unit(unit.id, &mut [0; 64], 0, owner)? != ([1, 0, 0, 0, 0, 0, 0], 1.) {
                return Err(unsupported("native analytical projection requires metre project coordinates"));
            }
        }
    }
    if lengths != 1 {
        return Err(invalid("project length units are absent or ambiguous"));
    }
    frame.memberships = vector(count, owner)?;
    for entity in &source.entities {
        work(owner, 8)?;
        if matches!(entity.name.as_str(), "IFCSTRUCTURALPOINTCONNECTION" | "IFCSTRUCTURALCURVEMEMBER") {
            work(owner, size_of::<(u64, usize)>())?;
            frame.memberships.push((entity.id, 0));
        }
    }
    sort(&mut frame.memberships, owner, |a, b| a.0.cmp(&b.0))?;
    for entity in &source.entities {
        work(owner, 8)?;
        if entity.name.starts_with("IFCRELASSIGNSTOGROUP") && entity.args.get(6).and_then(IfcValue::as_reference) == Some(analysis) {
            if entity.name != "IFCRELASSIGNSTOGROUP" {
                return Err(unsupported("factored analysis membership requires explicit load scaling"));
            }
            for value in list(entity.args.get(4))? {
                work(owner, 64)?;
                let id = reference(Some(value))?;
                let index = frame.memberships.binary_search_by_key(&id, |v| v.0).map_err(|_| invalid("analysis membership includes an unsupported object"))?;
                frame.memberships[index].1 += 1;
                if frame.memberships[index].1 != 1 {
                    return Err(invalid("analysis membership is repeated"));
                }
            }
        }
    }
    for (_, count) in &frame.memberships {
        work(owner, 8)?;
        if *count != 1 {
            return Err(invalid("every analytical item must belong exactly once to its analysis"));
        }
    }
    Ok(())
}

fn material_id(graph: &Graph<'_>, member: u64, owner: &mut Owner<'_, '_>) -> Result<u64, ValueError> {
    let mut found = None;
    for r in graph.rows(2, member) {
        work(owner, 8)?;
        let id = reference(graph.source.entities[r.index].args.get(5))?;
        if found.is_some_and(|p| p != id) {
            return Err(invalid("analytical member material is ambiguous"));
        }
        found = Some(id);
    }
    found.ok_or_else(|| invalid("analytical member material is absent"))
}
fn keys(source: &IfcSnapshot, frame: &mut Projection, members: usize, owner: &mut Owner<'_, '_>) -> Result<usize, ValueError> {
    frame.material_keys = vector(members, owner)?;
    frame.section_keys = vector(members, owner)?;
    let graph = Graph { source, index: &frame.index, relations: &frame.relations };
    let mut supports = 0;
    for entity in &source.entities {
        work(owner, 8)?;
        if entity.name == "IFCSTRUCTURALPOINTCONNECTION" && entity.args.get(7) != Some(&IfcValue::Unset) {
            supports += 1;
        }
        if entity.name == "IFCSTRUCTURALCURVEMEMBER" {
            let material = material_id(&graph, entity.id, owner)?;
            let section = definition(&graph, 1, entity.id, "semio_FemSection", owner)?;
            if section.name != "IFCPROPERTYSET" {
                return Err(unsupported("canonical analytical section requires a property set"));
            }
            work(owner, 16)?;
            frame.material_keys.push(material);
            frame.section_keys.push(section.id);
        }
    }
    for values in [&mut frame.material_keys, &mut frame.section_keys] {
        sort(values, owner, |a, b| a.cmp(b))?;
        work(owner, values.len().checked_mul(size_of::<u64>()).ok_or_else(|| invalid("analytical key span overflow"))?)?;
        values.dedup();
    }
    Ok(supports)
}
fn nodes(source: &IfcSnapshot, frame: &mut Projection, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
    let Projection { index, relations, vertices, value, node, support, .. } = frame;
    let graph = Graph { source, index, relations };
    let output = value.as_mut().ok_or_else(|| invalid("analytical destination is absent"))?;
    for entity in &source.entities {
        work(owner, 8)?;
        if entity.name != "IFCSTRUCTURALPOINTCONNECTION" {
            continue;
        }
        if entity.args.get(8) != Some(&IfcValue::Unset) {
            return Err(unsupported("local analytical support axes require additional mapping"));
        }
        let vertex = graph.item(entity, owner)?;
        if vertex.name != "IFCVERTEXPOINT" {
            return Err(unsupported("point connection requires a vertex"));
        }
        let xyz = graph.xyz(vertex.args.first(), "IFCCARTESIANPOINT", owner)?;
        work(owner, size_of::<FemNodeValue>())?;
        *node = Some(FemNodeValue { id: String::new(), x: xyz[0], y: xyz[1], z: xyz[2] });
        node.as_mut().unwrap().id = copy(key(entity.id, &mut [0; 23]), owner)?;
        work(owner, size_of::<FemNodeValue>() + size_of::<(u64, u64, [f64; 3])>())?;
        output.nodes.push(node.take().unwrap());
        vertices.push((vertex.id, entity.id, xyz));
        if entity.args.get(7) == Some(&IfcValue::Unset) {
            continue;
        }
        let condition = graph.at(entity.args.get(7), owner)?;
        if condition.name != "IFCBOUNDARYNODECONDITION" || condition.args.len() != 7 {
            return Err(unsupported("analytical boundary condition requires explicit booleans"));
        }
        let mut fixed = [false; 6];
        let mut count = 0;
        for (i, v) in fixed.iter_mut().enumerate() {
            work(owner, 8)?;
            let Some(IfcValue::TypedValue(p)) = condition.args.get(i + 1) else {
                return Err(unsupported("analytical support requires explicit boolean stiffness selectors"));
            };
            if p.name != "IFCBOOLEAN" || p.items.len() != 1 {
                return Err(unsupported("spring/select stiffness requires additional engineering semantics"));
            }
            *v = match p.items[0].as_enum() {
                Some("T") => true,
                Some("F") => false,
                _ => return Err(invalid("analytical support stiffness is not boolean")),
            };
            count += usize::from(*v);
        }
        work(owner, size_of::<FemSupport>())?;
        *support = Some(FemSupport { id: String::new(), node_id: String::new(), fixed: Vec::new() });
        let v = support.as_mut().unwrap();
        v.id = copy(key(entity.id, &mut [0; 23]), owner)?;
        v.node_id = copy(key(entity.id, &mut [0; 23]), owner)?;
        v.fixed = vector(count, owner)?;
        for (i, dof) in [FemDof::Tx, FemDof::Ty, FemDof::Tz, FemDof::Rx, FemDof::Ry, FemDof::Rz].into_iter().enumerate() {
            work(owner, size_of::<FemDof>())?;
            if fixed[i] {
                v.fixed.push(dof);
            }
        }
        work(owner, size_of::<FemSupport>())?;
        output.supports.push(support.take().unwrap());
    }
    sort(vertices, owner, |a, b| a.0.cmp(&b.0))?;
    for pair in vertices.windows(2) {
        work(owner, 16)?;
        if pair[0].0 == pair[1].0 {
            return Err(invalid("shared analytical vertex has ambiguous connection identity"));
        }
    }
    Ok(())
}
fn definitions(source: &IfcSnapshot, frame: &mut Projection, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
    let Projection { index, relations, value, material, section, material_keys, section_keys, .. } = frame;
    let graph = Graph { source, index, relations };
    let output = value.as_mut().ok_or_else(|| invalid("analytical destination is absent"))?;
    for id in material_keys {
        work(owner, 8)?;
        let row = graph.entity(*id, owner)?;
        if row.name != "IFCMATERIAL" {
            return Err(unsupported("plain analytical material definition is required"));
        }
        let p = definition(&graph, 4, *id, "semio_FemMaterial", owner)?;
        let e = engineering(&graph, p, 2, "YoungModulus", "IFCMODULUSOFELASTICITYMEASURE", [-1, 1, -2, 0, 0, 0, 0], owner)?;
        let g = engineering(&graph, p, 2, "ShearModulus", "IFCMODULUSOFELASTICITYMEASURE", [-1, 1, -2, 0, 0, 0, 0], owner)?;
        let nu = engineering(&graph, p, 2, "PoissonRatio", "IFCRATIOMEASURE", [0; 7], owner)?;
        let rho = engineering(&graph, p, 2, "MassDensity", "IFCMASSDENSITYMEASURE", [-3, 1, 0, 0, 0, 0, 0], owner)?;
        if e <= 0. || g <= 0. || rho <= 0. || nu <= -1. || nu >= 0.5 {
            return Err(invalid("analytical isotropic material is outside physical bounds"));
        }
        work(owner, size_of::<FemMaterialValue>())?;
        *material = Some(FemMaterialValue { id: String::new(), name: String::new(), e, g, nu, rho });
        let m = material.as_mut().unwrap();
        m.id = copy(key(*id, &mut [0; 23]), owner)?;
        m.name = copy(row.args.first().and_then(IfcValue::as_str).unwrap_or(""), owner)?;
        work(owner, size_of::<FemMaterialValue>())?;
        output.materials.push(material.take().unwrap());
    }
    for id in section_keys {
        work(owner, 8)?;
        let p = graph.entity(*id, owner)?;
        let area = engineering(&graph, p, 4, "CrossSectionArea", "IFCAREAMEASURE", [2, 0, 0, 0, 0, 0, 0], owner)?;
        let iy = engineering(&graph, p, 4, "MomentOfInertiaY", "IFCMOMENTOFINERTIAMEASURE", [4, 0, 0, 0, 0, 0, 0], owner)?;
        let iz = engineering(&graph, p, 4, "MomentOfInertiaZ", "IFCMOMENTOFINERTIAMEASURE", [4, 0, 0, 0, 0, 0, 0], owner)?;
        let j = engineering(&graph, p, 4, "TorsionalConstant", "IFCMOMENTOFINERTIAMEASURE", [4, 0, 0, 0, 0, 0, 0], owner)?;
        if [area, iy, iz, j].iter().any(|v| *v <= 0.) {
            return Err(invalid("analytical section measures must be positive"));
        }
        let name = graph.property(p, 4, "SectionName", owner)?;
        let Some(IfcValue::TypedValue(v)) = name.args.get(2) else {
            return Err(invalid("typed analytical section name is required"));
        };
        if v.name != "IFCLABEL" || v.items.len() != 1 || name.args.get(3) != Some(&IfcValue::Unset) {
            return Err(invalid("dimensionless analytical section label is required"));
        }
        let name = v.items[0].as_str().ok_or_else(|| invalid("analytical section name is not text"))?;
        work(owner, size_of::<FemSection>())?;
        *section = Some(FemSection { id: String::new(), name: String::new(), area, iy, iz, j });
        let s = section.as_mut().unwrap();
        s.id = copy(key(*id, &mut [0; 23]), owner)?;
        s.name = copy(name, owner)?;
        work(owner, size_of::<FemSection>())?;
        output.sections.push(section.take().unwrap());
    }
    Ok(())
}

pub(super) fn axis(start: [f64; 3], end: [f64; 3]) -> Result<[f64; 3], ValueError> {
    let mut x = std::array::from_fn::<_, 3, _>(|i| end[i] - start[i]);
    let length = x[0].hypot(x[1]).hypot(x[2]);
    if !length.is_finite() || length == 0. {
        return Err(invalid("analytical member length must be finite and nonzero"));
    }
    for n in &mut x {
        *n /= length;
    }
    let reference = if x[2].abs() > 0.99 { [1., 0., 0.] } else { [0., 0., 1.] };
    let mut y = [reference[1] * x[2] - reference[2] * x[1], reference[2] * x[0] - reference[0] * x[2], reference[0] * x[1] - reference[1] * x[0]];
    let length = y[0].hypot(y[1]).hypot(y[2]);
    for n in &mut y {
        *n /= length;
    }
    Ok([x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]])
}
fn endpoint(vertices: &[(u64, u64, [f64; 3])], id: u64, owner: &mut Owner<'_, '_>) -> Result<(u64, [f64; 3]), ValueError> {
    let (mut lo, mut hi) = (0, vertices.len());
    while lo < hi {
        work(owner, size_of::<(u64, u64, [f64; 3])>())?;
        let m = lo + (hi - lo) / 2;
        match vertices[m].0.cmp(&id) {
            std::cmp::Ordering::Less => lo = m + 1,
            std::cmp::Ordering::Greater => hi = m,
            std::cmp::Ordering::Equal => return Ok((vertices[m].1, vertices[m].2)),
        }
    }
    Err(invalid("analytical edge references an absent point connection"))
}
fn members(source: &IfcSnapshot, frame: &mut Projection, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
    let Projection { index, relations, vertices, value, element, .. } = frame;
    let graph = Graph { source, index, relations };
    let output = value.as_mut().ok_or_else(|| invalid("analytical destination is absent"))?;
    for entity in &source.entities {
        work(owner, 8)?;
        if entity.name != "IFCSTRUCTURALCURVEMEMBER" {
            continue;
        }
        let edge = graph.item(entity, owner)?;
        if edge.name != "IFCEDGE" {
            return Err(unsupported("analytical member requires a reference edge"));
        }
        let (start, start_xyz) = endpoint(vertices, reference(edge.args.first())?, owner)?;
        let (end, end_xyz) = endpoint(vertices, reference(edge.args.get(1))?, owner)?;
        let expected = axis(start_xyz, end_xyz)?;
        let direction = graph.xyz(entity.args.get(8), "IFCDIRECTION", owner)?;
        let scale = direction.iter().fold(0.0f64, |m, v| m.max(v.abs()));
        if scale == 0. {
            return Err(invalid("analytical member axis cannot be zero"));
        }
        let norm = (direction[0] / scale).hypot(direction[1] / scale).hypot(direction[2] / scale);
        if (0..3).any(|i| (direction[i] / scale / norm - expected[i]).abs() > 1e-10) {
            return Err(unsupported("native analytical member requires its reference zero roll axis"));
        }
        let mut endpoints = [false; 2];
        let mut count = 0;
        for relation in graph.rows(3, entity.id) {
            work(owner, 32)?;
            let row = &source.entities[relation.index];
            if (6..=9).any(|i| row.args.get(i) != Some(&IfcValue::Unset)) {
                return Err(unsupported("analytical member conditions and releases require additional semantics"));
            }
            let id = reference(row.args.get(5))?;
            let slot = if id == start {
                0
            } else if id == end {
                1
            } else {
                return Err(invalid("analytical member connection disagrees with its edge"));
            };
            if endpoints[slot] {
                return Err(invalid("analytical endpoint connection repeats"));
            }
            endpoints[slot] = true;
            count += 1;
        }
        if count != 2 || !endpoints.into_iter().all(|v| v) {
            return Err(invalid("analytical edge requires exactly its two endpoint connections"));
        }
        let material = material_id(&graph, entity.id, owner)?;
        let section = definition(&graph, 1, entity.id, "semio_FemSection", owner)?.id;
        work(owner, size_of::<FemElement>())?;
        *element = Some(match entity.args.get(7).and_then(IfcValue::as_enum) {
            Some("PIN_JOINED_MEMBER") => FemElement::Bar { id: String::new(), start: String::new(), end: String::new(), material_id: String::new(), section_id: String::new() },
            Some("RIGID_JOINED_MEMBER") => FemElement::Frame { id: String::new(), start: String::new(), end: String::new(), material_id: String::new(), section_id: String::new(), roll: 0. },
            _ => return Err(unsupported("analytical member requires explicit pin or rigid joining")),
        });
        let (id, s, t, m, p) = match element.as_mut().unwrap() {
            FemElement::Bar { id, start, end, material_id, section_id } | FemElement::Frame { id, start, end, material_id, section_id, .. } => (id, start, end, material_id, section_id),
        };
        *id = copy(key(entity.id, &mut [0; 23]), owner)?;
        *s = copy(key(start, &mut [0; 23]), owner)?;
        *t = copy(key(end, &mut [0; 23]), owner)?;
        *m = copy(key(material, &mut [0; 23]), owner)?;
        *p = copy(key(section, &mut [0; 23]), owner)?;
        work(owner, size_of::<FemElement>())?;
        output.elements.push(element.take().unwrap());
    }
    Ok(())
}

/// 🫴️ Projects admitted analytical IFC data directly into the FEM owner's persistent receiving boundary.
pub(super) fn receive(source: &IfcSnapshot, owner: &mut Owner<'_, '_>) -> Result<Fem3dSnapshot, ValueError> {
    owner.scoped_stage(|owner| owner.receive_nested::<Projection, Fem3dSnapshot>(|slot, original| {
        original.native().begin_stage(0)?;
        work(original, size_of::<Projection>())?;
        *slot = Some(Projection::default());
        let frame = slot.as_mut().unwrap();
        let (n, m, analysis) = build_index(source, frame, original)?;
        graph_values(source, frame, original)?;
        admit(source, frame, analysis, n + m, original)?;
        work(original,size_of::<load_import::State>())?;
        frame.load_state=Some(load_import::State::default());
        load_import::prepare(&Graph{source,index:&frame.index,relations:&frame.relations},analysis,frame.load_state.as_mut().unwrap(),original)?;
        let supports = keys(source, frame, m, original)?;
        if n + m + supports + frame.material_keys.len() + frame.section_keys.len() > 64000 {
            return Err(unsupported("analytical destination exceeds retained row admission"));
        }
        frame.vertices = vector(n, original)?;
        work(original, size_of::<Fem3dSnapshotValue>())?;
        frame.value = Some(Fem3dSnapshotValue::default());
        let value = frame.value.as_mut().unwrap();
        value.analysis.modal_count = 0;
        value.analysis.buckling_count = 0;
        value.analysis.deformation_scale = 1.;
        value.nodes = vector(n, original)?;
        value.elements = vector(m, original)?;
        value.materials = vector(frame.material_keys.len(), original)?;
        value.sections = vector(frame.section_keys.len(), original)?;
        value.supports = vector(supports, original)?;
        nodes(source, frame, original)?;
        definitions(source, frame, original)?;
        members(source, frame, original)?;
        let base=n+m+supports+frame.material_keys.len()+frame.section_keys.len();
        load_import::receive(&Graph{source,index:&frame.index,relations:&frame.relations},frame.unit_assignment,frame.load_state.as_mut().unwrap(),frame.value.as_mut().unwrap(),base,original)?;
        Fem3dSnapshot::receive_authoring(&mut frame.value, original)
    }))
}
#[cfg(test)]
#[path = "fem/nodal/native-units-tests.rs"]
mod nodal_unit_tests;
