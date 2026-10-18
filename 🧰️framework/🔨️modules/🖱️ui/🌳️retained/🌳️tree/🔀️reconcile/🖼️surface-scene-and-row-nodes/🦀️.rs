//! 🖼️ Surface scene and row nodes: the surface scene node, the drag data, the data attributes, the row actions, the tree window and the table section and row nodes. See [`super`].

use super::*;

// #region reconcile
//#region 🌳️DocumentTreeReconcile
/// 🗺️ Projects one `Component::Surface` record onto the `UiComponentSceneNode` every engine-surface
/// host already reads — the Rust twin of the React Interpreter's `surfacePropsToComponentSceneNode`,
/// including its identity rule (`surfaceHostIdentityV1`): `surface_id` is the OWNING DOCUMENT's
/// surface (the window the host is mounted into, i.e. the plugin host's own `surface_contexts` key),
/// `controller_id` scopes the pane-independent per-app registries, and `pane_id` carries the
/// program's own authored surface id — the one identity `SurfaceProps` really did drop.
///
/// A schema this target cannot decode yields a scene node with NO payload rather than `None`: the
/// host paints its own empty viewport and the document keeps every sibling, which is the contract's
/// "an unrecognised `doc_schema` must never reject the surrounding patch" rule.
pub(super) fn surface_scene_node(document: &UiDocumentTree, record: &UiNodeRecord, props: &ui_contract::SurfaceProps, surface: &str, controller: &str) -> UiComponentSceneNode {
    let mut node = UiComponentSceneNode {
        host_id: String::new(),
        surface_id: surface.to_string(),
        controller_id: controller.to_string(),
        component_kind: surface_kind(props.kind),
        pane_id: Some(record.key.as_str().to_string()),
        binding_id: None,
        presence: record_presence(record),
        menu: menu_ref(record),
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
    if ui_contract::parse_doc_schema(props.doc_schema.as_str()).is_err() || props.doc.bytes.is_empty() {
        return node;
    }
    match props.kind {
        ui_contract::SurfaceKind::World3d => {
            if let Ok(mut scene) = ui_scene::decode::<ui_scene::World3dScene>(props) {
                let declared = std::mem::take(&mut scene.lanes);
                merge_scene_lanes(document, record, &mut scene, &declared, |key| ui_scene::World3dSceneLane::from_body_key(key).map(ui_scene::World3dSceneLane::name));
                scene.lanes = declared;
                node.world_3d = Some(scene);
            }
        }
        ui_contract::SurfaceKind::NodeGraph => {
            if let Ok(mut scene) = ui_scene::decode::<ui_scene::NodeGraphScene>(props) {
                let declared = std::mem::take(&mut scene.lanes);
                merge_scene_lanes(document, record, &mut scene, &declared, |key| ui_scene::NodeGraphSceneLane::from_body_key(key).map(ui_scene::NodeGraphSceneLane::name));
                scene.lanes = declared;
                node.node_graph = Some(scene);
            }
        }
        ui_contract::SurfaceKind::Canvas2d => {
            if let Ok(mut scene) = ui_scene::decode::<ui_scene::Canvas2dScene>(props) {
                let declared = std::mem::take(&mut scene.lanes);
                merge_scene_lanes(document, record, &mut scene, &declared, |key| ui_scene::Canvas2dSceneLane::from_body_key(key).map(ui_scene::Canvas2dSceneLane::name));
                scene.lanes = declared;
                node.canvas_2d = Some(scene);
            }
        }
        ui_contract::SurfaceKind::TextEditor => {
            if let Ok(mut scene) = ui_scene::decode::<ui_scene::TextEditorScene>(props) {
                let declared = scene.lanes.clone();
                merge_scene_lanes(document, record, &mut scene, &declared, |key| (key == ui_scene::TEXT_EDITOR_BUFFER_LANE_KEY).then_some("buffer"));
                node.text_editor = Some(scene);
            }
        }
        ui_contract::SurfaceKind::Table => {
            if let Ok(mut scene) = ui_scene::decode::<ui_scene::TableScene>(props) {
                let declared = scene.lanes.clone();
                merge_scene_lanes(document, record, &mut scene, &declared, |key| match key {
                    ui_scene::TABLE_COLUMNS_LANE_KEY => Some("columns"),
                    ui_scene::TABLE_ROWS_LANE_KEY => Some("rows"),
                    _ => None,
                });
                node.table = Some(scene);
            }
        }
        ui_contract::SurfaceKind::Paint2d => {
            if let Ok(mut scene) = ui_scene::decode::<ui_scene::Paint2dScene>(props) {
                let declared = std::mem::take(&mut scene.lanes);
                merge_scene_lanes(document, record, &mut scene, &declared, |key| ui_scene::Paint2dSceneLane::from_body_key(key).map(ui_scene::Paint2dSceneLane::name));
                scene.lanes = declared;
                node.paint_2d = Some(scene);
            }
        }
        ui_contract::SurfaceKind::VirtualFileSystem => node.virtual_file_system = ui_scene::decode::<ui_scene::VirtualFileSystemScene>(props).ok(),
        ui_contract::SurfaceKind::TiledMap => node.tiled_map = ui_scene::decode::<ui_scene::TiledMapScene>(props).ok(),
        ui_contract::SurfaceKind::Board2d => {
            if let Ok(mut scene) = ui_scene::decode::<ui_scene::Board2dScene>(props) {
                let declared = std::mem::take(&mut scene.lanes);
                merge_scene_lanes(document, record, &mut scene, &declared, |key| ui_scene::Board2dSceneLane::from_body_key(key).map(ui_scene::Board2dSceneLane::name));
                scene.lanes = declared;
                node.board2d = Some(scene);
            }
        }
        ui_contract::SurfaceKind::IconRender => node.icon_render = ui_scene::decode::<ui_scene::IconRenderScene>(props).ok(),
        ui_contract::SurfaceKind::InkCanvas => node.ink_canvas = ui_scene::decode::<ui_scene::InkCanvasScene>(props).ok(),
        ui_contract::SurfaceKind::GraphTimeline => node.graph_timeline = ui_scene::decode::<ui_scene::GraphTimelineScene>(props).ok(),
        ui_contract::SurfaceKind::BlockList => node.block_list = ui_scene::decode::<ui_scene::BlockListScene>(props).ok(),
        ui_contract::SurfaceKind::DiffView => node.diff_view = ui_scene::decode::<ui_scene::DiffViewScene>(props).ok(),
        ui_contract::SurfaceKind::EventFeed => node.event_feed = ui_scene::decode::<ui_scene::EventFeedScene>(props).ok(),
    }
    node
}

pub(super) fn drag_data(map: Option<&ui_contract::UiFixedMap<ui_contract::UiText>>) -> Option<HashMap<String, String>> {
    map.map(|entries| entries.iter().map(|(key, value)| (key.as_str().to_string(), value.as_str().to_string())).collect())
}

pub(super) fn data_attributes(map: Option<&ui_contract::UiFixedMap<ui_contract::UiText>>) -> Option<HashMap<String, String>> {
    drag_data(map)
}

/// 🎬️ A row's actions as the retained item's trailing actions — each its verb on the row's ONE target.
pub(super) fn row_actions(row_actions: &ui_contract::UiFixedList<ui_contract::RowAction>, target: Option<&ui_contract::RowTarget>) -> Option<Vec<UiTreeItemAction>> {
    let target = target.filter(|_| !row_actions.is_empty())?;
    Some(row_actions.iter().map(|action| row_action(action, target)).collect())
}

fn row_action(action: &ui_contract::RowAction, target: &ui_contract::RowTarget) -> UiTreeItemAction {
    UiTreeItemAction {
        icon_id: icon_name(&action.icon),
        label: optional_contract_label(action.label.as_ref()),
        action: row_verb_action(target, &action.verb),
        placement: Some(match action.placement {
            ui_contract::RowActionPlacement::Row => UiTreeActionPlacement::Row,
            ui_contract::RowActionPlacement::Menu => UiTreeActionPlacement::Menu,
        }),
        disabled: action.disabled,
        reason: optional_contract_label(action.reason.as_ref()),
    }
}

/// 🪟️ Projects the contract's [`ui_contract::TreeWindow`] onto the legacy wgpu tree node's own
/// mirror. The two structs are deliberately separate types: the legacy `UiNode` wire shape is rooted
/// in `dsl`'s `ToValue`/`FromValue`, the contract's in `protocol::value`.
pub(super) fn tree_window(window: Option<&ui_contract::TreeWindow>) -> Option<UiTreeWindow> {
    window.map(|window| UiTreeWindow {
        total: window.total,
        offset: window.offset,
        row_extent: match window.row_extent {
            ui_contract::TreeWindowRowExtent::Standard => UiTreeWindowRowExtent::Standard,
            ui_contract::TreeWindowRowExtent::CompactText => UiTreeWindowRowExtent::CompactText,
            ui_contract::TreeWindowRowExtent::CompactSmallControl => UiTreeWindowRowExtent::CompactSmallControl,
            ui_contract::TreeWindowRowExtent::CompactControl => UiTreeWindowRowExtent::CompactControl,
        },
    })
}

/// 📊️ A `Component::Table` as the ONE section of the retained `Tree` it paints through, keyed by the table's own record key:
/// a table has no section record, so its rows mount directly under the table node, and `TableProps::window` windows it
/// exactly as it windows a tree section. The column grid stays on the document — `TableProps::columns`/`actions_label` and
/// every row's `TableRowProps::cells` are read through the node binding (`mounted_layout::document_table`) by the layout, the
/// painter, the pointer router and the accessibility projection, so no second copy of a cell travels in the retained spec.
pub(super) fn table_section(document: &UiDocumentTree, record: &UiNodeRecord, props: &ui_contract::TableProps) -> UiTreeSectionNode {
    UiTreeSectionNode {
        header_toolbar: None,
        window: tree_window(props.window.as_ref()),
        id: record.key.as_str().to_string(),
        label: None,
        default_open: Some(true),
        presence: UiPresence::default(),
        items: record.children.iter().filter_map(|child| document.record(*child)).filter_map(table_row_item).collect(),
    }
}

/// 📊️ One `Component::TableRow` as its tree item: named by its first cell (React's row name), activated by its target's
/// activation verb, its `RowAction`s the item's trailing actions — the one representation every target paints.
fn table_row_item(record: &UiNodeRecord) -> Option<UiTreeItemNode> {
    let ui_contract::Component::TableRow(props) = &record.component else { return None };
    let mut item = UiTreeItemNode::base(record.key.as_str(), Label::data(props.cells.get(0).map_or(record.key.as_str(), |cell| cell.as_str())));
    item.presence = record_presence(record);
    item.action = row_activation(record);
    item.actions = row_actions(&props.row_actions, props.target.as_ref());
    item.menu = menu_ref(record);
    Some(item)
}
//#endregion 🌳️DocumentTreeReconcile
// #endregion reconcile
