import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

type Entry = "progress" | "checkpoint" | "yield" | "complete" | "cancelled" | Readonly<{ fault: string }>;
type Refusal = Readonly<{ code: string; message?: string }>;
type Verdict = Readonly<{ steps: number } | { fault: Refusal }>;
type Fixture = Readonly<{ why: string; budget: Readonly<{ wallMicros: number; turns: number }>; cases: Readonly<{ name: string; clockMicrosPerRead: number; cancelled?: boolean; script: Entry[]; verdict: Verdict }>[] }>;

const REDUCER_PREFIX = "retained command reducer rejected operation: ";
const ajv = new Ajv({ strict: true, allErrors: true });

/** 🧪️ One JSON Schema compiled by AJV as a plain predicate — a match decides a branch, it never narrows the value's static type. */
function schemaKind(schema: Readonly<Record<string, unknown>>): (value: unknown) => boolean {
  const validate = ajv.compile(schema);
  return (value) => validate(value);
}

/** 📜️ Every outcome kind of the verdict as one JSON Schema over a script entry or a fault detail, evaluated by AJV — the third-party engine. */
const kinds = {
  complete: schemaKind({ const: "complete" }),
  cancelled: schemaKind({ const: "cancelled" }),
  fault: schemaKind({ type: "object", required: ["fault"], properties: { fault: { type: "string" } } }),
  framed: schemaKind({ type: "string", pattern: "^[^\\u001f]+\\u001f" }),
  reducer: schemaKind({ type: "string", pattern: "^retained command reducer rejected operation: [^ ]+ " }),
};

/** 🧯️ What the shell's fault page carries for one job fault detail, then the reducer's own code when the reducer wrote it. */
function refusal(detail: string): Refusal {
  const at = detail.indexOf("\u001f");
  const [code, message] = kinds.framed(detail) ? [detail.slice(0, at), detail.slice(at + 1)] : ["interactive-job.app-owned-output", at === 0 ? detail.slice(1) : detail];
  if (!kinds.reducer(message)) return { code, message };
  const rest = message.slice(REDUCER_PREFIX.length);
  const space = rest.indexOf(" ");
  return { code: rest.slice(0, space), message: rest.slice(space + 1) };
}

/** 🔍️ The verdict of one script: its first terminal entry, or the budget refusal when it has none. */
function verdict(script: Entry[], cancelled: boolean): Verdict {
  if (cancelled) return { fault: { code: "interactive-job.cancelled" } };
  const index = script.findIndex((entry) => kinds.complete(entry) || kinds.cancelled(entry) || kinds.fault(entry));
  if (index < 0) return { fault: { code: "interactive-job.preview-budget" } };
  const entry = script[index]!;
  if (kinds.complete(entry)) return { steps: index + 1 };
  if (kinds.cancelled(entry)) return { fault: { code: "interactive-job.cancelled" } };
  return { fault: refusal((entry as Readonly<{ fault: string }>).fault) };
}

/** ⚖️ The AJV twin of the Rust law `app::agent_lane_preview_tests`: every case of the language-agnostic agent-lane preview fixture answered exactly as written. */
export function agentLanePreviewVerdictOracle(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🤖️agent-lane-preview-verdicts.json", import.meta.url), "utf8")) as Fixture;
  assert(fixture.cases.length >= 9, "the fixture keeps every verdict's case");
  for (const { name, script, cancelled, verdict: expected } of fixture.cases) {
    const derived = verdict(script, cancelled === true);
    if ("steps" in expected) assert.deepEqual(derived, expected, name);
    else {
      assert("fault" in derived, `${name}: expected a refusal`);
      assert.equal(derived.fault.code, expected.fault.code, name);
      if (expected.fault.message !== undefined) assert.equal(derived.fault.message, expected.fault.message, name);
    }
  }
  return fixture.cases.length;
}

type Publication = Readonly<{ carriedOps: number; effects: unknown[]; windowConfigOps: number; extensionCalls: number; events: number; selectionWrites: number; tasks: number; presence: number; transient: number; windowTransient: number }>;
type Carriage = Readonly<{ why: string; lanes: string[]; presentation: string[]; interactionVerbs: string[]; cases: Readonly<{ name: string; publication: Publication; uncarried: string[]; verdict: "carried" | "noEffect" | "uncarried" }>[] }>;

