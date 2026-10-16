import "../📥️invocation/🏃️process/🧪️tests/📊️profile/🟦️.ts";
import "../📤️exports/🧪️tests/🟦️.ts";
import "../📥️invocation/🏃️process/🧹️cleanup/🧪️tests/🟦️.ts";
import { spawnSync } from "node:child_process";
import { dirname, join, resolve } from "node:path";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import findUp from "find-up";
import { findWorkspaceRoot } from "../🟦️.ts";
import { runScriptMain } from "../🚪️entrypoint/🟦️.ts";
import { expect, test, spyOn } from "bun:test";

import { Command } from "commander";
import { BundleScript, ScriptRouter, readScriptPolicy, checkScriptInvocation, type ScriptInvocation, type ScriptProgress } from "../🟦️.ts";
import corpus from "./🧫️fixtures/🔣️.json";
import invocationSchema from "../📥️invocation/🧬️schema/🔣️.json";
import invocationCorpus from "../📥️invocation/🧪️tests/🧫️fixtures/🔣️.json";
import Ajv from "ajv";
import ts from "typescript";
import {types} from "node:util";

test("immutable admitted policy data avoids schema replay while mutable and executable properties remain checked",async()=>{
 const source=readFileSync(new URL("../📥️invocation/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("invocation.ts",source,ts.ScriptTarget.Latest,true),printer=ts.createPrinter(),body=tree.statements.filter(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="readScriptPolicy"||ts.isVariableStatement(node)&&node.declarationList.declarations.some(declaration=>declaration.name.getText(tree)==="immutablePolicies")).map(node=>printer.printNode(ts.EmitHint.Unspecified,node,tree)).join("\n").replaceAll("export function","function"),javascript=ts.transpileModule(body,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,{compileJsonSchemaSubset}=await import("@semio-tech/schema-validator-ts"),contract=compileJsonSchemaSubset(invocationSchema),oracle=new Ajv({strict:true}).compile(invocationSchema);
 for(const row of invocationCorpus.immutablePolicyCases){let validations=0;const read=new Function("scriptPolicyContract","types",javascript+";return readScriptPolicy;")({validate(value:unknown){validations++;return contract.validate(value);}},types),base={version:1,owner:"immutable-policy-fixture",maximumElapsedMilliseconds:15000};let value:any;
  if(row.kind==="null-prototype")value=Object.freeze(Object.assign(Object.create(null),base));else if(row.kind==="accessor")value=Object.freeze({...base,get owner(){return base.owner;}});else if(row.kind==="proxy")value=new Proxy(Object.freeze({...base}),{});else value=row.kind==="frozen"?Object.freeze({...base}):{...base};
  expect(oracle(value),row.kind).toBe(true);expect(read(value)).toBe(value);expect(read(value)).toBe(value);expect(validations,row.kind).toBe(row.validations);
  if(row.kind==="mutable"||row.kind==="accessor"){base.owner="";if(row.kind==="mutable")value.owner="";expect(oracle(value)).toBe(false);expect(()=>read(value)).toThrow("Complete Script policy required");}
 }
 for(const value of [Object.freeze({version:1,owner:"",maximumElapsedMilliseconds:15000}),Object.freeze({version:1,owner:"invalid-frozen",maximumElapsedMilliseconds:-1})]){let validations=0;const read=new Function("scriptPolicyContract","types",javascript+";return readScriptPolicy;")({validate(value:unknown){validations++;return contract.validate(value);}},types);expect(oracle(value)).toBe(false);expect(()=>read(value)).toThrow("Complete Script policy required");expect(()=>read(value)).toThrow("Complete Script policy required");expect(validations).toBe(2);}
 console.log("[DEBUG] Actual policy admission method agrees with independent Ajv; only exact immutable plain data avoids repeated schema work");
});

test("owned expected keysets avoid repeated sorting without caching current invocation checks",()=>{
 const value=fixtureInvocation("compiled-keyset-count"),invocation={...value,policy:Object.freeze({...value.policy})};checkScriptInvocation(invocation);
 const native=Array.prototype.sort;let sorts=0;const spy=spyOn(Array.prototype,"sort").mockImplementation(function(this:unknown[],...args:any[]){sorts++;return native.apply(this,args as any);});
 try{checkScriptInvocation(invocation);checkScriptInvocation(invocation);}finally{spy.mockRestore();}
 expect(sorts).toBe(0);console.log("[DEBUG] Actual current invocation and control checks retain enumeration without repeated expected/actual sorting");
});

test("compiled exact key plans use bounded direct membership without array callbacks",async()=>{
 const vm=await import("node:vm"),{default:fixture}=await import("../📥️invocation/🧪️tests/🧫️fixtures/🔑️keys.json"),{Database}=await import("bun:sqlite"),source=readFileSync(new URL("../../../🧬️schema/✅️validator/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("validator.ts",source,ts.ScriptTarget.Latest,true),node=tree.statements.find(value=>ts.isFunctionDeclaration(value)&&value.name?.text==="compileExactKeysV1");if(!node)throw Error("Actual exact key plan implementation required");const code=ts.transpileModule(node.getText(tree).replace("export function","function"),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,context=vm.createContext({});vm.runInContext(`const counts={enumerations:0,arrayCallbacks:0,arrayMembership:0,directMembership:0};const keys=Object.keys,some=Array.prototype.some,includes=Array.prototype.includes,has=Set.prototype.has;Object.keys=value=>{counts.enumerations++;return keys(value)};Array.prototype.some=function(...args){counts.arrayCallbacks++;return Reflect.apply(some,this,args)};Array.prototype.includes=function(...args){counts.arrayMembership++;return Reflect.apply(includes,this,args)};Set.prototype.has=function(...args){counts.directMembership++;return Reflect.apply(has,this,args)};${code};globalThis.compile=compileExactKeysV1;globalThis.counts=counts;`,context);
 const metadata=invocationSchema.definitions.ExpectedKeys.const,compile=context.compile as(keys:readonly string[])=>(value:object,name:string)=>void,invocation=compile(metadata.invocation),control=compile(metadata.control),db=new Database(":memory:");const api:typeof import("../📥️invocation/🏃️process/🟦️.ts")=await import("../📥️invocation/🏃️process/🟦️.ts");
 try{await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:"exact-key-plan-work",maximumElapsedMilliseconds:5000},{fixture:"exact-key-plan-work"},Date.now()),async original=>{for(const key of Object.keys(fixture.work))if(key!=="iterations")context.counts[key]=0;for(let index=0;index<fixture.work.iterations;index++){api.requireIssuedScriptInvocationV1(original);invocation(original,"Script invocation");control(original.control,"Script control");}});const actual=JSON.parse(JSON.stringify(context.counts)),oracle=db.query("select (json_array_length(?1,'$.invocation')+json_array_length(?1,'$.control'))*?2 membership,2*?2 enumerations").get(JSON.stringify(metadata),fixture.work.iterations)as{membership:number;enumerations:number};expect(oracle).toEqual({membership:fixture.work.directMembership,enumerations:fixture.work.enumerations});expect(actual).toEqual({enumerations:fixture.work.enumerations,arrayCallbacks:fixture.work.arrayCallbacks,arrayMembership:fixture.work.arrayMembership,directMembership:fixture.work.directMembership});console.log("[DEBUG] Actual isolated key-plan method work "+JSON.stringify({actual,oracle,scope:"VM instrumentation is isolated from Original and test/schema prototypes"}));}finally{db.close();}
});

test("compiled exact key snapshots match SQLite and preserve current object observables",async()=>{
 const {Database}=await import("bun:sqlite"),{compileExactKeysV1,requireExactKeys}=await import("@semio-tech/schema-validator-ts"),{default:keys}=await import("../📥️invocation/🧪️tests/🧫️fixtures/🔑️keys.json"),db=new Database(":memory:"),recordKeys=invocationSchema.definitions.ExpectedKeys.const;
 expect(new Ajv({strict:true}).compile(invocationSchema.definitions.ExpectedKeys)(recordKeys)).toBe(true);
 const result=(run:()=>void)=>{try{run();return{accepted:true,error:undefined};}catch(error){return{accepted:false,error:(error as Error)?.message};}};
 try{for(const row of keys.cases){const value=Object.fromEntries(row.actual.map(key=>[key,1])),compiled=compileExactKeysV1(row.keys),expected=db.query("select json_array_length(?1)=json_array_length(?2) and not exists(select value from json_each(?1) except select value from json_each(?2)) accepted").get(JSON.stringify(row.actual),JSON.stringify(row.keys))as{accepted:number};expect(Boolean(expected.accepted)).toBe(row.accepted);expect(result(()=>compiled(value,"fixture"))).toEqual(result(()=>requireExactKeys(value,row.keys,"fixture")));expect(result(()=>compiled(value,"fixture")).accepted).toBe(row.accepted);}}
 finally{db.close();}
 const mutable=["a"],compiled=compileExactKeysV1(mutable);mutable[0]="b";expect(()=>requireExactKeys({b:1},mutable,"fixture")).not.toThrow();expect(()=>compiled({b:1},"fixture")).toThrow("exactly a");expect(()=>compiled({a:1},"fixture")).not.toThrow();
 let reads=0,traps=0;const target=Object.create({inherited:1});Object.defineProperty(target,"a",{enumerable:true,configurable:true,get(){reads++;return 1;}});Object.defineProperty(target,"hidden",{value:1,enumerable:false});target[Symbol("extra")]=1;const proxy=new Proxy(target,{ownKeys(value){traps++;return Reflect.ownKeys(value);}});
 compiled(proxy,"fixture");compiled(proxy,"fixture");expect(traps).toBe(2);expect(reads).toBe(0);Object.defineProperty(target,"a",{enumerable:false});expect(()=>compiled(proxy,"fixture")).toThrow("exactly a");expect(traps).toBe(3);expect(reads).toBe(0);
 let caught=false,reason:unknown;try{compiled(new Proxy({},{ownKeys(){throw undefined;}}),"fixture");}catch(error){caught=true;reason=error;}expect(caught).toBe(true);expect(reason).toBeUndefined();
 console.log("[DEBUG] Explicit owned expected snapshots match independent SQLite; generic mutable metadata and actual proxy/getter/symbol/nonenumerable/undefined failure semantics remain current");
});

test("owned expected keysets retain every current invocation value signal and deadline check",()=>{
 const controller=new AbortController(),reason=Object.freeze(Error("current Original cancelled"));let remaining:number|null=10,capabilities:any={},reads=0;const base=fixtureInvocation("keyset-current-values"),control={...base.control,signal:controller.signal,remainingMilliseconds:()=>{reads++;return remaining;}},value:any={policy:Object.freeze({...base.policy}),control,get capabilities(){return capabilities;}};
 checkScriptInvocation(value);expect(reads).toBe(1);remaining=0;expect(()=>checkScriptInvocation(value)).toThrow("deadline");expect(reads).toBe(2);remaining=10;capabilities=null;expect(()=>checkScriptInvocation(value)).toThrow("capabilities");expect(reads).toBe(2);capabilities={};control.publish=undefined as any;expect(()=>checkScriptInvocation(value)).toThrow("ports");control.publish=base.control.publish;value.extra=1;expect(()=>checkScriptInvocation(value)).toThrow("exactly");delete value.extra;controller.abort(reason);let error:unknown;try{checkScriptInvocation(value);}catch(caught){error=caught;}expect(error).toBe(reason);expect(reads).toBe(2);
 console.log("[DEBUG] Actual invocation key plans recheck changed values, callable ports, added own fields, deadline and exact current cancellation on every call");
});

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
      for (const filename of marker.files) writeFileSync(resolve(root, marker.directory, filename), JSON.stringify(filename === "package.json" && !("workspace" in marker && marker.workspace === false) ? { workspaces: [] } : {}));
    }
    const oracle = findUp.sync((directory: string) => existsSync(join(directory, "📋️project.json")) && existsSync(join(directory, "package.json")) && Object.hasOwn(JSON.parse(readFileSync(join(directory, "package.json"), "utf8")), "workspaces") ? "📋️project.json" : directory === root ? findUp.stop : undefined, { cwd: resolve(root, row.start) });
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

