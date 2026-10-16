//! 🛠️ Small runtime helpers the instance family names: turn grants, render section, addressed view, internal fault.

use super::*;
// 🚫️async: E1 pure error constructor consumed pervasively by sync closures
// (`.map_err`/`.ok_or_else`) — see R9.
pub(super) fn plugin_internal_fault(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), message)
}
pub(crate) fn debug_runtime_line(line: impl std::fmt::Display) {
    if semio_framework_trace::runtime_diagnostics_enabled() {
        eprintln!("{line}");
    }
}
pub(super) fn addressed_action_view(view: &ViewModel, address: &semio_framework::manifest::ActionAddress) -> Result<ViewModel, Fault> {
    let projected = view.for_window_instance(&address.window_instance_id).ok_or_else(|| plugin_internal_fault(format!("unknown action window instance {}", address.window_instance_id)))?;
    if projected.active_window_kind_id.as_deref() != Some(address.window_kind_id.as_str()) {
        return Err(plugin_internal_fault(format!("action window instance {} has a different kind than {}", address.window_instance_id, address.window_kind_id)));
    }
    Ok(projected)
}
/// 🎯️ Window projection for a catalog action, or the unprojected view for a framework-reserved verb.
/// Retained actor ingress (`AppCommand::Command`) uses this rule so undo/redo
/// reach `dispatch_framework_reserved_action` when the chrome addresses a window that does not
/// declare the verb (or is absent from `window_instances`).
pub(crate) fn admit_addressed_action_view(view: &ViewModel, address: &semio_framework::manifest::ActionAddress) -> Result<ViewModel, Fault> {
    match addressed_action_view(view, address) {
        Ok(projected) => Ok(projected),
        Err(_) if crate::reserved_jobs::is_framework_reserved_action_id(&address.action_id) => Ok(view.clone()),
        Err(fault) => Err(fault),
    }
}
// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): return type flipped from
// `UiNode` to `ui_runtime::ComponentTree`, matching `PluginApp::render`/`ArtifactApp::render` —
// this is the exact boundary `⚛️reactor/🦀️.rs`'s `poll()` calls into before handing the
// tree to `PatchTracker::diff`.
/// 🧩️ Canonical JSON of one reserved refresh section — key-ordered through a `BTreeMap` because the
/// accessors return `HashMap`s whose iteration order would otherwise change the retained text
/// leaves (and therefore the surface hash) on every render without the section itself changing.
// 🚫️async: E1 pure serialization consumed synchronously by `plugin_render_section` — see R9.
fn canonical_section_json<T: Serialize>(section: &HashMap<String, T>) -> String {
    let ordered: std::collections::BTreeMap<&str, &T> = section.iter().map(|(key, value)| (key.as_str(), value)).collect();
    serde_json::to_string(&ordered).unwrap_or_else(|_| String::from("{}"))
}
/// 🧩️ Renders one reserved section surface. A section body key never reaches `PluginApp::render`:
/// it names the object-safe accessor (`window_engagements`/`window_measures`/`tool_measures`/
/// `app_catalogue_json`) whose
/// canonical JSON `crate::app::section_component_tree` carries as retained text leaves, so the
/// shell's `SET_WINDOW_MEASURES_BY_WINDOW_ID`/`SET_TOOL_MEASURES_BY_TOOL_ID`/
/// `SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID` reducers receive the same shapes the deleted JSON
/// `refreshUi` wire produced.
// 🚫️async: E5 executor bridge — the accessors stay genuinely `async fn`; resolved synchronously
// here for the same reason `plugin_render_surface`'s own `instance.app.render` call is.
/// 🛍️ App-STATIC and view-independent: the catalogue is the app's own registered operator set,
/// already canonical JSON authored by the app, so it needs neither `view_state` nor the
/// `HashMap` key-ordering pass the three window-scoped sections above go through.
pub(crate) fn original_render_section<PA: PluginApp>(app: &mut PA, section: semio_framework::UiRefreshSection, view_state: &ViewModel) -> Result<ComponentTree, Fault> {
    let payload = match section {
        semio_framework::UiRefreshSection::Engagements => canonical_section_json(&::semio_framework_async::poll::resolve_ready(app.window_engagements(view_state))),
        semio_framework::UiRefreshSection::Measures => canonical_section_json(&::semio_framework_async::poll::resolve_ready(app.window_measures(view_state))),
        semio_framework::UiRefreshSection::Tools => canonical_section_json(&::semio_framework_async::poll::resolve_ready(app.tool_measures(view_state))),
        semio_framework::UiRefreshSection::Catalogue => ::semio_framework_async::poll::resolve_ready(app.app_catalogue_json()),
    };
    crate::window_surface::section_component_tree(section, &payload).map_err(|error| plugin_internal_fault(error.to_string()))
}
/// 🧮️ Intersects two original authorities independently without creating new currency.
pub(crate) fn original_plugin_turn_grant(policy:RetainedCloneGrant,remaining:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:policy.maximum_items.min(remaining.maximum_items),maximum_copy_bytes:policy.maximum_copy_bytes.min(remaining.maximum_copy_bytes),maximum_capacity_bytes:policy.maximum_capacity_bytes.min(remaining.maximum_capacity_bytes),maximum_release_bytes:policy.maximum_release_bytes.min(remaining.maximum_release_bytes),maximum_depth:policy.maximum_depth.min(remaining.maximum_depth)}}
/// 🧾️ What one host turn still owns after its units spent `spent`: every consumed axis shrinks, nesting depth is not consumed.
pub(crate) fn remaining_plugin_turn_grant(turn:RetainedCloneGrant,spent:RetainedCloneProgress)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:turn.maximum_items.saturating_sub(spent.copied_items),maximum_copy_bytes:turn.maximum_copy_bytes.saturating_sub(spent.copied_bytes),maximum_capacity_bytes:turn.maximum_capacity_bytes.saturating_sub(spent.retained_capacity_bytes),maximum_release_bytes:turn.maximum_release_bytes.saturating_sub(spent.released_bytes),maximum_depth:turn.maximum_depth}}

/// 🐞️ `[TRACE]` last maintenance stage entered — temporary, ticket 26/09/02/PUZZLE-3D-END-TO-END.
pub static LAST_MAINTENANCE_STAGE: AtomicU64 = AtomicU64::new(0);
