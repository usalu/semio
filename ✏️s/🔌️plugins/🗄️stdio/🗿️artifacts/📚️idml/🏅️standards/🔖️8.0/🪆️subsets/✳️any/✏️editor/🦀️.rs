//! ✏️ Idml editor — one `🪟️main` Document window; each story `Content` run is a revision-guarded draft applied through the
//! artifact's own `IdmlMutation::ReplacePart`, so every unedited byte of every part is preserved.

use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, DraftView, Editor, EditorApp, Emit, Fault, FaultCode, FaultOrigin, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use semio_framework_ui_locale::Label;
use semio_s_artifact_stdio_idml_story::{set_story_content, story_pages};
use semio_s_artifact_stdio_idml::{IdmlMutation, IdmlPackage, IdmlSnapshot, DOCUMENT_SCHEMA};

#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "🎭️modes/✏️edit/🦀️.rs"]
pub mod modes;

use modes::main;

/// 🪪️ Artifact coordinate `s.stdio.idml@8.0/*`, duplicated in the read-only surface so a viewer never depends on this module.
pub const IDML_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.idml", standard: StandardId("8.0"), subset: SubsetId::ANY };

/// ✏️ The editor's typed command channel: the one edit `🪟️main`'s `set-page` action triggers.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum IdmlEditorCommand {
    SetPage { page: u32, item: u32, revision: String, text: String },
}

semio_s_artifact_stdio_contract::impl_serde_op_codec!(IdmlEditorCommand, "idml editor command");
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(IdmlEditorCommand, ["set-page"]);

fn fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

/// 🧮️ Maps one addressed, revision-checked story run to a reversible `ReplacePart`; `None` when nothing changes.
pub fn build_set_page_mutation(snapshot: &IdmlSnapshot, page: u32, item: u32, revision: &str, text: &str) -> Result<Option<IdmlMutation>, Fault> {
    let pages = story_pages(snapshot);
    let run = pages.get(page as usize).and_then(|runs| runs.get(item as usize)).ok_or_else(|| fault("stdio.idml.set-page.address", "the addressed story run does not exist"))?;
    if semio_framework_plugin::app::DocumentWindowKit::text_revision(&run.text) != revision {
        return Err(fault("stdio.idml.set-page.stale-revision", "the story run changed since the draft was opened"));
    }
    set_story_content(snapshot, &run.part, run.index, &run.revision, text).map_err(|error| fault(error.code(), error.code()))
}

fn minimal_package() -> Result<IdmlSnapshot, Fault> {
    let story = concat!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>",
        "<idPkg:Story xmlns:idPkg=\"http://ns.adobe.com/AdobeInDesign/idml/1.0/packaging\" DOMVersion=\"8.0\"><Story Self=\"u1\"><ParagraphStyleRange><CharacterStyleRange><Content></Content></CharacterStyleRange></ParagraphStyleRange></Story></idPkg:Story>"
    );
    let parts = vec![
        ("mimetype".to_string(), semio_s_artifact_stdio_idml::PACKAGE_MIME.as_bytes().to_vec()),
        ("designmap.xml".to_string(), b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><Document DOMVersion=\"8.0\"/>".to_vec()),
        ("Stories/Story_u1.xml".to_string(), story.as_bytes().to_vec()),
    ];
    IdmlPackage::from_parts(parts).map(IdmlSnapshot::from_package).map_err(|message| fault("stdio.idml.birth", message))
}

#[derive(Default, Clone, Copy)]
pub struct IdmlEditor;

impl ArtifactEditor for IdmlEditor {
    type Snapshot = IdmlSnapshot;
    type Mutation = IdmlMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<IdmlEditorCommand>;

    const DIALECT: Dialect = IDML_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DOCUMENT_SCHEMA;

    fn import_media(port: &str, media: &semio_framework_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_framework_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec { format_kind: "s.stdio.idml@8.0", extension: ".idml", media_type: semio_s_artifact_stdio_idml::PACKAGE_MIME, binary: true })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        snapshot.package().map_err(|error| error.to_string()).and_then(|package| package.encode()).map_err(|message| semio_framework_plugin::MediaError::Payload("artifact:native".into(), message))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        IdmlPackage::decode(bytes).map(IdmlSnapshot::from_package).map_err(|message| semio_framework_plugin::MediaError::Payload("artifact:native".into(), message))
    }

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🏅️standards/🔖️8.0/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.idml@8.0/*#editor",
        artifact_schema: "stdio.idml",
        preparation: "stdio-idml-any-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |native| match native {
            IdmlEditorCommand::SetPage { .. } => "set-page",
        })
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() {
            return Err(fault);
        }
        let args = args.as_ref();
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-page" => Ok(IdmlEditorCommand::SetPage {
                page: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "page")?,
                item: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "item")?,
                revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?,
                text: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "text")?,
            }),
            other => Err(fault("stdio.idml.unhandled-action", format!("unknown idml editor action '{other}'"))),
        })
    }

    fn reference_birth() -> Option<fn() -> Result<IdmlSnapshot, Fault>> {
        Some(minimal_package)
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &semio_framework_2d_compute::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(IdmlEditorCommand::SetPage { page, item, revision, text }) => {
                let Some(mutation) = build_set_page_mutation(doc.snapshot, *page, *item, revision, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let publication_revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
                main::render_windowed(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), view_state.locale, publication_revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.idml@8.0/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for IdmlEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_rules() -> &'static semio_s_artifact_stdio_contract::editing::EditRules {
        &edit_rules::EDIT_RULES
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: IdmlEditor,
    tools: ["set-page"],
    payload_schema: "semio.stdio.document-text-edit-command.v1",
    reduce: |command, snapshot| {
        let mutation = match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(IdmlEditorCommand::SetPage { page, item, revision, text }) => build_set_page_mutation(snapshot, *page, *item, revision, text)?,
            _ => return Err(Fault::from("stdio-idml-native-edit-command-mismatch")),
        };
        Ok(Emit { artifact_mutations: mutation.into_iter().collect(), ..Default::default() })
    },
}

/// 🧩️ The editor's manifest: the story window beside the shared snapshot details window.
pub fn create_idml_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(IDML_EDITOR_DIALECT)
        .document(["semio", "stdio", "idml"])
        .icon_id("file-text")
        .mode_def(modes::definition())
        .default_mode_id(modes::IDML_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Stories"));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}

pub type IdmlEditorApp = EditorApp<IdmlEditor>;
