import assert from "node:assert/strict";
import Ajv from "ajv/dist/2020";
import policySchema from "../../📋️policy/⏯️tool-run/🧬️schema/🔣️.json";
import {
  interactivityToolRunDeclarationFailures,
  interactivityToolRunLegacyTraceFailures,
  interactivityToolRunLocalLifecycleFailures,
  interactivityToolRunReservedActionFailures,
  type InteractivityToolRunFinding,
  type InteractivityToolRunRequirement,
  type InteractivityToolRunSource,
} from "../../📋️policy/⏯️tool-run/🟦️.ts";

const ROOT = "✏️s/🔌️plugins/🧪️sweep/🗿️artifacts/🧪️sweep/🏅️standards/🔖️1/🪆️subsets/✳️any";
const TOOL = `${ROOT}/✏️editor/🎭️modes/✏️edit/🛠️tools/🧹️sweep/🦀️.rs`;
const COMMAND = `${ROOT}/✏️editor/🎮️commands/🧹️sweep-step/🦀️.rs`;
const EDITOR = `${ROOT}/✏️editor/🦀️.rs`;
const UTILITY = `${ROOT}/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🧲️gather/🦀️.rs`;
const PANEL = `${ROOT}/✏️editor/🟦️.ts`;
const OTHER = "✏️s/🔌️plugins/🧪️spin/🗿️artifacts/🧪️spin/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🌀️spin/🦀️.rs";
const LEGACY = "✏️s/🔌️plugins/🧪️legacy/🗿️artifacts/🧪️legacy/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐢️legacy-build/🦀️.rs";
const HOST = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx";
const FRAMEWORK = "🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs";

/** 🧫️ A contract-conforming miniature repo: a run-declaring tool and utility, a discovered run tool, an undeclared legacy command, framework constants and a trace host. */
const CLEAN: Readonly<Record<string, string>> = {
  [TOOL]: `//! 🧹️ Sweep tool. Prose such as FILL_TRIED_RING never counts.
use semio_framework_plugin::{LocalizedLabel, ToolDefinition, ToolRunDefinition, WindowMeasure};

pub const TOOL_ID: &str = "sweep";

pub fn definition(label: LocalizedLabel, run: ToolRunDefinition) -> ToolDefinition {
    ToolDefinition { run: Some(run), ..semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, label, "brush")) }
}

pub fn count_measure(value: f64) -> WindowMeasure {
    WindowMeasure::Number { id: "sweep-count".into(), value, on_change: sweep_action("setSweepCount", None), ..Default::default() }
}
`,
  [COMMAND]: `//! 🧹️ \`sweep-step\` command.
use semio_framework_plugin::{Effect, Emit, TOOL_RUN_START_ACTION_ID};

pub fn sweep_step(ctx: &mut SweepCtx<'_>) -> Emit {
    Emit::mutations(ctx.take_ops())
}

pub fn start(ctx: &mut SweepCtx<'_>) {
    ctx.effects.push(Effect::DispatchAction { action: TOOL_RUN_START_ACTION_ID.into(), args: None, delay_ms: 0 });
}
`,
  [EDITOR]: `//! 🧹️ Sweep editor.
pub fn publish(ops: Vec<SweepMutation>) -> Emit {
    Emit::mutations(ops)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_only_reserved_literals_never_count() {
        let _ = ActionDefinition::new("toolRunAbort", label(), ActionKind::View, "square");
    }
}
`,
  [UTILITY]: `//! 🧲️ Gather utility.
pub fn definition(label: LocalizedLabel, run: ToolRunDefinition) -> UtilityDefinition {
    let mut definition = UtilityDefinition::new("gather", label, "magnet");
    definition.run = Some(run);
    definition
}
`,
  [PANEL]: `/** 🧹️ Sweep panel actions. */
export const SWEEP_ACTIONS = ["setSweepCount"] as const;
`,
  [OTHER]: `//! 🌀️ Spin tool, run-declaring but absent from the requirement table.
pub fn definition(label: LocalizedLabel, run: ToolRunDefinition) -> ToolDefinition {
    ToolDefinition { run: Some(run), ..ToolDefinition::new("spin", label, "rotate") }
}
`,
  [LEGACY]: `//! 🐢️ An undeclared, unlisted command keeps its own verbs until its lane migrates.
pub fn legacy_build(ops: Vec<LegacyMutation>) -> Emit {
    let _ = "cancelLegacyBuild";
    Emit::mutations(ops)
}
`,
  [HOST]: `/** 🌐️ World host; fillBuildPreview in prose never counts. */
export function Host({ pages }: { readonly pages: readonly Uint8Array[] }) {
  return <ToolRunTraceLayer pages={pages} />;
}
`,
  [FRAMEWORK]: `//! ⏯️ Framework-owned reserved ids.
pub const TOOL_RUN_START_ACTION_ID: &str = "toolRunStart";
pub fn is_tool_run_action_id(action: &str) -> bool {
    matches!(action, TOOL_RUN_START_ACTION_ID)
}
`,
};

