#!/usr/bin/env bun
/** 📦️ mp4 TypeScript artifact package router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
import { resolve } from "node:path";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
const command = process.argv[2] ?? "test";
if (command === "test" && process.argv.slice(3).some((argument) => argument !== "--")) {
  throw new Error("This artifact test target runs its complete registered suite and accepts no test selectors.");
}
await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/stdio-mp4",{suites:["🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"]});

if (command === "test") {
  const mutations = resolve(import.meta.dir, "../../🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
  const test = resolve(mutations, "🧪️tests/🟦️.test.ts");
  await runRepositoryCommand(process.execPath, ["x", "--no-install", "tsc", "--noEmit", "--strict", "--skipLibCheck", "--target", "ES2022", "--module", "ESNext", "--moduleResolution", "Bundler", "--allowImportingTsExtensions", resolve(mutations, "🟦️.ts"), resolve(mutations, "📝️text/🟦️.ts"), resolve(mutations, "💾️binary/🟦️.ts"), test], getWorkspaceRoot(), "mp4-mutation-facet-types", 120_000);
  await runRepositoryCommand(process.execPath, ["test", test], getWorkspaceRoot(), "mp4-mutation-facet-contract", 120_000);
}
