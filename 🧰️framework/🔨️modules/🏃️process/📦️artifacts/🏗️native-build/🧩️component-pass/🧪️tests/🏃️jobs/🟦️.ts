import { registerIsolatedBuildTestV1, withBuildTestInvocationV1 } from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import { expect, test } from "bun:test";
import { optimizeComponentCoresV1 } from "../../../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts";
import { Database } from "bun:sqlite";
import { formatStampProtocolV1 } from "../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { type LinkedComponentV1, type NativeBuildStageV1 } from "../../🟦️.ts";
import { type NativeBuildJobV1, type NativeBuildPlanV1 } from "../../📋️contract/🟦️.ts";
import { artifacts, stubCargo, stubWorkspace, runStub, stubJob } from "../🎭️stub/🟦️.ts";

test("jobs start as soon as the jobs they name complete, and post-link stages stream per component after the job they name", async () => {
  const root = mkdtempSync(join(artifacts, "j-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const checked: Record<string, number> = {};
  const plan: NativeBuildPlanV1 = { version: 1, passes: [pass], jobs: [stubJob(root, "tool", [], 50), stubJob(root, "pages", ["job:tool"], 10)] };
  let units: any, report = "";
  const outcome = await runStub(root, plan, cargo, { stages: (component) => [{ name: "check", capacity: "single", after: ["job:tool"], run: async () => { checked[component.package.name] = Date.now(); } }] }, "ok", ({ outcome: run }) => { if (!run) return; units = JSON.parse(readFileSync(join(run.generation.report, "units.json"), "utf8")); report = readFileSync(run.report, "utf8"); });
  expect(outcome.failed).toEqual([]);
  const jobs = readFileSync(join(root, "jobs.jsonl"), "utf8").trim().split("\n").map((line) => JSON.parse(line)), byId = Object.fromEntries(jobs.map((row) => [row.id, row]));
  const calls = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line)), passStart = calls.find((row) => row.at).at;
  expect(byId.tool.start).toBeLessThan(passStart + 2000);
  for (const name of ["comp-a", "comp-b"]) expect(checked[name]).toBeGreaterThanOrEqual(byId.tool.end);
  expect(byId.pages.start).toBeGreaterThanOrEqual(byId.tool.end);
  for (const row of jobs) { expect(row.jobserver).toMatch(/--jobserver-auth=(?:fifo:|semio-jobserver-)/u); expect(row.pass).toBe(row.id); }
  expect(units.stages.filter((stage: any) => stage.stage === "check").map((stage: any) => stage.component).sort()).toEqual(["comp-a", "comp-b"]);
  expect(report).toContain("## Stage cache");
}, 120_000);