for (const row of (await import("../📥️invocation/🏃️process/🧪️tests/🧫️fixtures/🔣️.json")).default.cases) test(`process-envelope:${row.id}`, async () => {
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

test("owned child environment carries the caller's current original invocation", async () => {
  const api = await import("../📥️invocation/🏃️process/🟦️.ts");
  const policy = readScriptPolicy({ version: 1, owner: "original-invocation-handoff", maximumElapsedMilliseconds: 60_000 });
  const capabilities = { command: "invocation-handoff" };
  await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope(policy, capabilities, Date.now()), async parent => {
    const environment = api.scriptInvocationEnvironment(parent, { KEPT: "yes" });
    expect(environment.KEPT).toBe("yes");
    const wire = JSON.parse(environment[api.SCRIPT_PROCESS_INVOCATION_ENV]!) as { policy: unknown; capabilities: unknown; deadlineEpochMilliseconds: number };
    expect(wire.policy).toEqual(policy);
    expect(wire.capabilities).toEqual(capabilities);
    expect<number|null>(wire.deadlineEpochMilliseconds).toBe(api.readIssuedScriptProcessEnvelopeV1(parent).deadlineEpochMilliseconds);
    await api.receiveScriptProcessInvocation(environment, async child => {
      expect(child.policy).toEqual(policy);
      expect(child.capabilities).toEqual(capabilities);
      expect(child.control.remainingMilliseconds()!).toBeLessThanOrEqual(parent.control.remainingMilliseconds()!);
    });
  });
  const aborted = new AbortController();
  await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope(policy, capabilities, Date.now()), async parent => {
    expect(()=>api.scriptInvocationEnvironment({...parent},{})).toThrow("Live Original process issuance required");
    await api.withScriptInvocationSignalV1(parent,aborted.signal,async cancelled=>{aborted.abort(Error("parent cancelled"));expect(()=>api.scriptInvocationEnvironment(cancelled,{})).toThrow("parent cancelled");api.acknowledgeScriptInvocationSignalStopV1(cancelled);});
    expect(parent.control.signal.aborted).toBe(false);
  });
});

