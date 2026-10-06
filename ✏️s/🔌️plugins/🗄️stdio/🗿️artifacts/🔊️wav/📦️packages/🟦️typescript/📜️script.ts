#!/usr/bin/env bun
/** 📦️ wav TypeScript artifact package router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
import { resolve } from "node:path";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
const command = process.argv[2] ?? "test";
if (command === "test" && process.argv.slice(3).some((argument) => argument !== "--")) {
  throw new Error("This artifact test target runs its complete registered suite and accepts no test selectors.");
}
await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/stdio-wav", { suites: ["🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });

if (command === "test") {
  const mutations = resolve(import.meta.dir, "../../🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
  const test = resolve(mutations, "🧪️tests/🟦️.test.ts");
  const audioEdit = resolve(import.meta.dir, "../../🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎮️commands/🔊️edit-audio");
  const audioTest = resolve(audioEdit, "🧪️tests/🟦️.test.ts");
  await runRepositoryCommand(process.execPath, ["x", "--no-install", "tsc", "--noEmit", "--strict", "--skipLibCheck", "--target", "ES2022", "--module", "ESNext", "--moduleResolution", "Bundler", "--allowImportingTsExtensions", resolve(mutations, "🟦️.ts"), resolve(mutations, "📝️text/🟦️.ts"), resolve(mutations, "💾️binary/🟦️.ts"), test, resolve(audioEdit, "🟦️.ts"), audioTest], getWorkspaceRoot(), "wav-mutation-facet-types", 120_000);
  await runRepositoryCommand(process.execPath, ["test", test, audioTest], getWorkspaceRoot(), "wav-mutation-facet-contract", 120_000);
}
