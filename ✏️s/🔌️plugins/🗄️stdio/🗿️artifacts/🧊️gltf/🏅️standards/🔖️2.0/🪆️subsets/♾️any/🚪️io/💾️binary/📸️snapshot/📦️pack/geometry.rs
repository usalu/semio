//! 🫴️ glTF static-geometry receiving admits the real native schema without advertising lossless foreign reconstruction.
use super::*;
use crate::standards::v2_0::subsets::any::schema::snapshot_root::GltfSnapshot;
use semio_framework_dsl_record::{
    BorrowedDslField, BorrowedDslRecord, DslField, FieldValue, RecordValue,
    receiving::owned::{DslOwnedReceivingField, OwnedScalar, ScalarReceiving},
};
use semio_framework_value::{RetainedCloneGrant, RetainedWorkOutcome, RetireOwned, RetirementDemand, ValueError, ValueRefusalKind, retained_clone::retained_clone_grant_admits};
use std::mem::size_of;
use store::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner, NativeSnapshotFailure, OriginalPackWriting, OriginalSnapshotRecordReceiving};
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
impl semio_framework_dsl_record::receiving::owned::scalar_sealed::Sealed for Document {}
impl OwnedScalar for Document {
    fn creation_bytes() -> usize {
        0
    }
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
    fn receiving_cursor() -> Self::Cursor {
        Default::default()
    }
}
impl semio_framework_dsl_record::DslReceivingField for Document {
    fn receive_value(_: &FieldValue, _: &mut Option<Self>, _: &mut NativeSnapshotDecodeOwner<'_, '_>) -> RetainedWorkOutcome<(), NativeSnapshotFailure> {
        store::native_snapshot_turn(|_| Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "glTF geometry source requires original owned receiving").into()))
    }
}
#[derive(RetireOwned, semio_framework_dsl_record_derive::DslScalar)]
enum Form {
    Json,
    Glb,
}
impl semio_framework_dsl_record::receiving::owned::scalar_sealed::Sealed for Form {}
impl OwnedScalar for Form {
    fn creation_bytes() -> usize {
        size_of::<Self>()
    }
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
    fn receiving_cursor() -> Self::Cursor {
        Default::default()
    }
}
impl semio_framework_dsl_record::DslReceivingField for Form {
    fn receive_value(value: &FieldValue, output: &mut Option<Self>, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> RetainedWorkOutcome<(), NativeSnapshotFailure> {
        store::native_snapshot_turn(|work| {
            work.performed();
            semio_framework_dsl_record::receiving::work(owner, size_of::<Self>(), 1)?;
            *output = Some(match value {
                FieldValue::Enum(0) => Self::Json,
                FieldValue::Enum(1) => Self::Glb,
                _ => return Err(invalid("glTF native source form is invalid").into()),
            });
            Ok(())
        })
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
const STEP_WORK: u64 = 1 << 16;
fn failure(failure: NativeSnapshotFailure) -> ValueError {
    ValueError::new(failure.kind(), failure.message().to_string())
}
/// 📥️ Receives selected static geometry under the original Pack schema and caller custody; non-geometric source fields retire there.
pub fn receive_geometry_pack(source: &[u8], owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<GltfSnapshot, ValueError> {
    let mut output = None;
    owner.receive_nested::<Receiving, GltfSnapshot, NativeSnapshotFailure>(&mut output, &mut |slot, published, _marker, _unwind, original| store::native_snapshot_turn(|work| {
        work.performed();
        original.native().checkpoint()?;
        let g = original.remaining_grant();
        if g.maximum_items == 0 || g.maximum_depth == 0 || g.maximum_copy_bytes < size_of::<Receiving>() {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "glTF geometry receiving header exceeds original grant").into());
        }
        *slot = Some(Receiving::default());
        original.record_progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Receiving>(), ..Default::default() })?;
        let mut wire = None;
        work.take(original.receive_nested::<OriginalSnapshotRecordReceiving<Source>, Source, NativeSnapshotFailure>(&mut wire, &mut |receiver, wire, _marker, _unwind, original| store::native_snapshot_turn(|work| {
            let grant=original.remaining_grant();
            let mut limits=store::mounted_pack_rt::PackLimits::default();limits.max_file_len=64*1024*1024;
            let Some((rx, p)) = OriginalSnapshotRecordReceiving::admit(source, "stdio.gltf", limits, original.native(), grant)? else {
                return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "glTF native Pack receiving birth exceeds original grant").into());
            };
            *receiver = Some(rx);
            work.performed();
            original.record_progress(p)?;
            original.native().begin_stage(0)?;
            work.take(original.drive_cursor::<OriginalSnapshotRecordReceiving<Source>, Source, NativeSnapshotFailure>(receiver, wire, &mut |rx, out, _marker, _unwind, native, body| {
                let grant = body.remaining_grant();
                if rx.is_ready() {
                    let result = rx.take_into(out, grant).and_then(|p| body.record_progress(p)).map_err(NativeSnapshotFailure::from);
                    return RetainedWorkOutcome { work: 0, result };
                }
                let limit = grant.maximum_copy_bytes.min(4096);
                match native.with_effect(grant, STEP_WORK, |native, effect| rx.step(source, native, limit, effect)) {
                    Ok(Some(turn)) => {
                        let result = match turn.result {
                            Ok(p) => match if p.copied_items != 0 { native.step().map(|_| ()) } else { Ok(()) } {
                                Ok(()) => body.record_progress(p).map_err(NativeSnapshotFailure::from),
                                Err(e) => Err(NativeSnapshotFailure::from(e)),
                            },
                            Err(e) => Err(NativeSnapshotFailure::from(e)),
                        };
                        RetainedWorkOutcome { work: turn.work, result }
                    }
                    Ok(None) => RetainedWorkOutcome { work: 0, result: Ok(()) },
                    Err(error) => RetainedWorkOutcome { work: 0, result: Err(error.into()) },
                }
            }))?;
            Ok(())
        })))?;
        slot.as_mut().unwrap().source = wire;
        work.take(read::receive(slot.as_mut().unwrap(), original))?;
        let g = original.remaining_grant();
        if g.maximum_items == 0 || g.maximum_depth == 0 || g.maximum_copy_bytes < size_of::<GltfSnapshot>() {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "glTF geometry target transfer exceeds original grant").into());
        }
        original.native().checkpoint()?;
        original.record_progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<GltfSnapshot>(), ..Default::default() })?;
        *published = slot.as_mut().unwrap().target.take();
        Ok(())
    })).result.map_err(failure)?;
    output.ok_or_else(|| invalid("glTF geometry receiving published no snapshot"))
}
/// 📦️ Publishes the actual geometry snapshot through its declared native Pack with one final output buffer.
pub fn encode_geometry_pack(source: &GltfSnapshot, owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<Vec<u8>, ValueError> {
    project::validate(source, owner)?;
    let mut output = None;
    owner.receive_nested::<OriginalPackWriting, Vec<u8>, NativeSnapshotFailure>(&mut output, &mut |slot, published, _marker, _unwind, original| store::native_snapshot_turn(|work| {
        work.performed();
        let g = original.remaining_grant();
        let n = size_of::<OriginalPackWriting>();
        if g.maximum_items == 0 || g.maximum_depth == 0 || g.maximum_copy_bytes < n {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "glTF geometry writer header exceeds original grant").into());
        }
        original.native().checkpoint()?;
        *slot = Some(OriginalPackWriting::new("stdio.gltf", original.native().maximum_bytes())?);
        original.record_progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: n, ..Default::default() })?;
        original.native().begin_stage(0)?;
        work.take(original.drive_cursor::<OriginalPackWriting, Vec<u8>, NativeSnapshotFailure>(slot, published, &mut |writer, produced, _marker, _unwind, native, body| {
            let wallet = body.remaining_grant();
            let demand = if writer.ready() { RetirementDemand { depth: 1, ..Default::default() } } else { match writer.demand(wallet.maximum_copy_bytes) { Ok(demand) => demand, Err(error) => return RetainedWorkOutcome { work: 0, result: Err(error.into()) } } };
            if !retained_clone_grant_admits(wallet, demand) {
                return RetainedWorkOutcome { work: 0, result: Ok(()) };
            }
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            match native.with_effect(grant, STEP_WORK, |native, effect| {
                let result = if writer.ready() { writer.take(effect).map(|r| r.map_or(Default::default(), |(wire, p)| { *produced = Some(wire); p })) } else { writer.advance(&project::Source(source), Source::RECORD, native, effect) };
                (result, effect.performed_work())
            }) {
                Ok(Some((result, work))) => RetainedWorkOutcome { work, result: result.and_then(|p| body.record_progress(p)).map_err(NativeSnapshotFailure::from) },
                Ok(None) => RetainedWorkOutcome { work: 0, result: Ok(()) },
                Err(error) => RetainedWorkOutcome { work: 0, result: Err(error.into()) },
            }
        }))?;
        Ok(())
    })).result.map_err(failure)?;
    output.ok_or_else(|| invalid("glTF geometry writer published no wire"))
}
