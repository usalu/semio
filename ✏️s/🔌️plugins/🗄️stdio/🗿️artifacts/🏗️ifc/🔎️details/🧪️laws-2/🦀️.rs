//! 🧪️ Retained IFC work laws, part 2, run against the real IFC4 editor.
#![cfg(test)]

use semio_s_artifact_stdio_ifc_declarations::{IfcAttributeDeclaration, IfcCatalog, IfcTypeExpression};
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Locale;
use semio_framework_value::{DslValue, FromValue, RetainedWorkOutcome, ToValue};
use semio_s_artifact_stdio_contract::editing;
use semio_s_artifact_stdio_contract::editing::{DslSnapshotDetailsProvider, SnapshotDetailItemCapabilities, SnapshotDetailPathSegment as P, SnapshotDetailPresentation, SnapshotDetailValue, SnapshotDetailVariant, SnapshotDetailsProvider};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{ifc_arguments_identical, ifc_complex_identical, IfcComplexType, IfcEntity, IfcSnapshot, IfcTypedValue, IfcValue};
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{edit_rules, IfcMutation};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{EditorApp, Emit};
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::sync::Arc;
use semio_s_artifact_stdio_ifc_details::*;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
type DetailsTestEditor = semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::editor::ifc4_any::Ifc4AnyEditor;
fn details_test_command(event: semio_s_artifact_stdio_contract::editing::SnapshotEditEvent) -> semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::editor::ifc4_any::Ifc4AnyEditCommand {
    semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::editor::ifc4_any::Ifc4AnyEditCommand::EditSnapshot { event }
}
type Source = SnapshotSource;
use protocol::ColdMutation;
use semio_framework_job::StepContext;
use semio_framework_plugin::plugin_app_close_prelude::OriginalWorkFixture;
use semio_framework_plugin::retained_command::ArtifactCommandSnapshotRead;
use semio_framework_plugin::{AppOperationContext, HistoryView, NoConfig};
type Work = IfcDetailsWork<DetailsTestEditor, DetailsTestProfile>;

#[path="../constituent_remap_tests.rs"]mod constituent_remap_tests;
#[path="../primary_remap_tests.rs"]mod primary_remap_tests;
#[path="../type_input_tests.rs"]mod type_input_tests;
#[path="../candidate_rule_tests.rs"]mod candidate_rule_tests;
#[path="../type_number_binary_input_tests.rs"]mod type_number_binary_input_tests;
#[path="../source_loan_tests.rs"]mod source_loan_tests;
#[path="../type_guidance_request_tests.rs"]mod type_guidance_request_tests;
#[path="../type_guidance_candidate_tests.rs"]mod type_guidance_candidate_tests;
#[path="../type_number_binary_guidance_tests.rs"]mod type_number_binary_guidance_tests;
#[path="../type_list_input_tests.rs"]mod type_list_input_tests;
#[path="../type_select_input_tests.rs"]mod type_select_input_tests;
#[path="../type_reference_select_input_tests.rs"]mod type_reference_select_input_tests;
#[path="../type_mixed_select_input_tests.rs"]mod type_mixed_select_input_tests;
#[path="../type_set_select_input_tests.rs"]mod type_set_select_input_tests;
#[path="../type_measure_select_input_tests.rs"]mod type_measure_select_input_tests;
#[path="../type_value_select_input_tests.rs"]mod type_value_select_input_tests;
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1_000_000, maximum_copy_bytes: 1_048_576, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 }
}
fn turn<T>(caller: &mut OriginalWorkFixture, fuel: u64, cancelled: bool, run: impl FnOnce(&mut StepContext<'_>) -> Result<T, semio_framework_job::OriginalTurnFailure>) -> Result<T, semio_framework_job::OriginalTurnFailure> {
    if cancelled {
        caller.cancel().cancel_now();
    }
    caller.work_turn(grant(), fuel, run)
}
fn wall(count: usize) -> Arc<IfcSnapshot> {
    let mut args = vec![IfcValue::Unset; count];
    args[0] = IfcValue::String("0123456789012345678901".into());
    if count > 2 {
        args[2] = IfcValue::String("Wall".into());
    }
    Arc::new(IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCWALL".into(), args, complex: Vec::new() }], ..Default::default() })
}
fn inputs<T>(
    snapshot: &Arc<IfcSnapshot>,
    event: editing::SnapshotEditEvent,
    caller: &mut OriginalWorkFixture,
    work: &mut Work,
    run: impl FnOnce(&ArtifactCommandInputs<'_, EditorApp<DetailsTestEditor>>, &mut OriginalWorkFixture, &mut Work) -> T,
) -> T {
    let mut command = details_test_command(event);
    assert!(turn(caller, 1_000_000, false, |cx| work.receive_original_command(&mut command, cx)).unwrap());
    let config = NoConfig {};
    let history = HistoryView::empty();
    let interaction = Default::default();
    let hover = Default::default();
    let operation = AppOperationContext { app_instance_id: 17, parent_document_id: "ifc-work-test".into(), operation_id: 716, generation: 3, canonical_base_revision: [0; 32], retained: grant(), authoring_seed: "ifc-work-test".into() };
    let input = ArtifactCommandInputs {
        command: &command,
        snapshot: snapshot.as_ref(),
        snapshot_owner: Some(snapshot),
        snapshot_read: ArtifactCommandSnapshotRead::unavailable(&operation),
        config: &config,
        history: &history,
        interaction: &interaction,
        hover: &hover,
        context: None,
        operation: &operation,
    };
    run(&input, caller, work)
}
fn prepare(caller: &mut OriginalWorkFixture, work: &mut Work, input: &ArtifactCommandInputs<'_, EditorApp<DetailsTestEditor>>) {
    for _ in 0..2048 {
        if work.entity.is_some() || work.value.is_some() {
            return;
        }
        let step = turn(caller, 1_000_000, false, |cx| work.step(input, cx)).unwrap();
        assert!(matches!(step, ArtifactCommandWorkStep::Pending { .. } | ArtifactCommandWorkStep::Progress { .. }));
    }
    panic!("retained IFC source preparation did not finish")
}
fn close(caller: &mut OriginalWorkFixture, work: &mut Work) {
    work.begin_close();
    for _ in 0..8192 {
        if work.terminal_is_empty() {
            return;
        }
        let step = caller.close(work, grant());
        assert!(step.is_ok());
    }
    panic!("retained IFC owned close did not drain")
}
fn finish(caller: &mut OriginalWorkFixture, work: &mut Work, input: &ArtifactCommandInputs<'_, EditorApp<DetailsTestEditor>>) -> ArtifactCommandWorkStep<EditorApp<DetailsTestEditor>> {
    for _ in 0..2048 {
        let step = turn(caller, 1_000_000, false, |cx| work.step(input, cx)).unwrap();
        if matches!(step, ArtifactCommandWorkStep::Complete(_)) {
            return step;
        }
    }
    panic!("retained IFC publication did not finish")
}
