#!/usr/bin/env bun
/** 📦️ zip TypeScript artifact package router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
import { resolve } from "node:path";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
const command = process.argv[2] ?? "test";
if (command === "test" && process.argv.slice(3).some((argument) => argument !== "--")) {
  throw new Error("This artifact test target runs its complete registered suite and accepts no test selectors.");
}
await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/stdio-zip", { suites: ["🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts", "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/📦️public/🟦️.ts"] });

if (command === "test") {
  await runOwnedCommand(process.execPath, ["x", "--no-install", "tsc", "--noEmit", "--strict", "--skipLibCheck", "--target", "ES2022", "--module", "ESNext", "--moduleResolution", "Bundler", "--allowImportingTsExtensions", "--resolveJsonModule", resolve(import.meta.dir, "../../✏️editor/🟦️.ts"), resolve(import.meta.dir, "../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts")], getWorkspaceRoot(), "artifact-primary-editor-types", 120_000);
  await runOwnedCommand(process.execPath, ["test", resolve(import.meta.dir, "../../✏️editor/🧪️tests/🟦️.test.ts")], getWorkspaceRoot(), "archive-primary-fixture", 120_000);
}
