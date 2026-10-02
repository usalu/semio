import { HISTORY_CLOSURE_RULES, historyClosureFindings, type HistoryClosureFinding, type HistoryClosureRule } from "../../../../../📜️script.ts";

/** 🧩️ Joins token fragments so this planted-violation file never carries a banned token itself. */
const t = (...parts: readonly string[]): string => parts.join("");

const AMEND = t("Emit::", "amend");
const AMEND_CONFIG = t("amend", "_config");
const AMEND_LAST = t("Amend", "Last");
const KEY = t("coalesce", "_key");
const KEY_TS = t("coalesce", "Key");
const PREVIEW = t("UtilityPreview", "Contract");
const BEGIN = t("transform", "Begin");
const STROKE_END = t("paintStroke", "End");
const HOST_SNAPSHOT = t("setHost", "Snapshot");
const EDIT = t("protocol::", "Edit {");
const INVERTIBLE = t("for_one_", "invertible_item");
const FOOTPRINT = t("ArtifactStoreOneItem", "Footprint {");

const ANY = "🏅️standards/🔖️1/🪆️subsets/✳️any";
const EDITOR = `✏️s/🔌️plugins/🧪️sweep/🗿️artifacts/🧪️sweep/${ANY}/✏️editor/🦀️.rs`;
const EDITOR_TEST = `✏️s/🔌️plugins/🧪️sweep/🗿️artifacts/🧪️sweep/${ANY}/✏️editor/🧪️tests/🔬️unit/🦀️.rs`;
const DESCRIPTOR = "🌎️hub/🧩️compositions/🧪️sweep/🔣️.json";
const STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs";
const TWIN = "🧰️framework/🛍️products/💻️os/🟦️.ts";
const FIXTURE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/📤️outbound-announcement/🔣️.json";
const ACTOR = "🧰️framework/🔨️modules/🎭️actor/🦀️.rs";
const NEGATIVE = "🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🖐️gumball-verb-audience.json";

/** 🧫️ A closure-conforming miniature repo: a plugin editor minting edits through the Store authority with a derived footprint, its test building an edit literally, the Store owning the footprint type, a TS twin, a fixture, an actor mailbox key and the bracket negative case. */
const CLEAN: Readonly<Record<string, string>> = {
  [EDITOR]: `//! 🧹️ Sweep editor; the transaction's open edit amends nothing but its own ticks.
pub fn sweep_edit(forward: SweepMutation, inverse: Vec<SweepMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<SweepMutation> {
    authority.next_edit(forward, inverse, description)
}

pub fn sweep_footprint(mutation: &SweepMutation, retained_bytes: usize) -> store::ArtifactStoreOneItemFootprint {
    store::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes)
}

pub fn sweep_drag(transaction: protocol::TransactionRef, ops: Vec<SweepMutation>) -> Emit {
    Emit::commit_transaction(transaction, ops)
}

pub fn sweep_camera(camera: SweepConfigMutation) -> Emit {
    Emit::config(vec![camera])
}
`,
  [EDITOR_TEST]: `//! 🧪️ Sweep tests.
fn edit() -> protocol::Edit<SweepMutation> {
    ${EDIT} line: None, id: "e".into(), ..fixture() }
}
`,
  [DESCRIPTOR]: `{ "actions": [{ "id": "translateSelection" }, { "id": "paintStroke" }] }
`,
  [STORE]: `//! 🏪️ Store.
pub struct ${FOOTPRINT} pub work_items: usize, pub retained_bytes: usize }
impl ArtifactStoreOneItemFootprint {
    pub fn ${INVERTIBLE}(retained_bytes: usize) -> Self { Self { work_items: 2, retained_bytes } }
}
pub fn empty() -> Edit<M> { Edit { line: None, id: String::new() } }
`,
  [TWIN]: `/** 🧵️ Channel twin. */
export type TransactionProposal = { readonly in_reply_to: number; readonly proposal_id: string; readonly description: string };
`,
  [FIXTURE]: `{ "edit": { "id": "edit-1", "transaction": { "id": "tx-1", "tool": "sweep#drag" } } }
`,
  [ACTOR]: `//! 🎭️ Actor mailbox.
pub struct CoalesceKey(pub String);
#[test]
fn pack_round_trip_${t("coalesce", "_key")}() {}
`,
  [NEGATIVE]: `{ "verbs": ["${BEGIN}", "${t("transform", "End")}"] }
`,
};

type Case = { readonly name: string; readonly expect: "report" | "silent"; readonly rule?: string; readonly file?: string; readonly edits: readonly (readonly [path: string, from: string, to: string])[]; readonly rules?: readonly HistoryClosureRule[] };

