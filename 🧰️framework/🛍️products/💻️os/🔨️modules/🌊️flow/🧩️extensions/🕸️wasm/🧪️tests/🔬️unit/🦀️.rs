
use super::*;
use neural_engine::{Atom, ChannelSpec, EvalError, Operator, OperatorImpl, Value, channel_output};

struct Echo;

impl Operator for Echo {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("x", input.clone()))
    }
}

#[test]
fn manifest_lists_catalogue() {
    let mut reg = Registry::new();
    reg.register_operator(
        OperatorInfo {
            id: "test.echo".into(),
            extension: "test".into(),
            name: "Echo".into(),
            abbreviation: "Echo".into(),
            icon: "emoji:📣️".into(),
            summary: "Echo".into(),
            inputs: vec![ChannelSpec::any("x")],
            outputs: vec![ChannelSpec::named("X", "x", "x", "Echoed")],
            ..Default::default()
        },
        vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }],
        &[],
    );
    let json = build_manifest_json("test", "Test", "0.1.0", &reg, vec!["onStartup".into()], vec![], vec![], vec![]);
    assert!(json.contains("flow.extension"));
    assert!(json.contains("test.echo"));
}

#[test]
fn evaluate_round_trips_dictionary() {
    let mut reg = Registry::new();
    reg.register_operator(
        OperatorInfo {
            id: "test.echo".into(),
            extension: "test".into(),
            name: "Echo".into(),
            abbreviation: "Echo".into(),
            icon: "emoji:📣️".into(),
            summary: "Echo".into(),
            inputs: vec![ChannelSpec::any("x")],
            outputs: vec![ChannelSpec::named("X", "x", "x", "Echoed")],
            ..Default::default()
        },
        vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }],
        &[],
    );
    let input = Dictionary::new().insert("number", Value::Atom(Atom::Decimal(2.0)));
    let out_json = evaluate_json(&reg, "test.echo", &crate::os_pack::json::to_json_string(&input));
    let out: Dictionary = crate::os_pack::json::from_json_str(&out_json).unwrap();
    assert_eq!(out.get("x").and_then(|v| v.as_dictionary()), Some(&input));
}

// #region 🔁️ExtensionInvocationWire
/// 🔁️ The `extension::invoke` "evaluate" wire, driven by the language-agnostic
/// `🧫️fixtures/🔁️extension-invocation-wire/🔣️.json` the host twin reads in
/// `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`. `outcome` is what the SHELL submits through
/// `PluginExtensionCompletion::complete`: an extension that answered — even with an `{"error": …}`
/// body — is an `ok` completion, and only a request this SDK could not decode is a `fault` one.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ExtensionInvocationWireFixture {
    capability: String,
    request_fields: Vec<String>,
    operator_id: String,
    output_channel: String,
    rows: Vec<ExtensionInvocationWireRow>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ExtensionInvocationWireRow {
    name: String,
    request_json: String,
    outcome: String,
    #[serde(default)]
    output_keys: Vec<String>,
    #[serde(default)]
    output_empty: bool,
    #[serde(default)]
    fault: Option<String>,
}

fn echo_registry(operator_id: &str, output_channel: &str) -> Registry {
    let mut registry = Registry::new();
    registry.register_operator(
        OperatorInfo {
            id: operator_id.into(),
            extension: "test".into(),
            name: "Echo".into(),
            abbreviation: "Echo".into(),
            icon: "emoji:📣️".into(),
            summary: "Echo".into(),
            inputs: vec![ChannelSpec::any(output_channel)],
            outputs: vec![ChannelSpec::named("X", output_channel, output_channel, "Echoed")],
            ..Default::default()
        },
        vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }],
        &[],
    );
    registry
}

#[test]
fn the_evaluate_wire_answers_every_fixture_row() {
    let fixture: ExtensionInvocationWireFixture = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).expect("the extension-invocation-wire fixture must parse");
    assert_eq!(fixture.capability, "evaluate");
    assert_eq!(fixture.request_fields, vec!["operatorId".to_string(), "inputJson".to_string()]);
    let registry = echo_registry(&fixture.operator_id, &fixture.output_channel);
    for row in &fixture.rows {
        match evaluate_invoke_json(&registry, row.request_json.as_bytes()) {
            Ok(bytes) => {
                assert_eq!(row.outcome, "ok", "{}", row.name);
                let text = String::from_utf8(bytes).expect("the answer is UTF-8 JSON");
                let answer: DslValue = crate::os_pack::json::from_json_str(&text).expect("the answer decodes");
                let DslValue::Object(entries) = &answer else { panic!("{} must answer an object", row.name) };
                let keys: Vec<String> = entries.iter().map(|(key, _)| key.clone()).collect();
                assert_eq!(keys, row.output_keys, "{}", row.name);
                let body_is_empty = matches!(entries.first().map(|(_, value)| value), Some(DslValue::Object(body)) if body.is_empty());
                assert_eq!(body_is_empty, row.output_empty, "{}", row.name);
            }
            Err(error) => {
                assert_eq!(row.outcome, "fault", "{}", row.name);
                assert_eq!(Some(error), row.fault.clone(), "{}", row.name);
            }
        }
    }
}
// #endregion 🔁️ExtensionInvocationWire
