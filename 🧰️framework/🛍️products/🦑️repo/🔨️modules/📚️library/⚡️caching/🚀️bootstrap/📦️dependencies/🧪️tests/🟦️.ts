import "../💾️storage/🧪️tests/🌐️platform/🟦️.ts";
import "../💾️storage/🧪️tests/📤️diagnostics/🟦️.ts";
import {boundedNxNamespaceV1} from "../../../💾️storage/🔒️enforcement/🔌️nx/🟦️.ts";
/** 📦️ Portable locked-install and owner-requested lock refresh command laws. */
import { expect, test } from "bun:test";
import { openSync, readFileSync, mkdirSync, mkdtempSync, writeFileSync, renameSync, rmSync } from "node:fs";
import { resolve, join, dirname, isAbsolute } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import { build, transform } from "esbuild";
import glob from "fast-glob";
import * as bunOwnership from "../../../../🗂️workspaces/🟦️bun/🟦️.ts";
import { createSourceFile, isFunctionDeclaration, parseConfigFileTextToJson, ScriptTarget } from "typescript";
import {prepareJavascriptDependencies,javascriptDependencyProducerArgumentsV1} from "../💾️storage/🟦️.ts";
import dependencyInstallSchema from "../💾️storage/🧬️schema/📥️install.json"with{type:"json"};
import {SyncScript,RefreshLockScript} from "../🧪️tests/🔬️script-commands/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { bunDistribution, bunArchiveExecutable } from "../../🛠️tools/🟦️bun/🟦️.ts";
import { zipSync, unzipSync } from "fflate";
import { activateWhenNxWatcherReady, waitForNxTargetStart, nxChildEnvironment, nxWatcherEnvironment, NxOutputSpoolV1 } from "../../📜️script.ts";
import { EventEmitter } from "node:events";
import { ChildProcess } from "node:child_process";

const owner = resolve(import.meta.dir, "..");
const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8")) as { operations: { mode: "sync" | "lock"; arguments: string[] }[]; platforms: string[]; rejections: string[][] };

/** 🧭️ Executes neutral output churn while retaining authored topology, manifest and cancellation fences. */
async function outputCensus(root:string,input:any,mutation:string,api:typeof bunOwnership):Promise<any>{
 const base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);if(!isAbsolute(root)||dirname(root)!==base||dirname(root+"-retired")!==base)throw Error("Neutral census root escaped");
 const put=(path:string,value:any):void=>{mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),JSON.stringify(value));};
 put("package.json",{name:"neutral-census",private:true,workspaces:["source/member"],semio:{workspace:{schemaVersion:1,members:input.members,owners:[]}}});
 put("source/member/package.json",{name:"neutral-source",version:"1.0.0"});
 if(mutation!=="generatedBirth")for(const name of input.outputs)put("source/"+name+"/runtime/package.json",{name:"neutral-output-"+input.outputs.indexOf(name),version:"1.0.0"});
 const controller=new AbortController(),visited:string[]=[];let changed=false;
 try{const report=await api.publishBunWorkspaceOwnership(root,{kind:"all"},{signal:controller.signal,advance:async progress=>{
  visited.push(progress.path);
  const at=mutation==="cancel"||mutation==="authoredDirectory"?join(root,"source"):join(root,"source/member/package.json");
  if(!changed&&progress.path===at&&progress.operation===(mutation==="cancel"||mutation==="authoredDirectory"?"list":"readText")){
   changed=true;
   if(mutation==="cancel")controller.abort(Error("Neutral census canceled"));
   else if(mutation==="authoredManifest")put("source/member/package.json",{name:"neutral-changed",version:"1.0.0"});
   else if(mutation==="authoredDirectory"){renameSync(join(root,"source/member"),join(root,"source/retired"));put("source/member/package.json",{name:"neutral-source",version:"1.0.0"});}
   else if(mutation==="authoredRoot"){renameSync(root,root+"-retired");put("package.json",{name:"neutral-census",private:true,workspaces:["source/member"],semio:{workspace:{schemaVersion:1,members:input.members,owners:[]}}});put("source/member/package.json",{name:"neutral-source",version:"1.0.0"});}
   else for(const name of input.outputs){const path=join(root,"source",name);if(dirname(path)!==join(root,"source"))throw Error("Neutral output path escaped");if(mutation==="generatedRemoval"){rmSync(path,{recursive:true,force:true});continue;}put("source/"+name+"/leases/late/package.json",{name:"neutral-late-output",version:"1.0.0"});mkdirSync(join(root,"source",name,"captures"),{recursive:true});}
  }
  await new Promise<void>(done=>setImmediate(done));
 }});return {outcome:"admitted",packages:report.packages,visitedOutputs:input.outputs.filter((name:string)=>visited.some(path=>path===join(root,"source",name)||path.startsWith(join(root,"source",name)+"/"))) };}
 catch(error){return {outcome:controller.signal.aborted?"canceled":"rejected",error:String(error),changed};}
}

