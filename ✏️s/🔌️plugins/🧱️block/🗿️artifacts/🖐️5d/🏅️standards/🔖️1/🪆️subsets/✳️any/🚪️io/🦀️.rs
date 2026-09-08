//! 🚪️ IO `s.block.block5d` (1/✳️any) — `io() -> IoDeclaration`: this subset's native codecs plus every
//! foreign hop, aggregated from the typed `Serializer<Block5dSnapshot>`/`Deserializer<Block5dSnapshot>`
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
//! | `s.stdio.zip@2.0/*` | both | `Exact` | a real zip 2.0 container: `snapshot.block5d.semio` + `snapshot.json` |
//! | `s.stdio.stl@ascii/*` | both | `Lossy` | typed `Err` — the schema carries no triangle geometry |
//! | `s.stdio.obj@3.0/*` | both | `Lossy` | typed `Err` — the schema carries no vertex/face geometry |
//! | `s.stdio.png@1.2/*` | both | `Lossy` | typed `Err` — no raster in the schema, no rasterizer here |
//!
//! The three refusing hops stay registered on purpose: an unregistered hop yields a bare "no route",
//! a registered one hands the caller the actual reason.

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::schema::Block5dAnalyzer;
    use crate::Block5dSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.block.block5d", standard: StandardId("1"), subset: SubsetId("*") };

    /// 🎹️ The `ArtifactComposition` facet `derive_artifact_facets!` binds in `🧬️schema/🦀️.rs`.
    /// Native-only by design: foreign dialects are the `io_mechanism` entries in `io()` below, not
    /// composer sources.
    pub struct Block5dComposerComposition;

    impl ArtifactComposition for Block5dComposerComposition {
        type Snapshot = Block5dSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = Block5dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "Block5dComposerComposition: no source in this artifact's own dialect".into(), diagnostics: Vec::new() })
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
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::{Block5dMutation, Block5dSnapshot, BLOCK5D_DIALECT, BLOCK_5D_SCHEMA};
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
                    serializer_entry::<Block5dSnapshot, export::txt::v_utf_8::any::Block5dIntoTxt>(BLOCK5D_DIALECT),
                    deserializer_entry::<Block5dSnapshot, import::txt::v_utf_8::any::TxtIntoBlock5d>(BLOCK5D_DIALECT),
                    serializer_entry::<Block5dSnapshot, export::json::v_rfc8259::any::Block5dIntoJson>(BLOCK5D_DIALECT),
                    deserializer_entry::<Block5dSnapshot, import::json::v_rfc8259::any::JsonIntoBlock5d>(BLOCK5D_DIALECT),
                    serializer_entry::<Block5dSnapshot, export::zip::v2_0::any::Block5dIntoZip>(BLOCK5D_DIALECT),
                    deserializer_entry::<Block5dSnapshot, import::zip::v2_0::any::ZipIntoBlock5d>(BLOCK5D_DIALECT),
                    serializer_entry::<Block5dSnapshot, export::stl::v_ascii::any::Block5dIntoStl>(BLOCK5D_DIALECT),
                    deserializer_entry::<Block5dSnapshot, import::stl::v_ascii::any::StlIntoBlock5d>(BLOCK5D_DIALECT),
                    serializer_entry::<Block5dSnapshot, export::obj::v3_0::any::Block5dIntoObj>(BLOCK5D_DIALECT),
                    deserializer_entry::<Block5dSnapshot, import::obj::v3_0::any::ObjIntoBlock5d>(BLOCK5D_DIALECT),
                    serializer_entry::<Block5dSnapshot, export::png::v1_2::any::Block5dIntoPng>(BLOCK5D_DIALECT),
                    deserializer_entry::<Block5dSnapshot, import::png::v1_2::any::PngIntoBlock5d>(BLOCK5D_DIALECT),
                ]
            })
            .as_slice()
    }

    let langs = crate::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<Block5dSnapshot, Block5dMutation>(BLOCK_5D_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
