//! 🧵️ PDF view work: the retained work every view tool rides. It reduces one view intent over the current transient and publishes the next view on the transient lane only; the document is never touched.
//! [Retained command work](https://www.w3.org/TR/css-transforms-1/)

use crate::editor::pdf_view::transient::{PdfViewTransient, PdfViewTransientMutation, SetView};
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_plugin::{ArtifactEditor, EditorApp, Emit, EphemeralEmit, Fault};
use semio_s_artifact_stdio_contract::editing::{bounded_native_edit_work, BoundedNativeEditingEditor, SnapshotEditingCommand};
use semio_plugin::{built_to_component_tree, ArtifactView, ComponentTree, UiAssemblyResult};
use semio_s_artifact_stdio_pdf_page_editor::page::{self, hit_layers, is_view_action, reduce_view, search, view_intent, OverlayLayer, SearchControl, ViewIntent, ViewState, ZoomMode};
use semio_s_artifact_stdio_pdf_presence_lane::{interaction_samples, is_presence_action, PeerBoard, PeerSample, presence_intent, PresenceIntent, PresenceLane};
use semio_ui_locale::Locale;
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use std::borrow::Cow;
use std::marker::PhantomData;

//#region 🧵️ViewWork
/// 📐️ The viewport fits and zoom steps resolve against until the host reports its own; the host re-fits through the framing it is handed.
const NOMINAL_VIEWPORT: (f64, f64) = (1024.0, 768.0);

/// 🔌️ How an editor's native command carries one page-edit (action id and JSON payload).
pub trait ViewNative {
    fn page_edit(&self) -> Option<(&str, &str)>;
}

/// 🔌️ A command that can carry a view action.
pub trait ViewCarrier {
    fn view_edit(&self) -> Option<(&str, &str)>;
}

impl<N: ViewNative> ViewCarrier for SnapshotEditingCommand<N> {
    fn view_edit(&self) -> Option<(&str, &str)> {
        match self {
            SnapshotEditingCommand::Native(native) => native.page_edit(),
            _ => None,
        }
    }
}

/// 🔌️ Implements [`ViewNative`] for a native command enum whose `PageEdit { action, payload }` variant carries view actions.
#[macro_export]
macro_rules! view_native {
    ($command:ty) => {
        impl $crate::ViewNative for $command {
            fn page_edit(&self) -> Option<(&str, &str)> {
                match self {
                    Self::PageEdit { action, payload } => Some((action.as_str(), payload.as_str())),
                    _ => None,
                }
            }
        }
    };
}

/// 🔭️ The view a transient holds.
pub fn view_state(transient: &PdfViewTransient) -> ViewState {
    let mode = match transient.zoom_mode.as_str() {
        "fit-page" => ZoomMode::FitPage,
        "fit-width" => ZoomMode::FitWidth,
        "factor" => ZoomMode::Factor(f64::from(transient.zoom_permille) / 1000.0),
        _ => ZoomMode::ActualSize,
    };
    ViewState { mode, page: transient.page as usize, revision: transient.revision, query: transient.query.clone(), match_case: transient.match_case, active: transient.active.map(|at| at as usize) }
}

/// 🔭️ The mutation that makes the transient hold `view`.
pub fn set_view(view: &ViewState) -> SetView {
    let (zoom_mode, zoom_permille) = match view.mode {
        ZoomMode::FitPage => ("fit-page", 0),
        ZoomMode::FitWidth => ("fit-width", 0),
        ZoomMode::ActualSize => ("actual-size", 0),
        ZoomMode::Factor(factor) => ("factor", (factor * 1000.0).round() as u32),
    };
    SetView { zoom_mode: zoom_mode.into(), zoom_permille, page: view.page as u32, revision: view.revision, query: view.query.clone(), match_case: view.match_case, active: view.active.map(|at| at as u32) }
}

/// 🔬️ How an editor's own snapshot becomes the page model the view reduces over: borrowed for the 1.7 model, lifted for a dialect that stores less.
pub type PageLens<S> = for<'a> fn(&'a S) -> Result<Cow<'a, PdfSnapshot>, Fault>;

