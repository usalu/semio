//! 🧳️ Pack, controlled-record and native-encode codecs of the persisted snapshot root, the only owner-side part of the binary snapshot codec that names the root type.
use crate::standards::v2_0::subsets::any::schema::snapshot::*;
use crate::standards::v2_0::subsets::any::schema::snapshot_root::GltfSnapshot;
use crate::standards::v2_0::subsets::any::io::binary::snapshot::owned_pack::encoding::{build, emit, list, put, spec_producer};
use crate::standards::v2_0::subsets::any::io::binary::snapshot::owned_pack::{ResolvedBuffer, Snapshot};
use semio_framework_dsl_record::{DslField, FieldValue, RecordValue};
use semio_framework_value::NativeEncodeControl;
use semio_framework_value::{ValueError, ValueRefusalKind};

pub fn record(value:&GltfSnapshot)->semio_framework_dsl_record::RecordValue{let buffers=value.buffers.iter().map(|bytes|ResolvedBuffer{bytes:bytes.clone()}).collect::<Vec<_>>();semio_framework_dsl_record::RecordValue{fields:[(0,value.schema.to_value()),(1,value.document.to_value()),(2,buffers.to_value()),(3,value.source_form.to_value())].into_iter().collect()}}
impl From<Snapshot> for GltfSnapshot{fn from(value:Snapshot)->Self{Self{schema:value.schema,document:value.document,buffers:value.buffers.into_iter().map(|buffer|buffer.bytes).collect(),source_form:value.source_form}}}

impl pack::snapshot::ArtifactPack for GltfSnapshot{
 
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let inner=store::pack_rt::encode_document(&Snapshot::__dsl_spec(),&record(self),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.gltf",store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;if !envelope.matches_identity("stdio.gltf",store::semio_format::Component::Pack,1){return Err(store::PackError::from(ValueError::new(ValueRefusalKind::InvalidValue, "GLTF owned pack identity differs")));}let(record,_)=store::pack_rt::decode_document(&inner,&Snapshot::__dsl_spec(),options)?;Snapshot::__dsl_from_record(&record).map(Into::into).map_err(store::os_store::serialization::text_error_to_pack_error)}
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Snapshot::__dsl_spec())}
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for GltfSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec()->Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec>{Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())}
}

/// 🛬️ Admit each declared native child and the resolved buffer collection before ownership transfer.
pub fn reconstruct_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<GltfSnapshot,ValueError>{
 let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Snapshot::__dsl_from_record_controlled(record,control)?,Snapshot::retire_decoded);let count=owner.as_mut().buffers.len();let mut owned=control.allocate_vec(count)?;control.begin_stage(count)?;for _ in&owner.as_mut().buffers{control.step()?;}control.checkpoint()?;let Snapshot{schema,document,buffers,source_form}=owner.take();for buffer in buffers{owned.push(buffer.bytes);}Ok(GltfSnapshot{schema,document,buffers:owned,source_form})
}

/// 📸️ Projects the four actual root fields and resolved buffers without a native mirror clone.
fn snapshot_value(value: &GltfSnapshot, control: &mut NativeEncodeControl<'_>) -> Result<RecordValue, ValueError> {
    let FieldValue::Record(record) = build(4, control, |record, control| {
        put(record, 0, emit(&value.schema, control)?, control)?;
        put(record, 1, emit(&value.document, control)?, control)?;
        put(record, 2, list(&value.buffers, control, |bytes, control| build(1, control, |record, control| put(record, 0, emit(bytes, control)?, control)))?, control)?;
        put(record, 3, emit(&value.source_form, control)?, control)
    })?
    else {
        unreachable!("authored GLTF record")
    };
    Ok(record)
}

/// 🛫️ Emits every literal logical field through the declared controlled native output terminal.
pub fn encode_native(value:&GltfSnapshot,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{store::encode_sqlite_snapshot_record_native(encoding,"stdio.gltf",spec_producer(),|native|snapshot_value(value,native),control,native_owner)}
