//! 🖼️ PDF 1.4 page canvas: the resolved 1.4 pages lifted into the shared 1.7 page model, so one scene builder, view lane, selection and text path serve both dialects. PDF 1.4 is a subset of 1.7, and the lift is exactly the file the 1.4 writer saves.
//! [PDF 32000-1 page tree](https://www.iso.org/standard/51502.html)

use semio_os_kernel::{ColdMutation, Mutation};
use semio_plugin::app::InteractionView;
use semio_plugin::retained_command::ArtifactCommandWork;
use semio_plugin::{ArtifactView, Axis, ComponentTree, EditorApp, Emit, Fault, FaultCode, FaultOrigin, PluginAssemblyError, UiAssemblyResult, WindowKindDefinition, WindowLayout, WindowLayoutNode};
use semio_s_artifact_stdio_contract::editing::{bounded_native_edit_work, BoundedNativeEditingEditor};
use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_4::subsets::base::io::encode_pdf;
use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations::{InsertPage, MovePage, PdfMutation, RemovePage, ReplacePageText, ResizePage};
use semio_s_artifact_stdio_pdf_mut_family_11::standards::v1_7::subsets::base::io::import::decode_pdf;
use semio_s_artifact_stdio_pdf_page_editor::page;
pub use page::{pdf_editor_labels, pdf_fault_notices, refuse, PdfFault};
use semio_s_artifact_stdio_pdf_presence_lane::PresenceLane;
use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot as Snapshot14};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot as Snapshot17;
use semio_s_artifact_stdio_pdf_view_lane::{lifted_view_work, render_page_body_of, render_page_body_peers, PdfViewTransient, PdfViewTransientMutation, ViewCarrier};
use semio_ui_locale::Locale;
use serde_json::Value;
use std::borrow::Cow;

const CANVAS_ACTIONS: [&str; 8] = ["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "set-text", "insert-page", "remove-page", "move-page", "set-page-size"];

fn refusal(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("stdio.pdf.1.4.page-edit"), message)
}

/// 🧭️ Whether `action` is one the canvas window offers over the 1.4 page domain.
pub fn is_page_action(action: &str) -> bool {
    CANVAS_ACTIONS.contains(&action) || page::is_view_action(action) || semio_s_artifact_stdio_pdf_presence_lane::is_presence_action(action)
}

/// 🏷️ The stable command identifier of a canvas action, for the command log.
pub fn static_action(action: &str) -> &'static str {
    CANVAS_ACTIONS.into_iter().chain(page::VIEW_ACTION_IDS).chain(semio_s_artifact_stdio_pdf_presence_lane::PRESENCE_ACTION_IDS).find(|candidate| *candidate == action).unwrap_or("page-edit")
}

/// 🔼️ Lifts the resolved 1.4 pages into the 1.7 page model through the bytes the 1.4 writer saves, so what is painted is what a reader opens.
pub fn lift(snapshot: &Snapshot14) -> Result<Snapshot17, Fault> {
    let bytes = encode_pdf(snapshot).map_err(refusal)?;
    decode_pdf(&bytes).map_err(|error| refusal(error.to_string()))
}

/// 🔬️ The page lens of a 1.4 editor's view work: the lifted page model, owned.
pub fn lens(snapshot: &Snapshot14) -> Result<Cow<'_, Snapshot17>, Fault> {
    lift(snapshot).map(Cow::Owned)
}

/// 🪟️ The canvas window: the shared page window reduced to the actions the 1.4 page domain can publish.
pub fn window_definition() -> WindowKindDefinition {
    let mut definition = page::window_definition();
    definition.actions.retain(|action| CANVAS_ACTIONS.iter().chain(page::VIEW_ACTION_IDS.iter()).any(|id| action.id == *id));
    definition.actions.extend(semio_s_artifact_stdio_pdf_presence_lane::presence_action_definitions());
    definition
}

/// 🪟️ Every window kind the canvas contributes: the page canvas, the page thumbnails and the accessible text layer.
pub fn window_kinds() -> Vec<WindowKindDefinition> {
    vec![window_definition(), page::thumbnails_window_definition(), page::text_layer_window_definition()]
}

/// 🎯️ Declares the page-object selection domain and points the canvas window at it.
pub fn install_object_interaction(definition: &mut semio_plugin::AppDefinition) {
    page::install_object_interaction(definition);
}

/// 🪟️ Thumbnails, canvas, text layer, addressed text drafts and snapshot details side by side.
pub fn document_layout(text_window_kind_id: &str) -> WindowLayout {
    let stack = |size: f64, id: &str| WindowLayoutNode::Stack { size: Some(size), active_window_id: None, children: vec![WindowLayoutNode::Window { window_kind_id: id.into(), title: None, instance_id: None, template_id: None, corner: None }] };
    WindowLayout { root: WindowLayoutNode::Split { axis: Axis::Horizontal, size: None, children: vec![
        stack(0.12, page::THUMBNAILS_WINDOW_KIND_ID),
        stack(0.38, page::WINDOW_KIND_ID),
        stack(0.14, page::TEXT_LAYER_WINDOW_KIND_ID),
        stack(0.2, text_window_kind_id),
        stack(0.16, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_WINDOW_KIND_ID),
    ] } }
}

/// 🧾️ Reads a canvas action into the replayable payload the shared page editor defines.
pub fn edit_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<String, Fault> {
    if !is_page_action(action) { return Err(refusal(format!("unknown PDF 1.4 page action '{action}'"))); }
    page::edit_from_action(action, args).map(|edit| edit.payload)
}