test("reserved nested compiler processes receive held worker slots and release them before the next compiler", async () => {
  const root = mkdtempSync(join(artifacts, "b-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release");
  const jobs: NativeBuildJobV1[] = [
    { ...stubJob(root, "reserved", [], 20), lane: "release", workerProtocol: "reserved" },
    { ...stubJob(root, "shared", [], 20), lane: "release" },
  ];
  const outcome = await runStub(root, { version: 1, passes: [pass], jobs }, cargo);
  expect(outcome.failed).toEqual([]);
  const [reserved, shared] = readFileSync(join(root, "jobs.jsonl"), "utf8").trim().split("\n").map((line) => JSON.parse(line));
  expect(reserved.parallelism).toBe(fixture.stub.tokens);
  expect(reserved.capacityHeld).toBeUndefined();
  expect(reserved.jobserver).toBeNull();
  expect(reserved.makeflags).toBeNull();
  expect(shared.parallelism).toBe(fixture.stub.tokens);
  expect(shared.jobserver).toMatch(/--jobserver-auth=(?:fifo:|semio-jobserver-)/u);
  expect(shared.start).toBeGreaterThanOrEqual(reserved.end);
  console.log(`[component-pass] reserved ${reserved.parallelism} workers; shared successor and Cargo completed`);
}, 120_000);

test("exclusive compiler jobs and stages hold the whole pool without deadlocking concurrent requests", async () => {
  const root = mkdtempSync(join(artifacts, "x-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const jobs = ["a", "b"].map((id) => ({ ...stubJob(root, id, [], 15), workerProtocol: "exclusive" as const }));
  const stages: { start: number; end: number; workers: number; mode: string | undefined }[] = [];
  const outcome = await runStub(root, { version: 1, passes: [pass], jobs }, cargo, { stages: () => [{ name: "whole-host", capacity: "whole-host", after: [], run: async (context) => {
    const start = Date.now();
    expect(context.environment.SEMIO_COMPILER_CAPACITY_HELD).toBeUndefined();
    expect(context.environment.CARGO_MAKEFLAGS).toBeUndefined();
    await Bun.sleep(15);
    stages.push({ start, end: Date.now(), workers: Number(context.environment.SEMIO_COMPILER_PARALLELISM), mode: context.environment.SEMIO_BUILD_MODE });
  } }] });
  expect(outcome.failed).toEqual([]);
  const children = readFileSync(join(root, "jobs.jsonl"), "utf8").trim().split("\n").map((line) => JSON.parse(line));
  expect(children).toHaveLength(2);
  for (const child of children) { expect(child.parallelism).toBe(fixture.stub.tokens); expect(child.jobserver).toBeNull(); }
  expect(children[1].start).toBeGreaterThanOrEqual(children[0].end);
  expect(stages).toHaveLength(2);
  for (const stage of stages) { expect(stage.workers).toBe(fixture.stub.tokens); expect(stage.mode).toBe("release"); }
  expect(stages[1]!.start).toBeGreaterThanOrEqual(stages[0]!.end);
  console.log(`[component-pass] exclusive jobs=2 stages=2 workers=${fixture.stub.tokens}; shared Cargo completed`);
}, 120_000);

registerIsolatedBuildTestV1(import.meta.path, "a component Cargo did not link fails every stage that waits for it and fails the release", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "k-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const ran: string[] = [];
  await expect(runStub(root, { version: 1, passes: [pass] }, cargo, { stages: (component) => [{ name: "check", capacity: "single", after: [], run: async () => { ran.push(component.package.name + ":check"); } }, { name: "publish", capacity: "single", after: ["check"], run: async () => { ran.push(component.package.name + ":publish"); } }] }, "omit-last")).rejects.toThrow(/Native build failed: comp-b/u);
  expect(ran.sort()).toEqual(["comp-a:check", "comp-a:publish"]);
  checkpoint();
}, /Native build failed: comp-b/u);

test("a lane job runs inside its profile's Cargo lane before that lane's passes, and an idle stage widens onto free slots", async () => {
  const root = mkdtempSync(join(artifacts, "l-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release");
  const widths: number[] = [];
  const plan: NativeBuildPlanV1 = { version: 1, passes: [pass], jobs: [{ ...stubJob(root, "module", [], 200), lane: "release", profile: "release", target: "wasm32-unknown-unknown" }] };
  const outcome = await runStub(root, plan, cargo, { stages: () => [{ name: "wide", capacity: "available", after: [], run: async (context) => { widths.push(context.parallelism); } }] });
  expect(outcome.failed).toEqual([]);
  const module = JSON.parse(readFileSync(join(root, "jobs.jsonl"), "utf8").trim()), passStart = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line)).find((row) => row.at).at;
  expect(module.end).toBeLessThanOrEqual(passStart);
  expect(widths).toHaveLength(1);
  expect(widths[0]).toBeGreaterThanOrEqual(1);
  expect(widths[0]).toBeLessThanOrEqual(3);
}, 120_000);

registerIsolatedBuildTestV1(import.meta.path, "a finalizer runs once its stages settled, also after one failed, and the failure still fails the release", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "z-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const law=fixture.finalizerSettlement, failedAt:Record<string,number>={}, events:string[]=[];
  const assemblyError=Error(law.primaryError);
  const stages = (component: LinkedComponentV1): readonly NativeBuildStageV1[] => {
    const name=component.package.name;
    const row=law.components.find(row=>row.name===name)!;
    return [
      { name: "assemble", capacity: "single", after: [], run: async () => {
        events.push(name+":assemble");
        if (!row.assembleSucceeds) { failedAt[name]=Date.now(); throw assemblyError; }
      } },
      { name: "publish", capacity: "single", after: ["assemble"], run: async () => { events.push(name+":publish"); } },
      { name: "discard", capacity: "cleanup", after: ["publish"], run: async (context) => {
        expect(context.signal.aborted).toBe(false);
        expect(context.parallelism).toBe(0);
        if (!row.assembleSucceeds) expect(Date.now()).toBeGreaterThanOrEqual(failedAt[name]!);
        events.push(name+":discard");
      } },
    ];
  };
  const failure=await runStub(root,{version:1,passes:[pass]},cargo,{stages,verifyStageWriters:true}).then(()=>undefined,error=>error);
  expect(failure?.message).toMatch(/Native build failed: comp-b \(assemble\)/u);
  expect(failure?.cause).toBe(assemblyError);
  const oracle=new Database(":memory:");
  try {
    oracle.exec("create table component(name text, assembled integer);");
    for (const row of law.components) oracle.query("insert into component values(?,?)").run(row.name,Number(row.assembleSucceeds));
    const expected=oracle.query<{event:string},[]>("select name||':assemble' as event from component union all select name||':publish' from component where assembled=1 union all select name||':discard' from component order by event").all();
    expect(events.sort()).toEqual(expected.map(row=>row.event));
  } finally { oracle.close(); }
  checkpoint();
}, /Native build failed: comp-b/u);

