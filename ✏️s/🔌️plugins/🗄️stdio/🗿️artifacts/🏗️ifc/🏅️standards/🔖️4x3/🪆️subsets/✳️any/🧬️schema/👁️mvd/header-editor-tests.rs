use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcValue;
use semio_s_artifact_stdio_ifc4x3_mvd::{ifc4x3_mvd_header_text, IfcMvdView};

fn header(values: &serde_json::Value) -> Vec<IfcValue> {
    vec![IfcValue::Aggregate(values.as_array().unwrap().iter().map(|value| IfcValue::String(value.as_str().unwrap().into())).collect()), IfcValue::String("2;1".into())]
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