/// 🔬️ The lens of an editor whose snapshot already is the page model.
pub fn own_page(snapshot: &PdfSnapshot) -> Result<Cow<'_, PdfSnapshot>, Fault> {
    Ok(Cow::Borrowed(snapshot))
}

/// 🧵️ One view tool run: reduce, publish on the transient lane, settle.
pub struct ViewCommandWork<E: ArtifactEditor> {
    tool_id: &'static str,
    consumed: bool,
    lens: PageLens<E::Snapshot>,
    editor: PhantomData<fn() -> E>,
}

impl<E: ArtifactEditor> ViewCommandWork<E> {
    pub fn new(tool_id: &'static str, lens: PageLens<E::Snapshot>) -> Self {
        Self { tool_id, consumed: false, lens, editor: PhantomData }
    }
}

impl<E> ArtifactCommandWork<EditorApp<E>> for ViewCommandWork<E>
where
    E: ArtifactEditor<Transient = PdfViewTransient, TransientMutation = PdfViewTransientMutation> + 'static,
    E::Command: ViewCarrier,
    E::Presence: PresenceLane<Mutation = E::PresenceMutation>,
{
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &E::Command, _snapshot: &E::Snapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<E>>>) -> Option<usize> {
        command.view_edit().is_some_and(|(action, _)| is_view_action(action)).then_some(1)
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn reports_progress(&self) -> bool {
        false
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<E>>, _cx: &mut semio_plugin::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<E>>, semio_plugin::__job::OriginalTurnFailure> {
        let settled: Result<_, Fault> = (|| {
            if std::mem::replace(&mut self.consumed, true) {
                return Err(Fault::from("pdf-view-work-repeated"));
            }
            let (action, payload) = input.command.view_edit().ok_or_else(|| Fault::from("pdf-view-route-mismatch"))?;
            let value: serde_json::Value = serde_json::from_str(payload).map_err(|_| Fault::from("pdf-view-payload-invalid"))?;
            let page_model = (self.lens)(input.snapshot)?;
            let page = match value["page"].as_u64() {
                Some(number) if action == "go-to-page" => page::resolve_page(page_model.pages.len(), number)?,
                _ => 0,
            };
            let intent = view_intent(action, value["text"].as_str().unwrap_or(""), page).ok_or_else(|| Fault::from("pdf-view-route-mismatch"))?;
            let current = input.context.map(|context| view_state(context.transient.as_ref())).unwrap_or_default();
            let hits = if matches!(intent, ViewIntent::NextHit | ViewIntent::PreviousHit) {
                search(&page_model, &current.query, current.match_case, &mut SearchControl { cancelled: &|| false, progress: &mut |_, _| {} }).hits
            } else {
                Vec::new()
            };
            let next = reduce_view(&page_model, &current, &intent, NOMINAL_VIEWPORT, &hits);
            let presence = local_presence(input).follow(&page_model, &PresenceIntent::View { view: next.clone(), viewport: NOMINAL_VIEWPORT });
            Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit: Emit { ui_scope: semio_plugin::kernel::UiDirtyScope::Full, ..Default::default() },
                ephemeral: EphemeralEmit { transient: vec![PdfViewTransientMutation::SetView(set_view(&next))], presence, ..Default::default() },
            })
        })();
        settled.map_err(semio_plugin::__job::OriginalTurnFailure::from)
    }
}

fn local_presence<E: ArtifactEditor + 'static>(input: &ArtifactCommandInputs<'_, EditorApp<E>>) -> E::Presence {
    input.context.and_then(|context| context.presence_view()).map(|view| view.local.clone()).unwrap_or_default()
}

/// 👥️ One presence tool run: the collaborator's pointer or caret becomes the next presence, published on the presence lane only; the document and the view are never touched.
pub struct PresenceCommandWork<E: ArtifactEditor> {
    tool_id: &'static str,
    consumed: bool,
    lens: PageLens<E::Snapshot>,
    editor: PhantomData<fn() -> E>,
}

impl<E: ArtifactEditor> PresenceCommandWork<E> {
    pub fn new(tool_id: &'static str, lens: PageLens<E::Snapshot>) -> Self {
        Self { tool_id, consumed: false, lens, editor: PhantomData }
    }
}

