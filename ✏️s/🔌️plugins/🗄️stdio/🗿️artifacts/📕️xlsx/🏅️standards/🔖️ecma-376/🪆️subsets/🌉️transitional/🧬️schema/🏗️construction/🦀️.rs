//! 🖋️ Logical SpreadsheetML namespace stamping.

use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::stamp_workbook_namespaces;

//#region 🔖️Stamp
/// 🖋️ Stamps all SpreadsheetML namespace and relationship declarations as transitional.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn stamp_transitional_namespace(snapshot: XlsxSnapshot) -> XlsxSnapshot {
    stamp_workbook_namespaces(snapshot, false)
}
//#endregion 🔖️Stamp

