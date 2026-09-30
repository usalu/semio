/** ⏪️ TypeScript oracle of the plugin runtime's history-edit scenarios (`🧫️fixtures/🧫️time-travel/🔣️.json`), the twin of
 * `🧪️tests/🧪️time-travel/🦀️.rs`. Independent where it counts: Ajv (2020-12) validates the fixture against its schema, and
 * every scenario's verbs are replayed through the time-travel reducer's own TypeScript twin (`⏪️time-travel/🟦️.ts`,
 * itself checked against xstate and fast-check), with synthetic drafts and replay reports standing in for the store —
 * so the stage, accepted drafts, blocking and finalize refusal the Rust runtime reaches live are the reducer law's. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020";
import { applyTimeTravel, timeTravelFinalizeRefusal, timeTravelSession, type TimeTravelEvent, type TimeTravelSession } from "../../../../../../🔨️modules/⏪️time-travel/🟦️.ts";
import { replayReportBlocksFinalize, type InputReplacement, type ReplayReport } from "../../../../../../🔨️modules/📡️replication/🟦️.ts";

type Step = Readonly<Record<string, unknown>>;
type Scenario = Readonly<{ id: string; steps: readonly Step[]; status: Readonly<{ stage: string; blocking: boolean; acceptedCount: number }> | null; finalizeRefusal?: string | null }>;
type Fixture = Readonly<{ scenarios: readonly Scenario[]; refusals: Readonly<Record<string, unknown>> }>;

const PLUGIN_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";

function input(payload: string): InputReplacement {
  return { kind: "input", schema: "semio.testkit-time-travel/v1", payload: [...new TextEncoder().encode(payload)] };
}

function report(session: TimeTravelSession, verdict: string): ReplayReport {
  const outcomes = session.accepted.map((draft) => ({ mutationId: draft.target.mutation, editId: `e${draft.target.position}`, opIndex: 0, worst: verdict === "fatal" && draft.replacement.kind === "input" ? ("fatal" as const) : null, messages: [], superseded: true, withdrawn: draft.replacement.kind === "withdrawn" }));
  return { fromPosition: 0, outcomes, worst: outcomes.some((outcome) => outcome.worst === "fatal") ? "fatal" : null };
}

function apply(session: TimeTravelSession, event: TimeTravelEvent, scenario: string): TimeTravelSession {
  const result = applyTimeTravel(session, event);
  assert.ok(result.ok, `${scenario}: ${event.type} was refused: ${result.ok ? "" : result.rejection}`);
  return result.session;
}

/** ⏯️ Replays one fixture step through the reducer twin; a `replay` completes the running replay with a synthetic report. */
function step(session: TimeTravelSession, value: Step, scenario: string): TimeTravelSession {
  const [name, argument] = Object.entries(value)[0]!;
  const generation = session.generation;
  switch (name) {
    case "begin": {
      const position = argument as number;
      return apply(session, { type: "begin", target: { mutation: `m${position}`, position }, original: input(`original-${position}`) }, scenario);
    }
    case "input":
      return apply(session, { type: "draft", generation, replacement: input(JSON.stringify(argument)) }, scenario);
    case "withdraw":
    case "accept":
    case "discard":
      return apply(session, { type: name, generation }, scenario);
    case "replay":
      return session.stage === "replaying" ? apply(session, { type: "replayCompleted", generation, report: report(session, argument as string) }, scenario) : session;
    case "finalize":
      return apply(session, { type: "requestFinalize", generation }, scenario);
    case "commit": {
      const commit = argument as { choice?: string; name?: string };
      const chosen = apply(session, { type: "choose", generation, choice: commit.choice === "overwrite" ? { kind: "overwrite" } : { kind: "alternative", name: commit.name ?? "" } }, scenario);
      return apply(chosen, { type: "finalized", generation: chosen.generation }, scenario);
    }
    case "exit":
      return apply(session, { type: "exit" }, scenario);
    default:
      throw new Error(`${scenario}: unknown step ${name}`);
  }
}

/** ⚖️ Validates the fixture and replays every scenario through the reducer twin; answers the scenario count. */
export function timeTravelScenarioOracle(repoRoot: string): number {
  const fixtureRoot = join(repoRoot, PLUGIN_ROOT, "🧫️fixtures/🧫️time-travel");
  const fixture: Fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(fixtureRoot, "🧬️schema/🔣️.json"), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(validate({ ...fixture, scenarios: [{ ...fixture.scenarios[0], steps: [{ begin: 0, accept: null }] }] }), false, "a step names exactly one verb");
  assert.equal(validate({ ...fixture, refusals: { ...fixture.refusals, frozen: "timeTravel.busy" } }), false, "the frozen code is pinned");
  const base = { storeGeneration: 0n, contentRevision: new Uint8Array(32) };
  for (const scenario of fixture.scenarios) {
    let session = timeTravelSession(base);
    for (const value of scenario.steps) session = step(session, value, scenario.id);
    if (scenario.status === null) {
      assert.equal(session.stage, "inactive", `${scenario.id}: the session closed`);
      continue;
    }
    assert.equal(session.stage, scenario.status.stage, `${scenario.id}: stage`);
    assert.equal(session.accepted.length, scenario.status.acceptedCount, `${scenario.id}: accepted drafts`);
    assert.equal(session.report !== null && replayReportBlocksFinalize(session.report), scenario.status.blocking, `${scenario.id}: blocking`);
    if (scenario.finalizeRefusal !== undefined) assert.equal(timeTravelFinalizeRefusal(session), scenario.finalizeRefusal, `${scenario.id}: finalize refusal`);
  }
  return fixture.scenarios.length;
}
