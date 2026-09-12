mod ui_node_wire_format_tests {
    use super::*;

    fn act(action: &str) -> ActionDescriptor {
        ActionDescriptor { controller_id: "ctrl".into(), action: action.into(), args: None }
    }

    fn sample_tree() -> UiNode {
        UiNode::Stack(UiStackNode {
            menu: None,
            direction: "vertical".into(),
            gap: Some("md".into()),
            padding: None,
            id: Some("root".into()),
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children: vec![
                UiNode::Text(UiTextNode { menu: None, value: Label::data("Hello"), emphasize: Some(true), data_attributes: None, presence: UiPresence::default() }),
                UiNode::Button(UiButtonNode { menu: None, id: Some("btn1".into()), icon_id: IconName::Save, label: Label::data("Save"), action: act("save"), style: None, presence: UiPresence::default() }),
                UiNode::Separator(UiSeparatorNode { menu: None, presence: UiPresence::default() }),
                UiNode::Input(UiInputNode {
                    menu: None,
                    id: "inp1".into(),
                    input_kind: "text".into(),
                    value: "abc".into(),
                    placeholder: Some(Label::data("type...")),
                    commit: None,
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    on_change: act("setValue"),
                    presence: UiPresence::default(),
                }),
                UiNode::Select(UiSelectNode {
                    menu: None,
                    id: "sel1".into(),
                    value: "a".into(),
                    items: vec![UiSelectItem { value: "a".into(), label: Label::data("A") }, UiSelectItem { value: "b".into(), label: Label::data("B") }],
                    placeholder: None,
                    on_change: act("selectChange"),
                    presence: UiPresence::default(),
                }),
                UiNode::Toggle(UiToggleNode { menu: None, id: "tog1".into(), icon_id: IconName::AlignLeft, text: None, on_change: act("toggle"), presence: UiPresence::selected(true) }),
                UiNode::Group(UiGroupNode {
                    menu: None,
                    id: "grp1".into(),
                    label: Label::data("Group"),
                    default_open: Some(true),
                    presence: UiPresence::default(),
                    children: vec![UiNode::Text(UiTextNode { menu: None, value: Label::data("child"), emphasize: None, data_attributes: None, presence: UiPresence::default() })],
                }),
                UiNode::KeyValue(UiKeyValueNode { menu: None, entries: vec![UiKeyValueEntry { label: Label::data("K"), value: "V".into() }], presence: UiPresence::default() }),
                UiNode::Slider(UiSliderNode { menu: None, id: "sl1".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.1, unit: Some("%".into()), on_change: act("sliderChange"), presence: UiPresence::default() }),
                UiNode::NumberStepper(UiNumberStepperNode { menu: None, id: "num1".into(), value: 2.0, step: 1.0, uniform: true, on_absolute: act("setAbs"), on_delta: act("setDelta"), presence: UiPresence::default() }),
                UiNode::Ring(UiRingNode { menu: None, id: "ring1".into(), orb_id: "orb1".into(), t: 0.25, presence: UiPresence::default(), on_change: act("ringChange") }),
                UiNode::IconSelect(UiIconSelectNode { menu: None, id: "icn1".into(), value: "star".into(), uniform: true, classifier_kind: "icon".into(), on_change: act("iconChange"), presence: UiPresence::default() }),
                UiNode::Field(UiFieldNode {
                    menu: None,
                    id: "field1".into(),
                    label: Label::data("Field"),
                    description: Some("desc".into()),
                    required: Some(true),
                    error: None,
                    child: Box::new(UiNode::Text(UiTextNode { menu: None, value: Label::data("child"), emphasize: None, data_attributes: None, presence: UiPresence::default() })),
                    presence: UiPresence::default(),
                }),
                UiNode::Section(UiSectionNode { menu: None, id: "sec1".into(), label: Some(Label::data("Section")), default_open: Some(true), presence: UiPresence::default(), children: vec![] }),
                UiNode::Tree(UiTreeNode {
                    menu: None,
                    sections: vec![UiTreeSectionNode {
                        id: "treesec1".into(),
                        label: Some(Label::data("Items")),
                        default_open: Some(true),
                        presence: UiPresence::default(),
                        items: vec![{
                            let mut item = UiTreeItemNode::base("item1", Label::data("Item 1"));
                            item.presence.selected = true;
                            item
                        }],
                    }],
                    presence: UiPresence::default(),
                    interaction_domain: None,
                    drop_action: None,
                }),
                UiNode::Image(UiImageNode { menu: None, id: "img1".into(), src: "icon.png".into(), alt: Some(Label::data("alt text")), presence: UiPresence::default() }),
                UiNode::ComponentScene(UiComponentSceneNode {
                    menu: None,
                    surface_id: "surf1".into(),
                    controller_id: "ctrl".into(),
                    component_kind: SurfaceKind::World3d,
                    pane_id: None,
                    binding_id: None,
                    presence: UiPresence::default(),
                    canvas_2d: None,
                    world_3d: Some(World3dScene {
                        snapshot: None,
                        camera_json: "{}".into(),
                        meshes_json: "[]".into(),
                        instances_json: "[]".into(),
                        selection_json: "{}".into(),
                        vortices_json: None,
                        attractions_json: None,
                        target_volumes_json: None,
                        references_json: None,
                        brush_preview_json: None,
                        interaction_json: None,
                        engagement_preview_json: None,
                        lod_json: None,
                        chunking_json: None,
                        environment_json: None,
                        frame_json: None,
                        fit_json: None,
                        terrain_json: None,
                        points_json: None,
                        status_json: None,
                        domain_id: None,
                        domain_granularity_id: None,
                        lanes: Vec::new(),
                    }),
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
                }),
                UiNode::ExternalSlot(UiExternalSlotNode { menu: None, plugin_id: "plugin1".into(), app_id: "app1".into(), body_key: "body1".into(), params_json: "{}".into(), presence: UiPresence::default() }),
            ],
        })
    }

    const GOLDEN_UI_NODE_TREE_JSON: &str = "{\"type\":\"stack\",\"direction\":\"vertical\",\"gap\":\"md\",\"id\":\"root\",\"children\":[{\"type\":\"text\",\"value\":\"Hello\",\"emphasize\":true},{\"type\":\"button\",\"id\":\"btn1\",\"iconId\":\"save\",\"label\":\"Save\",\"action\":{\"controllerId\":\"ctrl\",\"action\":\"save\"}},{\"type\":\"separator\"},{\"type\":\"input\",\"id\":\"inp1\",\"inputKind\":\"text\",\"value\":\"abc\",\"placeholder\":\"type...\",\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"setValue\"}},{\"type\":\"select\",\"id\":\"sel1\",\"value\":\"a\",\"items\":[{\"value\":\"a\",\"label\":\"A\"},{\"value\":\"b\",\"label\":\"B\"}],\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"selectChange\"}},{\"type\":\"toggle\",\"id\":\"tog1\",\"iconId\":\"align-left\",\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"toggle\"},\"presence\":{\"selected\":true}},{\"type\":\"group\",\"id\":\"grp1\",\"label\":\"Group\",\"defaultOpen\":true,\"children\":[{\"type\":\"text\",\"value\":\"child\"}]},{\"type\":\"keyValue\",\"entries\":[{\"label\":\"K\",\"value\":\"V\"}]},{\"type\":\"slider\",\"id\":\"sl1\",\"value\":0.5,\"min\":0.0,\"max\":1.0,\"step\":0.1,\"unit\":\"%\",\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"sliderChange\"}},{\"type\":\"numberStepper\",\"id\":\"num1\",\"value\":2.0,\"step\":1.0,\"uniform\":true,\"onAbsolute\":{\"controllerId\":\"ctrl\",\"action\":\"setAbs\"},\"onDelta\":{\"controllerId\":\"ctrl\",\"action\":\"setDelta\"}},{\"type\":\"ring\",\"id\":\"ring1\",\"orbId\":\"orb1\",\"t\":0.25,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"ringChange\"}},{\"type\":\"iconSelect\",\"id\":\"icn1\",\"value\":\"star\",\"uniform\":true,\"classifierKind\":\"icon\",\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"iconChange\"}},{\"type\":\"field\",\"id\":\"field1\",\"label\":\"Field\",\"description\":\"desc\",\"required\":true,\"child\":{\"type\":\"text\",\"value\":\"child\"}},{\"type\":\"section\",\"id\":\"sec1\",\"label\":\"Section\",\"defaultOpen\":true,\"children\":[]},{\"type\":\"tree\",\"sections\":[{\"id\":\"treesec1\",\"label\":\"Items\",\"defaultOpen\":true,\"items\":[{\"id\":\"item1\",\"label\":\"Item 1\",\"presence\":{\"selected\":true}}]}]},{\"type\":\"image\",\"id\":\"img1\",\"src\":\"icon.png\",\"alt\":\"alt text\"},{\"type\":\"componentScene\",\"surfaceId\":\"surf1\",\"controllerId\":\"ctrl\",\"componentKind\":\"world-3d\",\"world3d\":{\"cameraJson\":\"{}\",\"meshesJson\":\"[]\",\"instancesJson\":\"[]\",\"selectionJson\":\"{}\"}},{\"type\":\"externalSlot\",\"pluginId\":\"plugin1\",\"appId\":\"app1\",\"bodyKey\":\"body1\",\"paramsJson\":\"{}\"}]}";

    #[semio_framework_async_macros::async_test]
    async fn ui_node_tree_serializes_to_golden_json() {
        let node = sample_tree();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, GOLDEN_UI_NODE_TREE_JSON, "UiNode wire format drifted \u{2014} lock this in before moving the type into ui_wgpu");
        let roundtripped: UiNode = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, node);
    }

    /// 🌀️ `presence.status` follows the same skip-if-default convention as `presence.selected`: the whole `presence` key is absent when fully default, and round-trips when set.
    #[semio_framework_async_macros::async_test]
    async fn ui_tree_item_loading_status_skips_when_default_and_roundtrips_when_set() {
        let idle = UiTreeItemNode::base("idle", Label::data("Idle"));
        assert!(!serde_json::to_string(&idle).unwrap().contains("presence"));

        let mut loading = UiTreeItemNode::base("loading1", Label::data("Loading"));
        loading.presence.status = UiStatus::Loading;
        let json = serde_json::to_string(&loading).unwrap();
        assert!(json.contains("\"presence\":{\"status\":\"loading\"}"));
        let roundtripped: UiTreeItemNode = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, loading);
    }

    /// 🌀️ `waiting` follows the same skip-if-default convention as `loading`: absent when unset, round-trips when set.
    #[semio_framework_async_macros::async_test]
    async fn ui_tree_item_waiting_status_skips_when_default_and_roundtrips_when_set() {
        let idle = UiTreeItemNode::base("idle", Label::data("Idle"));
        assert!(!serde_json::to_string(&idle).unwrap().contains("presence"));

        let mut waiting = UiTreeItemNode::base("waiting1", Label::data("Waiting"));
        waiting.presence.status = UiStatus::Waiting;
        let json = serde_json::to_string(&waiting).unwrap();
        assert!(json.contains("\"presence\":{\"status\":\"waiting\"}"));
        let roundtripped: UiTreeItemNode = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, waiting);
    }

    /// 🚫️ `presence.state == Hidden` short-circuits everything else — round-trips like any other state.
    #[semio_framework_async_macros::async_test]
    async fn ui_tree_item_hidden_state_roundtrips() {
        let mut hidden = UiTreeItemNode::base("hidden1", Label::data("Hidden"));
        hidden.presence.state = UiState::Hidden;
        assert!(!hidden.presence.visible());
        let json = serde_json::to_string(&hidden).unwrap();
        assert!(json.contains("\"presence\":{\"state\":\"hidden\"}"));
        let roundtripped: UiTreeItemNode = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, hidden);
    }

    /// 🎉️ `presence.state == Celebrating` serializes to `"celebrating"` — guards the TS/Rust `UiState`
    /// mirror staying byte-for-byte (see `ui/styling/js/index.ts`'s `UI_STATES`).
    #[semio_framework_async_macros::async_test]
    async fn ui_tree_item_celebrating_state_roundtrips() {
        let mut celebrating = UiTreeItemNode::base("celebrating1", Label::data("Celebrating"));
        celebrating.presence.state = UiState::Celebrating;
        assert!(celebrating.presence.visible());
        let json = serde_json::to_string(&celebrating).unwrap();
        assert!(json.contains("\"presence\":{\"state\":\"celebrating\"}"));
        let roundtripped: UiTreeItemNode = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, celebrating);
    }

    /// ✨ Every `UiNode` variant's `presence` field actually serializes when set — an exhaustiveness
    /// belt-and-braces check so a future variant can't silently drop its shared state on the wire.
    #[semio_framework_async_macros::async_test]
    async fn every_ui_node_variant_serializes_a_non_default_presence() {
        fn assert_presence_serializes(mut node: UiNode, label: &str) {
            *node.presence_mut() = UiPresence::selected(true);
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("\"presence\""), "{label} did not serialize a non-default presence: {json}");
        }
        assert_presence_serializes(
            UiNode::Stack(UiStackNode { menu: None, direction: "vertical".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: vec![] }),
            "Stack",
        );
        assert_presence_serializes(UiNode::Text(UiTextNode { menu: None, value: Label::data("x"), emphasize: None, data_attributes: None, presence: UiPresence::default() }), "Text");
        assert_presence_serializes(UiNode::Button(UiButtonNode { menu: None, id: None, icon_id: IconName::CircleDot, label: Label::data("l"), action: act("a"), style: None, presence: UiPresence::default() }), "Button");
        assert_presence_serializes(UiNode::Separator(UiSeparatorNode { menu: None, presence: UiPresence::default() }), "Separator");
        assert_presence_serializes(
            UiNode::Input(UiInputNode {
                menu: None,
                id: "i".into(),
                input_kind: "text".into(),
                value: "v".into(),
                placeholder: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                on_change: act("a"),
                presence: UiPresence::default(),
            }),
            "Input",
        );
        assert_presence_serializes(UiNode::Select(UiSelectNode { menu: None, id: "i".into(), value: "v".into(), items: vec![], placeholder: None, on_change: act("a"), presence: UiPresence::default() }), "Select");
        assert_presence_serializes(UiNode::Toggle(UiToggleNode { menu: None, id: "i".into(), icon_id: IconName::CircleDot, text: None, on_change: act("a"), presence: UiPresence::default() }), "Toggle");
        assert_presence_serializes(UiNode::KeyValue(UiKeyValueNode { menu: None, entries: vec![], presence: UiPresence::default() }), "KeyValue");
        assert_presence_serializes(UiNode::Slider(UiSliderNode { menu: None, id: "i".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1, unit: None, on_change: act("a"), presence: UiPresence::default() }), "Slider");
        assert_presence_serializes(UiNode::NumberStepper(UiNumberStepperNode { menu: None, id: "i".into(), value: 0.0, step: 1.0, uniform: true, on_absolute: act("a"), on_delta: act("a"), presence: UiPresence::default() }), "NumberStepper");
        assert_presence_serializes(UiNode::Ring(UiRingNode { menu: None, id: "i".into(), orb_id: "o".into(), t: 0.0, on_change: act("a"), presence: UiPresence::default() }), "Ring");
        assert_presence_serializes(UiNode::IconSelect(UiIconSelectNode { menu: None, id: "i".into(), value: "v".into(), uniform: true, classifier_kind: "icon".into(), on_change: act("a"), presence: UiPresence::default() }), "IconSelect");
        assert_presence_serializes(
            UiNode::Field(UiFieldNode {
                menu: None,
                id: "i".into(),
                label: Label::data("l"),
                description: None,
                required: None,
                error: None,
                child: Box::new(UiNode::Text(UiTextNode { menu: None, value: Label::data("x"), emphasize: None, data_attributes: None, presence: UiPresence::default() })),
                presence: UiPresence::default(),
            }),
            "Field",
        );
        assert_presence_serializes(UiNode::Section(UiSectionNode { menu: None, id: "i".into(), label: None, default_open: None, presence: UiPresence::default(), children: vec![] }), "Section");
        assert_presence_serializes(UiNode::Group(UiGroupNode { menu: None, id: "i".into(), label: Label::data("l"), default_open: None, presence: UiPresence::default(), children: vec![] }), "Group");
        assert_presence_serializes(UiNode::Tree(UiTreeNode { menu: None, sections: vec![], presence: UiPresence::default(), drop_action: None, interaction_domain: None }), "Tree");
        assert_presence_serializes(UiNode::Image(UiImageNode { menu: None, id: "i".into(), src: "s".into(), alt: None, presence: UiPresence::default() }), "Image");
        assert_presence_serializes(UiNode::ExternalSlot(UiExternalSlotNode { menu: None, plugin_id: "p".into(), app_id: "a".into(), body_key: "b".into(), params_json: "{}".into(), presence: UiPresence::default() }), "ExternalSlot");
        assert_presence_serializes(
            UiNode::ComponentScene(UiComponentSceneNode {
                menu: None,
                surface_id: "s".into(),
                controller_id: "c".into(),
                component_kind: SurfaceKind::Canvas2d,
                pane_id: None,
                binding_id: None,
                presence: UiPresence::default(),
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
            }),
            "ComponentScene",
        );
    }

    /// ☁ `points_json` follows the same `Option<String>` skip-if-none convention as `terrain_json`:
    /// absent when unset, round-trips (camelCase `pointsJson`) when set.
    #[semio_framework_async_macros::async_test]
    async fn world_3d_scene_points_json_skips_when_none_and_roundtrips_when_set() {
        let bare = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
        assert!(!serde_json::to_string(&bare).unwrap().contains("pointsJson"));

        let mut with_points = bare;
        with_points.points_json = Some(r#"[{"id":"cloud-1","positionsB64":"AACAPwAAAEAAAEBA","colorsB64":"/wAA","size":2.0,"sizeAttenuation":true}]"#.into());
        let json = serde_json::to_string(&with_points).unwrap();
        assert!(json.contains("\"pointsJson\":"));
        let roundtripped: World3dScene = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, with_points);
    }

    const GOLDEN_SURFACE_KIND_JSON: &str =
        "[\"canvas-2d\",\"world-3d\",\"node-graph\",\"text-editor\",\"table\",\"paint-2d\",\"virtualFileSystem\",\"tiled-map\",\"board-2d\",\"icon-render\",\"ink-canvas\",\"graph-timeline\",\"diff-view\",\"event-feed\"]";

    #[semio_framework_async_macros::async_test]
    async fn surface_kind_serializes_to_golden_json() {
        let kinds = vec![
            SurfaceKind::Canvas2d,
            SurfaceKind::World3d,
            SurfaceKind::NodeGraph,
            SurfaceKind::TextEditor,
            SurfaceKind::Table,
            SurfaceKind::Paint2d,
            SurfaceKind::VirtualFileSystem,
            SurfaceKind::TiledMap,
            SurfaceKind::Board2d,
            SurfaceKind::IconRender,
            SurfaceKind::InkCanvas,
            SurfaceKind::GraphTimeline,
            SurfaceKind::DiffView,
            SurfaceKind::EventFeed,
        ];
        let json = serde_json::to_string(&kinds).unwrap();
        assert_eq!(json, GOLDEN_SURFACE_KIND_JSON);
        let roundtripped: Vec<SurfaceKind> = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, kinds);
    }

    const GOLDEN_SCENES_JSON: &str = "[{\"cameraX\":1.0,\"cameraY\":2.0,\"zoom\":1.5,\"layersJson\":\"[]\"},{\"columnsJson\":\"[]\",\"rowsJson\":\"[]\"},{\"documentSyncJson\":\"{}\",\"assetsJson\":\"[]\",\"cameraJson\":\"{}\",\"selectionJson\":\"[]\",\"hoveredId\":\"h1\",\"activeUtility\":\"brush\",\"brushSize\":4.0,\"brushOpacity\":1.0,\"viewMode\":\"composite\"},{\"requestJson\":\"{}\"},{\"schemaJson\":\"{}\",\"rowsJson\":\"[]\",\"emptyMessage\":\"Empty\",\"dragDropEnabled\":true},{\"mapFixtureJson\":\"{}\",\"cameraJson\":\"{}\",\"renderMode\":\"combined\",\"vectorStyle\":\"colored\",\"lodMode\":\"automatic\",\"tileUrlTemplate\":\"/osm/{z}/{x}/{y}.png\",\"vectorTileUrlTemplate\":\"/vt/{z}/{x}/{y}.pbf\",\"layerVisibilityJson\":\"{}\",\"layerStrokeScaleJson\":\"{}\",\"selectionJson\":\"{}\",\"hoverJson\":\"null\",\"selectionMethod\":\"rectangle\",\"selectionMode\":\"default\"},{\"fixtureJson\":\"{}\",\"cameraJson\":\"{}\",\"glyphCatalogsJson\":\"{}\",\"selectionJson\":\"[]\",\"interactive\":true,\"selectionMethod\":\"rectangle\",\"gridSnapEnabled\":false,\"gridFactor\":1.0,\"suggestionOffset\":0.0,\"brushWeightsJson\":\"{}\",\"placementCompatibilityJson\":\"[]\",\"lodMode\":\"automatic\"},{\"documentJson\":\"{}\",\"selectionJson\":\"[]\",\"activeUtility\":\"select\",\"viewMode\":\"edit\",\"interactive\":true},{\"columnsJson\":\"[]\"},{\"nodes\":[],\"edges\":[],\"viewport\":{\"x\":0.0,\"y\":0.0,\"zoom\":1.0}},{\"buffer\":\"buf\",\"language\":\"rust\"},{\"stepsJson\":\"[]\",\"paletteJson\":\"[]\"}]";

    #[semio_framework_async_macros::async_test]
    async fn scene_records_serialize_to_golden_json() {
        let scenes = (
            Canvas2dScene { camera_x: 1.0, camera_y: 2.0, zoom: 1.5, layers_json: "[]".into(), snapshot: None },
            TableScene::base("[]", "[]"),
            Paint2dScene {
                document_sync_json: "{}".into(),
                assets_json: "[]".into(),
                camera_json: "{}".into(),
                selection_json: "[]".into(),
                hovered_id: Some("h1".into()),
                active_utility: "brush".into(),
                brush_size: 4.0,
                brush_opacity: 1.0,
                view_mode: "composite".into(),
                composite_viewport_json: None,
            },
            IconRenderScene { request_json: "{}".into(), footer: None, frame_json: None },
            VirtualFileSystemScene { schema_json: "{}".into(), rows_json: "[]".into(), selected_row_ids_json: None, hovered_row_id: None, empty_message: Some("Empty".into()), drag_drop_enabled: Some(true) },
            TiledMapScene::base("{}".into(), "{}".into()),
            Board2dScene::base("{}".into(), "{}".into(), true),
            InkCanvasScene::base("{}".into(), "select".into(), "edit".into(), true),
            GraphTimelineScene { columns_json: "[]".into() },
            NodeGraphScene::base(vec![], vec![], semio_framework_os_kernel::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 }),
            TextEditorScene::base("buf".into(), Some("rust".into()), None),
            BlockListScene { steps_json: "[]".into(), palette_json: "[]".into(), selected_id: None, dragging_id: None, domain_id: None },
        );
        let json = serde_json::to_string(&scenes).unwrap();
        assert_eq!(json, GOLDEN_SCENES_JSON);
        let roundtripped: (Canvas2dScene, TableScene, Paint2dScene, IconRenderScene, VirtualFileSystemScene, TiledMapScene, Board2dScene, InkCanvasScene, GraphTimelineScene, NodeGraphScene, TextEditorScene, BlockListScene) =
            serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, scenes);
    }

    /// 🆚 `DiffViewScene`/`EventFeedScene` golden coverage lives in its own pair-tuple (rather than
    /// joining `scene_records_serialize_to_golden_json`'s tuple above) because std only implements
    /// `Debug`/`PartialEq` for tuples up to 12 elements, and that tuple is already at the cap.
    const GOLDEN_DIFF_VIEW_EVENT_FEED_SCENES_JSON: &str = "[{\"before\":\"a\",\"after\":\"b\",\"language\":\"rust\",\"mode\":\"unified\"},{\"entriesJson\":\"[]\",\"follow\":true,\"activateAction\":\"openEvent\"}]";

    #[semio_framework_async_macros::async_test]
    async fn diff_view_and_event_feed_scenes_serialize_to_golden_json() {
        let scenes = (
            DiffViewScene { before: "a".into(), after: "b".into(), language: Some("rust".into()), mode: Some("unified".into()), domain_id: None },
            EventFeedScene { entries_json: "[]".into(), follow: Some(true), activate_action: Some("openEvent".into()), domain_id: None },
        );
        let json = serde_json::to_string(&scenes).unwrap();
        assert_eq!(json, GOLDEN_DIFF_VIEW_EVENT_FEED_SCENES_JSON);
        let roundtripped: (DiffViewScene, EventFeedScene) = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, scenes);
    }

    /// 🖱️ `UiMenuRef`/`ContextMenuItemSpec` camelCase wire shape — in particular `hover_args` must
    /// serialize as `hoverArgs` (the exact field-rename pitfall documented on `UiDirtyScope`).
    #[semio_framework_async_macros::async_test]
    async fn ui_menu_ref_and_context_menu_item_spec_roundtrip_camel_case() {
        let menu_ref = UiMenuRef { id: "row".into(), args: Some(DslValue::Object(vec![("id".into(), DslValue::String("row-1".into()))])) };
        let json = serde_json::to_string(&menu_ref).unwrap();
        assert_eq!(json, r#"{"id":"row","args":{"id":"row-1"}}"#);
        assert_eq!(serde_json::from_str::<UiMenuRef>(&json).unwrap(), menu_ref);

        let item = ContextMenuItemSpec {
            id: "delete".into(),
            label: Some("Delete".into()),
            icon: Some("trash".into()),
            color: None,
            shortcut: Some("Del".into()),
            disabled: Some(false),
            separator: None,
            checked: None,
            destructive: Some(true),
            action: Some("deleteSelection".into()),
            args: None,
            hover_action: Some("previewDelete".into()),
            hover_args: Some(DslValue::Object(vec![("x".into(), DslValue::float(1.0)), ("y".into(), DslValue::float(2.0))])),
            children: None,
        };
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("\"hoverAction\""), "hover_action must serialize as hoverAction: {json}");
        assert!(json.contains("\"hoverArgs\":{\"x\":1.0,\"y\":2.0}"), "hover_args must serialize as hoverArgs: {json}");
        assert!(!json.contains("\"color\""), "None fields must be omitted: {json}");
        let roundtripped: ContextMenuItemSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, item);
    }

    /// 🖱️ Every `UiNode` variant's `menu` ref actually serializes when set, and is omitted by default
    /// — the same exhaustiveness belt-and-braces check as `every_ui_node_variant_serializes_a_non_default_presence`.
    #[semio_framework_async_macros::async_test]
    async fn every_ui_node_variant_serializes_a_set_menu_ref() {
        fn assert_menu_serializes(mut node: UiNode, label: &str) {
            assert!(!serde_json::to_string(&node).unwrap().contains("\"menu\""), "{label} must omit a default menu ref");
            *node.menu_mut() = Some(UiMenuRef { id: "m".into(), args: None });
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("\"menu\":{\"id\":\"m\"}"), "{label} did not serialize a set menu ref: {json}");
            assert_eq!(node.menu(), Some(&UiMenuRef { id: "m".into(), args: None }));
        }
        assert_menu_serializes(
            UiNode::Stack(UiStackNode { menu: None, direction: "vertical".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: vec![] }),
            "Stack",
        );
        assert_menu_serializes(UiNode::Text(UiTextNode { menu: None, value: Label::data("x"), emphasize: None, data_attributes: None, presence: UiPresence::default() }), "Text");
        assert_menu_serializes(UiNode::Button(UiButtonNode { menu: None, id: None, icon_id: IconName::CircleDot, label: Label::data("l"), action: act("a"), style: None, presence: UiPresence::default() }), "Button");
        assert_menu_serializes(UiNode::Separator(UiSeparatorNode { menu: None, presence: UiPresence::default() }), "Separator");
        assert_menu_serializes(UiNode::Image(UiImageNode { menu: None, id: "i".into(), src: "s".into(), alt: None, presence: UiPresence::default() }), "Image");
        assert_menu_serializes(UiNode::Tree(UiTreeNode { menu: None, sections: vec![], presence: UiPresence::default(), drop_action: None, interaction_domain: None }), "Tree");
    }

    //#region 🗂️OrganizeContextMenuTests
    fn menu_leaf(id: &str) -> ContextMenuItemSpec {
        ContextMenuItemSpec { id: id.into(), label: Some(id.into()), action: Some(id.into()), ..Default::default() }
    }

    fn menu_destructive(id: &str) -> ContextMenuItemSpec {
        ContextMenuItemSpec { destructive: Some(true), ..menu_leaf(id) }
    }

    fn menu_group(category: &str, children: Vec<ContextMenuItemSpec>) -> ContextMenuItemSpec {
        ContextMenuItemSpec { id: format!("menu.group.{category}"), label: None, children: Some(children), ..Default::default() }
    }

    fn menu_header(label: &str) -> ContextMenuItemSpec {
        ContextMenuItemSpec { id: format!("header-{label}"), label: Some(label.into()), separator: Some(true), ..Default::default() }
    }

    fn menu_separator(id: &str) -> ContextMenuItemSpec {
        ContextMenuItemSpec { id: id.into(), separator: Some(true), ..Default::default() }
    }

    // 🚫️async: E1 pure accessor consumed by sync-only std call sites (&dyn Fn value) — see R9
    fn no_category(_id: &str) -> Option<String> {
        None
    }

    #[semio_framework_async_macros::async_test]
    async fn organize_context_menu_emits_as_is_within_budget() {
        let items = vec![menu_leaf("a"), menu_leaf("b"), menu_group("view", vec![menu_leaf("c")])];
        let organized = organize_context_menu(items.clone(), &no_category);
        assert_eq!(organized, items, "within budget with leaves already before groups, nothing is reordered: {organized:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn organize_context_menu_puts_destructive_leaves_last_after_a_separator() {
        let items = vec![menu_destructive("delete"), menu_leaf("a"), menu_leaf("b")];
        let organized = organize_context_menu(items, &no_category);
        assert_eq!(organized.len(), 4, "a separator is inserted before the destructive tail: {organized:?}");
        assert_eq!(organized[0].id, "a");
        assert_eq!(organized[1].id, "b");
        assert_eq!(organized[2].separator, Some(true));
        assert_eq!(organized[2].label, None, "the inserted separator is bare, not a header");
        assert_eq!(organized[3].id, "delete");
        assert_eq!(organized[3].destructive, Some(true));
    }

    #[semio_framework_async_macros::async_test]
    async fn organize_context_menu_merges_same_id_groups_and_dedupes_children_by_id() {
        let items = vec![menu_group("view", vec![menu_leaf("zoomIn"), menu_leaf("zoomOut")]), menu_leaf("a"), menu_group("view", vec![menu_leaf("zoomOut"), menu_leaf("resetZoom")])];
        let organized = organize_context_menu(items, &no_category);
        assert_eq!(organized.iter().filter(|item| item.id == "menu.group.view").count(), 1, "only one merged row remains: {organized:?}");
        let view_group = organized.iter().find(|item| item.id == "menu.group.view").expect("merged view group present");
        let child_ids: Vec<&str> = view_group.children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
        assert_eq!(child_ids, vec!["zoomIn", "zoomOut", "resetZoom"], "children concat in first-seen order, deduped by id: {child_ids:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn organize_context_menu_collapses_doubled_bare_separators_and_drops_leading_trailing_ones() {
        let items = vec![menu_separator("lead-bare"), menu_leaf("a"), menu_separator("dup-1"), menu_separator("dup-2"), menu_leaf("b"), menu_separator("trail-bare")];
        let organized = organize_context_menu(items, &no_category);
        assert_eq!(organized.len(), 3, "leading/trailing bare separators drop, the doubled run collapses to one: {organized:?}");
        assert_eq!(organized[0].id, "a");
        assert_eq!(organized[1].separator, Some(true));
        assert_eq!(organized[1].label, None, "the surviving separator is bare, not a header");
        assert_eq!(organized[2].id, "b");
    }

    #[semio_framework_async_macros::async_test]
    async fn organize_context_menu_keeps_a_labeled_separator_as_a_non_interactive_header() {
        let items = vec![menu_leaf("a"), menu_header("Recent"), menu_leaf("b")];
        let organized = organize_context_menu(items.clone(), &no_category);
        assert_eq!(organized, items, "a header is preserved in place, untouched by budget/ordering: {organized:?}");
        assert_eq!(organized[1].label.as_deref(), Some("Recent"));
        assert_eq!(organized[1].separator, Some(true));
    }

    #[semio_framework_async_macros::async_test]
    async fn organize_context_menu_sorts_group_rows_in_taxonomy_order_unknown_last() {
        let items = vec![menu_group("mystery", vec![menu_leaf("x")]), menu_group("export", vec![menu_leaf("y")]), menu_group("view", vec![menu_leaf("z")])];
        let organized = organize_context_menu(items, &no_category);
        let ids: Vec<&str> = organized.iter().map(|item| item.id.as_str()).collect();
        assert_eq!(ids, vec!["menu.group.view", "menu.group.export", "menu.group.mystery"], "view < export < unknown category: {ids:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn organize_context_menu_folds_overflow_groups_into_menu_group_more() {
        let mut items: Vec<ContextMenuItemSpec> = Vec::with_capacity(5);
        for index in 0..5 {
            items.push(menu_leaf(&format!("primary{index}")));
        }
        for category in ["hand", "selection", "lasso", "filter", "open", "save", "transfer", "transform"] {
            items.push(menu_group(category, vec![menu_leaf(&format!("{category}-child"))]));
        }
        assert!(items.len() > 9, "fixture must exceed the row budget to exercise the >9 path");
        let organized = organize_context_menu(items, &no_category);
        assert_eq!(organized.len(), 9, "primaries + groups clamp to the 9-row budget: {organized:?}");
        assert_eq!(organized.last().unwrap().id, "menu.group.more");
        assert!(!organized.last().unwrap().children.as_ref().unwrap().is_empty(), "the folded group carries the overflowing groups' children");
    }

    #[semio_framework_async_macros::async_test]
    async fn organize_context_menu_buckets_overflow_leaves_by_category_of() {
        let mut items: Vec<ContextMenuItemSpec> = Vec::with_capacity(5);
        for index in 0..5 {
            items.push(menu_leaf(&format!("primary{index}")));
        }
        for index in 0..6 {
            items.push(menu_leaf(&format!("overflow{index}")));
        }
        let categorize = |id: &str| if id.starts_with("overflow") { Some("view".to_string()) } else { None };
        let organized = organize_context_menu(items, &categorize);
        assert_eq!(organized.len(), 6, "5 primaries + 1 view group: {organized:?}");
        assert_eq!(organized[5].id, "menu.group.view");
        assert_eq!(organized[5].children.as_ref().unwrap().len(), 6);
    }

    #[semio_framework_async_macros::async_test]
    async fn ribbon_parent_label_covers_exactly_the_twenty_taxonomy_ids_and_rejects_unknown() {
        assert_eq!(RIBBON_PARENT_CATEGORIES.len(), 20);
        for category in RIBBON_PARENT_CATEGORIES {
            assert!(ribbon_parent_label(category, false).is_some(), "missing EN label for {category:?}");
            assert!(ribbon_parent_label(category, true).is_some(), "missing DE label for {category:?}");
        }
        assert_eq!(ribbon_parent_label("not-a-category", false), None);
    }

    /// 🗂️ Every group row `organize_context_menu` can emit resolves to a real label in BOTH locales —
    /// the 20 taxonomy ids AND the overflow bucket it synthesizes itself. `menu.group.more` is the one
    /// id outside the taxonomy, so resolving group rows through `ribbon_parent_label` alone rendered the
    /// overflow row with an empty label in the wgpu shell (React twin: the `ui.contextMenu.more` key).
    #[semio_framework_async_macros::async_test]
    async fn context_menu_group_label_covers_every_group_row_organize_context_menu_can_emit() {
        for category in RIBBON_PARENT_CATEGORIES {
            let id = format!("{CONTEXT_MENU_GROUP_ID_PREFIX}{category}");
            assert_eq!(context_menu_group_label(&id, false), ribbon_parent_label(category, false), "taxonomy rows keep the ribbon-parent table as their only source: {id}");
            assert!(context_menu_group_label(&id, true).is_some(), "missing DE label for {id}");
        }
        let overflow = format!("{CONTEXT_MENU_GROUP_ID_PREFIX}{CONTEXT_MENU_OVERFLOW_CATEGORY}");
        assert_eq!(context_menu_group_label(&overflow, false), Some("More"));
        assert_eq!(context_menu_group_label(&overflow, true), Some("Mehr"));
        assert_eq!(context_menu_group_label("shell.rename", false), None, "a leaf row is not a group row");
        assert_eq!(context_menu_group_label("menu.group.not-a-category", false), None);
    }

    /// 🗂️ The overflow row the organizer synthesizes when the row budget is exceeded carries the
    /// declared overflow id and no label of its own — the host resolves it. Pins the id the
    /// `context_menu_group_label` law above answers for against the producer that mints it.
    #[semio_framework_async_macros::async_test]
    async fn organize_context_menu_overflow_row_is_the_declared_unlabeled_group_id() {
        let mut items: Vec<ContextMenuItemSpec> = Vec::new();
        for index in 0..5 {
            items.push(menu_leaf(&format!("primary{index}")));
        }
        for category in ["hand", "selection", "lasso", "filter", "open", "save", "transfer", "transform"] {
            items.push(menu_group(category, vec![menu_leaf(&format!("{category}-child"))]));
        }
        let organized = organize_context_menu(items, &no_category);
        let overflow_id = format!("{CONTEXT_MENU_GROUP_ID_PREFIX}{CONTEXT_MENU_OVERFLOW_CATEGORY}");
        let overflow = organized.iter().find(|row| row.id == overflow_id).unwrap_or_else(|| panic!("40 rows overflow the budget: {organized:?}"));
        assert_eq!(overflow.label, None, "the host owns a group row's label");
        assert!(context_menu_group_label(&overflow.id, false).is_some(), "and the host must be able to resolve it");
        for row in &organized {
            if row.id.starts_with(CONTEXT_MENU_GROUP_ID_PREFIX) {
                assert!(context_menu_group_label(&row.id, false).is_some(), "every group row the organizer emits must resolve: {}", row.id);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn build_shell_context_menu_specs_shapes_arg_carrying_actions_and_appends_the_palette() {
        let actions = vec![
            ShellMenuAction { id: "shell.rename".into(), label: "Rename".into(), icon: None, keys: None, kind: "Mutation".into(), category: None, in_palette: true, arg_carrying: true },
            ShellMenuAction { id: "shell.hidden".into(), label: "Hidden".into(), icon: None, keys: None, kind: "Mutation".into(), category: None, in_palette: false, arg_carrying: false },
        ];
        let specs = build_shell_context_menu_specs(&actions, true);
        assert_eq!(specs.len(), 2, "the non-palette action is filtered out, the palette leaf is appended: {specs:?}");
        assert_eq!(specs[0].id, "shell.rename");
        assert_eq!(specs[0].action.as_deref(), Some("shell.openActionPane"), "arg-carrying actions route through the reserved action");
        assert_eq!(specs[0].args, Some(DslValue::Object(vec![("actionId".into(), DslValue::String("shell.rename".into()))])));
        assert_eq!(specs[1].id, "shell.openPalette");
        assert_eq!(specs[1].action.as_deref(), Some("shell.openPalette"));
    }
    //#endregion 🗂️OrganizeContextMenuTests
}
