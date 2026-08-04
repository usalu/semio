//! ⚖️ Trinity Rewrite app — binary command protocol surface + laws (constitutional: protocol).

use protocol::OpBinary;
use rewrite_op::RewriteRuleOperation;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `RewriteRuleOperation` to its binary command form.
pub fn encode_op(operation: &RewriteRuleOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RewriteRuleOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<RewriteRuleOperation, protocol::ProtocolError> {
    RewriteRuleOperation::decode_op(bytes)
}

//#region 🔖️TrinityRewriteCommand
/// 🎯️ B1: `TrinityRewritePlayApp::Command` — the SOLE dispatch surface for rewrite's own behavior,
/// replacing the deleted stringly-typed `handle_action`. Field shapes mirror each pre-B1 action's real
/// args exactly; `#[derive(dsl::DslOps)]` gives this a binary (`OpBinary`) AND text (`OpText`) codec,
/// matching `trinity_jack_protocol::TrinityJackCommand`'s derive/attribute conventions. `NodeGraphEdit`
/// keeps its JSON-array `operations` shape (rather than a typed sub-enum) — the same
/// `{"operation":"setFixture"|"deleteSelection", ...}` payload `apply_rewrite_node_graph_edit_operations`
/// already parses, now carried as a `TrinityJackCommand::SetFixtureJson`-style opaque string field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum TrinityRewriteCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "node-graph-edit")]
    NodeGraphEdit { surface_id: String, operations_json: String },
    #[dsl(key = "set-lhs-json")]
    SetLhsJson { value: String },
    #[dsl(key = "set-rhs-json")]
    SetRhsJson { value: String },
    #[dsl(key = "set-parameter")]
    SetParameter { name: String, value: String },
    #[dsl(key = "add-rule-clause")]
    AddRuleClause { kind: String },
    #[dsl(key = "reset-rule")]
    ResetRule,
    #[dsl(key = "patch-nodes")]
    PatchNodes { node_ids: Vec<String>, field: String, value: String },

    // 👁️ Config-only — was ephemeral `RewritePlayRuntime` state, now emits `config_operations`.
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String>, surface_id: Option<String> },
    #[dsl(key = "node-graph-hover")]
    NodeGraphHover { surface_id: Option<String>, node_id: Option<String> },
    #[dsl(key = "set-viewport")]
    SetViewport { surface_id: Option<String>, viewport_json: String },
    #[dsl(key = "graph-pointer-down")]
    GraphPointerDown { node_id: Option<String> },
    #[dsl(key = "text-select")]
    TextSelect { var: Option<String>, start: Option<u64> },
    #[dsl(key = "text-hover")]
    TextHover { var: Option<String>, offset: Option<u64> },
    #[dsl(key = "reorganize")]
    Reorganize,
    #[dsl(key = "set-lod-mode")]
    SetLodMode { window_id: String, value: String },
    #[dsl(key = "set-locale")]
    SetLocale { value: String },
}
//#endregion 🔖️TrinityRewriteCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::OpText;
    use rewrite::{LayoutPoint, RewriteRuleState};
    use rewrite_op::{create_rewrite_rule_envelope, dispatch_rewrite_rule_state, RewriteRuleStore};
    use std::collections::BTreeMap;
    use store::test_support::{assert_document_pack_round_trip, assert_document_text_round_trip, assert_op_line_round_trip};
    use trinity_ram::PropertyValue;

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

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `RewriteRuleOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this
    /// file's existing pack round-trip law (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
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
    //#endregion 🔖️CommandEnvelopeTests

    #[test]
    fn op_text_parse_op_errors_on_unknown_keyword() {
        let err = RewriteRuleOperation::parse_op("bogus xyz").unwrap_err();
        assert!(err.message.contains("unknown operation line"));
    }

    #[test]
    fn trinity_rewrite_command_text_and_binary_round_trip() {
        let commands = vec![
            TrinityRewriteCommand::NodeGraphEdit { surface_id: "trinity.rewrite.before".into(), operations_json: "[]".into() },
            TrinityRewriteCommand::SetLhsJson { value: "{}".into() },
            TrinityRewriteCommand::SetRhsJson { value: "{}".into() },
            TrinityRewriteCommand::SetParameter { name: "label".into(), value: "hi".into() },
            TrinityRewriteCommand::AddRuleClause { kind: "where".into() },
            TrinityRewriteCommand::ResetRule,
            TrinityRewriteCommand::PatchNodes { node_ids: vec!["a".into()], field: "name".into(), value: "Renamed".into() },
            TrinityRewriteCommand::SetSelection { ids: vec!["n1".into()], surface_id: Some("trinity.rewrite.before".into()) },
            TrinityRewriteCommand::NodeGraphHover { surface_id: Some("trinity.rewrite.before".into()), node_id: Some("n1".into()) },
            TrinityRewriteCommand::SetViewport { surface_id: Some("trinity.rewrite.before".into()), viewport_json: "{\"x\":1.0,\"y\":2.0,\"zoom\":1.0}".into() },
            TrinityRewriteCommand::GraphPointerDown { node_id: Some("n1".into()) },
            TrinityRewriteCommand::TextSelect { var: Some("a".into()), start: None },
            TrinityRewriteCommand::TextHover { var: None, offset: Some(3) },
            TrinityRewriteCommand::Reorganize,
            TrinityRewriteCommand::SetLodMode { window_id: "trinity-rewrite-before".into(), value: "compact".into() },
            TrinityRewriteCommand::SetLocale { value: "de-DE".into() },
        ];
        for command in commands {
            let bytes = command.encode_op().expect("encode");
            assert_eq!(TrinityRewriteCommand::decode_op(&bytes).expect("decode"), command);
            let text = command.print_op();
            assert_eq!(TrinityRewriteCommand::parse_op(&text).expect("parse"), command);
        }
    }
}
//#endregion 🧪️Tests
