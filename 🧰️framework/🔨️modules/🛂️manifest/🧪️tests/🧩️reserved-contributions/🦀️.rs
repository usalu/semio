//! 🧩️ The reserved-contribution registration point honours every non-default contribution: app actions in contribution
//! order, window actions computed from the built app, editor dialogs, and the first fixed audience that owns an id.

use super::*;

fn probe_actions() -> Vec<ActionDefinition> {
    vec![ActionDefinition::resumable_framework("probeApp", LocalizedLabel::data("Probe"), ActionKind::History, "clock")]
}

fn second_actions() -> Vec<ActionDefinition> {
    vec![ActionDefinition::resumable_framework("secondApp", LocalizedLabel::data("Second"), ActionKind::History, "clock")]
}

fn probe_window_actions(app: &AppDefinition) -> Vec<ActionDefinition> {
    app.utilities.iter().map(|utility| ActionDefinition::resumable_framework(format!("probe.{}", utility.id), LocalizedLabel::data("Probe window"), ActionKind::View, "eye")).collect()
}

fn probe_dialogs() -> Vec<DialogDefinition> {
    vec![DialogDefinition::new("probeDialog", LocalizedLabel::data("Probe"), ActionRef::new("probeApp"))]
}

fn probe_audience(action_id: &str) -> Option<CapabilityAudience> {
    (action_id == "probeChrome").then_some(CapabilityAudience::Chrome)
}

fn shadowed_audience(action_id: &str) -> Option<CapabilityAudience> {
    (action_id == "probeChrome").then_some(CapabilityAudience::Input)
}

const PROBE: ReservedContribution = ReservedContribution::new("probe").with_app_actions(probe_actions).with_window_actions(probe_window_actions).with_editor_dialogs(probe_dialogs).with_fixed_audience(probe_audience);
const SECOND: ReservedContribution = ReservedContribution::new("second").with_app_actions(second_actions).with_fixed_audience(shadowed_audience);

fn ids(actions: Vec<ActionDefinition>) -> Vec<String> {
    actions.into_iter().map(|action| action.id).collect()
}

/// 🌱️ A contribution built with `new` alone reserves nothing and fixes no audience.
#[semio_framework_async_macros::async_test]
async fn an_empty_contribution_reserves_nothing() {
    let empty = [ReservedContribution::new("empty")];
    let app = crate::testkit::app_with(vec![], vec![]).await;
    assert!(reserved_app_actions(&empty).is_empty());
    assert!(reserved_window_actions(&empty, &app).is_empty());
    assert!(reserved_editor_dialogs(&empty).is_empty());
    assert_eq!(reserved_fixed_audience(&empty, "probeChrome"), None);
}

/// 🕹️ App actions follow the order the host passes the contributions in.
#[test]
fn app_actions_follow_contribution_order() {
    assert_eq!(ids(reserved_app_actions(&[PROBE, SECOND])), ["probeApp", "secondApp"]);
    assert_eq!(ids(reserved_app_actions(&[SECOND, PROBE])), ["secondApp", "probeApp"]);
}

/// 🪟️ Window actions are computed from the built app the host passes.
#[semio_framework_async_macros::async_test]
async fn window_actions_are_computed_from_the_built_app() {
    let mut app = crate::testkit::app_with(vec![], vec![]).await;
    assert!(reserved_window_actions(&[PROBE, SECOND], &app).is_empty());
    app.utilities = vec![UtilityDefinition::new("brush", LocalizedLabel::data("Brush"), "paintbrush"), UtilityDefinition::new("eraser", LocalizedLabel::data("Eraser"), "eraser")];
    assert_eq!(ids(reserved_window_actions(&[PROBE, SECOND], &app)), ["probe.brush", "probe.eraser"]);
}

/// 🗳️ Editor dialogs come from every contribution that reserves one.
#[test]
fn editor_dialogs_come_from_every_contribution() {
    let dialogs = reserved_editor_dialogs(&[SECOND, PROBE]);
    assert_eq!(dialogs.iter().map(|dialog| dialog.id.as_str()).collect::<Vec<_>>(), ["probeDialog"]);
}

/// 🧭️ The first contribution that owns an id fixes its audience, and a declaration that resolves elsewhere violates it.
#[test]
fn the_first_owning_contribution_fixes_the_audience() {
    assert_eq!(reserved_fixed_audience(&[PROBE, SECOND], "probeChrome"), Some(CapabilityAudience::Chrome));
    assert_eq!(reserved_fixed_audience(&[SECOND, PROBE], "probeChrome"), Some(CapabilityAudience::Input));
    assert_eq!(reserved_fixed_audience(&[PROBE, SECOND], "probeApp"), None);
    let mut toggle = ActionDefinition::new("probeChrome", LocalizedLabel::data("Toggle"), ActionKind::View, "eye");
    toggle.in_palette = false;
    assert_eq!(reserved_audience_violation(&[PROBE], &toggle), None, "chrome derives from an off-palette view verb");
    assert_eq!(reserved_audience_violation(&[SECOND], &toggle), Some(CapabilityAudience::Input));
    toggle.in_palette = true;
    assert_eq!(reserved_audience_violation(&[PROBE], &toggle), Some(CapabilityAudience::Chrome), "a palette view verb derives the agent audience");
    toggle.semantics.audience = Some(CapabilityAudience::Chrome);
    assert_eq!(reserved_audience_violation(&[PROBE], &toggle), None, "a declared audience overrides the derivation");
}
