//! 🧬️ JpgArtifact schema — full artifact state.

use crate::JpgSnapshot;
use framework_schema::ArtifactSchema;

/// 🎪️ Reduced UI-editable view: identity + the raster the user is directly manipulating. Ticket
/// 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION killed the shared
/// `RasterImage` wrapper (jpg/png/tiff each copy-pasted it) — `width`/`height`/`pixels` are
/// first-class fields here, matching `JpgSnapshot`'s own shape.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.jpg")]
pub struct JpgArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub width: u32,
    #[state(artifact)]
    #[value(default)]
    pub height: u32,
    #[state(artifact)]
    #[value(default)]
    pub pixels: Vec<u8>,
}

impl Default for JpgArtifact {
    fn default() -> Self {
        Self::from_snapshot(JpgSnapshot::default())
    }
}

impl JpgArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> JpgSnapshot {
        // 🎪️ `JpgArtifact` is the reduced UI-editable view (schema+raster only) — it never
        // carries frame/table data, so `frame`/`sof_marker`/`arithmetic`/`quant_tables`/
        // `huffman_tables`/etc. fall back to `JpgSnapshot::default()`'s "no decoded frame" state.
        JpgSnapshot { schema: self.schema.clone(), width: self.width, height: self.height, pixels: self.pixels.clone(), ..JpgSnapshot::default() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: JpgSnapshot) -> Self {
        Self { schema: snapshot.schema, width: snapshot.width, height: snapshot.height, pixels: snapshot.pixels }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: JpgSnapshot) {
        self.schema = snapshot.schema;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.pixels = snapshot.pixels;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn jpg_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.jpg",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// `blank_jpg_snapshot`/`demo_jpg_snapshot` relocated here verbatim (pure helpers over the
// document type, destination rule 5); `JpgEngine` (zero construction sites) and the dead
// `register`/`register_pilot_languages`/`register_artifact_inferences`/`register_schema_specs`
// cluster (superseded by `declaration()` in the artifact root, zero real callers) deleted
// outright; the real codec (`encode_jpg`/`decode_jpg`/`JpgError` + every pure format algorithm)
// and `io_registry` moved to `../🚪️io`; tests moved beside what they now test.
/// 🆕️ A new jpg document: one opaque white pixel as the real codec round-trips it — JPEG has no empty image (T.81 §B.2.2:
/// a frame is at least 1×1), and a new document must save and reopen as itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


/// 🧪️ P2-FG2: the demo `JpgSnapshot` used by `conformance_laws::protocol_walk_law`/
/// `fixture_honesty_law` — a real, `encode_jpg`-round-trippable 16x16 image (16x16 = exactly one
/// 4:2:0 MCU, no edge-replication padding needed). Deliberately carries NO `jfif_thumbnail` and NO
/// `other_segments`: `encode_jpg` always canonicalizes fresh Annex K DQT/DHT tables and a fixed
/// 3-component frame regardless of `frame`/`quant_tables`/`huffman_tables`/`sof_marker`/
/// `arithmetic`/`restart_interval` (those fields are decode-only, per the F3b-wave's own
/// documented `EncodeScopeNote`), so this snapshot leaves them at their `Default` values — the
/// thumbnail/other_segments omission specifically sidesteps the two arithmetic-count mechanism
/// gaps `../🚪️io/🦀️.rs`'s own `📡️.protocol.semio` documents (thumbnail-size =
/// width*height*3 needs a two-field product; other_segments' body length needs `Lp - 2`, neither
/// expressible by this dialect's `Field`/`Array` primitives).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_jpg_snapshot() -> JpgSnapshot {
    use crate::JpgSnapshot;
    use crate::STDIO_JPG_DOCUMENT_SCHEMA;
    let (w, h) = (16u32, 16u32);
    let mut pixels = vec![0u8; (w * h * 4) as usize];
    for (i, px) in pixels.chunks_mut(4).enumerate() {
        px[0] = (i * 7 % 255) as u8;
        px[1] = (i * 13 % 255) as u8;
        px[2] = (i * 17 % 255) as u8;
        px[3] = 255;
    }
    JpgSnapshot {
        schema: STDIO_JPG_DOCUMENT_SCHEMA.into(),
        width: w,
        height: h,
        pixels,
        jfif_version: (1, 1),
        jfif_density_units: crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::JfifDensityUnits::PixelsPerInch,
        jfif_x_density: 72,
        jfif_y_density: 72,
        jfif_thumbnail: None,
        other_segments: Vec::new(),
        ..JpgSnapshot::default()
    }
}
//#endregion 🔖️DocumentHelpers
