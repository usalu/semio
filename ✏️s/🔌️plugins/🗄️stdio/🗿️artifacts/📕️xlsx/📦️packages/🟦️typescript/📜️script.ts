#!/usr/bin/env repo
/** 📦️ xlsx TypeScript artifact package router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/stdio-xlsx", {
  bundleRuntime: "runtime",
  suites: [
    "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧪️tests/🟦️.ts",
    "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
  ],
});
