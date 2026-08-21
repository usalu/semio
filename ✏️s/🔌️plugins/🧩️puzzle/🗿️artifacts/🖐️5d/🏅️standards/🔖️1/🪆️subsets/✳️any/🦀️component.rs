//! 🪆️ Subset root for `s.puzzle.puzzle5d@1/*` (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
//! RUNTIME`, `terra-descriptors` packet, following the `terra-fleet-trinity-recipe` recipe —
//! `📓️terra-fleet-trinity-recipe-report.md`). Exports `subset() -> SubsetDeclaration`, assembling the
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children — `crate::editor::puzzle5d`/
//! `crate::viewer::puzzle5d` stay mounted at the plugin's top-level `editor`/`viewer` modules
//! (`🗒️note`/`🖍️draw` recipe §5 gotcha 1), not here. `examples` is THREE real fixtures
//! (`crate::examples::puzzle5d::{nakagin_capsule_tower, capsule_dream, concrete_forest}::SOURCE`,
//! each a `LazyLock<ExampleSource>` cloned here) — mounted at the CRATE ROOT
//! (`crate::examples::puzzle5d`), not under `artifacts::puzzle5d` (same shape puzzle2d/puzzle3d's
//! own subset roots hit).
//!
//! ⚠️ DEVIATION from the `🗒️note`/`🖍️draw` template (documented, not an oversight — same shape
//! trinity's own subset-root files carry): those two call `io: io::io()`, delegating to a same-named
//! `io() -> IoDeclaration` function their migration added to `🚪️io/🦀️component.rs`. puzzle5d's own
//! `🚪️io/🦀️component.rs` is still on the OLD `ComposerEntry`/`io_registry` channel, and
//! hand-authoring typed `Deserializer<Puzzle5dSnapshot>`/`Serializer<Puzzle5dSnapshot>` impls for the
//! foreign formats is real, non-trivial migration work outside this packet's descriptor-emission
//! scope. `io_declaration()` below is the same `IoDeclaration` shape, built here instead: `native` is
//! real (reuses `crate::artifacts::puzzle5d::pilot_languages()`'s already-real grammar/protocol
//! pairs, and a real `store::ArtifactCodec::of::<Puzzle5dSnapshot, Puzzle5dMutation>(...)`), but
//! `entries: &[]` — the foreign-format hops stay UNREGISTERED on the new `io_mechanism` channel (an
//! honest gap, not an oversight; `try_build()` still succeeds since an empty batch trivially passes
//! `preflight_io_entries`). Lease-request (mirrors trinity's own): once this artifact's
//! `🚪️io/🦀️component.rs` is migrated to the typed `serializer_entry`/`deserializer_entry` shape,
//! relocate `io_declaration()` there as `io()` (verbatim rename), add the typed leaves, and swap
//! this file's `io: io_declaration()` back to `io: io::io()` to match the template exactly.

use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema;
use crate::artifacts::puzzle5d::{Puzzle5dMutation, Puzzle5dSnapshot, PUZZLE5D_DIALECT, PUZZLE_5D_SCHEMA};
use crate::editor::puzzle5d as editor;
use crate::viewer::puzzle5d as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, IoDeclaration, LanguagePair, NativeCodecs, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

async fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::puzzle5d::nakagin_capsule_tower::SOURCE.clone(), crate::examples::puzzle5d::capsule_dream::SOURCE.clone(), crate::examples::puzzle5d::concrete_forest::SOURCE.clone()]).as_slice()
}

async fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::puzzle5d_artifact_inference_descriptor()]).as_slice()
}

/// 🚪️ See this file's own module doc for why this is not `io::io()`. `pilot_languages()` indices
/// are fixed by that function's own literal `vec![document, op, diff, pack, spr]` order — the same
/// role→slot mapping `🗒️note`'s `io()` uses for its own five-language array.
async fn io_declaration() -> IoDeclaration {
    let langs = crate::artifacts::puzzle5d::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<Puzzle5dSnapshot, Puzzle5dMutation>(PUZZLE_5D_SCHEMA.to_string()),
        },
        entries: &[],
    }
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub async fn subset() -> SubsetDeclaration {
    SubsetDeclaration {
        dialect: PUZZLE5D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::puzzle5d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io_declaration(),
        viewer: viewer_surface::<viewer::Puzzle5dViewer>(viewer::create_puzzle5d_viewer()),
        editor: editor_surface::<editor::Puzzle5dPlayApp>(editor::create_puzzle5d_app()),
        examples: examples(),
    }
}
