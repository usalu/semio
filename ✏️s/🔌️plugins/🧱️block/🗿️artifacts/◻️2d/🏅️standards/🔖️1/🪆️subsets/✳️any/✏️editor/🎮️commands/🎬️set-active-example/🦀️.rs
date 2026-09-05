//! 🎬️ Block 2D play app command — `set-active-example`.

//#region 🔖️ExampleIds
pub const BLOCK2D_EXAMPLE_LEFT: &str = "hexagonal-cut-concrete-forest-left";
pub const BLOCK2D_EXAMPLE_RIGHT: &str = "hexagonal-cut-concrete-forest-right";
//#endregion 🔖️ExampleIds

//#region 🔖️ReplaceDocument
/// ✏️ Emits the minimal ordered batch of semantic mutations that carries `current` to `next` — the
/// whole-document-load replacement for a document-wide replace mutation (banned outright).
async fn replace_document_operations(current: &Block2dSnapshot, next: &Block2dSnapshot) -> Vec<Block2dMutation> {
    use crate::artifacts::block2d::mutations as m;
    let mut ops = Vec::new();

    if next.node_kind.name != current.node_kind.name {
        ops.push(m::rename_node_kind(next.node_kind.name.clone()));
    }
    if next.node_kind.label != current.node_kind.label {
        ops.push(m::change_node_kind_label(next.node_kind.label.clone()));
    }
    if next.node_kind.variant != current.node_kind.variant {
        ops.push(m::change_node_kind_variant(next.node_kind.variant.clone()));
    }
    if next.node_kind.description != current.node_kind.description {
        ops.push(m::change_node_kind_description(next.node_kind.description.clone()));
    }
    if next.node_kind.icon != current.node_kind.icon {
        ops.push(m::change_node_kind_icon(next.node_kind.icon.clone()));
    }
    if next.node_kind.unit != current.node_kind.unit {
        ops.push(m::change_node_kind_unit(next.node_kind.unit.clone()));
    }

    if next.presentation != current.presentation {
        ops.push(m::update_presentation(next.presentation.shape.clone(), next.presentation.radius, next.presentation.width, next.presentation.height, next.presentation.color.clone(), next.presentation.icon_kind.clone()));
    }

    for handle_kind in &current.handle_kinds {
        if !next.handle_kinds.iter().any(|entry| entry.id == handle_kind.id) {
            ops.push(m::delete_handle_kind(handle_kind.id.clone()));
        }
    }
    for handle_kind in &next.handle_kinds {
        match current.handle_kinds.iter().find(|entry| entry.id == handle_kind.id) {
            None => ops.push(m::create_handle_kind(handle_kind.clone())),
            Some(prior) => {
                if prior.name != handle_kind.name {
                    ops.push(m::rename_handle_kind(handle_kind.id.clone(), handle_kind.name.clone()));
                }
                if prior.label != handle_kind.label {
                    ops.push(m::change_handle_kind_label(handle_kind.id.clone(), handle_kind.label.clone()));
                }
                if prior.color != handle_kind.color {
                    ops.push(m::change_handle_kind_color(handle_kind.id.clone(), handle_kind.color.clone()));
                }
                if prior.default_wire_kind != handle_kind.default_wire_kind {
                    ops.push(m::change_handle_kind_default_wire_kind(handle_kind.id.clone(), handle_kind.default_wire_kind.clone()));
                }
            }
        }
    }

    for handle in &current.handles {
        if !next.handles.iter().any(|entry| entry.id == handle.id) {
            ops.push(m::delete_handle(handle.id.clone()));
        }
    }
    for handle in &next.handles {
        match current.handles.iter().find(|entry| entry.id == handle.id) {
            None => ops.push(m::create_handle(handle.clone())),
            Some(prior) => {
                if prior.angle != handle.angle || prior.radius != handle.radius {
                    ops.push(m::move_handle(handle.id.clone(), handle.angle, handle.radius));
                }
                if prior.handle_kind != handle.handle_kind {
                    ops.push(m::change_handle_handle_kind(handle.id.clone(), handle.handle_kind.clone()));
                }
            }
        }
    }

    for rule in &current.compatibility {
        match next.compatibility.iter().find(|entry| entry.id == rule.id) {
            None => ops.push(m::remove_compatibility_rule(rule.id.clone())),
            Some(updated) if updated != rule => {
                ops.push(m::remove_compatibility_rule(rule.id.clone()));
                ops.push(m::add_compatibility_rule(updated.clone()));
            }
            _ => {}
        }
    }
    for rule in &next.compatibility {
        if !current.compatibility.iter().any(|entry| entry.id == rule.id) {
            ops.push(m::add_compatibility_rule(rule.clone()));
        }
    }

    for attribute in &current.attributes {
        match next.attributes.iter().find(|entry| entry.key == attribute.key) {
            None => ops.push(m::remove_attribute(attribute.key.clone())),
            Some(updated) if updated != attribute => {
                ops.push(m::remove_attribute(attribute.key.clone()));
                ops.push(m::add_attribute(updated.clone()));
            }
            _ => {}
        }
    }
    for attribute in &next.attributes {
        if !current.attributes.iter().any(|entry| entry.key == attribute.key) {
            ops.push(m::add_attribute(attribute.clone()));
        }
    }

    for author in &current.authors {
        match next.authors.iter().find(|entry| entry.id == author.id) {
            None => ops.push(m::remove_author(author.id.clone())),
            Some(updated) if updated != author => {
                ops.push(m::remove_author(author.id.clone()));
                ops.push(m::add_author(updated.clone()));
            }
            _ => {}
        }
    }
    for author in &next.authors {
        if !current.authors.iter().any(|entry| entry.id == author.id) {
            ops.push(m::add_author(author.clone()));
        }
    }

    if next.camera2d.x != current.camera2d.x || next.camera2d.y != current.camera2d.y {
        ops.push(m::move_camera2d(next.camera2d.x, next.camera2d.y));
    }
    if next.camera2d.zoom != current.camera2d.zoom {
        ops.push(m::scale_camera2d(next.camera2d.zoom));
    }
    if next.meta.description != current.meta.description {
        ops.push(m::change_meta_description(next.meta.description.clone()));
    }

    ops
}
//#endregion 🔖️ReplaceDocument

use crate::artifacts::block2d::op::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::editor::block2d::config::{Block2dConfig, Block2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "setActiveExample")]
pub struct SetActiveExample {
    pub id: String,
}

pub async fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
    let example = match payload.id.as_str() {
        BLOCK2D_EXAMPLE_LEFT => crate::artifacts::block2d::dsl::parse_dsl(crate::artifacts::block2d::dsl::BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
        BLOCK2D_EXAMPLE_RIGHT => crate::artifacts::block2d::dsl::parse_dsl(crate::artifacts::block2d::dsl::BLOCK2D_CONCRETE_FOREST_RIGHT_EXAMPLE_TEXT).ok(),
        _ => None,
    };
    match example {
        Some(document) => Ok(Emit::mutations(replace_document_operations(doc.snapshot, &document))),
        None => Ok(Emit::default()),
    }
}
