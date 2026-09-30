use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧫️dialog-choices/🔣️.json")).expect("dialog-choices fixture")
}

fn fixture_dialog(fixture: &serde_json::Value) -> DialogDefinition {
    serde_json::from_value(fixture["dialog"].clone()).expect("fixture dialog deserializes")
}

#[test]
fn each_button_is_gated_on_its_own_required_args_and_dispatches_the_fixture_args() {
    let fixture = fixture();
    let dialog = fixture_dialog(&fixture);
    assert_eq!(dialog.validate_choices(), Ok(()));
    let seed = DslValue::from(fixture["seed"].clone());
    for case in fixture["cases"].as_array().expect("cases") {
        let name = case["case"].as_str().expect("case");
        let effective = effective_action_args(&dialog.args, &DslValue::from(case["staged"].clone()), Some(&seed));
        let enabled = |button: &str| match button.split_once(':') {
            Some(("choice", id)) => dialog.choices.iter().find(|choice| choice.id == id).expect("fixture choice").unresolved_args(&dialog.args, &effective).is_empty(),
            _ if button == "submit" => unresolved_action_args(&dialog.args, &effective).is_empty(),
            _ => true,
        };
        for (list, expected) in [("enabled", true), ("disabled", false)] {
            for button in case[list].as_array().expect("buttons") {
                assert_eq!(enabled(button.as_str().expect("button")), expected, "{name}: {button} {list}");
            }
        }
        for dispatch in case["dispatches"].as_array().expect("dispatches") {
            let button = dispatch["button"].as_str().expect("button");
            assert!(enabled(button), "{name}: {button} dispatches only while enabled");
            let (action, args) = match button.split_once(':') {
                Some(("choice", id)) => {
                    let choice = dialog.choices.iter().find(|choice| choice.id == id).expect("fixture choice");
                    (choice.action.as_str().to_string(), Some(choice.dispatch_args(&dialog.args, &effective)))
                }
                _ if button == "submit" => (dialog.submit_action.as_str().to_string(), Some(effective.clone())),
                _ => (dialog.cancel_action.as_ref().expect("cancel action").as_str().to_string(), None),
            };
            assert_eq!(action, dispatch["action"].as_str().expect("action"), "{name}: {button}");
            assert_eq!(args.map(|args| serde_json::to_value(args).expect("args wire")).unwrap_or(serde_json::Value::Null), dispatch["args"], "{name}: {button}");
        }
    }
}

#[test]
fn a_choice_requiring_an_arg_is_gated_on_it_and_sends_it() {
    let fixture = fixture();
    let mut dialog = fixture_dialog(&fixture);
    dialog.choices[0] = dialog.choices[0].clone().requires(["name"]);
    assert_eq!(dialog.validate_choices(), Ok(()));
    let seed = DslValue::from(fixture["seed"].clone());
    let cleared = effective_action_args(&dialog.args, &DslValue::from(serde_json::json!({ "name": "" })), Some(&seed));
    assert_eq!(dialog.choices[0].unresolved_args(&dialog.args, &cleared), vec!["name".to_string()]);
    let named = effective_action_args(&dialog.args, &DslValue::from(serde_json::json!({ "name": "Variant B" })), Some(&seed));
    assert_eq!(serde_json::to_value(dialog.choices[0].dispatch_args(&dialog.args, &named)).expect("args wire"), serde_json::json!({ "generation": 7, "name": "Variant B", "choice": "overwrite" }));
}

#[test]
fn a_dialog_round_trips_its_choices_and_omits_them_when_it_has_none() {
    let fixture = fixture();
    let dialog = fixture_dialog(&fixture);
    let choice = &dialog.choices[0];
    assert_eq!((choice.tone, choice.destructive), (semio_framework_ui_contract::Tone::Danger, true));
    assert_eq!(serde_json::to_value(&dialog).expect("dialog wire")["choices"], fixture["dialog"]["choices"]);
    let plain = DialogDefinition::new("plain", LocalizedLabel::native("Plain", "Schlicht"), ActionRef::new("ok"));
    assert!(serde_json::to_value(&plain).expect("plain wire").get("choices").is_none());
    let neutral = DialogChoice::new("keep", LocalizedLabel::native("Keep", "Behalten"), ActionRef::new("keep"));
    let wire = serde_json::to_value(&neutral).expect("choice wire");
    assert!(wire.get("tone").is_none() && wire.get("destructive").is_none() && wire.get("description").is_none() && wire.get("requires").is_none());
    let value = dialog.to_value();
    assert_eq!(DialogDefinition::from_value(value).expect("value round trip"), dialog);
}

#[test]
fn a_dialog_breaking_the_choice_law_is_refused_with_the_fixture_reason() {
    let fixture = fixture();
    let base = fixture_dialog(&fixture);
    for row in fixture["invalid"].as_array().expect("invalid rows") {
        let mut dialog = base.clone();
        dialog.choices = row["choiceIds"].as_array().expect("choice ids").iter().map(|id| DialogChoice::new(id.as_str().expect("id"), LocalizedLabel::native("Choice", "Wahl"), ActionRef::new("historyEditCommit"))).collect();
        dialog.args = row["argIds"].as_array().expect("arg ids").iter().map(|id| ActionArgDef { id: id.as_str().expect("id").to_string(), ..base.args[0].clone() }).collect();
        let requires: Vec<String> = row["requires"].as_array().map(|ids| ids.iter().map(|id| id.as_str().expect("id").to_string()).collect()).unwrap_or_default();
        dialog.choices = dialog.choices.into_iter().map(|choice| choice.requires(requires.clone())).collect();
        assert_eq!(dialog.validate_choices(), Err(row["reason"].as_str().expect("reason").to_string()), "{}", row["case"]);
    }
}