const ROWS: readonly InteractivityToolRunRequirement[] = [
  { toolId: "sweep", root: ROOT, scope: ["✏️editor/🎭️modes/✏️edit/🛠️tools/🧹️sweep", "✏️editor/🎮️commands/🧹️sweep-step"], verbs: ["cancelSweep", "sweep_tick"], measures: ["cancel_measure"], lane: "W9-Z", inventory: "§0" },
  { toolId: "gather", root: ROOT, scope: ["✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🧲️gather"], verbs: [], measures: [], lane: "W9-Z", inventory: "§0" },
];

type Predicate = "lifecycle" | "legacy" | "declaration" | "reserved";

type Case = { readonly name: string; readonly predicate: Predicate; readonly expect: "report" | "silent"; readonly file?: string; readonly edits: readonly (readonly [path: string, from: string, to: string])[]; readonly rows?: (rows: readonly InteractivityToolRunRequirement[]) => readonly InteractivityToolRunRequirement[] };

const DROP_SWEEP_RUN = [TOOL, "ToolDefinition { run: Some(run), ..", "ToolDefinition { .."] as const;

const CASES: readonly Case[] = [
  { name: "row-verb", predicate: "lifecycle", expect: "report", file: EDITOR, edits: [[EDITOR, "#[cfg(test)]", 'pub fn cancel_sweep() -> &\'static str { "cancelSweep" }\n\n#[cfg(test)]']] },
  { name: "row-measure", predicate: "lifecycle", expect: "report", file: TOOL, edits: [[TOOL, "pub fn count_measure", "pub fn cancel_measure() -> Option<WindowMeasure> { None }\n\npub fn count_measure"]] },
  { name: "generic-cancel-action", predicate: "lifecycle", expect: "report", file: TOOL, edits: [[TOOL, 'sweep_action("setSweepCount", None)', 'sweep_action("abortSweep", None)']] },
  { name: "generic-tick-verb", predicate: "lifecycle", expect: "report", file: UTILITY, edits: [[UTILITY, "    definition\n}", '    definition\n}\n\nconst STEP: &str = "gatherTick";']] },
  { name: "generic-kebab-verb", predicate: "lifecycle", expect: "report", file: OTHER, edits: [[OTHER, "\n}\n", '\n}\n\nconst CANCEL: &str = "cancel-spin-build";\n']] },
  { name: "generic-progress-measure-fn", predicate: "lifecycle", expect: "report", file: OTHER, edits: [[OTHER, "\n}\n", "\n}\n\nfn progress_measure() {}\n"]] },
  { name: "undeclared-row-verbs-are-silent", predicate: "lifecycle", expect: "silent", edits: [DROP_SWEEP_RUN, [EDITOR, "#[cfg(test)]", 'pub fn cancel_sweep() -> &\'static str { "cancelSweep" }\n\n#[cfg(test)]']] },
  { name: "tried-ring", predicate: "legacy", expect: "report", file: FRAMEWORK, edits: [[FRAMEWORK, "pub fn is_tool_run_action_id", "pub const FILL_TRIED_RING: usize = 12;\npub fn is_tool_run_action_id"]] },
  { name: "preview-tail", predicate: "legacy", expect: "report", file: HOST, edits: [[HOST, "  return <ToolRunTraceLayer", "  const tail = pages.fillBuildPreview;\n  return <ToolRunTraceLayer"]] },
  { name: "preview-cap", predicate: "legacy", expect: "report", file: HOST, edits: [[HOST, "export function Host", "const WORLD_FILL_PREVIEW_JSON_MAX_BYTES = 16 * 1024;\nexport function Host"]] },
  { name: "tried-record", predicate: "legacy", expect: "report", file: TOOL, edits: [[TOOL, "pub const TOOL_ID", "struct FillTriedCandidate;\npub const TOOL_ID"]] },
  { name: "commented-legacy-is-silent", predicate: "legacy", expect: "silent", edits: [[FRAMEWORK, "pub fn is_tool_run_action_id", "/* FILL_TRIED_RING\n   fillBuildPreview */\npub fn is_tool_run_action_id"]] },
  { name: "missing-run", predicate: "declaration", expect: "report", file: TOOL, edits: [DROP_SWEEP_RUN] },
  { name: "wrong-tool-id", predicate: "declaration", expect: "report", file: ROOT, edits: [[TOOL, '"sweep";', '"sweeper";']] },
  { name: "missing-utility-run", predicate: "declaration", expect: "report", file: UTILITY, edits: [[UTILITY, "definition.run = Some(run);", "let _ = run;"]] },
  { name: "test-only-declaration", predicate: "declaration", expect: "report", file: ROOT, edits: [[TOOL, "pub fn definition(label: LocalizedLabel, run: ToolRunDefinition) -> ToolDefinition {\n    ToolDefinition { run: Some(run), ..semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, label, \"brush\")) }\n}", "#[cfg(test)]\nmod tests {\n    pub fn definition(label: LocalizedLabel, run: ToolRunDefinition) -> ToolDefinition {\n        ToolDefinition { run: Some(run), ..ToolDefinition::new(TOOL_ID, label, \"brush\") }\n    }\n}"]] },
  { name: "run-in-another-root-does-not-count", predicate: "declaration", expect: "report", file: TOOL, edits: [DROP_SWEEP_RUN, [OTHER, '"spin"', '"sweep"']] },
  { name: "stale-root", predicate: "declaration", expect: "report", file: `${ROOT}/🫥️missing`, edits: [], rows: (rows) => [rows[0]!, { ...rows[1]!, root: `${ROOT}/🫥️missing` }] },
  { name: "stale-scope", predicate: "declaration", expect: "report", file: ROOT, edits: [], rows: (rows) => [rows[0]!, { ...rows[1]!, scope: ["✏️editor/🫥️missing"] }] },
  { name: "duplicate-row", predicate: "declaration", expect: "report", file: ROOT, edits: [], rows: (rows) => [...rows, rows[0]!] },
  { name: "literal-reserved-definition", predicate: "reserved", expect: "report", file: EDITOR, edits: [[EDITOR, "#[cfg(test)]", 'pub fn declare() -> ActionDefinition { ActionDefinition::new("toolRunAbort", label(), ActionKind::View, "square") }\n\n#[cfg(test)]']] },
  { name: "constant-reserved-definition", predicate: "reserved", expect: "report", file: EDITOR, edits: [[EDITOR, "#[cfg(test)]", 'pub fn declare() -> ActionDefinition { ActionDefinition::new(TOOL_RUN_FINALIZE_ACTION_ID, label(), ActionKind::History, "check") }\n\n#[cfg(test)]']] },
  { name: "reserved-route-arm", predicate: "reserved", expect: "report", file: EDITOR, edits: [[EDITOR, "#[cfg(test)]", "pub fn route(action: &str) { match action { semio_framework::TOOL_RUN_ABORT_ACTION_ID => abort(), _ => {} } }\n\n#[cfg(test)]"]] },
  { name: "ts-reserved-literal", predicate: "reserved", expect: "report", file: PANEL, edits: [[PANEL, '["setSweepCount"]', '["setSweepCount", "toolRunStep"]']] },
  { name: "framework-reserved-literal-is-silent", predicate: "reserved", expect: "silent", edits: [[FRAMEWORK, '"toolRunStart";', '"toolRunStart";\npub const TOOL_RUN_ABORT_ACTION_ID: &str = "toolRunAbort";']] },
];

