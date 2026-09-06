#!/usr/bin/env bun
/** 🖨️ Raster TypeScript package — runs every bun test this plugin owns (examples + io parity twins). */

import { resolve } from "node:path";
import { BundleScript, ScriptRouter, runCmd, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const CASES = [
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🟦️.ts",
];

class TestScript extends BundleScript {
  run(): void {
    runCmd(process.execPath, ["test", ...CASES.map(path => resolve(this.repoRoot, path))], { cwd: this.repoRoot });
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
