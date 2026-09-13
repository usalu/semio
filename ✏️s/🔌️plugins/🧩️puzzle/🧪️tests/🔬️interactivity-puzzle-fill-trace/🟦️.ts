import { interactivityPuzzleFillTraceFailures } from "../../../../../📜️script.ts";
import { PUZZLE_FILL_TOOL_RUN_FIXTURE, puzzleFillToolRunFixtureWith } from "../🔬️interactivity-puzzle-fill-run-job/🟦️.ts";

const CASES: readonly (readonly [name: string, expect: "report" | "silent", edits: Parameters<typeof puzzleFillToolRunFixtureWith>[1]])[] = [
  ["candidate-page-producer", "report", [["fill", "    retirement: FillBuilderRetirementCursor,", "    retirement: FillBuilderRetirementCursor,\n    candidate_page: [Option<String>; 8],"]]],
  ["candidate-ghost-schema", "report", [["schema", "    pub requested: u32,", "    pub requested: u32,\n    pub candidate_ghost: Option<FillCandidateGhost>,"]]],
  ["diagnostic-transport", "report", [["transport", "WorldScene::from_session(precompute)", "world_fill_preview_json(precompute).map(WorldScene::from_json).unwrap_or_default()"]]],
  ["diagnostic-5d-window", "report", [["puzzle5dWindow", "WorldScene::from_session(precompute)", "world_fill_preview_json(precompute).map(WorldScene::from_json).unwrap_or_default()"]]],
  ["diagnostic-5d-precompute", "report", [["puzzle5dPrecompute", "    inner.fill_run_job(scene, operation)", "    let _ = FillDiagnostic::default();\n    inner.fill_run_job(scene, operation)"]]],
  ["diagnostic-overlay", "report", [["renderer", "  return <ToolRunTraceLayer lane={lane} />;", "  return <FillDiagnosticOverlay diagnostic={parse(lane)} />;"]]],
  ["fill-data-attributes", "report", [["renderer", "<ToolRunTraceLayer lane={lane} />", "<ToolRunTraceLayer lane={lane} data-fill-truncated={false} />"]]],
  ["reveal-index", "report", [["schema", "    pub requested: u32,", "    pub requested: u32,\n    pub reveal_index: Option<usize>,"]]],
  ["reveal-cutoffs", "report", [["renderer", "<ToolRunTraceLayer lane={lane} />", "<ToolRunTraceLayer lane={lane} revealCutoffs={{}} />"]]],
  ["unmounted-trace-layer", "report", [["renderer", "<ToolRunTraceLayer lane={lane} />", "null"]]],
  ["legacy-preview-fixture", "report", [["previewFixture", "", '{ "schema": "semio.puzzle3d.fill-preview-json.v1" }']]],
  ["commented-reveal-is-silent", "silent", [["renderer", "/** 🌐️ World host. */", "/** 🌐️ World host; the revealCutoffs and FillDiagnosticOverlay are gone. */"]]],
];

/** 🧪️ Mutation self-test of the puzzle fill trace law: the clean tool-run fixture passes, every planted diagnostic page, reveal cutoff or missing trace mount is reported, and prose stays silent. Returns the executed case count. */
export function interactivityPuzzleFillTraceSelfTests(): number {
  const clean = interactivityPuzzleFillTraceFailures(PUZZLE_FILL_TOOL_RUN_FIXTURE);
  if (clean.length > 0) throw new Error(`[verify interactivity] Puzzle fill trace clean fixture was falsely rejected: ${clean.join("; ")}`);
  for (const [name, expect, edits] of CASES) {
    const failures = interactivityPuzzleFillTraceFailures(puzzleFillToolRunFixtureWith(name, edits));
    if (expect === "report" && failures.length === 0) throw new Error(`[verify interactivity] Puzzle fill trace self-test ${name} was falsely accepted.`);
    if (expect === "silent" && failures.length > 0) throw new Error(`[verify interactivity] Puzzle fill trace self-test ${name} was falsely reported: ${failures.join("; ")}`);
  }
  return CASES.length + 1;
}
