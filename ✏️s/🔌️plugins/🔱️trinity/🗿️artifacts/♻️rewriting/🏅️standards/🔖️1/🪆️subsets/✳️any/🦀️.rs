//! 🪆️ Subset root for `s.trinity.rewriting@1/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM, fleet-trinity-recipe). Exports `subset() -> SubsetDeclaration`, assembling the
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children — `crate::editor::rewriting`/
//! `crate::viewer::rewriting` stay mounted at the plugin's top-level `editor`/`viewer` modules
//! (`🗒️note`/`🖍️draw` recipe §5 gotcha 1), not here.
//!
//! ⚠️ DEVIATION from the `🗒️note`/`🖍️draw` template (documented, not an oversight) — same reasoning
//! as `crate::artifacts::jack::standards::v1::subsets::any`'s own module doc: this packet
//! (`fleet-trinity-recipe`) excludes every path containing `🚪️io/` (a live peer packet,
//! `io-async-signatures`, is mid-sweep rewriting it), so `io_declaration()` below builds the same
//! `IoDeclaration` shape here instead of delegating to a sibling `io::io()`. `native` is real
//! (`crate::artifacts::rewriting::pilot_languages()`'s already-real grammar/protocol pairs plus a real
//! `store::ArtifactCodec::of::<RewritingSnapshot, RewriteRuleMutation>(...)`); `entries: &[]` — the
//! foreign-format hops (txt/pdf/docx/md/json import+export) stay unregistered on the new
//! `io_mechanism` channel pending that file's own migration. See
//! `📓️terra-fleet-trinity-recipe-report.md`'s lease-request.

use crate::artifacts::rewriting::op::RewriteRuleMutation;
use crate::artifacts::rewriting::standards::v1::subsets::any::schema;
use crate::artifacts::rewriting::{RewritingSnapshot, REWRITE_RULE_SCHEMA, TRINITY_REWRITING_DIALECT};
use crate::editor::rewriting as editor;
use crate::viewer::rewriting as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, IoDeclaration, LanguagePair, NativeCodecs, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::rewriting::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::rewriting_artifact_inference_descriptor()]).as_slice()
}

/// 🚪️ See this file's own module doc for why this is not `io::io()`. `pilot_languages()` indices
/// are fixed by that function's own literal `vec![document, op, diff, pack, spr]` order — the same
/// role→slot mapping `🗒️note`'s `io()` uses for its own five-language array.
fn io_declaration() -> IoDeclaration {
    let langs = crate::artifacts::rewriting::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<RewritingSnapshot, RewriteRuleMutation>(REWRITE_RULE_SCHEMA.to_string()),
        },
        entries: &[],
    }
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset() -> SubsetDeclaration<crate::TrinityApps> {
    SubsetDeclaration {
        dialect: TRINITY_REWRITING_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::rewriting_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io_declaration(),
        viewer: viewer_surface::<viewer::TrinityRewritingViewer, crate::TrinityApps>(viewer::create_trinity_rewriting_viewer()),
        editor: editor_surface::<editor::TrinityRewritingPlayApp, crate::TrinityApps>(editor::create_rewriting_app()),
        examples: examples(),
    }
}
