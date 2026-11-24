/// 🧯️ A refused reducer step must report the APP's own fault, not a fixed sentence. Every refusal used
/// to read `retained command reducer rejected operation` — a missing retained session, an unsupported
/// export format and a rejected route were one indistinguishable message, which is why the viewer's
/// broken export took a browser probe to diagnose (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn a_refused_reducer_step_reports_the_apps_own_fault_code_and_message() {
    let mut fault = semio_framework::Fault::new(semio_framework::FaultOrigin::App, semio_framework::FaultCode::new("generation3d.io.export"), "format `dwg` has no readable descriptor set").with_param("path", "/format");
    fault.span = Some(semio_framework::TextSpan::with_length(3, 8, 3));
    let detail = crate::reducer_fault_detail(&fault);
    let oracle: serde_json::Value = serde_json::from_slice(&detail).expect("the independent JSON oracle accepts the canonical fault wire");
    assert_eq!(oracle["code"], "generation3d.io.export");
    let back = semio_plugin_decl_app::tool_contract::reducer_fault_of_detail(&detail).expect("the canonical reducer fault reads back");
    assert_eq!(back, fault);
}

/// 🧯️ A runaway message narrows the report rather than losing it, and never truncates mid-codepoint.
#[test]
fn an_oversized_reducer_fault_detail_is_clipped_on_a_char_boundary() {
    let fault = semio_framework::Fault::new(semio_framework::FaultOrigin::App, semio_framework::FaultCode::new("x"), "ü".repeat(4096));
    let detail = crate::reducer_fault_detail(&fault);
    assert!(detail.len() <= crate::ARTIFACT_COMMAND_FAULT_DETAIL_MAXIMUM_BYTES);
    assert!(serde_json::from_slice::<serde_json::Value>(&detail).is_ok());
    let back = semio_plugin_decl_app::tool_contract::reducer_fault_of_detail(&detail).expect("a narrowed detail remains a typed fault");
    assert!(back.message.is_char_boundary(back.message.len()));
}

/// 🔎️ LAW: the reducer's own code and message come back out of the detail the job fault carried, exactly — the agent
/// lane's preview answers with them — and a detail this module did not write reads as no reducer fault at all.
#[test]
fn a_reducer_fault_detail_reads_back_as_the_reducers_own_code_and_message() {
    let fault = semio_framework::Fault::new(semio_framework::FaultOrigin::App, semio_framework::FaultCode::new("app.command.targets-required"), "patchNodes needs node ids or a node selection");
    let back = semio_plugin_decl_app::tool_contract::reducer_fault_of_detail(&crate::reducer_fault_detail(&fault)).expect("a reducer detail reads back");
    assert_eq!((back.code.0.as_str(), back.message.as_str()), ("app.command.targets-required", "patchNodes needs node ids or a node selection"));
    for foreign in [b"puzzle command reducer rejected the admitted operation".as_slice(), b"retained command reducer rejected operation: ".as_slice(), b"".as_slice()] {
        assert!(semio_plugin_decl_app::tool_contract::reducer_fault_of_detail(foreign).is_none(), "{foreign:?} is not a reducer detail");
    }
}

/// 🧭️ LAW: a stalled publication names the cause it last waited on, and only for the operation that waited.
#[test]
fn a_publication_wait_names_its_cause_for_exactly_its_operation() {
    use crate::typed_operation::{last_publication_wait, note_publication_wait};
    note_publication_wait(7, "the store owner refused the granted page (copy bytes, capacity bytes, depth)", [1, 2, 3]);
    let wait = last_publication_wait(7).expect("the wait is readable for its operation");
    assert_eq!((wait.cause.starts_with("the store owner refused"), wait.have), (true, [1, 2, 3]));
    assert!(last_publication_wait(8).is_none());
}

/// 🧭️ LAW: a typed funding wait reads its phase, the short resource and `[stage, demand, grant]` in the stall message, and a plain wait keeps its bare cause.
#[test]
fn a_funding_wait_names_its_phase_resource_demand_and_grant() {
    use crate::typed_operation::{last_publication_wait, note_publication_unit_wait, note_publication_wait, PublicationWait};
    note_publication_unit_wait(9, PublicationWait::shortfall("the press scrub writer is not funded", "writer admission", "copy bytes", 700_000, 532_480).at_stage(2));
    let text = last_publication_wait(9).expect("the typed wait is readable").to_string();
    assert_eq!(text, "the press scrub writer is not funded [phase 'writer admission', short of copy bytes; stage, demand, grant] [2, 700000, 532480]");
    note_publication_wait(10, "the worker completion is not ready", [1, 2, 3]);
    assert_eq!(last_publication_wait(10).expect("the plain wait is readable").to_string(), "the worker completion is not ready [1, 2, 3]");
}
