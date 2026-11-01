use super::*;

fn header(values: &serde_json::Value) -> Vec<IfcValue> {
    vec![IfcValue::Aggregate(values.as_array().unwrap().iter().map(|value| IfcValue::String(value.as_str().unwrap().into())).collect()), IfcValue::String("2;1".into())]
}

fn view(value: &serde_json::Value) -> Option<IfcMvdView> {
    match value.as_str() {
        Some("ReferenceView") => Some(IfcMvdView::Reference),
        Some("Alignment-basedReferenceView") => Some(IfcMvdView::Alignment),
        None => None,
        _ => panic!("neutral selection requires released names"),
    }
}

#[test]
fn neutral_header_commands_share_central_inverse_and_preserve_metadata() {
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
    let cases: serde_json::Value = serde_json::from_str(include_str!("header-cases.json")).unwrap();
    for case in cases["cases"].as_array().unwrap() {
        let mut base = IfcSnapshot::default();
        base.header.file_description = header(&case["before"]);
        let mutation = ifc4x3_mvd_header_mutation(&base.header.file_description, view(&case["view"]), &mut || Ok(())).unwrap();
        let IfcMutation::SetFileDescription(command) = &mutation else { panic!("ordinary header mutation expected") };
        assert_eq!(command.values, header(&case["after"]));
        let outcome = protocol::ColdMutation::diff(&mutation, &base);
        let next = protocol::apply_diff_cold(outcome.diff(), &base).unwrap();
        assert_eq!(next.header.file_description, command.values);
        assert_eq!(next.header.file_name, base.header.file_name);
        assert_eq!(next.header.file_schema, base.header.file_schema);
        let inverse = protocol::ColdMutation::inverse(&mutation, &base).unwrap();
        let mut restored = next;
        for command in inverse {
            let reverse = protocol::ColdMutation::diff(&command, &restored);
            restored = protocol::apply_diff_cold(reverse.diff(), &restored).unwrap();
        }
        assert_eq!(restored, base);
    }
}

#[test]
fn neutral_declarations_retain_unknown_names_and_refuse_malformed_replacement() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("header-cases.json")).unwrap();
    for case in cases["declarations"].as_array().unwrap() {
        let values = header(&serde_json::json!([case["text"]]));
        let found = ifc4x3_mvd_declarations(&values, &mut || Ok(())).unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].views, case["views"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_owned()).collect::<Vec<_>>());
        assert_eq!(found[0].malformed, case["malformed"].as_bool().unwrap());
        assert_eq!(found[0].known.len(), case["known"].as_array().unwrap().len());
        if found[0].malformed {
            assert!(ifc4x3_mvd_header_mutation(&values, Some(IfcMvdView::Reference), &mut || Ok(())).unwrap_err().contains("malformedMvdDeclaration"));
        }
    }
    for text in cases["unrelated"].as_array().unwrap() {
        assert!(ifc4x3_mvd_declarations(&header(&serde_json::json!([text])), &mut || Ok(())).unwrap().is_empty());
    }
}

#[test]
fn long_header_copy_and_parse_can_cancel_before_returning_a_mutation() {
    let values = header(&serde_json::json!([format!("ViewDefinition [{}]", "x".repeat(65536)), format!("Comment [{}]", "ä".repeat(65536))]));
    for limit in [1, 10, 256, 300] {
        let mut ticks = 0;
        let error = ifc4x3_mvd_header_mutation(&values, Some(IfcMvdView::Reference), &mut || {
            ticks += 1;
            if ticks == limit { Err("cancelled-original-owner".into()) } else { Ok(()) }
        })
        .unwrap_err();
        assert_eq!(error, "cancelled-original-owner");
        assert_eq!(ticks, limit);
    }
}
