//! 💾️ Binary representation codec surface for `stdio.semio` (mutations) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::schema::mutations::*;
use {semio_s_artifact_stdio_semio_animation::schema::mutations::SemioAnimationMutation, semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot};
#[cfg(test)]
use semio_s_artifact_stdio_semio_audio_mutations::schema::mutations::set_sample_rate;
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

/// 🏷️ Binary tag ordinal for [`SemioMutation`] — `0` = `SetSnapshot`, `1..=18` = the 18 wrapped
/// subset kinds (enum declaration order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn mutation_tag(m: &SemioMutation) -> u8 {
    match m {
        SemioMutation::ApplyBrep(_) => TAG_APPLY_BREP,
        SemioMutation::ApplyMesh(_) => TAG_APPLY_MESH,
        SemioMutation::ApplyModel(_) => TAG_APPLY_MODEL,
        SemioMutation::ApplyValue(_) => TAG_APPLY_VALUE,
        SemioMutation::ApplyDocument(_) => TAG_APPLY_DOCUMENT,
        SemioMutation::ApplyCad(_) => TAG_APPLY_CAD,
        SemioMutation::ApplyDrawing(_) => TAG_APPLY_DRAWING,
        SemioMutation::ApplyImage(_) => TAG_APPLY_IMAGE,
        SemioMutation::ApplyVideo(_) => TAG_APPLY_VIDEO,
        SemioMutation::ApplyAudio(_) => TAG_APPLY_AUDIO,
        SemioMutation::ApplyAnimation(_) => TAG_APPLY_ANIMATION,
        SemioMutation::ApplyPresentation(_) => TAG_APPLY_PRESENTATION,
        SemioMutation::ApplyFlow(_) => TAG_APPLY_FLOW,
        SemioMutation::ApplyText(_) => TAG_APPLY_TEXT,
        SemioMutation::ApplyTable(_) => TAG_APPLY_TABLE,
        SemioMutation::ApplyGraph(_) => TAG_APPLY_GRAPH,
        SemioMutation::ApplyObject(_) => TAG_APPLY_OBJECT,
        SemioMutation::ApplyKit(_) => TAG_APPLY_KIT,
    }
}

impl OpBinary for SemioMutation {
    /// ⚡️ Real delegating binary: `format u8` + `tag u8` ([`mutation_tag`]) as two genuine,
    /// individually protocol-walkable fixed header fields, then ONE opaque trailing payload — for
    /// the 18 wrapped variants, the wrapped subset's OWN real `OpBinary::encode_op()` bytes
    /// (genuine reuse); for `SetSnapshot`, the wrapped snapshot's own real
    /// `ArtifactPack::encode_pack()` bytes.
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, mutation_tag(self)];
        let payload: Vec<u8> = match self {
            SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyModel(apply_model::ApplyModel { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyValue(apply_value::ApplyValue { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyImage(apply_image::ApplyImage { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyText(apply_text::ApplyText { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyTable(apply_table::ApplyTable { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyObject(apply_object::ApplyObject { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation }) => mutation.encode_op()?,
        };
        out.extend_from_slice(&payload);
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated".to_string() });
        }
        let format = bytes[0];
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported format {format}") });
        }
        let tag = bytes[1];
        let payload = &bytes[2..];
        Ok(match tag {
            TAG_APPLY_BREP => SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation: SemioBrepMutation::decode_op(payload)? }),
            TAG_APPLY_MESH => SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation: SemioMeshMutation::decode_op(payload)? }),
            TAG_APPLY_MODEL => SemioMutation::ApplyModel(apply_model::ApplyModel { mutation: SemioModelMutation::decode_op(payload)? }),
            TAG_APPLY_VALUE => SemioMutation::ApplyValue(apply_value::ApplyValue { mutation: SemioValueMutation::decode_op(payload)? }),
            TAG_APPLY_DOCUMENT => SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation: SemioDocumentMutation::decode_op(payload)? }),
            TAG_APPLY_CAD => SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation: SemioCadMutation::decode_op(payload)? }),
            TAG_APPLY_DRAWING => SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation: SemioDrawingMutation::decode_op(payload)? }),
            TAG_APPLY_IMAGE => SemioMutation::ApplyImage(apply_image::ApplyImage { mutation: SemioImageMutation::decode_op(payload)? }),
            TAG_APPLY_VIDEO => SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation: SemioVideoMutation::decode_op(payload)? }),
            TAG_APPLY_AUDIO => SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation: SemioAudioMutation::decode_op(payload)? }),
            TAG_APPLY_ANIMATION => SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation: SemioAnimationMutation::decode_op(payload)? }),
            TAG_APPLY_PRESENTATION => SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation: SemioPresentationMutation::decode_op(payload)? }),
            TAG_APPLY_FLOW => SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation: SemioFlowMutation::decode_op(payload)? }),
            TAG_APPLY_TEXT => SemioMutation::ApplyText(apply_text::ApplyText { mutation: SemioTextMutation::decode_op(payload)? }),
            TAG_APPLY_TABLE => SemioMutation::ApplyTable(apply_table::ApplyTable { mutation: SemioTableMutation::decode_op(payload)? }),
            TAG_APPLY_GRAPH => SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation: SemioGraphMutation::decode_op(payload)? }),
            TAG_APPLY_OBJECT => SemioMutation::ApplyObject(apply_object::ApplyObject { mutation: SemioObjectMutation::decode_op(payload)? }),
            TAG_APPLY_KIT => SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation: SemioKitMutation::decode_op(payload)? }),
            other => return Err(protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("unknown tag {other}") }),
        })
    }
}
}

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_APPLY_BREP: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-brep");
const TAG_APPLY_MESH: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-mesh");
const TAG_APPLY_MODEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-model");
const TAG_APPLY_VALUE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-value");
const TAG_APPLY_DOCUMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-document");
const TAG_APPLY_CAD: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-cad");
const TAG_APPLY_DRAWING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-drawing");
const TAG_APPLY_IMAGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-image");
const TAG_APPLY_VIDEO: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-video");
const TAG_APPLY_AUDIO: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-audio");
const TAG_APPLY_ANIMATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-animation");
const TAG_APPLY_PRESENTATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-presentation");
const TAG_APPLY_FLOW: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-flow");
const TAG_APPLY_TEXT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-text");
const TAG_APPLY_TABLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-table");
const TAG_APPLY_GRAPH: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-graph");
const TAG_APPLY_OBJECT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-object");
const TAG_APPLY_KIT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "apply-kit");
//#endregion 🏷️WireTags
