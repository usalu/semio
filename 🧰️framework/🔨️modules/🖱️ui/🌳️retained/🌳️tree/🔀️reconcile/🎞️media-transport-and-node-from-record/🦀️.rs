//! 🎞️ Media transport and node from record: the media transport extension validation and host status and the `ui_node_from_record` conversion. See [`super`].

use super::*;

// #region reconcile
//#region 🌳️DocumentTreeReconcile
/// 🎬️ Host-owned extension address reserved for the versioned audio/video transport boundary.
pub const MEDIA_TRANSPORT_EXTENSION_ID: &str = "framework.media.transport@1";

fn json_object_has_exact_keys(object: &serde_json::Map<String, serde_json::Value>, keys: &[&str]) -> bool {
    object.len() == keys.len() && keys.iter().all(|key| object.contains_key(*key))
}

fn json_string_length(value: Option<&serde_json::Value>, minimum: usize, maximum: usize) -> bool {
    value.and_then(serde_json::Value::as_str).is_some_and(|value| (minimum..=maximum).contains(&value.chars().count()))
}

fn media_transport_revision(value: Option<&serde_json::Value>) -> bool {
    value
        .and_then(serde_json::Value::as_str)
        .is_some_and(|value| (value == "0" || (value.len() <= 20 && value.as_bytes().first().is_some_and(|first| (b'1'..=b'9').contains(first)) && value.as_bytes()[1..].iter().all(u8::is_ascii_digit))) && value.parse::<u64>().is_ok())
}

pub fn media_transport_contract_valid(params_json: &str) -> bool {
    serde_json::from_str(params_json).is_ok_and(|value| media_transport_value_valid(&value))
}

fn media_transport_value_valid(value: &serde_json::Value) -> bool {
    const ROOT_KEYS: &[&str] = &["schemaVersion", "kind", "mediaType", "revision", "durationMs", "positionMs", "selectionStartMs", "selectionEndMs", "locale", "labels", "resource", "capability", "hostContentHeight"];
    const LABEL_KEYS: &[&str] = &["play", "pause", "seek", "position", "duration", "selectionStart", "selectionEnd", "loading", "progress", "cancel", "unsupported", "unknownDuration", "audio", "video"];
    let Some(root) = value.as_object().filter(|root| json_object_has_exact_keys(root, ROOT_KEYS)) else { return false };
    if root.get("schemaVersion").and_then(serde_json::Value::as_u64) != Some(1)
        || !matches!(root.get("kind").and_then(serde_json::Value::as_str), Some("audio" | "video"))
        || !json_string_length(root.get("mediaType"), 1, 128)
        || !media_transport_revision(root.get("revision"))
        || !matches!(root.get("locale").and_then(serde_json::Value::as_str), Some("en" | "de"))
    {
        return false;
    }
    let duration = match root.get("durationMs") {
        Some(serde_json::Value::Null) => None,
        Some(value) => match value.as_u64().filter(|value| (1..=9_007_199_254_740_991).contains(value)) {
            Some(value) => Some(value),
            None => return false,
        },
        None => return false,
    };
    let Some(position) = root.get("positionMs").and_then(serde_json::Value::as_u64).filter(|value| *value <= 9_007_199_254_740_991) else { return false };
    let selection_start = match root.get("selectionStartMs") {
        Some(serde_json::Value::Null) => None,
        Some(value) => match value.as_u64().filter(|value| *value <= 9_007_199_254_740_991) {
            Some(value) => Some(value),
            None => return false,
        },
        None => return false,
    };
    let selection_end = match root.get("selectionEndMs") {
        Some(serde_json::Value::Null) => None,
        Some(value) => match value.as_u64().filter(|value| (1..=9_007_199_254_740_991).contains(value)) {
            Some(value) => Some(value),
            None => return false,
        },
        None => return false,
    };
    if duration.is_some_and(|duration| position > duration)
        || duration.is_none() && (position != 0 || selection_start.is_some() || selection_end.is_some())
        || selection_start.is_some() != selection_end.is_some()
        || selection_start.zip(selection_end).is_some_and(|(start, end)| start >= end || duration.is_none_or(|duration| end > duration))
    {
        return false;
    }
    let Some(labels) = root.get("labels").and_then(serde_json::Value::as_object).filter(|labels| json_object_has_exact_keys(labels, LABEL_KEYS)) else { return false };
    if !LABEL_KEYS.iter().all(|key| labels.get(*key).is_some_and(serde_json::Value::is_string)) {
        return false;
    }
    let resource_valid = match root.get("resource") {
        Some(serde_json::Value::Null) => true,
        Some(serde_json::Value::Object(resource)) => {
            json_object_has_exact_keys(resource, &["kind", "controllerId", "appInstanceId", "parentDocumentId", "outputPort", "revision", "generation"])
                && resource.get("kind").and_then(serde_json::Value::as_str) == Some("artifact-media-export")
                && json_string_length(resource.get("controllerId"), 1, 256)
                && resource.get("appInstanceId").and_then(serde_json::Value::as_u64).is_some_and(|value| value <= u32::MAX as u64)
                && json_string_length(resource.get("parentDocumentId"), 1, 512)
                && resource.get("outputPort").and_then(serde_json::Value::as_str) == Some("playback:out")
                && media_transport_revision(resource.get("revision"))
                && resource.get("revision") == root.get("revision")
                && media_transport_revision(resource.get("generation"))
        }
        _ => false,
    };
    if !resource_valid {
        return false;
    }
    let Some(capability) = root.get("capability").and_then(serde_json::Value::as_object).filter(|capability| json_object_has_exact_keys(capability, &["status", "reason"])) else { return false };
    let Some(capability_status @ ("ready" | "loading" | "unsupported")) = capability.get("status").and_then(serde_json::Value::as_str) else { return false };
    let reason_valid = match capability.get("reason") {
        Some(serde_json::Value::Null) => true,
        Some(reason) => reason.as_str().is_some_and(|reason| reason.chars().count() <= 512),
        None => false,
    };
    let height_valid = root.get("hostContentHeight").and_then(serde_json::Value::as_f64).is_some_and(|height| height.is_finite() && (0.0..=4096.0).contains(&height));
    reason_valid && height_valid && (capability_status != "ready" || root.get("resource").is_some_and(serde_json::Value::is_object))
}

