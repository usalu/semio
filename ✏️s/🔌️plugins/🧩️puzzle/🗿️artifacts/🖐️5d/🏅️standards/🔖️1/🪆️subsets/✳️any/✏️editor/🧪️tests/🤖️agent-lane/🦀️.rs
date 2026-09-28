//! 🤖️ The puzzle 5d editor's own tool-command job (`puzzle.5d.tool-command.v1`) on the agent lane: what the semio MCP gateway's
//! `action_prepare` previews for the verbs its coverage battery reaches on a puzzle 5d document.

use super::*;
use semio_framework_plugin::artifact_app_laws::{declared_verb_agent_divergences, declared_verb_probe, declared_verb_wrote_document, probe_agent_lane, DeclaredVerbOutcome};

/// 🤖️ The verbs the semio MCP coverage battery (`plugin-coverage-check`) tries on a fresh puzzle 5d document, in its order —
/// every one was refused `interactive-job.preview-unsupported` before the agent lane ran the editor's own job.
const BATTERY_VERBS: &[&str] = &["addPartKind", "addBrushPart", "addNode", "rotateSelection", "scaleSelection", "translateSelection", "patchFastener", "patchGrip"];

/// ⚖️ LAW: the agent lane runs the editor's own retained puzzle job for every battery verb — none is unreachable there — and
/// `addPartKind` previews the document write the shell publishes; the agent lane diverges from the shell lane on exactly
/// the pinned verbs.
#[semio_framework_async_macros::async_test]
async fn the_agent_lane_runs_the_puzzle5d_editors_own_tool_command_job() {
    let probes = probe_agent_lane::<semio_framework_plugin::EditorApp<Puzzle5dPlayApp>, <Puzzle5dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(create_puzzle5d_app, None, BATTERY_VERBS).await;
    for probe in &probes {
        assert!(!matches!(&probe.agent, Some(DeclaredVerbOutcome::Unreachable { .. })), "{} is unreachable on the agent lane: {:?}", probe.verb, probe.agent);
    }
    let written = declared_verb_probe(&probes, "addPartKind");
    assert!(declared_verb_wrote_document(written.agent.as_ref().expect("addPartKind is agent-facing")), "addPartKind previews its document write: {:?}", written.agent);
    assert_eq!(declared_verb_agent_divergences(&probes), Vec::<String>::new(), "agent-lane divergences");
}
