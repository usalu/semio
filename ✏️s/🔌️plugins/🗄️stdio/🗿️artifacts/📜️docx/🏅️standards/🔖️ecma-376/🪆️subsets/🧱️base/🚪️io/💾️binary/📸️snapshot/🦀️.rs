//! docx rep for stdio.docx 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::*;
use crate::{
    standards::v_ecma_376::subsets::base::io::{DocxError, REL_TYPE_STYLES, STRICT_REL_TYPE_OFFICE_DOCUMENT, STRICT_REL_TYPE_STYLES},
    STDIO_DOCX_DOCUMENT_SCHEMA,
};
use framework_schema::ArtifactSchema;
use semio_framework_value::list::PagedList;
use semio_s_artifact_stdio_xml::schema::snapshot::{retained::RetainedXmlDocument, sqlite::{retire_xml_document, retire_xml_document_with_frontier}, xml_document_to_text, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::{resolve_relationship_target, retained::RetainedOpcPackage, OpcPackage, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};
use std::collections::HashSet;

impl store::ArtifactPack for DocxSnapshot {
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{match native::encode(self,semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Binary,&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,native::pack_limits(&options.limits))).map_err(store::PackError::from)?{semio_framework_os_kernel::io_schema::IoPayload::Binary(bytes)=>Ok(bytes),semio_framework_os_kernel::io_schema::IoPayload::Text(_)=>unreachable!()}}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{native::decode_binary(bytes,&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,native::pack_limits(&options.limits))).map_err(store::PackError::from)}
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::component::native::*;
use semio_framework_value::{NativeEncodeControl,NativeDecodeControl,ValueError,ValueRefusalKind};
use semio_framework_value::DecodedValue;
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::component::{docx_xml_parts_from_iter_controlled,DocxSnapshot,DocxXmlPart};
use semio_framework_os_kernel::{sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,SqliteDatabaseLimits},io_schema::IoPayload};
use semio_s_artifact_stdio_zip::opc::native::{OpcNativeWriter,OpcNativeReader};
use semio_s_artifact_stdio_zip::opc::retained::RetainedOpcPackage;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::{XmlDocumentView,retire_xml_document};
use crate::standards::v_ecma_376::subsets::base::io::text::snapshot::{input};
pub(super) fn encode(snapshot:&DocxSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<IoPayload,ValueError>{
 backing::encode(snapshot,encoding,control)
}

pub(super) fn decode_binary(bytes:&[u8],control:&mut SqliteSnapshotControl<'_>)->Result<DocxSnapshot,ValueError>{input(bytes,true,control)}

pub(super) fn pack_limits(limits:&store::mounted_pack_rt::PackLimits)->SqliteDatabaseLimits{SqliteDatabaseLimits{max_file_bytes:usize::try_from(limits.max_file_len).unwrap_or(usize::MAX),max_value_bytes:usize::try_from(limits.max_total_alloc).unwrap_or(usize::MAX),max_rows:usize::try_from(limits.max_items).unwrap_or(usize::MAX),..SqliteDatabaseLimits::default()}}
}
pub use snapshot_wire_codec::*;
