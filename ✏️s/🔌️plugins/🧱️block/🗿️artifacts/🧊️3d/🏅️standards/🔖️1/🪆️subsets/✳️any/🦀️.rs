//! 🪆️ Subset root for `s.block.block3d@1/*` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME,
//! `descriptor-prep`, following `🔱️trinity`'s `fleet-trinity-recipe`). Exports
//! `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/
//! `📚️examples` children — `crate::editor::block3d`/`crate::viewer::block3d` stay mounted at the
//! plugin's top-level `editor`/`viewer` modules, not here.
//!
//! ⚠️ DEVIATION — see `◻2d`'s sibling subset-root file for the full rationale (identical here):
//! `entries: &[]`, real `native` codec, `io_declaration()` local rather than `io::io()`. Real gap:
//! zip/txt/png/json/stl/obj import+export (12 typed impls) stay unregistered on the `io_mechanism`
//! channel, recommended as dedicated follow-up.

use crate::artifacts::block3d::standards::v1::subsets::any::schema;
use crate::artifacts::block3d::{Block3dMutation, Block3dSnapshot, BLOCK3D_DIALECT, BLOCK_3D_SCHEMA};
use crate::editor::block3d as editor;
use crate::viewer::block3d as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, IoDeclaration, LanguagePair, NativeCodecs, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::art_3d_hexagonal_cut_concrete_forest_left::source(), crate::examples::art_3d_nakagin_capsule::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::block3d_artifact_inference_descriptor()]).as_slice()
}

/// 🚪️ See `◻2d`'s sibling file's module doc for why this is not `io::io()`. `pilot_languages()`
/// indices are fixed by that function's own literal `vec![document, op, diff, pack, spr]` order —
/// the same role→slot mapping `🗒️note`'s `io()` uses for its own five-language array.
fn io_declaration() -> IoDeclaration {
    let langs = crate::artifacts::block3d::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<Block3dSnapshot, Block3dMutation>(BLOCK_3D_SCHEMA.to_string()),
        },
        entries: &[],
    }
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset() -> SubsetDeclaration<crate::BlockApps> {
    SubsetDeclaration {
        dialect: BLOCK3D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::block3d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io_declaration(),
        viewer: viewer_surface::<viewer::Block3dViewer, crate::BlockApps>(viewer::create_block3d_viewer()),
        editor: editor_surface::<editor::Block3dPlayApp, crate::BlockApps>(editor::create_block3d_app()),
        examples: examples(),
    }
}
