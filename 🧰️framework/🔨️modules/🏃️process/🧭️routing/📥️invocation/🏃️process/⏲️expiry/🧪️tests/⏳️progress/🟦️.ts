import { afterAll, beforeAll, expect, test } from "bun:test";
import progressSchema from "../../../🧬️schema/📈️progress.json";
import { createScriptProcessEnvelope, withScriptProcessEnvelope, createIssuedResourceControlV1, createRenewableIssuedResourceControlV1 } from "../../../🟦️.ts";
import { noteScriptProcessActivityV1, enterScriptProcessWaitV1, createScriptProcessActivitySamplerV1, scriptProcessActivityIdleMillisecondsV1 } from "../../../🐕️watchdog/🟦️.ts";
import { ProgressDeadlineExpiredV1 } from "../../../../../../⏱️budget/🟦️.ts";
import { runOwnedCommand } from "../../../../../../🎛️owned-execution/🟦️.ts";

const within = <T>(run: (control: ReturnType<typeof createIssuedResourceControlV1>, invocation: any) => Promise<T>) => withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "fixture:progress-deadline", maximumElapsedMilliseconds: 60000 }, {}, Date.now()), (invocation) => run(createIssuedResourceControlV1(invocation, "fixture:progress", 0), invocation));
const languageVariables = progressSchema.const.firstFailure.systemVariables, savedLanguage = languageVariables.map(name => [name, process.env[name]] as const);
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
    let failure: unknown;
    try { await runOwnedCommand(process.execPath, child("setTimeout(()=>{},30000)"), process.cwd(), "fixture:silent", 0, { signal: control.signal, progressIntervalMs: 100, onProgress: () => {}, output: { maximumBytes: 65536, maximumLines: 1000 }, stdout: "ignore" }); } catch (error) { failure = error; }
    expect(String(failure)).toContain("fixture:silent stopped: fixture:progress stopped: no progress for");
    expect(String(failure)).toContain("gestoppt: seit");
    expect(performance.now() - started).toBeLessThan(10000);
  });
}, 20000);
