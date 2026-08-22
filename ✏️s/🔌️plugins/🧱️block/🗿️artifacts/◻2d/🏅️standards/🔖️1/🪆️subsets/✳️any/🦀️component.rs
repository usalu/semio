//! 🪆️ Subset root for `s.block.block2d@1/*` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME,
//! `descriptor-prep`, following `🔱️trinity`'s `fleet-trinity-recipe`). Exports
//! `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/
//! `📚️examples` children — `crate::editor::block2d`/`crate::viewer::block2d` stay mounted at the
//! plugin's top-level `editor`/`viewer` modules, not here.
//!
//! ⚠️ DEVIATION from the `🗒️note`/`🖍️draw`/`🔱️trinity` template (documented, not an oversight):
//! those call `io: io::io()`, delegating to a same-named `io() -> IoDeclaration` function their own
//! migration added to `🚪️io/🦀️component.rs`. Unlike trinity, `🧱️block/🚪️io/` is NOT excluded from
//! this packet's ownership — the gap here is a deliberate scope decision (`descriptor-prep` bounds
//! itself to the `.declare_artifact` capability-claim migration, not a from-scratch rewrite of the
//! old `ArtifactComposition`/`io_registry` mechanism into typed `Serializer`/`Deserializer` impls),
//! not a boundary constraint. `io_declaration()` below carries the same `IoDeclaration` shape as the
//! template: `native` is real (reuses `crate::artifacts::block2d::pilot_languages()`'s already-real
//! grammar/protocol pairs, unchanged, and a real `store::ArtifactCodec::of::<Block2dSnapshot,
//! Block2dMutation>(...)`), but `entries: &[]` — the six foreign-format hops (zip/txt/png/json/stl/obj
//! import+export, both directions) stay UNREGISTERED on the new `io_mechanism` channel. Converting
//! them requires hand-authoring `Deserializer<Block2dSnapshot>`/`Serializer<Block2dSnapshot>` impls
//! (the `serializer_entry`/`deserializer_entry` typed-trait shape `🗒️note`'s own
//! `🚪️io/🦀️component.rs` uses) per format leaf, replacing the OLD `ComposerEntry`/`serialize_bytes`/
//! `deserialize_bytes` free-fn machinery still live in this artifact's own `🚪️io/🦀️component.rs` and
//! its `📥️import/🧩️deserializers`/`📤️export/🧵️serializers` leaves — real, non-trivial migration work
//! (12 typed impls for this artifact alone) recommended as dedicated follow-up, not attempted here.

use crate::artifacts::block2d::standards::v1::subsets::any::schema;
use crate::artifacts::block2d::{Block2dMutation, Block2dSnapshot, BLOCK2D_DIALECT, BLOCK_2D_SCHEMA};
use crate::editor::block2d as editor;
use crate::viewer::block2d as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, IoDeclaration, LanguagePair, NativeCodecs, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::art_2d_hexagonal_cut_concrete_forest_left::source(), crate::examples::art_2d_hexagonal_cut_concrete_forest_right::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::block2d_artifact_inference_descriptor()]).as_slice()
}

/// 🚪️ See this file's own module doc for why this is not `io::io()`. `pilot_languages()` indices
/// are fixed by that function's own literal `vec![document, op, diff, pack, spr]` order — the same
/// role→slot mapping `🗒️note`'s `io()` uses for its own five-language array.
fn io_declaration() -> IoDeclaration {
    let langs = crate::artifacts::block2d::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<Block2dSnapshot, Block2dMutation>(BLOCK_2D_SCHEMA.to_string()),
        },
        entries: &[],
    }
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset() -> SubsetDeclaration<crate::BlockApps> {
    SubsetDeclaration {
        dialect: BLOCK2D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::block2d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io_declaration(),
        viewer: viewer_surface::<viewer::Block2dViewer, crate::BlockApps>(viewer::create_block2d_viewer()),
        editor: editor_surface::<editor::Block2dPlayApp, crate::BlockApps>(editor::create_block2d_app()),
        examples: examples(),
    }
}