test("original process child handoff preserves deadline and real ports", async () => {
  const api = await import("../📥️invocation/🏃️process/🟦️.ts");
  const policy = readScriptPolicy({ version: 1, owner: "original-process-handoff", maximumElapsedMilliseconds: 1000 });
  const capabilities = { owner: "original-process-handoff", command: "original-handoff" };
  const envelope = api.createScriptProcessEnvelope(policy, capabilities, Date.now()), environment = api.scriptProcessEnvironment(envelope, {});
  expect(JSON.parse(environment[api.SCRIPT_PROCESS_INVOCATION_ENV]!)).toEqual(envelope);
  const beforeInterrupt = process.listenerCount("SIGINT"), beforeTerminate = process.listenerCount("SIGTERM");
  const priorWire=process.env[api.SCRIPT_PROCESS_INVOCATION_ENV];process.env[api.SCRIPT_PROCESS_INVOCATION_ENV]=environment[api.SCRIPT_PROCESS_INVOCATION_ENV];
  try{await api.receiveScriptProcessInvocation(environment, async original => {
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
  }finally{if(priorWire===undefined)delete process.env[api.SCRIPT_PROCESS_INVOCATION_ENV];else process.env[api.SCRIPT_PROCESS_INVOCATION_ENV]=priorWire;}
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

test("the complete process envelope captures immutable original caller capabilities", async () => {
  const api = await import("../📥️invocation/🏃️process/🟦️.ts"), schema = (await import("../📥️invocation/🏃️process/🧬️schema/🔣️.json")).default;
  const capabilities = { owner: "original-process-caller", command: "original-handoff" }, policy = readScriptPolicy({ version: 1, owner: "original-process-caller", maximumElapsedMilliseconds: 1000 });
  const envelope = { version: 1 as const, policy, deadlineEpochMilliseconds: Date.now() + 1000, capabilities };
  expect(new Ajv({ strict: true }).addSchema(invocationSchema).compile(schema)(envelope)).toBe(true);
  await api.withScriptProcessEnvelope(envelope, async original => {
    expect(original.policy).toEqual(policy);
    expect(original.capabilities).toEqual(capabilities);
    expect(Object.isFrozen(original.policy)).toBe(true);
    expect(Object.isFrozen(original.capabilities)).toBe(true);
    capabilities.command="changed-after-admission";
    expect((original.capabilities as typeof capabilities).command).toBe("original-handoff");
    await original.control.publish({ owner: policy.owner, command: "original-handoff", stage: "running" });
    await original.control.yieldContinuation();
    expect(original.control.remainingMilliseconds()).toBeLessThanOrEqual(1000);
  });
});

test("process policy schema references the canonical original Script owner", async () => {
  const schema = (await import("../📥️invocation/🏃️process/🧬️schema/🔣️.json")).default, fixture = (await import("../📥️invocation/🏃️process/🧪️tests/🧫️fixtures/🔣️.json")).default;
  const reference = { $ref: fixture.policyReference }, oracle = new Ajv({ strict: true }).addSchema(invocationSchema).compile({ ...schema, properties: { ...schema.properties, policy: reference } });
  expect(invocationSchema.$id).toBe(fixture.policyReference);
  for (const row of fixture.cases) expect(oracle(row.value)).toBe(new Ajv({ strict: true }).addSchema(invocationSchema).compile(schema)(row.value));
  expect(schema.properties.policy).toEqual(reference);
});


test("original ordinary no-deadline authority preserves cancellation and fractional finite child ceilings", async () => {
  const api = await import("../📥️invocation/🏃️process/🟦️.ts"), shared = await import("../📥️invocation/🟦️.ts"), rows = (await import("../📥️invocation/🏃️process/🧪️tests/🧫️fixtures/🌿️ordinary.json")).default;
  const wire = (await import("../📥️invocation/🏃️process/🧬️schema/🔣️.json")).default;
  const policyValidate = new Ajv({ strict: true }).compile(invocationSchema), envelopeValidate = new Ajv({ strict: true }).addSchema(invocationSchema).compile(wire);
  for (const row of rows.envelopes) {
    expect(envelopeValidate(row.value)).toBe(row.accepted);
    if (row.accepted) {const admitted=api.readScriptProcessEnvelope(row.value,row.now);expect<unknown>(admitted).toEqual(row.value);expect(Object.is(admitted,row.value)).toBe(false);expect(Object.isFrozen(admitted)).toBe(true);}
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

test("unacknowledged derived cancellation refuses terminal success after accepted work settles",async()=>{const api=await import("../📥️invocation/🏃️process/🟦️.ts"),controller=new AbortController();await expect(api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:"fixture:signal-stop",maximumElapsedMilliseconds:1000},{fixture:"signal-stop"},Date.now()),original=>api.withScriptInvocationSignalV1(original,controller.signal,async()=>{controller.abort(Error("unacknowledged stop"));}))).rejects.toThrow("unacknowledged stop");});

test("derived signal stop requires an explicit private acknowledgement with independent SQLite agreement",async()=>{
 const api=await import("../📥️invocation/🏃️process/🟦️.ts"),fixture=(await import("./🧫️fixtures/🛑️signal.json")).default,rows:boolean[]=[];
 for(const row of fixture.cases){let accepted=true;try{await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:"fixture:signal-corpus",maximumElapsedMilliseconds:2000},{fixture:"signal-corpus"},Date.now()),original=>{const controller=new AbortController();return api.withScriptInvocationSignalV1(original,controller.signal,async alias=>{if(row.additionalAborted)controller.abort(Error("owned stop"));if(row.parentAborted)process.emit("SIGINT");if(row.acknowledged)api.acknowledgeScriptInvocationSignalStopV1(alias);});});}catch{accepted=false;}expect(accepted).toBe(row.accepted);rows.push(accepted);}
 const{captureOwnedCommandV1}=await import("../../🎛️owned-execution/🟦️.ts"),oracle="import sqlite3,json,sys;d=sqlite3.connect(':memory:');rows=json.loads(sys.argv[1]);print(json.dumps([bool(d.execute('SELECT NOT ? AND ((NOT ? AND NOT ?) OR (? AND ?))',(r['parentAborted'],r['additionalAborted'],r['acknowledged'],r['additionalAborted'],r['acknowledged'])).fetchone()[0]) for r in rows]))";
 await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:"fixture:signal-oracle",maximumElapsedMilliseconds:5000},{fixture:"signal-oracle"},Date.now()),async original=>{const output=await captureOwnedCommandV1("python3",["-c",oracle,JSON.stringify(fixture.cases)],process.cwd(),{control:api.createIssuedResourceControlV1(original,"fixture:sqlite-signal",5000),output:{maximumBytes:4096,maximumLines:32}});expect(JSON.parse(output)).toEqual(rows);});console.log("[DEBUG] Actual private signal-stop disposition agrees with independent SQLite for five neutral cases");
});
