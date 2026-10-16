//! 📝️ The text window kit.
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, WindowKindDefinition};
use semio_framework_ui_contract::BuiltNode;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use serde::Serialize;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use crate::WindowKit;
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_plugin_window_transient::scene_surface;
use semio_framework_plugin_interaction::ui_assembly_error;
use semio_framework_plugin_window_transient::window_kind_definition;



/// 📝️ One language-tagged read-only text buffer (`TextEditorScene.settings_json` = `{"readOnly":true}`). An editable text
/// window is never live whole-text typing: it is an explicit draft ([`TextEditView`], [`TextWindowKit::render_editable`]) —
/// the host keeps the draft locally with its own undo, and Apply publishes ONE `textEdit` (design §13.2 of ticket
/// 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, audit F-5).
#[derive(Clone, Debug, PartialEq)]
pub struct TextView {
    pub text: String,
    pub language: Option<String>,
}

/// ✍️ One editable text buffer of the kit's `textEdit` verb: the window's surface, the text, its language, and the document
/// revision the draft starts from (`revision` argument) when the editor refuses a draft over a document that changed.
#[derive(Clone, Debug, PartialEq)]
pub struct TextEditView {
    pub surface_id: String,
    pub text: String,
    pub language: Option<String>,
    pub revision: Option<String>,
    pub publication_revision: UiPublicationRevision,
}

/// 📝 One locally buffered text edit committed through an artifact-owned action.
#[derive(Debug, PartialEq)]
pub struct TextDraftView {
    pub surface_id: String,
    pub text: String,
    pub language: Option<String>,
    pub action_id: String,
    pub argument: String,
    pub arguments: Option<UiValue>,
    pub publication_revision: UiPublicationRevision,
    pub apply_label: String,
    pub discard_label: String,
    pub conflict_label: String,
    pub applying_label: String,
    pub cancel_label: String,
    pub failed_label: String,
    pub syntax_error_label: String,
    pub validation_error_label: String,
    pub path_label: String,
    pub line_label: String,
    pub column_label: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TextDraftSettings<'a> {
    read_only: bool,
    edit_action: &'a str,
    edit_argument: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    edit_arguments: &'a Option<UiValue>,
    commit: &'static str,
    publication_revision: String,
    apply_label: &'a str,
    discard_label: &'a str,
    conflict_label: &'a str,
    applying_label: &'a str,
    cancel_label: &'a str,
    failed_label: &'a str,
    syntax_error_label: &'a str,
    validation_error_label: &'a str,
    path_label: &'a str,
    line_label: &'a str,
    column_label: &'a str,
}

pub struct TextWindowKit;

impl WindowKit for TextWindowKit {
    type ViewModel = TextView;
    const KIND_ID: &'static str = "framework.window.text";

    fn window_kind() -> WindowKindDefinition {
        window_kind_definition(Self::KIND_ID, "Text", "Text", SurfaceKind::TextEditor, "type", Vec::new())
    }

    fn editable_window_kind() -> WindowKindDefinition {
        window_kind_definition(
            Self::KIND_ID,
            "Text",
            "Text",
            SurfaceKind::TextEditor,
            "type",
            vec![ActionDefinition::bounded_catalog(Self::EDIT_ACTION_ID, LocalizedLabel::native("Edit Text", "Text bearbeiten"), ActionKind::Mutation)
                .with_args(vec![ActionArgDef::document_revision(Self::REVISION_ARGUMENT, LocalizedLabel::native("Revision", "Revision")), ActionArgDef::text(Self::TEXT_ARGUMENT, LocalizedLabel::native("Text", "Text")).min_length(0).required()])
                .describe(LocalizedLabel::native(
                    "Replaces the entire text of the document with the given text; the previous text is gone unless the edit is undone.",
                    "Ersetzt den gesamten Text des Dokuments durch den angegebenen Text; der bisherige Text ist fort, sofern die Änderung nicht rückgängig gemacht wird.",
                ))
                .destructive()],
        )
    }

    fn render(view: &TextView) -> UiAssemblyResult<BuiltNode> {
        Self::render_read_only(Self::KIND_ID, view)
    }
}

impl TextWindowKit {
    /// ✍️ The kit's whole-text verb (Apply of an editable text window's draft).
    pub const EDIT_ACTION_ID: &'static str = "textEdit";
    /// ✍️ The argument carrying the applied text.
    pub const TEXT_ARGUMENT: &'static str = "text";
    /// 📐️ The argument carrying the document revision the draft started from.
    pub const REVISION_ARGUMENT: &'static str = "revision";
    /// ✂️ The edit argument of a window that takes the draft's CHANGE SET instead of its text: the JSON list of ranges
    /// `{offset, delete, insert}` (Unicode scalar values, ascending, never overlapping, in the coordinates of the text the draft
    /// started from) the editor made, which the artifact applies verbatim — twin of `DRAFT_SPLICES_ARGUMENT` in `✂️text-splice`.
    pub const SPLICES_ARGUMENT: &'static str = "splices";

