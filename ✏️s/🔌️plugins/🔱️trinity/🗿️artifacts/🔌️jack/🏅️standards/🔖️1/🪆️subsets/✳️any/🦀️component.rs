//! 🪆️ Subset root for `s.trinity.jack@1/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM, fleet-trinity-recipe). Exports `subset() -> SubsetDeclaration`, assembling the
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children — `crate::editor::jack`/
//! `crate::viewer::jack` stay mounted at the plugin's top-level `editor`/`viewer` modules
//! (`🗒️note`/`🖍️draw` recipe §5 gotcha 1), not here.
//!
//! ⚠️ DEVIATION from the `🗒️note`/`🖍️draw` template (documented, not an oversight): those two call
//! `io: io::io()`, delegating to a same-named `io() -> IoDeclaration` function the migration itself
//! added to `🚪️io/🦀️component.rs`. This packet (`fleet-trinity-recipe`) was scoped to EXCLUDE every
//! path containing `🚪️io/` — a live peer packet (`io-async-signatures`) is mid-sweep rewriting that
//! exact file this minute. `io_declaration()` below is the same `IoDeclaration` shape, built here
//! instead: `native` is real (reuses `crate::artifacts::jack::pilot_languages()`'s already-real
//! grammar/protocol pairs, unchanged, and a real `store::ArtifactCodec::of::<JackSnapshot,
//! TrinityGraphMutation>(...)`), but `entries: &[]` — the foreign-format hops (svg/csv/md/png/json
//! import+export) stay UNREGISTERED on the new `io_mechanism` channel. Converting them requires
//! hand-authoring `Deserializer<JackSnapshot>`/`Serializer<JackSnapshot>` impls (the
//! `serializer_entry`/`deserializer_entry` typed-trait shape `🗒️note`'s own `🚪️io/🦀️component.rs`
//! uses) to replace the OLD `ComposerEntry`/`deserialize_bytes` machinery still live in this
//! artifact's own `🚪️io/🦀️component.rs` — real migration work that belongs in that excluded file.
//! See `📓️terra-fleet-trinity-recipe-report.md`'s lease-request for the follow-up: once
//! `io-async-signatures` lands, relocate `io_declaration()` into `🚪️io/🦀️component.rs` as `io()`
//! (verbatim rename), add the typed leaves, and swap this file's `io: io_declaration()` back to
//! `io: io::io()` to match the template exactly.

use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::standards::v1::subsets::any::schema;
use crate::artifacts::jack::{JackSnapshot, TRINITY_GRAPH_SCHEMA, TRINITY_JACK_DIALECT};
use crate::editor::jack as editor;
use crate::viewer::jack as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, IoDeclaration, LanguagePair, NativeCodecs, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

async fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::jack::examples::demo::source()]).as_slice()
}

async fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::jack_artifact_inference_descriptor()]).as_slice()
}

/// 🚪️ See this file's own module doc for why this is not `io::io()`. `pilot_languages()` indices
/// are fixed by that function's own literal `vec![document, op, diff, pack, spr]` order — the same
/// role→slot mapping `🗒️note`'s `io()` uses for its own five-language array.
async fn io_declaration() -> IoDeclaration {
    let langs = crate::artifacts::jack::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<JackSnapshot, TrinityGraphMutation>(TRINITY_GRAPH_SCHEMA.to_string()),
        },
        entries: &[],
    }
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub async fn subset() -> SubsetDeclaration {
    SubsetDeclaration {
        dialect: TRINITY_JACK_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::jack_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io_declaration(),
        viewer: viewer_surface::<viewer::TrinityJackViewer>(viewer::create_trinity_jack_viewer()),
        editor: editor_surface::<editor::TrinityJackPlayApp>(editor::create_trinity_jack_app()),
        examples: examples(),
    }
}
