//! 🧬️ GifArtifact schema (89a) — full artifact state, mirrors `GifSnapshot`'s frame/GCE/loop model.

use crate::standards::v89a::subsets::any::schema::snapshot::{GifAppExtension, GifColorTable, GifFrame, GifSnapshot};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.89a")]
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
    pub loop_count: Option<u16>,
    #[state(artifact)]
    #[value(default)]
    pub frames: Vec<GifFrame>,
    #[state(artifact)]
    #[value(default)]
    pub comments: Vec<String>,
    #[state(artifact)]
    #[value(default)]
    pub app_extensions: Vec<GifAppExtension>,
}

impl Default for GifArtifact {
    fn default() -> Self {
        Self::from_snapshot(GifSnapshot::default())
    }
}

impl GifArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> GifSnapshot {
        GifSnapshot {
            schema: self.schema.clone(),
            width: self.width,
            height: self.height,
            gct: self.gct.clone(),
            background_color_index: self.background_color_index,
            pixel_aspect_ratio: self.pixel_aspect_ratio,
            loop_count: self.loop_count,
            frames: self.frames.clone(),
            comments: self.comments.clone(),
            app_extensions: self.app_extensions.clone(),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: GifSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            width: snapshot.width,
            height: snapshot.height,
            gct: snapshot.gct,
            background_color_index: snapshot.background_color_index,
            pixel_aspect_ratio: snapshot.pixel_aspect_ratio,
            loop_count: snapshot.loop_count,
            frames: snapshot.frames,
            comments: snapshot.comments,
            app_extensions: snapshot.app_extensions,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: GifSnapshot) {
        self.schema = snapshot.schema;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.gct = snapshot.gct;
        self.background_color_index = snapshot.background_color_index;
        self.pixel_aspect_ratio = snapshot.pixel_aspect_ratio;
        self.loop_count = snapshot.loop_count;
        self.frames = snapshot.frames;
        self.comments = snapshot.comments;
        self.app_extensions = snapshot.app_extensions;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gif_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.gif.89a",
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
// the real GIF89a codec (multi-frame animation, Graphic Control Extension, NETSCAPE2.0 loop —
// reusing 87a's own `pub` byte-level LZW/sub-block/color-table/quantize/interlace helpers
// verbatim) + the protected `register()` cluster (`crate::engine::register()`'s
// own local override explicitly calls BOTH `standards::v87a::engine::register()` AND
// `standards::v89a::engine::register()` — untouched) + `io_registry` all moved to `../🚪️io`;
// tests moved beside what they now test.
/// 🆕️ A new gif (89a) document: a 1×1 logical screen with one frame of one white pixel as the real codec round-trips it —
/// GIF89a has no empty screen nor a frame-less stream, and a new document must save and reopen as itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn blank_gif_snapshot() -> GifSnapshot {
    use crate::standards::v89a::subsets::any::io::{decode_gif, encode_gif};
    use crate::standards::v89a::subsets::any::schema::snapshot::{GifColorTable, GifFrame, GifRgb};
    let white = GifRgb { r: 255, g: 255, b: 255 };
    let seed = GifSnapshot { width: 1, height: 1, gct: Some(GifColorTable { sorted: false, colors: vec![white, white] }), frames: vec![GifFrame { width: 1, height: 1, indices: vec![0], ..GifFrame::default() }], ..GifSnapshot::default() };
    encode_gif(&seed).and_then(|bytes| decode_gif(&bytes)).expect("blank_gif_snapshot: the 1×1 seed round-trips through the real codec")
}

/// 🧪️ P2-FG2: real, deterministic demo `GifSnapshot` for `conformance_laws` (in `../🚪️io`'s own
/// tests) and the shipped `.dsl.semio`/`.pack.semio` fixtures (`../📚️examples/🎬️demo/🖼️assets/`)
/// — per the ticket's own instruction, this reuses the REAL `dancing.gif` fixture
/// (`crate::examples::dancing::decoded_snapshot()`, 54 frames, 800×800,
/// per-frame LCTs, NETSCAPE2.0 loop) decoded via the real 89a codec, for byte-real
/// conformance — not a synthetic stand-in.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_gif_snapshot() -> GifSnapshot {
    crate::examples::dancing::decoded_snapshot()
}
//#endregion 🔖️DocumentHelpers
