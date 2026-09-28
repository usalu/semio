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

if (import.meta.main) console.log(`agent-lane-preview-verdict-oracle cases=${agentLanePreviewVerdictOracle()}`);
