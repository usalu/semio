/** 🪶️ Wasm opt of the plugin web materialization: the shipped optimization arguments, the selected tool and the blocking and asynchronous optimization of the extracted core modules. */
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { readdirSync, renameSync } from "node:fs";
import { join, resolve } from "node:path";
import { preparedBinaryen } from "@semio-tech/nx-bootstrap-tools/wasm";
import { runCmdStatus } from "@semio-tech/process/execution";
import { semioBuildMode } from "@semio-tech/repo-lib/process";
import type { PluginWebMaterializeContext } from "../🧾️contract/🟦️.ts";

const WASM_OPT_ARGS: readonly string[] = [
  "-Oz",
  "--low-memory-unused",
  "--strip-debug",
  "--strip-producers",
  "--enable-bulk-memory",
  "--enable-bulk-memory-opt",
  "--enable-call-indirect-overlong",
  "--enable-extended-const",
  "--enable-multivalue",
  "--enable-mutable-globals",
  "--enable-nontrapping-float-to-int",
  "--enable-reference-types",
  "--enable-sign-ext",
];

/** 🪶️ Optimizes extracted component cores with the native tool prepared by the build tool. */
export function optimizePluginCoreModules(outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): void {
  if (!(ctx.optimize ?? (semioBuildMode() === "release" && process.env.SEMIO_WASM_OPT !== "0"))) return;
  const selected = ctx.wasmOptBin ?? process.env.SEMIO_WASM_OPT_BIN;
  const wasmOptBin = selected ? resolve(ctx.repoRoot, selected) : preparedBinaryen(ctx.repoRoot);
  for (const file of readdirSync(outDir)) {
    if (!file.startsWith(`${componentBase}.core`) || !file.endsWith(".wasm")) continue;
    const coreWasm = join(outDir, file);
    const optimized = `${coreWasm}.opt`;
    if (runCmdStatus(wasmOptBin, [coreWasm, ...WASM_OPT_ARGS, "-o", optimized], { cwd: ctx.repoRoot, budgetMs: buildBudgetMs() }) !== 0) {
      throw new Error(`wasm-opt failed for ${coreWasm}`);
    }
    renameSync(optimized, coreWasm);
  }
}

/** 🎚️ The `wasm-opt` executable a context selects and the exact arguments every shipped core is optimized with. */
export function pluginWasmOptToolV1(ctx: Pick<PluginWebMaterializeContext, "repoRoot" | "wasmOptBin">): Readonly<{ command: string; args: readonly string[] }> {
  const selected = ctx.wasmOptBin ?? process.env.SEMIO_WASM_OPT_BIN;
  return Object.freeze({ command: selected ? resolve(ctx.repoRoot, selected) : preparedBinaryen(ctx.repoRoot), args: WASM_OPT_ARGS });
}

/** 🧩️ The extracted core modules jco wrote for one component, in name order. */
export function pluginCoreModuleFilesV1(outDir: string, componentBase: string): readonly string[] {
  return readdirSync(outDir).filter((file) => file.startsWith(`${componentBase}.core`) && file.endsWith(".wasm")).sort();
}

/** 🪶️ Optimizes one extracted core with `wasm-opt -Oz` into `output`, on `ctx.cores` threads (Binaryen's `BINARYEN_CORES`) when the caller holds that many job slots. */
export async function optimizePluginCoreModuleAsync(core: string, output: string, ctx: PluginWebMaterializeContext): Promise<void> {
  const tool = pluginWasmOptToolV1(ctx);
  try { await captureOwnedCommandV1(tool.command, [core, ...tool.args, "-o", output], ctx.repoRoot, { env: { ...(ctx.environment ?? process.env), ...(ctx.cores === undefined ? {} : { BINARYEN_CORES: String(ctx.cores) }) }, ...(ctx.signal ? { signal: ctx.signal } : {}), ...(ctx.control ? { control: ctx.control } : {}) }); }
  catch (cause) { ctx.signal?.throwIfAborted(); throw new Error(`wasm-opt failed for ${core}`, { cause }); }
}

/** 🪶️ Async twin of {@link optimizePluginCoreModules}: the same ship-mode-only `wasm-opt` pass over every extracted core, spawned without blocking. */
export async function optimizePluginCoreModulesAsync(outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): Promise<void> {
  if (!(ctx.optimize ?? (semioBuildMode() === "release" && process.env.SEMIO_WASM_OPT !== "0"))) return;
  for (const file of pluginCoreModuleFilesV1(outDir, componentBase)) {
    const coreWasm = join(outDir, file);
    await optimizePluginCoreModuleAsync(coreWasm, `${coreWasm}.opt`, ctx);
    renameSync(`${coreWasm}.opt`, coreWasm);
  }
}
