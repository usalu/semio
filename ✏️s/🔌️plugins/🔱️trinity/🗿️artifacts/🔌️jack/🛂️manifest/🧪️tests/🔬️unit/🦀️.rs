// Generated manifest registry

pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::graph_manifest::nakagin;

use semio_framework_graph_manifest::Manifest;

pub const MANIFEST_IDS: &[&str] = &["nakagin"];

pub fn manifest_by_id(id: &str) -> Option<Manifest> {
    match id {
        "nakagin" => Some(nakagin::nakagin_manifest()),
        _ => None,
    }
}

