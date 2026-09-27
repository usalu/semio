#!/usr/bin/env bun
/** 📦️ png TypeScript artifact package router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
import { resolve } from "node:path";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
const command = process.argv[2] ?? "test";
if (command === "test" && process.argv.slice(3).some((argument) => argument !== "--")) {
  throw new Error("This artifact test target runs its complete registered suite and accepts no test selectors.");
}
await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/stdio-png");

if (command === "test") {
  await runOwnedCommand(process.execPath, ["x", "--no-install", "tsc", "--noEmit", "--strict", "--skipLibCheck", "--target", "ES2022", "--module", "ESNext", "--moduleResolution", "Bundler", "--allowImportingTsExtensions", "--resolveJsonModule", resolve(import.meta.dir, "../../🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🩹️patch-pixel-region/🟦️.ts")], getWorkspaceRoot(), "artifact-primary-editor-types", 120_000);
  await runOwnedCommand(process.execPath, ["test", resolve(import.meta.dir, "../../🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🩹️patch-pixel-region/🧪️tests/🟦️.test.ts")], getWorkspaceRoot(), "png-pixel-region-fixture", 120_000);
}
