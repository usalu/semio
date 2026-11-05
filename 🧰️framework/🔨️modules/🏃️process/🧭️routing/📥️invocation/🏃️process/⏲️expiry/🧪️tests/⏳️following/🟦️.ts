import { expect, spyOn, test } from "bun:test";
import { Database } from "bun:sqlite";
import { declareBuildTestEnvelopeV1, withBuildTestInvocationV1 } from "../../../../../../🧪️testing/📥️invocation/🟦️.ts";
import { join } from "node:path";
import Ajv from "ajv";
import { createIssuedResourceControlV1, createRenewableIssuedResourceControlV1, createRequestedIssuedResourceControlV1, scriptResourceProcessEnvironmentV1, SCRIPT_PROCESS_INVOCATION_ENV } from "../../../🟦️.ts";
import { noteScriptProcessActivityV1 } from "../../../🐕️watchdog/🟦️.ts";
import { runOwnedCommand } from "../../../../../../🎛️owned-execution/🟦️.ts";
import corpus from "./🔣️.json";
declareBuildTestEnvelopeV1({ owner: "test:deadline-following", maximumElapsedMilliseconds: 300000, capabilities: {} });
const within = <T>(run: (control: ReturnType<typeof createIssuedResourceControlV1>) => Promise<T>, maximum = 60000) => withBuildTestInvocationV1(invocation => run(createIssuedResourceControlV1(invocation, "fixture:deadline-following", maximum)));
const wire = (environment: Record<string, string | undefined>) => JSON.parse(environment[SCRIPT_PROCESS_INVOCATION_ENV]!) as { deadlineEpochMilliseconds: number; policy: { maximumElapsedMilliseconds: number } };
const pause = (milliseconds: number) => new Promise<void>(accept => setTimeout(accept, milliseconds));

/** 🧪️ Runs the neutral timeline under genuine issuance while an independent SQL oracle computes its bounds. */
async function timeline(run: (base: ReturnType<typeof createIssuedResourceControlV1>, advance: (elapsed: number) => void, oracle: Database) => Promise<void>, maximum = 60000) {
  const epoch = Date.now(), monotonic = performance.now(), oracle = new Database(":memory:");
  let elapsed = 0;
  const date = spyOn(Date, "now").mockImplementation(() => epoch + elapsed);
  const clock = spyOn(performance, "now").mockImplementation(() => monotonic + elapsed);
  try {
    await within(base => run(base, value => { elapsed = value; noteScriptProcessActivityV1(); }, oracle), maximum);
  } finally {
    date.mockRestore();
    clock.mockRestore();
    oracle.close();
  }
}

test("neutral progress keeps an inherited zero loan live beyond the initial idle window", async () => {
  expect(new Ajv().validate({ type: "object", required: ["version", "idleMilliseconds", "maximumMilliseconds", "pulses"], properties: { version: { const: 1 }, idleMilliseconds: { type: "integer", minimum: 1 }, maximumMilliseconds: { type: "integer", minimum: 1 }, pulses: { type: "array", items: { type: "integer", minimum: 0 } } } }, corpus)).toBe(true);
  await timeline(async (base, advance, oracle) => {
    noteScriptProcessActivityV1();
    const parent = createRenewableIssuedResourceControlV1(base, corpus, 1), inherited = createRequestedIssuedResourceControlV1(parent, 0);
    for (const elapsed of corpus.pulses) {
      advance(elapsed);
      const expected = oracle.query("SELECT min(?1 - ?2, ?3) AS remaining").get(corpus.maximumMilliseconds, elapsed, corpus.idleMilliseconds) as { remaining: number };
      expect(parent.remainingMilliseconds()!).toBeCloseTo(expected.remaining, 3);
      expect(inherited.remainingMilliseconds()!).toBeCloseTo(expected.remaining, 3);
    }
  });
});