test("Bun ownership excludes mutable outputs and preserves authored custody against Node and esbuild",async()=>{
 const censusOwner=resolve(owner,"../../../🗂️workspaces/🟦️bun"),input=JSON.parse(readFileSync(join(censusOwner,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")).outputCensus,schema=JSON.parse(readFileSync(join(censusOwner,"🧬️schema/🔣️.json"),"utf8"));expect(new Ajv({strict:true}).compile(schema.$defs.OutputCensus)(input)).toBe(true);
 const source=createSourceFile("census.ts",readFileSync(import.meta.path,"utf8"),ScriptTarget.Latest),operation=source.statements.find(statement=>isFunctionDeclaration(statement)&&statement.name?.text==="outputCensus")!;
 const compiled=await transform(operation.getText(source),{loader:"ts",format:"esm",target:"es2022"});
 for(const row of input.cases){
  const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"q")),actual=await outputCensus(root,input,row.mutation,bunOwnership);expect(actual.outcome,JSON.stringify(actual)).toBe(row.outcome);
  if(row.outcome==="admitted"){expect(actual.packages).toEqual(["source/member"]);expect(actual.visitedOutputs).toEqual([]);expect(glob.sync("source/**/package.json",{cwd:root,ignore:input.outputs.map((name:string)=>"**/"+name+"/**"),onlyFiles:true}).map(path=>path.replace(/\/package.json$/u,""))).toEqual(actual.packages);}
  else {expect(actual.changed).toBe(true);expect(actual.error).toContain(row.outcome==="canceled"?"Neutral census canceled":"Bun current");}
  const oracleRoot=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"q")),entry='import {mkdirSync,writeFileSync,renameSync,rmSync} from "node:fs";import {join,dirname} from "node:path";import {createRequire} from "node:module";const api=createRequire(import.meta.url)('+JSON.stringify(join(censusOwner,"🟨️.cjs"))+').createBunWorkspaceOwnershipV1({ownershipPath:'+JSON.stringify(join(censusOwner,"../🔣️policy.json"))+',ownershipSource:'+JSON.stringify(readFileSync(join(censusOwner,"../🔣️policy.json"),"utf8"))+'});'+compiled.code+'console.log(JSON.stringify(await outputCensus('+JSON.stringify(oracleRoot)+','+JSON.stringify(input)+','+JSON.stringify(row.mutation)+',api)));';
  const child=Bun.spawn(["node","--input-type=module","-e",entry.replace('import {join,dirname}', 'import {join,dirname,resolve,isAbsolute}')],{stdout:"pipe",stderr:"pipe",signal:new AbortController().signal}),[stdout,stderr,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);expect(status,stderr).toBe(0);const reference=JSON.parse(stdout);expect(reference.outcome).toBe(row.outcome);if(row.outcome==="admitted")expect(reference).toEqual(actual);else expect(reference.changed).toBe(true);
 }
 console.log("[DEBUG] Bun output census and independent esbuild/Node/fast-glob agree: generated birth/churn/removal admitted; authored manifest/directory/root drift and original cancellation refused");
});

