#!/usr/bin/env bun
/** 📦️ mp4 TypeScript artifact package router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts";
import { resolve } from "node:path";
import { findWorkspaceRoot } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
const command = process.argv[2] ?? "test";
if (command === "test" && process.argv.slice(3).some((argument) => argument !== "--")) {
  throw new Error("This artifact test target runs its complete registered suite and accepts no test selectors.");
}
await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/stdio-mp4",{ bundleRuntime: "bun",suites:["🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"],afterTest:async original=>{
  const mutations = resolve(import.meta.dir, "../../🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
  const test = resolve(mutations, "🧪️tests/🟦️.test.ts");
  await runRepositoryCommand(original,process.execPath,["x", "--no-install", "tsc", "--noEmit", "--strict", "--skipLibCheck", "--target", "ES2022", "--module", "ESNext", "--moduleResolution", "Bundler", "--allowImportingTsExtensions", resolve(mutations, "🟦️.ts"), resolve(mutations, "📝️text/🟦️.ts"), resolve(mutations, "💾️binary/🟦️.ts"), test],findWorkspaceRoot(),"mp4-mutation-facet-types",{version:1,maximumElapsedMilliseconds:120_000,output:{maximumBytes:134217728,maximumLines:65536}});
  await runRepositoryCommand(original,process.execPath,["test", test],findWorkspaceRoot(),"mp4-mutation-facet-contract",{version:1,maximumElapsedMilliseconds:120_000,output:{maximumBytes:134217728,maximumLines:65536}});
}});


