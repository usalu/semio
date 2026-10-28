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

#[test]
fn native_view_variants_render_and_pass_existing_header_admission_and_reduction() {
    use crate::details::Ifc4x3DetailsProvider;
    use crate::editor::{IFC4X3_CONTROLLER, Ifc4x3AnyEditor};
    use semio_framework_ui_locale::Locale;
    use semio_s_artifact_stdio_contract::editing::{self, SnapshotDetailPathSegment as P, SnapshotDetailsProvider, SnapshotEditingEditor};
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
    let mut base = IfcSnapshot::default();
    base.header.file_description = header(&serde_json::json!(["ViewDefinition [ReferenceView]", "Comment [keep] "]));
    let path = [P::Key("header".into()), P::Key("fileDescription".into()), P::Index(0), P::Key("value".into()), P::Index(0)];
    for (locale, expected_label) in [(Locale::En, "declared view"), (Locale::De, "deklarierte Ansicht")] {
        let provider = Ifc4x3DetailsProvider::new(&base);
        assert_eq!(provider.variant_count(&path, locale), 2);
        assert_eq!(provider.variants(&path, locale).len(), 2);
        let variant = provider.variant(&path, locale, 1).unwrap();
        assert!(variant.label.contains(expected_label));
        let event = editing::SnapshotEditEvent::SetValue { path: "/header/fileDescription/0/value/0".into(), value: variant.value };
        assert!(<Ifc4x3AnyEditor as SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &base));
        let emit = <Ifc4x3AnyEditor as SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).unwrap();
        assert_eq!(emit.artifact_mutations.len(), 1);
        let outcome = protocol::ColdMutation::diff(&emit.artifact_mutations[0], &base);
        let next = protocol::apply_diff_cold(outcome.diff(), &base).unwrap();
        assert_eq!(next.header.file_description, header(&serde_json::json!([ifc4x3_mvd_header_text(IfcMvdView::Alignment), "Comment [keep] "])));
        let rendered = editing::render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(42), locale, IFC4X3_CONTROLLER, &semio_framework_plugin::TreeWindows::unhosted()).unwrap();
        let serialization = serde_json::to_string(&rendered).unwrap();
        assert!(serialization.contains(expected_label));
        assert!(serialization.contains("Alignment-basedReferenceView"));
        assert!(serialization.contains(editing::SET_SNAPSHOT_VALUE_ACTION_ID));
        assert!(serialization.contains(IFC4X3_CONTROLLER));
    }
}
