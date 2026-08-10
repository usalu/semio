//! 🧬️ DxfSnapshot schema — persistent fields + real codecs.
//!
//! DXF R12 ASCII's own wire format is a flat stream of `(group code, value)` pairs, one per
//! line. That tag stream is the lossless source of truth persisted on `DxfSnapshot` — every
//! tag round-trips through `parse_dsl`/`print_dsl` even for entity types and group codes this
//! codec has no typed view for (e.g. `POLYLINE`, `TEXT`, unrecognized codes). Typed entity
//! views (`DxfLine`/`DxfCircle`/`DxfArc`/`DxfLwPolyline`) are read-only accessors computed
//! on demand by scanning the tag stream — they never become the encode source, so mutating a
//! view alone has no effect; encode always regenerates text from `tags` directly.

use crate::artifacts::dxf::STDIO_DXF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️TagStream
/// 🏷️ One raw DXF group-code/value pair — the lossless unit of the wire format.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfTag {
    pub code: i32,
    pub value: String,
}
//#endregion 🔖️TagStream

//#region 🔖️EntityViews
/// 📐️ `LINE` entity view — group codes 10/20/30 (start) and 11/21/31 (end).
///
/// 📌️ `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg` has no typed entity structs yet; when it
/// grows LINE/CIRCLE/ARC/LWPOLYLINE views it should adopt these shapes rather than defining
/// its own, so the two formats share one typed-entity vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DxfLine {
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub x2: f64,
    pub y2: f64,
    pub z2: f64,
}

/// 🔘 `CIRCLE` entity view — group codes 10/20/30 (center), 40 (radius).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DxfCircle {
    pub cx: f64,
    pub cy: f64,
    pub cz: f64,
    pub radius: f64,
}

/// 🌙 `ARC` entity view — group codes 10/20/30 (center), 40 (radius), 50/51 (start/end angle).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DxfArc {
    pub cx: f64,
    pub cy: f64,
    pub cz: f64,
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
}

/// 🪢 `LWPOLYLINE` entity view — group code 70 (flags, bit 0 = closed), repeated 10/20 vertex
/// pairs, 90 (vertex count, informational — the actual vertex count is `vertices.len()`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DxfLwPolyline {
    pub vertices: Vec<(f64, f64)>,
    pub closed: bool,
}
//#endregion 🔖️EntityViews

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf")]
pub struct DxfSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub tags: Vec<DxfTag>,
}

impl Default for DxfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), tags: Vec::new() }
    }
}

impl DxfSnapshot {
    //#region 🔖️TypedViews
    /// 📐️ Scans the tag stream for `LINE` entities.
    pub fn lines(&self) -> Vec<DxfLine> {
        entity_bodies(&self.tags, "LINE").into_iter().map(parse_line_entity).collect()
    }
    /// 🔘 Scans the tag stream for `CIRCLE` entities.
    pub fn circles(&self) -> Vec<DxfCircle> {
        entity_bodies(&self.tags, "CIRCLE").into_iter().map(parse_circle_entity).collect()
    }
    /// 🌙 Scans the tag stream for `ARC` entities.
    pub fn arcs(&self) -> Vec<DxfArc> {
        entity_bodies(&self.tags, "ARC").into_iter().map(parse_arc_entity).collect()
    }
    /// 🪢 Scans the tag stream for `LWPOLYLINE` entities.
    pub fn lwpolylines(&self) -> Vec<DxfLwPolyline> {
        entity_bodies(&self.tags, "LWPOLYLINE").into_iter().map(parse_lwpolyline_entity).collect()
    }
    //#endregion 🔖️TypedViews
}
//#endregion 🔖️Snapshot