fn admit(snapshot: &Snapshot14, mutation: PdfMutation) -> Result<Emit<PdfMutation>, Fault> {
    if !mutation.diff(snapshot).messages().is_empty() { return Err(refusal("Page action is outside this document's domain")); }
    Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
}

fn number(value: &Value, key: &str) -> f64 {
    value.get(key).and_then(Value::as_f64).filter(|number| number.is_finite()).unwrap_or(0.0)
}

fn set_text(snapshot: &Snapshot14, value: &Value) -> Result<Emit<PdfMutation>, Fault> {
    let object = value.get("object").and_then(Value::as_str).unwrap_or("");
    let text = value.get("text").and_then(Value::as_str).unwrap_or("");
    let found = page::find_object(&lift(snapshot)?, object).filter(|found| found.kind == page::ObjectKind::Text).ok_or_else(|| refusal(format!("PDF page text '{object}' is not on the page")))?;
    let current = snapshot.pages.get(found.page).ok_or_else(|| refusal("Page target does not exist"))?;
    if current.text == text { return Ok(Emit::default()); }
    admit(snapshot, PdfMutation::ReplacePageText(ReplacePageText { index: found.page, text: text.into() }))
}

/// ▶️ Admits one replayed canvas action against the 1.4 page domain: a hit selects, text and page structure become the own mutation, anything else is refused.
pub fn emit_page_edit(snapshot: &Snapshot14, action: &str, payload: &str) -> Result<Emit<PdfMutation>, Fault> {
    let value: Value = serde_json::from_str(payload).map_err(|error| refusal(error.to_string()))?;
    let page_index = value.get("page").and_then(Value::as_u64).map_or(0, |page| page as usize);
    let (width, height) = (number(&value, "width").max(1.0), number(&value, "height").max(1.0));
    match action {
        "canvasPointerMove" | "canvasPointerUp" => Ok(Emit::default()),
        "canvasPointerDown" => Ok(Emit { effects: page::emit_page_edit(&lift(snapshot)?, action, payload)?.effects, ..Default::default() }),
        "set-text" => set_text(snapshot, &value),
        "insert-page" => admit(snapshot, PdfMutation::InsertPage(InsertPage { index: page_index, page: PageDoc::new(width, height) })),
        "remove-page" => admit(snapshot, PdfMutation::RemovePage(RemovePage { index: page_index })),
        "move-page" => admit(snapshot, PdfMutation::MovePage(MovePage { from: page_index, to: number(&value, "x").max(0.0) as usize })),
        "set-page-size" => admit(snapshot, PdfMutation::ResizePage(ResizePage { index: page_index, width, height })),
        other => Err(refusal(format!("PDF 1.4 pages keep size and text only; '{other}' is not a 1.4 page action"))),
    }
}

/// 🖼️ Paints the shared page bodies (canvas through the view, thumbnails, text layer) over the lifted pages; `None` for any other body.
pub fn render_body(body_key: &str, doc: &ArtifactView<'_, Snapshot14>, controller: &str, interaction: Option<&InteractionView<'_>>, transient: Option<&PdfViewTransient>, locale: Locale) -> Option<UiAssemblyResult<ComponentTree>> {
    if ![page::BODY_KEY, page::THUMBNAILS_BODY_KEY, page::TEXT_LAYER_BODY_KEY].contains(&body_key) { return None; }
    let lifted = match lift(doc.snapshot) {
        Ok(lifted) => lifted,
        Err(fault) => return Some(Err(PluginAssemblyError::new("stdio.pdf.1.4.lift", fault.message))),
    };
    render_page_body_of(body_key, &lifted, &|| semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc), controller, interaction, transient, locale, page::render_view)
}

/// 👥️ [`render_body`] with the collaborators' typed presence: `typed` lists every peer's presence, `domain` is the object selection domain.
#[allow(clippy::too_many_arguments)]
pub fn render_body_peers<'a>(body_key: &str, doc: &ArtifactView<'_, Snapshot14>, controller: &str, interaction: &InteractionView<'_>, transient: &PdfViewTransient, locale: Locale, typed: impl Iterator<Item = (&'a str, &'a semio_s_artifact_stdio_pdf_presence_lane::PdfPresence)>) -> Option<UiAssemblyResult<ComponentTree>> {
    if ![page::BODY_KEY, page::THUMBNAILS_BODY_KEY, page::TEXT_LAYER_BODY_KEY].contains(&body_key) { return None; }
    let lifted = match lift(doc.snapshot) {
        Ok(lifted) => lifted,
        Err(fault) => return Some(Err(PluginAssemblyError::new("stdio.pdf.1.4.lift", fault.message))),
    };
    let typed: Vec<_> = typed.collect();
    render_page_body_peers(body_key, &lifted, &|| semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc), controller, Some(interaction), Some(transient), locale, page::render_view, &|interaction| semio_s_artifact_stdio_pdf_presence_lane::typed_samples(interaction, page::OBJECT_DOMAIN, &page::object_page_of(&lifted), typed.iter().copied()))
}

/// 🧵️ The work of one tool of a 1.4 editor: the view work for a view tool over the lifted pages, the shared bounded edit work for every other.
pub fn native_work<E>(tool: &'static str) -> Box<dyn ArtifactCommandWork<EditorApp<E>>>
where
    E: BoundedNativeEditingEditor<Snapshot = Snapshot14, Transient = PdfViewTransient, TransientMutation = PdfViewTransientMutation> + 'static,
    E::Command: ViewCarrier,
    E::Presence: PresenceLane<Mutation = E::PresenceMutation>,
{
    lifted_view_work::<E>(tool, lens, bounded_native_edit_work::<E>)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
