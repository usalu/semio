import{createIssuedResourceControlV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withNextStorageV1} from "../../../../🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🏗️builder/💾️storage/🟦️.ts";
import {withRepositoryArtifactV1,withRepositoryArtifactTaskV1} from "../../📦️artifacts/👓️reading/🟦️.ts";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { createServer } from "node:net";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { withCountedBunCapacity } from "../../📦️bundle/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { scriptInvocationBudget, scriptInvocationEnvironment, type ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🌐️ Runs the assembled Next artifact with outside-module resolution refused by an independent Node guard. */
export async function proveNextRuntime(workspace: string, output: string, invocation: ScriptInvocation): Promise<void> {
  const mode=readBuildMode(process.env.SEMIO_BUILD_MODE);
  return withNextStorageV1(workspace,mode,invocation,createIssuedResourceControlV1(invocation,"cache:entry",0),async(store,layout)=>withRepositoryArtifactV1({workspace,invocation,storage:store.storage,control:store.control,request:{version:1,path:layout.output,owner:`repo-coordinator:next:${mode}`,maximumBytes:2147483648,maximumFiles:131072}},reader=>withRepositoryArtifactTaskV1(reader,invocation,async artifactReader=>{await artifactReader.recheck();
  const fixture = JSON.parse(await readFile(join(import.meta.dir, "../🏗️web-build/🧫️cases.json"), "utf8")), artifact = join(layout.output,"standalone"), manifest = JSON.parse(await readFile(join(artifact, "node_modules/next/package.json"), "utf8"));
  const packageRoot = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript"), require = createRequire(join(packageRoot, "package.json"));
  assert.equal(manifest.version, require("next/package.json").version);
  const socket = createServer(); await new Promise<void>((accept, reject) => { socket.once("error", reject); socket.listen(0, "127.0.0.1", accept); });
  const port = (socket.address() as { port: number }).port; await new Promise<void>((accept, reject) => socket.close(error => error ? reject(error) : accept()));
  const controller = new AbortController(), signal = AbortSignal.any([controller.signal, invocation.control.signal]), lines: string[] = [];
  let ready!: () => void; const started = new Promise<void>(accept => { ready = accept; });
  const child = runOwnedCommand("node", ["--experimental-strip-types", join(import.meta.dir, "📜️script.ts"), "guard", artifact], workspace, "next:runtime-oracle", scriptInvocationBudget(invocation, 30_000), { signal, env: scriptInvocationEnvironment(invocation, { ...store.environment, PORT: String(port), HOSTNAME: "127.0.0.1" }), onLine(line) { lines.push(line); if (line.includes("Ready in")) ready(); } });
  const completed = child.then(() => { throw Error("Standalone server ended before cancellation"); }, error => { if (!controller.signal.aborted) throw error; });
  try {
    await Promise.race([started, completed]);
    const address = `http://127.0.0.1:${port}`, response = await fetch(address, { signal: invocation.control.signal }); assert.equal(response.status, fixture.nextRuntime.status);
    const html = await response.text(), assets = [...new Set([...html.matchAll(/(?:src|href)="([^" ]*\/_next\/static\/[^" ]+)"/gu)].map(match => { const asset=match[1];assert.ok(asset!==undefined,"Matched Next static asset capture required");return asset.replaceAll("&amp;", "&"); }))];
    assert.ok(assets.length > 0); let bytes = 0;
    for (const asset of assets) { const response = await fetch(new URL(asset, address), { signal: invocation.control.signal }); assert.equal(response.status, fixture.nextRuntime.status, asset); bytes += (await response.arrayBuffer()).byteLength; }
    assert.ok(lines.some(line => line.includes("Next.js " + manifest.version)));
    await mkdir(output, { recursive: true }); await writeFile(join(output, `next-${mode}-runtime.html`), html);
  } finally { controller.abort(); await completed; }
  assert.ok(lines.some(line => line.includes("local modules without ancestor fallback")));
  })));
}

/** 🔥️ Proves actual lazy Next compilation, cached serving during another graph and owned child cancellation. */
export async function proveNextDevRuntime(workspace: string, output: string, invocation: ScriptInvocation): Promise<void> {
  const mode = readBuildMode(process.env.SEMIO_BUILD_MODE), fixture = JSON.parse(await readFile(join(import.meta.dir, "../🏗️web-build/🧫️cases.json"), "utf8")), packageRoot = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript");
  const stop = new AbortController(), signal = AbortSignal.any([stop.signal, invocation.control.signal]), lines: string[] = [];
  let port = 0, ready!: () => void, coldQueued!: () => void, observeCold = false;
  const started = new Promise<void>(accept => { ready = accept; }), queued = new Promise<void>(accept => { coldQueued = accept; });
  const child = runOwnedCommand(process.execPath, [join(packageRoot, "📜️script.ts"), "dev", "--port", "0", "--hostname", "127.0.0.1"], packageRoot, "next:dev-runtime", scriptInvocationBudget(invocation, 600_000), { signal, env: scriptInvocationEnvironment(invocation, { ...process.env, SEMIO_BUILD_MODE: mode }), onLine(line) { lines.push(line); const match = line.match(/http:\/\/[^ ]+:(\d+)/u); if (match) port = Number(match[1]); if (line.includes("Ready in")) ready(); if (observeCold && line.includes('"command":"next:development-rebuild"')) coldQueued(); } });
  const completed = child.then(() => { throw Error("Development server ended before cancellation"); }, error => { if (!stop.signal.aborted) throw error; });
  try {
    await Promise.race([started, completed]); assert.ok(port > 0);
    const address = `http://127.0.0.1:${port}`, initial = await fetch(address, { signal: invocation.control.signal }); assert.equal(initial.status, fixture.nextRuntime.status);
    const html = await initial.text(), assets = [...new Set([...html.matchAll(/(?:src|href)="([^" ]*\/_next\/static\/[^" ]+)"/gu)].map(match => { const asset=match[1];assert.ok(asset!==undefined,"Matched Next static asset capture required");return asset.replaceAll("&amp;", "&"); }))];
    for (const asset of assets) { const response = await fetch(new URL(asset, address), { signal: invocation.control.signal }); assert.equal(response.status, fixture.nextRuntime.status); await response.arrayBuffer(); }
    let cold!: Promise<Response>, coldCompleted = false;
    await withCountedBunCapacity(join(workspace, ".cache/b/l/m/data"), scriptCompilerAdmissionControlV1(invocation, "next:idle-probe", 600_000), async () => {
      const warm = await fetch(address, { signal: AbortSignal.any([invocation.control.signal, AbortSignal.timeout(5_000)]) }); assert.equal(warm.status, fixture.nextRuntime.status); await warm.text();
      observeCold = true; cold = fetch(address + fixture.nextRuntime.coldPath, { signal: invocation.control.signal }).then(response => { coldCompleted = true; return response; });
      await Promise.race([queued, cold.then(() => { throw Error("Cold route bypassed compiler admission"); }), completed]); assert.equal(coldCompleted, false);
    });
    const response = await cold; assert.equal(response.status, fixture.nextRuntime.status); assert.equal(await response.text(), fixture.nextRuntime.coldBody);
    await mkdir(output, { recursive: true }); await writeFile(join(output, `next-${mode}-dev-runtime.html`), html);
  } finally { stop.abort(); await completed; }
  await withCountedBunCapacity(join(workspace, ".cache/b/l/m/data"), scriptCompilerAdmissionControlV1(invocation, "next:cancel-probe", 600_000), async () => {});
}