/** 🏷️ One host-effect lane as a JSON Schema over the kernel `Effect`'s camelCase wire form: an object whose one key is a listed variant. */
const tagged = (...tags: string[]) => schemaKind({ type: "object", minProperties: 1, maxProperties: 1, propertyNames: { enum: tags } });

/** 🚚️ Every named host-effect lane; an effect none of them matches is a host request. */
const effectLanes: Readonly<Record<string, (value: unknown) => boolean>> = {
  "a whole-document replacement": tagged("loadDocument"),
  "a file download": tagged("downloadMediaExport", "iconRenderExport", "videoRenderExport"),
  "a file request": tagged("requestFileOpen", "requestMediaFrames"),
  "a follow-up dispatch": tagged("dispatchAction"),
  "a dialog": tagged("openDialog"),
  "a notice": tagged("notify"),
  "a clipboard write": tagged("clipboardWrite"),
  "a shell change": tagged("openWindow", "closeWindow", "setPanel", "navigate", "openExternalUrl", "spawnPluginInstance", "openPluginInstance", "setActiveUtility", "setActiveTool", "replayShellCommand"),
  "extension calls": tagged("invokeExtension"),
};

/** 🔢️ Every counted lane and the publication field that sizes it. */
const countedLanes: Readonly<Record<string, keyof Publication>> = {
  "extension calls": "extensionCalls",
  "app events": "events",
  "selection changes": "selectionWrites",
  "window configuration changes": "windowConfigOps",
  "follow-up tasks": "tasks",
  "presence changes": "presence",
  "transient state changes": "transient",
  "window transient state changes": "windowTransient",
};

const nonEmpty = schemaKind({ type: "integer", minimum: 1 });

/** 🕹️ A guest-emitted interaction verb: a dispatch-action or replay-shell-command effect naming one of `verbs`. */
const interactionVerb = (verbs: readonly string[]) =>
  schemaKind({
    type: "object",
    minProperties: 1,
    maxProperties: 1,
    anyOf: [
      { required: ["dispatchAction"], properties: { dispatchAction: { type: "object", required: ["action"], properties: { action: { enum: [...verbs] } } } } },
      { required: ["replayShellCommand"], properties: { replayShellCommand: { type: "object", required: ["actionId"], properties: { actionId: { enum: [...verbs] } } } } },
    ],
  });

/** 🚧️ The lanes of one publication an agent transaction cannot carry, each once, in the fixture's lane order; presentation lanes beside carried operations are omitted. */
function uncarried(lanes: readonly string[], presentation: readonly string[], selects: (value: unknown) => boolean, publication: Publication): string[] {
  const found = new Set<string>();
  for (const effect of publication.effects) found.add(selects(effect) ? "selection changes" : (Object.keys(effectLanes).find((lane) => effectLanes[lane]!(effect)) ?? "a host request"));
  for (const [lane, key] of Object.entries(countedLanes)) if (nonEmpty(publication[key])) found.add(lane);
  const carries = nonEmpty(publication.carriedOps);
  return lanes.filter((lane) => found.has(lane) && !(carries && presentation.includes(lane)));
}

/** ⚖️ The AJV twin of the Rust law `app::agent_lane_preview_tests::agent_lane_carriage_matches_the_language_agnostic_fixture`: every case of the language-agnostic carriage fixture answered exactly as written. */
export function agentLaneCarriageOracle(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🤖️agent-lane-carriage.json", import.meta.url), "utf8")) as Carriage;
  assert(fixture.cases.length >= 15, "the fixture keeps every lane family's case");
  const selects = interactionVerb(fixture.interactionVerbs);
  assert.equal(new Set(fixture.lanes).size, fixture.lanes.length, "every lane is named once");
  assert(fixture.presentation.every((lane) => fixture.lanes.includes(lane)), "every presentation lane is a named lane");
  for (const { name, publication, uncarried: expected, verdict } of fixture.cases) {
    const lanes = uncarried(fixture.lanes, fixture.presentation, selects, publication);
    assert.deepEqual(lanes, expected, name);
    const derived = lanes.length === 0 ? "carried" : !nonEmpty(publication.carriedOps) && lanes.every((lane) => fixture.presentation.includes(lane)) ? "noEffect" : "uncarried";
    assert.equal(derived, verdict, name);
  }
  return fixture.cases.length;
}

if (import.meta.main) console.log(`agent-lane-preview-verdict-oracle cases=${agentLanePreviewVerdictOracle()} agent-lane-carriage-oracle cases=${agentLaneCarriageOracle()}`);