test("source watchers have stable independent graph stores and sockets for each activation binding", () => {
  const workspace = resolve(owner), environment:Record<string,string|undefined> = { S_OS_PORT: "6064", PLAYGROUND_LOCKED_EXAMPLE_ID: "🎬️demo",...boundedNxNamespaceV1(workspace,workspace,"a".repeat(64),workspace,null,"d".repeat(64)).environment,SEMIO_NX_TOOLING_IDENTITY:"a".repeat(64),SEMIO_NX_SOCKET_STORAGE:"/c/l/m" }, target = "a:activate-react";
  const a = nxWatcherEnvironment(workspace, environment, target);
  expect(nxWatcherEnvironment(workspace, environment, target)).toEqual(a);
  expect(a.S_OS_PORT).toBe("6064");
  expect(a.NX_WORKSPACE_DATA_DIRECTORY).not.toBe(environment.NX_WORKSPACE_DATA_DIRECTORY);
  const identity = JSON.stringify([workspace.replaceAll("\\", "/"), target, Object.entries(environment).filter(([key]) => /^(?:S_|VITE_|PLAYGROUND_|SEMIO_(?:PLUGIN|RENDERER|BUILD_MODE|NX_TOOLING_IDENTITY|LOCALE|LOCKED_LOCALE|LOCKED_TERMINOLOGY)$)/.test(key)).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0)]);
  const hash = new Bun.CryptoHasher("sha256").update(identity).digest("hex").slice(0, 24);
  expect(a.NX_WORKSPACE_DATA_DIRECTORY.endsWith(hash)).toBe(true);
  for (const b of [nxWatcherEnvironment(workspace, environment, "a:activate-wgpu"), nxWatcherEnvironment(workspace, { ...environment, PLAYGROUND_LOCKED_EXAMPLE_ID: "🎬️demo-session" }, target)]) {
    expect(b.NX_WORKSPACE_DATA_DIRECTORY).not.toBe(a.NX_WORKSPACE_DATA_DIRECTORY);
    expect(b.NX_SOCKET_DIR).not.toBe(a.NX_SOCKET_DIR);
  }
});