const CASES: readonly Case[] = [
  { name: "artifact-amend", expect: "report", rule: "amend-emit", file: EDITOR, edits: [[EDITOR, "Emit::commit_transaction(transaction, ops)", `${AMEND}(ops, "sweep")`]] },
  { name: "config-amend", expect: "report", rule: "amend-emit", file: EDITOR, edits: [[EDITOR, "Emit::config(vec![camera])", `Emit::${AMEND_CONFIG}(vec![camera], "camera")`]] },
  { name: "amend-in-a-doc-comment", expect: "report", rule: "amend-emit", file: EDITOR, edits: [[EDITOR, "//! 🧹️ Sweep editor;", `//! 🧹️ Sweep editor, formerly ${AMEND};`]] },
  { name: "transaction-prose-is-silent", expect: "silent", edits: [[EDITOR, "amends nothing but its own ticks", "amends its open edit (amended_same_edit)"]] },
  { name: "store-amend-command", expect: "report", rule: "amend-last", file: STORE, edits: [[STORE, "pub fn empty()", `pub enum Command { ${AMEND_LAST} { mutations: Vec<M> } }\npub fn empty()`]] },
  { name: "store-amend-in-lane", expect: "report", rule: "amend-last", file: STORE, edits: [[STORE, "pub fn empty()", `pub enum Command { ${AMEND_LAST}InLane { lane: HistoryLane } }\npub fn empty()`]] },
  { name: "ts-amend-command", expect: "report", rule: "amend-last", file: TWIN, edits: [[TWIN, "readonly description: string", `readonly description: string; readonly ${t("amend", "Last")}: boolean`]] },
  { name: "edit-key-literal", expect: "report", rule: "coalesce-key", file: EDITOR_TEST, edits: [[EDITOR_TEST, `line: None, id: "e".into(),`, `line: None, id: "e".into(), ${KEY}: None,`]] },
  { name: "twin-wire-key", expect: "report", rule: "coalesce-key", file: TWIN, edits: [[TWIN, "readonly description: string", `readonly description: string; readonly ${KEY}: string`]] },
  { name: "ts-camel-key", expect: "report", rule: "coalesce-key", file: TWIN, edits: [[TWIN, "readonly description: string", `readonly description: string; readonly ${KEY_TS}?: string`]] },
  { name: "fixture-key", expect: "report", rule: "coalesce-key", file: FIXTURE, edits: [[FIXTURE, `"id": "edit-1",`, `"id": "edit-1", "${KEY_TS}": "drag",`]] },
  { name: "emission-key", expect: "report", rule: "coalesce-key", file: EDITOR, edits: [[EDITOR, "Emit::config(vec![camera])", `Emit { config_mutations: vec![camera], ${KEY}: Some("camera".into()), ..Default::default() }`]] },
  { name: "actor-mailbox-key-is-silent", expect: "silent", edits: [[ACTOR, "pub struct CoalesceKey(pub String);", "pub struct CoalesceKey(pub String);\npub fn mailbox(key: CoalesceKey) -> CoalesceKey { key }"]] },
  { name: "preview-contract", expect: "report", rule: "preview-contract", file: EDITOR, edits: [[EDITOR, "//! 🧹️ Sweep editor;", `//! 🧹️ Sweep editor (see ${PREVIEW});`]] },
  { name: "descriptor-bracket", expect: "report", rule: "bracket-verb", file: DESCRIPTOR, edits: [[DESCRIPTOR, `{ "id": "paintStroke" }`, `{ "id": "${STROKE_END}" }`]] },
  { name: "descriptor-transform-bracket", expect: "report", rule: "bracket-verb", file: DESCRIPTOR, edits: [[DESCRIPTOR, `{ "id": "translateSelection" }`, `{ "id": "${BEGIN}" }`]] },
  { name: "negative-bracket-fixture-is-silent", expect: "silent", edits: [[NEGATIVE, `"verbs": [`, `"verbs": ["${STROKE_END}", `]] },
  { name: "host-snapshot-row", expect: "report", rule: "host-snapshot-bracket", file: EDITOR, edits: [[EDITOR, "pub fn sweep_camera", `const ROW: &str = "${HOST_SNAPSHOT}";\npub fn sweep_camera`]] },
  { name: "plugin-edit-literal", expect: "report", rule: "edit-literal", file: EDITOR, edits: [[EDITOR, "authority.next_edit(forward, inverse, description)", `${EDIT} line: None, id: "e".into(), forwards: vec![forward], inverse, description, ..Default::default() }`]] },
  { name: "imported-edit-literal", expect: "report", rule: "edit-literal", file: EDITOR, edits: [[EDITOR, "authority.next_edit(forward, inverse, description)", "Edit { line: authority.line_id().map(str::to_owned), forwards: vec![forward], inverse, description, ..Default::default() }"]] },
  { name: "store-and-test-edit-literals-are-silent", expect: "silent", edits: [[STORE, "pub fn empty()", "pub fn other() -> Edit<M> { Edit { line: Some(\"main\".into()), id: String::new() } }\npub fn empty()"]] },
  { name: "invertible-footprint", expect: "report", rule: "footprint-hand", file: EDITOR, edits: [[EDITOR, "store::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes)", `store::ArtifactStoreOneItemFootprint::${INVERTIBLE}(retained_bytes)`]] },
  { name: "counted-footprint", expect: "report", rule: "footprint-hand", file: EDITOR, edits: [[EDITOR, "store::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes)", `store::ArtifactStoreOneItemFootprint::${t("for_one", "_item")}(targets.len(), retained_bytes)`]] },
  { name: "literal-footprint", expect: "report", rule: "footprint-hand", file: EDITOR, edits: [[EDITOR, "store::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes)", `store::${FOOTPRINT} work_items: 1, retained_bytes }`]] },
  { name: "test-footprint-is-silent", expect: "silent", edits: [[EDITOR_TEST, "//! 🧪️ Sweep tests.", `//! 🧪️ Sweep tests.\nconst F: ArtifactStoreOneItemFootprint = ArtifactStoreOneItemFootprint::${INVERTIBLE}(8);`]] },
  {
    name: "declared-intent-function-is-silent",
    expect: "silent",
    edits: [[EDITOR, "pub fn sweep_camera", "pub fn sweep_replacement(snapshot: Sweep) -> SweepMutation {\n    SweepMutation::SetSnapshot(snapshot)\n}\n\npub fn sweep_camera"]],
    rules: [{ id: "set-snapshot", anchors: ["SetSnapshot"], pattern: /\bSetSnapshot\b/, applies: () => true, allow: [], functions: ["sweep_replacement"], reason: "planted" }],
  },
  {
    name: "undeclared-function-reports",
    expect: "report",
    rule: "set-snapshot",
    file: EDITOR,
    edits: [[EDITOR, "pub fn sweep_camera", "pub fn sweep_drop(snapshot: Sweep) -> SweepMutation {\n    SweepMutation::SetSnapshot(snapshot)\n}\n\npub fn sweep_camera"]],
    rules: [{ id: "set-snapshot", anchors: ["SetSnapshot"], pattern: /\bSetSnapshot\b/, applies: () => true, allow: [], functions: ["sweep_replacement"], reason: "planted" }],
  },
];

