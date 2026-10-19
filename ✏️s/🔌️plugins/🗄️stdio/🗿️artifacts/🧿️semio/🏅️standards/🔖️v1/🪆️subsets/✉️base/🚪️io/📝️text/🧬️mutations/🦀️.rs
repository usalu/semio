//! 📝️ Text representation codec surface for `stdio.semio` (mutations) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::schema::mutations::*;
use {semio_s_artifact_stdio_semio_animation::schema::mutations::SemioAnimationMutation, semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot};
#[cfg(test)]
use semio_s_artifact_stdio_semio_audio_mut_family::schema::mutations::set_sample_rate;
use {semio_s_artifact_stdio_semio_audio_mutations::schema::mutations::SemioAudioMutation, semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot};
use crate::schema::diff::SemioDiff;
use crate::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use {semio_s_artifact_stdio_semio_brep::schema::mutations::SemioBrepMutation, semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot};
use {semio_s_artifact_stdio_semio_cad::schema::mutations::SemioCadMutation, semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot};
use {semio_s_artifact_stdio_semio_document::schema::mutations::SemioDocumentMutation, semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot};
use {semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation, semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot};
use {semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation, semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot};
use {semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation, semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot};
use {semio_s_artifact_stdio_semio_image::schema::mutations::SemioImageMutation, semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot};
use {semio_s_artifact_stdio_semio_kit::schema::mutations::SemioKitMutation, semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot};
use {semio_s_artifact_stdio_semio_mesh::schema::mutations::SemioMeshMutation, semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot};
use {semio_s_artifact_stdio_semio_model::schema::mutations::SemioModelMutation, semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot};
use {semio_s_artifact_stdio_semio_object::schema::mutations::SemioObjectMutation, semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot};
use {semio_s_artifact_stdio_semio_presentation::schema::mutations::SemioPresentationMutation, semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot};
use {semio_s_artifact_stdio_semio_table_mutations::schema::mutations::SemioTableMutation, semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot};
use {semio_s_artifact_stdio_semio_text_mutations::schema::mutations::SemioTextMutation, semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot};
use {semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation, semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot};
use {semio_s_artifact_stdio_semio_video_mutations::schema::mutations::SemioVideoMutation, semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot};
use protocol::Mutation;
use protocol::OpBinary;
use protocol::OpText;

/// 📤️ The envelope mutation's JSON carrier, derived from `SemioMutation`'s own `ToValue` (adjacently
/// tagged `{"mutation": "<camelCaseVariant>", "payload": …}`, each payload the wrapped arm's own
/// mutation) and printed by the first-party `pack` JSON codec — the shape `🔣️.json` beside this file
/// publishes. See <🔣️.json>.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_mutation_json(mutation: &SemioMutation) -> String {
    semio_framework_pack_json::to_json_string(mutation)
}

/// 📥️ The inverse of [`encode_semio_mutation_json`], through `SemioMutation`'s own `FromValue`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_mutation_json(text: &str) -> Result<SemioMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 🎙️ Real delegating text/binary op codec — replaces the old whole-enum `serde_json` passthrough.
/// Text is one `tag:payload` line: `payload` for the 18 wrapped variants is exactly that subset's
/// OWN already-real `OpText::print_op()`/`parse_op()` output (genuine reuse, never re-derived
/// here); `setSnapshot`'s payload is hex(`SemioSnapshot::print_dsl`) — real delegation to this
/// envelope's own now-real `ArtifactDsl` (📸️snapshot/🦀️.rs), hex-flattened to keep
/// `print_op`'s one-physical-line contract.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn subset_mutation_tag(m: &SemioMutation) -> &'static str {
    match m {
        SemioMutation::ApplyBrep(_) => "brep",
        SemioMutation::ApplyMesh(_) => "mesh",
        SemioMutation::ApplyModel(_) => "model",
        SemioMutation::ApplyValue(_) => "value",
        SemioMutation::ApplyDocument(_) => "document",
        SemioMutation::ApplyCad(_) => "cad",
        SemioMutation::ApplyDrawing(_) => "drawing",
        SemioMutation::ApplyImage(_) => "image",
        SemioMutation::ApplyVideo(_) => "video",
        SemioMutation::ApplyAudio(_) => "audio",
        SemioMutation::ApplyAnimation(_) => "animation",
        SemioMutation::ApplyPresentation(_) => "presentation",
        SemioMutation::ApplyFlow(_) => "flow",
        SemioMutation::ApplyText(_) => "text",
        SemioMutation::ApplyTable(_) => "table",
        SemioMutation::ApplyGraph(_) => "graph",
        SemioMutation::ApplyObject(_) => "object",
        SemioMutation::ApplyKit(_) => "kit",
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_semio_mutation(m: &SemioMutation) -> String {
    let tag = subset_mutation_tag(m);
    match m {
        SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyModel(apply_model::ApplyModel { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyValue(apply_value::ApplyValue { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyImage(apply_image::ApplyImage { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyText(apply_text::ApplyText { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyTable(apply_table::ApplyTable { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyObject(apply_object::ApplyObject { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation }) => format!("{tag}:{}", mutation.print_op()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_semio_mutation(line: &str) -> Result<SemioMutation, String> {
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("semio mutation: missing ':' in {line:?}"))?;
    match tag {
        "brep" => Ok(SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation: SemioBrepMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "mesh" => Ok(SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation: SemioMeshMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "model" => Ok(SemioMutation::ApplyModel(apply_model::ApplyModel { mutation: SemioModelMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "value" => Ok(SemioMutation::ApplyValue(apply_value::ApplyValue { mutation: SemioValueMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "document" => Ok(SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation: SemioDocumentMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "cad" => Ok(SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation: SemioCadMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "drawing" => Ok(SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation: SemioDrawingMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "image" => Ok(SemioMutation::ApplyImage(apply_image::ApplyImage { mutation: SemioImageMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "video" => Ok(SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation: SemioVideoMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "audio" => Ok(SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation: SemioAudioMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "animation" => Ok(SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation: SemioAnimationMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "presentation" => Ok(SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation: SemioPresentationMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "flow" => Ok(SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation: SemioFlowMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "text" => Ok(SemioMutation::ApplyText(apply_text::ApplyText { mutation: SemioTextMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "table" => Ok(SemioMutation::ApplyTable(apply_table::ApplyTable { mutation: SemioTableMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "graph" => Ok(SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation: SemioGraphMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "object" => Ok(SemioMutation::ApplyObject(apply_object::ApplyObject { mutation: SemioObjectMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "kit" => Ok(SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation: SemioKitMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        other => Err(format!("semio mutation: unknown tag {other:?}")),
    }
}

impl OpText for SemioMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_semio_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        print_semio_mutation(self)
    }
}
}
pub use mutations_codec::*;
