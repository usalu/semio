import { spawnSync } from "node:child_process";
import { dirname, join, resolve } from "node:path";
import { existsSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import findUp from "find-up";
import { findWorkspaceRoot } from "../🟦️.ts";
import { runScriptMain } from "../🚪️entrypoint/🟦️.ts";
import { expect, test } from "bun:test";

import { Command } from "commander";
import { BundleScript, ScriptRouter, readScriptPolicy, type ScriptInvocation, type ScriptProgress } from "../🟦️.ts";
import corpus from "../🧫️fixtures/🔣️.json";
import invocationSchema from "../📥️invocation/🧬️schema/🔣️.json";
import invocationCorpus from "../📥️invocation/🧫️fixtures/🔣️.json";
import loadingSchema from "../📥️invocation/📥️loading/🧬️schema/🔣️.json";
import Ajv from "ajv";

/** 🧫️ Supplies each original routing case its real authored 15-second control and progress ledger. */
function fixtureInvocation(owner: string): ScriptInvocation<{ events: ScriptProgress[] }> {
  const controller = new AbortController(), started = performance.now(), events: ScriptProgress[] = [];
  return {
    policy: readScriptPolicy({ version: 1, owner, maximumElapsedMilliseconds: 15000 }),
    control: {
      signal: controller.signal,
      remainingMilliseconds: () => Math.max(0, 15000 - (performance.now() - started)),
      publish: event => { events.push(event); },
      yieldContinuation: () => new Promise<void>(done => setImmediate(done)),
    },
    capabilities: { events },
  };
}

for (const row of corpus.cases) test(row.id, async () => {
  const execute = async (oracle: boolean): Promise<typeof row.expected> => {
    const trace: string[] = [], router = new ScriptRouter("/future-owner", "/neutral-workspace"), invocation = fixtureInvocation(row.id);
    const commander = new Command().exitOverride().configureOutput({ writeErr() {}, writeOut() {} });
    for (const owner of row.commands) {
      class OwnedCommand extends BundleScript {
        run(args: string[]): void {
          expect(this.root).toBe("/future-owner");
          expect(this.repoRoot).toBe("/neutral-workspace");
          trace.push(`run:${owner.id}:${args.join(",")}`);
        }
      }
      const load = (): Promise<typeof OwnedCommand> => Promise.resolve().then(() => {
        trace.push(`load:${owner.id}`);
        if (owner.refuse) throw Error("absent owner");
        return OwnedCommand;
      });
      let oracleLoad: Promise<typeof OwnedCommand> | undefined;
      if (oracle) commander.command(owner.id).argument("[args...]").action(async (args: string[]) => {
        const Selected = owner.mode === "lazy" ? await (oracleLoad ??= load()) : OwnedCommand;
        await new Selected("/future-owner", "/neutral-workspace", invocation).run(args);
      });
      else if (owner.mode === "lazy") router.registerLazy(owner.id, load);
      else router.register(owner.id, OwnedCommand);
    }
    try {
      const run = (args: string[]): Promise<unknown> => oracle ? commander.parseAsync(args, { from: "user" }) : router.run(args, invocation);
      if (row.parallel) await Promise.all(row.requests.map(run));
      else for (const args of row.requests) await run(args);
      return { trace, rejected: false };
    } catch { return { trace, rejected: true }; }
  };
  expect(await execute(true)).toEqual(row.expected);
  expect(await execute(false)).toEqual(row.expected);
});

test("ambiguous registration and absent commands refuse without process termination", async () => {
  const router = new ScriptRouter("/future-owner", "/neutral-workspace");
  class OwnedCommand extends BundleScript { run(): void {} }
  router.register("future", OwnedCommand);
  expect(() => router.register("future", OwnedCommand)).toThrow("already registered");
  expect(() => router.registerLazy("future", async () => OwnedCommand)).toThrow("already registered");
  await expect(router.run(["absent"], fixtureInvocation("absent-command"))).rejects.toThrow("unknown command");
  await expect(router.run([], fixtureInvocation("empty-command"))).rejects.toThrow("usage:");
});


test("keeps routing loadable without any specific product", async () => {
  const { build } = await import("esbuild");
  const source = `import { BundleScript, ScriptRouter } from ${JSON.stringify(resolve(import.meta.dir, "../🟦️.ts"))};
import { runScriptMain } from ${JSON.stringify(resolve(import.meta.dir, "../🚪️entrypoint/🟦️.ts"))};
const corpus = ${JSON.stringify(corpus)};
function invocationFor(owner) {
  const controller = new AbortController(), started = performance.now(), events = [];
  return { policy: { version: 1, owner, maximumElapsedMilliseconds: 15000 }, control: { signal: controller.signal, remainingMilliseconds: () => Math.max(0, 15000 - (performance.now() - started)), publish: event => { events.push(event); }, yieldContinuation: () => new Promise(done => setImmediate(done)) }, capabilities: { events } };
}
const results = [];
for (const row of corpus.cases) {
  const trace = [], router = new ScriptRouter("/future-owner", "/neutral-workspace"), invocation = invocationFor(row.id);
  for (const owner of row.commands) {
    class OwnedCommand extends BundleScript { run(args) { trace.push("run:" + owner.id + ":" + args.join(",")); } }
    if (owner.mode === "lazy") router.registerLazy(owner.id, async () => { trace.push("load:" + owner.id); if (owner.refuse) throw Error("absent owner"); return OwnedCommand; });
    else router.register(owner.id, OwnedCommand);
  }
  let rejected = false;
  try { if (row.parallel) await Promise.all(row.requests.map(args => router.run(args, invocation))); else for (const args of row.requests) await router.run(args, invocation); } catch { rejected = true; }
  results.push({ id: row.id, actual: { trace, rejected } });
}
for (const row of corpus.mainCases) {
  const trace = [], router = new ScriptRouter("/future-owner", "/neutral-workspace"), invocation = invocationFor(row.id);
  for (const id of row.commands) { class OwnedCommand extends BundleScript { run(args) { trace.push("run:" + id + ":" + args.join(",")); } } router.register(id, OwnedCommand); }
  const beforeDispatch = row.interception === "none" ? undefined : async argv => { trace.push("intercept:" + argv.join(",")); if (row.interception === "refuse") throw Error("contribution refused"); return row.interception === "consume"; };
  let rejected = false;
  try { await runScriptMain(router, { invocation, argv: row.argv, defaultCommand: row.defaultCommand ?? undefined, beforeDispatch }); } catch { rejected = true; }
  results.push({ id: row.id, actual: { trace, rejected } });
}
console.log(JSON.stringify(results));`;
  const result = await build({ stdin: { contents: source, resolveDir: process.cwd(), sourcefile: "routing-product-refusal.ts" }, bundle: true, platform: "node", format: "esm", write: false, logLevel: "silent", plugins: [{ name: "refuse-specific-products", setup(builder) { builder.onLoad({ filter: /./ }, (args) => { if (args.path.replaceAll("\\", "/").includes("/🧰️framework/🛍️products/")) throw Error(`general routing loads specific product ${args.path}`); return undefined; }); } }] });
  const child = spawnSync("node", ["--input-type=module", "-e", result.outputFiles[0]!.text], { encoding: "utf8" });
  expect(child.status, child.stderr).toBe(0);
  expect(JSON.parse(child.stdout)).toEqual([...corpus.cases, ...corpus.mainCases].map(row => ({ id: row.id, actual: row.expected })));
});

for (const row of corpus.mainCases) test(row.id, async () => {
  const execute = async (oracle: boolean): Promise<typeof row.expected> => {
    const trace: string[] = [], router = new ScriptRouter("/future-owner", "/neutral-workspace"), invocation = fixtureInvocation(row.id), commander = new Command().exitOverride().configureOutput({ writeErr() {}, writeOut() {} });
    for (const id of row.commands) {
      class OwnedCommand extends BundleScript { run(args: string[]): void { trace.push(`run:${id}:${args.join(",")}`); } }
      router.register(id, OwnedCommand);
      commander.command(id).argument("[args...]").action((args: string[]) => { trace.push(`run:${id}:${args.join(",")}`); });
    }
    const beforeDispatch = row.interception === "none" ? undefined : async (argv: readonly string[], received: ScriptInvocation): Promise<boolean> => {
      expect(received).toBe(invocation);
      trace.push(`intercept:${argv.join(",")}`);
      if (row.interception === "refuse") throw Error("contribution refused");
      return row.interception === "consume";
    };
    try {
      if (oracle) {
        if (await beforeDispatch?.(row.argv, invocation)) return { trace, rejected: false };
        const argv = row.argv.length ? row.argv : row.defaultCommand ? [row.defaultCommand] : [];
        if (!row.commands.length || !argv.length) throw Error("missing command");
        await commander.parseAsync(argv, { from: "user" });
      } else await runScriptMain(router, { invocation, argv: row.argv, defaultCommand: row.defaultCommand ?? undefined, beforeDispatch });
      return { trace, rejected: false };
    } catch { return { trace, rejected: true }; }
  };
  expect(await execute(true)).toEqual(row.expected);
  expect(await execute(false)).toEqual(row.expected);
});

for (const row of corpus.workspaceCases) test(row.id, () => {
  const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "process-root-"));
  try {
    mkdirSync(resolve(root, row.start), { recursive: true });
    for (const marker of row.markers) {
      mkdirSync(resolve(root, marker.directory), { recursive: true });
      for (const filename of marker.files) writeFileSync(resolve(root, marker.directory, filename), "{}");
    }
    const oracle = findUp.sync((directory: string) => existsSync(join(directory, "nx.json")) && existsSync(join(directory, "package.json")) ? "nx.json" : directory === root ? findUp.stop : undefined, { cwd: resolve(root, row.start) });
    const expected = row.expected === null ? undefined : resolve(root, row.expected);
    expect(oracle === undefined ? undefined : dirname(oracle)).toBe(expected);
    if (expected === undefined) expect(() => findWorkspaceRoot(resolve(root, row.start), { stopAt: root })).toThrow("workspace root");
    else expect(findWorkspaceRoot(resolve(root, row.start), { stopAt: root })).toBe(expected);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("actual neutral constructors, dispatch and main receive the original finite caller authority", async () => {
  const validate = new Ajv({ strict: true }).compile(invocationSchema);
  for (const row of invocationCorpus.cases) {
    expect(validate(row.value), row.id).toBe(row.accepted);
    if (row.accepted) expect<unknown>(readScriptPolicy(row.value)).toBe(row.value);
    else expect(() => readScriptPolicy(row.value)).toThrow();
  }
  const controller = new AbortController(), started = performance.now(), events: unknown[] = [], traces: string[] = [];
  const invocation = {
    policy: readScriptPolicy(invocationCorpus.cases[0]!.value),
    control: {
      signal: controller.signal,
      remainingMilliseconds: () => Math.max(0, 15000 - (performance.now() - started)),
      publish: (event: unknown) => { events.push(event); },
      yieldContinuation: () => new Promise<void>(done => setImmediate(done)),
    },
    capabilities: { record: (value: string) => traces.push(value) },
  };
  const oracle = new Command().exitOverride().configureOutput({ writeErr() {}, writeOut() {} });
  oracle.command("authority").argument("[args...]").action((args: string[]) => { invocation.capabilities.record(args.join(",")); });
  await oracle.parseAsync(["authority", "direct"], { from: "user" });
  await oracle.parseAsync(["authority", "main"], { from: "user" });
  const expected = [...traces];
  traces.length = 0;
  class AuthorityCommand extends BundleScript<typeof invocation.capabilities> {
    async run(args: string[]): Promise<void> {
      const received = this.invocation;
      expect(received).toBe(invocation);
      expect(received.control.signal).toBe(controller.signal);
      expect(received.control.remainingMilliseconds()).toBeLessThanOrEqual(15000);
      await received.control.yieldContinuation();
      received.capabilities.record(args.join(","));
    }
  }
  const router = new ScriptRouter<typeof invocation.capabilities>("/future-owner", "/neutral-workspace").register("authority", AuthorityCommand);
  await router.run(["authority", "direct"], invocation);
  await runScriptMain(router, { argv: ["authority", "main"], invocation });
  expect(expected).toEqual(["direct", "main"]);
  expect(traces).toEqual(expected);
  expect(events.length).toBeGreaterThan(0);
  controller.abort();
  await expect(router.run(["authority", "cancelled"], invocation)).rejects.toThrow();
  console.log("[DEBUG] original neutral invocation signal/capability identity and finite caller deadline received");
});

test("missing incoming authority refuses before acquiring the selected command owner", async () => {
  let loads = 0, executions = 0;
  class AuthorityCommand extends BundleScript { run(): void { executions++; } }
  const router = new ScriptRouter("/future-owner", "/neutral-workspace").registerLazy("authority", async () => {
    loads++;
    return AuthorityCommand;
  });
  await expect((router.run as unknown as (args: string[]) => Promise<void>)(["authority"])).rejects.toThrow();
  expect(loads).toBe(0);
  expect(executions).toBe(0);
  await expect((runScriptMain as unknown as (router: ScriptRouter, options: { argv: string[] }) => Promise<void>)(router, { argv: ["authority"] })).rejects.toThrow();
  expect(loads).toBe(0);
  expect(executions).toBe(0);
});

for (const row of invocationCorpus.loadingCases) test(row.id, async () => {
  expect(new Ajv({ strict: true }).compile(loadingSchema)(row)).toBe(true);
  const execute = async (oracle: boolean): Promise<{ trace: string[]; rejected: boolean }> => {
    const controller = new AbortController(), started = performance.now(), events: ScriptProgress[] = [], trace: string[] = [];
    const invocation: ScriptInvocation<{ trace: string[] }> = {
      policy: readScriptPolicy({ version: 1, owner: row.id, maximumElapsedMilliseconds: 15000 }),
      control: {
        signal: controller.signal,
        remainingMilliseconds: () => Math.max(0, 15000 - (performance.now() - started)),
        publish: event => { events.push(event); },
        yieldContinuation: () => new Promise<void>(done => setImmediate(done)),
      },
      capabilities: { trace },
    };
    class LoadedCommand extends BundleScript<typeof invocation.capabilities> {
      run(): void { this.invocation.capabilities.trace.push("run"); }
    }
    const load = async (received: typeof invocation): Promise<typeof LoadedCommand> => {
      expect(received).toBe(invocation);
      received.capabilities.trace.push("load");
      await received.control.yieldContinuation();
      if (row.cancelAtLoad) controller.abort();
      return LoadedCommand;
    };
    try {
      if (oracle) {
        const commander = new Command().exitOverride().configureOutput({ writeErr() {}, writeOut() {} });
        commander.command("owned").action(async () => {
          const Command = await load(invocation);
          invocation.control.signal.throwIfAborted();
          await new Command("/future-owner", "/neutral-workspace", invocation).run();
        });
        await commander.parseAsync(["owned"], { from: "user" });
      } else {
        const router = new ScriptRouter<typeof invocation.capabilities>("/future-owner", "/neutral-workspace");
        router.registerLazy("owned", load);
        await router.run(["owned"], invocation);
      }
      return { trace, rejected: false };
    } catch { return { trace, rejected: true }; }
  };
  const expected = { trace: row.trace, rejected: row.rejected };
  expect(await execute(true)).toEqual(expected);
  expect(await execute(false)).toEqual(expected);
});

for (const row of (await import("../📥️invocation/🏃️process/🧫️fixtures/🔣️.json")).default.cases) test(`process-envelope:${row.id}`, async () => {
  const { readScriptProcessEnvelope } = await import("../📥️invocation/🏃️process/🟦️.ts");
  const schema = (await import("../📥️invocation/🏃️process/🧬️schema/🔣️.json")).default;
  const structural = new Ajv({ strict: true }).addSchema(invocationSchema).compile(schema)(row.value);
  const value = row.value as { policy: { maximumElapsedMilliseconds: number }; deadlineEpochMilliseconds?: number };
  const remaining = (value.deadlineEpochMilliseconds ?? NaN) - row.now;
  const oracle = structural && remaining > 0 && remaining <= value.policy.maximumElapsedMilliseconds;
  expect(oracle).toBe(row.accepted);
  if (row.accepted) { if (row.remaining === undefined) throw Error("Accepted neutral row requires original remaining authority"); const deadline=readScriptProcessEnvelope(row.value,row.now).deadlineEpochMilliseconds;if(deadline===null)throw Error("Original finite test row requires finite deadline");expect(deadline-row.now).toBe(row.remaining); }
  else expect(() => readScriptProcessEnvelope(row.value, row.now)).toThrow();
  console.log(`[DEBUG] Original process envelope ${row.id} accepted=${oracle} remaining=${remaining}`);
});

test("original process child handoff preserves deadline and real ports", async () => {
  const api = await import("../📥️invocation/🏃️process/🟦️.ts");
  const policy = readScriptPolicy({ version: 1, owner: "original-process-handoff", maximumElapsedMilliseconds: 1000 });
  const capabilities = { owner: "original-process-handoff", command: "original-handoff" };
  const envelope = api.createScriptProcessEnvelope(policy, capabilities, Date.now()), environment = api.scriptProcessEnvironment(envelope, {});
  expect(JSON.parse(environment[api.SCRIPT_PROCESS_INVOCATION_ENV]!)).toEqual(envelope);
  const beforeInterrupt = process.listenerCount("SIGINT"), beforeTerminate = process.listenerCount("SIGTERM");
  await api.receiveScriptProcessInvocation(environment, async original => {
    expect(original.policy).toEqual(policy);
    expect(original.capabilities).toEqual(envelope.capabilities);
    expect(original.capabilities).toEqual(capabilities);
    expect(original.control.signal.aborted).toBe(false);
    const before = original.control.remainingMilliseconds();
    if(before===null)throw Error("Original finite handoff requires finite remaining authority");
    await original.control.publish({ owner: policy.owner, command: "original-handoff", stage: "running" });
    await original.control.yieldContinuation();
    expect(original.control.remainingMilliseconds()).toBeLessThanOrEqual(before);
  });
  expect(process.listenerCount("SIGINT")).toBe(beforeInterrupt);
  expect(process.listenerCount("SIGTERM")).toBe(beforeTerminate);
  await expect(api.receiveScriptProcessInvocation({}, async () => true)).rejects.toThrow("Original parent");
  console.log(`[DEBUG] Original process deadline preserved=${envelope.deadlineEpochMilliseconds} listener custody returned`);
});

test("original process deadline refuses late completion through actual AbortSignal", async () => {
  const api = await import("../📥️invocation/🏃️process/🟦️.ts");
  const envelope = api.createScriptProcessEnvelope(readScriptPolicy({ version: 1, owner: "original-process-timeout", maximumElapsedMilliseconds: 30 }), { command: "original-timeout" }, Date.now());
  let observed = false;
  await expect(api.withScriptProcessEnvelope(envelope, async original => {
    await new Promise(done => setTimeout(done, 50));
    observed = original.control.signal.aborted;
  })).rejects.toThrow();
  expect(observed).toBe(true);
  console.log("[DEBUG] Original process deadline cancelled actual signal and refused late terminal success");
});

test("the complete process envelope preserves original caller capabilities by identity", async () => {
  const api = await import("../📥️invocation/🏃️process/🟦️.ts"), schema = (await import("../📥️invocation/🏃️process/🧬️schema/🔣️.json")).default;
  const capabilities = { owner: "original-process-caller", command: "original-handoff" }, policy = readScriptPolicy({ version: 1, owner: "original-process-caller", maximumElapsedMilliseconds: 1000 });
  const envelope = { version: 1 as const, policy, deadlineEpochMilliseconds: Date.now() + 1000, capabilities };
  expect(new Ajv({ strict: true }).addSchema(invocationSchema).compile(schema)(envelope)).toBe(true);
  await api.withScriptProcessEnvelope(envelope, async original => {
    expect(original.policy).toBe(policy);
    expect(original.capabilities).toBe(capabilities);
    await original.control.publish({ owner: policy.owner, command: "original-handoff", stage: "running" });
    await original.control.yieldContinuation();
    expect(original.control.remainingMilliseconds()).toBeLessThanOrEqual(1000);
  });
});

test("process policy schema references the canonical original Script owner", async () => {
  const schema = (await import("../📥️invocation/🏃️process/🧬️schema/🔣️.json")).default, fixture = (await import("../📥️invocation/🏃️process/🧫️fixtures/🔣️.json")).default;
  const reference = { $ref: fixture.policyReference }, oracle = new Ajv({ strict: true }).addSchema(invocationSchema).compile({ ...schema, properties: { ...schema.properties, policy: reference } });
  expect(invocationSchema.$id).toBe(fixture.policyReference);
  for (const row of fixture.cases) expect(oracle(row.value)).toBe(new Ajv({ strict: true }).addSchema(invocationSchema).compile(schema)(row.value));
  expect(schema.properties.policy).toEqual(reference);
});


test("original ordinary no-deadline authority preserves cancellation and fractional finite child ceilings", async () => {
  const api = await import("../📥️invocation/🏃️process/🟦️.ts"), shared = await import("../📥️invocation/🟦️.ts"), rows = (await import("../📥️invocation/🏃️process/🧫️fixtures/🌿️ordinary.json")).default;
  const wire = (await import("../📥️invocation/🏃️process/🧬️schema/🔣️.json")).default;
  const policyValidate = new Ajv({ strict: true }).compile(invocationSchema), envelopeValidate = new Ajv({ strict: true }).addSchema(invocationSchema).compile(wire);
  for (const row of rows.envelopes) {
    expect(envelopeValidate(row.value)).toBe(row.accepted);
    if (row.accepted) expect(Object.is(api.readScriptProcessEnvelope(row.value, row.now),row.value)).toBe(true);
    else expect(() => api.readScriptProcessEnvelope(row.value, row.now)).toThrow();
  }
  const controller = new AbortController(), progress: ScriptProgress[] = [], capabilities = { owner: "zero-authority-original-caller", commands: [] as string[] };
  const policy = { version: 1 as const, owner: capabilities.owner, maximumElapsedMilliseconds: 0 };
  const invocation = { policy, control: { signal: controller.signal, remainingMilliseconds: () => null, publish: async (event: ScriptProgress) => { progress.push(event); }, yieldContinuation: () => new Promise<void>(done => setImmediate(done)) }, capabilities };
  expect(policyValidate(policy)).toBe(true); expect(readScriptPolicy(policy)).toEqual(policy);
  let acquisitions = 0, executions = 0;
  class OriginalCommand extends BundleScript { run(): void { executions++; } }
  const router = new ScriptRouter("/original-owner", "/original-workspace").registerLazy("owned", async original => { acquisitions++; expect(original).toBe(invocation); return OriginalCommand; });
  await router.run(["owned"], invocation);
  await runScriptMain(router, {argv:["owned"], invocation});
  expect(() => new OriginalCommand("/original-owner", "/original-workspace", invocation)).not.toThrow();
  expect(acquisitions).toBe(1); expect(executions).toBe(2); expect(progress.some(event=>event.stage==="complete")).toBe(true);
  const exhaustedProgress:ScriptProgress[]=[], exhausted={policy:{version:1 as const,owner:"original-exhausted-finite",maximumElapsedMilliseconds:1000},control:{signal:controller.signal,remainingMilliseconds:()=>0,publish:async(event:ScriptProgress)=>{exhaustedProgress.push(event);},yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))},capabilities};
  let exhaustedLoads=0;
  const exhaustedRouter=new ScriptRouter("/original-owner","/original-workspace").registerLazy("owned",async()=>{exhaustedLoads++;return OriginalCommand;});
  await expect(exhaustedRouter.run(["owned"],exhausted)).rejects.toThrow("deadline refused");
  await expect(runScriptMain(exhaustedRouter,{argv:["owned"],invocation:exhausted})).rejects.toThrow("deadline refused");
  expect(()=>new OriginalCommand("/original-owner","/original-workspace",exhausted)).toThrow("deadline refused");
  expect(exhaustedLoads).toBe(0);expect(exhaustedProgress).toEqual([]);
  const listeners=[process.listenerCount("SIGINT"),process.listenerCount("SIGTERM")];
  await expect(api.withScriptProcessEnvelope(api.createScriptProcessEnvelope(policy,capabilities,Date.now()),async original=>{expect(original.control.remainingMilliseconds()).toBe(null);await new Promise<void>(done=>setTimeout(done,20));expect(original.control.remainingMilliseconds()).toBe(null);expect(original.control.signal.aborted).toBe(false);process.emit("SIGINT");expect(original.control.signal.aborted).toBe(true);})).rejects.toThrow("Original Script process interrupted");
  expect([process.listenerCount("SIGINT"),process.listenerCount("SIGTERM")]).toEqual(listeners);
  const oracle = Bun.spawnSync(["node", "--eval", 'const rows=JSON.parse(process.argv[1]);console.log(JSON.stringify(rows.map(row=>{const accepted=Number.isSafeInteger(row.parent)&&Number.isSafeInteger(row.requested)&&row.requested>=0&&((row.parent===0&&row.remaining===null)||(row.parent>0&&Number.isFinite(row.remaining)&&row.remaining>=1&&row.remaining<=row.parent));return{accepted,...(accepted?{expected:row.parent===0?row.requested:Math.floor(row.requested===0?row.remaining:Math.min(row.remaining,row.requested))}:{})}})))', JSON.stringify(rows.budgets)]);
  expect(oracle.exitCode).toBe(0); expect(JSON.parse(new TextDecoder().decode(oracle.stdout))).toEqual(rows.budgets.map(row => ({ accepted: row.accepted, ...("expected" in row ? { expected: row.expected } : {}) })));
  const outcomes = [];
  for (const row of rows.budgets) {
    const signal = new AbortController(), events: ScriptProgress[] = [];
    const original = { policy: { version: 1 as const, owner: "original-child-budget", maximumElapsedMilliseconds: row.parent }, control: { signal: signal.signal, remainingMilliseconds: () => row.remaining, publish: async (event: ScriptProgress) => { events.push(event); }, yieldContinuation: () => new Promise<void>(done => setImmediate(done)) }, capabilities: { originalChildCeiling: row.requested } };
    try { outcomes.push({ accepted: true, expected: shared.scriptInvocationBudget(original, row.requested) }); }
    catch { outcomes.push({ accepted: false }); }
  }
  expect(outcomes).toEqual(rows.budgets.map(row => ({ accepted: row.accepted, ...("expected" in row ? { expected: row.expected } : {}) })));
  console.log("[DEBUG] Original ordinary0/null actual load, main, constructor and SIGINT preserve no deadline; strict Ajv and Node preserve fractional remaining with original integer child ceilings");
});
