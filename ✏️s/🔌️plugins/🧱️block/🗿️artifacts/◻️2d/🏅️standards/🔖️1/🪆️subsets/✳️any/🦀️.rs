//! 🪆️ Subset root for `s.block.block2d@1/*` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME,
//! `descriptor-prep`, following `🔱️trinity`'s `fleet-trinity-recipe`). Exports
//! `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/
//! `📚️examples` children — `crate::editor::block2d`/`crate::viewer::block2d` stay mounted at the
//! plugin's top-level `editor`/`viewer` modules, not here.
//!
//! 🚪️ `io: io::io()` matches the `🗒️note`/`🖍️draw`/`🔱️trinity` template exactly: the local
//! `io_declaration()` this file used to carry (with `entries: &[]` and a DEVIATION note explaining
//! that the six foreign formats stayed unregistered on the `io_mechanism` channel) is gone — ticket
//! 26/09/05/BLOCK-PLUGIN-END-TO-END, W3 hand-authored the twelve typed
//! `Serializer<Block2dSnapshot>`/`Deserializer<Block2dSnapshot>` leaves that gap called for and relocated the
//! declaration into `🚪️io/🦀️.rs` as `io()`. See that file's own module doc for the per-format
//! fidelity table.

use crate::artifacts::block2d::standards::v1::subsets::any::{io, schema};
use crate::artifacts::block2d::BLOCK2D_DIALECT;
use crate::editor::block2d as editor;
use crate::viewer::block2d as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
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

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset() -> SubsetDeclaration<crate::BlockApps> {
    SubsetDeclaration {
        dialect: BLOCK2D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::block2d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::Block2dViewer, crate::BlockApps>(viewer::create_block2d_viewer()),
        editor: editor_surface::<editor::Block2dPlayApp, crate::BlockApps>(editor::create_block2d_app()),
        examples: examples(),
    }
}
