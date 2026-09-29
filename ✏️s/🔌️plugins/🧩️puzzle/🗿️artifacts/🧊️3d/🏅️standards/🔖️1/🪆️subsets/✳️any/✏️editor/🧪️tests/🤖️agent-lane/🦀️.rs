//! 🤖️ The puzzle 3d editor's own tool-command job (`puzzle.3d.fixture.tool-command.v1`) on the agent lane: what the semio MCP gateway's
//! `action_prepare` previews for the verbs its coverage battery reaches on a puzzle 3d document.

use super::*;
use semio_framework_plugin::artifact_app_laws::{declared_verb_agent_divergences, declared_verb_probe, declared_verb_wrote_document, probe_agent_lane, DeclaredVerbOutcome};

/// 🤖️ The verbs the semio MCP coverage battery (`plugin-coverage-check`) tries on a fresh puzzle 3d document, in its order —
/// every one was refused `interactive-job.preview-unsupported` before the agent lane ran the editor's own job.
const BATTERY_VERBS: &[&str] = &["addObjectKind", "rotateSelection", "scaleSelection", "translateSelection", "patchInspector", "relocateTargetVolume", "worldRelocate", "createAttraction"];

/// 🧭️ The battery verbs whose agent lane diverges from the shell lane on the boot example, pinned so a change turns this law red:
/// nothing is selected there, so the shell answers each with a notice alone and the agent lane refuses it
/// `app.command.no-effect` (the notice its structured cause) — neither lane changes the document.
const AGENT_LANE_DIVERGENCES: &[&str] = &["translateSelection", "rotateSelection", "scaleSelection"];

/// ⚖️ LAW: the agent lane runs the editor's own retained puzzle job for every battery verb — nothing refuses one before its
/// job ran — `addObjectKind` previews the document write the shell publishes, and the agent lane diverges from the shell lane on
/// exactly [`AGENT_LANE_DIVERGENCES`]: the fail-closed preview refuses by name what a shell run publishes beyond the
/// transaction, never settles without it.
#[semio_framework_async_macros::async_test]
async fn the_agent_lane_runs_the_puzzle3d_editors_own_tool_command_job() {
    let probes = probe_agent_lane::<semio_framework_plugin::EditorApp<Puzzle3dPlayApp>, <Puzzle3dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(create_puzzle3d_app, None, BATTERY_VERBS).await;
    let outcomes = probes.iter().map(|probe| format!("{}: shell {:?} / agent {:?}", probe.verb, probe.windows.iter().map(|window| &window.staged).collect::<Vec<_>>(), probe.agent)).collect::<Vec<_>>();
    for probe in &probes {
        assert!(!matches!(&probe.agent, Some(DeclaredVerbOutcome::Unreachable { code, .. }) if code != "interactive-job.agent-lane-uncarried"), "{} is refused before its job ran: {outcomes:#?}", probe.verb);
    }
    let written = declared_verb_probe(&probes, "addObjectKind");
    assert!(declared_verb_wrote_document(written.agent.as_ref().expect("addObjectKind is agent-facing")), "addObjectKind previews its document write: {outcomes:#?}");
    assert_eq!(declared_verb_agent_divergences(&probes), AGENT_LANE_DIVERGENCES, "agent-lane divergences: {outcomes:#?}");
    for verb in AGENT_LANE_DIVERGENCES {
        assert!(matches!(&declared_verb_probe(&probes, verb).agent, Some(DeclaredVerbOutcome::Refused { code, .. }) if code == "app.command.no-effect"), "{verb} is refused as no effect: {outcomes:#?}");
    }
}
