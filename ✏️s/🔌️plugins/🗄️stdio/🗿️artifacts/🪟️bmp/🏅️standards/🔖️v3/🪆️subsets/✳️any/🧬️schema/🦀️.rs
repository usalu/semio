//! 🧬️ BmpArtifact schema — full artifact state.

use crate::BmpSnapshot;
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp")]
pub struct BmpArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub bytes: Vec<u8>,
}

impl Default for BmpArtifact {
    fn default() -> Self {
        Self::from_snapshot(BmpSnapshot::default())
    }
}

impl BmpArtifact {
    pub fn to_snapshot(&self) -> BmpSnapshot {
        BmpSnapshot { schema: self.schema.clone(), bytes: self.bytes.clone() }
    }

    pub fn from_snapshot(snapshot: BmpSnapshot) -> Self {
        Self { schema: snapshot.schema, bytes: snapshot.bytes }
    }

    pub fn set_snapshot(&mut self, snapshot: BmpSnapshot) {
        self.schema = snapshot.schema;
        self.bytes = snapshot.bytes;
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

/// 🎬 P2-FG2: canonical demo snapshot — the same value the real `.dsl.semio`/`.pack.semio`
/// fixtures under `📚️examples/🎬️demo/🖼️assets/` are genuine `print_dsl`/`encode_pack` output
/// of (regenerated this wave via a real `encode_bmp`/`print_dsl`/`encode_pack` call, replacing
/// the pre-existing fake "hello" placeholder text). 4x2 24-bit `BI_RGB`, bottom-up, 8 distinct
/// non-solid RGBA pixels (`row_bytes(4, 24) == 12`, already a multiple of 4, so this fixture
/// does NOT exercise row padding — `gradient_checkerboard_24bit_round_trip`'s own 6-wide fixture
/// in `../🚪️io`'s own tests already covers that) — `header_size`/`planes`/`bits_per_pixel`/
/// `compression` are exactly what `encode_bmp` always hardcodes (40/1/24/0, see its own
/// `EncodeScopeNote`), so this snapshot is safe against `encode_bmp`'s own canonicalization (any
/// other value here would silently "self-correct" on the first decode and break
/// `fixture_honesty_law`'s `parse_dsl(fixture) == demo()` identity). No palette (bpp=24 has none).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_bmp_snapshot() -> BmpSnapshot {
    BmpSnapshot { schema: crate::STDIO_BMP_DOCUMENT_SCHEMA.into(), bytes: crate::standards::v_v3::subsets::any::io::demo_bmp_bytes() }
}
//#endregion 🔖️DocumentHelpers
