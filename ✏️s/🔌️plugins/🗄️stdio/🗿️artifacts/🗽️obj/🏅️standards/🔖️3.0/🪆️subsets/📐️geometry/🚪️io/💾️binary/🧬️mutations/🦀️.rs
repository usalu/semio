//! binary rep for stdio.obj 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v3_0::subsets::any::schema::mutations::*;
use crate::schema::diff::{
    diff_insert_face, diff_insert_normal, diff_insert_texcoord, diff_insert_vertex, diff_remove_face, diff_remove_group, diff_remove_normal, diff_remove_object, diff_remove_texcoord, diff_remove_vertex, diff_set_face, diff_set_group,
    diff_set_mtllib, diff_set_normal, diff_set_object, diff_set_smoothing_groups, diff_set_texcoord, diff_set_unknown_statements, diff_set_usemtl, diff_set_vertex, face_field_changes, normal_field_changes, texcoord_field_changes,
    vertex_field_changes, ObjDiff,
};
use crate::schema::snapshot::{ObjFace, ObjNormal, ObjSmoothingRange, ObjTexCoord, ObjUnknownStatement, ObjUsemtlRange, ObjVertex};
#[cfg(test)]
use crate::schema::snapshot::{ObjFaceVertex, ObjGroup, ObjObject};
use crate::ObjSnapshot;
use protocol::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`.
impl OpBinary for ObjMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(crate::standards::v3_0::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO, self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(crate::standards::v3_0::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO, bytes)
    }
}
}
pub use mutations_codec::*;