/** 🧪️ Runs one tool-run predicate. */
function runPredicate(predicate: Predicate, sources: readonly InteractivityToolRunSource[], rows: readonly InteractivityToolRunRequirement[]): readonly InteractivityToolRunFinding[] {
  switch (predicate) {
    case "lifecycle":
      return interactivityToolRunLocalLifecycleFailures(sources, rows);
    case "legacy":
      return interactivityToolRunLegacyTraceFailures(sources);
    case "declaration":
      return interactivityToolRunDeclarationFailures(sources, rows);
    case "reserved":
      return interactivityToolRunReservedActionFailures(sources);
  }
}

/** 🧬️ Validates actual policy inputs and outputs against their production contract. */
function run(predicate: Predicate, sources: readonly InteractivityToolRunSource[], rows: readonly InteractivityToolRunRequirement[]): readonly InteractivityToolRunFinding[] {
  const validator = new Ajv({strict:true}).addSchema(policySchema);
  const source = validator.getSchema(policySchema.$id+"#/$defs/Source")!;
  const requirement = validator.getSchema(policySchema.$id+"#/$defs/Requirement")!;
  const finding = validator.getSchema(policySchema.$id+"#/$defs/Finding")!;
  for(const value of sources) assert(source(value), JSON.stringify(source.errors));
  for(const value of rows) assert(requirement(value), JSON.stringify(requirement.errors));
  assert.equal(source({path:"owner/source",text:"",extra:true}),false);
  assert.equal(requirement({...rows[0],scope:[null]}),false);
  assert.equal(finding({file:"owner/source",line:-1,text:"invalid"}),false);
  const findings = runPredicate(predicate,sources,rows);
  for(const value of findings) assert(finding(value),JSON.stringify(finding.errors));
  return findings;
}

