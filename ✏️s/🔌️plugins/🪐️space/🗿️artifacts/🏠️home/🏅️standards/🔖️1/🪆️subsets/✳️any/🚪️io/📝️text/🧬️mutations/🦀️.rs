//! ⚡️ SHome artifact — OpText/OpBinary codecs + grammar for `SHomeMutation`.

use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;

pub const TEXT_OPCODES: &[(&str, &str)] = &[("ChangeCatalogGeneration", crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation::TEXT_OPCODE)];

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for SHomeMutation {
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

impl protocol::OpBinary for SHomeMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

#[path = "🔢️change-catalog-generation/🦀️.rs"]
pub mod change_catalog_generation;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::operations::*;
use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
#[cfg(test)]
use crate::standards::v1::subsets::any::schema::mutations::{change_catalog_generation as semantic_change_catalog_generation, register_s_home_mutation_descriptors};
use crate::SHomeSnapshot;

/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `SHomeMutation` nor
/// `SHomeSnapshot` can be named there, and hand-transcribing either into a Rust literal
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
pub fn s_home_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<SHomeSnapshot, String> {
        let decoded: SHomeSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: SHomeMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(step, &undone).apply_to(&mut undone);
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
