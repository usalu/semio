//! ⚖️ Imperative app — binary command protocol surface + laws (constitutional: protocol).

use imperative_op::ImperativeOperation;
use protocol::OpBinary;
use std::collections::BTreeMap;

/// 📦️ Encodes an `ImperativeOperation` to its binary command form.
pub fn encode_op(operation: &ImperativeOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `ImperativeOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<ImperativeOperation, protocol::ProtocolError> {
    ImperativeOperation::decode_op(bytes)
}

//#region 🔖️ImperativeCommand
/// 🎯️ B1: `ImperativePlayApp::Command` — the SOLE dispatch surface for imperative's own behavior
/// (`HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS`-style typed channel, mirroring
/// `shooting_protocol::ShootingCommand`). One variant per `create_imperative_app`'s declared action;
/// field shapes mirror each action's former JSON `args` object exactly. No `Serialize`/`Deserialize`
/// derive: `params`'s `BTreeMap<String, imperative::ValueDsl>` element type is DSL-only (`ValueDsl`
/// derives `dsl::DslRecord`, not `serde::Serialize`) — `DocumentApp::Command` only requires
/// `protocol::OpBinary + Send`, which `#[derive(dsl::DslOps)]` supplies directly.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
pub enum ImperativeCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse. The `*At` variants
    // address a nested `control.*` body via `owner`/`slot` (drag-and-drop into blocks).
    #[dsl(key = "add-step")]
    AddStep { kind: String, index: Option<usize> },
    #[dsl(key = "add-step-at")]
    AddStepAt { kind: String, index: Option<usize>, owner: Option<String>, slot: Option<String> },
    #[dsl(key = "remove-step")]
    RemoveStep { id: String },
    #[dsl(key = "remove-step-at")]
    RemoveStepAt { id: String, owner: Option<String>, slot: Option<String> },
    #[dsl(key = "move-step")]
    MoveStep { id: String, index: usize },
    #[dsl(key = "move-step-at")]
    MoveStepAt { id: String, index: usize, owner: Option<String>, slot: Option<String> },
    #[dsl(key = "set-step-params")]
    SetStepParams { id: String, params: BTreeMap<String, imperative::ValueDsl> },
    #[dsl(key = "set-step-params-at")]
    SetStepParamsAt { id: String, owner: Option<String>, slot: Option<String>, params: BTreeMap<String, imperative::ValueDsl> },
    // 👁️ Ephemeral view state / runtime effect — selection is scratch, `run` evaluates into config.
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "run")]
    Run,
    // 🗣️ Config-only (was ephemeral `ViewState::locale`) — emits `config_operations`, never document operations.
    #[dsl(key = "locale")]
    SetLocale { value: String },
}
//#endregion 🔖️ImperativeCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use imperative::PathRef;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Remove { id: "step-1".into() } };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    // [DEBUG] wire baseline dump for CRATE-CONSOLIDATION migration — remove after diffing.
    #[test]
    fn wire_baseline_dump() {
        use protocol::{OpBinary, OpText};
        fn hex(bytes: &[u8]) -> String {
            bytes.iter().map(|b| format!("{b:02x}")).collect()
        }
        let cases: Vec<(&str, ImperativeCommand)> = vec![
            ("AddStep(index=Some)", ImperativeCommand::AddStep { kind: "log.print".into(), index: Some(1) }),
            ("AddStep(index=None)", ImperativeCommand::AddStep { kind: "log.print".into(), index: None }),
            ("AddStepAt", ImperativeCommand::AddStepAt { kind: "log.print".into(), index: None, owner: Some("step-if".into()), slot: Some("then".into()) }),
            ("RemoveStep", ImperativeCommand::RemoveStep { id: "step-1".into() }),
            ("RemoveStepAt", ImperativeCommand::RemoveStepAt { id: "step-1".into(), owner: Some("step-if".into()), slot: Some("then".into()) }),
            ("MoveStep", ImperativeCommand::MoveStep { id: "step-1".into(), index: 2 }),
            ("MoveStepAt", ImperativeCommand::MoveStepAt { id: "step-1".into(), index: 2, owner: None, slot: None }),
            ("SetStepParams", ImperativeCommand::SetStepParams { id: "step-1".into(), params: imperative::dictionary_to_value_dsl_map(&imperative::Dictionary::new().insert("message", neural_engine::Value::Atom(neural_engine::Atom::String("updated".into())))) }),
            ("SetStepParamsAt", ImperativeCommand::SetStepParamsAt { id: "step-1".into(), owner: Some("step-if".into()), slot: Some("then".into()), params: imperative::dictionary_to_value_dsl_map(&imperative::Dictionary::new().insert("message", neural_engine::Value::Atom(neural_engine::Atom::String("updated".into())))) }),
            ("SetSelection", ImperativeCommand::SetSelection { ids: vec!["step-1".into(), "step-2".into()] }),
            ("Run", ImperativeCommand::Run),
            ("SetLocale", ImperativeCommand::SetLocale { value: "de-DE".into() }),
        ];
        for (label, command) in cases {
            let printed = command.print_op();
            let bytes = command.encode_op().expect("encode");
            println!("[DEBUG-WIRE] {label} | text={printed} | hex={}", hex(&bytes));
        }
    }

    #[test]
    fn document_text_round_trip_with_applied_operation() {
        use imperative::{Dictionary, ImperativeDocument, Step};
        use std::collections::BTreeMap;

        let document = imperative_engine::default_document();
        let envelope = store::create_document_envelope::<ImperativeDocument, ImperativeOperation>("imperative.document/v1", "test", document, None);
        let mut store = store::DocumentStore::new(envelope);
        let step = Step { id: "step-x".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Add { id: "step-x".to_string(), item: step, at: 0 } };
        store.dispatch(store::DocumentCommand::Apply { operations: vec![operation], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    //#region ImperativeCommand
    #[test]
    fn command_op_text_and_binary_round_trip_every_variant() {
        store::test_support::assert_op_line_round_trip(&ImperativeCommand::AddStep { kind: "log.print".into(), index: Some(1) });
        store::test_support::assert_op_line_round_trip(&ImperativeCommand::AddStep { kind: "log.print".into(), index: None });
        store::test_support::assert_op_line_round_trip(&ImperativeCommand::AddStepAt { kind: "log.print".into(), index: None, owner: Some("step-if".into()), slot: Some("then".into()) });
        store::test_support::assert_op_line_round_trip(&ImperativeCommand::RemoveStep { id: "step-1".into() });
        store::test_support::assert_op_line_round_trip(&ImperativeCommand::RemoveStepAt { id: "step-1".into(), owner: Some("step-if".into()), slot: Some("then".into()) });
        store::test_support::assert_op_line_round_trip(&ImperativeCommand::MoveStep { id: "step-1".into(), index: 2 });
        store::test_support::assert_op_line_round_trip(&ImperativeCommand::MoveStepAt { id: "step-1".into(), index: 2, owner: None, slot: None });
        store::test_support::assert_op_line_round_trip(&ImperativeCommand::SetSelection { ids: vec!["step-1".into(), "step-2".into()] });
        store::test_support::assert_op_line_round_trip(&ImperativeCommand::Run);
        store::test_support::assert_op_line_round_trip(&ImperativeCommand::SetLocale { value: "de-DE".into() });

        let bytes = ImperativeCommand::Run.encode_op().expect("encode");
        assert_eq!(ImperativeCommand::decode_op(&bytes).expect("decode"), ImperativeCommand::Run);
    }

    #[test]
    fn command_set_step_params_round_trips_via_dsl_value() {
        use neural_engine::{Atom, Value};
        let params = imperative::dictionary_to_value_dsl_map(&imperative::Dictionary::new().insert("message", Value::Atom(Atom::String("updated".into()))));
        let command = ImperativeCommand::SetStepParams { id: "step-1".into(), params };
        let printed = <ImperativeCommand as protocol::OpText>::print_op(&command);
        let parsed = <ImperativeCommand as protocol::OpText>::parse_op(&printed).expect("round trips");
        assert_eq!(parsed, command);
    }
    //#endregion ImperativeCommand
}
//#endregion 🧪️Tests
