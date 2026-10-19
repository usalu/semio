#!/usr/bin/env bun
/** 📦️ png TypeScript artifact package router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts";
import { resolve } from "node:path";
import { findWorkspaceRoot } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
const command = process.argv[2] ?? "test";
if (command === "test" && process.argv.slice(3).some((argument) => argument !== "--")) {
  throw new Error("This artifact test target runs its complete registered suite and accepts no test selectors.");
}
await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/stdio-png", { bundleRuntime: "bun", suites: ["🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts","🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧬️owned-native-contract/🟦️.ts"] ,afterTest:async original=>{
  await runRepositoryCommand(original,process.execPath,["x", "--no-install", "tsc", "--noEmit", "--strict", "--skipLibCheck", "--target", "ES2022", "--module", "ESNext", "--moduleResolution", "Bundler", "--allowImportingTsExtensions", "--resolveJsonModule", resolve(import.meta.dir, "../../🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🩹️patch-pixel-region/🟦️.ts")],findWorkspaceRoot(),"artifact-primary-editor-types",{version:1,maximumElapsedMilliseconds:120_000,output:{maximumBytes:134217728,maximumLines:65536}});
  await runRepositoryCommand(original,process.execPath,["test", resolve(import.meta.dir, "../../🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🩹️patch-pixel-region/🧪️tests/🟦️.ts")],findWorkspaceRoot(),"png-pixel-region-fixture",{version:1,maximumElapsedMilliseconds:120_000,output:{maximumBytes:134217728,maximumLines:65536}});
}});


