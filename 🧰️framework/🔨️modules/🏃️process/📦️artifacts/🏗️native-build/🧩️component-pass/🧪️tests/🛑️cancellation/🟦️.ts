import { testPythonV1 } from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import { withBuildTestInvocationV1 } from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../../🎛️owned-execution/🟦️.ts";
import { scriptInvocationEnvironment } from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import { lawSourceBoundaryV1, lawStoragePolicyV1 } from "../../../🧪️tests/🦀️cargo/🟦️.ts";
import { type NativeBuildJobV1 } from "../../📋️contract/🟦️.ts";
import { artifacts, stubCargo, stubWorkspace, runStub, stubJob } from "../🎭️stub/🟦️.ts";

test("queued exclusive compiler cancellation drains rejected work and permits a successor",async()=>{
 const root=mkdtempSync(join(artifacts,"q-")),cargo=stubCargo(root),pass=stubWorkspace(root,"hub",["comp-a"],"release"),law=fixture.queuedExclusiveCancellation;
 const holder={...stubJob(root,"holder",[],law.holdMs),workerProtocol:"reserved" as const},queued={...stubJob(root,"queued",[],0),workerProtocol:"exclusive" as const};
 const calls=join(root,"started.jsonl");writeFileSync(holder.command[1]!,`import {appendFileSync} from "node:fs";appendFileSync(${JSON.stringify(calls)},JSON.stringify({id:"holder",pid:process.pid})+"\\n");await Bun.sleep(${law.holdMs});`);
 const script=join(root,"h","📜️script.ts");mkdirSync(dirname(script));
 const options={plan:{version:1,passes:[pass],jobs:[holder,queued]},cwd:root,generation:"warm",leaseDirectory:artifacts,environment:{...process.env,STUB_MODE:"ok"},tokens:fixture.stub.tokens,physicalBytes:fixture.stub.physicalGiB*1024**3,storageReserveBytes:lawStoragePolicyV1.reserveBytes,scheduler:process.execPath,cargo,sourceBoundary:lawSourceBoundaryV1};
 const source=new URL("../../🟦️.ts",import.meta.url).href;
 writeFileSync(script,`import {runNativeBuildDriverV1} from ${JSON.stringify(source)};import {withLawStorageV1,lawStoragePolicyV1} from ${JSON.stringify(new URL("../../../🧪️tests/🦀️cargo/🟦️.ts",import.meta.url).href)};import {scriptCompilerAdmissionControlV1} from ${JSON.stringify(new URL("../../../../../🏗️build/🎟️capacity/🟦️.ts",import.meta.url).href)};import {receiveScriptProcessInvocation} from ${JSON.stringify(new URL("../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts",import.meta.url).href)};import {existsSync,readFileSync} from "node:fs";import {join} from "node:path";const unhandled=[];process.on("unhandledRejection",error=>unhandled.push(String(error)));const options=${JSON.stringify(options)};await receiveScriptProcessInvocation(process.env,async invocation=>{const run=(signal,extra)=>withLawStorageV1(scriptCompilerAdmissionControlV1(invocation,"law:queued-exclusive",30000,signal),lawStoragePolicyV1.stubImageBytes,(held,control)=>runNativeBuildDriverV1({...options,...extra,storage:held.storage,cacheRoot:join(held.root,"cache"),shared:{target:join(held.root,"target"),build:join(held.root,"build")},environment:{...options.environment,...held.environment,CARGO_TARGET_DIR:join(held.root,"target")},control,log:()=>undefined}),invocation);const controller=new AbortController();const cancel=setInterval(()=>{if(existsSync(${JSON.stringify(calls)})){clearInterval(cancel);controller.abort(Error("cancel queued exclusive by law"));}},${law.pollMs});let refused=false;try{await run(controller.signal,{});}catch{refused=true;}finally{clearInterval(cancel);}await Bun.sleep(${law.settleMs});const successor=await run(new AbortController().signal,{plan:{version:1,passes:[options.plan.passes[0]],jobs:[]}});console.log(JSON.stringify({refused,unhandled,children:JSON.parse(readFileSync(${JSON.stringify(calls)},"utf8").trim()).id,resumed:successor.failed.length===0}));});`);
 const {output,diagnostic,exit}=await withBuildTestInvocationV1(async original=>{const child=Bun.spawn([process.execPath,script],{cwd:root,env:scriptInvocationEnvironment(original,process.env),stdout:"pipe",stderr:"pipe"}),[output,diagnostic]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text()]);return {output,diagnostic,exit:await child.exited};});expect(exit,diagnostic).toBe(0);
 const result=JSON.parse(output.trim().split("\n").at(-1)!);expect(result.refused).toBe(true);expect(result.unhandled).toHaveLength(law.expectedUnhandled);expect([result.children]).toEqual(law.expectedChildren);expect(result.resumed).toBe(law.resumed);expect(existsSync(join(root,"jobs.jsonl"))).toBe(false);
 const pid=JSON.parse(readFileSync(calls,"utf8").trim()).pid;expect(()=>process.kill(pid,0)).toThrow();
},120_000);