impl<E> ArtifactCommandWork<EditorApp<E>> for PresenceCommandWork<E>
where
    E: ArtifactEditor + 'static,
    E::Command: ViewCarrier,
    E::Presence: PresenceLane<Mutation = E::PresenceMutation>,
{
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &E::Command, _snapshot: &E::Snapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<E>>>) -> Option<usize> {
        command.view_edit().is_some_and(|(action, _)| is_presence_action(action)).then_some(1)
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn reports_progress(&self) -> bool {
        false
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<E>>, _cx: &mut semio_plugin::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<E>>, semio_plugin::__job::OriginalTurnFailure> {
        let settled: Result<_, Fault> = (|| {
            if std::mem::replace(&mut self.consumed, true) {
                return Err(Fault::from("pdf-presence-work-repeated"));
            }
            let (action, payload) = input.command.view_edit().ok_or_else(|| Fault::from("pdf-presence-route-mismatch"))?;
            let value: serde_json::Value = serde_json::from_str(payload).map_err(|_| Fault::from("pdf-presence-payload-invalid"))?;
            let page_model = (self.lens)(input.snapshot)?;
            let intent = presence_intent(action, &value, page_model.pages.len()).map_err(page::page_fault)?.ok_or_else(|| Fault::from("pdf-presence-route-mismatch"))?;
            let presence = local_presence(input).follow(&page_model, &intent);
            Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit: Emit::default(), ephemeral: EphemeralEmit { presence, ..Default::default() } })
        })();
        settled.map_err(semio_plugin::__job::OriginalTurnFailure::from)
    }
}

/// 🧵️ The work of `tool_id` over a page model reached through `lens`: the view work for a view tool, `fallback` for every other tool.
pub fn lifted_view_work<E>(tool_id: &'static str, lens: PageLens<E::Snapshot>, fallback: impl FnOnce(&'static str) -> Box<dyn ArtifactCommandWork<EditorApp<E>>>) -> Box<dyn ArtifactCommandWork<EditorApp<E>>>
where
    E: ArtifactEditor<Transient = PdfViewTransient, TransientMutation = PdfViewTransientMutation> + 'static,
    E::Command: ViewCarrier,
    E::Presence: PresenceLane<Mutation = E::PresenceMutation>,
{
    if is_view_action(tool_id) {
        Box::new(ViewCommandWork::<E>::new(tool_id, lens))
    } else if is_presence_action(tool_id) {
        Box::new(PresenceCommandWork::<E>::new(tool_id, lens))
    } else {
        fallback(tool_id)
    }
}

/// 🧵️ [`lifted_view_work`] for an editor whose snapshot is the page model.
pub fn view_work<E>(tool_id: &'static str, fallback: impl FnOnce(&'static str) -> Box<dyn ArtifactCommandWork<EditorApp<E>>>) -> Box<dyn ArtifactCommandWork<EditorApp<E>>>
where
    E: ArtifactEditor<Snapshot = PdfSnapshot, Transient = PdfViewTransient, TransientMutation = PdfViewTransientMutation> + 'static,
    E::Command: ViewCarrier,
    E::Presence: PresenceLane<Mutation = E::PresenceMutation>,
{
    lifted_view_work::<E>(tool_id, own_page, fallback)
}

/// 🧵️ [`view_work`] over the shared bounded native edit work, for editors with no further native work.
pub fn bounded_view_work<E>(tool_id: &'static str) -> Box<dyn ArtifactCommandWork<EditorApp<E>>>
where
    E: BoundedNativeEditingEditor<Snapshot = PdfSnapshot, Transient = PdfViewTransient, TransientMutation = PdfViewTransientMutation> + 'static,
    E::Command: ViewCarrier,
    E::Presence: PresenceLane<Mutation = E::PresenceMutation>,
{
    view_work::<E>(tool_id, bounded_native_edit_work::<E>)
}
/// 🔌️ The page window painter an editor's main window supplies: snapshot, selected object, view, viewport, highlights.
pub type MainRender = fn(&PdfSnapshot, Option<&str>, &ViewState, (f64, f64), Vec<OverlayLayer>, Locale) -> UiAssemblyResult<semio_ui_contract::BuiltNode>;

/// 🖼️ Renders the bodies every PDF editor shares (page window through the view, thumbnails, text layer, inspector); `None` for any other body.
pub fn render_page_body(body_key: &str, doc: &ArtifactView<'_, PdfSnapshot>, controller: &str, interaction: Option<&semio_plugin::app::InteractionView<'_>>, transient: Option<&PdfViewTransient>, locale: Locale, main: MainRender) -> Option<UiAssemblyResult<ComponentTree>> {
    render_page_body_of(body_key, doc.snapshot, &|| semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc), controller, interaction, transient, locale, main)
}

