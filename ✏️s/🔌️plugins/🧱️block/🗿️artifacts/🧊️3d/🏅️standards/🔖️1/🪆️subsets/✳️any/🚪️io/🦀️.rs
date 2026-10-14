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
//!
//! No stl, obj or png hop is declared: a node kind definition carries no geometry and no raster, so
//! those formats are not claimed at all (a declared hop that can only refuse is a stub).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🔖️IoDeclaration
/// 🚪️ This subset's complete io declaration — the native `LanguagePair`s and `ArtifactCodec` plus
/// the twelve typed foreign entries. `pilot_languages()` indices are fixed by that function's own
/// literal `vec![document, op, diff, pack, spr]` order, the same role→slot mapping `🗒️note`'s
/// `io()` uses.
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use semio_s_artifact_block_3d_io_write::serializers::artifacts as export;
    use semio_s_artifact_block_3d_io_read::deserializers::artifacts as import;
    use crate::{Block3dMutation, Block3dSnapshot, BLOCK3D_DIALECT, BLOCK_3D_SCHEMA};
    use semio_framework_os_kernel::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
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
            codec: crate::store::bare_artifact_codec::<Block3dSnapshot, Block3dMutation>(BLOCK_3D_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration

semio_framework_plugin::derive_artifact_facets!(
    pub spec Block3dBuilderFacets {
        construction: Block3dBuilderConstruction,
        analysis: Block3dAnalyzerAnalysis,
        composition: Block3dComposerComposition,
    }
    builder: Block3dBuilder,
    analyzer: Block3dAnalyzer,
    composer: Block3dComposer,
);

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. `pub` (was
/// private): the new `🪆️subsets/✳️any/🦀️.rs` reads it to build `io_declaration()`'s native
/// codec pairs.
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "block.block3d",
                    extension: Some("block3d"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("block.block3d"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "block.block3d.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("block.block3d.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "block.block3d.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("block.block3d.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "3d.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("3d.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "3d.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("3d.spr"),
                },
            ]
        })
        .as_slice()
}

//#region 🪢️Mounts
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🪢️Mounts
