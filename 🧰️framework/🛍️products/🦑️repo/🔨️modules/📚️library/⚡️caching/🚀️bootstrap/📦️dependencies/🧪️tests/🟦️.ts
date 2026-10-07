/** 📦️ Portable locked-install and owner-requested lock refresh command laws. */
import { expect, test } from "bun:test";
import { readFileSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { resolve, join } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import { build } from "esbuild";
import { createSourceFile, isFunctionDeclaration, parseConfigFileTextToJson, ScriptTarget } from "typescript";
import { prepareJavascriptDependencies, SyncScript, RefreshLockScript } from "../📜️script.ts";
import { bunDistribution, bunArchiveExecutable } from "../../🛠️tools/🟦️bun/📜️script.ts";
import { zipSync, unzipSync } from "fflate";
import { activateWhenNxWatcherReady, waitForNxTargetStart, nxChildEnvironment, nxWatcherEnvironment } from "../../📜️script.ts";
import { EventEmitter } from "node:events";

const owner = resolve(import.meta.dir, "..");
const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8")) as { operations: { mode: "sync" | "lock"; arguments: string[] }[]; platforms: string[]; rejections: string[][] };

test("source watchers have stable independent graph stores and sockets for each activation binding", () => {
  const workspace = resolve(owner), environment = { S_OS_PORT: "6064", PLAYGROUND_LOCKED_EXAMPLE_ID: "🎬️demo", NX_WORKSPACE_DATA_DIRECTORY: "shared" }, target = "a:activate-react";
  const a = nxWatcherEnvironment(workspace, environment, target);
  expect(nxWatcherEnvironment(workspace, environment, target)).toEqual(a);
  expect(a.S_OS_PORT).toBe("6064");
  expect(a.NX_WORKSPACE_DATA_DIRECTORY).not.toBe(environment.NX_WORKSPACE_DATA_DIRECTORY);
  const identity = JSON.stringify([workspace.replaceAll("\\", "/"), target, Object.entries(environment).filter(([key]) => /^(?:S_|VITE_|PLAYGROUND_|SEMIO_(?:PLUGIN|RENDERER|BUILD_MODE|LOCALE)$)/.test(key)).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0)]);
  const hash = new Bun.CryptoHasher("sha256").update(identity).digest("hex").slice(0, 24);
  expect(a.NX_WORKSPACE_DATA_DIRECTORY.endsWith(hash)).toBe(true);
  for (const b of [nxWatcherEnvironment(workspace, environment, "a:activate-wgpu"), nxWatcherEnvironment(workspace, { ...environment, PLAYGROUND_LOCKED_EXAMPLE_ID: "🎬️demo-session" }, target)]) {
    expect(b.NX_WORKSPACE_DATA_DIRECTORY).not.toBe(a.NX_WORKSPACE_DATA_DIRECTORY);
    expect(b.NX_SOCKET_DIR).not.toBe(a.NX_SOCKET_DIR);
  }
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
  expect(nxChildEnvironment({ TASK: "a" }, ["run", "a:dev"], false)).toEqual({ TASK: "a", NX_DAEMON: "false" });
  expect(nxChildEnvironment({ TASK: "b" }, ["watch"], true)).toEqual({ TASK: "b", NX_DAEMON: "true" });
  expect(nxChildEnvironment({ NX_DAEMON: "true" }, ["run", "a:build"], true)).toEqual({ NX_DAEMON: "false" });
  expect(nxChildEnvironment({ NX_DAEMON: "true" }, ["run", "a:test"], false)).toEqual({ NX_DAEMON: "true" });
});

