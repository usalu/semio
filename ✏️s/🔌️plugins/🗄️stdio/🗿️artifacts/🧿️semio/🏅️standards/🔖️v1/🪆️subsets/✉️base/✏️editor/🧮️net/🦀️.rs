//! 🧮️ Net of one snapshot edit as envelope domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `envelope` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::mutations::{apply_brep, apply_mesh, apply_model, apply_value, apply_document, apply_cad, apply_drawing, apply_image, apply_video, apply_audio, apply_animation, apply_presentation, apply_flow, apply_text, apply_table, apply_graph, apply_object, apply_kit, SemioMutation};
use crate::standards::v1::subsets::base::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap<M>(leaves: Vec<M>, wrap: impl Fn(M) -> SemioMutation) -> Vec<SemioMutation> {
    leaves.into_iter().map(wrap).collect()
}

/// 🧮️ A same-kind edit nets through the wrapped subset's own leaves; a subset-kind change has no mutation (it is a document load)
/// and nets to nothing, which the editor's publication check refuses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioSnapshot, next: &SemioSnapshot) -> Vec<SemioMutation> {
    use SemioSubsetSnapshot as S;
    match (&base.subset, &next.subset) {
        (S::Brep(before), S::Brep(after)) => wrap(crate::editor::semio_brep::net::net(before, after), |mutation| SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation })),
        (S::Mesh(before), S::Mesh(after)) => wrap(crate::editor::semio_mesh::net::net(before, after), |mutation| SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation })),
        (S::Model(before), S::Model(after)) => wrap(crate::editor::semio_model::net::net(before, after), |mutation| SemioMutation::ApplyModel(apply_model::ApplyModel { mutation })),
        (S::Value(before), S::Value(after)) => wrap(crate::editor::semio_value::net::net(before, after), |mutation| SemioMutation::ApplyValue(apply_value::ApplyValue { mutation })),
        (S::Document(before), S::Document(after)) => wrap(crate::editor::semio_document::net::net(before, after), |mutation| SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation })),
        (S::Cad(before), S::Cad(after)) => wrap(crate::editor::semio_cad::net::net(before, after), |mutation| SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation })),
        (S::Drawing(before), S::Drawing(after)) => wrap(crate::editor::semio_drawing::net::net(before, after), |mutation| SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation })),
        (S::Image(before), S::Image(after)) => wrap(crate::editor::semio_image::net::net(before, after), |mutation| SemioMutation::ApplyImage(apply_image::ApplyImage { mutation })),
        (S::Video(before), S::Video(after)) => wrap(crate::editor::semio_video::net::net(before, after), |mutation| SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation })),
        (S::Audio(before), S::Audio(after)) => wrap(crate::editor::semio_audio::net::net(before, after), |mutation| SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation })),
        (S::Animation(before), S::Animation(after)) => wrap(crate::editor::semio_animation::net::net(before, after), |mutation| SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation })),
        (S::Presentation(before), S::Presentation(after)) => wrap(crate::editor::semio_presentation::net::net(before, after), |mutation| SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation })),
        (S::Flow(before), S::Flow(after)) => wrap(crate::editor::semio_flow::net::net(before, after), |mutation| SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation })),
        (S::Text(before), S::Text(after)) => wrap(crate::editor::semio_text::net::net(before, after), |mutation| SemioMutation::ApplyText(apply_text::ApplyText { mutation })),
        (S::Table(before), S::Table(after)) => wrap(crate::editor::semio_table::net::net(before, after), |mutation| SemioMutation::ApplyTable(apply_table::ApplyTable { mutation })),
        (S::Graph(before), S::Graph(after)) => wrap(crate::editor::semio_graph::net::net(before, after), |mutation| SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation })),
        (S::Object(before), S::Object(after)) => wrap(crate::editor::semio_object::net::net(before, after), |mutation| SemioMutation::ApplyObject(apply_object::ApplyObject { mutation })),
        (S::Kit(before), S::Kit(after)) => wrap(crate::editor::semio_kit::net::net(before, after), |mutation| SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation })),
        _ => Vec::new(),
    }
}