test("source watcher namespace retains the exact pinned graph dimension",()=>{
 const workspace=resolve(owner),tooling="a".repeat(64),rows=JSON.parse(readFileSync(join(owner,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")).graphBindings,seen=new Set<string>(),sockets=new Set<string>();
 for(const graphIdentity of rows){const environment={...boundedNxNamespaceV1(workspace,workspace,tooling,workspace,graphIdentity,"d".repeat(64)).environment,SEMIO_NX_TOOLING_IDENTITY:tooling,SEMIO_NX_SOCKET_STORAGE:"/c/l/m"},watcher=nxWatcherEnvironment(workspace,environment,"a:activate-react");expect(watcher.SEMIO_BUILD_GRAPH_DIGEST).toBe(graphIdentity??undefined);expect(nxWatcherEnvironment(workspace,environment,"a:activate-react")).toEqual(watcher);seen.add(watcher.NX_WORKSPACE_DATA_DIRECTORY);sockets.add(watcher.NX_SOCKET_DIR);expect(()=>nxWatcherEnvironment(workspace,{...environment,SEMIO_BUILD_GRAPH_DIGEST:graphIdentity===null?"b".repeat(64):undefined},"a:activate-react")).toThrow("base namespace");}
 expect(seen.size).toBe(rows.length);expect(sockets.size).toBe(rows.length);
});

test("initial work starts before watcher readiness and Node observes the same activation ordering", async () => {
  const artifact = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifact) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  let completeInitial!: () => void; const prepared = new Promise<void>(accept => { completeInitial = accept; });
  const events = ["initial"], watcher = Object.assign(new EventEmitter(), { stdout: new EventEmitter() });
  activateWhenNxWatcherReady(watcher as never, prepared, () => true, () => events.push("activation"), error => { throw error; });
  expect(events).toEqual(["initial"]);
  watcher.stdout.emit("data", Buffer.from("watch process ")); watcher.stdout.emit("data", Buffer.from("waiting..."));
  await new Promise(accept => setImmediate(accept));
  expect(events).toEqual(["initial"]);
  completeInitial(); await new Promise(accept => setImmediate(accept));
  expect(events).toEqual(["initial", "activation"]);
  const directory = mkdtempSync(join(artifact, "watcher-order-")), module = join(directory, "watcher.mjs");
  const source = createSourceFile("watcher.ts", readFileSync(resolve(owner, "../📜️script.ts"), "utf8"), ScriptTarget.Latest);
  const functions = source.statements.filter(statement => isFunctionDeclaration(statement) && ["waitForNxWatcher", "activateWhenNxWatcherReady"].includes(statement.name?.text ?? "")).map(statement => statement.getText(source)).join("\n");
  await build({ stdin: { contents: functions, loader: "ts" }, outfile: module, platform: "node", format: "esm" });
  const reference = `import {EventEmitter} from 'node:events'; import {activateWhenNxWatcherReady} from ${JSON.stringify(pathToFileURL(module).href)};
const events=['initial'], watcher=Object.assign(new EventEmitter(),{stdout:new EventEmitter()});
let complete;const prepared=new Promise(accept=>{complete=accept});activateWhenNxWatcherReady(watcher,prepared,()=>true,()=>events.push('activation'),error=>{throw error;});
if(events.length!==1)throw Error('startup blocked');watcher.stdout.emit('data',Buffer.from('watch process waiting...'));await new Promise(setImmediate);if(events.length!==1)throw Error('overlapping initial preparation');complete();await new Promise(setImmediate);console.log(JSON.stringify(events));`;
  const child = Bun.spawn(["node", "--input-type=module", "-e", reference], { stdout: "pipe", stderr: "pipe" });
  const output = await new Response(child.stdout).text(), errors = await new Response(child.stderr).text();
  expect(await child.exited, errors).toBe(0); expect(JSON.parse(output)).toEqual(events);
  let active = true, activated = false;
  const cancelled = Object.assign(new EventEmitter(), { stdout: new EventEmitter() });
  activateWhenNxWatcherReady(cancelled as never, Promise.resolve(), () => active, () => { activated = true; }, error => { throw error; });
  active = false; cancelled.emit("close", 0); await new Promise(accept => setImmediate(accept)); expect(activated).toBe(false);
});

test("ordinary graphs run independently while Nx source watchers use the daemon", () => {
  expect(nxChildEnvironment({ TASK: "a" }, ["run", "a:dev"], false)).toEqual({ TASK: "a", NX_DAEMON: "true" });
  expect(nxChildEnvironment({ TASK: "b" }, ["watch"], true)).toEqual({ TASK: "b", NX_DAEMON: "true" });
  expect(nxChildEnvironment({ NX_DAEMON: "true" }, ["run", "a:build"], true)).toEqual({ NX_DAEMON: "false" });
  expect(nxChildEnvironment({ NX_DAEMON: "true" }, ["run", "a:test"], false)).toEqual({ NX_DAEMON: "true" });
});

test("selects portable pinned Bun archives and extracts the same bytes as fflate", () => {
  const bunOwner = resolve(owner, "../🛠️tools/🟦️bun"), bunSchema = JSON.parse(readFileSync(join(bunOwner, "🧬️schema/🔣️.json"), "utf8")), bunManifest = bunSchema.definitions.Release.const;
  expect(new Ajv({ strict: true }).validate(bunSchema, bunManifest)).toBe(true);
  for (const platform of ["win32", "darwin", "linux"]) for (const arch of ["x64", "arm64"]) for (const musl of platform === "linux" ? [false, true] : [false]) {
    const distribution = bunDistribution("1.4.2", platform, arch, musl);
    const member = distribution.archive.slice(0, -4) + (platform === "win32" ? "/bun.exe" : "/bun");
    const archive = zipSync({ [member]: new TextEncoder().encode("pinned executable"), "ignored.txt": new Uint8Array([1, 2]) });
    expect(bunArchiveExecutable(Buffer.from(archive), member)).toEqual(Buffer.from(unzipSync(archive)[member]!));
    expect(() => bunArchiveExecutable(Buffer.from(archive), "absent/bun")).toThrow();
    expect(distribution.sha256).toMatch(/^[a-f0-9]{64}$/);
  }
  expect(() => bunDistribution("1.3.14", "win32", "x64", false)).toThrow();
  expect(() => bunDistribution("1.4.2", "freebsd", "x64", false)).toThrow();
  expect(() => bunArchiveExecutable(Buffer.from("invalid"), "bun")).toThrow();
});



test("selects exact portable Bun operations and refuses unissued stores before an installer child",async()=>{for(const operation of fixture.operations)expect(javascriptDependencyProducerArgumentsV1(operation.mode,"/cache")).toEqual(operation.arguments);await receiveScriptProcessInvocation(process.env,async original=>{for(const args of fixture.rejections)for(const Constructor of[SyncScript,RefreshLockScript])await expect(new Constructor(owner,owner,original).run(args)).rejects.toThrow("accepts --all or --scope <owner>");});for(const platform of fixture.platforms)await expect(prepareJavascriptDependencies("sync",platform,{} as never,{} as never,{kind:"all"})).rejects.toThrow("Actual current dependency Store writer required");});

test("an independent Node bundle executes the same authored command corpus", async () => {
  const artifact = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifact) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(artifact, { recursive: true });
  const output = mkdtempSync(join(artifact, "javascript-dependencies-")), module = join(output, "dependencies.mjs");
  const source = createSourceFile("dependencies.ts", readFileSync(join(owner, "💾️storage/🟦️.ts"), "utf8"), ScriptTarget.Latest, true);
  const operation = source.statements.find(statement => isFunctionDeclaration(statement) && statement.name?.text === "javascriptDependencyProducerArgumentsV1");
  if (!operation) throw new Error("Dependency operation is missing");
  await build({ stdin: { contents: "const installSchema="+JSON.stringify(dependencyInstallSchema)+";"+operation.getText(source), loader: "ts" }, outfile: module, platform: "node", format: "esm", logLevel: "silent" });
  const entry = join(output, "oracle.mjs");
  writeFileSync(entry, `import { javascriptDependencyProducerArgumentsV1 } from ${JSON.stringify(pathToFileURL(module).href)}; const cases = ${JSON.stringify(fixture.operations)};process.stdout.write(JSON.stringify(cases.map(operation=>({args:javascriptDependencyProducerArgumentsV1(operation.mode,"/cache"),cwd:"workspace"}))));`);
  const child = Bun.spawn(["node", entry], { stdout: "pipe", stderr: "pipe" });
  const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  expect({ code, stderr }).toEqual({ code: 0, stderr: "" });
  expect(JSON.parse(stdout)).toEqual(fixture.operations.map(operation => ({ args: operation.arguments, cwd: "workspace" })));
});


test("package-declared Node entry points resolve without platform launcher shims", async () => {
  const { resolveWorkspaceBin } = await import("../../../../🏃️process/🟦️.ts");
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR!;
  const root = mkdtempSync(join(output, "package-bin-"));
  const manifest = join(root, "node_modules/@fixture/tool/package.json"), executable = join(root, "node_modules/@fixture/tool/cli.js");
  mkdirSync(join(root, "node_modules/@fixture/tool"), { recursive: true });
  writeFileSync(manifest, JSON.stringify({ name: "@fixture/tool", type: "module", bin: { tool: "cli.js" } }));
  writeFileSync(executable, "console.log(JSON.stringify({ready:true}))");
  expect(resolveWorkspaceBin("@fixture/tool", root)).toBe(executable);
  const source = parseConfigFileTextToJson("package.json", readFileSync(manifest, "utf8")).config;
  expect(source.bin.tool).toBe("cli.js");
  const child = Bun.spawn(["node", resolveWorkspaceBin("@fixture/tool", root)!], { stdout: "pipe", stderr: "pipe" });
  const result = await new Response(child.stdout).text();
  expect(await child.exited).toBe(0); expect(JSON.parse(result)).toEqual({ready:true});
});


test("initial preparation waits for an exact complete serving target and releases its observers", async () => {
  const child=Object.assign(new EventEmitter(),{stdout:new EventEmitter()}),states:boolean[]=[];let ready=false;
  const prepared=waitForNxTargetStart(child as never,"application:dev").then(()=>{ready=true;});
  const chunks=["> nx run application:dev-tools"+String.fromCharCode(10),String.fromCharCode(27)+"[32m> nx run application:","dev",String.fromCharCode(27)+"[0m"+String.fromCharCode(13,10)];
  for(const chunk of chunks) {
    child.stdout.emit("data",Buffer.from(chunk));
    await new Promise(accept=>setImmediate(accept));states.push(ready);
  }
  expect(states).toEqual([false,false,false,true]);await prepared;
  expect(child.stdout.listenerCount("data")+child.listenerCount("close")+child.listenerCount("error")).toBe(0);
  const failed=Object.assign(new EventEmitter(),{stdout:new EventEmitter()}),failure=waitForNxTargetStart(failed as never,"application:dev");
  failed.emit("close",1);await expect(failure).rejects.toThrow("ended before serving target");
  const source=createSourceFile("bootstrap.ts",readFileSync(resolve(owner,"../\u{1f4dc}\uFE0Fscript.ts"),"utf8"),ScriptTarget.Latest);
  const operation=source.statements.find(statement=>isFunctionDeclaration(statement)&&statement.name?.text==="waitForNxTargetStart")!;
  const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"target-start-")),module=join(directory,"target.mjs");
  await build({stdin:{contents:operation.getText(source),loader:"ts"},outfile:module,platform:"node",format:"esm"});
  const reference="import {EventEmitter} from 'node:events';import {waitForNxTargetStart} from "+JSON.stringify(pathToFileURL(module).href)+";const child=Object.assign(new EventEmitter(),{stdout:new EventEmitter()}),states=[];let ready=false;const prepared=waitForNxTargetStart(child,'application:dev').then(()=>ready=true);for(const chunk of "+JSON.stringify(chunks)+"){child.stdout.emit('data',Buffer.from(chunk));await new Promise(setImmediate);states.push(ready);}await prepared;console.log(JSON.stringify(states));";
  const oracle=Bun.spawn(["node","--input-type=module","-e",reference],{stdout:"pipe",stderr:"pipe"});
  const [output,errors,code]=await Promise.all([new Response(oracle.stdout).text(),new Response(oracle.stderr).text(),oracle.exited]);
  expect(code,errors).toBe(0);expect(JSON.parse(output)).toEqual(states);
});


