mod form_kit_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn form_panel_builder_wraps_a_field_control_and_submit_button() {
        let on_change = ActionId::try_v1("app", "setValue").expect("bounded fixture");
        let submit_action = ActionId::try_v1("app", "submit").expect("bounded fixture");
        let control =
            input(InputKind::Text).try_id("ns-play-form.field.name").unwrap_or_else(|_| panic!("bounded fixture id")).try_on(Trigger::Change, on_change).unwrap_or_else(|_| panic!("bounded fixture binding")).try_build().expect("bounded fixture");
        let builder = FormPanelBuilder::new("ns-play-form").await.expect("bounded fixture");
        let builder = builder.field("name", "Name", Some("Full name".into()), control).await.expect("bounded fixture");
        let builder = builder.submit("Submit", submit_action).await.expect("bounded fixture");
        let node = builder.build().await.expect("bounded fixture");
        assert_eq!(node.key.as_str(), "ns-play-form");
        assert_eq!(node.children.len(), 2);
        let field = &node.children[0];
        assert_eq!(field.key.as_str(), "ns-play-form.field.name");
        let Component::Container(props) = &field.component else { panic!("expected a Field container") };
        assert_eq!(props.description.as_deref(), Some("Full name"));
        let Component::Button(button_props) = &node.children[1].component else { panic!("expected a Button node") };
        assert_eq!(button_props.label.0.as_str(), "Submit");
    }

    #[semio_framework_async_macros::async_test]
    async fn form_panel_builder_from_dictionary_routes_entries_into_field_rows() {
        let on_change = ActionId::try_v1("app", "setValue").expect("bounded fixture");
        let dictionary = serde_json::json!([
            { "id": "email", "label": "Email", "description": "Contact email", "value": "a@b.com" },
            { "id": "phone" },
        ]);
        let builder = FormPanelBuilder::new("ns-play-form").await.expect("bounded fixture");
        let builder = builder.from_dictionary(&dictionary, on_change).await.expect("bounded fixture");
        let node = builder.build().await.expect("bounded fixture");
        assert_eq!(node.children.len(), 2);
        let email_field = &node.children[0];
        assert_eq!(email_field.key.as_str(), "ns-play-form.field.email");
        let Component::Container(email_props) = &email_field.component else { panic!("expected a Field container") };
        assert_eq!(email_props.label.as_ref().map(|label| label.0.clone()).as_deref(), Some("Email"));
        let Component::Container(phone_props) = &node.children[1].component else { panic!("expected a Field container") };
        assert_eq!(phone_props.label.as_ref().map(|label| label.0.clone()).as_deref(), Some("phone"));
    }

    #[semio_framework_async_macros::async_test]
    async fn entity_detail_builds_a_stack_with_header_key_value_and_actions() {
        let action = ActionId::try_v1("app", "edit").expect("bounded fixture");
        let button = button(Label::try_from("Edit").expect("bounded fixture"))
            .icon(UiText::try_from_str("edit").expect("bounded fixture"))
            .try_on(Trigger::Activate, action)
            .unwrap_or_else(|_| panic!("bounded fixture binding"))
            .try_build()
            .expect("bounded fixture");
        let entry = KeyValueEntry { label: Label::try_from("Kind").expect("bounded fixture"), value: UiText::try_from_str("gizmo").expect("bounded fixture") };
        let node = entity_detail("Widget", Some(Label::try_from("A widget").expect("bounded fixture")), [entry], [button]).await.expect("bounded fixture");
        assert_eq!(node.children.len(), 4);
        let Component::KeyValueList(key_value) = &node.children[2].component else { panic!("expected a KeyValue node") };
        assert_eq!(key_value.entries[0].value.as_str(), "gizmo");
    }
}
