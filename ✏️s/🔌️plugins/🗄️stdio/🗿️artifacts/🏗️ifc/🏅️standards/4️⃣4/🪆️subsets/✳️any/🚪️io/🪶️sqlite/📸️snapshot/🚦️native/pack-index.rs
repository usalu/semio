//! 🗺️ Original-funded flat locators project the canonical literal Pack while borrowing every authored payload.
use super::{IfcSnapshot, IfcTypedValue, IfcValue};
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource, FieldProjectionView as V, projection_path_error};
use semio_os_kernel::NativeSnapshotEncodeOwner as Owner;
use semio_framework_value::retirement::RetireOwned as RetireOwnedTrait;
use semio_framework_value::{RetireOwned, ValueError, ValueRefusalKind};
use std::mem::size_of;
use semio_framework_dsl_record::receiving::{NativeSnapshotFailure,ReceivingWork,receiving_turn};
use semio_framework_value::RetainedWorkOutcome;
const NONE: usize = usize::MAX;
#[derive(RetireOwned)]
struct Node {
    parent: usize,
    child: usize,
    group: usize,
    edges: usize,
    length: usize,
}
#[derive(RetireOwned)]
struct Group {
    entity: usize,
    complex: usize,
    start: usize,
    length: usize,
}
#[derive(RetireOwned)]
struct Entity {
    group: usize,
    complexes: usize,
}
/// 🧭️ Only ordinal metadata is allocated; source strings and binary64 payloads remain borrowed.
#[derive(Default, RetireOwned)]
pub(super) struct Index {
    allocation:semio_framework_io_control::NativeSnapshotEncodeAllocation,
    marker:semio_framework_io_control::NativeSnapshotEffectProgress,
    unwind_failure:Option<NativeSnapshotFailure>,
    nodes: Option<Vec<Node>>,
    edges: Option<Vec<usize>>,
    roots: Option<Vec<usize>>,
    groups: Option<Vec<Group>>,
    entities: Option<Vec<Entity>>,
}
fn limit() -> ValueError {
    ValueError::literal(ValueRefusalKind::WorkLimit, "literal IFC Pack projection exceeds its bounded source admission")
}
fn work(owner: &mut Owner<'_, '_>, bytes: usize, meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    let g = owner.remaining_grant();
    if g.maximum_items == 0 || g.maximum_copy_bytes < bytes || g.maximum_depth == 0 {
        return Err(limit().into());
    }
    owner.native().checkpoint()?;
    owner.record_progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
    meter.performed();owner.native().step().map_err(Into::into)
}
fn vector<T:RetireOwnedTrait>(count:usize,output:&mut Option<Vec<T>>,owner:&mut Owner<'_,'_>,meter:&mut ReceivingWork,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind_failure:&mut Option<NativeSnapshotFailure>,allocation:&mut semio_framework_io_control::NativeSnapshotEncodeAllocation)->Result<(),NativeSnapshotFailure>{
 let turn=owner.allocate_vector(count,output,marker,unwind_failure,allocation,1);meter.take(turn)?;marker.reset_completed()?;Ok(())
}
fn children(value: &IfcValue) -> &[IfcValue] {
    match value {
        IfcValue::Aggregate(v) | IfcValue::TypedValue(IfcTypedValue { items: v, .. }) => v,
        _ => &[],
    }
}
fn measure(value: &IfcValue, depth: usize, nodes: &mut usize, edges: &mut usize, owner: &mut Owner<'_, '_>, meter:&mut ReceivingWork) -> Result<(), NativeSnapshotFailure> {
    work(owner, size_of::<IfcValue>(),meter)?;
    if depth > 64 {
        return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "literal IFC Pack projection exceeds value depth admission").into());
    }
    if let IfcValue::Binary(text) = value {
        binary_head(text)?;
        for chunk in text.as_bytes()[1..].chunks(4096) {
            work(owner, chunk.len(),meter)?;
            if chunk.iter().any(|v| !v.is_ascii_digit() && !(b'A'..=b'F').contains(v)) {
                return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "literal IFC Pack binary payload is not canonical").into());
            }
        }
    }
    *nodes = nodes.checked_add(1).filter(|n| *n <= 2_000_000).ok_or_else(limit)?;
    let c = children(value);
    *edges = edges.checked_add(c.len()).ok_or_else(limit)?;
    for child in c {
        measure(child, depth + 1, nodes, edges, owner,meter)?;
    }
    Ok(())
}
fn add(value: &IfcValue, parent: usize, child: usize, group: usize, index: &mut Index, owner: &mut Owner<'_, '_>, meter:&mut ReceivingWork) -> Result<usize, NativeSnapshotFailure> {
    work(owner, size_of::<Node>(),meter)?;
    let at = index.nodes.as_ref().unwrap().len();
    let edge = index.edges.as_ref().unwrap().len();
    let c = children(value);
    index.nodes.as_mut().unwrap().push(Node { parent, child, group, edges: edge, length: c.len() });
    for _ in c {
        work(owner, size_of::<usize>(),meter)?;
        index.edges.as_mut().unwrap().push(0);
    }
    for (i, value) in c.iter().enumerate() {
        let next = add(value, at, i, group, index, owner,meter)?;
        work(owner, size_of::<usize>(),meter)?;
        index.edges.as_mut().unwrap()[edge + i] = next;
    }
    Ok(at)
}
fn group(values: &[IfcValue], entity: usize, complex: usize, index: &mut Index, owner: &mut Owner<'_, '_>, meter:&mut ReceivingWork) -> Result<usize, NativeSnapshotFailure> {
    work(owner, size_of::<Group>(),meter)?;
    let at = index.groups.as_ref().unwrap().len();
    index.groups.as_mut().unwrap().push(Group { entity, complex, start: index.roots.as_ref().unwrap().len(), length: values.len() });
    for (i, value) in values.iter().enumerate() {
        let root = add(value, NONE, i, at, index, owner,meter)?;
        work(owner, size_of::<usize>(),meter)?;
        index.roots.as_mut().unwrap().push(root);
    }
    Ok(at)
}
impl Index {
    pub(super) fn build(&mut self, source: &IfcSnapshot, owner: &mut Owner<'_, '_>) -> RetainedWorkOutcome<(), NativeSnapshotFailure> {receiving_turn(|meter|{
        if source.entities.len() > 1_000_000 {
            return Err(limit().into());
        }
        let (mut nodes, mut edges, mut roots, mut groups) = (0usize, 0usize, 0usize, 3usize);
        for values in [&source.header.file_description, &source.header.file_name, &source.header.file_schema] {
            roots = roots.checked_add(values.len()).ok_or_else(limit)?;
            for value in values {
                measure(value, 1, &mut nodes, &mut edges, owner,meter)?;
            }
        }
        for entity in &source.entities {
            work(owner, size_of::<super::IfcEntity>(),meter)?;
            groups = groups.checked_add(1).and_then(|n| n.checked_add(entity.complex.len())).filter(|n| *n <= 2_000_000).ok_or_else(limit)?;
            roots = roots.checked_add(entity.args.len()).ok_or_else(limit)?;
            for value in &entity.args {
                measure(value, 1, &mut nodes, &mut edges, owner,meter)?;
            }
            for complex in &entity.complex {
                work(owner, size_of::<super::IfcComplexType>(),meter)?;
                roots = roots.checked_add(complex.args.len()).ok_or_else(limit)?;
                for value in &complex.args {
                    measure(value, 1, &mut nodes, &mut edges, owner,meter)?;
                }
            }
        }
        vector(nodes, &mut self.nodes, owner,meter,&mut self.marker,&mut self.unwind_failure,&mut self.allocation)?;
        vector(edges, &mut self.edges, owner,meter,&mut self.marker,&mut self.unwind_failure,&mut self.allocation)?;
        vector(roots, &mut self.roots, owner,meter,&mut self.marker,&mut self.unwind_failure,&mut self.allocation)?;
        vector(groups, &mut self.groups, owner,meter,&mut self.marker,&mut self.unwind_failure,&mut self.allocation)?;
        vector(source.entities.len(), &mut self.entities, owner,meter,&mut self.marker,&mut self.unwind_failure,&mut self.allocation)?;
        for (i, values) in [&source.header.file_description, &source.header.file_name, &source.header.file_schema].into_iter().enumerate() {
            group(values, NONE, i, self, owner,meter)?;
        }
        for (i, entity) in source.entities.iter().enumerate() {
            let g = group(&entity.args, i, NONE, self, owner,meter)?;
            let complexes = self.groups.as_ref().unwrap().len();
            work(owner, size_of::<Entity>(),meter)?;
            self.entities.as_mut().unwrap().push(Entity { group: g, complexes });
            for (j, complex) in entity.complex.iter().enumerate() {
                group(&complex.args, i, j, self, owner,meter)?;
            }
        }
        Ok(())
    })}
    pub(super) fn source<'a>(&'a self, source: &'a IfcSnapshot) -> Projection<'a> {
        Projection { index: self, source }
    }
}
pub(super) struct Projection<'a> {
    index: &'a Index,
    source: &'a IfcSnapshot,
}
impl Projection<'_> {
    fn value(&self, index: usize) -> Result<&IfcValue, ValueError> {
        let mut path = [0usize; 64];
        let mut count = 0;
        let mut node = self.index.nodes.as_ref().ok_or_else(projection_path_error)?.get(index).ok_or_else(projection_path_error)?;
        while node.parent != NONE {
            if count == path.len() {
                return Err(projection_path_error());
            }
            path[count] = node.child;
            count += 1;
            node = self.index.nodes.as_ref().ok_or_else(projection_path_error)?.get(node.parent).ok_or_else(projection_path_error)?;
        }
        let group = self.index.groups.as_ref().ok_or_else(projection_path_error)?.get(node.group).ok_or_else(projection_path_error)?;
        let values = if group.entity == NONE {
            match group.complex {
                0 => &self.source.header.file_description,
                1 => &self.source.header.file_name,
                2 => &self.source.header.file_schema,
                _ => return Err(projection_path_error()),
            }
        } else {
            let e = self.source.entities.get(group.entity).ok_or_else(projection_path_error)?;
            if group.complex == NONE { &e.args } else { &e.complex.get(group.complex).ok_or_else(projection_path_error)?.args }
        };
        let mut value = values.get(node.child).ok_or_else(projection_path_error)?;
        for child in path[..count].iter().rev() {
            value = children(value).get(*child).ok_or_else(projection_path_error)?;
        }
        Ok(value)
    }
    fn roots(&self, group: usize, path: &[usize]) -> Result<V<'_>, ValueError> {
        let group = self.index.groups.as_ref().ok_or_else(projection_path_error)?.get(group).ok_or_else(projection_path_error)?;
        match path {
            [] => Ok(V::List(group.length)),
            [i] if *i < group.length => Ok(V::UInt(self.index.roots.as_ref().ok_or_else(projection_path_error)?[group.start + i] as u64)),
            _ => Err(projection_path_error()),
        }
    }
    fn node(&self, index: usize, path: &[usize]) -> Result<V<'_>, ValueError> {
        let row = self.index.nodes.as_ref().ok_or_else(projection_path_error)?.get(index).ok_or_else(projection_path_error)?;
        let value = self.value(index)?;
        let kind = match value {
            IfcValue::Unset => "unset",
            IfcValue::Derived => "derived",
            IfcValue::Integer(_) => "integer",
            IfcValue::Real(_) => "real",
            IfcValue::String(_) => "string",
            IfcValue::Binary(_) => "binary",
            IfcValue::Enum(_) => "enum",
            IfcValue::Reference(_) => "reference",
            IfcValue::Aggregate(_) => "aggregate",
            IfcValue::TypedValue(_) => "typed",
        };
        match path {
            [] => Ok(V::Record(&[0, 1, 2, 3, 4, 5, 6])),
            [0] => Ok(V::Text(kind)),
            [1] => Ok(if let IfcValue::Integer(v) = value { V::Int(*v) } else { V::Absent }),
            [2] => Ok(if let IfcValue::Real(v) = value { V::Float(*v) } else { V::Absent }),
            [3] => Ok(if let IfcValue::String(v) | IfcValue::Binary(v) | IfcValue::Enum(v) = value { V::Text(v) } else { V::Absent }),
            [4] => Ok(if let IfcValue::Reference(v) = value { V::UInt(*v) } else { V::Absent }),
            [5] => Ok(if let IfcValue::TypedValue(v) = value { V::Text(&v.name) } else { V::Absent }),
            [6] => Ok(V::List(row.length)),
            [6, i] if *i < row.length => Ok(V::UInt(self.index.edges.as_ref().ok_or_else(projection_path_error)?[row.edges + i] as u64)),
            _ => Err(projection_path_error()),
        }
    }
}
impl FieldProjectionSource for Projection<'_> {
    fn projection_identity(&self) -> usize {
        self.source as *const IfcSnapshot as usize
    }
    fn projection_view(&self, path: &[usize]) -> Result<V<'_>, ValueError> {
        let Some((&field, path)) = path.split_first() else {
            return Ok(V::Record(&[0, 1, 2, 3, 4, 5]));
        };
        match field {
            0 if path.is_empty() => Ok(V::Text(&self.source.schema)),
            1..=3 => self.roots(field - 1, path),
            4 => {
                let Some((&i, path)) = path.split_first() else {
                    return Ok(V::List(self.source.entities.len()));
                };
                let e = self.source.entities.get(i).ok_or_else(projection_path_error)?;
                let at = self.index.entities.as_ref().ok_or_else(projection_path_error)?.get(i).ok_or_else(projection_path_error)?;
                let Some((&field, path)) = path.split_first() else {
                    return Ok(V::Record(&[0, 1, 2, 3]));
                };
                match field {
                    0 if path.is_empty() => Ok(V::UInt(e.id)),
                    1 if path.is_empty() => Ok(V::Text(&e.name)),
                    2 => self.roots(at.group, path),
                    3 => {
                        let Some((&j, path)) = path.split_first() else {
                            return Ok(V::List(e.complex.len()));
                        };
                        let c = e.complex.get(j).ok_or_else(projection_path_error)?;
                        match path {
                            [] => Ok(V::Record(&[0, 1])),
                            [0] => Ok(V::Text(&c.name)),
                            [1, tail @ ..] => self.roots(at.complexes + j, tail),
                            _ => Err(projection_path_error()),
                        }
                    }
                    _ => Err(projection_path_error()),
                }
            }
            5 => {
                let Some((&i, path)) = path.split_first() else {
                    return Ok(V::List(self.index.nodes.as_ref().unwrap().len()));
                };
                self.node(i, path)
            }
            _ => Err(projection_path_error()),
        }
    }
}

fn binary_head(text: &str) -> Result<(), ValueError> {
    let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "literal IFC Pack binary head is not canonical");
    let bytes = text.as_bytes();
    let Some(&unused) = bytes.first() else { return Err(invalid()) };
    if !(b'0'..=b'3').contains(&unused) || (bytes.len() == 1 && unused != b'0') {
        return Err(invalid());
    }
    if let Some(&first) = bytes.get(1) {
        let digit = match first {
            b'0'..=b'9' => first - b'0',
            b'A'..=b'F' => first - b'A' + 10,
            _ => return Err(invalid()),
        };
        if digit >= (16u8 >> (unused - b'0')) {
            return Err(invalid());
        }
    }
    Ok(())
}