test("selects portable pinned Bun archives and extracts the same bytes as fflate", () => {
  const bunOwner = resolve(owner, "../🛠️tools/🟦️bun"), bunManifest = JSON.parse(readFileSync(join(bunOwner, "🔣️.json"), "utf8"));
  expect(new Ajv({ strict: true }).validate(JSON.parse(readFileSync(join(bunOwner, "🧬️schema/🔣️.json"), "utf8")), bunManifest)).toBe(true);
  for (const platform of ["win32", "darwin", "linux"]) for (const arch of ["x64", "arm64"]) for (const musl of platform === "linux" ? [false, true] : [false]) {
    const distribution = bunDistribution("1.3.14", platform, arch, musl);
    const member = distribution.archive.slice(0, -4) + (platform === "win32" ? "/bun.exe" : "/bun");
    const archive = zipSync({ [member]: new TextEncoder().encode("pinned executable"), "ignored.txt": new Uint8Array([1, 2]) });
    expect(bunArchiveExecutable(Buffer.from(archive), member)).toEqual(Buffer.from(unzipSync(archive)[member]));
    expect(() => bunArchiveExecutable(Buffer.from(archive), "absent/bun")).toThrow();
    expect(distribution.sha256).toMatch(/^[a-f0-9]{64}$/);
  }
  expect(() => bunDistribution("1.4.2", "win32", "x64", false)).toThrow();
  expect(() => bunDistribution("1.3.14", "freebsd", "x64", false)).toThrow();
  expect(() => bunArchiveExecutable(Buffer.from("invalid"), "bun")).toThrow();
});



test("selects exact Bun arguments across all supported hosts and rejects installer overrides", async () => {
  for (const platform of fixture.platforms) for (const operation of fixture.operations) {
    const calls: unknown[] = [], signal = new AbortController().signal;
    await prepareJavascriptDependencies(operation.mode, platform, signal, async (args, cwd, actualSignal) => { calls.push({ args, cwd, identicalSignal: actualSignal === signal }); });
    expect(calls).toEqual([{ args: operation.arguments, cwd: platform, identicalSignal: true }]);
  }
  for (const args of fixture.rejections) for (const Constructor of [SyncScript, RefreshLockScript]) await expect(new Constructor(owner, owner).run(args)).rejects.toThrow("accepts --all or --scope <owner>");
  const aborted = new AbortController(); aborted.abort();
  let called = false;
  await expect(prepareJavascriptDependencies("lock", owner, aborted.signal, async () => { called = true; })).rejects.toThrow();
  expect(called).toBe(false);
  await expect(prepareJavascriptDependencies("lock", owner, new AbortController().signal, async () => { throw new Error("installer refused"); })).rejects.toThrow("installer refused");
});

test("an independent Node bundle executes the same authored command corpus", async () => {
  const artifact = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifact) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(artifact, { recursive: true });
  const output = mkdtempSync(join(artifact, "javascript-dependencies-")), module = join(output, "dependencies.mjs");
  const source = createSourceFile("dependencies.ts", readFileSync(join(owner, "📜️script.ts"), "utf8"), ScriptTarget.Latest, true);
  const operation = source.statements.find(statement => isFunctionDeclaration(statement) && statement.name?.text === "prepareJavascriptDependencies");
  if (!operation) throw new Error("Dependency operation is missing");
  await build({ stdin: { contents: `const runBun = () => { throw new Error("select a runner"); };\n${operation.getText(source)}`, loader: "ts" }, outfile: module, platform: "node", format: "esm", logLevel: "silent" });
  const entry = join(output, "oracle.mjs");
  writeFileSync(entry, `import { prepareJavascriptDependencies } from ${JSON.stringify(pathToFileURL(module).href)}; const cases = ${JSON.stringify(fixture.operations)}; const calls = []; for (const operation of cases) await prepareJavascriptDependencies(operation.mode, "workspace", new AbortController().signal, async (args, cwd) => calls.push({args,cwd})); process.stdout.write(JSON.stringify(calls));`);
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
  const watcher=startNxWatcherAfterPreparation(prepared,()=>true,()=>{events.push("watcher");return "registered" as never;});
  await new Promise(accept=>setImmediate(accept));expect(events).toEqual(["initial"]);complete();expect(await watcher).toBe("registered");expect(events).toEqual(["initial","watcher"]);
  let started=false;expect(await startNxWatcherAfterPreparation(Promise.resolve(),()=>false,()=>{started=true;return null as never;})).toBeUndefined();expect(started).toBe(false);
  await expect(startNxWatcherAfterPreparation(Promise.reject(Error("initial failure")),()=>true,()=>{started=true;return null as never;})).rejects.toThrow("initial failure");expect(started).toBe(false);
  const source=createSourceFile("bootstrap.ts",readFileSync(resolve(owner,"../\u{1f4dc}\uFE0Fscript.ts"),"utf8"),ScriptTarget.Latest),operation=source.statements.find(statement=>isFunctionDeclaration(statement)&&statement.name?.text==="startNxWatcherAfterPreparation")!;
  const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"watcher-registration-")),module=join(directory,"watcher.mjs");
  await build({stdin:{contents:operation.getText(source),loader:"ts"},outfile:module,platform:"node",format:"esm"});
  const reference="import {startNxWatcherAfterPreparation} from "+JSON.stringify(pathToFileURL(module).href)+";let complete;const prepared=new Promise(accept=>complete=accept),events=['initial'];const watcher=startNxWatcherAfterPreparation(prepared,()=>true,()=>events.push('watcher'));await new Promise(setImmediate);if(events.length!==1)throw Error('premature registration');complete();await watcher;console.log(JSON.stringify(events));";
  const oracle=Bun.spawn(["node","--input-type=module","-e",reference],{stdout:"pipe",stderr:"pipe"});
  const [output,errors,code]=await Promise.all([new Response(oracle.stdout).text(),new Response(oracle.stderr).text(),oracle.exited]);expect(code,errors).toBe(0);expect(JSON.parse(output)).toEqual(events);
});

