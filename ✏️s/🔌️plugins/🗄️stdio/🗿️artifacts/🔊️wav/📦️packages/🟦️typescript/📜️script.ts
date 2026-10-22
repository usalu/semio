#!/usr/bin/env repo
/** 📦️ wav TypeScript artifact package router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts";
import { resolve } from "node:path";
import { findWorkspaceRoot } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
const command = process.argv[2] ?? "test";
if (command === "test" && process.argv.slice(3).some((argument) => argument !== "--")) {
  throw new Error("This artifact test target runs its complete registered suite and accepts no test selectors.");
}
await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/stdio-wav", { bundleRuntime: "runtime", suites: ["🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] ,afterTest:async original=>{
  const mutations = resolve(import.meta.dir, "../../🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
  const test = resolve(mutations, "🧪️tests/🟦️.ts");
  const audioEdit = resolve(import.meta.dir, "../../🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎮️commands/🔊️edit-audio");
  const audioTest = resolve(audioEdit, "🧪️tests/🟦️.ts");
  await runRepositoryCommand(original,process.execPath,["x", "--no-install", "tsc", "--noEmit", "--strict", "--skipLibCheck", "--target", "ES2022", "--module", "ESNext", "--moduleResolution", "Bundler", "--allowImportingTsExtensions", resolve(mutations, "🟦️.ts"), resolve(mutations, "📝️text/🟦️.ts"), resolve(mutations, "💾️binary/🟦️.ts"), test, resolve(audioEdit, "🟦️.ts"), audioTest],findWorkspaceRoot(),"wav-mutation-facet-types",{version:1,maximumElapsedMilliseconds:120_000,output:{maximumBytes:134217728,maximumLines:65536}});
  await runRepositoryCommand(original,process.execPath,["test", test, audioTest],findWorkspaceRoot(),"wav-mutation-facet-contract",{version:1,maximumElapsedMilliseconds:120_000,output:{maximumBytes:134217728,maximumLines:65536}});
}});