test("automatic watcher callbacks register only after initial preparation and honour cancellation", async () => {
  const {startNxWatcherAfterPreparation}=await import("../../\u{1f4dc}\uFE0Fscript.ts");
  let complete!:()=>void;const prepared=new Promise<void>(accept=>{complete=accept;}),events=["initial"];
  const registered=new ChildProcess(),watcher=startNxWatcherAfterPreparation(prepared,()=>true,()=>{events.push("watcher");return registered;});
  await new Promise(accept=>setImmediate(accept));expect(events).toEqual(["initial"]);complete();expect(await watcher).toBe(registered);expect(events).toEqual(["initial","watcher"]);
  let started=false;expect(await startNxWatcherAfterPreparation(Promise.resolve(),()=>false,()=>{started=true;throw Error("Cancelled watcher callback must not start");})).toBeUndefined();expect(started).toBe(false);
  await expect(startNxWatcherAfterPreparation(Promise.reject(Error("initial failure")),()=>true,()=>{started=true;throw Error("Cancelled watcher callback must not start");})).rejects.toThrow("initial failure");expect(started).toBe(false);
  const source=createSourceFile("bootstrap.ts",readFileSync(resolve(owner,"../\u{1f4dc}\uFE0Fscript.ts"),"utf8"),ScriptTarget.Latest),operation=source.statements.find(statement=>isFunctionDeclaration(statement)&&statement.name?.text==="startNxWatcherAfterPreparation")!;
  const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"watcher-registration-")),module=join(directory,"watcher.mjs");
  await build({stdin:{contents:operation.getText(source),loader:"ts"},outfile:module,platform:"node",format:"esm"});
  const reference="import {startNxWatcherAfterPreparation} from "+JSON.stringify(pathToFileURL(module).href)+";import {ChildProcess} from 'node:child_process';let complete;const registered=new ChildProcess(),prepared=new Promise(accept=>complete=accept),events=['initial'];const watcher=startNxWatcherAfterPreparation(prepared,()=>true,()=>{events.push('watcher');return registered;});await new Promise(setImmediate);if(events.length!==1)throw Error('premature registration');complete();if(await watcher!==registered)throw Error('Watcher owner identity changed');console.log(JSON.stringify(events));";
  const oracle=Bun.spawn(["node","--input-type=module","-e",reference],{stdout:"pipe",stderr:"pipe"});
  const [output,errors,code]=await Promise.all([new Response(oracle.stdout).text(),new Response(oracle.stderr).text(),oracle.exited]);expect(code,errors).toBe(0);expect(JSON.parse(output)).toEqual(events);
});

