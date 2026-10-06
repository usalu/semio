//! ⚡️ SSpace index artifact — OpText/OpBinary codecs + grammar for `SSpaceMutation`.

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;

pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("create-artifact", crate::standards::v1::subsets::any::schema::mutations::create_artifact::TEXT_OPCODE),
    ("delete-artifact", crate::standards::v1::subsets::any::schema::mutations::delete_artifact::TEXT_OPCODE),
    ("rename-artifact", crate::standards::v1::subsets::any::schema::mutations::rename_artifact::TEXT_OPCODE),
    ("touch-artifact", crate::standards::v1::subsets::any::schema::mutations::touch_artifact::TEXT_OPCODE),
];

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for SSpaceMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for SSpaceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

#[path = "🏷️rename-artifact/🦀️.rs"]
pub mod rename_artifact;

#[path = "🗑️delete-artifact/🦀️.rs"]
pub mod delete_artifact;

#[path = "🕒touch-artifact/🦀️.rs"]
pub mod touch_artifact;

#[path = "🌱create-artifact/🦀️.rs"]
pub mod create_artifact;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::operations::*;
#[cfg(test)]
use crate::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
#[cfg(test)]
use crate::standards::v1::subsets::any::schema::mutations::{create_artifact as semantic_create_artifact, delete_artifact as semantic_delete_artifact, register_s_space_mutation_descriptors, rename_artifact as semantic_rename_artifact, touch_artifact as semantic_touch_artifact};
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `SSpaceMutation` nor
/// `SSpaceSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🔣️oracle.json — the catalog and the recorded no-oracle decision.
pub fn s_space_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<SSpaceSnapshot, String> {
        let decoded: SSpaceSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: SSpaceMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = semio_framework_pack_json::json!({
        "base": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&base)),
        "expectedSnapshot": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&expected)),
        "snapshot": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&applied)),
        "diff": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(forward.diff())),
        "messages": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&forward.messages().to_vec())),
        "inverseSteps": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&inverse)),
        "inverseSnapshot": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&undone)),
        "inverseMessages": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&inverse_messages)),
    });
    Ok(report.to_string())
}
}
pub use mutations_codec::*;
