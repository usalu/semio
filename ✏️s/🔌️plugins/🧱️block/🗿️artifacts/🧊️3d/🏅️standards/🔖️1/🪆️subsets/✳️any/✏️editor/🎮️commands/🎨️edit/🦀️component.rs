//! 🎨️ 🎨️ Block 3D play app commands command — `edit`.

//#region 🔖️ExampleIds
pub const BLOCK3D_EXAMPLE_CAPSULE: &str = "nakagin-capsule";
pub const BLOCK3D_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";
//#endregion 🔖️ExampleIds

//#region 🔖️ReplaceDocument
/// ✏️ Emits the minimal ordered batch of semantic mutations that carries `current` to `next` — the
/// whole-document-load replacement for a document-wide replace mutation (banned outright).
fn replace_document_operations(current: &crate::artifacts::block3d::Block3dSnapshot, next: &crate::artifacts::block3d::Block3dSnapshot) -> Vec<crate::artifacts::block3d::op::Block3dMutation> {
    use crate::artifacts::block3d::mutations as m;
    let mut ops = Vec::new();

    if next.object_kind.name != current.object_kind.name {
        ops.push(m::rename_object_kind(next.object_kind.name.clone()));
    }
    if next.object_kind.label != current.object_kind.label {
        ops.push(m::change_object_kind_label(next.object_kind.label.clone()));
    }
    if next.object_kind.variant != current.object_kind.variant {
        ops.push(m::change_object_kind_variant(next.object_kind.variant.clone()));
    }
    if next.object_kind.description != current.object_kind.description {
        ops.push(m::change_object_kind_description(next.object_kind.description.clone()));
    }
    if next.object_kind.icon != current.object_kind.icon {
        ops.push(m::change_object_kind_icon(next.object_kind.icon.clone()));
    }
    if next.object_kind.unit != current.object_kind.unit {
        ops.push(m::change_object_kind_unit(next.object_kind.unit.clone()));
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

    let current_vortex_kinds = crate::artifacts::block3d::vortex_kinds_of(current);
    let next_vortex_kinds = crate::artifacts::block3d::vortex_kinds_of(next);
    for vortex_kind in &current_vortex_kinds {
        if !next_vortex_kinds.iter().any(|entry| entry.id == vortex_kind.id) {
            ops.push(m::delete_vortex_kind(vortex_kind.id.clone()));
        }
    }
    for vortex_kind in &next_vortex_kinds {
        match current_vortex_kinds.iter().find(|entry| entry.id == vortex_kind.id) {
            None => ops.push(m::create_vortex_kind(vortex_kind.clone())),
            Some(prior) => {
                if prior.name != vortex_kind.name {
                    ops.push(m::rename_vortex_kind(vortex_kind.id.clone(), vortex_kind.name.clone()));
                }
                if prior.label != vortex_kind.label {
                    ops.push(m::change_vortex_kind_label(vortex_kind.id.clone(), vortex_kind.label.clone()));
                }
                if prior.color != vortex_kind.color {
                    ops.push(m::change_vortex_kind_color(vortex_kind.id.clone(), vortex_kind.color.clone()));
                }
                if prior.default_cable_kind != vortex_kind.default_cable_kind {
                    ops.push(m::change_vortex_kind_default_cable_kind(vortex_kind.id.clone(), vortex_kind.default_cable_kind.clone()));
                }
            }
        }
    }

    for vortex in &current.vortices {
        if !next.vortices.iter().any(|entry| entry.id == vortex.id) {
            ops.push(m::delete_vortex(vortex.id.clone()));
        }
    }
    for vortex in &next.vortices {
        match current.vortices.iter().find(|entry| entry.id == vortex.id) {
            None => ops.push(m::create_vortex(vortex.clone())),
            Some(prior) => {
                if prior.position != vortex.position || prior.direction != vortex.direction {
                    ops.push(m::move_vortex(vortex.id.clone(), vortex.position, vortex.direction));
                }
                if prior.radius != vortex.radius {
                    ops.push(m::resize_vortex(vortex.id.clone(), vortex.radius));
                }
                if prior.vortex_kind != vortex.vortex_kind {
                    ops.push(m::change_vortex_vortex_kind(vortex.id.clone(), vortex.vortex_kind.clone()));
                }
                if prior.label != vortex.label {
                    ops.push(m::change_vortex_label(vortex.id.clone(), vortex.label.clone()));
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

use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "edit")]
pub struct Edit {
    pub text: String,
}

pub fn handle(payload: &Edit, doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    match serde_json::from_str::<Block3dSnapshot>(&payload.text) {
        Ok(document) if &document != doc.snapshot => Ok(Emit::mutations(replace_document_operations(doc.snapshot, &document))),
        _ => Ok(Emit::default()),
    }
}
