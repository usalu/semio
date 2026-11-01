//! 📥️ Exact immutable ADD2 template declarations, decoded once from the committed IFCJ resource `templates.ifcj` whose neutral form is `🔣️.json`.
use super::{IfcTemplate, IfcTemplateCatalog, IfcTemplateEntry, IfcTemplateEntryKind, IfcTemplateEnumeration, IfcTemplateSource};
use semio_s_artifact_stdio_ifc_json_pack::{with_json_pack, Json, JsonPackError, Leaker};
use std::sync::OnceLock;

static CATALOG: OnceLock<IfcTemplateCatalog> = OnceLock::new();
const RESOURCE: &[u8] = include_bytes!("templates.ifcj");

impl IfcTemplateEntryKind {
    /// 🔑️ Resolve a kind from its source key.
    pub fn from_source_key(key: &str) -> Option<Self> {
        Some(match key {
            "P_SINGLEVALUE" => Self::SingleValue,
            "P_REFERENCEVALUE" => Self::ReferenceValue,
            "P_ENUMERATEDVALUE" => Self::EnumeratedValue,
            "P_LISTVALUE" => Self::ListValue,
            "P_BOUNDEDVALUE" => Self::BoundedValue,
            "P_TABLEVALUE" => Self::TableValue,
            "Q_WEIGHT" => Self::Weight,
            "Q_LENGTH" => Self::Length,
            "Q_AREA" => Self::Area,
            "Q_VOLUME" => Self::Volume,
            "Q_COUNT" => Self::Count,
            "Q_TIME" => Self::Time,
            _ => return None,
        })
    }
}

fn entry(leak: &mut Leaker, json: &Json<'_>) -> Result<IfcTemplateEntry, JsonPackError> {
    let fields = json.object(&["name", "definition", "kind", "dataType", "enumeration", "secondaryType", "expression"])?;
    let enumeration = match fields.get("enumeration") {
        None => None,
        Some(value) => {
            let row = value.object(&["name", "literals"])?;
            Some(IfcTemplateEnumeration { name: leak.text(row.require("name")?.string()?), literals: leak.texts(row.get("literals"))? })
        }
    };
    Ok(IfcTemplateEntry {
        name: leak.text(fields.require("name")?.string()?),
        definition: leak.text(fields.require("definition")?.string()?),
        kind: IfcTemplateEntryKind::from_source_key(fields.require("kind")?.string()?).ok_or(JsonPackError::Shape("template entry kind"))?,
        data_type: leak.text(fields.require("dataType")?.string()?),
        enumeration,
        secondary_type: leak.optional(fields.get("secondaryType"))?,
        expression: leak.optional(fields.get("expression"))?,
    })
}

fn template(leak: &mut Leaker, json: &Json<'_>) -> Result<IfcTemplate, JsonPackError> {
    let fields = json.object(&["name", "definition", "kind", "applicable", "classes", "sourceFile", "sourceSha256", "entries"])?;
    let entries = fields.require("entries")?.array()?.iter().map(|item| entry(leak, item)).collect::<Result<Vec<_>, _>>()?;
    Ok(IfcTemplate {
        name: leak.text(fields.require("name")?.string()?),
        definition: leak.text(fields.require("definition")?.string()?),
        kind: leak.text(fields.require("kind")?.string()?),
        applicable: leak.text(fields.require("applicable")?.string()?),
        classes: leak.texts(fields.get("classes"))?,
        entries: Leaker::slice(entries),
        source_file: leak.text(fields.require("sourceFile")?.string()?),
        source_sha256: leak.text(fields.require("sourceSha256")?.string()?),
    })
}

/// 📥️ Decode a committed template resource into immutable statics; every refusal is a closed cause.
pub fn decode_templates(pack: &[u8]) -> Result<IfcTemplateCatalog, JsonPackError> {
    with_json_pack(pack, |root| {
        let mut leak = Leaker::default();
        let fields = root.object(&["schema", "source", "templates"])?;
        let source = fields.require("source")?.object(&["url", "commit", "sha256", "language"])?;
        let templates = fields.require("templates")?.array()?.iter().map(|item| template(&mut leak, item)).collect::<Result<Vec<_>, _>>()?;
        Ok(IfcTemplateCatalog {
            schema: leak.text(fields.require("schema")?.string()?),
            source: IfcTemplateSource {
                url: leak.text(source.require("url")?.string()?),
                commit: leak.text(source.require("commit")?.string()?),
                sha256: leak.text(source.require("sha256")?.string()?),
                language: leak.text(source.require("language")?.string()?),
            },
            templates: Leaker::slice(templates),
        })
    })
}

/// 📖️ Borrow the full immutable release; the first call decodes the embedded resource once, which the per-resource laws prove cannot be refused (a refusal is a corrupted build artifact).
pub fn embedded_templates() -> &'static IfcTemplateCatalog {
    CATALOG.get_or_init(|| match decode_templates(RESOURCE) {
        Ok(catalog) => catalog,
        Err(error) => panic!("embedded IFC4x3 template resource refused: {error:?}"),
    })
}
