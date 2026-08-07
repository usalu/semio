#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-lib` router: `bun ./📜️script.ts <lint|test [level]|workspaces <--write|--check>>`. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, computeWorkspaces, runBundleScriptMain, runBunx, resolveTestLevel, runTestBudgeted } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class LintScript extends BundleScript {
  run(): void {
    runBunx(["tsc", "-p", "tsconfig.json", "--noEmit"], this.root);
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runTestBudgeted(process.execPath, ["test", "./🧪️index.test.ts", ...rest], { cwd: this.root });
  }
}

//#region 🔖️WorkspacesScript
/** 🗂️ `bun ./📜️script.ts workspaces --write` regenerates root `package.json`'s `workspaces` array from
 * `computeWorkspaces()`; `--check` verifies without writing (exits 1 when stale). Never touches any
 * other root `package.json` field — see `26/08/06/GENERATED-BUN-WORKSPACES-FROM-PACKAGE-CATALOG`. */
class WorkspacesScript extends BundleScript {
  run(segments: string[]): void {
    const write = segments.includes("--write");
    const check = segments.includes("--check");
    if (write === check) {
      console.error("usage: bun ./📜️script.ts workspaces <--write|--check>");
      process.exit(1);
    }
    const rootPkgPath = join(this.repoRoot, "package.json");
    const rootPkg = JSON.parse(readFileSync(rootPkgPath, "utf8")) as Record<string, unknown>;
    const current = Array.isArray(rootPkg.workspaces) ? (rootPkg.workspaces as string[]) : [];
    const expected = computeWorkspaces(this.repoRoot);
    const fresh = current.length === expected.length && current.every((entry, i) => entry === expected[i]);
    if (check) {
      if (!fresh) {
        const missing = expected.filter((entry) => !current.includes(entry));
        const stale = current.filter((entry) => !expected.includes(entry));
        console.error(`root package.json workspaces is stale (${expected.length} expected, ${current.length} current).`);
        if (missing.length > 0) console.error(`  missing: ${missing.join(", ")}`);
        if (stale.length > 0) console.error(`  stale:   ${stale.join(", ")}`);
        if (missing.length === 0 && stale.length === 0) console.error("  (same set, different order)");
        console.error("run `bun ./📜️script.ts workspaces --write` to refresh.");
        process.exit(1);
      }
      console.log(`root package.json workspaces is fresh (${expected.length} packages).`);
      return;
    }
    if (fresh) {
      console.log(`root package.json workspaces already fresh (${expected.length} packages) — no write needed.`);
      return;
    }
    rootPkg.workspaces = expected;
    writeFileSync(rootPkgPath, `${JSON.stringify(rootPkg, null, 2)}\n`);
    console.log(`root package.json workspaces regenerated -> ${expected.length} packages.`);
  }
}
//#endregion 🔖️WorkspacesScript

const router = new ScriptRouter(import.meta.dir).register("lint", LintScript).register("test", TestScript).register("workspaces", WorkspacesScript);

await runBundleScriptMain(router, import.meta.url);
