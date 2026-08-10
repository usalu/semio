//! ⚡️ `trinity.rewrite.rule` artifact — operation enum + laws (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::{RewriteSnapshot, TrinityRewriteError, REWRITE_RULE_SCHEMA};
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{create_document_envelope, DocumentCommand, DocumentEnvelope, DocumentStore};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum RewriteRuleMutation {
    SetState { state: RewriteSnapshot },
}





impl Mutation<RewriteSnapshot> for RewriteRuleMutation {
    type Diff = RewriteDiff;

    fn diff(&self, _snapshot: &RewriteSnapshot) -> Self::Diff {
        match self {
            RewriteRuleMutation::SetState { state } => crate::artifacts::rewrite::diff::diff_set_state(state),
        }
    }

    fn inverse(&self, snapshot: &RewriteSnapshot) -> Vec<Self> {
        vec![RewriteRuleMutation::SetState { state: snapshot.clone() }]
    }
}

pub type RewriteRuleEnvelope = DocumentEnvelope<RewriteSnapshot, RewriteRuleMutation>;
pub type RewriteRuleStore = DocumentStore<RewriteSnapshot, RewriteRuleMutation>;

pub fn create_rewrite_rule_envelope(id: &str, state: RewriteSnapshot) -> RewriteRuleEnvelope {
    create_document_envelope(REWRITE_RULE_SCHEMA, id, state, None)
}

pub fn dispatch_rewrite_rule_state(store: &mut RewriteRuleStore, state: RewriteSnapshot) -> Result<(), TrinityRewriteError> {
    let current = store.snapshot()?;
    if current == state {
        return Ok(());
    }
    store.dispatch(DocumentCommand::Apply { mutations: vec![RewriteRuleMutation::SetState { state }], description: None }).map_err(TrinityRewriteError::from).map(|_| ())
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::PropertyValue;
    use crate::artifacts::rewrite::LayoutPoint;
    use protocol::OpText;
    use std::collections::BTreeMap;
    use ::store::os_store::test_support::{assert_document_pack_round_trip, assert_document_text_round_trip, assert_op_line_round_trip};

    fn sample_rule_state() -> RewriteSnapshot {
        let mut parameter_bindings = BTreeMap::new();
        parameter_bindings.insert("label".to_string(), PropertyValue::String("nakagin-core".into()));
        parameter_bindings.insert("count".to_string(), PropertyValue::Number(3.0));
        let mut rule_layout = BTreeMap::new();
        rule_layout.insert("a".to_string(), LayoutPoint::from((10.5, -20.25)));
        RewriteSnapshot {
            before_fixture_json: "{\"schema\":\"trinity.graph\",\"name\":\"x \\\"quoted\\\"\\nline\"}".to_string(),
            lhs_json: r#"{"pattern":{"leftVar":"a","leftKind":"Piece"}}"#.to_string(),
            rhs_json: r#"{"set":[{"var":"a","prop":"label","value":"$label"}]}"#.to_string(),
            parameter_bindings,
            rule_layout,
        }
    }

    #[test]
    fn op_text_round_trip_set_state() {
        assert_op_line_round_trip(&RewriteRuleMutation::SetState { state: sample_rule_state() });
    }

    #[test]
    fn document_text_round_trip_rewrite_rule_store() {
        let mut store = RewriteRuleStore::new(create_rewrite_rule_envelope("test", sample_rule_state()));
        let mut next = sample_rule_state();
        next.lhs_json = "{}".into();
        dispatch_rewrite_rule_state(&mut store, next).unwrap();
        assert_document_text_round_trip(&store);
        assert_document_pack_round_trip(&store);
    }

    #[test]
    fn op_text_parse_op_errors_on_unknown_keyword() {
        let err = RewriteRuleMutation::parse_op("bogus xyz").unwrap_err();
        assert!(err.message.contains("unknown mutation line"));
    }

    /// 🎫️ CW7 command-envelope law: proves `RewriteRuleMutation`'s `Edit` round-trips through
    /// `protocol::MutationEnvelope`s.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};

        let mut store = RewriteRuleStore::new(create_rewrite_rule_envelope("test", sample_rule_state()));
        let mut next = sample_rule_state();
        next.lhs_json = "{}".into();
        dispatch_rewrite_rule_state(&mut store, next).unwrap();
        let edit: &Edit<RewriteRuleMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        ::store::os_store::test_support::assert_command_envelope_round_trip::<RewriteSnapshot, RewriteRuleMutation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
}
//#endregion 🧪️Tests


pub fn apply_rewrite_rule_mutation(snapshot: &mut RewriteSnapshot, mutation: &RewriteRuleMutation) {
    match mutation {
        RewriteRuleMutation::SetState { state } => *snapshot = state.clone(),
    }
}

pub fn inverse_rewrite_rule_mutation(snapshot: &RewriteSnapshot, mutation: &RewriteRuleMutation) -> Vec<RewriteRuleMutation> {
    mutation.inverse(snapshot)
}
