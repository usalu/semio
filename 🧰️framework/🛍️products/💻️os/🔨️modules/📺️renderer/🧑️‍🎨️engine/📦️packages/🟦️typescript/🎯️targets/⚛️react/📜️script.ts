#!/usr/bin/env bun
/** @emoji 🎨️ `@semio-tech/framework-renderer-react` task router. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, runBunx, runVitest } from "../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root);
  }
}

//#region 🔖️LintScript
const REGION_BALANCE_FILES = ["🟦️.tsx"] as const;

/** 🧭️Counts unmatched `//#region` / `//#endregion` markers per file — a typo'd region silently corrupts the file's canonical structure. */
function collectRegionBalanceViolations(root: string): string[] {
  const violations: string[] = [];
  for (const name of REGION_BALANCE_FILES) {
    const text = readFileSync(join(root, name), "utf8");
    const opens = (text.match(/#region\b/g) ?? []).length;
    const closes = (text.match(/#endregion\b/g) ?? []).length;
    if (opens !== closes) {
      violations.push(`${name}: ${opens} #region marker(s) vs ${closes} #endregion marker(s)`);
    }
  }
  return violations;
}

/** 🧭️Every host registered in `COMPONENT_SCENE_HOSTS` must have exactly one `export function XxxHost(...: ComponentSceneHostProps)` in `🟦️.tsx` — the contract the host registry table dispatches against. */
function collectHostSignatureViolations(root: string): string[] {
  const violations: string[] = [];
  const text = readFileSync(join(root, "🟦️.tsx"), "utf8");
  const registryNames = [...text.matchAll(/lazyHost\(\(\) => Promise\.resolve\(\{ ([A-Z][A-Za-z0-9]*Host) \}\), "\1"\)/g)].map((m) => m[1]!);
  const hostExportCounts = new Map<string, number>();
  for (const m of text.matchAll(/^export function ([A-Z][A-Za-z0-9]*Host)\([^)]*: ComponentSceneHostProps\)/gm)) {
    hostExportCounts.set(m[1]!, (hostExportCounts.get(m[1]!) ?? 0) + 1);
  }
  for (const name of registryNames) {
    const count = hostExportCounts.get(name) ?? 0;
    if (count === 0) {
      violations.push(`🟦️.tsx: no exported component matching ${name}(...: ComponentSceneHostProps)`);
    } else if (count > 1) {
      violations.push(`🟦️.tsx: multiple ${name} exports matching the host contract`);
    }
  }
  return violations;
}

class LintScript extends BundleScript {
  run(_segments: string[]): void {
    const violations = [...collectRegionBalanceViolations(this.root), ...collectHostSignatureViolations(this.root)];
    if (violations.length === 0) {
      console.log("framework-renderer-react: region/host-contract lint passed");
      return;
    }
    console.error(`framework-renderer-react: found ${violations.length} lint violation(s):`);
    for (const v of violations) console.error(`  ${v}`);
    process.exit(1);
  }
}
//#endregion 🔖️LintScript

const router = new ScriptRouter(fileURLToPath(new URL(".", import.meta.url))).register("test", TestScript).register("lint", LintScript).register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url);
