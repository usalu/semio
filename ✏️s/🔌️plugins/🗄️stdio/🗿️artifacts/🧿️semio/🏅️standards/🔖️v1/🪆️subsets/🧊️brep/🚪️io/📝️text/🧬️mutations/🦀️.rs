//! 📝️ Text representation codec surface for `stdio.semio.brep` (mutations).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

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
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_bool};
use crate::standards::v1::subsets::brep::io::text::snapshot::{parse_f64};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_point3};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_point3};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_list};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_list};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_str};
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

/// 📥️ Decodes this facet's own externally-tagged (`{"<VariantName>": {<snake_case payload>}}`)
/// JSON projection — no `#[value(rename_all)]` sits on this enum or its payload structs, which is
/// exactly the shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` vectors
/// carry — into a real [`SemioBrepMutation`]. `create-*`/`delete-*` payloads address topology by the string ids (`v1`, `e2`, `so1`) the
/// committed vectors use, so a decoded mutation is directly comparable with the vector it came from.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_brep_mutation_json(text: &str) -> Result<SemioBrepMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// ⚡️ Real hand-rolled `OpText`, grammar `keyword id=hex ...` — reusing the sibling `🔺️diff`
/// facet's now-`pub(crate)` hex/value primitives (one source of truth for entity encoding, same
/// convention this file's pre-rewrite version and `🌊️flow`'s mutations facet both established).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|byte| format!("{byte:02x}")).collect() }