test("Nx producers and watchers inherit the pinned Bun runtime on every host", async () => {
  const bootstrap=await import("../../📜️script.ts"),{win32,posix}=await import("node:path");
  expect(bootstrap.pinnedNxRuntimeEnvironment).toBeFunction();
  const rows=JSON.parse(readFileSync(resolve(owner,"../🛠️tools/🟦️bun/🧫️fixtures/🌿️environment/🔣️.json"),"utf8")).cases;
  const source=createSourceFile("bootstrap.ts",readFileSync(resolve(owner,"../📜️script.ts"),"utf8"),ScriptTarget.Latest),operation=source.statements.find(statement=>isFunctionDeclaration(statement)&&statement.name?.text==="pinnedNxRuntimeEnvironment")!;
  const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"pinned-bun-env-")),module=join(directory,"environment.mjs");
  await build({stdin:{contents:'import {win32,posix} from "node:path";\n'+operation.getText(source),loader:"ts"},outfile:module,platform:"node",format:"esm"});
  const oracle=Bun.spawn(["node","--input-type=module","-e",'import {pinnedNxRuntimeEnvironment} from '+JSON.stringify(pathToFileURL(module).href)+';console.log(JSON.stringify('+JSON.stringify(rows)+'.map(row=>pinnedNxRuntimeEnvironment(row.environment,row.bun,row.platform))));'],{stdout:"pipe",stderr:"pipe"});
  const [stdout,stderr,status]=await Promise.all([new Response(oracle.stdout).text(),new Response(oracle.stderr).text(),oracle.exited]);
  expect(status,stderr).toBe(0);
  const observed=rows.map((row:any)=>bootstrap.pinnedNxRuntimeEnvironment(row.environment,row.bun,row.platform));expect(observed).toEqual(rows.map((row:any)=>row.expected));expect(JSON.parse(stdout)).toEqual(observed);
  for(const platform of ["win32","linux","darwin"])expect(()=>bootstrap.pinnedNxRuntimeEnvironment({},"relative/bun",platform)).toThrow("absolute");
  console.log("[DEBUG] pinned Nx Bun environment checked against esbuild/Node on three hosts");
});


test("explicit languages and terminology isolate watcher graphs and sockets", () => {
  const selections=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")).watcherSelections as Record<string,string>[];
  const graphs=new Set<string>(),sockets=new Set<string>(),workspace=resolve(owner),target="a:activate-react";
  for(const selection of selections){
    const environment={S_OS_PORT:"6064",PLAYGROUND_LOCKED_EXAMPLE_ID:"🎬️demo",...selection},actual=nxWatcherEnvironment(workspace,environment,target);
    const entries=Object.entries(environment).sort(([a],[b])=>a<b?-1:a>b?1:0),identity=JSON.stringify([workspace.replaceAll("\\","/"),target,entries]),hash=new Bun.CryptoHasher("sha256").update(identity).digest("hex").slice(0,24);
    expect(actual.NX_WORKSPACE_DATA_DIRECTORY.endsWith(hash)).toBe(true);
    graphs.add(actual.NX_WORKSPACE_DATA_DIRECTORY);sockets.add(actual.NX_SOCKET_DIR);
  }
  expect(graphs.size).toBe(selections.length);expect(sockets.size).toBe(selections.length);
});
