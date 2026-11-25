#!/usr/bin/env repo
/** 🌐 Shared jco transpile + plugin web glue (dev runner + extension store).
 *
 * MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H2): `pluginWorkerSource`/`PLUGIN_WORKER_FILE`
 * (one Worker per plugin) are replaced by `shardWorkerSource`/`SHARD_WORKER_FILE` — ONE
 * package-agnostic `🟨️shard-worker.js`, served from `/🔌️plugin-modules/🧵️shard/`, multiplexed by
 * `actorId` across a bounded shard pool (see `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`'s
 * `ShardClient`, the client-side transport this worker pairs with). V8 reserves a 4 GiB guard region
 * per wasm module per worker — one-worker-per-plugin capped the browser at ~20 plugins; this is the
 * change that lifts that ceiling. */
import { captureOwnedCommandV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { runCmdStatus } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { rewritePreview2ShimImportSource } from "../🕸️imports/🟦️.ts";
import type { PluginWebMaterializeContext } from "./🧾️contract/🟦️.ts";
import { optimizePluginCoreModules, optimizePluginCoreModulesAsync } from "./🪶️wasm-opt/🟦️.ts";
import { stripWasmDebugSectionsV1 } from "./🪶️strip-debug/🟦️.ts";
import { nativeTranspileArgsV1, nativeTranspileBinaryV1, PLUGIN_COMPONENT_IMPORT_MAPS_V1 } from "./🦀️native-transpile/🟦️.ts";
import { rewriteJcoAsyncResultLiftingAt, rewriteJcoComponentFactoryAt } from "./🧬️jco-module-rewrites/🟦️.ts";

export function rewritePreview2ShimImports(componentJsPath: string, preview2VendorDir: string): void {
  const outDir = dirname(componentJsPath);
  const rel = relative(outDir, preview2VendorDir).replace(/\\/g, "/");
  const relativeUrl = rel === ".." || rel.startsWith("../") ? rel : `./${rel}`;
  const prefix = relativeUrl.endsWith("/") ? relativeUrl : `${relativeUrl}/`;
  const content = readFileSync(componentJsPath, "utf8");
  const rewritten = rewritePreview2ShimImportSource(content, prefix);
  if (rewritten !== content) writeFileSync(componentJsPath, rewritten);
}

export function transpilePluginComponent(artifact: string, outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): void {
  if (runCmdStatus(nativeTranspileBinaryV1(ctx.repoRoot, ctx.mode, ctx.environment), [...nativeTranspileArgsV1(artifact, outDir, componentBase, PLUGIN_COMPONENT_IMPORT_MAPS_V1)], { cwd: ctx.repoRoot }) !== 0) {
    throw new Error(`native transpile failed for ${artifact}`);
  }
  rewriteJcoAsyncResultLiftingAt(join(outDir, `${componentBase}.js`));
  rewriteJcoComponentFactoryAt(join(outDir, `${componentBase}.js`));
  optimizePluginCoreModules(outDir, componentBase, ctx);
  rewritePreview2ShimImports(join(outDir, `${componentBase}.js`), ctx.preview2VendorDir);
}

/** 🧬️ Runs native transpilation and rewrites async results and core asset URLs before optimization and vendor import assembly. */
export async function transpilePluginComponentModulesAsync(artifact: string, outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): Promise<void> {
  const slim = join(dirname(outDir), `${componentBase}.nodebug.wasm`);
  try {
    writeFileSync(slim, stripWasmDebugSectionsV1(readFileSync(artifact)));
    await captureOwnedCommandV1(nativeTranspileBinaryV1(ctx.repoRoot, ctx.mode, ctx.environment), [...nativeTranspileArgsV1(slim, outDir, componentBase, PLUGIN_COMPONENT_IMPORT_MAPS_V1)], ctx.repoRoot, { env: { ...(ctx.environment ?? process.env) }, ...(ctx.signal ? { signal: ctx.signal } : {}), ...(ctx.control ? { control: ctx.control } : {}) });
  } catch (cause) {
    ctx.signal?.throwIfAborted();
    const diagnostic = cause instanceof Error ? cause.message : String(cause);
    throw new Error(`native transpile failed for ${artifact}\n${diagnostic}`, { cause });
  } finally {
    rmSync(slim, { force: true });
  }
  rewriteJcoAsyncResultLiftingAt(join(outDir, `${componentBase}.js`));
  rewriteJcoComponentFactoryAt(join(outDir, `${componentBase}.js`));
}

/** 🚀️ Async twin of {@link transpilePluginComponent}: the same transpile invocation, native (no `--async-mode` flag, `pure` and `host-async` mapped to one shim, see
 * that function's doc), the ship-mode `wasm-opt` pass and the preview2 shim import rewrite, spawned without blocking so several components overlap. */
export async function transpilePluginComponentAsync(artifact: string, outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): Promise<void> {
  await transpilePluginComponentModulesAsync(artifact, outDir, componentBase, ctx);
  await optimizePluginCoreModulesAsync(outDir, componentBase, ctx);
  rewritePreview2ShimImports(join(outDir, `${componentBase}.js`), ctx.preview2VendorDir);
}
