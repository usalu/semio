//! ⚡️ RewriteRule mutation text codec, registry, and external operation bridge.

use crate::artifacts::rewrite::RewriteSnapshot;

pub use crate::artifacts::rewrite::schema::mutations::RewriteRuleMutation;
pub use crate::artifacts::rewrite::schema::operations::{apply_rewrite_rule_mutation, create_rewrite_rule_envelope, dispatch_rewrite_rule_mutations, inverse_rewrite_rule_mutation, rewrite_snapshot_mutations, RewriteRuleEnvelope, RewriteRuleStore};

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("EditBeforeFixture", super::edit_before_fixture::text::TEXT_OPCODE),
    ("EditLhs", super::edit_lhs::text::TEXT_OPCODE),
    ("EditRhs", super::edit_rhs::text::TEXT_OPCODE),
    ("ChangeParameterBinding", super::change_parameter_binding::text::TEXT_OPCODE),
    ("RemoveParameterBinding", super::remove_parameter_binding::text::TEXT_OPCODE),
    ("ChangeRuleLayoutPoint", super::change_rule_layout_point::text::TEXT_OPCODE),
    ("RemoveRuleLayoutPoint", super::remove_rule_layout_point::text::TEXT_OPCODE),
];
//#endregion 🧾️DerivedRegistry

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes the internally tagged JSON projection.
pub fn decode_rewrite_mutation_json(text: &str) -> Result<RewriteRuleMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies one mutation and returns its diagnostic code/severity pairs.
pub fn apply_rewrite_mutation_reporting(snapshot: &mut RewriteSnapshot, mutation: &RewriteRuleMutation) -> Vec<(String, String)> {
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::diff(mutation, snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ Computes the mutation's own undo steps.
pub fn inverse_rewrite_mutation_steps(mutation: &RewriteRuleMutation, base: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
    <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for RewriteRuleMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for RewriteRuleMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs
