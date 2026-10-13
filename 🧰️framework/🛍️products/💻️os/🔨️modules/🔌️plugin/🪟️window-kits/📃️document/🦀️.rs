//! 📃️ The document window kit.
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
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use crate::text_kit::TextDraftView;
use crate::text_kit::TextWindowKit;
use crate::tree_kit::TreeWindows;
use crate::tree_kit::tree_window_item;
use crate::tree_kit::tree_window_section;
use crate::window_kits::WindowKit;
use crate::window_surface::UiAssemblyResult;
use crate::window_surface::scene_surface;
use crate::window_surface::ui_assembly_error;
use crate::window_surface::ui_label;
use crate::window_surface::window_kind_definition;

/// 📄️ A paginated text document — one `ui_wgpu::wgpu::ui_text` child per page inside an unlabeled vertical stack.
#[derive(Clone, Debug, PartialEq)]
pub struct DocumentPage {
    pub text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocumentView {
    pub pages: Vec<DocumentPage>,
}

/// ✏️ One explicitly addressed document text target with a locally buffered replacement.
#[derive(Debug, PartialEq)]
pub struct EditableDocumentPage {
    pub page_index: u32,
    pub item_index: u32,
    pub text: String,
    pub arguments: Option<UiValue>,
}

/// ✏️ Addressed text targets rendered as prefilled apply/discard drafts.
#[derive(Debug, PartialEq)]
pub struct EditableDocumentView {
    pub pages: Vec<EditableDocumentPage>,
    pub publication_revision: UiPublicationRevision,
}

impl EditableDocumentPage {
    /// 📄️ Creates an ordinal document draft with a renderer-authored revision guard.
    pub fn new(page_index: u32, item_index: u32, text: impl Into<String>) -> Self {
        Self { page_index, item_index, text: text.into(), arguments: None }
    }

    /// 🧭️ Creates a document draft carrying the artifact's complete stable address.
    pub fn with_arguments(page_index: u32, item_index: u32, text: impl Into<String>, arguments: UiValue) -> Self {
        Self { page_index, item_index, text: text.into(), arguments: Some(arguments) }
    }
}

pub struct DocumentWindowKit;

impl WindowKit for DocumentWindowKit {
    type ViewModel = DocumentView;
    const KIND_ID: &'static str = "framework.window.document";

    fn window_kind() -> WindowKindDefinition {
        window_kind_definition(Self::KIND_ID, "Artifact", "Artefakt", SurfaceKind::TextEditor, "file-text", Vec::new())
    }

    fn editable_window_kind() -> WindowKindDefinition {
        Self::editable_window_kind_with_args(vec![
            ActionArgDef::index("page", LocalizedLabel::native("Page", "Seite")).required(),
            ActionArgDef::index("item", LocalizedLabel::native("Text Item", "Textelement")).required(),
            ActionArgDef::target_revision("revision", LocalizedLabel::native("Revision", "Revision")).required(),
        ])
    }

    fn render(view: &DocumentView) -> UiAssemblyResult<BuiltNode> {
        Self::render_windowed(view, &TreeWindows::unhosted())
    }
}

impl DocumentWindowKit {
    /// 🧭️ Declares the document edit action with an artifact-owned stable address contract.
    pub fn editable_window_kind_with_args(mut arguments: Vec<ActionArgDef>) -> WindowKindDefinition {
        arguments.push(ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")).min_length(0).required());
        window_kind_definition(
            Self::KIND_ID,
            "Artifact",
            "Artefakt",
            SurfaceKind::TextEditor,
            "file-text",
            vec![ActionDefinition::bounded_catalog("set-page", LocalizedLabel::native("Set Page Text", "Seitentext setzen"), ActionKind::Mutation)
                .with_args(arguments)
                .describe(LocalizedLabel::native(
                    "Replaces the addressed page text when its revision still matches; the previous text remains available through undo.",
                    "Ersetzt den adressierten Seitentext, wenn seine Revision noch übereinstimmt; der bisherige Text bleibt über Rückgängig verfügbar.",
                ))
                .in_palette(false)],
        )
    }

    /// 🔐️ Computes the compact optimistic-concurrency token carried by a document draft.
    pub fn text_revision(text: &str) -> String {
        let hash = text.as_bytes().iter().fold(0xcbf29ce484222325_u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3));
        format!("{hash:016x}")
    }

    /// ✏️ Renders only faithful, explicitly addressed text targets as local apply/discard drafts.
    pub fn render_editable_windowed(view: &EditableDocumentView, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
        Self::render_editable_windowed_with_accessory(view, windows, locale, |_, _| Ok(None))
    }