test("unit graph compilation cancellation stops its metadata child and releases admission",async()=>{
 const root=mkdtempSync(join(artifacts,"ug-")),cargo=stubCargo(root),pass=stubWorkspace(root,"metadata",["unit"],"release"),controller=new AbortController();
 const running=runStub(root,{version:1,passes:[pass]},cargo,{cancel: { signal: controller.signal, milliseconds: 30000 }},"unit-hang");
 const calls=join(root,"unit-calls.jsonl");while(!existsSync(calls))await new Promise(resolve=>setTimeout(resolve,10));
 const pid=JSON.parse(readFileSync(calls,"utf8").split("\n")[0]!).pid;controller.abort(Error("metadata cancellation"));try{await running;}catch(error){expect(controller.signal.aborted).toBe(true);}
 await new Promise(resolve=>setTimeout(resolve,50));let live=true;try{process.kill(pid,0);}catch{live=false;}
 if(live)process.kill(pid,"SIGKILL");expect(live).toBe(false);
 expect((await runStub(root,{version:1,passes:[pass]},cargo)).staged).toEqual(["unit:release"]);
},120000);

test("pending stages and finalizers keep the caller's issued control and settle under its finite deadline without aborting the caller's signal", async () => {
  const law = fixture.originalStageAuthority;
  for (const stage of law.stages) {
    const root = mkdtempSync(join(artifacts, "a-")), cargo = stubCargo(root), pass = stubWorkspace(root, "authority", ["unit"], "prototype"), controller = new AbortController();
    let entered = false;
    const safety = setTimeout(() => controller.abort(Error("Test safety cancellation")), law.safetyMilliseconds);
    try {
      await expect(runStub(root, { version: 1, passes: [pass] }, cargo, { cancel: { signal: controller.signal, milliseconds: law.milliseconds }, stages: () => [{ name: "authority", capacity: stage === "finalizer"?"cleanup":"single", after: [], run: async (context) => {
        entered = true;
        expect(context.control.signal).toBe(context.signal);
        expect(context.signal.aborted).toBe(false);
        const remaining = context.control.remainingMilliseconds()!;
        expect(remaining).toBeGreaterThan(0);
        expect(remaining).toBeLessThan(law.milliseconds);
        await context.control.publish({ resource: "native-stage-law", mode: "exclusive", elapsedMs: 0 });
        await context.control.yieldContinuation();
        if (stage === "owned-child") return captureOwnedCommandV1(process.execPath, ["-e", `import {writeFileSync} from "node:fs";writeFileSync(${JSON.stringify(join(root, "owned-stage.json"))},JSON.stringify({pid:process.pid}));await new Promise(()=>setInterval(()=>{},1000));`], root, { control: context.control }).then(() => undefined);
        await new Promise<void>(() => undefined);
      } }] })).rejects.toThrow(new RegExp(law.expectedError, "u"));
    } finally { clearTimeout(safety); }
    expect(entered).toBe(true);
    expect(controller.signal.aborted).toBe(law.expectedSignalAborted);
    if (stage === "owned-child") {
      await Bun.sleep(law.ownedChildSettleMilliseconds);
      const pid = JSON.parse(readFileSync(join(root, "owned-stage.json"), "utf8")).pid;
      let live = true;
      try { process.kill(pid, 0); } catch { live = false; }
      if (live) process.kill(pid, "SIGKILL");
      expect(live).toBe(false);
    }
    expect((await runStub(root, { version: 1, passes: [pass] }, cargo)).failed).toEqual([]);
  }
}, 240_000);

