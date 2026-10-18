//! 👁️ Xml viewer — the read-only counterpart of `✏️editor` for `s.stdio.xml@1.0/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `XmlAnyViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<XmlAnyViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact mutation. Must
//! not import anything from the sibling mutation-capable surface.

use crate::viewer::xml_any::modes::view;
use crate::viewer::xml_any::modes::view::windows::main;
use crate::{XmlMutation, XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
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
/// 🪪️ Same coordinate as the sibling editor surface's own `XML_EDITOR_DIALECT` — duplicated here
/// on purpose (never imported through the editor module).
pub const XML_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum XmlViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for XmlViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(XmlViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct XmlAnyViewer;

impl ArtifactViewer for XmlAnyViewer {
    type Snapshot = XmlSnapshot;
    type Mutation = XmlMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = XmlViewCommand;

    const DIALECT: Dialect = XML_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XML_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> XmlSnapshot {
        XmlSnapshot::with_empty_root()
    }

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

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_xml_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(XML_VIEWER_DIALECT)
        .document(["semio", "stdio", "xml"])
        .icon_id("list-tree")
        .mode_def(view::definition())
        .default_mode_id(view::XML_VIEW_MODE_ID)
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
use crate::{STDIO_XML_DOCUMENT_SCHEMA};
pub use crate::schema::mutations::XmlMutation;
pub use crate::schema::snapshot::XmlSnapshot;

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) — replaces
/// stdio's plugin root calling `crate::engine::register()` imperatively before
/// `Plugin::builder` was even constructed, mirroring the `🔋️energy`/`🗒️note` exemplars and this
/// artifact's own `🔣️json` sibling (same shape: an `✳️any` + one dependent subset). `crate::
/// artifacts::xml::standards::v1_0::subsets::base::engine::register()` (stdio's own `⚙️engine` —
/// UNTOUCHED, per this ticket's rule that stdio's engines stay a public surface other plugins reach
/// into) called, in call order: `io_registry::register()` → `.composers(...)` below, the same
/// `standards::v1_0::subsets::any::engine::io_registry::entries()` this artifact's own root
/// `io_registry` module already wraps — that list already carries BOTH the `✳️any` raw composer and
/// the `✳️valid` composer, so `.composers()` alone covers both; `register_artifact_schema()`/
/// `register_artifact_inferences()` → `.schema(...)`/`.inferences(...)`; `register_pilot_languages()`
/// → `.languages(...)`, replicated verbatim below (same `OnceLock`-leak shape `🔋️energy`'s own
/// `pilot_languages()` uses, since `dsl::LanguageSpec` isn't `const fn`-constructible);
/// `register_document_codec` → `.document_codec_bare::<XmlSnapshot, XmlMutation>(...)`; and
/// `crate::standards::v1_0::subsets::valid::io::register()` — the ✳️valid subset's
/// own `register_subset_validator` call, living in `🚪️io/` (not `⚙️engine/`, so freely editable) →
/// `.subset_validators(...)` below, built fresh via
/// `subset_validator_entry_of::<XmlValidValidator>()` rather than reaching into that module's private
/// `validator_entry()` OnceLock (left untouched — same pattern `🔣️json`'s `declaration()` establishes
/// for this field). `standards::v1_0::subsets::any::engine::register()` itself is left in place, now
/// orphaned/uncalled — deleting it means editing `⚙️engine/`, off-limits here.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("xml", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::schema::xml_artifact_schema_descriptor())
        .formats(formats)
        .schema_documents(crate::schema::XML_SHARED_SCHEMA_DOCUMENTS)
        .inferences([crate::standards::v1_0::subsets::base::schema::inferences::xml_artifact_inference_descriptor()])
        .composers(crate::standards::v1_0::subsets::base::io::io_registry::entries())
        .subset_validators(pilot_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<XmlSnapshot, XmlMutation>(STDIO_XML_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.xml", standard: semio_framework_artifact_reference::StandardId("1.0"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .document_codec_bare::<XmlSnapshot, XmlMutation>(STDIO_XML_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.xml", standard: semio_framework_artifact_reference::StandardId("1.0"), subset: semio_framework_artifact_reference::SubsetId("valid") })
        .try_build()
}

/// 🛡️ The ✳️valid subset's `SubsetValidatorEntry`, built once — see `declaration()`'s own doc for why
/// this is a fresh `subset_validator_entry_of::<XmlValidValidator>()` call rather than a reuse of
/// `subsets::valid::io::derived_composition`'s private `validator_entry()`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pilot_subset_validators() -> &'static [semio_framework_plugin::io::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::io::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::io::subset_validator_entry_of::<crate::standards::v1_0::subsets::valid::io::XmlValidValidator>()]).as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `🔋️energy` exemplar's helper of the same shape. Verbatim copy of `standards::v1_0::subsets::any::
/// engine::register_pilot_languages()`'s five `LanguageSpec`s.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.xml",
                    extension: Some("xml"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1_0::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1_0::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1_0::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1_0::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.xml"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.xml.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1_0::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1_0::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1_0::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1_0::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.xml.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.xml.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v1_0::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1_0::subsets::base::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.xml.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.xml.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1_0::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1_0::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.xml.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.xml.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1_0::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1_0::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.xml.spr"),
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
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "xml", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}
}
//#endregion 🧩️ArtifactItems10
