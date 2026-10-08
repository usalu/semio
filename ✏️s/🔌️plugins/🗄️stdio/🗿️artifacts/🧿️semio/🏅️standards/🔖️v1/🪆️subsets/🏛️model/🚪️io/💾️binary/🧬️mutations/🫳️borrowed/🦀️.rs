//! 🏛️ Exact original Model element tuple fields reborrowed by bounded operation traversal.
use crate::standards::v1::subsets::{model::schema::{snapshot::*, mutations::SemioModelMutation}, base::schema::geometry::{SemioTransform, SemioPoint3, SemioQuaternion, native::NativeF64}};
use semio_framework_plugin::plugin_app_close_prelude::store::{ArtifactOperationText, ArtifactOperationTextNode as Node};

enum View<'a> {
    Operation(&'a SemioModelMutation), Element(&'a SemioModelElement), Class(&'a ElementClass),
    Transform(&'a SemioTransform), Point(&'a SemioPoint3), Quaternion(&'a SemioQuaternion),
    Geometry(&'a GeometryRef), Option(&'a Option<String>), Sets(&'a [PropertySet]),
    Set(&'a PropertySet), Properties(&'a [Property]), Property(&'a Property), Value(&'a PsetValue),
    Bytes(&'a [u8]), Hex(&'a str), Float(f64),
}

impl<'a> View<'a> {
    fn child(self, index: usize) -> Option<Self> {
        use View as V;
        Some(match self {
            V::Operation(SemioModelMutation::InsertElement(payload)) => match (index, payload.at) { (0, _) => V::Bytes(b"element="), (1, _) => V::Element(&payload.element), (2, Some(_)) => V::Bytes(b" at="), (3, Some(at)) => V::Float(at as f64), _ => return None },
            V::Operation(SemioModelMutation::RemoveElement(payload)) => match index { 0 => V::Bytes(b"id="), 1 => V::Hex(&payload.id), _ => return None },
            V::Element(value) => match index { 0 => V::Hex(&value.id), 1 => V::Class(&value.class), 2 => V::Transform(&value.placement), 3 => V::Geometry(&value.geometry), 4 => V::Option(&value.spatial_id), 5 => V::Sets(&value.psets), _ => return None },
            V::Class(ElementClass::Other { name }) if index == 0 => V::Hex(name),
            V::Transform(value) => match index { 0 => V::Point(&value.translation), 1 => V::Quaternion(&value.rotation), 2 => V::Point(&value.scale), _ => return None },
            V::Point(value) => V::Float(match index { 0 => value.x, 1 => value.y, 2 => value.z, _ => return None }),
            V::Quaternion(value) => V::Float(match index { 0 => value.x, 1 => value.y, 2 => value.z, 3 => value.w, _ => return None }),
            V::Geometry(GeometryRef::Brep { brep_id }) if index == 0 => V::Hex(brep_id),
            V::Geometry(GeometryRef::Mesh { mesh_id }) if index == 0 => V::Hex(mesh_id),
            V::Option(Some(value)) if index == 0 => V::Hex(value),
            V::Sets(values) => V::Set(values.get(index)?),
            V::Set(value) => match index { 0 => V::Hex(&value.name), 1 => V::Properties(&value.properties), _ => return None },
            V::Properties(values) => V::Property(values.get(index)?),
            V::Property(value) => match index { 0 => V::Hex(&value.key), 1 => V::Value(&value.value), _ => return None },
            V::Value(value) if index == 0 => match value { PsetValue::Text { value } => V::Hex(value), PsetValue::Number { value } => V::Float(*value), PsetValue::Boolean { value } => V::Bytes(if *value { b"1" } else { b"0" }) },
            _ => return None,
        })
    }

    fn node(self) -> Result<Node<'a>, String> {
        use View as V;
        let sequence = |length: usize| Node::Sequence { length, open: b"[", separator: b",", close: b"]" };
        let tagged = |open: &'a [u8]| Node::Sequence { length: 1, open, separator: b"", close: b"]" };
        Ok(match self {
            V::Operation(SemioModelMutation::InsertElement(payload)) => Node::Sequence { length: if payload.at.is_some() { 4 } else { 2 }, open: b"", separator: b"", close: b"" },
            V::Operation(SemioModelMutation::RemoveElement(_)) => Node::Sequence { length: 2, open: b"", separator: b"", close: b"" },
            V::Operation(_) => return Err("model.operation-text.unsupported".into()),
            V::Element(_) => sequence(6), V::Transform(_) | V::Point(_) => sequence(3), V::Quaternion(_) => sequence(4),
            V::Class(value) => match value {
                ElementClass::Wall => Node::Bytes(b"WA"), ElementClass::Slab => Node::Bytes(b"SL"), ElementClass::Column => Node::Bytes(b"CO"), ElementClass::Beam => Node::Bytes(b"BE"), ElementClass::Door => Node::Bytes(b"DO"), ElementClass::Window => Node::Bytes(b"WI"), ElementClass::Roof => Node::Bytes(b"RO"), ElementClass::Stair => Node::Bytes(b"ST"), ElementClass::Furniture => Node::Bytes(b"FU"), ElementClass::Other { .. } => tagged(b"OT["),
            },
            V::Geometry(value) => match value { GeometryRef::None => Node::Bytes(b"N"), GeometryRef::Brep { .. } => tagged(b"B["), GeometryRef::Mesh { .. } => tagged(b"M[") },
            V::Option(value) => if value.is_some() { tagged(b"[1,") } else { Node::Bytes(b"[0]") },
            V::Sets(values) => sequence(values.len()), V::Properties(values) => sequence(values.len()), V::Set(_) | V::Property(_) => sequence(2),
            V::Value(value) => tagged(match value { PsetValue::Text { .. } => b"T[", PsetValue::Number { .. } => b"N[", PsetValue::Boolean { .. } => b"B[" }),
            V::Bytes(bytes) => Node::Bytes(bytes), V::Hex(value) => Node::Hex(value.as_bytes()), V::Float(_) => Node::Scalar,
        })
    }
}

fn view<'a>(operation: &'a SemioModelMutation, path: &[usize]) -> Result<View<'a>, String> {
    let mut value = View::Operation(operation);
    for index in path { value = value.child(*index).ok_or_else(|| "model.operation-text.invalid-path".to_string())?; }
    Ok(value)
}

struct ScalarWindow<'a> { output: &'a mut [u8], offset: usize, position: usize, written: usize }
impl std::fmt::Write for ScalarWindow<'_> {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        let end = self.position.checked_add(text.len()).ok_or(std::fmt::Error)?;
        let start = self.offset.max(self.position);
        let stop = (self.offset + self.output.len()).min(end);
        if stop > start { let count = stop - start; self.output[self.written..self.written + count].copy_from_slice(&text.as_bytes()[start - self.position..stop - self.position]); self.written += count; }
        self.position = end;
        Ok(())
    }
}

impl ArtifactOperationText for SemioModelMutation {
    fn operation_text_node(&self, path: &[usize]) -> Result<Node<'_>, String> { view(self, path)?.node() }
    fn operation_text_scalar(&self, path: &[usize], offset: usize, output: &mut [u8]) -> Result<(usize, bool), String> {
        use std::fmt::Write;
        let View::Float(value) = view(self, path)? else { return Err("model.operation-text.scalar-path".into()); };
        let mut window = ScalarWindow { output, offset, position: 0, written: 0 };
        write!(window, "{}", NativeF64(value)).map_err(|_| "model.operation-text.scalar-format".to_string())?;
        Ok((window.written, offset + window.written == window.position))
    }
}
