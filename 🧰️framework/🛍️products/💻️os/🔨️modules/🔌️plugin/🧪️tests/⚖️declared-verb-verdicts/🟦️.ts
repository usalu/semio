import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

type Effect = Readonly<{ lanes: string[]; documentChanged: boolean; documentReplaced: boolean; configChanged: boolean; userPathWritten: boolean; hostEffects: number; fingerprint: number; orphanedChildren?: string[] }>;
type Outcome = Readonly<{ settled: Effect } | { refused: { code: string; detail: string } } | { unreachable: { code: string; detail: string } }>;
type Argument = Readonly<{ argument: string; othersSpecified: boolean; first: Outcome; second: Outcome }>;
type Probe = Readonly<{ verb: string; kind: string; audience: string; destructive: boolean; bridge: string | null; windows: Readonly<{ window: string; staged: Outcome; arguments: Argument[] }>[]; agent: Outcome | null }>;
type Finding = Readonly<{ finding: string; argument?: string }>;
type Fixture = Readonly<{ why: string; cases: Readonly<{ name: string; probe: Probe; findings: Finding[] }>[] }>;

const ajv = new Ajv({ strict: true, allErrors: true, $data: true });

const touches = { type: "object", anyOf: [{ required: ["documentChanged"], properties: { documentChanged: { const: true } } }, { required: ["documentReplaced"], properties: { documentReplaced: { const: true } } }, { required: ["lanes"], properties: { lanes: { type: "array", contains: { enum: ["artifact", "child"] } } } }] };
const silent = {
  type: "object",
  required: ["configChanged", "userPathWritten", "hostEffects", "lanes"],
  not: touches,
  properties: { configChanged: { const: false }, userPathWritten: { const: false }, hostEffects: { const: 0 }, lanes: { type: "array", items: { enum: ["terminal", "ui"] } } },
};
const settled = (effect: object) => ({ type: "object", required: ["settled"], properties: { settled: effect } });
const refused = { type: "object", required: ["refused"], properties: { refused: { type: "object" } } };
const unreachable = { type: "object", required: ["unreachable"], properties: { unreachable: { type: "object" } } };
const refusedUnreachableOrSilent = { anyOf: [refused, unreachable, settled(silent)] };

/** 📜️ Every rule of the verdict as one JSON Schema over the probe's flattened view, evaluated by AJV — the third-party engine. */
const rules = {
  unbridged: ajv.compile({ type: "object", required: ["bridge", "audience"], properties: { bridge: { type: "string" }, audience: { not: { const: "input" } } } }),
  bridged: ajv.compile({ type: "object", required: ["bridge"], properties: { bridge: { type: "null" } } }),
  unreachable: ajv.compile({ type: "object", required: ["staged"], properties: { staged: { type: "array", minItems: 1, items: unreachable } } }),
  documentWriteFromNonMutation: ajv.compile({ type: "object", required: ["kind", "effects"], properties: { kind: { enum: ["view", "shell"] }, effects: { type: "array", contains: touches } } }),
  silentMutation: ajv.compile({ type: "object", required: ["kind", "audience", "outcomes"], properties: { kind: { const: "mutation" }, audience: { const: "agent" }, outcomes: { type: "array", items: settled(silent) } } }),
  destructiveWithoutDiscard: ajv.compile({
    type: "object",
    required: ["destructive", "effects"],
    properties: { destructive: { const: true }, effects: { type: "array", minItems: 1, items: { type: "object", not: { anyOf: [touches, { type: "object", required: ["userPathWritten"], properties: { userPathWritten: { const: true } } }] } } } },
  }),
  composedChildOrphaned: ajv.compile({ type: "object", required: ["effects"], properties: { effects: { type: "array", contains: { type: "object", required: ["orphanedChildren"], properties: { orphanedChildren: { type: "array", minItems: 1 } } } } } }),
  agentLaneDiverges: ajv.compile({
    type: "object",
    required: ["agent", "staged"],
    properties: { agent: { type: "object" }, staged: { type: "array" } },
    anyOf: [
      { oneOf: [{ type: "object", properties: { staged: { type: "array", contains: settled(touches) } } }, { type: "object", properties: { agent: settled(touches) } }] },
      { type: "object", properties: { agent: refusedUnreachableOrSilent, staged: { type: "array", contains: settled({ type: "object", not: silent }) } } },
    ],
  }),
  argumentAudience: ajv.compile({ type: "object", required: ["audience"], properties: { audience: { not: { const: "input" } } } }),
  ignoredArgument: ajv.compile({
    type: "array",
    minItems: 1,
    items: {
      type: "object",
      anyOf: [
        { type: "object", required: ["first", "second"], properties: { first: settled({ type: "object" }), second: { const: { $data: "1/first" } } } },
        { type: "object", required: ["first", "second", "staged", "othersSpecified"], properties: { first: refused, second: { const: { $data: "1/first" } }, staged: { const: { $data: "1/first" } }, othersSpecified: { const: true } } },
      ],
    },
  }),
};

