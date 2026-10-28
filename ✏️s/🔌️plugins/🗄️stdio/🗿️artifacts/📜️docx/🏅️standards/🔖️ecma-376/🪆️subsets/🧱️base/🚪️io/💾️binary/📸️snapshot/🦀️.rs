//! 🧾️ docx rep for stdio.docx 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::native;
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::*;
use crate::{standards::v_ecma_376::subsets::base::{schema::{refusal::{DocxError},vocabulary::{REL_TYPE_STYLES,STRICT_REL_TYPE_OFFICE_DOCUMENT,STRICT_REL_TYPE_STYLES}}}, standards::v_ecma_376::subsets::base::schema::snapshot::STDIO_DOCX_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value_persistent::list::PagedList;
use semio_s_artifact_stdio_xml::schema::snapshot::{retained::RetainedXmlDocument, ownership::{retire_xml_document, retire_xml_document_with_frontier}, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::{resolve_relationship_target, retained::RetainedOpcPackage, OpcPackage, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};
use std::collections::HashSet;

impl pack::snapshot::ArtifactPack for DocxSnapshot {
 
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{match native::encode_standalone(self,semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Binary,&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,native::pack_limits(&options.limits))).map_err(store::PackError::from)?{semio_framework_os_kernel::io_schema::IoPayload::Binary(bytes)=>Ok(bytes),semio_framework_os_kernel::io_schema::IoPayload::Text(_)=>unreachable!()}}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{native::decode_binary(bytes,&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,native::pack_limits(&options.limits))).map_err(store::PackError::from)}
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for DocxSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec()->Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec>{Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())}
}
}


#[path="🧩️native/🦀️.rs"]
pub(crate) mod native;
