#!/usr/bin/env bun
import { join } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
/** 📐️ Executes the complete schema-first neutral region surface corpus under an owned finite child. */
class SurfaceContractScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("Surface contract accepts no implicit selection");
  await runRepositoryTestCommand(process.execPath,["test",join(this.repoRoot,"✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,env:repoTestArtifactEnvironment(this.repoRoot,"fem2d-surface-contract"),budgetMs:120000});
 }
}
/** 🔮️ Validates real neutral native output against the complete portable corpus and independent oracle. */
class NeutralSurfaceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("Neutral surface oracle accepts no implicit selection");
  await runRepositoryTestCommand(process.execPath,["test",join(this.repoRoot,"✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧪️tests/🔬️neutral/🟦️.ts")],{cwd:this.repoRoot,env:repoTestArtifactEnvironment(this.repoRoot,"fem2d-neutral-surface-oracle"),budgetMs:120000});
 }
}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test-surface-contract",SurfaceContractScript).register("test-neutral-surface",NeutralSurfaceScript));