test("the original finite caller budget stops running metadata and returns admission", async () => {
  const vector = fixture.finiteCallerBudget, root = mkdtempSync(join(artifacts, "d-")), cargo = stubCargo(root), pass = stubWorkspace(root, "deadline", ["unit"], "prototype"), controller = new AbortController();
  const safety = setTimeout(() => controller.abort(Error("Test safety cancellation")), vector.safetyMilliseconds);
  try {
    await expect(runStub(root, { version: 1, passes: [pass] }, cargo, { cancel: { signal: controller.signal, milliseconds: vector.milliseconds } }, "unit-hang")).rejects.toThrow(vector.expectedError);
  } finally { clearTimeout(safety); }
  const pid = JSON.parse(readFileSync(join(root, "unit-calls.jsonl"), "utf8").split("\n")[0]!).pid;
  expect(() => process.kill(pid, 0)).toThrow();
  expect(controller.signal.aborted).toBe(false);
  expect((await runStub(root, { version: 1, passes: [pass] }, cargo)).failed).toEqual([]);
}, 90_000);

test("unlaned same-profile Cargo jobs reserve tokens before mutable state", async () => {
  const vector = fixture.unlanedStateOrder, root = mkdtempSync(join(artifacts, "o-")), cargo = stubCargo(root), pass = stubWorkspace(root, "w", ["order-component"], vector.profile), marker = join(root, "job.json"), script = join(root, "job", "📜️script.ts");
  mkdirSync(dirname(script), { recursive: true });
  const leaseModule = new URL("../../../🔒️lease/🟦️.ts", import.meta.url).href, resourceModule = new URL("../../../../../🔒️leases/🟦️.ts", import.meta.url).href;
  writeFileSync(script, `import { acquireCargoBuildLeaseV1 } from ${JSON.stringify(leaseModule)};
import { resourceLeasePathV1 } from ${JSON.stringify(resourceModule)};
import { scriptCompilerAdmissionControlV1 } from ${JSON.stringify(new URL("../../../../../🏗️build/🎟️capacity/🟦️.ts", import.meta.url).href)};
import { receiveScriptProcessInvocation } from ${JSON.stringify(new URL("../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts", import.meta.url).href)};
import { spawnSync } from "node:child_process";
await receiveScriptProcessInvocation(process.env, async invocation => {
const control = scriptCompilerAdmissionControlV1(invocation, "law:unlaned-job", ${vector.deadlineMs - 1000});
const lease = await acquireCargoBuildLeaseV1({ directory: ${JSON.stringify(artifacts)}, buildDirectory: process.env.CARGO_BUILD_BUILD_DIR, targetDirectory: process.env.CARGO_TARGET_DIR, args: ["--profile", ${JSON.stringify(vector.profile)}], signal: control.signal, control });
try {
 const oracle = spawnSync(${JSON.stringify(testPythonV1() ?? "python3")}, ["-c", "import sqlite3,sys;d=sqlite3.connect(sys.argv[1],timeout=0);\\ntry:d.execute('begin exclusive');print('unheld')\\nexcept sqlite3.OperationalError:print('held')", resourceLeasePathV1(${JSON.stringify(artifacts)}, lease.resource)], { encoding: "utf8" });
 if (oracle.status !== 0 || oracle.stdout.trim() !== ${JSON.stringify(vector.oracle)}) throw Error("Independent state lease oracle disagrees: " + oracle.stderr);
 await Bun.write(${JSON.stringify(marker)}, JSON.stringify({ participant: "job", oracle: oracle.stdout.trim() }));
} finally { lease.release(); }
});
`);
  const job: NativeBuildJobV1 = { id: "same-state", cwd: root, command: [process.execPath, script], after: [], profile: vector.profile, target: "wasm32-wasip2", workerProtocol: "jobserver" };
  expect(new Ajv({ strict: true, allowUnionTypes: true }).compile(schema)({ version: 1, passes: [pass], jobs: [job] })).toBe(true);
  const result = await runStub(root, { version: 1, passes: [pass], jobs: [job] }, cargo, { tokens: vector.tokens, cancel: { signal: AbortSignal.timeout(vector.deadlineMs), milliseconds: vector.deadlineMs } });
  expect(result.staged).toEqual(["order-component:" + vector.profile]);
  const observed = JSON.parse(readFileSync(marker, "utf8"));
  expect(observed.oracle).toBe(vector.oracle);
  expect([observed.participant, ...(result.staged.length ? ["pass"] : [])].sort()).toEqual(vector.expectedParticipants);
}, 120_000);
