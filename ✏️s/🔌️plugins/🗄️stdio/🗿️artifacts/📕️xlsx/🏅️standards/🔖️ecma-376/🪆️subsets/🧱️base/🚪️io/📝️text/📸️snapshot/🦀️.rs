//! 📝️ Text representation codec surface for `stdio.xlsx` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type XlsxSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::*;
use crate::STDIO_XLSX_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_to_text, XmlDocument};
use semio_s_artifact_stdio_zip::opc::{resolve_relationship_target, OpcPackage, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};
use std::collections::HashSet;

impl store::ArtifactDsl for XlsxSnapshot {
    const EXTENSION: &'static str = "xlsx";
    fn envelope_id() -> &'static str {
        "stdio.xlsx"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        native::decode_text(text, &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_| true, semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default()))
            .map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        match native::encode(
            self,
            semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Text,
            &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_| true, semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default()),
        )
        .expect("XLSX native ownership admission")
        {
            semio_framework_os_kernel::io_schema::IoPayload::Text(text) => text,
            semio_framework_os_kernel::io_schema::IoPayload::Binary(_) => unreachable!(),
        }
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::component::native::*;
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::component::{XlsxSnapshot, XlsxXmlPart};
use semio_framework_os_kernel::{
    io_schema::IoPayload,
    sqlite_snapshot::{SnapshotEncoding, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotPhase},
};
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::{retire_xml_document, XmlDocumentView};
use semio_s_artifact_stdio_zip::opc::native::{OpcNativeReader, OpcNativeWriter};

pub(crate) fn input(bytes: &[u8], binary: bool, control: &mut SqliteSnapshotControl<'_>) -> Result<XlsxSnapshot, ValueError> {
    backing::input(bytes, binary, control)
}

pub(super) fn decode_text(text: &str, control: &mut SqliteSnapshotControl<'_>) -> Result<XlsxSnapshot, ValueError> {
    input(text.as_bytes(), false, control)
}
}
pub use snapshot_wire_codec::*;
