//! bmp rep for stdio.bmp 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

/// 📦️ Owned BMP Record codecs are independent of native bitmap publication.
use crate::standards::v_v3::subsets::any::schema::snapshot::BmpSnapshot;
use crate::store;

impl pack::snapshot::ArtifactPack for BmpSnapshot{
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
 
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let body=store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"BMP logical Pack envelope mismatch")))}let snapshot=Self::__dsl_from_record(&store::pack_rt::decode_document(&body,&Self::__dsl_spec(),options)?.0).map_err(store::PackError::from)?;snapshot.validate().map_err(|error|store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error)))?;Ok(snapshot)}
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for BmpSnapshot {
fn sqlite_snapshot_codec()->Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec>{Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())}
}

#[path = "🧩️native/🦀️.rs"]
pub mod native;

//#region 🧩️ArtifactItems
pub mod artifact_items {
/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_BMP_DOCUMENT_SCHEMA: &str = "stdio.bmp";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
/// 🧬️ Artifact schema descriptor id.
pub const BMP_ARTIFACT_SCHEMA_ID: &str = "s.stdio.bmp";
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    Vec::new()
}
}
//#endregion 🧩️ArtifactItems6