/** 🧾️ Renders findings for a failure message. */
function describe(findings: readonly HistoryClosureFinding[]): string {
  return findings.map((finding) => `${finding.file}:${finding.line} ${finding.text}`).join("; ") || "none";
}

/** 🧪️ Mutation self-test of the history closure gate: the clean miniature repo passes, every planted violation is reported by its rule at the planted file, and every tolerated variant stays silent. Returns the executed case count. */
export function historyClosurePolicySelfTests(): number {
  const clean = historyClosureFindings(Object.entries(CLEAN).map(([path, text]) => ({ path, text })));
  if (clean.length > 0) throw new Error(`[verify history-closure] clean fixture was falsely rejected: ${describe(clean)}`);
  for (const entry of CASES) {
    const texts = { ...CLEAN };
    for (const [path, from, to] of entry.edits) {
      const text = texts[path]!;
      if (text.split(from).length !== 2) throw new Error(`[verify history-closure] self-test ${entry.name} anchor is missing or ambiguous in ${path}: ${from}`);
      texts[path] = text.replace(from, to);
    }
    const findings = historyClosureFindings(Object.entries(texts).map(([path, text]) => ({ path, text })), entry.rules ?? HISTORY_CLOSURE_RULES);
    if (entry.expect === "silent" && findings.length > 0) throw new Error(`[verify history-closure] self-test ${entry.name} was falsely reported: ${describe(findings)}`);
    if (entry.expect === "report" && !findings.some((finding) => finding.file === entry.file && finding.rule === entry.rule)) throw new Error(`[verify history-closure] self-test ${entry.name} was falsely accepted (findings: ${describe(findings)}).`);
  }
  return CASES.length + 1;
}
