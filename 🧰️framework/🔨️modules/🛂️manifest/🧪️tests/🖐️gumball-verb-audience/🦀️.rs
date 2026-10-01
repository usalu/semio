//! 🖐️ The gumball-verb audience rule, driven by the language-agnostic `🧫️fixtures/🖐️gumball-verb-audience.json`: the
//! handle toggle is window chrome, and the retired drag brackets are ordinary ids the rule leaves untouched.

use super::*;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GumballFixture {
    #[allow(dead_code)]
    why: String,
    cases: Vec<GumballCase>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GumballCase {
    name: String,
    action: GumballAction,
    expected: Option<CapabilityAudience>,
    violation: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GumballAction {
    id: String,
    kind: ActionKind,
    in_palette: bool,
    audience: Option<CapabilityAudience>,
}

/// 🖐️ `framework_fixed_audience` and its violation predicate answer every fixture row exactly.
#[test]
fn gumball_verb_audience_matches_the_language_agnostic_fixture() {
    let fixture: GumballFixture = serde_json::from_str(include_str!("../../🧫️fixtures/🖐️gumball-verb-audience.json")).expect("gumball fixture parses");
    assert!(fixture.cases.len() >= 8, "the fixture keeps the toggle, both retired brackets and the untouched ids");
    for case in fixture.cases {
        let mut action = ActionDefinition::new(case.action.id.clone(), LocalizedLabel::data(case.action.id.clone()), case.action.kind, "move").with_in_palette(case.action.in_palette);
        action.semantics.audience = case.action.audience;
        assert_eq!(framework_fixed_audience(&action.id), case.expected, "{}", case.name);
        assert_eq!(framework_fixed_audience_violation(&action).is_some(), case.violation, "{}", case.name);
    }
}