pub(crate) fn hex_decode(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) { return Err("snapshot payload has odd hexadecimal length".into()); }
    (0..value.len()).step_by(2).map(|index| u8::from_str_radix(&value[index..index + 2], 16).map_err(|error| error.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_at(at: Option<usize>) -> String {
    at.map(|at| format!(" at={at}")).unwrap_or_default()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_at(args: &std::collections::BTreeMap<&str, &str>) -> Result<Option<usize>, String> {
    args.get("at").map(|at| at.parse::<usize>().map_err(|error| error.to_string())).transpose()
}

pub(crate) fn print_brep_mutation(m: &SemioBrepMutation) -> String {
    match m {
        SemioBrepMutation::CreateVertex(p) => format!("create-vertex id={} point={} tol={}{}", enc_str(&p.id), enc_point3(&p.point), NativeF64(p.tol), enc_at(p.at)),
        SemioBrepMutation::DeleteVertex(p) => format!("delete-vertex id={}", enc_str(&p.id)),
        SemioBrepMutation::CreateEdge(p) => format!("create-edge id={} start={} end={} curve={} tol={}{}", enc_str(&p.id), enc_str(&p.start_vertex), enc_str(&p.end_vertex), enc_curve(&p.curve), NativeF64(p.tol), enc_at(p.at)),
        SemioBrepMutation::DeleteEdge(p) => format!("delete-edge id={}", enc_str(&p.id)),
        SemioBrepMutation::CreateFace(p) => {
            format!("create-face id={} outer={} inner={} surface={} orientation={} tol={}{}", enc_str(&p.id), enc_str(&p.outer_loop), enc_list(&p.inner_loops, |s: &String| crate::standards::v1::subsets::brep::io::text::snapshot::enc_loop_id(s)), enc_surface(&p.surface), enc_bool(p.orientation), NativeF64(p.tol), enc_at(p.at))
        }
        SemioBrepMutation::DeleteFace(p) => format!("delete-face id={}", enc_str(&p.id)),
        SemioBrepMutation::CreateShell(p) => format!("create-shell id={} faces={}{}", enc_str(&p.id), enc_list(&p.faces, enc_shell_face), enc_at(p.at)),
        SemioBrepMutation::DeleteShell(p) => format!("delete-shell id={}", enc_str(&p.id)),
        SemioBrepMutation::CreateSolid(p) => format!("create-solid id={} shells={}{}", enc_str(&p.id), enc_list(&p.shells, enc_solid_shell), enc_at(p.at)),
        SemioBrepMutation::DeleteSolid(p) => format!("delete-solid id={}", enc_str(&p.id)),
        SemioBrepMutation::ReplaceCurve(p) => format!("replace-curve edge={} curve={}", enc_str(&p.edge_id), enc_curve(&p.new_curve)),
        SemioBrepMutation::ReplaceSurface(p) => format!("replace-surface face={} surface={}", enc_str(&p.face_id), enc_surface(&p.new_surface)),
        SemioBrepMutation::MoveVertex(p) => format!("move-vertex id={} point={}", enc_str(&p.vertex_id), enc_point3(&p.new_point)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_brep_mutation(line: &str) -> Result<SemioBrepMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("brep mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("brep mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "create-vertex" => Ok(SemioBrepMutation::CreateVertex(create_vertex::CreateVertex { id: dec_str(arg("id")?)?, point: dec_point3(arg("point")?)?, tol: parse_f64(arg("tol")?)?, at: dec_at(&args)? })),
        "delete-vertex" => Ok(SemioBrepMutation::DeleteVertex(delete_vertex::DeleteVertex { id: dec_str(arg("id")?)? })),
        "create-edge" => {
            Ok(SemioBrepMutation::CreateEdge(create_edge::CreateEdge { id: dec_str(arg("id")?)?, start_vertex: dec_str(arg("start")?)?, end_vertex: dec_str(arg("end")?)?, curve: dec_curve(arg("curve")?)?, tol: parse_f64(arg("tol")?)?, at: dec_at(&args)? }))
        }
        "delete-edge" => Ok(SemioBrepMutation::DeleteEdge(delete_edge::DeleteEdge { id: dec_str(arg("id")?)? })),
        "create-face" => Ok(SemioBrepMutation::CreateFace(create_face::CreateFace {
            id: dec_str(arg("id")?)?,
            outer_loop: dec_str(arg("outer")?)?,
            inner_loops: dec_list(arg("inner")?, crate::standards::v1::subsets::brep::io::text::snapshot::dec_loop_id)?,
            surface: dec_surface(arg("surface")?)?,
            orientation: crate::standards::v1::subsets::brep::io::text::diff::parse_bool(arg("orientation")?)?,
            tol: parse_f64(arg("tol")?)?,
            at: dec_at(&args)?,
        })),
        "delete-face" => Ok(SemioBrepMutation::DeleteFace(delete_face::DeleteFace { id: dec_str(arg("id")?)? })),
        "create-shell" => Ok(SemioBrepMutation::CreateShell(create_shell::CreateShell { id: dec_str(arg("id")?)?, faces: dec_list(arg("faces")?, dec_shell_face)?, at: dec_at(&args)? })),
        "delete-shell" => Ok(SemioBrepMutation::DeleteShell(delete_shell::DeleteShell { id: dec_str(arg("id")?)? })),
        "create-solid" => Ok(SemioBrepMutation::CreateSolid(create_solid::CreateSolid { id: dec_str(arg("id")?)?, shells: dec_list(arg("shells")?, dec_solid_shell)?, at: dec_at(&args)? })),
        "delete-solid" => Ok(SemioBrepMutation::DeleteSolid(delete_solid::DeleteSolid { id: dec_str(arg("id")?)? })),
        "replace-curve" => Ok(SemioBrepMutation::ReplaceCurve(replace_curve::ReplaceCurve { edge_id: dec_str(arg("edge")?)?, new_curve: dec_curve(arg("curve")?)? })),
        "replace-surface" => Ok(SemioBrepMutation::ReplaceSurface(replace_surface::ReplaceSurface { face_id: dec_str(arg("face")?)?, new_surface: dec_surface(arg("surface")?)? })),
        "move-vertex" => Ok(SemioBrepMutation::MoveVertex(move_vertex::MoveVertex { vertex_id: dec_str(arg("id")?)?, new_point: dec_point3(arg("point")?)? })),
        other => Err(format!("brep mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioBrepMutation {
    fn print_op(&self) -> String {
        print_brep_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_brep_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
