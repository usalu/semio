//! 🧬️ EnergyModel snapshot schema — artifact-lane fields only.

use crate::artifacts::model::{energy_snapshot_with_state, EnergyStructureChild, EnergyZonesChild, ENERGY_MODEL_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted energy-model document snapshot (persistent fields of the artifact). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`energy→C:value,table R:model`): the old
/// `model_json: String` opaque-JSON field is replaced by two fixed composed CHILD slots — this
/// artifact no longer defines its own persisted-value/table content model, it composes stdio's
/// `value`/`table` subsets instead (see the artifact root's `🔖️Composition` region for the full
/// before/after and the honest exception carve-out for `Surface.vertices_m`). `referenced_model` is
/// a new forward `ArtifactLink` slot. `#[child(...)]`/`#[link_slot(...)]` drive
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Dropped the
/// `dsl::DslRecord` derive — `ArtifactChild<S>`/`ArtifactLink` have no `dsl::DslField` impl
/// reachable from this crate, the same wall every composed exemplar hit; every field's text/binary
/// shape is hand-rolled below instead.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    pub structure: EnergyStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub zones: EnergyZonesChild,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    #[serde(rename = "referencedModel", default, skip_serializing_if = "Option::is_none")]
    pub referenced_model: Option<store::ArtifactLink>,
}

impl Default for EnergyModelSnapshot {
    async fn default() -> Self {
        energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, crate::model::Model::default(), None)
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec — identical shape to `mathematical`'s/`layout`'s own
/// `enc_child`/`dec_child` (the working reference for a composite subset's child-handle
/// primitives): a handle is exactly two strings (`child_id`, the target's `ArtifactRef` flattened
/// via `to_uri()`), never the child's own content.
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
async fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
async fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}
async fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
async fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️JsonFieldPrimitives
/// 🧾️ `referenced_model` (an `Option<store::ArtifactLink>`) is JSON-serialized then hex-encoded,
/// same convention `layout`'s own `enc_json`/`dec_json` uses — `ArtifactLink`/`LinkPin`/`BlobRef`
/// are themselves plain `Serialize`/`Deserialize`, so no bespoke hex/bracket encoder is needed.
async fn enc_json<T: Serialize>(value: &T) -> String {
    enc_str(&serde_json::to_string(value).expect("EnergyModelSnapshot structured fields are always JSON-serializable"))
}
async fn dec_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    serde_json::from_str(&dec_str(s)?).map_err(|e| e.to_string())
}
//#endregion 🔖️JsonFieldPrimitives

//#region 🔖️TextPrimitives
async fn print_energy_model_snapshot_body(s: &EnergyModelSnapshot) -> String {
    format!(
        "schema={}\nstructure={}\nzones={}\nreferencedModel={}",
        enc_str(&s.schema),
        enc_child(&s.structure),
        enc_child(&s.zones),
        enc_json(&s.referenced_model),
    )
}
async fn parse_energy_model_snapshot_body(body: &str) -> Result<EnergyModelSnapshot, String> {
    let mut schema = None;
    let mut structure = None;
    let mut zones = None;
    let mut referenced_model = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("structure=") {
            structure = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("zones=") {
            zones = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("referencedModel=") {
            referenced_model = Some(dec_json(rest)?);
        } else {
            return Err(format!("energy model snapshot: unknown line {line:?}"));
        }
    }
    Ok(EnergyModelSnapshot {
        schema: schema.ok_or_else(|| "energy model snapshot: missing schema line".to_string())?,
        structure: structure.ok_or_else(|| "energy model snapshot: missing structure line".to_string())?,
        zones: zones.ok_or_else(|| "energy model snapshot: missing zones line".to_string())?,
        referenced_model: referenced_model.ok_or_else(|| "energy model snapshot: missing referencedModel line".to_string())?,
    })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
async fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
async fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
async fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
async fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
async fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
async fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
async fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}
async fn write_json<T: Serialize>(out: &mut Vec<u8>, value: &T) {
    write_str_lp(out, &serde_json::to_string(value).expect("EnergyModelSnapshot structured fields are always JSON-serializable"));
}
async fn read_json<T: serde::de::DeserializeOwned>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    serde_json::from_str(&read_str_lp(reader)?).map_err(|e| e.to_string())
}