test("progress projection preserves the immutable maximum across distinct child births", async () => {
  await timeline(async (base, advance, oracle) => {
    noteScriptProcessActivityV1();
    const parent = createRenewableIssuedResourceControlV1(base, corpus, 1), first = wire(scriptResourceProcessEnvironmentV1(parent, {}));
    for (const elapsed of corpus.pulses) {
      advance(elapsed);
      expect(parent.remainingMilliseconds()!).toBeGreaterThan(0);
    }
    const next = wire(scriptResourceProcessEnvironmentV1(parent, {}));
    expect((oracle.query("SELECT ?1 > ?2 AS live, ?1 <= ?3 AS bounded").get(next.deadlineEpochMilliseconds, Date.now(), first.deadlineEpochMilliseconds) as { live: number; bounded: number })).toEqual({ live: 1, bounded: 1 });
    expect(next.policy.maximumElapsedMilliseconds).toBe(corpus.maximumMilliseconds);
  });
});

test("progress cannot renew a finite Original loan or an explicit descendant maximum", async () => {
  await timeline(async (base, advance) => {
    noteScriptProcessActivityV1();
    const parent = createRenewableIssuedResourceControlV1(base, corpus, 1), inherited = createRequestedIssuedResourceControlV1(parent, 0), requested = createRequestedIssuedResourceControlV1(parent, corpus.requestedMilliseconds);
    advance(corpus.requestedMilliseconds + 1);
    expect(parent.remainingMilliseconds()!).toBeGreaterThan(0);
    expect(() => requested.remainingMilliseconds()).toThrow(/deadline/iu);
    advance(corpus.finiteOriginalMilliseconds + 1);
    expect(() => inherited.remainingMilliseconds()).toThrow(/deadline/iu);
    expect(() => scriptResourceProcessEnvironmentV1(parent, {})).toThrow(/deadline/iu);
  }, corpus.finiteOriginalMilliseconds);
});

test("a genuine inherited abort retains its exact reason", async () => {
  await within(async base => {
    const abort = new AbortController(), parent = createRenewableIssuedResourceControlV1(base, corpus, 1), inherited = createRequestedIssuedResourceControlV1(parent, 0, abort.signal), marker = Error(corpus.abortMessage);
    abort.abort(marker);
    let failure: unknown;
    try { inherited.remainingMilliseconds(); } catch (error) { failure = error; }
    expect(failure).toBe(marker);
    failure = undefined;
    try { scriptResourceProcessEnvironmentV1(inherited, {}); } catch (error) { failure = error; }
    expect(failure).toBe(marker);
  });
});

test("a real child's admitted deadline is not renewed by parent reprojection", async () => {
  await within(async base => {
    const row = corpus.realChild;
    noteScriptProcessActivityV1();
    const parent = createRenewableIssuedResourceControlV1(base, row, 1), inherited = createRequestedIssuedResourceControlV1(parent, 0), environment = scriptResourceProcessEnvironmentV1(inherited, {}), first = wire(environment);
    let stdout = "";
    const child = runOwnedCommand(process.execPath, [join(import.meta.dirname, "👷️child/📜️script.ts")], process.cwd(), "fixture:deadline-child", 0, { control: inherited, signal: inherited.signal, env: environment, stdout: "ignore", output: { maximumBytes: 65536, maximumLines: 256 }, outputConsumer: (bytes, stream) => { if (stream === "stdout") stdout += Buffer.from(bytes).toString("utf8"); } });
    let projected: ReturnType<typeof wire> | undefined, failure: unknown;
    try { await pause(row.reprojectAfterMilliseconds); projected = wire(scriptResourceProcessEnvironmentV1(parent, {})); } catch (error) { failure = error; }
    await child;
    expect(failure).toBeUndefined();
    expect(projected!.deadlineEpochMilliseconds).toBeLessThanOrEqual(first.deadlineEpochMilliseconds);
    expect(stdout.split(/\r?\n/u).filter(line => line.startsWith("issued-deadline:"))).toEqual(["issued-deadline:"+first.deadlineEpochMilliseconds, "issued-deadline:"+first.deadlineEpochMilliseconds]);
  });
}, 30000);
