//! 🚪️ IO `s.block.block3d` (1/✳️any) — `io() -> IoDeclaration`: this subset's native codecs plus every
//! foreign hop, aggregated from the typed `Serializer<Block3dSnapshot>`/`Deserializer<Block3dSnapshot>`
//! leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`. Foreign io goes EXCLUSIVELY
//! through the framework's `io_mechanism` registry, reached from the sibling `🪆️subsets/✳️any/🦀️.rs`'s
//! `io: io::io()` — the same wiring `🗒️note`/`🖍️draw` use.
//!
//! The OLD `ComposerEntry`/`io_registry` export channel this file used to carry is DELETED, not
//! shimmed (ticket 26/09/05/BLOCK-PLUGIN-END-TO-END, W3): nothing in the repo ever called its
//! `entries()`, and the `serialize_bytes` free functions it dispatched to handed back this subset's
//! DSL text mislabelled as zip/png/stl/obj bytes. `import_stdio_kinds()`/`export_stdio_kinds()` went
//! with it — the live lists are `artifact_kind()`'s own fields in the artifact root.
//!
//! `derived_composition` below STAYS and is now native-only: it is the `ArtifactComposition` facet
//! `semio_framework_plugin::derive_artifact_facets!` requires in the sibling `🧬️schema/🦀️.rs`, and
//! every foreign-format branch it used to carry now lives in a typed leaf instead.
//!
//! Format coverage — IDENTICAL in `◻️2d`, `🧊️3d` and `🖐️5d` (full decision table in the ticket's
//! `📓️w3-io.md`):
//!
//! | foreign dialect | direction | fidelity | behaviour |
//! |---|---|---|---|
//! | `s.stdio.txt@utf-8/*` | both | `Exact` | this subset's own `.semio` DSL snapshot text — the exact bytes `📚️examples/**/🗣️.dsl.semio` carry |
//! | `s.stdio.json@rfc8259/*` | both | `Exact` | the `dsl::ToValue` record tree as compact rfc8259 |
//! | `s.stdio.zip@2.0/*` | both | `Exact` | a real zip 2.0 container: `snapshot.block3d.semio` + `snapshot.json` |
//! | `s.stdio.stl@ascii/*` | both | `Lossy` | typed `Err` — the schema carries no triangle geometry |
//! | `s.stdio.obj@3.0/*` | both | `Lossy` | typed `Err` — the schema carries no vertex/face geometry |
//! | `s.stdio.png@1.2/*` | both | `Lossy` | typed `Err` — no raster in the schema, no rasterizer here |
//!
//! The three refusing hops stay registered on purpose: an unregistered hop yields a bare "no route",
//! a registered one hands the caller the actual reason.

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::block3d::standards::v1::subsets::any::schema::Block3dAnalyzer;
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.block.block3d", standard: StandardId("1"), subset: SubsetId("*") };

    /// 🎹️ The `ArtifactComposition` facet `derive_artifact_facets!` binds in `🧬️schema/🦀️.rs`.
    /// Native-only by design: foreign dialects are the `io_mechanism` entries in `io()` below, not
    /// composer sources.
    pub struct Block3dComposerComposition;

    impl ArtifactComposition for Block3dComposerComposition {
        type Snapshot = Block3dSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = Block3dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "Block3dComposerComposition: no source in this artifact's own dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️IoDeclaration
/// 🚪️ This subset's complete io declaration — the native `LanguagePair`s and `ArtifactCodec` plus
/// the twelve typed foreign entries. `pilot_languages()` indices are fixed by that function's own
/// literal `vec![document, op, diff, pack, spr]` order, the same role→slot mapping `🗒️note`'s
/// `io()` uses.
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::artifacts::block3d::io::export::serializers::artifacts as export;
    use crate::artifacts::block3d::io::import::deserializers::artifacts as import;
    use crate::artifacts::block3d::{Block3dMutation, Block3dSnapshot, BLOCK3D_DIALECT, BLOCK_3D_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    /// 🎹️ One vtable row per typed leaf, both directions per format — the single place this subset
    /// advertises what it can and cannot convert.
    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<Block3dSnapshot, export::txt::v_utf_8::any::Block3dIntoTxt>(BLOCK3D_DIALECT),
                    deserializer_entry::<Block3dSnapshot, import::txt::v_utf_8::any::TxtIntoBlock3d>(BLOCK3D_DIALECT),
                    serializer_entry::<Block3dSnapshot, export::json::v_rfc8259::any::Block3dIntoJson>(BLOCK3D_DIALECT),
                    deserializer_entry::<Block3dSnapshot, import::json::v_rfc8259::any::JsonIntoBlock3d>(BLOCK3D_DIALECT),
                    serializer_entry::<Block3dSnapshot, export::zip::v2_0::any::Block3dIntoZip>(BLOCK3D_DIALECT),
                    deserializer_entry::<Block3dSnapshot, import::zip::v2_0::any::ZipIntoBlock3d>(BLOCK3D_DIALECT),
                    serializer_entry::<Block3dSnapshot, export::stl::v_ascii::any::Block3dIntoStl>(BLOCK3D_DIALECT),
                    deserializer_entry::<Block3dSnapshot, import::stl::v_ascii::any::StlIntoBlock3d>(BLOCK3D_DIALECT),
                    serializer_entry::<Block3dSnapshot, export::obj::v3_0::any::Block3dIntoObj>(BLOCK3D_DIALECT),
                    deserializer_entry::<Block3dSnapshot, import::obj::v3_0::any::ObjIntoBlock3d>(BLOCK3D_DIALECT),
                    serializer_entry::<Block3dSnapshot, export::png::v1_2::any::Block3dIntoPng>(BLOCK3D_DIALECT),
                    deserializer_entry::<Block3dSnapshot, import::png::v1_2::any::PngIntoBlock3d>(BLOCK3D_DIALECT),
                ]
            })
            .as_slice()
    }

    let langs = crate::artifacts::block3d::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<Block3dSnapshot, Block3dMutation>(BLOCK_3D_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::artifacts::block3d::io::export::serializers::artifacts::json::v_rfc8259::any::json_text;
    use crate::artifacts::block3d::io::export::serializers::artifacts::obj::v3_0::any::Block3dIntoObj;
    use crate::artifacts::block3d::io::export::serializers::artifacts::png::v1_2::any::Block3dIntoPng;
    use crate::artifacts::block3d::io::export::serializers::artifacts::stl::v_ascii::any::Block3dIntoStl;
    use crate::artifacts::block3d::io::export::serializers::artifacts::txt::v_utf_8::any::dsl_text;
    use crate::artifacts::block3d::io::export::serializers::artifacts::zip::v2_0::any::Block3dIntoZip;
    use crate::artifacts::block3d::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text;
    use crate::artifacts::block3d::io::import::deserializers::artifacts::obj::v3_0::any::ObjIntoBlock3d;
    use crate::artifacts::block3d::io::import::deserializers::artifacts::png::v1_2::any::PngIntoBlock3d;
    use crate::artifacts::block3d::io::import::deserializers::artifacts::stl::v_ascii::any::StlIntoBlock3d;
    use crate::artifacts::block3d::io::import::deserializers::artifacts::txt::v_utf_8::any::from_dsl_text;
    use crate::artifacts::block3d::io::import::deserializers::artifacts::zip::v2_0::any::{from_zip_bytes, ZIP_MAGIC};
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework::io::io_mechanism::{Deserializer, Serializer};
    use semio_framework::io_schema::IoPayload;

    /// 📄️ Every handcrafted `.semio` DSL example asset of this subset — the language-agnostic
    /// fixtures the TypeScript mirror's own test reads too.
    const EXAMPLES: &[(&str, &str)] = &[
        ("hexagonal-cut-concrete-forest-left", include_str!("../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio")),
        ("nakagin-capsule", include_str!("../📚️examples/🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule/🗣️.dsl.semio")),
    ];

    #[semio_framework_async_macros::async_test]
    async fn txt_round_trips_every_example() {
        for (id, text) in EXAMPLES {
            let snapshot = from_dsl_text(text).unwrap_or_else(|error| panic!("{id}: {error:?}"));
            let printed = dsl_text(&snapshot);
            assert_eq!(printed.trim_end_matches('\n'), text.trim_end_matches('\n'), "{id}: txt export must reproduce the example asset");
            assert_eq!(from_dsl_text(&printed).unwrap(), snapshot, "{id}: txt is not a fixed point");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn json_round_trips_every_example() {
        for (id, text) in EXAMPLES {
            let snapshot = from_dsl_text(text).unwrap_or_else(|error| panic!("{id}: {error:?}"));
            let json = json_text(&snapshot);
            assert_eq!(from_json_text(&json).unwrap_or_else(|error| panic!("{id}: {error:?}")), snapshot, "{id}: json is not a lossless round trip");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn zip_round_trips_every_example_as_a_real_archive() {
        for (id, text) in EXAMPLES {
            let snapshot = from_dsl_text(text).unwrap_or_else(|error| panic!("{id}: {error:?}"));
            let IoPayload::Binary(bytes) = Block3dIntoZip::serialize(&snapshot).await.unwrap().value else {
                panic!("{id}: zip export must be a binary payload");
            };
            assert!(bytes.starts_with(ZIP_MAGIC), "{id}: zip export must be a real zip 2.0 container");
            assert_eq!(from_zip_bytes(&bytes).unwrap_or_else(|error| panic!("{id}: {error:?}")), snapshot, "{id}: zip is not a lossless round trip");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn geometry_and_raster_hops_refuse_with_a_reason() {
        let snapshot = Block3dSnapshot::default();
        let empty_text = IoPayload::Text(String::new());
        let empty_binary = IoPayload::Binary(Vec::new());
        for message in [
            Block3dIntoStl::serialize(&snapshot).await.expect_err("stl export must refuse").message,
            Block3dIntoObj::serialize(&snapshot).await.expect_err("obj export must refuse").message,
            Block3dIntoPng::serialize(&snapshot).await.expect_err("png export must refuse").message,
            StlIntoBlock3d::deserialize(&empty_text).await.expect_err("stl import must refuse").message,
            ObjIntoBlock3d::deserialize(&empty_text).await.expect_err("obj import must refuse").message,
            PngIntoBlock3d::deserialize(&empty_binary).await.expect_err("png import must refuse").message,
        ] {
            assert!(message.contains("not supported for"), "every refusing hop must name the reason, got: {message}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn io_declaration_registers_both_directions_of_all_six_formats() {
        let declaration = super::io();
        assert_eq!(declaration.entries.len(), 12, "six formats x two directions");
        let own = "s.block.block3d";
        let mut foreign: Vec<&str> = Vec::new();
        for entry in declaration.entries {
            assert!(entry.from.artifact_kind == own || entry.into.artifact_kind == own, "every entry must touch this subset's own dialect");
            foreign.push(if entry.from.artifact_kind == own { entry.into.artifact_kind } else { entry.from.artifact_kind });
        }
        foreign.sort_unstable();
        foreign.dedup();
        assert_eq!(foreign, vec!["s.stdio.json", "s.stdio.obj", "s.stdio.png", "s.stdio.stl", "s.stdio.txt", "s.stdio.zip"]);
    }
}
//#endregion 🧪️Tests