test("Nx producers and watchers inherit the pinned Bun runtime on every host", async () => {
  const bootstrap=await import("../../📜️script.ts"),{win32,posix}=await import("node:path");
  expect(bootstrap.pinnedNxRuntimeEnvironment).toBeFunction();
  const rows=JSON.parse(readFileSync(resolve(owner,"../🛠️tools/🟦️bun/🧪️tests/🧫️fixtures/🌿️environment/🔣️.json"),"utf8")).cases;
  const source=createSourceFile("bootstrap.ts",readFileSync(resolve(owner,"../📜️script.ts"),"utf8"),ScriptTarget.Latest),operation=source.statements.find(statement=>isFunctionDeclaration(statement)&&statement.name?.text==="pinnedNxRuntimeEnvironment")!;
  const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"pinned-bun-env-")),module=join(directory,"environment.mjs");
  await build({stdin:{contents:'import {win32,posix} from "node:path";\n'+operation.getText(source),loader:"ts"},outfile:module,platform:"node",format:"esm"});
  const oracle=Bun.spawn(["node","--input-type=module","-e",'import {pinnedNxRuntimeEnvironment} from '+JSON.stringify(pathToFileURL(module).href)+';console.log(JSON.stringify('+JSON.stringify(rows)+'.map(row=>pinnedNxRuntimeEnvironment(row.environment,row.bun,row.platform))));'],{stdout:"pipe",stderr:"pipe"});
  const [stdout,stderr,status]=await Promise.all([new Response(oracle.stdout).text(),new Response(oracle.stderr).text(),oracle.exited]);
  expect(status,stderr).toBe(0);
  const observed=rows.map((row:any)=>bootstrap.pinnedNxRuntimeEnvironment(row.environment,row.bun,row.platform));expect(observed).toEqual(rows.map((row:any)=>row.expected));expect(JSON.parse(stdout)).toEqual(observed);
  for(const platform of ["win32","linux","darwin"] as const)expect(()=>bootstrap.pinnedNxRuntimeEnvironment({},"relative/bun",platform)).toThrow("absolute");
});


