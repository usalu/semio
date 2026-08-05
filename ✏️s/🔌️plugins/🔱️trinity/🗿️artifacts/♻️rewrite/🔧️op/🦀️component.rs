//! ⚡️ `trinity.rewrite.rule` artifact — operation enum + laws (constitutional: op).

use crate::artifacts::rewrite::diff::RewriteRuleDiff;
use crate::artifacts::rewrite::{RewriteRuleState, TrinityRewriteError, REWRITE_RULE_SCHEMA};
use protocol::Operation;
use serde::{Deserialize, Serialize};
use store::{create_document_envelope, DocumentCommand, DocumentEnvelope, DocumentStore};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum RewriteRuleOperation {
    SetState { state: RewriteRuleState },
}

impl Operation<RewriteRuleState> for RewriteRuleOperation {
    type Diff = RewriteRuleDiff;

    fn diff(&self, _projection: &RewriteRuleState) -> Self::Diff {
        match self {
            RewriteRuleOperation::SetState { state } => RewriteRuleDiff { next: Some(state.clone()) },
        }
    }

    fn backwards(&self, projection: &RewriteRuleState) -> Vec<Self> {
        vec![RewriteRuleOperation::SetState { state: projection.clone() }]
    }
}

pub type RewriteRuleEnvelope = DocumentEnvelope<RewriteRuleState, RewriteRuleOperation>;
pub type RewriteRuleStore = DocumentStore<RewriteRuleState, RewriteRuleOperation>;

pub fn create_rewrite_rule_envelope(id: &str, state: RewriteRuleState) -> RewriteRuleEnvelope {
    create_document_envelope(REWRITE_RULE_SCHEMA, id, state, None)
}

pub fn dispatch_rewrite_rule_state(store: &mut RewriteRuleStore, state: RewriteRuleState) -> Result<(), TrinityRewriteError> {
    let current = store.projection()?;
    if current == state {
        return Ok(());
    }
    store.dispatch(DocumentCommand::Apply { operations: vec![RewriteRuleOperation::SetState { state }], description: None }).map_err(TrinityRewriteError::from)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::PropertyValue;
    use crate::artifacts::rewrite::LayoutPoint;
    use protocol::OpText;
    use std::collections::BTreeMap;
    use store::test_support::{assert_document_pack_round_trip, assert_document_text_round_trip, assert_op_line_round_trip};

    fn sample_rule_state() -> RewriteRuleState {
        let mut parameter_bindings = BTreeMap::new();
        parameter_bindings.insert("label".to_string(), PropertyValue::String("nakagin-core".into()));
        parameter_bindings.insert("count".to_string(), PropertyValue::Number(3.0));
        let mut rule_layout = BTreeMap::new();
        rule_layout.insert("a".to_string(), LayoutPoint::from((10.5, -20.25)));
        RewriteRuleState {
            before_fixture_json: "{\"schema\":\"trinity.graph\",\"name\":\"x \\\"quoted\\\"\\nline\"}".to_string(),
            lhs_json: r#"{"pattern":{"leftVar":"a","leftKind":"Piece"}}"#.to_string(),
            rhs_json: r#"{"set":[{"var":"a","prop":"label","value":"$label"}]}"#.to_string(),
            parameter_bindings,
            rule_layout,
        }
    }

    #[test]
    fn op_text_round_trip_set_state() {
        assert_op_line_round_trip(&RewriteRuleOperation::SetState { state: sample_rule_state() });
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
        let err = RewriteRuleOperation::parse_op("bogus xyz").unwrap_err();
        assert!(err.message.contains("unknown operation line"));
    }

    /// 🎫️ CW7 command-envelope law: proves `RewriteRuleOperation`'s `Edit` round-trips through
    /// `protocol::OperationEnvelope`s.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};

        let mut store = RewriteRuleStore::new(create_rewrite_rule_envelope("test", sample_rule_state()));
        let mut next = sample_rule_state();
        next.lhs_json = "{}".into();
        dispatch_rewrite_rule_state(&mut store, next).unwrap();
        let edit: &Edit<RewriteRuleOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<RewriteRuleState, RewriteRuleOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
}
//#endregion 🧪️Tests
