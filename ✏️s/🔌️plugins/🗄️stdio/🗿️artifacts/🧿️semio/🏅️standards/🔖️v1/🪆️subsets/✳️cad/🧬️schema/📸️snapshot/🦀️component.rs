//! 🧬️ SemioCadSnapshot — layers/blocks/entities, informed by dxf r12's typed entity list, dwg's
//! `DwgDrawing`/`DwgEntity`/`DwgGeometry`, and the 📐️cad plugin's domain artifact (master plan
//! "Subset snapshot cores" table, `cad` row). `CadEntity` carries the full 9-variant vocabulary
//! (Line/Arc/Circle/Ellipse/Polyline/Text/Insert/Solid/Dimension).

use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOCAD_DOCUMENT_SCHEMA: &str = "stdio.semio.cad";
//#endregion 🔖️Ids

//#region 🔖️Entity
/// 📐️ Owned by the `cad` subset — a WEAK value struct (see `🔺️diff`'s module doc comment): whole-
/// value replaced in diffs, never sub-diffed, same treatment as `BcfCamera`/`XlsxCellValue`.
///
/// 🧪️ `Default` (with `Line` as the zero-length degenerate default) is required here, not for any
/// domain reason, but to satisfy a spurious `T: Default` bound the shared
/// `engine::triples::NamedTripleDiff<K,D,T>`'s derived `Deserialize` impl infers from its own
/// `#[serde(default)]`-annotated `added: Vec<T>` field (same known `serde_derive` quirk bcf's
/// local `NamedTripleDiff` copy already worked around via an explicit `#[serde(bound(...))]—`
/// the SHARED copy under `⚙️engine/🧰️triples` is missing that override; noted as a shared-infra
/// gap for the closer, not fixed here per this ticket's write-scope rules).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CadEntity {
    Line { a: SemioPoint2, b: SemioPoint2 },
    Arc { center: SemioPoint2, radius: f64, start_angle: f64, end_angle: f64 },
    Circle { center: SemioPoint2, radius: f64 },
    Ellipse { center: SemioPoint2, major_axis_end: SemioPoint2, ratio: f64, start_param: f64, end_param: f64 },
    Polyline { vertices: Vec<SemioPoint2>, closed: bool },
    Text { position: SemioPoint2, height: f64, rotation: f64, content: String },
    Insert { block_name: String, insertion_point: SemioPoint2, scale: SemioPoint2, rotation: f64 },
    Solid { p1: SemioPoint2, p2: SemioPoint2, p3: SemioPoint2, p4: SemioPoint2 },
    Dimension { def_point: SemioPoint2, text_position: SemioPoint2, measurement: f64, text: String },
}

/// 🧭️ Manual impl (not `#[derive(Default)]`) -- `Default` on an enum requires a UNIT default
/// variant, but every `CadEntity` variant carries fields, so the derive attribute is structurally
/// rejected here; hand-written zero-length `Line` matches what a derive-with-unit-variant would
/// have produced field-by-field if it were allowed. See this type's own doc comment above for WHY
/// `Default` is needed at all (the shared `engine::triples` spurious-bound workaround).
impl Default for CadEntity {
    fn default() -> Self {
        CadEntity::Line { a: SemioPoint2::default(), b: SemioPoint2::default() }
    }
}
//#endregion 🔖️Entity

//#region 🔖️Layer
/// 🗂️ Name-keyed (dxf `TABLES/LAYER`-style) — strong entity, own per-field diff. `Default` is the
/// same spurious-bound workaround `CadEntity` documents above.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadLayer {
    pub name: String,
    pub color_index: i32,
    pub line_type: String,
    pub visible: bool,
}
//#endregion 🔖️Layer

//#region 🔖️EntityRecord
/// 🏷️ One placed entity — `handle` is the id key (dxf group code 5); `layer` names the owning
/// `CadLayer` by reference. Referential invariants (dangling `layer`/`Insert.block_name`) are
/// checked by the composer's `SemioCadValidator`, not enforced structurally here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadEntityRecord {
    pub handle: String,
    pub layer: String,
    pub entity: CadEntity,
}
//#endregion 🔖️EntityRecord

//#region 🔖️Block
/// 📦️ Name-keyed (dxf `BLOCKS` section) — strong entity; `entities` is its own nested id-keyed
/// collection (same shape as the top-level `entities`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadBlock {
    pub name: String,
    pub base_point: SemioPoint2,
    #[serde(default)]
    pub entities: Vec<CadEntityRecord>,
}
//#endregion 🔖️Block

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.cad")]
pub struct SemioCadSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub layers: Vec<CadLayer>,
    #[state(persistent)]
    #[serde(default)]
    pub blocks: Vec<CadBlock>,
    #[state(persistent)]
    #[serde(default)]
    pub entities: Vec<CadEntityRecord>,
}

impl Default for SemioCadSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
            layers: Vec::new(),
            blocks: Vec::new(),
            entities: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🧭️ JSON-pack round trip wrapped in the shared `store::semio_format` envelope — honest for a
/// NEUTRAL semio type (not an on-disk file format with its own byte layout): the `📝️text`/
/// `💾️binary` grammar leaves under this facet document this exact envelope+JSON shape.
impl store::ArtifactDsl for SemioCadSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOCAD_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        serde_json::from_slice(&bytes).map_err(|e| store::TextError::new(format!("json decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioCadSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn populated_snapshot() -> SemioCadSnapshot {
        SemioCadSnapshot {
            schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
            layers: vec![CadLayer { name: "0".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true }],
            blocks: vec![CadBlock {
                name: "door".into(),
                base_point: SemioPoint2 { x: 0.0, y: 0.0 },
                entities: vec![CadEntityRecord { handle: "b1".into(), layer: "0".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 1.0, y: 0.0 } } }],
            }],
            entities: vec![
                CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 2.0, y: 2.0 }, radius: 1.5 } },
                CadEntityRecord { handle: "h2".into(), layer: "0".into(), entity: CadEntity::Insert { block_name: "door".into(), insertion_point: SemioPoint2 { x: 5.0, y: 5.0 }, scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 90.0 } },
            ],
        }
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = SemioCadSnapshot::default();
        let bytes = <SemioCadSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioCadSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = SemioCadSnapshot::default();
        let text = <SemioCadSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ Law 5 — `codec_retention_law`: decode(encode(x)) == x on a fully populated snapshot
    /// (layers/blocks/nested-block-entities/top-level entities incl. `Insert`), both facets.
    #[test]
    fn codec_retention_law() {
        let snap = populated_snapshot();
        let bytes = <SemioCadSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let via_pack = <SemioCadSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode pack");
        assert_eq!(via_pack, snap);

        let text = <SemioCadSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let via_dsl = <SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse dsl");
        assert_eq!(via_dsl, snap);
    }
}
//#endregion 🔖️Tests
