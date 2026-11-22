import {LANGUAGE_PROTOCOL_V1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import { afterAll, beforeAll, expect, spyOn, test } from "bun:test";
import { Database } from "bun:sqlite";
import progressSchema from "../../../🧬️schema/📈️progress.json";
import { createScriptProcessEnvelope, withScriptProcessEnvelope, createIssuedResourceControlV1, createRenewableIssuedResourceControlV1 } from "../../../🟦️.ts";
import { noteScriptProcessActivityV1, enterScriptProcessWaitV1, createScriptProcessActivitySamplerV1, scriptProcessActivityIdleMillisecondsV1 } from "../../../🐕️watchdog/🟦️.ts";
import { buildPreparationBudgetMs, progressDeadlinePolicyV1, ProgressDeadlineExpiredV1 } from "../../../../../../⏱️budget/🟦️.ts";
import budgets from "../../../../../../⏱️budget/🧪️tests/🧫️fixtures/🔣️.json";
import { runOwnedCommand } from "../../../../../../🎛️owned-execution/🟦️.ts";

const within = <T>(run: (control: ReturnType<typeof createIssuedResourceControlV1>, invocation: any) => Promise<T>) => withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "fixture:progress-deadline", maximumElapsedMilliseconds: 60000 }, {}, Date.now()), (invocation) => run(createIssuedResourceControlV1(invocation, "fixture:progress", 0), invocation));
const languageVariables = [LANGUAGE_PROTOCOL_V1.explicitVariable,LANGUAGE_PROTOCOL_V1.preferenceListVariable,...LANGUAGE_PROTOCOL_V1.environmentOrder], savedLanguage = languageVariables.map(name => [name, process.env[name]] as const);
beforeAll(() => { for (const name of languageVariables) delete process.env[name]; });
afterAll(() => { for (const [name, value] of savedLanguage) { if (value === undefined) delete process.env[name]; else process.env[name] = value; } });
const pause = (milliseconds: number) => new Promise<void>((accept) => setTimeout(accept, milliseconds));
const aborted = (signal: AbortSignal): ProgressDeadlineExpiredV1 => signal.reason as ProgressDeadlineExpiredV1;

test("a renewable control lives on observable work and ends after its idle window with a notice in both languages", async () => {
  await within(async (parent) => {
    const control = createRenewableIssuedResourceControlV1(parent, { idleMilliseconds: 300, maximumMilliseconds: 5000 }, 1);
    for (let step = 0; step < 7; step++) { await pause(100); noteScriptProcessActivityV1(); expect(control.signal.aborted).toBe(false); expect(control.remainingMilliseconds()!).toBeGreaterThan(0); }
    const lastWork = performance.now();
    await pause(700);
    expect(control.signal.aborted).toBe(true);
    const reason = aborted(control.signal);
    expect(reason).toBeInstanceOf(ProgressDeadlineExpiredV1);
    expect(reason.reason).toBe("idle");
    expect(reason.message).toContain("fixture:progress stopped: no progress for");
    expect(reason.message).toContain("fixture:progress gestoppt: seit");
    expect(performance.now() - lastWork).toBeGreaterThanOrEqual(300);
  });
});

test("a renewable control cannot be renewed past its maximum", async () => {
  await within(async (parent) => {
    const control = createRenewableIssuedResourceControlV1(parent, { idleMilliseconds: 300, maximumMilliseconds: 450 }, 1), started = performance.now();
    while (!control.signal.aborted && performance.now() - started < 2000) { noteScriptProcessActivityV1(); await pause(40); }
    expect(control.signal.aborted).toBe(true);
    expect(aborted(control.signal).reason).toBe("maximum");
    expect(performance.now() - started).toBeLessThan(1500);
  });
});

