//! 🏷️ The app labels overlay, driven by the language-agnostic `🧫️fixtures/🏷️app-labels-overlay.json` the TypeScript twin reads too.

use super::*;

#[derive(serde::Deserialize)]
struct OverlayCase {
    name: String,
    kind: String,
    id: String,
    fallback: String,
    expected: String,
}

#[derive(serde::Deserialize)]
struct OverlayFixture {
    overlay: AppLabelsOverlay,
    cases: Vec<OverlayCase>,
}

/// 🗣️ Every case of the shared corpus resolves to the same label React gives it, and an overlay with omitted maps parses with those maps empty.
#[test]
fn the_overlay_resolves_every_shared_case_like_react() {
    let fixture: OverlayFixture = serde_json::from_str(include_str!("../🧫️fixtures/🏷️app-labels-overlay.json")).expect("app-labels-overlay fixture");
    assert!(fixture.overlay.group_labels.is_empty());
    for case in &fixture.cases {
        let kind = match case.kind.as_str() {
            "example" => AppLabelKind::Example,
            "action" => AppLabelKind::Action,
            "dialog" => AppLabelKind::Dialog,
            other => panic!("{}: unknown kind {other}", case.name),
        };
        assert_eq!(fixture.overlay.resolve(kind, &case.id, &case.fallback), case.expected, "{}", case.name);
    }
}