test("explicit languages and terminology isolate watcher graphs and sockets", () => {
  const selections=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8")).watcherSelections as Record<string,string>[];
  const graphs=new Set<string>(),sockets=new Set<string>(),workspace=resolve(owner),target="a:activate-react";
  for(const selection of selections){
    const environment={S_OS_PORT:"6064",PLAYGROUND_LOCKED_EXAMPLE_ID:"🎬️demo",...selection,...boundedNxNamespaceV1(workspace,workspace,"a".repeat(64),workspace,null,"d".repeat(64)).environment,SEMIO_NX_TOOLING_IDENTITY:"a".repeat(64),SEMIO_NX_SOCKET_STORAGE:"/c/l/m"},actual=nxWatcherEnvironment(workspace,environment,target);
    const entries=Object.entries({S_OS_PORT:"6064",PLAYGROUND_LOCKED_EXAMPLE_ID:"🎬️demo",SEMIO_NX_TOOLING_IDENTITY:"a".repeat(64),...selection}).sort(([a],[b])=>a<b?-1:a>b?1:0),identity=JSON.stringify([workspace.replaceAll("\\","/"),target,entries]),hash=new Bun.CryptoHasher("sha256").update(identity).digest("hex").slice(0,24);
    expect(actual.NX_WORKSPACE_DATA_DIRECTORY.endsWith(hash)).toBe(true);
    graphs.add(actual.NX_WORKSPACE_DATA_DIRECTORY);sockets.add(actual.NX_SOCKET_DIR);
  }
  expect(graphs.size).toBe(selections.length);expect(sockets.size).toBe(selections.length);
});

