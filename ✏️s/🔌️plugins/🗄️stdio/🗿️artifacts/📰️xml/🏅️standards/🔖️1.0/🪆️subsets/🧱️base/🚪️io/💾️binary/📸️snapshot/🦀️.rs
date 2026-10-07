//! binary rep for stdio.xml 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1_0::subsets::base::schema::snapshot::*;
use crate::STDIO_XML_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
/// 🧩️ Typed XML components for enclosing owned native documents.

/// 🧪️ P2-FG1: `stdio.xml` is TEXT-NATIVE (per the W0 census row) — there is no "binary XML"; the
/// pack container is the SEMIO envelope wrapping the artifact's own REAL wire text
/// (`xml_document_to_text`/`xml_document_from_text`) verbatim, same treatment json's own
/// `ArtifactPack` gives its RFC8259 text (`🔣️json/…/📸️snapshot/🦀️.rs`'s
/// `write_json_text(&self.value).into_bytes()`). Replaces the previous `serde_json::to_vec`/
/// `from_slice` placeholder, which satisfied the trait but was a literal-JSON-payload-disguised-as-
/// binary violation of `POLICY_STDIO_JSON_TRANSFER_BAN` (flagged by name in the P2-W0 recon report,
/// `xml` row, "Yes — in scope").
impl store::ArtifactPack for XmlSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let mut raw = vec![1];
        crate::standards::v1_0::subsets::base::io::binary::snapshot::encode_snapshot_binary(self, &mut raw);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        let mut reader = store::ByteReader::new(&inner);
        let version = reader.read_u8().map_err(|e| store::PackError::from(e))?;
        if version != 1 {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("unsupported xml snapshot state version {version}"))));
        }
        crate::standards::v1_0::subsets::base::io::binary::snapshot::decode_snapshot_binary(&mut reader).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use crate::standards::v1_0::subsets::base::schema::mutation_support::*;
use crate::schema::snapshot::{XmlDocument, XmlNode};
use crate::XmlSnapshot;

pub(crate) fn encode_snapshot_binary(snapshot: &XmlSnapshot, output: &mut Vec<u8>) {
    use crate::standards::v1_0::subsets::base::io::binary::diff::{enc_xml_node_bin};
    use crate::standards::v1_0::subsets::base::io::binary::diff::{write_str_lp};
    use crate::standards::v1_0::subsets::base::io::binary::diff::{enc_declaration_bin};
    use crate::standards::v1_0::subsets::base::io::binary::diff::{enc_doctype_bin};
    use crate::standards::v1_0::subsets::base::io::binary::diff::{enc_prolog_bin};
    write_str_lp(output, &snapshot.schema);
    output.push(u8::from(snapshot.doc.root.is_some()));
    if let Some(root) = &snapshot.doc.root {
        enc_xml_node_bin(root, output);
    }
    output.push(u8::from(snapshot.doc.doctype.is_some()));
    if let Some(doctype) = &snapshot.doc.doctype {
        enc_doctype_bin(doctype, output);
    }
    output.push(u8::from(snapshot.doc.declaration.is_some()));
    if let Some(declaration) = &snapshot.doc.declaration {
        enc_declaration_bin(declaration, output);
    }
    enc_prolog_bin(&snapshot.doc.prolog, output);
    enc_prolog_bin(&snapshot.doc.epilog, output);
}

pub(crate) fn decode_snapshot_binary(reader: &mut store::ByteReader<'_>) -> Result<XmlSnapshot, String> {
    use crate::standards::v1_0::subsets::base::io::binary::diff::{dec_xml_node_bin};
    use crate::standards::v1_0::subsets::base::io::binary::diff::{read_str_lp};
    use crate::standards::v1_0::subsets::base::io::binary::diff::{dec_declaration_bin};
    use crate::standards::v1_0::subsets::base::io::binary::diff::{dec_doctype_bin};
    use crate::standards::v1_0::subsets::base::io::binary::diff::{dec_prolog_bin};
    let schema = read_str_lp(reader)?;
    let root = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_xml_node_bin(reader)?) } else { None };
    let doctype = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_doctype_bin(reader)?) } else { None };
    let declaration = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_declaration_bin(reader)?) } else { None };
    let prolog = dec_prolog_bin(reader)?;
    let epilog = dec_prolog_bin(reader)?;
    let snapshot = XmlSnapshot { schema, doc: XmlDocument { root, doctype, declaration, prolog, epilog } };
    Ok(snapshot)
}
}
pub use snapshot_wire_codec::*;
