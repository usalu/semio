//! 🫴️ IFC static-geometry receiving admits the real native schema without advertising lossless foreign reconstruction.
use super::*;
use crate::standards::v2_0::subsets::any::schema::snapshot_root::GltfSnapshot;
use semio_framework_dsl_record::{
    BorrowedDslField, BorrowedDslRecord, DslField, FieldValue, RecordValue,
    receiving::owned::{DslOwnedReceivingField, OwnedScalar, ScalarReceiving},
};
use semio_framework_value::{RetireOwned, ValueError, ValueRefusalKind};
use std::mem::size_of;
use store::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner, OriginalPackWriting, OriginalSnapshotRecordReceiving};
#[path = "geometry-project.rs"]
mod project;
#[path = "geometry-read.rs"]
mod read;
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
#[derive(RetireOwned)]
struct Document(RecordValue);
impl BorrowedDslField for Document {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = <GltfDocument as BorrowedDslField>::SHAPE;
}
impl DslField for Document {
    fn shape() -> semio_framework_dsl_record::Shape {
        GltfDocument::shape()
    }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        GltfDocument::shape_controlled(control)
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Record(self.0.clone())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Record(r) => Ok(Self(r.clone())),
            _ => Err("glTF native document must be a declared record".into()),
        }
    }
    fn from_value_controlled(_: &FieldValue, _: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "glTF geometry source requires original owned receiving"))
    }
}
impl OwnedScalar for Document {
    fn take_source(source: &mut Option<FieldValue>) -> Result<Self, ValueError> {
        if !matches!(source, Some(FieldValue::Record(_))) {
            return Err(invalid("glTF native document is not a declared record"));
        }
        let Some(FieldValue::Record(r)) = source.take() else { unreachable!() };
        Ok(Self(r))
    }
}
impl DslOwnedReceivingField for Document {
    type Cursor = ScalarReceiving<Self>;
}
impl semio_framework_dsl_record::DslReceivingField for Document {
    fn receive_value(_: &FieldValue, _: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "glTF geometry source requires original owned receiving"))
    }
}
#[derive(RetireOwned, semio_framework_dsl_record_derive::DslScalar)]
enum Form {
    Json,
    Glb,
}
impl OwnedScalar for Form {
    fn take_source(source: &mut Option<FieldValue>) -> Result<Self, ValueError> {
        let out = match source {
            Some(FieldValue::Enum(0)) => Self::Json,
            Some(FieldValue::Enum(1)) => Self::Glb,
            _ => return Err(invalid("glTF native source form is invalid")),
        };
        source.take();
        Ok(out)
    }
}
impl DslOwnedReceivingField for Form {
    type Cursor = ScalarReceiving<Self>;
}
impl semio_framework_dsl_record::DslReceivingField for Form {
    fn receive_value(value: &FieldValue, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        semio_framework_dsl_record::receiving::work(owner, size_of::<Self>(), 1)?;
        match value {
            FieldValue::Enum(0) => Ok(Self::Json),
            FieldValue::Enum(1) => Ok(Self::Glb),
            _ => Err(invalid("glTF native source form is invalid")),
        }
    }
}
#[derive(RetireOwned, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving)]
struct Buffer {
    bytes: Vec<u8>,
}
#[derive(RetireOwned, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving)]
struct Source {
    schema: String,
    document: Document,
    buffers: Vec<Buffer>,
    source_form: Form,
}
#[derive(Default, RetireOwned)]
struct Receiving {
    source: Option<Source>,
    target: Option<GltfSnapshot>,
}
/// 📥️ Receives selected static geometry under the original Pack schema and caller custody; non-geometric source fields retire there.
pub fn receive_geometry_pack(source: &[u8], owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<GltfSnapshot, ValueError> {
    owner.receive_nested::<Receiving, GltfSnapshot>(|slot, original| {
        original.native().checkpoint()?;
        let g = original.remaining_grant();
        if g.maximum_items == 0 || g.maximum_depth == 0 || g.maximum_copy_bytes < size_of::<Receiving>() {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "glTF geometry receiving header exceeds original grant"));
        }
        *slot = Some(Receiving::default());
        original.record_progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Receiving>(), ..Default::default() })?;
        let wire = original.receive_nested::<OriginalSnapshotRecordReceiving<Source>, Source>(|receiver, original| {
            let grant=original.remaining_grant();
            let mut limits=store::mounted_pack_rt::PackLimits::default();limits.max_file_len=64*1024*1024;
            let Some((rx, p)) = OriginalSnapshotRecordReceiving::admit(source, "stdio.gltf", limits, original.native(), grant)? else {
                return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "glTF native Pack receiving birth exceeds original grant"));
            };
            *receiver = Some(rx);
            original.record_progress(p)?;
            original.native().begin_stage(0)?;
            original.drive_cursor(receiver, |rx, native, grant| {
                let mut out = None;
                let ready=rx.is_ready();
                let result = if ready { rx.take_into(&mut out, grant) } else { rx.step(source, native, grant.maximum_copy_bytes.min(4096), grant) };
                match result {
                    Ok(p) => {
                        let result = if !ready&&p.copied_items != 0 { native.step().map(|_| out) } else { Ok(out) };
                        (result, p)
                    }
                    Err(e) => {
                        let p = e.retained_progress();
                        (Err(e), p)
                    }
                }
            })
        })?;
        slot.as_mut().unwrap().source = Some(wire);
        read::receive(slot.as_mut().unwrap(), original)?;
        let g = original.remaining_grant();
        if g.maximum_items == 0 || g.maximum_depth == 0 || g.maximum_copy_bytes < size_of::<GltfSnapshot>() {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "glTF geometry target transfer exceeds original grant"));
        }
        original.native().checkpoint()?;
        original.record_progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<GltfSnapshot>(), ..Default::default() })?;
        Ok(slot.as_mut().unwrap().target.take().unwrap())
    })
}
/// 📦️ Publishes the actual geometry snapshot through its declared native Pack with one final output buffer.
pub fn encode_geometry_pack(source: &GltfSnapshot, owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<Vec<u8>, ValueError> {
    project::validate(source, owner)?;
    owner.receive_nested::<OriginalPackWriting, Vec<u8>>(|slot, original| {
        let g = original.remaining_grant();
        let n = size_of::<OriginalPackWriting>();
        if g.maximum_items == 0 || g.maximum_depth == 0 || g.maximum_copy_bytes < n {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "glTF geometry writer header exceeds original grant"));
        }
        original.native().checkpoint()?;
        *slot = Some(OriginalPackWriting::new("stdio.gltf", original.native().maximum_bytes())?);
        original.record_progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: n, ..Default::default() })?;
        original.native().begin_stage(0)?;
        original.drive_cursor(slot, |writer, native, grant| {
            let result = if writer.ready() { writer.take(grant).map(|r| r.map_or((None, Default::default()), |(out, p)| (Some(out), p))) } else { writer.advance(&project::Source(source), Source::RECORD, native, grant).map(|p| (None, p)) };
            match result {
                Ok((out, p)) => (Ok(out), p),
                Err(e) => {
                    let p = e.retained_progress();
                    (Err(e), p)
                }
            }
        })
    })
}
