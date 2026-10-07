//! 🖋️ Logical SpreadsheetML namespace stamping.

use crate::XlsxSnapshot;
use super::{STRICT_R_NS, STRICT_SML_NS};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};

//#region 🔖️Stamp
/// 🖋️ Real-rewrites the main workbook XML part's root attrs to Strict shape. A no-op on the
/// rest of the package (worksheets/sharedStrings/relationships) -- only the three attrs
/// `check_strict_conformance` actually inspects change.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn stamp_strict_namespace(mut snapshot: XlsxSnapshot) -> XlsxSnapshot {
    let main_path = snapshot.workbook_part_path();
    if let Some(part) = main_path.as_deref().and_then(|path| snapshot.xml_part_mut(path)) {
        if let Some(XmlNode::Element { attrs, .. }) = &mut part.document.root {
            set_attr(attrs, "xmlns", STRICT_SML_NS);
            set_attr(attrs, "xmlns:r", STRICT_R_NS);
            set_attr(attrs, "conformance", "strict");
        }
    }
    snapshot
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn set_attr(attrs: &mut Vec<XmlAttr>, name: &str, value: &str) {
    if let Some(existing) = attrs.iter_mut().find(|a| a.name == name) {
        existing.value = value.into();
    } else {
        attrs.push(XmlAttr { name: name.into(), value: value.into() });
    }
}
//#endregion 🔖️Stamp

