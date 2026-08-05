//! ⚖️ Sourcing curate app — binary command protocol surface + laws (constitutional: protocol).
//!
//! 🎯️ Also hosts `SourcingCurateCommand` — the app-engine `AppCommand::Command` binary command envelope
//! (mirrors `shooting_protocol::ShootingCommand`). One variant per `create_sourcing_curate_app`'s real
//! declared action — the SOLE dispatch surface for `sourcing_ui::SourcingCurateApp::handle`.

use protocol::OpBinary;
use serde::{Deserialize, Serialize};
use sourcing_op::SourcingOperation;

/// 📦️ Encodes a `SourcingOperation` to its binary command form.
pub fn encode_op(operation: &SourcingOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `SourcingOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<SourcingOperation, protocol::ProtocolError> {
    SourcingOperation::decode_op(bytes)
}

//#region 🔖️SourcingCurateCommand
/// 🎯️ B1: `SourcingCurateApp::Command` — the SOLE dispatch surface for curate's own behavior, covering
/// every declared action. Field shapes mirror each action's real args exactly; `#[derive(dsl::DslOps)]`
/// gives this a binary (`OpBinary`) AND text (`OpText`) codec, matching `sourcing_op`'s conventions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SourcingCurateCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    /// 🛠️ Dev-only whole-document import — kept out of the command palette.
    #[dsl(key = "document-json")]
    SetDocumentJson { json: String },
    #[dsl(key = "active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "stock-from-catalogue")]
    StockFromCatalogue,
    #[dsl(key = "curate-add")]
    CurateAdd { object_id: String },
    /// 🎚️ The pool/curated tables' count stepper cell dispatches this SAME action for both a relative
    /// drag tick (`delta`) and an absolute typed value (`value`) — mirrors `SetFilterMinAvailability`'s
    /// two-mode shape (checked in that order, `delta` first).
    #[dsl(key = "curate-set-count")]
    CurateSetCount { object_id: String, delta: Option<f64>, value: Option<f64> },
    #[dsl(key = "curate-remove")]
    CurateRemove { object_id: String },
    #[dsl(key = "drop-on-pool")]
    DropOnPool { object_id: String },
    #[dsl(key = "drop-on-curated")]
    DropOnCurated { object_id: String },

    // 👁️ Config-only (was `CurateDocument.filters`/`.runtime`) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "filter-query")]
    SetFilterQuery { value: String },
    #[dsl(key = "filter-module")]
    SetFilterModule { module_id: String, enabled: bool },
    #[dsl(key = "filter-typology")]
    SetFilterTypology { path: String },
    #[dsl(key = "filter-min-availability")]
    SetFilterMinAvailability { delta: Option<f64>, value: Option<f64> },
    #[dsl(key = "sort-table")]
    SortTable { column_id: String, direction: String },
    #[dsl(key = "select-row")]
    SelectRow { object_id: Option<String> },
    #[dsl(key = "world-select")]
    WorldSelect { ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}
//#endregion 🔖️SourcingCurateCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use sourcing::CurateDocument;

    /// 🧪️ [DEBUG] TICKET 26/08/05/SOURCING-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION wire
    /// baseline dump — one value per `SourcingCurateCommand` variant, printed as
    /// `print_op(&c) | bytes.len() | hex(bytes)`. Delete once the post-migration diff is clean.
    #[test]
    fn wire_baseline_dump() {
        fn hex(bytes: &[u8]) -> String {
            bytes.iter().map(|b| format!("{b:02x}")).collect()
        }
        let commands: Vec<SourcingCurateCommand> = vec![
            SourcingCurateCommand::SetDocumentJson { json: "{}".into() },
            SourcingCurateCommand::SetActiveExample { example_id: "demo-stock".into() },
            SourcingCurateCommand::StockFromCatalogue,
            SourcingCurateCommand::CurateAdd { object_id: "beam-glulam-gl24h".into() },
            SourcingCurateCommand::CurateSetCount { object_id: "beam-glulam-gl24h".into(), delta: Some(1.0), value: None },
            SourcingCurateCommand::CurateSetCount { object_id: "beam-glulam-gl24h".into(), delta: None, value: Some(4.0) },
            SourcingCurateCommand::CurateRemove { object_id: "beam-glulam-gl24h".into() },
            SourcingCurateCommand::DropOnPool { object_id: "beam-glulam-gl24h".into() },
            SourcingCurateCommand::DropOnCurated { object_id: "beam-glulam-gl24h".into() },
            SourcingCurateCommand::SetFilterQuery { value: "glulam".into() },
            SourcingCurateCommand::SetFilterModule { module_id: "beams".into(), enabled: true },
            SourcingCurateCommand::SetFilterTypology { path: "beams/steel".into() },
            SourcingCurateCommand::SetFilterMinAvailability { delta: Some(1.0), value: None },
            SourcingCurateCommand::SetFilterMinAvailability { delta: None, value: Some(5.0) },
            SourcingCurateCommand::SortTable { column_id: "availability".into(), direction: "desc".into() },
            SourcingCurateCommand::SelectRow { object_id: Some("beam-glulam-gl24h".into()) },
            SourcingCurateCommand::SelectRow { object_id: None },
            SourcingCurateCommand::WorldSelect { ids: vec!["beam-glulam-gl24h".into(), "beam-kvh-c24".into()] },
            SourcingCurateCommand::SetLocale { value: "de-DE".into() },
        ];
        for command in &commands {
            let text = protocol::OpText::print_op(command);
            let bytes = protocol::OpBinary::encode_op(command).expect("encode");
            println!("{text} | {} | {}", bytes.len(), hex(&bytes));
        }
    }

    /// 🌱️ Mirrors `sourcing_engine`'s private test-only helper (see that crate's tests for why this
    /// tiny fixture-assembly helper is duplicated rather than shared across crates).
    fn sample_document() -> CurateDocument {
        CurateDocument { stock: sourcing_engine::sourcing_modules().iter().flat_map(|module| module.demo_kinds()).collect(), ..Default::default() }
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = SourcingOperation::SetDocument { document: sourcing_engine::empty_document() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    //#region 🔖️DslAndOpTextStore
    #[test]
    fn curate_document_text_round_trips_through_a_vcs_store() {
        let envelope = store::create_document_envelope(sourcing::SOURCING_CURATE_SCHEMA, "sourcing-curate-test", sample_document(), None);
        let mut store = store::DocumentStore::new(envelope);
        let mut next = store.projection().expect("projection").clone();
        sourcing_engine::curate_delta(&mut next, "beam-glulam-gl24h", 3);
        store.dispatch(store::DocumentCommand::Apply { operations: vec![SourcingOperation::SetDocument { document: next }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖️DslAndOpTextStore

    //#region 🔖️SourcingCurateCommand
    #[test]
    fn sourcing_curate_command_op_text_round_trips_every_variant() {
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::SetDocumentJson { json: "{}".into() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::SetActiveExample { example_id: "demo-stock".into() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::StockFromCatalogue);
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::CurateAdd { object_id: "beam-glulam-gl24h".into() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::CurateSetCount { object_id: "beam-glulam-gl24h".into(), delta: Some(1.0), value: None });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::CurateSetCount { object_id: "beam-glulam-gl24h".into(), delta: None, value: Some(4.0) });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::CurateRemove { object_id: "beam-glulam-gl24h".into() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::DropOnPool { object_id: "beam-glulam-gl24h".into() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::DropOnCurated { object_id: "beam-glulam-gl24h".into() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::SetFilterQuery { value: "glulam".into() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::SetFilterModule { module_id: "beams".into(), enabled: true });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::SetFilterTypology { path: "beams/steel".into() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::SetFilterMinAvailability { delta: Some(1.0), value: None });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::SetFilterMinAvailability { delta: None, value: Some(5.0) });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::SortTable { column_id: "availability".into(), direction: "desc".into() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::SelectRow { object_id: Some("beam-glulam-gl24h".into()) });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::SelectRow { object_id: None });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::WorldSelect { ids: vec!["beam-glulam-gl24h".into(), "beam-kvh-c24".into()] });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateCommand::SetLocale { value: "de-DE".into() });
    }
    //#endregion 🔖️SourcingCurateCommand
}
//#endregion 🧪️Tests
