import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import type { CrateMeasure } from "../🧾️model/🟦️.ts";
import { readRustSourceClosureV1, type RustSourceInputV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import type { CrateBudget } from "../💰️budget/🟦️.ts";

const HOST_CFG: { value: string[] | null } = { value: null };

const WASM_CFG: { value: string[] | null } = { value: null };

/** 🖥️ Host cfg snapshot used by the census and the gate. */
export function crateTreeHostCfg(): readonly string[] {
  if (HOST_CFG.value) return HOST_CFG.value;
  const rustc = execFileSync("rustup", ["which", "rustc"], { encoding: "utf8" }).trim();
  const cfg = execFileSync(rustc, ["--print", "cfg"], { encoding: "utf8" }).trim().split("\n").map((value) => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
  return (HOST_CFG.value = [...cfg, "debug_assertions"]);
}

/** 🌉️ The cfg of a `wasm32-unknown-unknown` build, used to scan the code that only the guest target compiles. */
export function crateTreeWasmCfg(): readonly string[] {
  if (WASM_CFG.value) return WASM_CFG.value;
  const rustc = execFileSync("rustup", ["which", "rustc"], { encoding: "utf8" }).trim();
  const cfg = execFileSync(rustc, ["--print", "cfg", "--target", "wasm32-unknown-unknown"], { encoding: "utf8" }).trim().split("\n").map((value) => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
  return (WASM_CFG.value = [...cfg, "debug_assertions"]);
}

/** 📐️ Measures a crate root through the canonical source-closure counter, with optional in-memory files; `cfgOverride` replaces the host cfg (the `test` flag is still added). */
export async function measureRustCrate(repoRoot: string, entry: string, withTest: boolean, overlay: ReadonlyMap<string, string> = new Map(), manifestDirectory?: string, cfgOverride?: readonly string[]): Promise<CrateMeasure> {
  const read = async (path: string): Promise<Uint8Array | null> => {
    const virtual = overlay.get(path);
    if (virtual !== undefined) return Buffer.from(virtual, "utf8");
    try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
  };
  const cfg = [...(cfgOverride ?? crateTreeHostCfg()), ...(withTest ? ["test"] : [])];
  const inputs = await readRustSourceClosureV1([entry], repoRoot, cfg, read, { manifestDirectory: manifestDirectory ?? dirname(entry), generated: () => undefined });
  return summarize(inputs);
}

function summarize(inputs: readonly RustSourceInputV1[]): CrateMeasure {
  return { files: inputs.length, bytes: inputs.reduce((sum, input) => sum + input.bytes, 0), lines: inputs.reduce((sum, input) => sum + input.activeLines, 0), embedded: inputs.filter((input) => input.kind === "embedded").length, paths: inputs.map((input) => input.path), sourcePaths: inputs.filter((input) => input.kind === "source").map((input) => input.path), entries: new Map(inputs.map((input) => [input.path, { bytes: input.bytes, lines: input.activeLines }])) };
}

/** ✅️ Whether a measure fits a budget. */
export function measureFits(measure: CrateMeasure, budget: CrateBudget): boolean {
  return measure.files <= budget.files && measure.bytes <= budget.bytes && measure.lines <= budget.lines;
}

/** 🧪️ Source text of every file of a measure, keyed by repo-relative path. */
export function readMeasureSources(repoRoot: string, measure: CrateMeasure, overlay: ReadonlyMap<string, string> = new Map()): Map<string, string> {
  const out = new Map<string, string>();
  for (const path of measure.sourcePaths) {
    if (!path.endsWith(".rs")) continue;
    const absolute = resolve(repoRoot, path);
    const virtual = overlay.get(absolute);
    if (virtual !== undefined) out.set(path, virtual);
    else if (existsSync(absolute)) out.set(path, readFileSync(absolute, "utf8"));
  }
  return out;
}