registerIsolatedBuildTestV1(import.meta.path,"a blocked browser release drains real optimize before deleting its work",{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
  const vector=fixture.optimizerDrain,root=mkdtempSync(join(artifacts,"d-")),cargo=stubCargo(root),pass=stubWorkspace(root,"hub",[vector.root],"release");
  const failure=Error(vector.failure),events:string[]=[],logs:string[]=[];
  const stored=Promise.withResolvers<void>(),modelFailed=Promise.withResolvers<void>(),discarded=Promise.withResolvers<void>();
  await withBuildTestInvocationV1(async invocation=>{
    const result=await runStub(root,{version:1,passes:[pass]},cargo,{
      log(line){logs.push(line);if(line.includes(vector.failure)){events.push("model-failed");modelFailed.resolve();}},
      stages(component){
        let workspace:string,transpiled:string;
        const crate=component.package.name,file=crate.replaceAll("-","_")+".core.wasm";
        return [
          {name:"transpile",after:[],capacity:"single",async run(context){
            workspace=join(context.generation.root,"w",crate);
            transpiled=join(workspace,"transpiled");
            mkdirSync(transpiled,{recursive:true});
            writeFileSync(join(transpiled,file),Buffer.from(vector.core));
          }},
          {name:"model",after:[`provenance:${component.pass.id}`],capacity:"io",async run(context){
            await context.provenance();
            await stored.promise;
            throw failure;
          }},
          {name:"optimize",after:["transpile"],capacity:"parallel",async run(context){
            const cache:typeof context.cache={...context.cache,run(key,output,produce,signal,slot){
              return context.cache.run(key,output,async directory=>{
                await produce(directory);
                events.push("cas-ready");
                stored.resolve();
                await modelFailed.promise;
                await Promise.race([discarded.promise,Bun.sleep(150)]);
              },signal,slot);
            }};
            const target={pluginId:crate,packageId:crate,cratePath:root,packageName:crate,wasmOut:component.staged,role:"plugin" as const,consumes:[],dependsOn:[]};
            const deliverable={profile:"release" as const,directory:dirname(component.staged),wasm:component.staged,receipt:join(dirname(component.staged),"receipt.json")};
            const output=await optimizeComponentCoresV1({invocation,repoRoot:process.cwd(),target,deliverable,cache,work:workspace,environment:context.environment,control:context.control},transpiled,context.parallelism);
            expect(output).toBe(join(workspace,"optimized"));
            expect(WebAssembly.validate(readFileSync(join(output!,file)))).toBe(true);
            events.push("optimize-finished");
          }},
          {name:"browser",after:["model","optimize"],capacity:"single",async run(){events.push("browser");}},
          {name:"complete",after:["browser"],capacity:"io",async run(){events.push("complete");}},
          {name:"discard",after:["complete"],capacity:"cleanup",async run(){
            rmSync(workspace,{recursive:true,force:true});
            events.push("discard");
            discarded.resolve();
          }},
        ];
      },
    },"observe-linked").then(()=>undefined,error=>error);
    expect(result?.cause).toBe(failure);
    expect(logs.filter(line=>line.includes("ENOENT"))).toEqual([]);
    const oracle=new Database(":memory:");
    try{
      const ordered=oracle.query<{event:string},[string]>("select value as event from json_each(?) order by key");
      expect(events).toEqual(ordered.all(JSON.stringify(vector.events)).map(row=>row.event));
    }finally{oracle.close();}
  });
  checkpoint();
},/Native build failed: opt-drain \(model\)/u);

registerIsolatedBuildTestV1(import.meta.path, "a job failing at once rejects the driver, naming it, never starts the job after it and terminates the sibling job still running", { bodyMilliseconds: 60_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "ff-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release"), pids = join(root, "long.pid");
  const long = stubJob(root, "long", [], 0);
  writeFileSync(long.command[1]!, `import { writeFileSync } from "node:fs";\nwriteFileSync(${JSON.stringify(pids)}, String(process.pid));\nawait Bun.sleep(120000);\n`);
  const failing = stubJob(root, "broken", [], 0, 3);
  const started = performance.now();
  await expect(runStub(root, { version: 1, passes: [pass], jobs: [long, failing, stubJob(root, "pages", ["job:broken"], 0)] }, cargo)).rejects.toThrow(/job failed.*job:broken/su);
  expect(existsSync(join(root, "jobs.jsonl")) ? readFileSync(join(root, "jobs.jsonl"), "utf8") : "").not.toContain('"pages"');
  expect(performance.now() - started).toBeLessThan(30_000);
  if (existsSync(pids)) { const pid = Number(readFileSync(pids, "utf8")); await Bun.sleep(300); expect(() => process.kill(pid, 0)).toThrow(); }
  checkpoint();
}, /./su);

test("a job that exits with the shared restart status runs again once with the shared backoff instead of failing the build", async () => {
  const root = mkdtempSync(join(artifacts, "rs-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release"), marker = join(root, "restarted");
  const restarting = stubJob(root, "restarting", [], 0);
  writeFileSync(restarting.command[1]!, `import { existsSync, writeFileSync, appendFileSync } from "node:fs";\nappendFileSync(${JSON.stringify(join(root, "starts.log"))}, "x");\nif (!existsSync(${JSON.stringify(marker)})) { writeFileSync(${JSON.stringify(marker)}, ""); process.exit(${formatStampProtocolV1().exitCode}); }\n`);
  const outcome = await runStub(root, { version: 1, passes: [pass], jobs: [restarting] }, cargo);
  expect(outcome.failed).toEqual([]);
  expect(readFileSync(join(root, "starts.log"), "utf8")).toBe("xx");
}, 60_000);
