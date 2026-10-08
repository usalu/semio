//! ⚡️ RewriteRule mutation text codec, registry, and external operation bridge.

use crate::RewritingSnapshot;

use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::operations::{
    apply_rewrite_rule_mutation, create_rewrite_rule_envelope, dispatch_rewrite_rule_mutations, inverse_rewrite_rule_mutation, RewriteRuleEnvelope, RewriteRuleStore,
};

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("EditWorkingGraph", crate::standards::v1::subsets::any::io::text::mutations::edit_working_graph::TEXT_OPCODE),
    ("EditLhs", crate::standards::v1::subsets::any::io::text::mutations::edit_lhs::TEXT_OPCODE),
    ("EditRhs", crate::standards::v1::subsets::any::io::text::mutations::edit_rhs::TEXT_OPCODE),
    ("ChangeParameterBinding", crate::standards::v1::subsets::any::io::text::mutations::change_parameter_binding::TEXT_OPCODE),
    ("RemoveParameterBinding", crate::standards::v1::subsets::any::io::text::mutations::remove_parameter_binding::TEXT_OPCODE),
    ("ChangeRuleLayoutPoint", crate::standards::v1::subsets::any::io::text::mutations::change_rule_layout_point::TEXT_OPCODE),
    ("RemoveRuleLayoutPoint", crate::standards::v1::subsets::any::io::text::mutations::remove_rule_layout_point::TEXT_OPCODE),
    ("DragRuleNodes", crate::standards::v1::subsets::any::io::text::mutations::drag_rule_nodes::TEXT_OPCODE),
    ("SetRuleLayoutPoints", crate::standards::v1::subsets::any::io::text::mutations::set_rule_layout_points::TEXT_OPCODE),
];
//#endregion 🧾️DerivedRegistry

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes the internally tagged JSON projection.
pub fn decode_rewriting_mutation_json(text: &str) -> Result<RewriteRuleMutation, semio_framework_value::ValueError> {
    let parsed=semio_framework_pack_json::parse(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string()))?;
    let value=crate::standards::v1::subsets::any::schema::snapshot::json::mutation(semio_framework_pack_json::to_dsl_value(&parsed),true)?;
    <RewriteRuleMutation as semio_framework_value::FromValue>::from_value(value)
}

/// 📤️ Renders the declared mutation roles with exact words and typed properties.
pub fn encode_rewriting_mutation_json(mutation:&RewriteRuleMutation)->Result<String,semio_framework_value::ValueError>{
    let value=crate::standards::v1::subsets::any::schema::snapshot::json::mutation(semio_framework_value::ToValue::to_value(mutation),false)?;
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&value)))
}

/// ▶️ Applies one mutation and returns its diagnostic code/severity pairs.
pub fn apply_rewriting_mutation_reporting(snapshot: &mut RewritingSnapshot, mutation: &RewriteRuleMutation) -> Vec<(String, String)> {
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(mutation, snapshot);
    let mut reported: Vec<(String, String)> = outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect();
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => *snapshot = next,
        Err(error) => reported.push((error.code, format!("{:?}", semio_framework_diagnostic::Severity::Fatal))),
    }
    reported
}

/// ↩️ Computes the mutation's own undo steps.
pub fn inverse_rewriting_mutation_steps(mutation: &RewriteRuleMutation, base: &RewritingSnapshot) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    Ok({
    <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::inverse(mutation, base)?

    })
}
//#endregion 🌉️ExternalCodecBridge

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for RewriteRuleMutation {
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


//#endregion 🔖️HandcraftedOpCodecs

#[path = "📐️change-rule-layout/🦀️.rs"]
pub mod change_rule_layout_point;

#[path = "🫳️drag-rule/🦀️.rs"]
pub mod drag_rule_nodes;

#[path = "👉️edit-rhs/🦀️.rs"]
pub mod edit_rhs;

#[path = "🖼️edit-working-graph/🦀️.rs"]
pub mod edit_working_graph;

#[path = "🗑️remove-rule-layout/🦀️.rs"]
pub mod remove_rule_layout_point;

#[path = "🔧️change-parameter/🦀️.rs"]
pub mod change_parameter_binding;

#[path = "👈️edit-lhs/🦀️.rs"]
pub mod edit_lhs;

#[path = "🧹️remove-parameter-binding/🦀️.rs"]
pub mod remove_parameter_binding;

#[path = "📍️set-rule-layout/🦀️.rs"]
pub mod set_rule_layout_points;
