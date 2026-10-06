//! 💾️ Binary representation codec surface for `stdio.semio.brep` (mutations).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::brep::schema::mutations::*;
use crate::standards::v1::subsets::base::schema::geometry::native::NativeF64;
use crate::standards::v1::subsets::brep::schema::diff::{SemioBrepDiff};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_solid_shell};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_solid_shell};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_shell_face};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_shell_face};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_surface};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_surface};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_curve};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_curve};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_bool};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{parse_f64};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_point3};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_point3};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary` block below calls
/// `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in scope in
/// production code too, not merely under `#[cfg(test)]` (same fix this facet's OLD file already
/// needed, and flow's own mutations facet needs for the same reason).
use protocol::{OpBinary, OpText};
use crate::standards::v1::subsets::brep::schema::mutations::create_edge;
use crate::standards::v1::subsets::brep::schema::mutations::create_face;
use crate::standards::v1::subsets::brep::schema::mutations::create_shell;
use crate::standards::v1::subsets::brep::schema::mutations::create_solid;
use crate::standards::v1::subsets::brep::schema::mutations::create_vertex;
use crate::standards::v1::subsets::brep::schema::mutations::delete_edge;
use crate::standards::v1::subsets::brep::schema::mutations::delete_face;
use crate::standards::v1::subsets::brep::schema::mutations::delete_shell;
use crate::standards::v1::subsets::brep::schema::mutations::delete_solid;
use crate::standards::v1::subsets::brep::schema::mutations::delete_vertex;
use crate::standards::v1::subsets::brep::schema::mutations::move_vertex;
use crate::standards::v1::subsets::brep::schema::mutations::replace_curve;
use crate::standards::v1::subsets::brep::schema::mutations::replace_surface;
/// 🧬️ Every variant wraps exactly one `protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation>`
/// payload struct declared in the corresponding triad leaf's `🦠️mutation/🦀️.rs`. Thirteen
/// triads: vertex lifecycle (`create-vertex`/`delete-vertex`), edge lifecycle
/// (`create-edge`/`delete-edge`), face lifecycle (`create-face`/`delete-face`), shell lifecycle
/// (`create-shell`/`delete-shell`), solid lifecycle (`create-solid`/`delete-solid`), then the two
/// structured-payload replacements (`replace-curve`/`replace-surface`) and the one scalar
/// reposition (`move-vertex`).
use crate::standards::v1::subsets::brep::schema::mutations::set_snapshot::SetSnapshot;
use crate::standards::v1::subsets::brep::io::text::mutations::{print_brep_mutation};
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn wire_tag(m: &SemioBrepMutation) -> u8 {
    match m {
        SemioBrepMutation::SetSnapshot(_) => TAG_SET_SNAPSHOT,
        SemioBrepMutation::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
        SemioBrepMutation::CreateVertex(_) => TAG_CREATE_VERTEX,
        SemioBrepMutation::DeleteVertex(_) => TAG_DELETE_VERTEX,
        SemioBrepMutation::CreateEdge(_) => TAG_CREATE_EDGE,
        SemioBrepMutation::DeleteEdge(_) => TAG_DELETE_EDGE,
        SemioBrepMutation::CreateFace(_) => TAG_CREATE_FACE,
        SemioBrepMutation::DeleteFace(_) => TAG_DELETE_FACE,
        SemioBrepMutation::CreateShell(_) => TAG_CREATE_SHELL,
        SemioBrepMutation::DeleteShell(_) => TAG_DELETE_SHELL,
        SemioBrepMutation::CreateSolid(_) => TAG_CREATE_SOLID,
        SemioBrepMutation::DeleteSolid(_) => TAG_DELETE_SOLID,
        SemioBrepMutation::ReplaceCurve(_) => TAG_REPLACE_CURVE,
        SemioBrepMutation::ReplaceSurface(_) => TAG_REPLACE_SURFACE,
        SemioBrepMutation::MoveVertex(_) => TAG_MOVE_VERTEX,
    }
}

/// ✂️ Just the `key=value ...` argument tail of `print_brep_mutation` — the binary frame's `tag`
/// byte already carries the keyword, so the text keyword itself is redundant in the binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_brep_mutation_args(m: &SemioBrepMutation) -> String {
    match print_brep_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame: `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (its kind's record tag in
/// `💾️binary/📡️.protocol.semio`) as two REAL fixed fields, then the variant's own `key=value ...`
/// argument payload as one opaque trailing `bytes` chain — reusing the already-real, already-tested
/// `print_brep_mutation`/`parse_brep_mutation` text codec rather than re-deriving a second
/// independent encoding.
impl OpBinary for SemioBrepMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        if let Self::PatchSnapshot(payload) = self {
            let mut out = vec![1, TAG_PATCH_SNAPSHOT];
            out.extend(protocol::OpBinary::encode_op(&payload.patch)?);
            return Ok(out);
        }
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
        out.extend_from_slice(print_brep_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        if bytes[1] == TAG_PATCH_SNAPSHOT {
            return Ok(Self::PatchSnapshot(crate::standards::v1::subsets::brep::schema::mutations::patch_snapshot::PatchSnapshot { patch: protocol::OpBinary::decode_op(&bytes[2..])? }));
        }
        let tag = bytes[1];
        let keyword = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(tag)).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} names no record of 📡️.protocol.semio") })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioBrepMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_CREATE_VERTEX: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-vertex");
const TAG_DELETE_VERTEX: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-vertex");
const TAG_CREATE_EDGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-edge");
const TAG_DELETE_EDGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-edge");
const TAG_CREATE_FACE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-face");
const TAG_DELETE_FACE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-face");
const TAG_CREATE_SHELL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-shell");
const TAG_DELETE_SHELL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-shell");
const TAG_CREATE_SOLID: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-solid");
const TAG_DELETE_SOLID: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-solid");
const TAG_REPLACE_CURVE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "replace-curve");
const TAG_REPLACE_SURFACE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "replace-surface");
const TAG_MOVE_VERTEX: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "move-vertex");
//#endregion 🏷️WireTags
