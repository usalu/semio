//! 🪆️ Subset root for `s.playbook.playbook@1/*` (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
//! RUNTIME`, `terra-descriptors` packet, following the `terra-fleet-trinity-recipe` recipe —
//! `📓️terra-fleet-trinity-recipe-report.md`). Exports `subset() -> SubsetDeclaration`, assembling
//! the `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children — `crate::editor::playbook`/
//! `crate::viewer::playbook` stay mounted at the plugin's top-level `editor`/`viewer` modules
//! (`🗒️note`/`🖍️draw` recipe §5 gotcha 1), not here. `examples` is read via the plugin-root SHIM
//! path `crate::artifacts::playbook::examples::demo` — the deep
//! `standards::v1::subsets::any::examples` path does not resolve for this plugin (this crate's own
//! `📦️glue.rs` only mounts `examples` directly under `artifacts::playbook`, same shape trinity's
//! jack/rewrite hit, not note's).
//!
//! ⚠️ DEVIATION from the `🗒️note`/`🖍️draw` template (documented, not an oversight — same shape
//! trinity's own subset-root files carry): those two call `io: io::io()`, delegating to a same-named
//! `io() -> IoDeclaration` function their migration added to `🚪️io/🦀️component.rs`. Playbook's own
//! `🚪️io/🦀️component.rs` is still on the OLD `ComposerEntry`/`io_registry` channel (six composer
//! rows: `playbook`, `txt`, `pdf`, `docx`, `md`, `json` — see the artifact root's `definition()`),
//! and hand-authoring typed `Deserializer<PlaybookSnapshot>`/`Serializer<PlaybookSnapshot>` impls for
//! all six foreign formats is real, non-trivial migration work outside this packet's descriptor-
//! emission scope. `io_declaration()` below is the same `IoDeclaration` shape, built here instead:
//! `native` is real (reuses `crate::artifacts::playbook::pilot_languages()`'s already-real grammar/
//! protocol pairs, and a real `store::ArtifactCodec::of::<PlaybookSnapshot, PlaybookMutation>(...)`),
//! but `entries: &[]` — the foreign-format hops stay UNREGISTERED on the new `io_mechanism` channel
//! (an honest gap, not an oversight; `try_build()` still succeeds since an empty batch trivially
//! passes `preflight_io_entries`). Lease-request (mirrors trinity's own): once this artifact's
//! `🚪️io/🦀️component.rs` is migrated to the typed `serializer_entry`/`deserializer_entry` shape,
//! relocate `io_declaration()` there as `io()` (verbatim rename), add the six typed leaves, and swap
//! this file's `io: io_declaration()` back to `io: io::io()` to match the template exactly.

use crate::artifacts::playbook::standards::v1::subsets::any::schema;
use crate::artifacts::playbook::{PlaybookMutation, PlaybookSnapshot, PLAYBOOK_DIALECT, PLAYBOOK_DOCUMENT_SCHEMA};
use crate::editor::playbook as editor;
use crate::viewer::playbook as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, IoDeclaration, LanguagePair, NativeCodecs, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::playbook::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::playbook_artifact_inference_descriptor()]).as_slice()
}

/// 🚪️ See this file's own module doc for why this is not `io::io()`. `pilot_languages()` indices
/// are fixed by that function's own literal `vec![document, op, diff, pack, spr]` order — the same
/// role→slot mapping `🗒️note`'s `io()` uses for its own five-language array.
fn io_declaration() -> IoDeclaration {
    let langs = crate::artifacts::playbook::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<PlaybookSnapshot, PlaybookMutation>(PLAYBOOK_DOCUMENT_SCHEMA.to_string()),
        },
        entries: &[],
    }
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset() -> SubsetDeclaration<crate::PlaybookApps> {
    SubsetDeclaration {
        dialect: PLAYBOOK_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::playbook_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io_declaration(),
        viewer: viewer_surface::<viewer::PlaybookViewer, crate::PlaybookApps>(viewer::create_playbook_viewer()),
        editor: editor_surface::<editor::PlaybookPlayApp, crate::PlaybookApps>(editor::create_playbook_play_app()),
        examples: examples(),
    }
}
