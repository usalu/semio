//! 📝️ Text representation codec surface for `stdio.stl` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_ascii::subsets::any::schema::mutations::*;
use crate::schema::diff::{self, StlDiff};
use crate::schema::snapshot::StlTriangle;
use crate::StlSnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};

/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` grammar — see this file's top doc comment for why (the
/// same real, reproduced `dsl`-derive bug that forced `StlDiff`'s hand-roll also reaches here via
/// `InsertTriangle`/`SetTriangleVertices`'s `[[f64; 3]; 3]` payload).
///
/// **Grammar**: `<keyword> arg=value ...` — one space-separated `key=value` token per argument
/// (every variant's args are ALWAYS present, unlike `StlDiff`'s sparse tokens). `index`/floats
/// print via `Display`; `name`/`solid_name` are lowercase hex; `normal`/`vertices`/`triangle`
/// reuse `🔺️diff::component`'s `pub(crate)` value codecs verbatim (`enc_vec3`, `enc_vertices`,
/// `enc_triangle`).
pub(crate) fn print_stl_op(m: &StlMutation) -> String {
    match m {
        StlMutation::SetSolidName(set_solid_name::SetSolidName { name }) => format!("set-solid-name name={}", crate::standards::v_ascii::subsets::any::io::text::diff::hex_encode_str(name)),
        StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index, triangle }) => format!("insert-triangle index={index} triangle={}", crate::standards::v_ascii::subsets::any::io::text::diff::enc_triangle(triangle)),
        StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index }) => format!("remove-triangle index={index}"),
        StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index, normal }) => format!("set-triangle-normal index={index} normal={}", crate::standards::v_ascii::subsets::any::io::text::diff::enc_vec3(normal)),
        StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index, vertices }) => format!("set-triangle-vertices index={index} vertices={}", crate::standards::v_ascii::subsets::any::io::text::diff::enc_vertices(vertices)),
    }
}

pub(crate) fn parse_stl_op(line: &str) -> Result<StlMutation, String> {
    let mut tokens = line.split(' ');
    let keyword = tokens.next().ok_or_else(|| "stl op: empty line".to_string())?;
    let args: Vec<&str> = tokens.collect();
    let get = |key: &str| -> Result<&str, String> {
        let probe = format!("{key}=");
        args.iter().find_map(|t| t.strip_prefix(probe.as_str())).ok_or_else(|| format!("stl op: missing '{key}=' in {line:?}"))
    };
    match keyword {
        "set-solid-name" => Ok(StlMutation::SetSolidName(set_solid_name::SetSolidName { name: crate::standards::v_ascii::subsets::any::io::text::diff::hex_decode_str(get("name")?)? })),
        "insert-triangle" => Ok(StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index: crate::standards::v_ascii::subsets::any::io::text::diff::parse_usize(get("index")?)?, triangle: crate::standards::v_ascii::subsets::any::io::text::diff::dec_triangle(get("triangle")?)? })),
        "remove-triangle" => Ok(StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index: crate::standards::v_ascii::subsets::any::io::text::diff::parse_usize(get("index")?)? })),
        "set-triangle-normal" => Ok(StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index: crate::standards::v_ascii::subsets::any::io::text::diff::parse_usize(get("index")?)?, normal: crate::standards::v_ascii::subsets::any::io::text::diff::dec_vec3(get("normal")?)? })),
        "set-triangle-vertices" => Ok(StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index: crate::standards::v_ascii::subsets::any::io::text::diff::parse_usize(get("index")?)?, vertices: crate::standards::v_ascii::subsets::any::io::text::diff::dec_vertices(get("vertices")?)? })),
        other => Err(format!("stl op: unknown keyword {other:?}")),
    }
}

impl OpText for StlMutation {
    fn print_op(&self) -> String {
        print_stl_op(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_stl_op(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
