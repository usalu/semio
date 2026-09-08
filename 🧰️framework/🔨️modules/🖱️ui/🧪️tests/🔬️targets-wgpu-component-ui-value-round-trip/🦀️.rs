mod value_round_trip_tests {
    use super::*;

    fn act(action: &str) -> ActionDescriptor {
        ActionDescriptor { controller_id: "ctrl".into(), action: action.into(), args: None }
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_text_node_round_trips() {
        let value = UiTextNode { value: Label::data("Hello"), emphasize: Some(true), data_attributes: None, presence: UiPresence::default(), menu: Some(UiMenuRef { id: "menu-1".into(), args: None }) };
        assert_eq!(UiTextNode::from_value(value.to_value()).expect("valid DslValue decodes"), value);
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_button_node_round_trips() {
        let value = UiButtonNode {
            id: Some("btn1".into()),
            icon_id: IconName::Save,
            label: Label::data("Save"),
            action: act("save"),
            style: Some(StyleSpec { variant: Some("primary".into()), size: None, density: None }),
            presence: UiPresence::default(),
            menu: None,
        };
        assert_eq!(UiButtonNode::from_value(value.to_value()).expect("valid DslValue decodes"), value);
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_control_node_round_trips() {
        let values = [
            UiControlNode::Input(UiInputNode {
                id: "inp1".into(),
                input_kind: "text".into(),
                value: "abc".into(),
                placeholder: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                on_change: act("change"),
                presence: UiPresence::default(),
                menu: None,
            }),
            UiControlNode::Select(UiSelectNode {
                id: "sel1".into(),
                value: "a".into(),
                items: vec![UiSelectItem { value: "a".into(), label: Label::data("A") }],
                placeholder: None,
                on_change: act("select"),
                presence: UiPresence::default(),
                menu: None,
            }),
            UiControlNode::Toggle(UiToggleNode { id: "tog1".into(), icon_id: IconName::Save, text: None, on_change: act("toggle"), presence: UiPresence::default(), menu: None }),
            UiControlNode::KeyValue(UiKeyValueNode { entries: vec![UiKeyValueEntry { label: Label::data("K"), value: "v".into() }], presence: UiPresence::default(), menu: None }),
            UiControlNode::Slider(UiSliderNode { id: "sld1".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.1, unit: None, on_change: act("slide"), presence: UiPresence::default(), menu: None }),
            UiControlNode::NumberStepper(UiNumberStepperNode { id: "stp1".into(), value: 3.0, step: 1.0, uniform: false, on_absolute: act("abs"), on_delta: act("delta"), presence: UiPresence::default(), menu: None }),
            UiControlNode::Ring(UiRingNode { id: "ring1".into(), orb_id: "orb".into(), t: 0.25, on_change: act("ring"), presence: UiPresence::default(), menu: None }),
            UiControlNode::IconSelect(UiIconSelectNode { id: "icn1".into(), value: "a".into(), uniform: true, classifier_kind: "kind".into(), on_change: act("iconSelect"), presence: UiPresence::default(), menu: None }),
        ];
        for value in values {
            assert_eq!(UiControlNode::from_value(value.to_value()).expect("valid DslValue decodes"), value);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_tree_item_node_round_trips() {
        let value = UiTreeItemNode {
            id: "item1".into(),
            label: Label::data("Item"),
            description: Some("desc".into()),
            icon_id: Some(IconName::Save),
            presence: UiPresence::default(),
            default_open: Some(true),
            action: Some(act("open")),
            actions: Some(vec![UiTreeItemAction { icon_id: IconName::Save, label: Some(Label::data("Row action")), action: act("rowAction"), placement: Some(UiTreeActionPlacement::Menu) }]),
            draggable: Some(true),
            drag_data: None,
            items: None,
            control: Some(UiControlNode::Toggle(UiToggleNode { id: "tog1".into(), icon_id: IconName::Save, text: None, on_change: act("toggle"), presence: UiPresence::default(), menu: None })),
            dimmed: Some(false),
            menu: Some(UiMenuRef { id: "menu-1".into(), args: None }),
        };
        assert_eq!(UiTreeItemNode::from_value(value.to_value()).expect("valid DslValue decodes"), value);
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_tree_section_and_tree_node_round_trip() {
        let section = UiTreeSectionNode { id: "sec1".into(), label: Some(Label::data("Section")), default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("item1", Label::data("Item"))] };
        assert_eq!(UiTreeSectionNode::from_value(section.clone().to_value()).expect("valid DslValue decodes"), section);

        let tree = UiTreeNode { sections: vec![section], presence: UiPresence::default(), drop_action: Some(act("drop")), menu: None, interaction_domain: Some("domain-1".into()) };
        assert_eq!(UiTreeNode::from_value(tree.to_value()).expect("valid DslValue decodes"), tree);
    }

    #[semio_framework_async_macros::async_test]
    async fn table_cell_round_trips() {
        let values = [
            TableCell::Text { value: "hi".into() },
            TableCell::Number { value: 3.5 },
            TableCell::Stepper { value: 1.0, min: 0.0, max: 10.0, step: 1.0, action: act("step") },
            TableCell::Buttons { buttons: vec![UiTreeItemAction { icon_id: IconName::Save, label: None, action: act("btn"), placement: None }] },
        ];
        for value in values {
            assert_eq!(TableCell::from_value(value.to_value()).expect("valid DslValue decodes"), value);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn block_palette_entry_and_external_slot_round_trip() {
        let entry = BlockPaletteEntry { block_kind: "note".into(), label: "Note".into(), icon_id: IconName::Save };
        assert_eq!(BlockPaletteEntry::from_value(entry.to_value()).expect("valid DslValue decodes"), entry);

        let slot = UiExternalSlotNode { plugin_id: "plugin-1".into(), app_id: "app-1".into(), body_key: "body".into(), params_json: "{}".into(), presence: UiPresence::default(), menu: None };
        assert_eq!(UiExternalSlotNode::from_value(slot.to_value()).expect("valid DslValue decodes"), slot);
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_component_scene_node_round_trips() {
        let value = UiComponentSceneNode {
            surface_id: "surface-1".into(),
            controller_id: "ctrl-1".into(),
            component_kind: SurfaceKind::World3d,
            pane_id: Some("pane-1".into()),
            binding_id: None,
            presence: UiPresence::default(),
            menu: Some(UiMenuRef { id: "menu-1".into(), args: None }),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            block_list: None,
            diff_view: None,
            event_feed: None,
        };
        assert_eq!(UiComponentSceneNode::from_value(value.to_value()).expect("valid DslValue decodes"), value);
    }

    /// 🌳️ `UiNode` is genuinely recursive (`UiStackNode`/`UiGroupNode`/`UiSectionNode` hold
    /// `children: Vec<UiNode>`, `UiFieldNode` holds `child: Box<UiNode>`) — this is the test most
    /// likely to surface an infinite-recursion or shape bug in the derive, so it nests three
    /// levels deep: `Stack > Section > Group > [Text, Field > Text]`.
    #[semio_framework_async_macros::async_test]
    async fn ui_node_round_trips_with_nested_children() {
        let leaf_text = |s: &str| UiNode::Text(UiTextNode { value: Label::data(s), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None });

        let field = UiNode::Field(UiFieldNode { id: "field-1".into(), label: Label::data("Field"), description: None, required: Some(true), error: None, child: Box::new(leaf_text("field child")), presence: UiPresence::default(), menu: None });

        let group = UiNode::Group(UiGroupNode { id: "group-1".into(), label: Label::data("Group"), default_open: Some(true), presence: UiPresence::default(), menu: None, children: vec![leaf_text("group child"), field] });

        let section = UiNode::Section(UiSectionNode { id: "section-1".into(), label: Some(Label::data("Section")), default_open: Some(false), presence: UiPresence::default(), menu: None, children: vec![group] });

        let stack = UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: Some("8".into()),
            padding: None,
            id: Some("stack-1".into()),
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
            children: vec![section],
        });

        assert_eq!(UiNode::from_value(stack.to_value()).expect("valid DslValue decodes"), stack);
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_inspector_field_group_round_trips() {
        let value = UiInspectorFieldGroup {
            id: "inspector-1".into(),
            label: Label::data("Inspector"),
            default_open: Some(true),
            presence: UiPresence::default(),
            fields: vec![UiNode::Text(UiTextNode { value: Label::data("field"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })],
        };
        assert_eq!(UiInspectorFieldGroup::from_value(value.to_value()).expect("valid DslValue decodes"), value);
    }
}