fn media_transport_host_status(extension: &str, params_json: &str) -> Option<String> {
    if extension != MEDIA_TRANSPORT_EXTENSION_ID {
        return None;
    }
    let Ok(value) = serde_json::from_str::<serde_json::Value>(params_json) else { return Some(format!("{MEDIA_TRANSPORT_EXTENSION_ID} · invalid-contract")) };
    let locale = value.get("locale").and_then(serde_json::Value::as_str);
    if media_transport_value_valid(&value) {
        return value.get("labels").and_then(|labels| labels.get("unsupported")).and_then(serde_json::Value::as_str).map(str::to_string);
    }
    Some(match locale {
        Some("en") => "Invalid media transport contract".to_string(),
        Some("de") => "Ungültiger Medientransportvertrag".to_string(),
        _ => format!("{MEDIA_TRANSPORT_EXTENSION_ID} · invalid-contract"),
    })
}

/// 🧩️ Projects ONE published record onto the retained `UiNode` this target paints — the missing half
/// of the retained-document pipeline (`📓️wgpu-blank-paint-2026-09-12.md` §5).
///
/// Children are deliberately left EMPTY on every container variant: the arena's own
/// parent/first-child/sibling links are the tree, and `mounted_layout`/`paint`/`scene_slots` all walk
/// those links, never a spec's inline `children`. The two components that do own their subtree
/// (`Surface`, `Tree` — see [`record_consumes_subtree`]) are the exception and carry it inline.
pub fn ui_node_from_record(document: &UiDocumentTree, record: &UiNodeRecord, surface: &str, controller: &str) -> UiNode {
    let presence = record_presence(record);
    let menu = menu_ref(record);
    match &record.component {
        ui_contract::Component::Container(props) => match props.role {
            ui_contract::ContainerRole::Section => UiNode::Section(UiSectionNode { id: record.key.as_str().to_string(), label: optional_contract_label(props.label.as_ref()), default_open: props.default_open, presence, menu, children: Vec::new() }),
            ui_contract::ContainerRole::Group => {
                UiNode::Group(UiGroupNode { id: record.key.as_str().to_string(), label: optional_contract_label(props.label.as_ref()).unwrap_or_else(|| Label::data("")), default_open: props.default_open, presence, menu, children: Vec::new() })
            }
            ui_contract::ContainerRole::Field => UiNode::Field(UiFieldNode {
                id: record.key.as_str().to_string(),
                label: optional_contract_label(props.label.as_ref()).unwrap_or_else(|| Label::data("")),
                description: props.description.as_ref().map(|value| value.as_str().to_string()),
                required: props.required,
                error: props.error.as_ref().map(|value| value.as_str().to_string()),
                // 🍃️ The field's real child is an ARENA child (the record's `children[0]`); this
                // placeholder only satisfies `UiFieldNode`'s non-optional shape and is never painted
                // — `paint_node_step`'s `Field` arm paints the label and nothing of `child`.
                child: Box::new(UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None })),
                presence,
                menu,
            }),
            ui_contract::ContainerRole::Plain | ui_contract::ContainerRole::Form | ui_contract::ContainerRole::Toolbar => {
                let (direction, gap, padding) = stack_metrics(&record.layout);
                UiNode::Stack(UiStackNode {
                    direction,
                    gap,
                    padding,
                    id: Some(record.key.as_str().to_string()),
                    presence,
                    activate: record_action(record, ui_contract::Trigger::Activate, controller),
                    drop_action: record_action(record, ui_contract::Trigger::Drop, controller),
                    drop_overlay: drop_overlay(props.drop_overlay.as_ref()),
                    menu,
                    children: Vec::new(),
                })
            }
        },
        ui_contract::Component::Text(props) => UiNode::Text(UiTextNode { value: contract_label(&props.value), emphasize: props.emphasize, data_attributes: data_attributes(props.data_attributes.as_ref()), presence, menu }),
        ui_contract::Component::Separator(_) => UiNode::Separator(UiSeparatorNode { presence, menu }),
        ui_contract::Component::Button(_) => button_node(record, controller),
        ui_contract::Component::Input(_) => input_node(record, controller),
        ui_contract::Component::Select(_) => select_node(record, controller),
        ui_contract::Component::Toggle(_) => toggle_node(record, controller),
        ui_contract::Component::KeyValueList(_) => key_value_node(record),
        ui_contract::Component::Slider(_) => slider_node(record, controller),
        ui_contract::Component::NumberStepper(_) => number_stepper_node(record, controller),
        ui_contract::Component::Ring(_) => ring_node(record, controller),
        ui_contract::Component::IconSelect(_) => icon_select_node(record, controller),
        ui_contract::Component::Progress(props) => progress_node(record, props, presence, menu),
        ui_contract::Component::Image(props) => UiNode::Image(UiImageNode { id: record.key.as_str().to_string(), src: props.src.as_str().to_string(), alt: optional_contract_label(props.alt.as_ref()), presence, menu }),
        ui_contract::Component::Tree(props) => {
            let sections = record
                .children
                .iter()
                .filter_map(|child_id| document.record(*child_id))
                .map(|child| match &child.component {
                    ui_contract::Component::TreeSection(section) => UiTreeSectionNode {
                        header_toolbar: section.header_toolbar.and_then(|toolbar| tree_toolbar(document, toolbar, controller)),
                        window: tree_window(section.window.as_ref()),
                        id: child.key.as_str().to_string(),
                        label: optional_contract_label(section.label.as_ref()),
                        default_open: section.default_open,
                        presence: record_presence(child),
                        items: child.children.iter().filter(|item_id| section.header_toolbar != Some(**item_id)).filter_map(|item_id| document.record(*item_id)).map(|item| tree_item(document, item, surface, controller, 1)).collect(),
                    },
                    _ => UiTreeSectionNode { header_toolbar: None, window: None, id: child.key.as_str().to_string(), label: None, default_open: None, presence: record_presence(child), items: vec![tree_item(document, child, surface, controller, 1)] },
                })
                .collect();
            UiNode::Tree(UiTreeNode {
                presentation: props.presentation,
                sections,
                presence,
                drop_action: record_action(record, ui_contract::Trigger::Drop, controller),
                menu,
                interaction_domain: props.interaction_domain.as_ref().map(|value| value.as_str().to_string()),
            })
        }
        // 🌳️ A tree's section and item records mount as the keyed `Stack` ROWS this engine's
        // interactive sync resolves by id — the document-path twin of `children_of`'s `Tree` arm
        // (`tree_section_row`/`tree_item_row`). Their content is painted from the owning `Tree`'s own
        // inline spec; these rows carry identity, layout and drag/drop state. 🪟️ `TreeSectionProps.
        // window`/`TreeItemProps.window` are therefore NOT read here: the spacer pitch belongs to the
        // painted spec (`UiTreeSectionNode.window`/`UiTreeItemNode.window`, stamped in the `Tree` arm
        // above), not to the identity row, which has no extent of its own.
        ui_contract::Component::TreeSection(_) | ui_contract::Component::TreeItem(_) | ui_contract::Component::TableRow(_) => UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: None,
            padding: None,
            id: Some(record.key.as_str().to_string()),
            presence,
            activate: match record.component {
                ui_contract::Component::TreeSection(_) => record_action(record, ui_contract::Trigger::Activate, controller),
                _ => row_activation(record),
            },
            drop_action: record_action(record, ui_contract::Trigger::Drop, controller),
            drop_overlay: None,
            menu,
            children: Vec::new(),
        }),
        ui_contract::Component::Table(props) => UiNode::Tree(UiTreeNode {
            presentation: ui_contract::TreePresentation::Standard,
            sections: vec![table_section(document, record, props)],
            presence,
            drop_action: record_action(record, ui_contract::Trigger::Drop, controller),
            menu,
            interaction_domain: None,
        }),
        ui_contract::Component::Surface(props) => UiNode::ComponentScene(surface_scene_node(document, record, props, surface, controller)),
        ui_contract::Component::Extension(props) => {
            let body_key = props.extension.as_str().to_string();
            let params_json = serde_json::to_string(&props.props).unwrap_or_else(|_| "null".to_string());
            let host_status = media_transport_host_status(&body_key, &params_json);
            UiNode::ExternalSlot(semio_framework_ui_declaration::ui::UiExternalSlotNode { plugin_id: surface_plugin_id(surface), app_id: controller.to_string(), body_key, params_json, host_status, presence, menu })
        }
    }
}
//#endregion 🌳️DocumentTreeReconcile
// #endregion reconcile
