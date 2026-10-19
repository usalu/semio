#!/usr/bin/env bun
/** 🌐 Shared jco transpile + plugin web glue (dev runner + extension store).
 *
 * MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H2): `pluginWorkerSource`/`PLUGIN_WORKER_FILE`
 * (one Worker per plugin) are replaced by `shardWorkerSource`/`SHARD_WORKER_FILE` — ONE
 * package-agnostic `🟨️shard-worker.js`, served from `/🔌️plugin-modules/🧵️shard/`, multiplexed by
 * `actorId` across a bounded shard pool (see `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`'s
 * `ShardClient`, the client-side transport this worker pairs with). V8 reserves a 4 GiB guard region
 * per wasm module per worker — one-worker-per-plugin capped the browser at ~20 plugins; this is the
 * change that lifts that ceiling. */
import { compilerCapacityWorkerCountV1, type CompilerAdmissionControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { resolveWorkspaceBin, runNodeBinStatus } from "@semio-tech/process/execution";
import { rewritePreview2ShimImportSource } from "../🕸️imports/🟦️.ts";
import type { PluginWebMaterializeContext } from "./🧾️contract/🟦️.ts";
import { optimizePluginCoreModules, optimizePluginCoreModulesAsync } from "./🪶️wasm-opt/🟦️.ts";
import { rewriteJcoAsyncResultLiftingAt, rewriteJcoComponentAssetUrlsAt } from "./🧬️jco-module-rewrites/🟦️.ts";

export function rewritePreview2ShimImports(componentJsPath: string, preview2VendorDir: string): void {
  const outDir = dirname(componentJsPath);
  const rel = relative(outDir, preview2VendorDir).replace(/\\/g, "/");
  const prefix = rel.endsWith("/") ? rel : `${rel}/`;
  const content = readFileSync(componentJsPath, "utf8");
  const rewritten = rewritePreview2ShimImportSource(content, prefix);
  if (rewritten !== content) writeFileSync(componentJsPath, rewritten);
}

export function transpilePluginComponent(artifact: string, outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): void {
  // 🧪️ terra-web-bridges (📓️terra-jco-spike-report.md "what must change" #2): NO `--async-mode`
  // flag — confirmed byte-identical to jco's bare/"sync" default for a component whose every WIT
  // function is already `async func` (`--async-mode jspi` was diffed against the bare transpile of
  // the SAME wasm and produced 0 bytes of difference). `world actor`'s import surface is now `pure`
  // (component.wit's `interface pure { log; now-ms; trace-span; }`, still plain `func`) PLUS
  // `host-async` (`interface host-async`, ~:887 — 24 `async func` imports + `emit`/`emit-patch`) —
  // both map to the SAME `🟨️.js`, which now implements both interfaces' exports from one file.
  if (
    runNodeBinStatus(
      ["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/pure=./🟨️.js", "--map", "semio:framework/host-async=./🟨️.js"],
      ctx.repoRoot,
    ) !== 0
  ) {
    throw new Error(`jco transpile failed for ${artifact}`);
  }
  rewriteJcoAsyncResultLiftingAt(join(outDir, `${componentBase}.js`));
  rewriteJcoComponentAssetUrlsAt(join(outDir, `${componentBase}.js`));
  optimizePluginCoreModules(outDir, componentBase, ctx);
  rewritePreview2ShimImports(join(outDir, `${componentBase}.js`), ctx.preview2VendorDir);
}

function spawnNodeBinAsync(args: readonly string[], cwd: string, signal?: AbortSignal, environment: Readonly<Record<string, string | undefined>> = process.env, control?: CompilerAdmissionControl): Promise<string> {
  const binName = args[0]!;
  const resolved = resolveWorkspaceBin(binName, cwd);
  return captureOwnedCommandV1("node", [`--v8-pool-size=${compilerCapacityWorkerCountV1(environment)}`, resolved ?? binName, ...args.slice(1)], cwd, { env: { ...environment, UV_THREADPOOL_SIZE: String(compilerCapacityWorkerCountV1(environment)) }, ...(signal ? { signal } : {}), ...(control ? { control } : {}) });
}

/** 🧬️ The location-independent half of a component's transpile: jco's module files with the async result lifting and the versioned core asset URLs
 * rewritten, before any `wasm-opt` pass and before the preview2 shim imports are pointed at a vendor directory. A pure function of the component bytes,
 * the jco version, `componentBase` and this file, so it is cached by content. */
export async function transpilePluginComponentModulesAsync(artifact: string, outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): Promise<void> {
  try {
    await spawnNodeBinAsync(["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/pure=./🟨️.js", "--map", "semio:framework/host-async=./🟨️.js"], ctx.repoRoot, ctx.signal, ctx.environment, ctx.control);
  } catch (cause) {
    ctx.signal?.throwIfAborted();
    throw new Error(`jco transpile failed for ${artifact}`, { cause });
  }
  rewriteJcoAsyncResultLiftingAt(join(outDir, `${componentBase}.js`));
  rewriteJcoComponentAssetUrlsAt(join(outDir, `${componentBase}.js`));
}

/** 🚀️ Async twin of {@link transpilePluginComponent}: the same jco invocation (no `--async-mode` flag, `pure` and `host-async` mapped to one shim, see
 * that function's doc), the ship-mode `wasm-opt` pass and the preview2 shim import rewrite, spawned without blocking so several components overlap. */
export async function transpilePluginComponentAsync(artifact: string, outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): Promise<void> {
  await transpilePluginComponentModulesAsync(artifact, outDir, componentBase, ctx);
  await optimizePluginCoreModulesAsync(outDir, componentBase, ctx);
  rewritePreview2ShimImports(join(outDir, `${componentBase}.js`), ctx.preview2VendorDir);
}