/** 🧪️ Mutation self-test of the four tool-run policy predicates: the clean miniature repo passes all of them, every planted violation is reported by its predicate at the planted file, and every tolerated variant stays silent. Returns the executed case count. */
export function interactivityToolRunPolicySelfTests(): number {
  const predicates: readonly Predicate[] = ["lifecycle", "legacy", "declaration", "reserved"];
  const clean = Object.entries(CLEAN).map(([path, text]) => ({ path, text }));
  for (const predicate of predicates) {
    const findings = run(predicate, clean, ROWS);
    if (findings.length > 0) throw new Error(`[verify interactivity] tool-run ${predicate} clean fixture was falsely rejected: ${findings.map((finding) => `${finding.file}:${finding.line} ${finding.text}`).join("; ")}`);
  }
  for (const entry of CASES) {
    const texts = { ...CLEAN };
    for (const [path, from, to] of entry.edits) {
      const text = texts[path]!;
      if (text.split(from).length !== 2) throw new Error(`[verify interactivity] tool-run self-test ${entry.name} anchor is missing or ambiguous in ${path}: ${from}`);
      texts[path] = text.replace(from, to);
    }
    const sources = Object.entries(texts).map(([path, text]) => ({ path, text }));
    const findings = run(entry.predicate, sources, entry.rows ? entry.rows(ROWS) : ROWS);
    if (entry.expect === "silent" && findings.length > 0) throw new Error(`[verify interactivity] tool-run self-test ${entry.name} was falsely reported: ${findings.map((finding) => finding.text).join("; ")}`);
    if (entry.expect === "report" && !findings.some((finding) => finding.file === entry.file)) throw new Error(`[verify interactivity] tool-run self-test ${entry.name} was falsely accepted (findings: ${findings.map((finding) => `${finding.file}: ${finding.text}`).join("; ") || "none"}).`);
  }
  return predicates.length + CASES.length;
}
