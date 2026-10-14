//! 🧭️ The envelope editor's edit rules: an edit below `/subset` is the wrapped subset's own edit, resolved by that subset's rules and wrapped in its `apply-*` kind. The subset tag and the schema stamp are not edited; replacing the document changes them.

use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::edit_fault;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::event_path;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::unsupported;
use {semio_s_artifact_stdio_semio_animation::editor::SemioAnimationEditor, semio_s_artifact_stdio_semio_audio::editor::SemioAudioEditor, semio_s_artifact_stdio_semio_brep::editor::SemioBrepEditor, semio_s_artifact_stdio_semio_cad::editor::SemioCadEditor, semio_s_artifact_stdio_semio_document::editor::SemioDocumentEditor, semio_s_artifact_stdio_semio_drawing::editor::SemioDrawingEditor, semio_s_artifact_stdio_semio_flow::editor::SemioFlowEditor, semio_s_artifact_stdio_semio_graph::editor::SemioGraphEditor, semio_s_artifact_stdio_semio_image::editor::SemioImageEditor, semio_s_artifact_stdio_semio_kit::editor::SemioKitEditor, semio_s_artifact_stdio_semio_mesh::editor::SemioMeshEditor, semio_s_artifact_stdio_semio_model::editor::SemioModelEditor, semio_s_artifact_stdio_semio_object::editor::SemioObjectEditor, semio_s_artifact_stdio_semio_presentation::editor::SemioPresentationEditor, semio_s_artifact_stdio_semio_table::editor::SemioTableEditor, semio_s_artifact_stdio_semio_text::editor::SemioTextEditor, semio_s_artifact_stdio_semio_value::editor::SemioValueEditor, semio_s_artifact_stdio_semio_video::editor::SemioVideoEditor};
use crate::schema::mutations::{apply_animation, apply_audio, apply_brep, apply_cad, apply_document, apply_drawing, apply_flow, apply_graph, apply_image, apply_kit, apply_mesh, apply_model, apply_object, apply_presentation, apply_table, apply_text, apply_value, apply_video, SemioMutation};
use crate::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use semio_framework_plugin::Fault;
use semio_s_artifact_stdio_contract::editing::{EditRules, SnapshotEditEvent, SnapshotEditingEditor};

/// 📚 The envelope names no pointer of its own.
pub const EDIT_RULES: EditRules = EditRules { entities: &[], inserts: &[], removes: &[] };

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn below(pointer: &str) -> Result<String, Fault> {
    pointer.strip_prefix("/subset").filter(|rest| rest.starts_with('/')).map(str::to_string).ok_or_else(|| unsupported(pointer, "only the members of the wrapped subset are edited; replace the document to change its kind"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inner(event: &SnapshotEditEvent) -> Result<SnapshotEditEvent, Fault> {
    Ok(match event {
        SnapshotEditEvent::SetValue { path, value } => SnapshotEditEvent::SetValue { path: below(path)?, value: value.clone() },
        SnapshotEditEvent::InsertValue { path, value } => SnapshotEditEvent::InsertValue { path: below(path)?, value: value.clone() },
        SnapshotEditEvent::RemoveValue { path } => SnapshotEditEvent::RemoveValue { path: below(path)? },
        SnapshotEditEvent::MoveValue { from, path } => SnapshotEditEvent::MoveValue { from: below(from)?, path: below(path)? },
        SnapshotEditEvent::RenameKey { path, key } => SnapshotEditEvent::RenameKey { path: below(path)?, key: key.clone() },
        SnapshotEditEvent::ReplaceSource { .. } => return Err(unsupported(event_path(event), "replacing the whole source is a document load, not an edit")),
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn lift<E: SnapshotEditingEditor>(subset: &E::Snapshot, event: &SnapshotEditEvent, wrap: fn(E::Mutation) -> SemioMutation) -> Result<Option<Vec<SemioMutation>>, Fault> {
    let mutations = match E::snapshot_edit_special(event, subset)? {
        Some(mutations) => mutations,
        None => E::snapshot_edit_rules().resolve::<E::Snapshot, E::Mutation>(subset, event).map_err(edit_fault)?,
    };
    Ok(Some(mutations.into_iter().map(wrap).collect()))
}

/// 🎁 The wrapped subset's concrete kind for the edit, wrapped in its `apply-*` kind.
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioSnapshot) -> Result<Option<Vec<SemioMutation>>, Fault> {
    use SemioSubsetSnapshot as S;
    let event = &inner(event)?;
    match &snapshot.subset {
        S::Brep(subset) => lift::<SemioBrepEditor>(subset, event, |mutation| SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation })),
        S::Mesh(subset) => lift::<SemioMeshEditor>(subset, event, |mutation| SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation })),
        S::Model(subset) => lift::<SemioModelEditor>(subset, event, |mutation| SemioMutation::ApplyModel(apply_model::ApplyModel { mutation })),
        S::Value(subset) => lift::<SemioValueEditor>(subset, event, |mutation| SemioMutation::ApplyValue(apply_value::ApplyValue { mutation })),
        S::Document(subset) => lift::<SemioDocumentEditor>(subset, event, |mutation| SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation })),
        S::Cad(subset) => lift::<SemioCadEditor>(subset, event, |mutation| SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation })),
        S::Drawing(subset) => lift::<SemioDrawingEditor>(subset, event, |mutation| SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation })),
        S::Image(subset) => lift::<SemioImageEditor>(subset, event, |mutation| SemioMutation::ApplyImage(apply_image::ApplyImage { mutation })),
        S::Video(subset) => lift::<SemioVideoEditor>(subset, event, |mutation| SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation })),
        S::Audio(subset) => lift::<SemioAudioEditor>(subset, event, |mutation| SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation })),
        S::Animation(subset) => lift::<SemioAnimationEditor>(subset, event, |mutation| SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation })),
        S::Presentation(subset) => lift::<SemioPresentationEditor>(subset, event, |mutation| SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation })),
        S::Flow(subset) => lift::<SemioFlowEditor>(subset, event, |mutation| SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation })),
        S::Text(subset) => lift::<SemioTextEditor>(subset, event, |mutation| SemioMutation::ApplyText(apply_text::ApplyText { mutation })),
        S::Table(subset) => lift::<SemioTableEditor>(subset, event, |mutation| SemioMutation::ApplyTable(apply_table::ApplyTable { mutation })),
        S::Graph(subset) => lift::<SemioGraphEditor>(subset, event, |mutation| SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation })),
        S::Object(subset) => lift::<SemioObjectEditor>(subset, event, |mutation| SemioMutation::ApplyObject(apply_object::ApplyObject { mutation })),
        S::Kit(subset) => lift::<SemioKitEditor>(subset, event, |mutation| SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation })),
    }
}
