//! 📚️ The example-picker predicate, driven by the language-agnostic
//! `🧫️fixtures/📚️example-picker.json` every implementation answers — the TypeScript twin
//! `examplesForDialect` reads the SAME rows in `🔬️engine-contract/🟦️.ts`.

use super::*;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ExamplePickerFixture {
    examples: Vec<FixtureExample>,
    cases: Vec<FixtureCase>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FixtureExample {
    id: String,
    dialect: ArtifactDialect,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FixtureCase {
    name: String,
    app: FixtureApp,
    expected: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FixtureApp {
    id: String,
    role: AppRole,
    dialect: ArtifactDialect,
}

fn fixture() -> ExamplePickerFixture {
    serde_json::from_str(include_str!("../../🧫️fixtures/📚️example-picker.json")).expect("the example-picker fixture must parse")
}

/// 👁️✏️ An example belongs to a DIALECT, so an editor and its viewer resolve exactly the same
/// picker, a sibling subset resolves none of it, and a dialect with no authored example shows no
/// picker at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn every_surface_of_a_dialect_resolves_the_same_example_picker() {
    let fixture = fixture();
    let examples: Vec<ExampleDefinition> = fixture
        .examples
        .iter()
        .map(|row| ExampleDefinition { id: row.id.clone(), label: LocalizedLabel::data(&row.id), icon_id: IconName::from("file"), artifact_json: String::new(), dialect: row.dialect.clone() })
        .collect();
    for case in &fixture.cases {
        // 🪪️ The fixture's app id must be the derived surface id, so a row can never claim a
        // dialect/role pair its own id contradicts.
        assert_eq!(surface_app_id(&case.app.dialect, case.app.role), case.app.id, "{}: app id must derive from its dialect and role", case.name);
        let resolved: Vec<&str> = examples_for_dialect(&examples, &case.app.dialect).into_iter().map(|example| example.id.as_str()).collect();
        assert_eq!(resolved, case.expected.iter().map(String::as_str).collect::<Vec<&str>>(), "{}", case.name);
    }
    let editor = fixture.cases.iter().find(|case| case.name == "editor-of-the-dialect-offers-every-example-of-it").expect("editor case");
    let viewer = fixture.cases.iter().find(|case| case.name == "viewer-of-the-same-dialect-offers-exactly-the-same-picker").expect("viewer case");
    assert_eq!(editor.expected, viewer.expected, "the read-only surface offers exactly what its editor does");
    assert_ne!(editor.app.id, viewer.app.id, "and it is a different surface while doing so");
}
