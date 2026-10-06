//! 🧬️ TiffArtifact schema — full artifact state (mirrors `TiffSnapshot` field-for-field; see
//! `png_artifact_schema_descriptor`/`PngArtifact` for the established repo pattern this follows).

use crate::schema::snapshot::{TiffByteOrder, TiffIfd};
use crate::TiffSnapshot;
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tiff")]
pub struct TiffArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub byte_order: TiffByteOrder,
    #[state(artifact)]
    #[value(default)]
    pub ifds: Vec<TiffIfd>,
}

impl Default for TiffArtifact {
    fn default() -> Self {
        Self::from_snapshot(TiffSnapshot::default())
    }
}

impl TiffArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> TiffSnapshot {
        TiffSnapshot { schema: self.schema.clone(), byte_order: self.byte_order, ifds: self.ifds.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: TiffSnapshot) -> Self {
        Self { schema: snapshot.schema, byte_order: snapshot.byte_order, ifds: snapshot.ifds }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: TiffSnapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tiff_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.tiff",
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
// `blank_tiff_snapshot`/`demo_tiff_snapshot` relocated here verbatim (pure helpers over the
// document type, destination rule 5); `TiffEngine` (zero construction sites) and the dead
// `register`/`register_pilot_languages`/`register_artifact_inferences` cluster (superseded by
// `declaration()` in the artifact root, zero real callers) deleted outright; the real codec
// (`encode_tiff`/`encode_tiff_packbits`/`decode_tiff` + every pure format algorithm) and
// `io_registry` moved to `../🚪️io`; tests moved beside what they now test.
/// 🆕️ A new tiff document: one opaque white pixel in one IFD as the real codec round-trips it — baseline TIFF has no
/// image without `ImageWidth`/`ImageLength` (TIFF 6.0 §8), and a new document must save and reopen as itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn blank_tiff_snapshot() -> TiffSnapshot {
    use crate::standards::v6_0::subsets::document::schema::snapshot::*;
    TiffSnapshot { ifds: vec![TiffIfd { entries: vec![
        TiffTag { tag: TAG_IMAGE_WIDTH, values: TiffValues::Long(vec![1]) },TiffTag { tag: TAG_IMAGE_LENGTH, values: TiffValues::Long(vec![1]) },TiffTag { tag: TAG_BITS_PER_SAMPLE, values: TiffValues::Short(vec![8,8,8]) },TiffTag { tag: TAG_COMPRESSION, values: TiffValues::Short(vec![1]) },TiffTag { tag: TAG_PHOTOMETRIC, values: TiffValues::Short(vec![2]) },TiffTag { tag: TAG_SAMPLES_PER_PIXEL, values: TiffValues::Short(vec![3]) },TiffTag { tag: TAG_ROWS_PER_STRIP, values: TiffValues::Long(vec![1]) },
    ], storage: TiffStorage { kind:TiffStorageKind::Strips,offsets_kind:TiffFieldType::Long,byte_counts_kind:TiffFieldType::Long,chunks:vec![vec![255,255,255]] } }], ..TiffSnapshot::default() }
}

/// 📄️ P2-FG2: the demo `stdio.tiff` document — a genuinely non-trivial `TiffSnapshot` exercising
/// a non-solid checkerboard raster plus one carried non-core tag (`Artist`, 315). The single
/// source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` (`fixture_honesty_law`
/// in `../🚪️io`'s own tests asserts they're literally this snapshot's `print_dsl` output).
///
/// **Deliberately built via a real `encode_tiff`/`decode_tiff` round trip**, not hand-assembled
/// field values: `encode_tiff` always CANONICALIZES the core strip/geometry tags fresh from
/// `pixels` (see `encode_tiff_with`'s own `MultiIfdEncodeScopeNote`) — hand-picking `ImageWidth`/
/// `BitsPerSample`/`Compression`/`PhotometricInterpretation`/`SamplesPerPixel`/`RowsPerStrip`/
/// `StripByteCounts`/`StripOffsets` values here would silently "self-correct" on the very first
/// `print_dsl`/`parse_dsl` round trip and break `fixture_honesty_law`'s `parse_dsl(fixture) ==
/// demo()` identity (same class of trap `png`'s own `demo_png_snapshot()` doc comment documents
/// for its IHDR fields) — running the real codec once here guarantees `demo()` is ALREADY in
/// exactly the canonical shape a second `encode_tiff`/`decode_tiff` pass reproduces byte-for-byte.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_tiff_snapshot() -> TiffSnapshot {
    use crate::standards::v6_0::subsets::document::schema::snapshot::*;
    let (width,height)=(3u32,2u32);let mut rgb=Vec::with_capacity((width*height*3)as usize);for y in 0..height{for x in 0..width{let checker=if(x+y)%2==0{255}else{0};rgb.extend_from_slice(&[checker,((x*37)%256)as u8,((y*53)%256)as u8]);}}
    TiffSnapshot{schema:crate::STDIO_TIFF_DOCUMENT_SCHEMA.into(),byte_order:TiffByteOrder::LittleEndian,ifds:vec![TiffIfd{entries:vec![
        TiffTag{tag:TAG_IMAGE_WIDTH,values:TiffValues::Long(vec![width])},TiffTag{tag:TAG_IMAGE_LENGTH,values:TiffValues::Long(vec![height])},TiffTag{tag:TAG_BITS_PER_SAMPLE,values:TiffValues::Short(vec![8,8,8])},TiffTag{tag:TAG_COMPRESSION,values:TiffValues::Short(vec![1])},TiffTag{tag:TAG_PHOTOMETRIC,values:TiffValues::Short(vec![2])},TiffTag{tag:TAG_SAMPLES_PER_PIXEL,values:TiffValues::Short(vec![3])},TiffTag{tag:TAG_ROWS_PER_STRIP,values:TiffValues::Long(vec![height])},TiffTag{tag:315,values:TiffValues::Ascii(b"stdio.tiff demo\0".to_vec())},
    ],storage:TiffStorage{kind:TiffStorageKind::Strips,offsets_kind:TiffFieldType::Long,byte_counts_kind:TiffFieldType::Long,chunks:vec![rgb]}}]}
}
//#endregion 🔖️DocumentHelpers