/// 🖼️ [`render_page_body`] over a page model the caller already holds, with the publication revision asked for only when a body needs it.
#[allow(clippy::too_many_arguments)]
pub fn render_page_body_of(body_key: &str, snapshot: &PdfSnapshot, revision: &dyn Fn() -> UiAssemblyResult<semio_plugin::UiPublicationRevision>, controller: &str, interaction: Option<&semio_plugin::app::InteractionView<'_>>, transient: Option<&PdfViewTransient>, locale: Locale, main: MainRender) -> Option<UiAssemblyResult<ComponentTree>> {
    render_page_body_peers(body_key, snapshot, revision, controller, interaction, transient, locale, main, &|interaction| interaction_samples(interaction, page::OBJECT_DOMAIN, &page::object_page_of(snapshot)))
}

/// 👥️ [`render_page_body_of`] with the collaborators supplied by `samples` from the interaction view; editors that receive typed presence pass [`typed_samples`] here.
#[allow(clippy::too_many_arguments)]
pub fn render_page_body_peers(body_key: &str, snapshot: &PdfSnapshot, revision: &dyn Fn() -> UiAssemblyResult<semio_plugin::UiPublicationRevision>, controller: &str, interaction: Option<&semio_plugin::app::InteractionView<'_>>, transient: Option<&PdfViewTransient>, locale: Locale, main: MainRender, samples: &dyn Fn(&semio_plugin::app::InteractionView<'_>) -> Vec<PeerSample>) -> Option<UiAssemblyResult<ComponentTree>> {
    let selected = interaction.and_then(|interaction| interaction.selection(page::OBJECT_DOMAIN).ids.first().map(String::as_str));
    let peers = || interaction.map_or_else(PeerBoard::default, |interaction| PeerBoard::of_samples(samples(interaction)));
    let built = match body_key {
        page::BODY_KEY => {
            let view = transient.map(view_state).unwrap_or_default();
            let hits = if view.query.is_empty() { Vec::new() } else { search(snapshot, &view.query, view.match_case, &mut SearchControl { cancelled: &|| false, progress: &mut |_, _| {} }).hits };
            let mut layers = hit_layers(snapshot, &hits, view.active);
            layers.extend(page::peer_overlay_layers(snapshot, &peers().overlay(0, view.page as u32)));
            main(snapshot, selected, &view, NOMINAL_VIEWPORT, layers, locale)
        }
        page::THUMBNAILS_BODY_KEY => {
            let overlay = peers().overlay(0, 0);
            page::render_thumbnail_strip_marked(snapshot, controller, page::OBJECT_DOMAIN, locale, &|page| overlay.on_page(page as u32).iter().map(|mark| mark.caption(locale)).collect())
        }
        page::TEXT_LAYER_BODY_KEY => page::render_text_layer(snapshot, controller, locale),
        page::INSPECTOR_BODY_KEY => revision().and_then(|revision| page::render_inspector(snapshot, selected, controller, revision, locale)),
        _ => return None,
    };
    Some(built.map(built_to_component_tree))
}

/// 🫧️ Declares the two stores a transient lane needs inside an `ArtifactEditor` impl.
#[macro_export]
macro_rules! view_lane_stores {
    () => {
        fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn semio_plugin::plugin_app_close_prelude::store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
            Some(semio_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
        }
        fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn semio_plugin::plugin_app_close_prelude::store::SnapshotRetirementFactory<Self::Transient>>> {
            Some(semio_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
        }
    };
}
//#endregion 🧵️ViewWork
