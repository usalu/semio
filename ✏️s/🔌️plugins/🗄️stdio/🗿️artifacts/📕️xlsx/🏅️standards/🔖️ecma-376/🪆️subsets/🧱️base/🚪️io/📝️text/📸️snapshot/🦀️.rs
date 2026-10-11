//! 📝️ Text representation codec surface for `stdio.xlsx` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type XlsxSnapshotText = String;
//#endregion 🚚️Carrier

use crate::XlsxSnapshot;
use crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::native::decode_text;

impl store::ArtifactDsl for XlsxSnapshot {
    const EXTENSION: &'static str = "xlsx";
    fn envelope_id() -> &'static str {
        "stdio.xlsx"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        decode_text(text, &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_| true, semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default()))
            .map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        match crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::native::encode_standalone(
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