test("the release loan outlives generic preparation and still obeys its earlier real parent", async () => {
  const fixture = budgets.progressDerivations.find(row => row.kind === "native-release" && row.units === 82)!;
  const policy = progressDeadlinePolicyV1("native-release", fixture.units, {});
  const genericMaximum = buildPreparationBudgetMs(), parentMaximum = genericMaximum + 20 * 60_000;
  const epoch = Date.now(), monotonic = performance.now(), oracle = new Database(":memory:");
  let elapsed = 0;
  const date = spyOn(Date, "now").mockImplementation(() => epoch + elapsed);
  const clock = spyOn(performance, "now").mockImplementation(() => monotonic + elapsed);
  const advance = (value: number) => { elapsed = value; noteScriptProcessActivityV1(); };
  try {
    const envelope = createScriptProcessEnvelope({ version: 1, owner: "fixture:release-deadline", maximumElapsedMilliseconds: 0 }, {}, epoch);
    await withScriptProcessEnvelope(envelope, async original => {
      advance(0);
      const parent = createIssuedResourceControlV1(original, "native:driver", 0);
      const generic = createRenewableIssuedResourceControlV1(parent, progressDeadlinePolicyV1("native-child", 0, {}, { maximumMilliseconds: genericMaximum }), 0);
      const release = createRenewableIssuedResourceControlV1(parent, policy, fixture.units);
      const finiteParent = createIssuedResourceControlV1(original, "fixture:release-parent", parentMaximum);
      const nested = createRenewableIssuedResourceControlV1(finiteParent, policy, fixture.units);
      advance(genericMaximum + 1);
      expect(() => generic.remainingMilliseconds()).toThrow(/maximum duration of 360 min/u);
      const query = oracle.query("SELECT min(?1 - ?3, ?2 - ?3, ?4) AS remaining");
      const oracleRemaining = (maximum: number) => (query.get(fixture.expected!.maximumMilliseconds, maximum, elapsed, fixture.expected!.idleMilliseconds) as { remaining: number }).remaining;
      expect(release.remainingMilliseconds()).toBeCloseTo(oracleRemaining(fixture.expected!.maximumMilliseconds), 3);
      expect(nested.remainingMilliseconds()).toBeCloseTo(oracleRemaining(parentMaximum), 3);
      advance(parentMaximum + 1);
      expect(() => nested.remainingMilliseconds()).toThrow(/deadline/iu);
      expect(release.remainingMilliseconds()).toBeGreaterThan(0);
      advance(fixture.expected!.maximumMilliseconds + 1);
      expect(() => release.remainingMilliseconds()).toThrow(/maximum duration/u);
    });
  } finally {
    date.mockRestore();
    clock.mockRestore();
    oracle.close();
  }
});

test("a declared wait keeps a renewable control alive and its end starts the idle window", async () => {
  await within(async (parent) => {
    const control = createRenewableIssuedResourceControlV1(parent, { idleMilliseconds: 300, maximumMilliseconds: 5000 }, 1), leave = enterScriptProcessWaitV1();
    await pause(800);
    expect(control.signal.aborted).toBe(false);
    expect(scriptProcessActivityIdleMillisecondsV1()).toBe(0);
    leave();
    await pause(700);
    expect(control.signal.aborted).toBe(true);
    expect(aborted(control.signal).reason).toBe("idle");
  });
});

test("the activity sampler counts any change of the descendants' CPU time as work, an unreadable table as work, and no descendants as none", () => {
  const rows: { children: readonly number[] | undefined; cpu: number | undefined; work: boolean }[] = [
    { children: [], cpu: undefined, work: false },
    { children: [10], cpu: 1000, work: true },
    { children: [10], cpu: 1000, work: false },
    { children: [10], cpu: 1250, work: true },
    { children: [10], cpu: undefined, work: true },
    { children: undefined, cpu: undefined, work: true },
    { children: [], cpu: undefined, work: false },
    { children: [11], cpu: 5, work: true },
  ];
  let index = 0, noted = 0;
  const sample = createScriptProcessActivitySamplerV1({ children: () => rows[index]!.children, cpu: () => rows[index]!.cpu, note: () => { noted++; } });
  const seen = rows.map((_, at) => { index = at; return sample(); });
  expect(seen).toEqual(rows.map((row) => row.work));
  expect(noted).toBe(rows.filter((row) => row.work).length);
});

const child = (script: string) => ["-e", script];

test("a command that writes output keeps its idle window open", async () => {
  await within(async (parent) => {
    const control = createRenewableIssuedResourceControlV1(parent, { idleMilliseconds: 600, maximumMilliseconds: 20000 }, 1);
    await runOwnedCommand(process.execPath, child("let n=0;const t=setInterval(()=>{console.log('tick',n++);if(n===12){clearInterval(t)}},150)"), process.cwd(), "fixture:chatty", 0, { signal: control.signal, output: { maximumBytes: 65536, maximumLines: 1000 }, stdout: "ignore" });
    expect(control.signal.aborted).toBe(false);
  });
}, 20000);

test("a silent command is stopped after its idle window although the waiting process writes its own heartbeat", async () => {
  await within(async (parent) => {
    const control = createRenewableIssuedResourceControlV1(parent, { idleMilliseconds: 600, maximumMilliseconds: 20000 }, 1), started = performance.now();
    await expect(runOwnedCommand(process.execPath, child("setTimeout(()=>{},30000)"), process.cwd(), "fixture:silent", 0, { signal: control.signal, progressIntervalMs: 100, onProgress: () => {}, output: { maximumBytes: 65536, maximumLines: 1000 }, stdout: "ignore" })).rejects.toMatchObject({
      code: "timeout",
      key: "process.execution.progress-expired",
      values: { label: "fixture:silent", notice: expect.stringMatching(/fixture:progress stopped: no progress for.*fixture:progress gestoppt: seit/su) },
    });
    expect(control.signal.aborted).toBe(true);
    expect(aborted(control.signal).reason).toBe("idle");
    expect(performance.now() - started).toBeLessThan(10000);
  });
}, 20000);