//#region 🔖️FormatCodec
/// 📥️ Tokenizes raw DXF ASCII text into its lossless `(code, value)` tag stream. Every
/// well-formed line pair becomes one `DxfTag`, regardless of whether this codec has a typed
/// view for the entity/section it belongs to — that's what makes round-tripping lossless.
/// Blank trailing lines are ignored; group codes and values are trimmed of surrounding
/// whitespace and `\r` (a deliberate normalization — see module docs).
pub fn tokenize_dxf(text: &str) -> Result<Vec<DxfTag>, String> {
    let raw: Vec<&str> = text.lines().map(|l| l.trim_end_matches('\r')).collect();
    let mut tags = Vec::new();
    let mut i = 0usize;
    while i < raw.len() {
        let code_line = raw[i].trim();
        if code_line.is_empty() {
            i += 1;
            continue;
        }
        let value = raw.get(i + 1).ok_or_else(|| format!("dxf: group code {code_line:?} missing its value line"))?;
        let code: i32 = code_line.parse().map_err(|e| format!("dxf: invalid group code {code_line:?}: {e}"))?;
        tags.push(DxfTag { code, value: value.trim().to_string() });
        i += 2;
    }
    Ok(tags)
}

/// 📤️ Writes the tag stream back out as DXF ASCII text — the encode side always regenerates
/// from `tags` (the source of truth), never from the derived typed views.
pub fn write_dxf_tags(tags: &[DxfTag]) -> String {
    let mut out = String::with_capacity(tags.len() * 8);
    for t in tags {
        out.push_str(&t.code.to_string());
        out.push('\n');
        out.push_str(&t.value);
        out.push('\n');
    }
    out
}

/// 🔎 Slices the tag stream into the bodies (tags after the `0`/`kind` header, up to but
/// excluding the next `0`-code tag) of every entity whose type matches `kind`.
fn entity_bodies<'a>(tags: &'a [DxfTag], kind: &str) -> Vec<&'a [DxfTag]> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < tags.len() {
        if tags[i].code == 0 && tags[i].value == kind {
            let start = i + 1;
            let mut end = start;
            while end < tags.len() && tags[end].code != 0 {
                end += 1;
            }
            out.push(&tags[start..end]);
            i = end;
        } else {
            i += 1;
        }
    }
    out
}

fn f64_at(v: &str) -> f64 {
    v.trim().parse::<f64>().unwrap_or(0.0)
}

fn parse_line_entity(body: &[DxfTag]) -> DxfLine {
    let mut line = DxfLine::default();
    for t in body {
        match t.code {
            10 => line.x1 = f64_at(&t.value),
            20 => line.y1 = f64_at(&t.value),
            30 => line.z1 = f64_at(&t.value),
            11 => line.x2 = f64_at(&t.value),
            21 => line.y2 = f64_at(&t.value),
            31 => line.z2 = f64_at(&t.value),
            _ => {}
        }
    }
    line
}

fn parse_circle_entity(body: &[DxfTag]) -> DxfCircle {
    let mut circle = DxfCircle::default();
    for t in body {
        match t.code {
            10 => circle.cx = f64_at(&t.value),
            20 => circle.cy = f64_at(&t.value),
            30 => circle.cz = f64_at(&t.value),
            40 => circle.radius = f64_at(&t.value),
            _ => {}
        }
    }
    circle
}

fn parse_arc_entity(body: &[DxfTag]) -> DxfArc {
    let mut arc = DxfArc::default();
    for t in body {
        match t.code {
            10 => arc.cx = f64_at(&t.value),
            20 => arc.cy = f64_at(&t.value),
            30 => arc.cz = f64_at(&t.value),
            40 => arc.radius = f64_at(&t.value),
            50 => arc.start_angle = f64_at(&t.value),
            51 => arc.end_angle = f64_at(&t.value),
            _ => {}
        }
    }
    arc
}

