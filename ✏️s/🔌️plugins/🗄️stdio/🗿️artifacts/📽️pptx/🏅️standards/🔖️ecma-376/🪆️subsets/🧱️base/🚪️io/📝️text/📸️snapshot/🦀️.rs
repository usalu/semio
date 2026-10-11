//! 📝️ Text representation codec surface for `stdio.pptx` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type PptxSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::native;
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::*;
use crate::STDIO_PPTX_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::OpcPackage;

impl store::ArtifactDsl for PptxSnapshot {
    const EXTENSION: &'static str = "pptx";
    fn envelope_id() -> &'static str {
        "stdio.pptx"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        native::decode_text(text, &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_| true, semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default()))
            .map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        match crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::native::encode_standalone(
            self,
            semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Text,
            &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_| true, semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default()),
        )
        .expect("PPTX native ownership admission")
        {
            semio_framework_os_kernel::io_schema::IoPayload::Text(text) => text,
            semio_framework_os_kernel::io_schema::IoPayload::Binary(_) => unreachable!(),
        }
    }
}
}
pub use snapshot_codec::*;



impl crate::schema::snapshot::PptxSnapshot {
    pub fn part_text(&self, path: &str) -> Option<String> {
        let key = path.trim_start_matches('/');
        if let Some(part) = self.xml_parts.iter().find(|part| part.path == key) {
            return Some(semio_s_artifact_stdio_zip::opc::xml_document_to_opc_text(&part.document));
        }
        self.opc.part_bytes(key).and_then(|bytes| String::from_utf8(bytes.to_vec()).ok())
    }

    pub fn part_texts(&self) -> Vec<(String, String)> {
        let mut out: Vec<(String, String)> = self.xml_parts.iter().map(|part| (part.path.clone(), semio_s_artifact_stdio_zip::opc::xml_document_to_opc_text(&part.document))).collect();
        out.extend(self.opc.parts.iter().filter_map(|part| String::from_utf8(part.bytes.clone()).ok().map(|text| (part.path.clone(), text))));
        out.sort_by(|left, right| left.0.cmp(&right.0));
        out
    }
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
struct PptxBinaryPartRecord {
    path: String,
    content_type: String,
    #[dsl(base64)]
    bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
struct PptxRelationshipGroupRecord {
    owner: String,
    relationships: semio_framework_value::DslValue,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct PptxSnapshotRecord {
    schema: String,
    opc: semio_framework_value::DslValue,
    #[dsl(table)]
    binary_parts: Vec<PptxBinaryPartRecord>,
    #[dsl(table)]
    relationship_groups: Vec<PptxRelationshipGroupRecord>,
    xml_parts: semio_framework_value::DslValue,
}

impl PptxSnapshotRecord {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn from_snapshot(snapshot: &PptxSnapshot) -> Result<Self, String> {
        let mut opc = snapshot.opc.clone();
        let binary_parts = std::mem::take(&mut opc.parts).into_iter().map(|part| PptxBinaryPartRecord { path: part.path, content_type: part.content_type, bytes: part.bytes }).collect();
        let relationships = std::mem::take(&mut opc.relationships);
        let mut relationship_groups = relationships.into_groups().map(|(owner, relationships)| Ok(PptxRelationshipGroupRecord { owner, relationships: semio_framework_value::ToValue::to_value(&relationships) })).collect::<Result<Vec<_>, String>>()?;
        relationship_groups.sort_by(|left, right| left.owner.cmp(&right.owner));
        Ok(Self { schema: snapshot.schema.clone(), opc: semio_framework_value::ToValue::to_value(&opc), binary_parts, relationship_groups, xml_parts: semio_framework_value::ToValue::to_value(&snapshot.xml_parts) })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn into_snapshot(self) -> Result<PptxSnapshot, String> {
        let schema = self.schema;
        let mut opc: OpcPackage = semio_framework_value::FromValue::from_value(self.opc).map_err(|error| error.to_string())?;
        if !opc.parts.is_empty() {
            return Err("PPTX DSL OPC metadata must not contain binary parts".into());
        }
        if opc.relationships.owner_count() != 0 {
            return Err("PPTX DSL OPC metadata must not contain relationship groups".into());
        }
        opc.parts = self.binary_parts.into_iter().map(|part| semio_s_artifact_stdio_zip::opc::OpcPart { path: part.path, content_type: part.content_type, bytes: part.bytes }).collect();
        for group in self.relationship_groups {
            if opc.relationships.relationships(&group.owner).is_some() {
                return Err(format!("PPTX DSL repeats relationship owner {}", group.owner));
            }
            opc.relationships.replace_owner(group.owner, semio_framework_value::FromValue::from_value(group.relationships).map_err(|error| error.to_string())?);
        }
        let mut snapshot = PptxSnapshot::from_parts(opc, semio_framework_value::FromValue::from_value(self.xml_parts).map_err(|error| error.to_string())?);
        snapshot.schema = schema;
        Ok(snapshot)
    }
}


use crate::PptxSnapshot;
use semio_s_artifact_stdio_zip::opc::OpcPackage;
