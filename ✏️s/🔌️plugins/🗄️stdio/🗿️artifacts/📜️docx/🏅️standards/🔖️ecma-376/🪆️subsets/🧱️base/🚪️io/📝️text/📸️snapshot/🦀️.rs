//! 📝️ Text representation codec surface for `stdio.docx` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type DocxSnapshotText = String;
//#endregion 🚚️Carrier

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

impl store::ArtifactDsl for DocxSnapshot {
 const EXTENSION:&'static str="docx";
 fn envelope_id()->&'static str{"stdio.docx"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{native::decode_text(text,&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default())).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))}
 fn print_dsl(&self)->String{match native::encode(self,semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Text,&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default())).expect("DOCX native ownership admission"){semio_framework_os_kernel::io_schema::IoPayload::Text(text)=>text,semio_framework_os_kernel::io_schema::IoPayload::Binary(_)=>unreachable!()}}
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

pub(crate) fn input(bytes:&[u8],binary:bool,control:&mut SqliteSnapshotControl<'_>)->Result<DocxSnapshot,ValueError>{
 backing::input(bytes,binary,control)
}

pub(super) fn decode_text(text:&str,control:&mut SqliteSnapshotControl<'_>)->Result<DocxSnapshot,ValueError>{input(text.as_bytes(),false,control)}
}
pub use snapshot_wire_codec::*;
