#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import{readNativeCallerAuthorityV1}from"../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { join } from "node:path";
import { readFileSync, lstatSync, realpathSync } from "node:fs";
import { createHash } from "node:crypto";
import { isAbsolute, relative, resolve, sep } from "node:path";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { buildModePolicy, readBuildMode, type BuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

/** 🧬️ An immutable compiler request whose captured source bytes are delivered through an owned pipe. */
export type BunBundleRequest = Readonly<{ version: 1; mode: BuildMode }> & (Readonly<{ kind: "snapshot"; entrypoint: string; observe?: boolean; roots: readonly Readonly<{ name: string; path: string }>[] }> | Readonly<{ kind: "actor"; contents: string; runtime: Readonly<{ host: string; wasi: string }> }> | Readonly<{ kind: "module"; entrypoint: string; contents: string; external: readonly string[] }> | Readonly<{ kind: "bun-package"; entrypoint: string }>);
/** 📦️ Generated JavaScript and the actual source identities observed by the compiler. */
export type BunBundleResult = Readonly<{ source: string; sourceMap?: string; inputs: readonly Readonly<{ logicalPath: string; sha256: string; byteLength: number }>[]; physicalInputs: readonly Readonly<{ logicalPath: string; path: string; sha256: string; byteLength: number }>[]; loadedRuntime: number }>;

/** 🏗️ Compiles admitted snapshots in an isolated process that its caller can terminate during native bundling. */
async function compileBunBundle(request: BunBundleRequest): Promise<BunBundleResult> {
  if (validateJsonSchemaSubset(schema, request).length || Buffer.byteLength(JSON.stringify(request)) > 16 * 1024 * 1024) throw Error("Invalid Bun bundle request");
  readBuildMode(request.mode);
  console.error(`[bun-worker] compiling pid=${process.pid} kind=${request.kind}`);
  if (request.kind === "snapshot") {
    const roots = request.roots.map(root => ({ ...root, path: realpathSync(root.path) })), snapshots = new Map<string, { bytes: Buffer; logicalPath: string; sha256: string; byteLength: number }>();
    let remaining = 8 * 1024 * 1024;
    const result = await Bun.build({ entrypoints: [request.entrypoint], root: roots[0]!.path, target: "browser", format: "esm", treeShaking: buildModePolicy(request.mode).optimize, splitting: false, minify: false, metafile: true, external: ["node:fs/promises"], plugins: [{ name: "semio-browser-codegen-snapshot", setup(build) {
      build.onLoad({ filter: /.*/, namespace: "file" }, args => {
        const path = realpathSync(args.path), root = roots.find(root => path.startsWith(root.path + sep));
        if (!root || !path.endsWith(".js")) throw Error("Browser compiler source root or type refused");
        let captured = snapshots.get(path);
        if (!captured) {
          if (snapshots.size >= 128) throw Error("Browser compiler source count refused");
          const before = lstatSync(path); if (!before.isFile() || before.size > remaining) throw Error("Browser compiler source bytes refused");
          const bytes = readFileSync(path), after = lstatSync(path);
          if (["dev", "ino", "size", "mtimeMs", "ctimeMs"].some(key => before[key as keyof typeof before] !== after[key as keyof typeof after]) || bytes.byteLength !== before.size) throw Error("Browser compiler source changed during capture");
          remaining -= bytes.byteLength;
          captured = { bytes, logicalPath: root.name + "/" + relative(root.path, path).split(sep).join("/"), sha256: createHash("sha256").update(bytes).digest("hex"), byteLength: bytes.byteLength }; snapshots.set(path, captured); if (request.observe) console.error("[bun-worker] captured " + JSON.stringify(path));
        }
        return { contents: captured.bytes, loader: "js" };
      });
    } }] });
    if (!result.success || result.outputs.length !== 1 || !result.metafile) throw Error("Browser compiler closure failed");
    const observed = Object.keys(result.metafile.inputs).map(path => isAbsolute(path) ? path : resolve(path)).sort();
    if (JSON.stringify(observed) !== JSON.stringify([...snapshots.keys()].sort())) throw Error("Browser compiler source coverage or output bytes refused");
    const physicalInputs = [...snapshots].map(([path, value]) => ({ path, logicalPath: value.logicalPath, sha256: value.sha256, byteLength: value.byteLength })).sort((left, right) => left.logicalPath < right.logicalPath ? -1 : left.logicalPath > right.logicalPath ? 1 : 0);
    return { source: await result.outputs[0]!.text(), physicalInputs, inputs: physicalInputs.map(({ path, ...value }) => value), loadedRuntime: 0 };
  }
  if (request.kind === "bun-package") {
    const workspace = findWorkspaceRoot(import.meta.dir), physicalInputs = new Map<string, BunBundleResult["physicalInputs"][number]>();
    const result = await Bun.build({ entrypoints: [request.entrypoint], target: "bun", format: "esm", treeShaking: buildModePolicy(request.mode).optimize, naming: "🟦️.js", minify: buildModePolicy(request.mode).optimize, sourcemap: buildModePolicy(request.mode).sourceMap ? "external" : "none", metafile: true, plugins: [{ name: "semio-package-source-custody", setup(build) {
      build.onLoad({ filter: /.*/, namespace: "file" }, args => {
        const path = realpathSync(args.path), before = lstatSync(path), bytes = readFileSync(path), after = lstatSync(path);
        if (!before.isFile() || bytes.byteLength !== before.size || ["dev", "ino", "size", "mtimeMs", "ctimeMs"].some(key => before[key as keyof typeof before] !== after[key as keyof typeof after])) throw Error("Bun package source changed during capture");
        physicalInputs.set(path, { path, logicalPath: relative(workspace, path).split(sep).join("/"), sha256: createHash("sha256").update(bytes).digest("hex"), byteLength: bytes.byteLength });
        return { contents: bytes, loader: args.loader };
      });
    } }] });
    if (!result.success) throw Error("Bun package closure failed: " + result.logs.map(log => log.message).join("; "));
    const source = result.outputs.find(output => output.kind === "entry-point"), sourceMap = result.outputs.find(output => output.path.endsWith(".map"));
    if (!source) throw Error("Bun package entry output missing");
    const observed = Object.keys(result.metafile?.inputs ?? {}).map(path => isAbsolute(path) ? path : resolve(path)).sort();
    if (JSON.stringify(observed) !== JSON.stringify([...physicalInputs.keys()].sort())) throw Error("Bun package source coverage refused");
    const captured = [...physicalInputs.values()];
    return { source: await source.text(), ...(sourceMap ? { sourceMap: await sourceMap.text() } : {}), inputs: captured.map(({ path, ...value }) => value), physicalInputs: captured, loadedRuntime: 0 };
  }
  if (request.kind === "module") {
    const result = await Bun.build({ entrypoints: [request.entrypoint], target: "browser", format: "esm", treeShaking: buildModePolicy(request.mode).optimize, minify: buildModePolicy(request.mode).optimize, sourcemap: buildModePolicy(request.mode).sourceMap ? "inline" : "none", plugins: [{ name: "semio-browser-module", setup(build) {
      build.onLoad({ filter: /.*/ }, args => args.path === request.entrypoint ? { contents: request.contents, loader: "js" } : undefined);
      build.onResolve({ filter: /.*/ }, args => request.external.includes(args.path) ? { path: args.path, external: true } : undefined);
    } }] });
    if (!result.success || result.outputs.length !== 1) throw Error("Browser module binding failed");
    return { source: await result.outputs[0]!.text(), inputs: [], physicalInputs: [], loadedRuntime: 0 };
  }
  const loaded = new Set<string>();
  const result = await Bun.build({ entrypoints: ["semio:closed-browser-actor"], target: "browser", format: "esm", treeShaking: buildModePolicy(request.mode).optimize, splitting: false, minify: buildModePolicy(request.mode).optimize, plugins: [{ name: "semio-closed-browser-actor", setup(build) {
    build.onResolve({ filter: /^semio:closed-browser-actor$/ }, () => ({ path: "actor.ts", namespace: "semio-actor" }));
    build.onResolve({ filter: /^semio:actor-(host|wasi)$/ }, args => ({ path: args.path.slice("semio:actor-".length), namespace: "semio-runtime" }));
    build.onResolve({ filter: /.*/, namespace: "semio-runtime" }, () => { throw Error("Browser runtime import refused"); });
    build.onLoad({ filter: /.*/, namespace: "file" }, () => { throw Error("Browser runtime file refused"); });
    build.onLoad({ filter: /.*/, namespace: "semio-actor" }, () => ({ contents: request.contents, loader: "ts" }));
    build.onLoad({ filter: /.*/, namespace: "semio-runtime" }, args => {
      if (!Object.hasOwn(request.runtime, args.path) || loaded.has(args.path)) throw Error("Browser runtime snapshot refused");
      loaded.add(args.path); return { contents: request.runtime[args.path as "host" | "wasi"], loader: "ts" };
    });
  } }] });
  if (!result.success || result.outputs.length !== 1) throw Error("Browser actor closure failed");
  return { source: await result.outputs[0]!.text(), inputs: [], physicalInputs: [], loadedRuntime: loaded.size };
}

if(import.meta.main){
 if(process.argv.length!==3||process.argv[2]!=="compile")throw Error("Usage: compile (bounded request stdin; output data stdout)");
 await receiveScriptProcessInvocation(process.env,async invocation=>{
 const authority=readNativeCallerAuthorityV1(invocation.capabilities),workspace=findWorkspaceRoot(import.meta.dir),control=scriptCompilerAdmissionControlV1(invocation,"bun:compile-leaf",120000);if(workspace!==authority.repositoryRoot||process.env.SEMIO_BUILD_MODE!==authority.original.buildMode||process.env.SEMIO_COMPILER_CAPACITY_HELD!==undefined)throw Error("Exact unmarked compiler leaf Original required");
 let size=0;const chunks:Buffer[]=[];for await(const chunk of process.stdin){control.signal.throwIfAborted();size+=chunk.byteLength;if(size>16*1024*1024)throw Error("Bun compiler request bytes refused");chunks.push(chunk);}const request=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(Buffer.concat(chunks)))as BunBundleRequest;if(readBuildMode(authority.original.buildMode)!==request.mode)throw Error("Bun compiler request contradicts the selected build mode");
 const result=await compileBunBundle(request),{bunBundleStreamFramesV1}=await import("./📤️stream/🟦️.ts"),{writeOwnedCommandOutputV1}=await import("../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts"),files=[{name:"source.js",bytes:Buffer.from(result.source)},...(result.sourceMap!==undefined?[{name:"source.js.map",bytes:Buffer.from(result.sourceMap)}]:[]),{name:"metadata.json",bytes:Buffer.from(JSON.stringify({inputs:result.inputs,physicalInputs:result.physicalInputs,loadedRuntime:result.loadedRuntime}))}];for(const frame of bunBundleStreamFramesV1(createHash("sha256").update(JSON.stringify(request)).digest("hex"),files))await writeOwnedCommandOutputV1(process.stdout,frame,control);
 });
}
