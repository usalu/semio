//! 📝️ Text representation codec surface for `stdio.obj` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v3_0::subsets::any::schema::mutations::*;
use crate::schema::diff::{
    diff_insert_face, diff_insert_normal, diff_insert_texcoord, diff_insert_vertex, diff_remove_face, diff_remove_group, diff_remove_normal, diff_remove_object, diff_remove_texcoord, diff_remove_vertex, diff_set_face, diff_set_group,
    diff_set_mtllib, diff_set_normal, diff_set_object, diff_set_smoothing_groups, diff_set_texcoord, diff_set_unknown_statements, diff_set_usemtl, diff_set_vertex, face_diff_between, normal_diff_between, texcoord_diff_between,
    vertex_diff_between, ObjDiff,
};
use crate::schema::snapshot::{ObjFace, ObjNormal, ObjSmoothingRange, ObjTexCoord, ObjUnknownStatement, ObjUsemtlRange, ObjVertex};
#[cfg(test)]
use crate::schema::snapshot::{ObjFaceVertex, ObjGroup, ObjObject};
use crate::ObjSnapshot;
use protocol::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};

/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only) — the same ~15-line body
/// every `DslOps`-derived enum's `OpText` impl uses (`GifMutation`, `FlowMutationDsl`,
/// `SpaceMutation`; see `f6-recon-report.md` §2).
impl OpText for ObjMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}
}
pub use mutations_codec::*;