test("a nested Nx run spools every line to its log and forwards only a bounded summary", () => {
  const rows = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8")).spool as { forwarded: string[]; suppressed: string[]; forwardBytes: number; heartbeatMs: number };
  const directory = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "nx-spool-")), path = join(directory, "o.log");
  const forwarded: string[] = [], clock = { now: 0 };
  const spool = new NxOutputSpoolV1(openSync(path, "a"), path, (line) => forwarded.push(line), () => clock.now, rows.forwardBytes, rows.heartbeatMs);
  const input = [...rows.forwarded, ...rows.suppressed];
  for (const line of input) spool.line(line);
  expect(forwarded).toEqual(rows.forwarded);
  expect(spool.lines).toBe(input.length);
  clock.now = rows.heartbeatMs;
  spool.line(rows.suppressed[0]!);
  expect(forwarded.at(-1)).toBe(`[nx-output] ${input.length + 1} line(s), ${spool.bytes} byte(s) in ${path}`);
  spool.line(rows.suppressed[0]!);
  expect(forwarded.filter((line) => line.startsWith("[nx-output]")).length).toBe(1);
  const budgeted: string[] = [], bounded = new NxOutputSpoolV1(openSync(join(directory, "b.log"), "a"), path, (line) => budgeted.push(line), () => 0, rows.forwardBytes, rows.heartbeatMs);
  for (let index = 0; index < 100; index++) bounded.line(rows.forwarded[0]!);
  expect(budgeted.join("\n").length).toBeLessThanOrEqual(rows.forwardBytes);
  expect(bounded.lines).toBe(100);
  spool.close(); bounded.close();
  expect(readFileSync(path, "utf8").split("\n").slice(0, input.length)).toEqual(input);
  if (process.platform !== "win32") {
    const oracle = Bun.spawnSync(["grep", "-E", "-c", "^(> nx run |[[:space:]]*NX[[:space:]]|[[:space:]]*Successfully ran|[[:space:]]*Failed tasks|[[:space:]]*Tasks not run|[[:space:]]*Tasks stopped|[[:space:]]*Run duration|\\[budget\\])"], { stdin: new TextEncoder().encode(rows.forwarded.map((line) => line.replace(/\u001b\[[0-9;]*m/gu, "")).concat(rows.suppressed).join("\n") + "\n") });
    expect(Number(new TextDecoder().decode(oracle.stdout).trim())).toBe(rows.forwarded.length);
  }
});


test("loaded Nx toolkit root comes from the exact live bootstrap module",async()=>{
 const ts=await import("typescript"),rows=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8")).loadedToolkit,source=ts.createSourceFile("bootstrap.ts",readFileSync(resolve(owner,"../📜️script.ts"),"utf8"),ts.ScriptTarget.Latest,true),objects:any[]=[];
 const visit=(node:any)=>{if(ts.isObjectLiteralExpression(node)&&node.properties.some((entry:any)=>ts.isPropertyAssignment(entry)&&entry.name.getText(source)==="NODE_PATH"&&entry.initializer.getText(source)==="tooling.modulePath"))objects.push(node);ts.forEachChild(node,visit);};visit(source);expect(objects).toHaveLength(1);
 const fields=objects[0].properties.filter((entry:any)=>ts.isPropertyAssignment(entry)&&entry.name.getText(source)===rows.field);expect(fields).toHaveLength(1);expect(fields[0].initializer.getText(source)).toBe(rows.expression);
 const child=Bun.spawn(["node","--input-type=module","-e",'import{posix}from"node:path";console.log(JSON.stringify('+JSON.stringify(rows.cases)+'.map(row=>posix.dirname(row.modulePath))));'],{stdout:"pipe",stderr:"pipe"}),[stdout,stderr,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);expect(status,stderr).toBe(0);expect(JSON.parse(stdout)).toEqual(rows.cases.map((row:any)=>row.root));
 const validate=new Ajv({strict:true}).compile({type:"object",required:["modulePath","root"],additionalProperties:false,properties:{modulePath:{type:"string",pattern:"/node_modules$"},root:{type:"string",pattern:"/[a-f0-9]{64}$"}}});for(const row of rows.cases){expect(validate(row)).toBe(true);expect(row.root).not.toBe(row.modulePath);}
});