async fn encode_energy_model_snapshot_binary(s: &EnergyModelSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_child(&mut out, &s.structure);
    write_child(&mut out, &s.zones);
    write_json(&mut out, &s.referenced_model);
    out
}
async fn decode_energy_model_snapshot_binary(bytes: &[u8]) -> Result<EnergyModelSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    Ok(EnergyModelSnapshot {
        schema: read_str_lp(&mut reader)?,
        structure: read_child(&mut reader)?,
        zones: read_child(&mut reader)?,
        referenced_model: read_json(&mut reader)?,
    })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for EnergyModelSnapshot {
    const EXTENSION: &'static str = "energy";
    async fn envelope_id() -> &'static str {
        "energy.model"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_energy_model_snapshot_body(body.trim()).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let body = print_energy_model_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for EnergyModelSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_energy_model_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) =
            store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        decode_energy_model_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
mod round_trip_tests {
    use super::*;

    async fn sample_with_composition() -> EnergyModelSnapshot {
        let mut snapshot = energy_snapshot_with_state(
            ENERGY_MODEL_DOCUMENT_SCHEMA,
            crate::model::Model { name: "Demo".into(), version: "1".into(), ..crate::model::Model::default() },
            None,
        );
        snapshot.referenced_model = Some(store::ArtifactLink {
            target: store::os_io::ArtifactRef::parse_uri("doc-2!s.stdio.semio@v1/model").expect("valid link ref uri"),
            pin: store::LinkPin::Head,
            role: "model".into(),
        });
        snapshot
    }

    /// 🧪️ Every field on `EnergyModelSnapshot` — including the two composition slots and the link
    /// slot — must survive both hand-rolled codecs (text and binary), independently. Codec
    /// completeness is not caught by `cargo check`; this is the real round-trip proof the migration
    /// recipe requires.
    #[test]
    async fn structure_zones_and_referenced_model_round_trip_through_text_and_binary() {
        let snapshot = sample_with_composition();
        let text = store::ArtifactDsl::print_dsl(&snapshot);
        let from_text = <EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse round-tripped text");
        assert_eq!(from_text, snapshot);

        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let from_binary = <EnergyModelSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode round-tripped binary");
        assert_eq!(from_binary, snapshot);
    }

    #[test]
    async fn absent_link_slot_round_trips_as_none() {
        let mut snapshot = sample_with_composition();
        snapshot.referenced_model = None;
        let text = store::ArtifactDsl::print_dsl(&snapshot);
        assert_eq!(<EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), snapshot);
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        assert_eq!(<EnergyModelSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode"), snapshot);
    }

    /// 🧪️ `energy_structure_from_model`/`energy_model_from_structure` round-trip the whole `Model`
    /// losslessly through the generic JSON<->`SemioValue` bridge — the real codec-completeness proof
    /// for the artifact root's `🔖️Converters` region.
    #[test]
    async fn model_round_trips_through_the_structure_child_content() {
        let model = crate::model::Model {
            name: "Demo".into(),
            version: "1".into(),
            zones: vec![crate::model::Zone {
                id: crate::model::EntityId(1),
                name: "Zone1".into(),
                volume_m3: 100.0,
                multiplier: 1,
                conditioned: true,
                part_of_total_floor_area: true,
            }],
            ..crate::model::Model::default()
        };
        let structure = crate::artifacts::model::energy_structure_from_model(&model);
        let restored = crate::artifacts::model::energy_model_from_structure(&structure);
        assert_eq!(restored, model);
    }
}
//#endregion 🧪️Tests
