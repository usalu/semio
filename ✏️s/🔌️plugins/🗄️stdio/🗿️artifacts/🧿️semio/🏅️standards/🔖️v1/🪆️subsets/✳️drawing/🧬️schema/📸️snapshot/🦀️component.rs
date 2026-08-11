//! 🧬️ SemioDrawingSnapshot — canvas + name-keyed styles + ordered layers, each a recursive
//! `DrawNode`{Path{segments}/Text/Group{transform,children}/Image} scene graph — from svg;
//! replaces DwgDrawing-as-neutral. Real, complete-per-spec-row shape (master plan "drawing" row):
//! no `serde_json::Value`, no bare tuples/nested fixed arrays (geometry fields reuse
//! `engine::geometry`'s named structs throughout).

use crate::artifacts::semio::standards::v1::engine::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️PathSegment
/// ✏️ A single SVG-style path command — the honest, complete production set for `Path.segments`
/// (no `*OCTET`/size-eos catch-all: every field a real drawn quantity).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum PathSegment {
    MoveTo { to: SemioPoint2 },
    LineTo { to: SemioPoint2 },
    CubicTo { c1: SemioPoint2, c2: SemioPoint2, to: SemioPoint2 },
    QuadTo { c: SemioPoint2, to: SemioPoint2 },
    /// 🌙️ Elliptical arc, SVG `A rx ry x-rotation large-arc sweep x y` shape.
    ArcTo { rx: f64, ry: f64, x_rotation: f64, large_arc: bool, sweep: bool, to: SemioPoint2 },
    Close,
}
//#endregion 🔖️PathSegment

//#region 🔖️DrawNode
/// 🖍️ Owned by the `drawing` subset: the recursive scene-graph node, matching svg's
/// `SvgNodeDiff` recursive-diff template per the master plan. `style` fields are a referential
/// `Option<String>` into `SemioDrawingSnapshot.styles` by name (checked by `SemioDrawingValidator`
/// — dangling references are a real referential-invariant breach, not silently tolerated).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DrawNode {
    Path {
        segments: Vec<PathSegment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<String>,
    },
    Text {
        value: String,
        at: SemioPoint2,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<String>,
    },
    Group {
        transform: SemioTransform,
        #[serde(default)]
        children: Vec<DrawNode>,
    },
    /// 🖼️ Raster payload embedded verbatim (typed raw retention — real bytes, not a lie).
    Image { at: SemioPoint2, width: f64, height: f64, mime: String, bytes: Vec<u8> },
}

impl Default for DrawNode {
    fn default() -> Self { DrawNode::Group { transform: SemioTransform::identity(), children: Vec::new() } }
}
//#endregion 🔖️DrawNode

//#region 🔖️Style
/// 🎨️ A named presentation style, referenced by `DrawNode::Path`/`Text.style`. Name-keyed
/// (`NamedTripleDiff<String, DrawStyleDiff, DrawStyle>` in the diff facet).
/// 🩹 `Default` derived (not just decoration) — required transitively as the `T` of
/// `triples::NamedTripleDiff<String, DrawStyleDiff, DrawStyle>`'s generated `Deserialize` impl
/// (serde-derive's bound inference for `#[serde(default)]` fields on a generic container reaches
/// every type parameter, not just the immediately-defaulted field's own type).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawStyle {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fill: Option<SemioRgba>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<SemioRgba>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f32>,
}
//#endregion 🔖️Style

//#region 🔖️Layer
/// 🗂️ One ordered layer (index-keyed z-order, `IndexedTripleDiff<DrawLayerDiff, DrawLayer>` in
/// the diff facet — mirrors gif-frame ordering precedent).
/// 🩹 `Default` derived for the same reason as `DrawStyle` above (needed as the `T` of
/// `triples::IndexedTripleDiff<DrawLayerDiff, DrawLayer>`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayer {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub root: DrawNode,
}
//#endregion 🔖️Layer

//#region 🔖️Canvas
/// 🖼️ Document-level viewport/backdrop.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawCanvas {
    pub width: f64,
    pub height: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<SemioRgba>,
}

impl Default for DrawCanvas {
    fn default() -> Self { Self { width: 0.0, height: 0.0, background: None } }
}
//#endregion 🔖️Canvas

//#region 🔖️Ids
pub const STDIO_SEMIODRAWING_DOCUMENT_SCHEMA: &str = "stdio.semio.drawing";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.drawing")]
pub struct SemioDrawingSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub canvas: DrawCanvas,
    #[state(persistent)]
    #[serde(default)]
    pub styles: Vec<DrawStyle>,
    #[state(persistent)]
    #[serde(default)]
    pub layers: Vec<DrawLayer>,
}

impl Default for SemioDrawingSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
            canvas: DrawCanvas::default(),
            styles: Vec::new(),
            layers: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🗜️ JSON-pack round trip (honest, genuinely working — not a per-format binary codec, since
/// this subset's snapshot is a NEUTRAL semio type, not an on-disk file format). Wrapped in the
/// same `store::semio_format` envelope every stdio artifact uses.
impl store::ArtifactDsl for SemioDrawingSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIODRAWING_DOCUMENT_SCHEMA }

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

impl store::ArtifactPack for SemioDrawingSnapshot {
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

    fn sample() -> SemioDrawingSnapshot {
        SemioDrawingSnapshot {
            schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
            canvas: DrawCanvas { width: 100.0, height: 50.0, background: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
            styles: vec![DrawStyle { name: "s1".into(), fill: Some(SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(2.0), opacity: None }],
            layers: vec![DrawLayer {
                id: "l0".into(),
                name: "base".into(),
                visible: true,
                root: DrawNode::Group {
                    transform: SemioTransform::identity(),
                    children: vec![
                        DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }, PathSegment::LineTo { to: SemioPoint2 { x: 10.0, y: 10.0 } }, PathSegment::Close], style: Some("s1".into()) },
                        DrawNode::Text { value: "hi".into(), at: SemioPoint2 { x: 5.0, y: 5.0 }, style: None },
                        DrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: 8.0, height: 8.0, mime: "image/png".into(), bytes: vec![1, 2, 3] },
                    ],
                },
            }],
        }
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = sample();
        let bytes = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = sample();
        let text = <SemioDrawingSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[test]
    fn default_snapshot_round_trips() {
        let snap = SemioDrawingSnapshot::default();
        let bytes = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }
}
//#endregion 🔖️Tests
