//! 🧭️ Presence intents: how a collaborator's own view, pointer and caret become the next [`PdfPresence`]. Pure and total; the editors publish the result on the presence lane only and the document is never touched.

use crate::PdfPresence;
use semio_s_artifact_stdio_pdf_page_editor_view::{page_size, resolve_zoom, ViewState};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;

//#region 🧭️Intent
/// 🎬️ The presence action ids: pointer over a page, pointer leaving it, text caret placement and caret removal.
pub const PRESENCE_ACTION_IDS: [&str; 4] = ["presence-pointer", "presence-pointer-leave", "presence-caret", "presence-caret-clear"];

/// 🪟️ The presence action definitions with en and de labels, for the windows that report the pointer and the caret.
pub fn presence_action_definitions() -> Vec<semio_plugin::ActionDefinition> {
    let labels = [("Show Pointer to Peers", "Zeiger für andere zeigen"), ("Hide Pointer from Peers", "Zeiger für andere verbergen"), ("Show Caret to Peers", "Textcursor für andere zeigen"), ("Hide Caret from Peers", "Textcursor für andere verbergen")];
    PRESENCE_ACTION_IDS.iter().zip(labels).map(|(id, (en, de))| {
        let args = match *id {
            "presence-pointer" => vec![semio_plugin::ActionArgDef::number("page", semio_ui_locale::LocalizedLabel::native("Page", "Seite")).required(), semio_plugin::ActionArgDef::number("x", semio_ui_locale::LocalizedLabel::native("Horizontal per mille", "Waagerecht in Promille")).required(), semio_plugin::ActionArgDef::number("y", semio_ui_locale::LocalizedLabel::native("Vertical per mille", "Senkrecht in Promille")).required()],
            "presence-caret" => vec![semio_plugin::ActionArgDef::text("object", semio_ui_locale::LocalizedLabel::native("Object", "Objekt")).min_length(1).required(), semio_plugin::ActionArgDef::number("offset", semio_ui_locale::LocalizedLabel::native("Offset", "Position")).required(), semio_plugin::ActionArgDef::number("anchor", semio_ui_locale::LocalizedLabel::native("Anchor", "Anker"))],
            _ => Vec::new(),
        };
        let mut definition = semio_plugin::ActionDefinition::bounded_catalog(*id, semio_ui_locale::LocalizedLabel::native(en, de), semio_plugin::ActionKind::View).with_args(args).with_in_palette(false);
        definition.semantics.execution.interactive_job = semio_plugin::InteractiveJobClassification::Migrated;
        definition
    }).collect()
}

pub fn is_presence_action(action: &str) -> bool {
    PRESENCE_ACTION_IDS.contains(&action)
}

/// 🎬️ Every change a collaborator can make to what peers see of them.
#[derive(Clone, Debug, PartialEq)]
pub enum PresenceIntent {
    View { view: ViewState, viewport: (f64, f64) },
    Pointer { page: u32, x: u32, y: u32 },
    PointerLeave,
    Caret { object: String, offset: u32, anchor: u32 },
    CaretClear,
}

/// 🎬️ The intent an action id and its JSON payload stand for.
pub fn presence_intent(action: &str, payload: &serde_json::Value) -> Option<PresenceIntent> {
    let whole = |key: &str| payload[key].as_f64().map_or(0, |value| value.max(0.0).round() as u32);
    let number = |key: &str| payload[key].as_f64().map_or(0, |value| value.round().clamp(0.0, 1000.0) as u32);
    Some(match action {
        "presence-pointer" => PresenceIntent::Pointer { page: payload["page"].as_u64().unwrap_or(0) as u32, x: number("x"), y: number("y") },
        "presence-pointer-leave" => PresenceIntent::PointerLeave,
        "presence-caret" => PresenceIntent::Caret { object: payload["object"].as_str()?.to_string(), offset: whole("x"), anchor: whole("y") },
        "presence-caret-clear" => PresenceIntent::CaretClear,
        _ => return None,
    })
}

