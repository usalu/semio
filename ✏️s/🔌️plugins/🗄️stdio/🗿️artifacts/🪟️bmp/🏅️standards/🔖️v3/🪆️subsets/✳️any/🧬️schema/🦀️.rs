//! 🧬️ BmpArtifact schema — full artifact state.

use crate::BmpSnapshot;
use crate::standards::v_v3::subsets::any::schema::snapshot;
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp")]
pub struct BmpArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub image: snapshot::BmpImage,
}

impl Default for BmpArtifact {
    fn default() -> Self {
        Self::from_snapshot(BmpSnapshot::default())
    }
}

impl BmpArtifact {
    pub fn to_snapshot(&self) -> BmpSnapshot {
        BmpSnapshot { schema: self.schema.clone(), image: self.image.clone() }
    }

    pub fn from_snapshot(snapshot: BmpSnapshot) -> Self {
        Self { schema: snapshot.schema, image: snapshot.image }
    }

    pub fn set_snapshot(&mut self, snapshot: BmpSnapshot) {
        self.schema = snapshot.schema;
        self.image = snapshot.image;
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.bmp`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bmp_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.bmp",
        artifact: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
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
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// `empty_bmp_snapshot`/`demo_bmp_snapshot` relocated here verbatim (pure helpers over the
// document type, destination rule 5); `BmpEngine` (zero construction sites) deleted outright;
// the real codec (`encode_bmp`/`decode_bmp` + every pure format algorithm) + the protected
// `register()` cluster (`crate::engine::register()` is one of stdio's 10
// deliberate imperative plugin-root calls — untouched, reached via this standard's own inline
// `engine` barrel) + `io_registry` all moved to `../🚪️io`; tests moved beside what they now test.
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_bmp_snapshot() -> BmpSnapshot {
    BmpSnapshot::default()
}

/// 🎬️ Authored eight-color BMP native image demo.
pub fn demo_bmp_snapshot() -> BmpSnapshot {
    let image = snapshot::BmpImage { width: 4, height: 2, x_pixels_per_meter: 2835, y_pixels_per_meter: 2835, pixels: snapshot::BmpPixels::Direct { samples: [[255,0,0],[0,255,0],[0,0,255],[255,255,0],[0,255,255],[255,0,255],[255,255,255],[128,128,128]].into_iter().map(|[red,green,blue]| snapshot::BmpNativeSample { red,green,blue,alpha:0,reserved:0 }).collect() }, ..snapshot::BmpImage::default() };
    BmpSnapshot { schema: crate::STDIO_BMP_DOCUMENT_SCHEMA.into(), image }
}
//#endregion 🔖️DocumentHelpers