    /// 🧩️ Renders one domain-owned accessory beside each materialized document draft.
    pub fn render_editable_windowed_with_accessory(
        view: &EditableDocumentView,
        windows: &TreeWindows<'_>,
        locale: Locale,
        mut accessory: impl FnMut(usize, &EditableDocumentPage) -> UiAssemblyResult<Option<BuiltNode>>,
    ) -> UiAssemblyResult<BuiltNode> {
        let pages: Vec<_> = view.pages.iter().enumerate().collect();
        tree_window_section(windows, Self::KIND_ID, Label::default(), true, &pages, |(ordinal, page)| {
            let id = format!("page-{}-item-{}", page.page_index, page.item_index);
            let title = match locale {
                Locale::De => format!("Seite {}, Textelement {}", page.page_index + 1, page.item_index + 1),
                Locale::En => format!("Page {}, text item {}", page.page_index + 1, page.item_index + 1),
            };
            let item = tree_item(ui_label(title, "document-window.editable-page-label")?).try_id(&id).map_err(|_| ui_assembly_error("document-window.editable-page-id"))?;
            tree_window_item(windows, item, &id, *ordinal == 0, &[*page], |page| {
                let arguments = match &page.arguments {
                    Some(arguments @ UiValue::Map(_)) => arguments.credited_clone().ok_or_else(|| ui_assembly_error("document-window.editable-argument-credits"))?,
                    Some(_) => return Err(ui_assembly_error("document-window.editable-arguments-map")),
                    None => {
                        let mut arguments = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("document-window.editable-arguments"))?;
                        arguments.try_insert("page".into(), UiValue::Number(f64::from(page.page_index))).map_err(|_| ui_assembly_error("document-window.editable-page-argument"))?;
                        arguments.try_insert("item".into(), UiValue::Number(f64::from(page.item_index))).map_err(|_| ui_assembly_error("document-window.editable-item-argument"))?;
                        arguments
                            .try_insert("revision".into(), UiValue::Text(UiText::try_from_str(&Self::text_revision(&page.text)).ok_or_else(|| ui_assembly_error("document-window.editable-revision"))?))
                            .map_err(|_| ui_assembly_error("document-window.editable-revision-argument"))?;
                        UiValue::Map(arguments.finish())
                    }
                };
                let (apply_label, discard_label, conflict_label, applying_label, cancel_label, failed_label) = match locale {
                    Locale::De => ("Anwenden", "Verwerfen", "Der gespeicherte Text hat sich geändert. Bitte neu abgleichen.", "Wird angewendet …", "Abbrechen", "Änderung fehlgeschlagen. Der Entwurf bleibt erhalten."),
                    Locale::En => ("Apply", "Discard", "The saved text changed. Reconcile before applying.", "Applying…", "Cancel", "The edit failed. Your draft is preserved."),
                };
                let (syntax_error_label, validation_error_label, path_label, line_label, column_label) = match locale {
                    Locale::De => ("Der Quelltext enthält ungültige Syntax.", "Der Entwurf entspricht nicht dem Dokumentformat.", "Pfad", "Zeile", "Spalte"),
                    Locale::En => ("The source contains invalid syntax.", "The draft does not match the document format.", "Path", "Line", "Column"),
                };
                let draft = TextWindowKit::render_draft(&TextDraftView {
                    surface_id: format!("document-draft-{id}"),
                    text: page.text.clone(),
                    language: None,
                    action_id: "set-page".into(),
                    argument: "text".into(),
                    arguments: Some(arguments),
                    publication_revision: view.publication_revision,
                    apply_label: apply_label.into(),
                    discard_label: discard_label.into(),
                    conflict_label: conflict_label.into(),
                    applying_label: applying_label.into(),
                    cancel_label: cancel_label.into(),
                    failed_label: failed_label.into(),
                    syntax_error_label: syntax_error_label.into(),
                    validation_error_label: validation_error_label.into(),
                    path_label: path_label.into(),
                    line_label: line_label.into(),
                    column_label: column_label.into(),
                })?;
                let Some(accessory) = accessory(*ordinal, page)? else { return Ok(draft) };
                column()
                    .try_id("document-page-content")
                    .map_err(|_| ui_assembly_error("document-window.editable-content-id"))?
                    .try_children([accessory, draft])
                    .map_err(|_| ui_assembly_error("document-window.editable-content-children"))?
                    .try_build()
                    .map_err(|_| ui_assembly_error("document-window.editable-content-build"))
            })
        })
    }

    /// 📄️ Pages are expandable and windowed; each complete page buffer uses the text scene carrier.
    pub fn render_windowed(view: &DocumentView, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
        let pages: Vec<_> = view.pages.iter().enumerate().collect();
        tree_window_section(windows, Self::KIND_ID, Label::default(), true, &pages, |(index, page)| {
            let id = format!("page-{index}");
            let item = tree_item(ui_label((index + 1).to_string(), "document-window.page-label")?).try_id(&id).map_err(|_| ui_assembly_error("document-window.page-id"))?;
            tree_window_item(windows, item, &id, *index == 0, &[*page], |page| {
                let mut scene = semio_framework_ui_scene::TextEditorScene::base(page.text.clone(), None, None);
                scene.settings_json = Some("{\"readOnly\":true}".into());
                scene_surface("content", SurfaceKind::TextEditor, &scene)
            })
        })
    }
}
