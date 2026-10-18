//! 👁️ Deflate viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `DeflateViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<DeflateViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact mutation.
//! Must not import anything from the sibling mutation-capable surface (policy forbids the substring
//! outright, including inside comments).

use crate::viewer::deflate::modes::view;
use crate::viewer::deflate::modes::view::windows::main;
use crate::{DeflateMutation, DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use {semio_framework_artifact_reference::StandardId};
use {semio_framework_artifact_reference::SubsetId};
use semio_framework_plugin::ViewEmit;
use semio_framework_plugin::Viewer;

//#region 🔖️Dialect
/// 🎯️ This surface's dialect coordinate — `s.stdio.deflate@rfc1950/*`. Kept as its own independent
/// const (never imported from the sibling authoring surface) so this file can never reach the
/// mutation-capable module even transitively.
pub const DEFLATE_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DeflateViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for DeflateViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(DeflateViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct DeflateViewer;

impl ArtifactViewer for DeflateViewer {
    type Snapshot = DeflateSnapshot;
    type Mutation = DeflateMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = DeflateViewCommand;

    const DIALECT: Dialect = DEFLATE_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_DEFLATE_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> DeflateSnapshot {
        DeflateSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `DeflateViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit`.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &semio_framework_2d_compute::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_deflate_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(DEFLATE_VIEWER_DIALECT)
        .document(["stdio", "deflate"])
        .icon_id("package")
        .mode_def(view::definition())
        .default_mode_id(view::DEFLATE_VIEW_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧩️ArtifactItems9
pub mod artifact_items_9 {
use crate::{definition};
use crate::{formats};
use crate::{STDIO_DEFLATE_DOCUMENT_SCHEMA};
use crate::{artifact_kind};
pub use crate::schema::mutations::DeflateMutation;
pub use crate::schema::snapshot::DeflateSnapshot;

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g2) —
/// replaces the old side-effecting `crate::engine::register()`, which the
/// plugin root called unconditionally before `PluginBuilder::new(...)` was even constructed. Mirrors
/// `🔋️energy`'s `s.model` exemplar exactly: a headless library artifact with zero `ArtifactApp`s,
/// so `.document_codec_bare::<Snapshot, Mutation>(schema)` stands in for
/// `store::os_store::codec_registry::register_document_codec(store::artifact_codec::<DeflateSnapshot,
/// DeflateMutation>(...))`. `.composers(...)` reaches the SUBSET IO module's own `io_registry`
/// (returns `&'static [ComposerEntry]`, owned rows) by its full path
/// (`standards::v_rfc1950::subsets::any::io::io_registry` — the former `⚙️engine`'s `io_registry`,
/// dissolved there per ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — deliberately
/// NOT this file's own `io_registry` module below, whose `entries()` returns
/// `&'static [&'static ComposerEntry]` (references) and would silently rebind under a bare call
/// (see this ticket's "SILENT REBIND" hazard).
///
/// **NOT covered by any `ArtifactDeclaration` field**: `register_schema_specs()`
/// (`dsl::registry::register_schema_spec`, the P2-M3 `FullResolver` insertion API — a registry
/// distinct from `.languages()`'s `dsl::register_language`), now living beside `io_registry` in
/// `standards::v_rfc1950::subsets::any::io`. No field here closes it, so it is not invented and the
/// call is not dropped — it survives on the plugin root's `.setup(...)`, repointed at this new
/// location by the dissolution.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("deflate", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::schema::deflate_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::schema::inferences::deflate_artifact_inference_descriptor()])
        .composers(crate::standards::v_rfc1950::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<DeflateSnapshot, DeflateMutation>(STDIO_DEFLATE_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.deflate", standard: semio_framework_artifact_reference::StandardId("rfc1950"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built
/// once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, copied
/// verbatim (five `LanguageSpec` rows, one per role) from the former
/// `crate::engine::register_pilot_languages`'s own
/// `dsl::register_language(...)` call bodies — same ids, same grammar/protocol paths, same
/// `passthrough_hooks` calls.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.deflate",
                    extension: Some("zz"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v_rfc1950::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v_rfc1950::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v_rfc1950::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_rfc1950::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.deflate"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.deflate.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v_rfc1950::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v_rfc1950::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v_rfc1950::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_rfc1950::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.deflate.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.deflate.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v_rfc1950::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v_rfc1950::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.deflate.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.deflate.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v_rfc1950::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_rfc1950::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.deflate.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.deflate.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v_rfc1950::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_rfc1950::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.deflate.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems9

//#region 🧩️ArtifactItems10
pub mod artifact_items_10 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use crate::{definition};
use crate::{assembly};
use crate::{formats};
use crate::{native_codecs};

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "deflate", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}
}
//#endregion 🧩️ArtifactItems10
