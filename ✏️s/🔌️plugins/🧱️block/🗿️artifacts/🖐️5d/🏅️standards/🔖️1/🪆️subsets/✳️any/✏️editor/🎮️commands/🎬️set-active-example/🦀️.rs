//! 🎬️ Block 5D play app command — `set-active-example`.

//#region 🔖️ExampleIds
pub const BLOCK5D_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";
pub const BLOCK5D_EXAMPLE_CAPSULE: &str = "nakagin-capsule";
//#endregion 🔖️ExampleIds

//#region 🔖️ReplaceDocument
/// ✏️ Emits the minimal ordered batch of semantic mutations that carries `current` to `next` — the
/// whole-document-load replacement for a document-wide replace mutation (banned outright).
fn replace_document_operations(current: &Block5dSnapshot, next: &Block5dSnapshot) -> Vec<Block5dMutation> {
    use crate::artifacts::block5d::mutations as m;
    let mut ops = Vec::new();

    if next.part_kind.name != current.part_kind.name {
        ops.push(m::rename_part_kind(next.part_kind.name.clone()));
    }
    if next.part_kind.label != current.part_kind.label {
        ops.push(m::change_part_kind_label(next.part_kind.label.clone()));
    }
    if next.part_kind.variant != current.part_kind.variant {
        ops.push(m::change_part_kind_variant(next.part_kind.variant.clone()));
    }
    if next.part_kind.description != current.part_kind.description {
        ops.push(m::change_part_kind_description(next.part_kind.description.clone()));
    }
    if next.part_kind.icon != current.part_kind.icon {
        ops.push(m::change_part_kind_icon(next.part_kind.icon.clone()));
    }
    if next.part_kind.unit != current.part_kind.unit {
        ops.push(m::change_part_kind_unit(next.part_kind.unit.clone()));
    }

    if next.part_2d != current.part_2d {
        ops.push(m::update_part_2d(next.part_2d.shape.clone(), next.part_2d.radius, next.part_2d.width, next.part_2d.height, next.part_2d.color.clone(), next.part_2d.icon_kind.clone()));
    }
    if next.part_3d != current.part_3d {
        ops.push(m::update_part_3d(next.part_3d.orientation, next.part_3d.scale));
    }

    for representation in &current.representations {
        if !next.representations.iter().any(|entry| entry.id == representation.id) {
            ops.push(m::delete_representation(representation.id.clone()));
        }
    }
    for representation in &next.representations {
        match current.representations.iter().find(|entry| entry.id == representation.id) {
            None => ops.push(m::create_representation(representation.clone())),
            Some(prior) => {
                if prior.name != representation.name {
                    ops.push(m::rename_representation(representation.id.clone(), representation.name.clone()));
                }
                if prior.mesh_url != representation.mesh_url {
                    ops.push(m::change_representation_mesh_url(representation.id.clone(), representation.mesh_url.clone()));
                }
                if prior.lod != representation.lod {
                    ops.push(m::change_representation_lod(representation.id.clone(), representation.lod.clone()));
                }
                if prior.description != representation.description {
                    ops.push(m::change_representation_description(representation.id.clone(), representation.description.clone()));
                }
                for tag in &prior.tags {
                    if !representation.tags.contains(tag) {
                        ops.push(m::remove_representation_tag(representation.id.clone(), tag.clone()));
                    }
                }
                for tag in &representation.tags {
                    if !prior.tags.contains(tag) {
                        ops.push(m::add_representation_tag(representation.id.clone(), tag.clone()));
                    }
                }
                for attribute in &prior.attributes {
                    match representation.attributes.iter().find(|entry| entry.key == attribute.key) {
                        None => ops.push(m::remove_representation_attribute(representation.id.clone(), attribute.key.clone())),
                        Some(updated) if updated != attribute => {
                            ops.push(m::remove_representation_attribute(representation.id.clone(), attribute.key.clone()));
                            ops.push(m::add_representation_attribute(representation.id.clone(), updated.clone()));
                        }
                        _ => {}
                    }
                }
                for attribute in &representation.attributes {
                    if !prior.attributes.iter().any(|entry| entry.key == attribute.key) {
                        ops.push(m::add_representation_attribute(representation.id.clone(), attribute.clone()));
                    }
                }
            }
        }
    }

    for grip_kind in &current.grip_kinds {
        if !next.grip_kinds.iter().any(|entry| entry.id == grip_kind.id) {
            ops.push(m::delete_grip_kind(grip_kind.id.clone()));
        }
    }
    for grip_kind in &next.grip_kinds {
        match current.grip_kinds.iter().find(|entry| entry.id == grip_kind.id) {
            None => ops.push(m::create_grip_kind(grip_kind.clone())),
            Some(prior) => {
                if prior.name != grip_kind.name {
                    ops.push(m::rename_grip_kind(grip_kind.id.clone(), grip_kind.name.clone()));
                }
                if prior.label != grip_kind.label {
                    ops.push(m::change_grip_kind_label(grip_kind.id.clone(), grip_kind.label.clone()));
                }
                if prior.color != grip_kind.color {
                    ops.push(m::change_grip_kind_color(grip_kind.id.clone(), grip_kind.color.clone()));
                }
                if prior.default_rope_kind != grip_kind.default_rope_kind {
                    ops.push(m::change_grip_kind_default_rope_kind(grip_kind.id.clone(), grip_kind.default_rope_kind.clone()));
                }
            }
        }
    }

    for grip in &current.grips {
        if !next.grips.iter().any(|entry| entry.id == grip.id) {
            ops.push(m::delete_grip(grip.id.clone()));
        }
    }
    for grip in &next.grips {
        match current.grips.iter().find(|entry| entry.id == grip.id) {
            None => ops.push(m::create_grip(grip.clone())),
            Some(prior) => {
                if prior.angle != grip.angle || prior.radius_2d != grip.radius_2d {
                    ops.push(m::move_grip_2d(grip.id.clone(), grip.angle, grip.radius_2d));
                }
                if prior.position != grip.position || prior.direction != grip.direction {
                    ops.push(m::move_grip_3d(grip.id.clone(), grip.position, grip.direction));
                }
                if prior.radius_3d != grip.radius_3d {
                    ops.push(m::resize_grip_3d(grip.id.clone(), grip.radius_3d));
                }
                if prior.grip_kind != grip.grip_kind {
                    ops.push(m::change_grip_grip_kind(grip.id.clone(), grip.grip_kind.clone()));
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
    if next.camera3d.position != current.camera3d.position || next.camera3d.target != current.camera3d.target {
        ops.push(m::move_camera3d(next.camera3d.position, next.camera3d.target));
    }
    if next.camera3d.zoom != current.camera3d.zoom {
        ops.push(m::scale_camera3d(next.camera3d.zoom));
    }
    if next.meta.description != current.meta.description {
        ops.push(m::change_meta_description(next.meta.description.clone()));
    }

    ops
}
//#endregion 🔖️ReplaceDocument

use crate::artifacts::block5d::op::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::editor::block5d::config::{Block5dConfig, Block5dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "setActiveExample")]
pub struct SetActiveExample {
    pub id: String,
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
    let example = match payload.id.as_str() {
        BLOCK5D_EXAMPLE_FOREST_LEFT => crate::artifacts::block5d::dsl::parse_dsl(crate::artifacts::block5d::dsl::BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
        BLOCK5D_EXAMPLE_CAPSULE => crate::artifacts::block5d::dsl::parse_dsl(crate::artifacts::block5d::dsl::BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).ok(),
        _ => None,
    };
    match example {
        Some(document) => Ok(Emit::mutations(replace_document_operations(doc.snapshot, &document))),
        None => Ok(Emit::default()),
    }
}
