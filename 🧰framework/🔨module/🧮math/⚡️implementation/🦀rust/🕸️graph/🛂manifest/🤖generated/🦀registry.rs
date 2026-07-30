// Generated manifest registry

pub mod draw_layers;
pub mod flow_dag;
pub mod note_blocks;
pub mod concrete_forest;
pub mod puzzle2d_default;
pub mod puzzle3d_default;
pub mod puzzle5d_default;
pub mod wires;
pub mod s_resources;
pub mod nakagin;
pub mod rewrite_lhs;
pub mod rewrite_rhs;
pub mod writer_languages;

use crate::Manifest;

pub const MANIFEST_IDS: &[&str] = &["draw-layers", "flow-dag", "note-blocks", "concrete-forest", "puzzle2d-default", "puzzle3d-default", "puzzle5d-default", "wires", "s-resources", "nakagin", "rewrite-lhs", "rewrite-rhs", "writer-languages"];

pub fn manifest_by_id(id: &str) -> Option<Manifest> {
    match id {
        "draw-layers" => Some(draw_layers::draw_layers_manifest()),
        "flow-dag" => Some(flow_dag::flow_dag_manifest()),
        "note-blocks" => Some(note_blocks::note_blocks_manifest()),
        "concrete-forest" => Some(concrete_forest::concrete_forest_manifest()),
        "puzzle2d-default" => Some(puzzle2d_default::puzzle2d_default_manifest()),
        "puzzle3d-default" => Some(puzzle3d_default::puzzle3d_default_manifest()),
        "puzzle5d-default" => Some(puzzle5d_default::puzzle5d_default_manifest()),
        "wires" => Some(wires::wires_manifest()),
        "s-resources" => Some(s_resources::s_resources_manifest()),
        "nakagin" => Some(nakagin::nakagin_manifest()),
        "rewrite-lhs" => Some(rewrite_lhs::rewrite_lhs_manifest()),
        "rewrite-rhs" => Some(rewrite_rhs::rewrite_rhs_manifest()),
        "writer-languages" => Some(writer_languages::writer_languages_manifest()),
        _ => None,
    }
}