/** 🧮️ An outcome in the Rust verdict's own equality: lanes are a set, a missing orphan list is empty. */
const normal = (outcome: Outcome): Outcome => ("settled" in outcome ? { settled: { ...outcome.settled, lanes: [...new Set(outcome.settled.lanes)].sort(), orphanedChildren: outcome.settled.orphanedChildren ?? [] } } : outcome);

/** 🔍️ The findings of one probe, in the Rust verdict's order: an unbridged or unreachable verb reports that alone. */
function findings(probe: Probe): Finding[] {
  const staged = probe.windows.map((window) => normal(window.staged));
  const outcomes = probe.windows.flatMap((window) => [window.staged, ...window.arguments.flatMap((argument) => [argument.first, argument.second])]).map(normal);
  const effects = outcomes.flatMap((outcome) => ("settled" in outcome ? [outcome.settled] : []));
  const view = { kind: probe.kind, audience: probe.audience, destructive: probe.destructive, bridge: probe.bridge, staged, outcomes, effects, ...(probe.agent === null ? {} : { agent: normal(probe.agent) }) };
  if (!rules.bridged(view)) return rules.unbridged(view) ? [{ finding: "unbridged" }] : [];
  if (rules.unreachable(view)) return [{ finding: "unreachable" }];
  const found: Finding[] = (["documentWriteFromNonMutation", "silentMutation", "destructiveWithoutDiscard", "composedChildOrphaned", "agentLaneDiverges"] as const).filter((rule) => rules[rule](view)).map((rule) => ({ finding: rule }));
  if (!rules.argumentAudience(view)) return found;
  const names = [...new Set(probe.windows.flatMap((window) => window.arguments.map((argument) => argument.argument)))];
  for (const name of names) {
    const rows = probe.windows.flatMap((window) => window.arguments.filter((argument) => argument.argument === name).map((argument) => ({ staged: normal(window.staged), first: normal(argument.first), second: normal(argument.second), othersSpecified: argument.othersSpecified })));
    if (rules.ignoredArgument(rows)) found.push({ finding: "ignoredArgument", argument: name });
  }
  return found;
}

/** ⚖️ The AJV twin of `artifact_app_laws::declared_verb_findings`: every case of the language-agnostic verdict fixture answered exactly as written. */
export function declaredVerbVerdictOracle(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/⚖️declared-verb-verdicts.json", import.meta.url), "utf8")) as Fixture;
  assert(fixture.cases.length >= 31, "the fixture keeps every rule's positive and negative case");
  for (const { name, probe, findings: expected } of fixture.cases) assert.deepEqual(findings(probe), expected.map((finding) => (finding.argument === undefined ? { finding: finding.finding } : finding)), name);
  return fixture.cases.length;
}

/** 🧬️ AJV independently admits every declared representative and refuses every invalid neutral payload. */
export function declaredBridgeArgumentOracle(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎮️declared-bridge-arguments.json", import.meta.url), "utf8")) as { cases: { name: string; expected?: unknown; valueSchema?: object; invalid?: unknown[]; errorArgument?: string }[] };
  const validator = new Ajv({ strict: true, allErrors: true });
  let count = 0;
  for (const row of fixture.cases) {
    if (row.errorArgument !== undefined) continue;
    assert(row.valueSchema !== undefined && row.invalid !== undefined, row.name);
    const validate = validator.compile(row.valueSchema);
    assert.equal(validate(row.expected), true, `${row.name}: ${validator.errorsText(validate.errors)}`);
    for (const invalid of row.invalid) assert.equal(validate(invalid), false, `${row.name} rejects ${JSON.stringify(invalid)}`);
    count += 1 + row.invalid.length;
  }
  assert.equal(count, 38, "every authored representative and negative payload reaches AJV");
  return count;
}

if (import.meta.main) console.log(`declared-verb-verdict-oracle cases=${declaredVerbVerdictOracle()}`);
