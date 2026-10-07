//! 🧬️ GifArtifact schema — full artifact state.

// 🔀️ S-6: `crate::schema` now shims to 89a (canonical) -- 87a's own schema uses
// its own standard-local snapshot type directly rather than the shared root re-export.
use crate::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifImage, GifSnapshot};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif")]
pub struct GifArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    #[value(default)]
    pub gct: Option<GifColorTable>,
    #[state(artifact)]
    #[value(default)]
    pub background_color_index: u8,
    #[state(artifact)]
    #[value(default)]
    pub pixel_aspect_ratio: u8,
    #[state(artifact)]
    #[value(default)]
    pub images: Vec<GifImage>,
}

impl Default for GifArtifact {
    fn default() -> Self {
        Self::from_snapshot(GifSnapshot::default())
    }
}

impl GifArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> GifSnapshot {
        GifSnapshot { schema: self.schema.clone(), width: self.width, height: self.height, gct: self.gct.clone(), background_color_index: self.background_color_index, pixel_aspect_ratio: self.pixel_aspect_ratio, images: self.images.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: GifSnapshot) -> Self {
        Self { schema: snapshot.schema, width: snapshot.width, height: snapshot.height, gct: snapshot.gct, background_color_index: snapshot.background_color_index, pixel_aspect_ratio: snapshot.pixel_aspect_ratio, images: snapshot.images }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: GifSnapshot) {
        self.schema = snapshot.schema;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.gct = snapshot.gct;
        self.background_color_index = snapshot.background_color_index;
        self.pixel_aspect_ratio = snapshot.pixel_aspect_ratio;
        self.images = snapshot.images;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gif_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.gif",
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
// `blank_gif_snapshot`/`demo_gif_snapshot` relocated here verbatim (pure helpers over the
// document type, destination rule 5); `GifEngine` (zero construction sites) deleted outright;
// the real byte-level LZW/sub-block/color-table/quantize/interlace codec (`pub`, reused verbatim
// by 89a's own engine) + `encode_gif`/`decode_gif` + `sniff_magic` + the protected `register()`
// cluster (`crate::engine::register()` is one of stdio's 10 deliberate imperative
// plugin-root calls — untouched, reached via this standard's own inline `engine` barrel) +
// `io_registry` all moved to `../🚪️io`; tests moved beside what they now test.
/// 🆕️ A new gif (87a) document: a 1×1 logical screen with one image of one white pixel as the real codec round-trips it —
/// GIF87a has no empty screen nor an image-less stream, and a new document must save and reopen as itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn blank_gif_snapshot() -> GifSnapshot {
    use crate::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifImage, GifRgb};
    let white = GifRgb { r: 255, g: 255, b: 255 };
    let seed = GifSnapshot { width: 1, height: 1, gct: Some(GifColorTable { sorted: false, colors: vec![white, white] }), images: vec![GifImage { width: 1, height: 1, indices: vec![0], ..GifImage::default() }], ..GifSnapshot::default() };
    seed
}

/// 🧪️ P2-FG2: real, deterministic demo `GifSnapshot` — a real GCT plus two real images (one
/// with its own LCT, exercising every field a genuine encode/decode round-trip touches) — used
/// by `conformance_laws` (in `../🚪️io`'s own tests) and by the shipped `.dsl.semio`/
/// `.pack.semio` fixtures (`../📚️examples/🎬️demo/🖼️assets/`), matching png's own
/// `demo_png_snapshot()` precedent.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_gif_snapshot() -> GifSnapshot {
    use crate::standards::v87a::subsets::any::schema::snapshot::GifRgb;
    let gct = GifColorTable { sorted: false, colors: vec![GifRgb { r: 0, g: 0, b: 0 }, GifRgb { r: 255, g: 255, b: 255 }] };
    let image_a = GifImage { left: 0, top: 0, width: 2, height: 2, interlace: false, lct: None, indices: vec![0, 1, 1, 0] };
    let image_b = GifImage { left: 0, top: 0, width: 2, height: 2, interlace: false, lct: Some(GifColorTable { sorted: true, colors: vec![GifRgb { r: 10, g: 20, b: 30 }, GifRgb { r: 200, g: 100, b: 50 }] }), indices: vec![1, 0, 0, 1] };
    GifSnapshot { schema: crate::STDIO_GIF_DOCUMENT_SCHEMA.into(), width: 2, height: 2, gct: Some(gct), background_color_index: 0, pixel_aspect_ratio: 0, images: vec![image_a, image_b] }
}
//#endregion 🔖️DocumentHelpers
