import {withInstalledNxFixtureV1} from "../../🔒️nx/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "@semio-tech/process/leases";
import {runOwnedCommand} from "@semio-tech/process/execution";
import {scriptInvocationEnvironment,requireIssuedResourceControlInvocationV1,createRequestedIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, existsSync, symlinkSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { setTimeout as delay } from "node:timers/promises";

/** 🔐️ Exercises the installed Nx cache against a real source write while its child is running. */
export async function testNxSourceCacheCustody(workspace: string, output: string, invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);
  output=resolve(output);
  const stop=new AbortController(),scopeControl=phaseControl,pending=new Set<Promise<void>>();let failed=false,failure:unknown;
  const row = JSON.parse(readFileSync(new URL("../../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).cacheCustody;
  const root = mkdtempSync(join(output, "s-"));
  try {
   await withInstalledNxFixtureV1(workspace,{kind:"external",root},invocation,scopeControl,async(_root,environment,productOutput,actualPhase,toolkitRoot)=>{
    const cli=join(toolkitRoot,"node_modules/nx/dist/bin/nx.js"),childPhase=createRequestedIssuedResourceControlV1(actualPhase,0,stop.signal);
    try{
    mkdirSync(join(root, ".nx"), { recursive: true });
    symlinkSync(join(workspace, ".nx/installation"), join(root, ".nx/installation"), process.platform === "win32" ? "junction" : "dir");
    symlinkSync(join(workspace, "🧰️framework"), join(root, "🧰️framework"), process.platform === "win32" ? "junction" : "dir");
    const contract = JSON.stringify({ version: 1, files: ["source.json", "📜️script.ts"] }), key = createHash("sha256").update(contract).digest("hex"), inventory = join(root, ".🧬semio/🦑️repo/⚡️cache/build-input-closures");
    mkdirSync(inventory, { recursive: true }); writeFileSync(join(inventory, key + ".json"), contract);
    writeFileSync(join(root, "nx.json"), JSON.stringify({ useDaemonProcess: false, plugins: [], tasksRunnerOptions: { default: { runner: "nx/tasks-runners/default", options: {} } } }));
    writeFileSync(join(root, "package.json"), JSON.stringify({ name: "source-custody-fixture", private: true, scripts: { build: "nx run fixture:build" } }));
    writeFileSync(join(root, "project.json"), JSON.stringify({ name: "fixture", root: ".", targets: { build: { executor: "nx:run-commands", cache: true, inputs: [{ env: "SEMIO_SOURCE_CLOSURE_" + key }], outputs: [productOutput], options: { command: "bun ./📜️script.ts build", cwd: "." } } } }));
    writeFileSync(join(root, "source.json"), JSON.stringify({ value: row.initial }));
    writeFileSync(join(root, "📜️script.ts"), `#!/usr/bin/env bun
import {existsSync,writeFileSync,readFileSync,mkdirSync} from "node:fs";
import {receiveScriptProcessInvocation,checkScriptInvocation} from ${JSON.stringify(join(workspace,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"))};
await receiveScriptProcessInvocation(process.env,async original=>{
checkScriptInvocation(original);
writeFileSync("received.json",JSON.stringify({wire:process.env.SEMIO_SCRIPT_PROCESS_INVOCATION}));
if(process.argv[2]!=="build")throw Error("Expected build");
writeFileSync("ready","ready");
const deadline=Date.now()+30000;while(!existsSync("continue")){checkScriptInvocation(original);if(Date.now()>deadline)throw Error("Fixture deadline");await Bun.sleep(20);}
checkScriptInvocation(original);const count=existsSync("count")?Number(readFileSync("count","utf8")):0;writeFileSync("count",String(count+1));
mkdirSync(${JSON.stringify(productOutput)},{recursive:true});writeFileSync(${JSON.stringify(join(productOutput,"result.json"))},readFileSync("source.json"));console.log("[DEBUG] fixture compiler executed");
});
`);
    const launch = (suffix: string) => {
      const control=createRequestedIssuedResourceControlV1(childPhase,60000),env:NodeJS.ProcessEnv=scriptInvocationEnvironment(invocation,{...process.env,...environment,NX_SKIP_NX_CACHE:"false",NX_DISABLE_NX_CACHE:"false",NX_SKIP_REMOTE_CACHE:"true",NX_DAEMON:"false",NX_ISOLATE_PLUGINS:"false",NX_FORCE_REUSE_CACHED_GRAPH:"false"});delete env.SEMIO_BUILD_GRAPH_PATH;delete env.SEMIO_BUILD_GRAPH_DIGEST;delete env.SEMIO_BUILD_REUSE_GRAPH;
      let log="",finished=false;const remaining=checkResourceLeaseControl(control),done=(async()=>{let failed=false,failure:unknown;try{await runOwnedCommand("node",[cli,"run","fixture:build","--output-style=stream","--verbose"],root,"test:nx-source-custody",Math.min(60000,Math.floor(remaining??60000)),{env,signal:control.signal,stdout:"ignore",output:{maximumBytes:8388608,maximumLines:65536},onLine:line=>{log+=line+"\n";},onProgress:async()=>{checkResourceLeaseControl(control);await control.publish({resource:"test:nx-source-custody",mode:"shared",elapsedMs:0});await control.yieldContinuation();checkResourceLeaseControl(control);}});}catch(error){failed=true;failure=error;}try{writeFileSync(join(output,`source-custody-${suffix}.log`),log);}catch(error){if(!failed){failed=true;failure=error;}}if(failed)throw failure;})().finally(()=>{finished=true;});pending.add(done);void done.then(()=>pending.delete(done),()=>pending.delete(done));void done.catch(()=>{});return{done,finished:()=>finished,output:()=>log};
    };
    const first = launch("edited"), deadline = Date.now() + 45000;
    while (!existsSync(join(root, "ready"))) { if (Date.now() > deadline || first.finished()) { await first.done; throw Error("Fixture child did not start"); } await delay(20); }
    writeFileSync(join(root, "source.json"), JSON.stringify({ value: row.during })); writeFileSync(join(root, "continue"), "continue");
    await first.done;
    assert.equal(JSON.parse(readFileSync(join(root,"received.json"),"utf8")).wire,scriptInvocationEnvironment(invocation,{}).SEMIO_SCRIPT_PROCESS_INVOCATION);
    assert.match(first.output(), /cache promotion withheld/);
    await launch("fresh").done;
    await launch("warm").done;
    assert.equal(Number(readFileSync(join(root, "count"), "utf8")), row.expectedExecutions, "Edited execution must not become a stale cache hit; the unchanged third execution must hit");
    assert.equal(JSON.parse(readFileSync(join(productOutput, "result.json"), "utf8")).value, row.during);
    }catch(error){failed=true;failure=error;}finally{stop.abort(Error("Source custody fixture closing"));const settled=await Promise.allSettled([...pending]);for(const row of settled)if(row.status==="rejected"&&!failed){failed=true;failure=row.reason;}}if(failed)throw failure;
   });
  }catch(error){failed=true;failure=error;}finally{stop.abort(Error("Source custody fixture closing"));const settled=await Promise.allSettled([...pending]);for(const row of settled)if(row.status==="rejected"&&!failed){failed=true;failure=row.reason;}try{rmSync(root,{recursive:true,force:true});}catch(error){if(!failed){failed=true;failure=error;}}}if(failed)throw failure;
}