    /// 📖️ Renders read-only text on a caller-owned surface identity.
    pub fn render_read_only(surface_id: &str, view: &TextView) -> UiAssemblyResult<BuiltNode> {
        let scene = semio_framework_ui_scene::TextEditorScene {
            buffer: view.text.clone(),
            lanes: Vec::new(),
            language: view.language.clone(),
            selection_json: None,
            tokens_json: None,
            diagnostics_json: None,
            completions_json: None,
            overlays_json: None,
            occurrences_json: None,
            placeholders_json: None,
            extra_carets_json: None,
            selectable_spans_json: None,
            settings_json: Some("{\"readOnly\":true}".to_string()),
            camera_json: None,
            hover_json: None,
            newline_gates_json: None,
            rename_json: None,
        };
        scene_surface(surface_id, SurfaceKind::TextEditor, &scene)
    }

    /// ✍️ Renders an editable text window under the explicit-draft policy: the kit's `textEdit` verb, the draft's starting
    /// revision when given, and the kit's own localized Apply / Discard / conflict / progress / cancel / failure copy.
    pub fn render_editable(view: &TextEditView, locale: Locale) -> UiAssemblyResult<BuiltNode> {
        Self::render_editable_with(view, locale, Self::TEXT_ARGUMENT)
    }

    /// ✂️ Renders an editable text window whose Apply publishes the draft's change set ([`Self::SPLICES_ARGUMENT`]) rather than its text:
    /// the host records the ranges the editor made, so the artifact receives the gesture, never a draft to compare with its document.
    pub fn render_editable_by_splices(view: &TextEditView, locale: Locale) -> UiAssemblyResult<BuiltNode> {
        Self::render_editable_with(view, locale, Self::SPLICES_ARGUMENT)
    }

    fn render_editable_with(view: &TextEditView, locale: Locale, argument: &str) -> UiAssemblyResult<BuiltNode> {
        let arguments = match &view.revision {
            None => None,
            Some(revision) => {
                let mut arguments = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("text-window.revision-arguments"))?;
                arguments.try_insert(Self::REVISION_ARGUMENT.into(), UiValue::Text(UiText::try_from_str(revision).ok_or_else(|| ui_assembly_error("text-window.revision"))?)).map_err(|_| ui_assembly_error("text-window.revision-arguments"))?;
                Some(UiValue::Map(arguments.finish()))
            }
        };
        let (apply, discard, conflict, applying, cancel, failed, syntax_error, validation_error, path, line, column) = match locale {
            Locale::En => ("Apply", "Discard", "The document changed. Copy your draft or discard it before applying.", "Applying…", "Cancel", "The draft could not be applied. It is kept.", "The source contains invalid syntax.", "The draft does not match the document format.", "Path", "Line", "Column"),
            Locale::De => {
                ("Anwenden", "Verwerfen", "Das Dokument wurde geändert. Kopieren Sie Ihren Entwurf oder verwerfen Sie ihn vor dem Anwenden.", "Wird angewendet …", "Abbrechen", "Der Entwurf konnte nicht angewendet werden. Er bleibt erhalten.", "Der Quelltext enthält ungültige Syntax.", "Der Entwurf entspricht nicht dem Dokumentformat.", "Pfad", "Zeile", "Spalte")
            }
        };
        Self::render_draft(&TextDraftView {
            surface_id: view.surface_id.clone(),
            text: view.text.clone(),
            language: view.language.clone(),
            action_id: Self::EDIT_ACTION_ID.into(),
            argument: argument.into(),
            arguments,
            publication_revision: view.publication_revision,
            apply_label: apply.into(),
            discard_label: discard.into(),
            conflict_label: conflict.into(),
            applying_label: applying.into(),
            cancel_label: cancel.into(),
            failed_label: failed.into(),
            syntax_error_label: syntax_error.into(),
            validation_error_label: validation_error.into(),
            path_label: path.into(),
            line_label: line.into(),
            column_label: column.into(),
        })
    }

    /// 💾 Renders a prepopulated local draft with explicit, localized apply and discard controls.
    pub fn render_draft(view: &TextDraftView) -> UiAssemblyResult<BuiltNode> {
        let settings = TextDraftSettings {
            read_only: false,
            edit_action: &view.action_id,
            edit_argument: &view.argument,
            edit_arguments: &view.arguments,
            commit: "explicit",
            publication_revision: view.publication_revision.0.to_string(),
            apply_label: &view.apply_label,
            discard_label: &view.discard_label,
            conflict_label: &view.conflict_label,
            applying_label: &view.applying_label,
            cancel_label: &view.cancel_label,
            failed_label: &view.failed_label,
            syntax_error_label: &view.syntax_error_label,
            validation_error_label: &view.validation_error_label,
            path_label: &view.path_label,
            line_label: &view.line_label,
            column_label: &view.column_label,
        };
        let settings_json = serde_json::to_string(&settings).map_err(|_| ui_assembly_error("text-window.draft-settings"))?;
        let mut scene = semio_framework_ui_scene::TextEditorScene::base(view.text.clone(), view.language.clone(), None);
        scene.settings_json = Some(settings_json);
        scene_surface(&view.surface_id, SurfaceKind::TextEditor, &scene)
    }
}
