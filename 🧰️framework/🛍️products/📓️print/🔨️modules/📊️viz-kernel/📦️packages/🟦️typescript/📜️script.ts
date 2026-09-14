#!/usr/bin/env bun
/** 📊️ `@semio-tech/print-viz-kernel` router: `bun ./📜️script.ts build|test [level]`.
 * `test` runs the kernel's own differential harness: every module measured against the d3 package
 * that is its registered oracle. The check table and its d3 imports live in `🔬️probes/🟦️.ts`; the
 * kernel modules themselves have no runtime dependency on anything outside this repository.
 */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { VIZ_KERNEL_LEVELS, vizKernelChecks, type Level } from "./🔬️probes/🟦️.ts";

//#region 🔖️Comparison
type Failure = { readonly path: string; readonly ours: unknown; readonly oracle: unknown };

function compare(ours: unknown, oracle: unknown, tolerance: number, path = "$", failures: Failure[] = []): Failure[] {
  if (failures.length > 6) return failures;
  if (typeof ours === "number" || typeof oracle === "number") {
    const a = Number(ours);
    const b = Number(oracle);
    const equal = (Number.isNaN(a) && Number.isNaN(b)) || Math.abs(a - b) <= tolerance * Math.max(1, Math.abs(a), Math.abs(b));
    if (!equal) failures.push({ path, ours, oracle });
    return failures;
  }
  if (Array.isArray(ours) || Array.isArray(oracle)) {
    const a = Array.isArray(ours) ? ours : [];
    const b = Array.isArray(oracle) ? oracle : [];
    if (a.length !== b.length) failures.push({ path: `${path}.length`, ours: a.length, oracle: b.length });
    for (let i = 0; i < Math.min(a.length, b.length); i += 1) compare(a[i], b[i], tolerance, `${path}[${i}]`, failures);
    return failures;
  }
  if (ours !== null && oracle !== null && typeof ours === "object" && typeof oracle === "object") {
    const keys = [...new Set([...Object.keys(ours as object), ...Object.keys(oracle as object)])].sort();
    for (const key of keys) compare((ours as Record<string, unknown>)[key], (oracle as Record<string, unknown>)[key], tolerance, `${path}.${key}`, failures);
    return failures;
  }
  if (ours !== oracle) failures.push({ path, ours, oracle });
  return failures;
}
//#endregion 🔖️Comparison

//#region 🔖️Router
const LEVELS = VIZ_KERNEL_LEVELS;

function resolveLevel(segments: readonly string[]): { level: Level; modules: readonly string[] } {
  const level = (LEVELS as readonly string[]).includes(segments[0] ?? "") ? (segments[0] as Level) : "quick";
  const rest = (LEVELS as readonly string[]).includes(segments[0] ?? "") ? segments.slice(1) : segments;
  return { level, modules: rest.filter((segment) => !segment.startsWith("-")) };
}

/** 🧪️ Runs every registered differential check for the requested level and module filter. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, modules } = resolveLevel(segments);
    const checks = (await vizKernelChecks(level)).filter((check) => modules.length === 0 || modules.includes(check.module));
    if (checks.length === 0) throw new Error(`no checks for level ${level}${modules.length > 0 ? ` and module(s) ${modules.join(", ")}` : ""}`);
    let passed = 0;
    const failed: string[] = [];
    const errored: string[] = [];
    const byModule = new Map<string, { passed: number; failed: number }>();
    for (const check of checks) {
      const tally = byModule.get(check.module) ?? { passed: 0, failed: 0 };
      byModule.set(check.module, tally);
      let outcome: Failure[];
      try {
        outcome = compare(await check.subject(), await check.oracle(), check.tolerance ?? 1e-9);
      } catch (error) {
        errored.push(`${check.module}/${check.name}: ${(error as Error).message}`);
        tally.failed += 1;
        continue;
      }
      if (outcome.length === 0) {
        passed += 1;
        tally.passed += 1;
        continue;
      }
      tally.failed += 1;
      failed.push(`${check.module}/${check.name}: ${outcome.map((failure) => `${failure.path} ours=${JSON.stringify(failure.ours)} oracle=${JSON.stringify(failure.oracle)}`).join("; ")}`);
    }
    for (const [module, tally] of [...byModule].sort((a, b) => a[0].localeCompare(b[0]))) console.log(`[viz-kernel] ${module.padEnd(11)} passed=${tally.passed} failed=${tally.failed}`);
    for (const line of [...failed, ...errored]) console.error(`[viz-kernel] FAIL ${line}`);
    console.log(`[viz-kernel] level=${level} checks=${checks.length} passed=${passed} failed=${failed.length} errored=${errored.length}`);
    if (failed.length > 0 || errored.length > 0) process.exit(1);
  }
}

/** 🏗️ Type-checks the kernel by loading every module through the barrel. */
class BuildScript extends BundleScript {
  async run(): Promise<void> {
    const barrel = (await import("./🟦️.ts")) as Record<string, unknown>;
    console.log(`[viz-kernel] barrel exports ${Object.keys(barrel).length} symbols`);
  }
}
//#endregion 🔖️Router



const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
