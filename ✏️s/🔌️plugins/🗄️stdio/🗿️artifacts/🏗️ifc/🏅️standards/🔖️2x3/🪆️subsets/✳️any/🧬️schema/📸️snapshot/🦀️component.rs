//! 📸️ Ifc2x3Snapshot — the `2x3` standard's OWN typed snapshot (buildingSMART Coordination
//! View 2.0 era, IFC2X3 / ISO-PAS 16739:2005 schema, still ISO 10303-21 Part-21 syntax like
//! `📐️step`/`🔖️4`). Deliberately its own newtype (NOT a `pub use` of
//! `step::engine::part21::Part21Document`, and not the same Rust type as `4`'s `IfcSnapshot`) —
//! W1's own recon (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/STATUS.md`,
//! "shared-type violation" entry) flags reusing a cross-artifact type's IDENTITY as the exact
//! anti-pattern this repo bans ("copy-pasted shared types... die"). Reuse here is scoped to
//! PARSING CODE ONLY: this struct wraps a `Part21Document` as an internal field and the codec
//! below calls straight into `step::engine::part21::{parse_part21, write_part21}` — the tokenizer
//! itself is genuinely shared (IFC2X3 is STEP Part-21 syntax + a different EXPRESS schema), but
//! `Ifc2x3Snapshot` the TYPE is this standard's own.

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::{
    dec_edm_preamble_bin, dec_instance_list, dec_instance_list_bin, dec_optional_edm_preamble, dec_part21_header, dec_part21_header_bin, dec_str, enc_edm_preamble_bin, enc_instance_list_bin, enc_instance_list_into,
    enc_optional_edm_preamble, enc_part21_header, enc_part21_header_bin, enc_str, read_str_bin, write_str_bin,
};
use crate::artifacts::step::engine::part21::Part21Document;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
/// 🏷️ Document schema / DSL envelope id — distinct from `4`'s `"stdio.ifc"` so the two
/// standards' document codecs never collide in the shared `store::document_codec_registry`.
pub const STDIO_IFC2X3_DOCUMENT_SCHEMA: &str = "stdio.ifc.2x3";
/// 🧬️ Artifact schema descriptor id — distinct from `4`'s `"s.stdio.ifc"`.
pub const IFC2X3_ARTIFACT_SCHEMA_ID: &str = "s.stdio.ifc.2x3";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
/// 🏭️ Logical fields carried by an EXPRESS Data Manager Part-21 production header.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.2x3")]
pub struct Ifc2x3Snapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub document: Part21Document,
    #[state(artifact)]
    #[serde(default)]
    pub edm_preamble: Option<Ifc2x3EdmPreamble>,
}

impl Default for Ifc2x3Snapshot {
    async fn default() -> Self {
        Self { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document: Part21Document::default(), edm_preamble: None }
    }
}

/// ✅ Validates the logical IFC2X3 document without materializing native Part-21 text.
pub async fn validate_ifc2x3_snapshot(snapshot: &Ifc2x3Snapshot) -> Result<(), String> {
    if snapshot.schema != STDIO_IFC2X3_DOCUMENT_SCHEMA {
        return Err(format!("ifc2x3: unsupported snapshot schema {:?}", snapshot.schema));
    }
    let declares_ifc2x3 = snapshot.document.header.file_schema.iter().any(|value| value.as_list().map(|items| items.iter().any(|item| item.as_str() == Some("IFC2X3"))).unwrap_or(false));
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
impl store::ArtifactDsl for Ifc2x3Snapshot {
    const EXTENSION: &'static str = "ifc";
    async fn envelope_id() -> &'static str {
        STDIO_IFC2X3_DOCUMENT_SCHEMA
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_snapshot(body.trim()).map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
    }

    async fn print_dsl(&self) -> String {
        let mut body = String::with_capacity(self.document.instances.len().saturating_mul(64));
        body.push_str("schema=");
        body.push_str(&enc_str(&self.schema));
        body.push_str(" header=");
        body.push_str(&enc_part21_header(&self.document.header));
        body.push_str(" instances=");
        enc_instance_list_into(&self.document.instances, &mut body);
        body.push_str(" edm-preamble=");
        body.push_str(&enc_optional_edm_preamble(&self.edm_preamble));
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Ifc2x3Snapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let mut raw = vec![store::pack_rt::OP_BINARY_FORMAT];
        write_str_bin(&mut raw, &self.schema);
        enc_part21_header_bin(&self.document.header, &mut raw);
        enc_instance_list_bin(&self.document.instances, &mut raw);
        match &self.edm_preamble {
            None => raw.push(0),
            Some(preamble) => {
                raw.push(1);
                enc_edm_preamble_bin(preamble, &mut raw);
            }
        }
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let mut reader = store::ByteReader::new(&inner);
        let format = reader.read_u8().map_err(|error| store::PackError::Schema(format!("snapshot format: {error}")))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(store::PackError::Schema(format!("snapshot format: unsupported format {format}")));
        }
        let schema = read_str_bin(&mut reader).map_err(|error| store::PackError::Schema(format!("snapshot schema: {error}")))?;
        let header = dec_part21_header_bin(&mut reader).map_err(|error| store::PackError::Schema(format!("snapshot header: {error}")))?;
        let instances = dec_instance_list_bin(&mut reader).map_err(|error| store::PackError::Schema(format!("snapshot instances: {error}")))?;
        let edm_preamble = match reader.read_u8().map_err(|error| store::PackError::Schema(format!("snapshot EDM preamble presence: {error}")))? {
            0 => None,
            1 => Some(dec_edm_preamble_bin(&mut reader).map_err(|error| store::PackError::Schema(format!("snapshot EDM preamble: {error}")))?),
            tag => return Err(store::PackError::Schema(format!("snapshot EDM preamble presence: unknown tag {tag}"))),
        };
        if reader.remaining() != 0 {
            return Err(store::PackError::Schema(format!("snapshot trailing bytes: {}", reader.remaining())));
        }
        Ok(Self { schema, document: Part21Document { header, instances }, edm_preamble })
    }
}

async fn parse_snapshot(body: &str) -> Result<Ifc2x3Snapshot, String> {
    let mut schema = None;
    let mut header = None;
    let mut instances = None;
    let mut edm_preamble = None;
    for token in body.split_whitespace() {
        if let Some(value) = token.strip_prefix("schema=") {
            if schema.replace(dec_str(value)?).is_some() {
                return Err("duplicate snapshot schema".into());
            }
        } else if let Some(value) = token.strip_prefix("header=") {
            if header.replace(dec_part21_header(value)?).is_some() {
                return Err("duplicate snapshot header".into());
            }
        } else if let Some(value) = token.strip_prefix("instances=") {
            if instances.replace(dec_instance_list(value)?).is_some() {
                return Err("duplicate snapshot instances".into());
            }
        } else if let Some(value) = token.strip_prefix("edm-preamble=") {
            if edm_preamble.replace(dec_optional_edm_preamble(value)?).is_some() {
                return Err("duplicate snapshot EDM preamble".into());
            }
        } else {
            return Err(format!("ifc2x3 snapshot: unknown token {token:?}"));
        }
    }
    Ok(Ifc2x3Snapshot {
        schema: schema.ok_or_else(|| "missing snapshot schema".to_string())?,
        document: Part21Document { header: header.ok_or_else(|| "missing snapshot header".to_string())?, instances: instances.ok_or_else(|| "missing snapshot instances".to_string())? },
        edm_preamble: edm_preamble.ok_or_else(|| "missing snapshot EDM preamble".to_string())?,
    })
}
//#endregion 🔖️Codec