/// 🔭️ The visible rectangle of the view's page as per mille of the page, `[left, top, right, bottom]`: the camera is the page centre, so the visible fraction is the viewport over the zoomed page, centred.
pub fn visible_rect(snapshot: &PdfSnapshot, view: &ViewState, viewport: (f64, f64)) -> [u32; 4] {
    let (width, height) = page_size(snapshot, view.page).unwrap_or((1.0, 1.0));
    let zoom = resolve_zoom(snapshot, view, viewport);
    let fraction = |extent: f64, page: f64| ((extent / zoom) / page.max(1.0)).clamp(0.0, 1.0);
    let edge = |fraction: f64| (((1.0 - fraction) / 2.0) * 1000.0).round() as u32;
    let (fx, fy) = (fraction(viewport.0, width), fraction(viewport.1, height));
    [edge(fx), edge(fy), 1000 - edge(fx), 1000 - edge(fy)]
}

/// 🧭️ The presence after `intent`. A page change drops the pointer, which belongs to the page it was on.
pub fn reduce_presence(snapshot: &PdfSnapshot, current: &PdfPresence, intent: &PresenceIntent) -> PdfPresence {
    let mut next = current.clone();
    match intent {
        PresenceIntent::View { view, viewport } => {
            let page = view.page as u32;
            let [left, top, right, bottom] = visible_rect(snapshot, view, *viewport);
            if page != current.page {
                next.pointer_active = false;
            }
            next.page = page;
            (next.view_left, next.view_top, next.view_right, next.view_bottom) = (left, top, right, bottom);
        }
        PresenceIntent::Pointer { page, x, y } => (next.page, next.pointer_active, next.pointer_x, next.pointer_y) = (*page, true, (*x).min(1000), (*y).min(1000)),
        PresenceIntent::PointerLeave => next.pointer_active = false,
        PresenceIntent::Caret { object, offset, anchor } => (next.caret_object, next.caret_offset, next.caret_anchor) = (object.clone(), *offset, *anchor),
        PresenceIntent::CaretClear => (next.caret_object, next.caret_offset, next.caret_anchor) = (String::new(), 0, 0),
    }
    next
}

/// 🔌️ The mutation that makes the lane hold `presence`.
pub fn set_presence(presence: &PdfPresence) -> crate::SetPresence {
    crate::SetPresence { page: presence.page, pointer_active: presence.pointer_active, pointer_x: presence.pointer_x, pointer_y: presence.pointer_y, view_left: presence.view_left, view_top: presence.view_top, view_right: presence.view_right, view_bottom: presence.view_bottom, caret_object: presence.caret_object.clone(), caret_offset: presence.caret_offset, caret_anchor: presence.caret_anchor }
}

/// 🔭️ The presence a mutation of the lane leaves.
pub fn presence_of(leaf: &crate::SetPresence) -> PdfPresence {
    PdfPresence { page: leaf.page, pointer_active: leaf.pointer_active, pointer_x: leaf.pointer_x, pointer_y: leaf.pointer_y, view_left: leaf.view_left, view_top: leaf.view_top, view_right: leaf.view_right, view_bottom: leaf.view_bottom, caret_object: leaf.caret_object.clone(), caret_offset: leaf.caret_offset, caret_anchor: leaf.caret_anchor }
}

/// 🔌️ How an editor's presence type follows what its collaborator does. An editor without presence follows nothing, so the view and presence work serve every PDF editor through one bound.
pub trait PresenceLane: Sized {
    type Mutation;
    fn follow(&self, snapshot: &PdfSnapshot, intent: &PresenceIntent) -> Vec<Self::Mutation>;
}

impl PresenceLane for semio_plugin::NoPresence {
    type Mutation = semio_plugin::NoPresenceMutation;
    fn follow(&self, _snapshot: &PdfSnapshot, _intent: &PresenceIntent) -> Vec<Self::Mutation> {
        Vec::new()
    }
}

impl PresenceLane for PdfPresence {
    type Mutation = crate::PdfPresenceMutation;
    fn follow(&self, snapshot: &PdfSnapshot, intent: &PresenceIntent) -> Vec<Self::Mutation> {
        let next = reduce_presence(snapshot, self, intent);
        if next == *self { Vec::new() } else { vec![crate::PdfPresenceMutation::SetPresence(set_presence(&next))] }
    }
}
//#endregion 🧭️Intent

/// 👥️ Declares the presence lane's preparation store inside an `ArtifactEditor` impl; displaced roots and the close ladder ride the framework's bounded defaults.
#[macro_export]
macro_rules! presence_lane_stores {
    () => {
        fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn semio_plugin::plugin_app_close_prelude::store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
            Some(semio_plugin::bounded_transient_preparation_factory::<Self::Presence, Self::PresenceMutation>())
        }
    };
}
