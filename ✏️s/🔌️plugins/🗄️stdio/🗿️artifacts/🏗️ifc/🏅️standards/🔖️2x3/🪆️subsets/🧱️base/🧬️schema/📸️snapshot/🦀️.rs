//! 📸️ Ifc2x3Snapshot — the `2x3` standard's OWN typed snapshot (buildingSMART Coordination
//! View 2.0 era, IFC2X3 / ISO-PAS 16739:2005 schema, still ISO 10303-21 Part-21 syntax like
//! `📐️step`/`4️⃣4`). Deliberately its own newtype (NOT a `pub use` of
//! `semio_s_artifact_stdio_contract::part21::Part21Document`, and not the same Rust type as `4`'s `IfcSnapshot`) —
//! W1's own recon (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/STATUS.md`,
//! "shared-type violation" entry) flags reusing a cross-artifact type's IDENTITY as the exact
//! anti-pattern this repo bans ("copy-pasted shared types... die"). Reuse here is scoped to
//! PARSING CODE ONLY: this struct wraps a `Part21Document` as an internal field and the codec
//! below calls straight into `semio_s_artifact_stdio_contract::part21::{parse_part21, write_part21}` — the tokenizer
//! itself is genuinely shared (IFC2X3 is STEP Part-21 syntax + a different EXPRESS schema), but
//! `Ifc2x3Snapshot` the TYPE is this standard's own.

use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_contract::part21::Part21Document;
//#region 🔖️Ids
/// 🏷️ Document schema / DSL envelope id — distinct from `4`'s `"stdio.ifc"` so the two
/// standards' document codecs never collide in the shared `store::document_codec_registry`.
pub const STDIO_IFC2X3_DOCUMENT_SCHEMA: &str = "stdio.ifc.2x3";
/// 🧬️ Artifact schema descriptor id — distinct from `4`'s `"s.stdio.ifc"`.
pub const IFC2X3_ARTIFACT_SCHEMA_ID: &str = "s.stdio.ifc.2x3";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
/// 🏭️ Logical fields carried by an EXPRESS Data Manager Part-21 production header.
#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Ifc2x3EdmPreamble {
    pub producer: String,
    pub module: String,
    pub creation_date: String,
    pub host: String,
    pub database: String,
    pub database_version: String,
    pub database_creation_date: String,
    pub schema: String,
    pub model: String,
    pub model_creation_date: String,
    pub header_model: String,
    pub header_model_creation_date: String,
    pub user: String,
    pub group: String,
    pub license: String,
    pub options: String,
}

/// 📸️ Persisted `stdio.ifc.2x3` snapshot — the full, lossless generic Part-21 graph, own type.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.2x3")]
pub struct Ifc2x3Snapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub document: Part21Document,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edm_preamble: Option<Ifc2x3EdmPreamble>,
}

impl Default for Ifc2x3Snapshot {
    fn default() -> Self {
        Self { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document: Part21Document::default(), edm_preamble: None }
    }
}

/// ✅ Validates the logical IFC2X3 document without materializing native Part-21 text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_ifc2x3_snapshot(snapshot: &Ifc2x3Snapshot) -> Result<(), String> {
    if snapshot.schema != STDIO_IFC2X3_DOCUMENT_SCHEMA {
        return Err(format!("ifc2x3: unsupported snapshot schema {:?}", snapshot.schema));
    }
    let declares_ifc2x3 = snapshot.document.header.file_schema.iter().any(|value| value.as_list().is_some_and(|items| items.iter().any(|item| item.as_str() == Some("IFC2X3"))));
    if !declares_ifc2x3 {
        return Err("ifc2x3: FILE_SCHEMA does not declare IFC2X3".into());
    }
    let mut ids = std::collections::HashSet::new();
    for instance in &snapshot.document.instances {
        if !ids.insert(instance.id) {
            return Err(format!("ifc2x3: duplicate instance #{}", instance.id));
        }
        if instance.entities.is_empty() {
            return Err(format!("ifc2x3: instance #{} has no entities", instance.id));
        }
    }
    Ok(())
}
//#endregion 🔖️Snapshot

//#region 🔖️Codec


//#endregion 🔖️Codec

semio_framework_value::artifact_retire_struct!(Ifc2x3EdmPreamble { producer,module,creation_date,host,database,database_version,database_creation_date,schema,model,model_creation_date,header_model,header_model_creation_date,user,group,license,options });
semio_framework_value::artifact_retire_struct!(Ifc2x3Snapshot { schema,document,edm_preamble });
