mod declared_verb_verdict_tests {
    use super::*;

    const LANES: [&str; 15] = ["artifact", "config", "draft", "presence", "transient", "windowConfig", "windowTransient", "child", "interaction", "effect", "event", "ui", "download", "terminal", "fault"];

    /// 🧬️ Required bridge representatives obey every independently authored neutral value schema.
    #[test]
    fn declared_bridge_required_arguments_match_neutral_schemas() {
        use semio_framework::{ActionArgDef, ActionArgOption, DslValue};
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎮️declared-bridge-arguments.json")).unwrap();
        let policies: serde_json::Value = serde_json::from_str(include_str!("../../🧪️testing/⚖️declared-verb-verdicts/🧬️schema/🔣️.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let args = case["args"].as_array().unwrap().iter().map(|argument| {
                let mut schema = argument["schema"].clone();
                if let Some(options) = schema.get_mut("options").and_then(serde_json::Value::as_array_mut) {
                    for option in options { *option = serde_json::to_value(ActionArgOption::new(option.as_str().unwrap(), semio_framework_ui_locale::LocalizedLabel::native("Choice", "Auswahl"))).unwrap(); }
                }
                let mut arg = ActionArgDef::text(argument["id"].as_str().unwrap(), semio_framework_ui_locale::LocalizedLabel::native("Argument", "Argument"));
                arg.schema = serde_json::from_value(schema).unwrap();
                arg.required = argument["required"].as_bool().unwrap();
                arg.nullable = argument["nullable"].as_bool().unwrap_or(false);
                arg.default = argument.get("default").map(DslValue::from);
                arg
            }).collect::<Vec<_>>();
            let actual = declared_bridge_arguments(&args);
            if let Some(argument) = case["errorArgument"].as_str() {
                assert!(actual.unwrap_err().contains(argument), "{}", case["name"]);
            } else {
                let actual = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&actual.unwrap())).unwrap();
                assert_eq!(actual, case["expected"], "{}", case["name"]);
                let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(&policies["$defs"][case["name"].as_str().unwrap()].to_string()).unwrap();
                validator.validate_json(&actual.to_string()).unwrap();
                for invalid in case["invalid"].as_array().unwrap() { assert!(validator.validate_json(&invalid.to_string()).is_err(), "{} rejects {invalid}", case["name"]); }
            }
        }
    }

    fn outcome(value: &serde_json::Value) -> DeclaredVerbOutcome {
        let text = |value: &serde_json::Value, key: &str| value[key].as_str().expect("fixture outcome text").to_string();
        if let Some(refusal) = value.get("unreachable") {
            return DeclaredVerbOutcome::Unreachable { code: text(refusal, "code"), detail: text(refusal, "detail") };
        }
        if let Some(refusal) = value.get("refused") {
            return DeclaredVerbOutcome::Refused { code: text(refusal, "code"), detail: text(refusal, "detail") };
        }
        let settled = &value["settled"];
        let flag = |key: &str| settled[key].as_bool().expect("fixture outcome flag");
        DeclaredVerbOutcome::Settled(DeclaredVerbEffect {
            lanes: settled["lanes"].as_array().expect("fixture lanes").iter().map(|lane| *LANES.iter().find(|name| Some(**name) == lane.as_str()).expect("fixture lane is a published lane")).collect(),
            document_changed: flag("documentChanged"),
            document_replaced: flag("documentReplaced"),
            config_changed: flag("configChanged"),
            user_path_written: flag("userPathWritten"),
            host_effects: settled["hostEffects"].as_u64().expect("fixture host effects") as usize,
            fingerprint: settled["fingerprint"].as_u64().expect("fixture fingerprint"),
            orphaned_children: settled["orphanedChildren"].as_array().map_or_else(Vec::new, |children| children.iter().map(|child| child.as_str().expect("fixture orphaned child").to_string()).collect()),
        })
    }

    fn probe(value: &serde_json::Value) -> DeclaredVerbProbe {
        let kind = match value["kind"].as_str() {
            Some("mutation") => semio_framework::ActionKind::Mutation,
            Some("view") => semio_framework::ActionKind::View,
            Some("shell") => semio_framework::ActionKind::Shell,
            other => panic!("fixture kind {other:?}"),
        };
        let audience = match value["audience"].as_str() {
            Some("agent") => semio_framework::CapabilityAudience::Agent,
            Some("input") => semio_framework::CapabilityAudience::Input,
            Some("chrome") => semio_framework::CapabilityAudience::Chrome,
            other => panic!("fixture audience {other:?}"),
        };
        DeclaredVerbProbe {
            verb: value["verb"].as_str().expect("fixture verb").to_string(),
            kind,
            audience,
            destructive: value["destructive"].as_bool().expect("fixture destructive"),
            bridge: value["bridge"].as_str().map_or(Ok(()), |detail| Err(detail.to_string())),
            windows: value["windows"]
                .as_array()
                .expect("fixture windows")
                .iter()
                .map(|window| DeclaredVerbWindowProbe {
                    window: window["window"].as_str().expect("fixture window").to_string(),
                    staged: outcome(&window["staged"]),
                    arguments: window["arguments"]
                        .as_array()
                        .expect("fixture arguments")
                        .iter()
                        .map(|argument| DeclaredArgumentProbe {
                            argument: argument["argument"].as_str().expect("fixture argument").to_string(),
                            others_specified: argument["othersSpecified"].as_bool().expect("fixture othersSpecified"),
                            first: outcome(&argument["first"]),
                            second: outcome(&argument["second"]),
                        })
                        .collect(),
                })
                .collect(),
            agent: (!value["agent"].is_null()).then(|| outcome(&value["agent"])),
        }
    }

    fn finding_name(finding: &DeclaredVerbFinding) -> (&'static str, Option<&str>) {
        match finding {
            DeclaredVerbFinding::DocumentWriteFromNonMutation { .. } => ("documentWriteFromNonMutation", None),
            DeclaredVerbFinding::SilentMutation { .. } => ("silentMutation", None),
            DeclaredVerbFinding::IgnoredArgument { argument, .. } => ("ignoredArgument", Some(argument.as_str())),
            DeclaredVerbFinding::DestructiveWithoutDiscard { .. } => ("destructiveWithoutDiscard", None),
            DeclaredVerbFinding::Unreachable { .. } => ("unreachable", None),
            DeclaredVerbFinding::Unbridged { .. } => ("unbridged", None),
            DeclaredVerbFinding::AgentLaneDiverges { .. } => ("agentLaneDiverges", None),
            DeclaredVerbFinding::ComposedChildOrphaned { .. } => ("composedChildOrphaned", None),
        }
    }

    /// ⚖️ The Rust verdict answers every case of the language-agnostic fixture exactly as written.
    #[test]
    fn declared_verb_verdicts_match_the_language_agnostic_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚖️declared-verb-verdicts.json")).expect("declared-verb verdict fixture parses");
        let cases = fixture["cases"].as_array().expect("fixture cases");
        assert!(cases.len() >= 31, "the fixture keeps every rule's positive and negative case");
        for case in cases {
            let name = case["name"].as_str().expect("case name");
            let actual = declared_verb_findings(&probe(&case["probe"])).iter().map(|finding| finding_name(finding)).map(|(finding, argument)| (finding.to_string(), argument.map(str::to_string))).collect::<Vec<_>>();
            let expected = case["findings"].as_array().expect("case findings").iter().map(|finding| (finding["finding"].as_str().expect("finding name").to_string(), finding["argument"].as_str().map(str::to_string))).collect::<Vec<_>>();
            assert_eq!(actual, expected, "{name}");
        }
    }

    /// 🔀️ Every perturbable argument schema yields two distinct values, and a single-option choice none.
    #[test]
    fn declared_argument_alternatives_are_distinct_valid_values() {
        use semio_framework_ui_locale::LocalizedLabel;
        use semio_framework::{ActionArgDef, ActionArgOption};
        let label = || LocalizedLabel::native("Probe", "Probe");
        let two = ActionArgDef::select("kind", label(), vec![ActionArgOption::new("owns", label()), ActionArgOption::new("uses", label())]);
        let one = ActionArgDef::select("kind", label(), vec![ActionArgOption::new("owns", label())]);
        for argument in [two, ActionArgDef::text("name", label()), ActionArgDef::json_text("patch", label()), ActionArgDef::number("zoom", label()), ActionArgDef::slider("opacity", label(), 0.0, 1.0), ActionArgDef::toggle("snap", label()), ActionArgDef::vector("offset", label(), 3), ActionArgDef::text_list("ids", label())] {
            let (first, second) = declared_argument_alternatives(&argument).unwrap_or_else(|| panic!("{} yields a pair", argument.id));
            assert_ne!(first, second, "{}", argument.id);
        }
        assert!(declared_argument_alternatives(&one).is_none());
    }
}