fn parse_lwpolyline_entity(body: &[DxfTag]) -> DxfLwPolyline {
    let mut poly = DxfLwPolyline::default();
    let mut pending_x: Option<f64> = None;
    for t in body {
        match t.code {
            70 => poly.closed = t.value.trim().parse::<i64>().map(|f| f & 1 == 1).unwrap_or(false),
            10 => pending_x = Some(f64_at(&t.value)),
            20 => {
                if let Some(x) = pending_x.take() {
                    poly.vertices.push((x, f64_at(&t.value)));
                }
            }
            _ => {}
        }
    }
    poly
}
//#endregion 🔖️FormatCodec

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for DxfSnapshot {
    const EXTENSION: &'static str = "dxf";
    fn envelope_id() -> &'static str { "stdio.dxf" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let tags = tokenize_dxf(body).map_err(|e| store::TextError::new(format!("dxf parse: {e}"), dsl::TextSpan::at(1, 1)))?;
        Ok(Self { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), tags })
    }
    fn print_dsl(&self) -> String {
        let body = write_dxf_tags(&self.tags);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DxfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_dxf_tags(&self.tags).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let tags = tokenize_dxf(&text).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), tags })
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_dxf_text() -> String {
        // A HEADER section (unmodeled — proves losslessness), a POLYLINE entity (also
        // unmodeled — the codec has no typed view for it), and one each of LINE/CIRCLE/
        // ARC/LWPOLYLINE (the modeled typed views).
        concat!(
            "0\nSECTION\n2\nHEADER\n9\n$ACADVER\n1\nAC1009\n0\nENDSEC\n",
            "0\nSECTION\n2\nENTITIES\n",
            "0\nLINE\n8\n0\n10\n1\n20\n2\n30\n3\n11\n4\n21\n5\n31\n6\n",
            "0\nCIRCLE\n8\n0\n10\n10\n20\n20\n30\n30\n40\n5\n",
            "0\nARC\n8\n0\n10\n1\n20\n2\n30\n3\n40\n7\n50\n0\n51\n180\n",
            "0\nLWPOLYLINE\n8\n0\n90\n3\n70\n1\n10\n0\n20\n0\n10\n1\n20\n0\n10\n1\n20\n1\n",
            "0\nPOLYLINE\n8\n0\n66\n1\n0\nVERTEX\n8\n0\n10\n9\n20\n9\n0\nSEQEND\n",
            "0\nENDSEC\n0\nEOF\n",
        ).to_string()
    }

    #[test]
    fn tag_stream_round_trips_including_unmodeled_entities() {
        let text = sample_dxf_text();
        let tags = tokenize_dxf(&text).expect("tokenize");
        // POLYLINE/VERTEX/SEQEND and the HEADER's $ACADVER are not modeled by any typed view —
        // yet every tag must still be present, in order, verbatim.
        let round_tripped = write_dxf_tags(&tags);
        assert_eq!(round_tripped, text, "tag-for-tag round trip must be exact, including unmodeled tags");
        // And decoding the round-tripped text again must yield the identical tag stream.
        let tags_again = tokenize_dxf(&round_tripped).expect("re-tokenize");
        assert_eq!(tags_again, tags);
    }

    #[test]
    fn snapshot_parse_dsl_print_dsl_round_trips_tag_stream() {
        let text = sample_dxf_text();
        let tags = tokenize_dxf(&text).expect("tokenize");
        let snap = DxfSnapshot { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), tags: tags.clone() };
        let printed = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <DxfSnapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("parse");
        assert_eq!(parsed.tags, tags, "DSL round trip must preserve every tag, modeled or not");

        let packed = store::ArtifactPack::encode_pack(&snap);
        let unpacked = <DxfSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("unpack");
        assert_eq!(unpacked.tags, tags, "pack round trip must preserve every tag, modeled or not");
    }

    #[test]
    fn typed_views_parse_line_circle_arc_lwpolyline() {
        let tags = tokenize_dxf(&sample_dxf_text()).expect("tokenize");
        let snap = DxfSnapshot { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), tags };

        let lines = snap.lines();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], DxfLine { x1: 1.0, y1: 2.0, z1: 3.0, x2: 4.0, y2: 5.0, z2: 6.0 });

        let circles = snap.circles();
        assert_eq!(circles.len(), 1);
        assert_eq!(circles[0], DxfCircle { cx: 10.0, cy: 20.0, cz: 30.0, radius: 5.0 });

        let arcs = snap.arcs();
        assert_eq!(arcs.len(), 1);
        assert_eq!(arcs[0], DxfArc { cx: 1.0, cy: 2.0, cz: 3.0, radius: 7.0, start_angle: 0.0, end_angle: 180.0 });

        let polylines = snap.lwpolylines();
        assert_eq!(polylines.len(), 1);
        assert_eq!(polylines[0].closed, true);
        assert_eq!(polylines[0].vertices, vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]);
    }
}
//#endregion 🧪️Tests
