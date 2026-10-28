//! 🗺️ Original-funded flat locators project the canonical literal Pack while borrowing every authored payload.
use super::{IfcSnapshot, IfcTypedValue, IfcValue};
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource, FieldProjectionView as V, projection_path_error};
use semio_framework_os_kernel::NativeSnapshotEncodeOwner as Owner;
use semio_framework_value::{RetireOwned, ValueError, ValueRefusalKind};
use std::mem::size_of;
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
    nodes: Vec<Node>,
    edges: Vec<usize>,
    roots: Vec<usize>,
    groups: Vec<Group>,
    entities: Vec<Entity>,
}
fn limit() -> ValueError {
    ValueError::literal(ValueRefusalKind::WorkLimit, "literal IFC Pack projection exceeds its bounded source admission")
}
fn work(owner: &mut Owner<'_, '_>, bytes: usize) -> Result<(), ValueError> {
    let g = owner.remaining_grant();
    if g.maximum_items == 0 || g.maximum_copy_bytes < bytes || g.maximum_depth == 0 {
        return Err(limit());
    }
    owner.native().checkpoint()?;
    owner.record_progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
    owner.native().step()
}
fn vector<T: RetireOwned>(count: usize, owner: &mut Owner<'_, '_>) -> Result<Vec<T>, ValueError> {
    owner.receive::<Vec<T>, Vec<T>>(|slot, native, body| {
        *slot = Some(Vec::new());
        body.allocate_encode_vec_into(native, count, slot.as_mut().unwrap(), 1)?;
        Ok(slot.take().unwrap())
    })
}
fn children(value: &IfcValue) -> &[IfcValue] {
    match value {
        IfcValue::Aggregate(v) | IfcValue::TypedValue(IfcTypedValue { items: v, .. }) => v,
        _ => &[],
    }
}
fn measure(value: &IfcValue, depth: usize, nodes: &mut usize, edges: &mut usize, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
    work(owner, size_of::<IfcValue>())?;
    if depth > 64 {
        return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "literal IFC Pack projection exceeds value depth admission"));
    }
    if let IfcValue::Binary(text) = value {
        super::owned_graph::binary_head(text)?;
        for chunk in text.as_bytes()[1..].chunks(4096) {
            work(owner, chunk.len())?;
            if chunk.iter().any(|v| !v.is_ascii_digit() && !(b'A'..=b'F').contains(v)) {
                return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "literal IFC Pack binary payload is not canonical"));
            }
        }
    }
    *nodes = nodes.checked_add(1).filter(|n| *n <= 2_000_000).ok_or_else(limit)?;
    let c = children(value);
    *edges = edges.checked_add(c.len()).ok_or_else(limit)?;
    for child in c {
        measure(child, depth + 1, nodes, edges, owner)?;
    }
    Ok(())
}
fn add(value: &IfcValue, parent: usize, child: usize, group: usize, index: &mut Index, owner: &mut Owner<'_, '_>) -> Result<usize, ValueError> {
    work(owner, size_of::<Node>())?;
    let at = index.nodes.len();
    let edge = index.edges.len();
    let c = children(value);
    index.nodes.push(Node { parent, child, group, edges: edge, length: c.len() });
    for _ in c {
        work(owner, size_of::<usize>())?;
        index.edges.push(0);
    }
    for (i, value) in c.iter().enumerate() {
        let next = add(value, at, i, group, index, owner)?;
        work(owner, size_of::<usize>())?;
        index.edges[edge + i] = next;
    }
    Ok(at)
}
fn group(values: &[IfcValue], entity: usize, complex: usize, index: &mut Index, owner: &mut Owner<'_, '_>) -> Result<usize, ValueError> {
    work(owner, size_of::<Group>())?;
    let at = index.groups.len();
    index.groups.push(Group { entity, complex, start: index.roots.len(), length: values.len() });
    for (i, value) in values.iter().enumerate() {
        let root = add(value, NONE, i, at, index, owner)?;
        work(owner, size_of::<usize>())?;
        index.roots.push(root);
    }
    Ok(at)
}
impl Index {
    pub(super) fn build(&mut self, source: &IfcSnapshot, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
        if source.entities.len() > 1_000_000 {
            return Err(limit());
        }
        let (mut nodes, mut edges, mut roots, mut groups) = (0usize, 0usize, 0usize, 3usize);
        for values in [&source.header.file_description, &source.header.file_name, &source.header.file_schema] {
            roots = roots.checked_add(values.len()).ok_or_else(limit)?;
            for value in values {
                measure(value, 1, &mut nodes, &mut edges, owner)?;
            }
        }
        for entity in &source.entities {
            work(owner, size_of::<super::IfcEntity>())?;
            groups = groups.checked_add(1).and_then(|n| n.checked_add(entity.complex.len())).filter(|n| *n <= 2_000_000).ok_or_else(limit)?;
            roots = roots.checked_add(entity.args.len()).ok_or_else(limit)?;
            for value in &entity.args {
                measure(value, 1, &mut nodes, &mut edges, owner)?;
            }
            for complex in &entity.complex {
                work(owner, size_of::<super::IfcComplexType>())?;
                roots = roots.checked_add(complex.args.len()).ok_or_else(limit)?;
                for value in &complex.args {
                    measure(value, 1, &mut nodes, &mut edges, owner)?;
                }
            }
        }
        self.nodes = vector(nodes, owner)?;
        self.edges = vector(edges, owner)?;
        self.roots = vector(roots, owner)?;
        self.groups = vector(groups, owner)?;
        self.entities = vector(source.entities.len(), owner)?;
        for (i, values) in [&source.header.file_description, &source.header.file_name, &source.header.file_schema].into_iter().enumerate() {
            group(values, NONE, i, self, owner)?;
        }
        for (i, entity) in source.entities.iter().enumerate() {
            let g = group(&entity.args, i, NONE, self, owner)?;
            let complexes = self.groups.len();
            work(owner, size_of::<Entity>())?;
            self.entities.push(Entity { group: g, complexes });
            for (j, complex) in entity.complex.iter().enumerate() {
                group(&complex.args, i, j, self, owner)?;
            }
        }
        Ok(())
    }
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
        let mut node = self.index.nodes.get(index).ok_or_else(projection_path_error)?;
        while node.parent != NONE {
            if count == path.len() {
                return Err(projection_path_error());
            }
            path[count] = node.child;
            count += 1;
            node = self.index.nodes.get(node.parent).ok_or_else(projection_path_error)?;
        }
        let group = self.index.groups.get(node.group).ok_or_else(projection_path_error)?;
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
        let group = self.index.groups.get(group).ok_or_else(projection_path_error)?;
        match path {
            [] => Ok(V::List(group.length)),
            [i] if *i < group.length => Ok(V::UInt(self.index.roots[group.start + i] as u64)),
            _ => Err(projection_path_error()),
        }
    }
    fn node(&self, index: usize, path: &[usize]) -> Result<V<'_>, ValueError> {
        let row = self.index.nodes.get(index).ok_or_else(projection_path_error)?;
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
            [6, i] if *i < row.length => Ok(V::UInt(self.index.edges[row.edges + i] as u64)),
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
                let at = self.index.entities.get(i).ok_or_else(projection_path_error)?;
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
                    return Ok(V::List(self.index.nodes.len()));
                };
                self.node(i, path)
            }
            _ => Err(projection_path_error()),
        }
    }
}
