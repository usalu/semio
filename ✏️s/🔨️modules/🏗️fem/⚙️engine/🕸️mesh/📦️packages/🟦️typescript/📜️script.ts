#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { join } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
/** 📐️ Executes the complete schema-first neutral region surface corpus under an owned finite child. */
class SurfaceContractScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("Surface contract accepts no implicit selection");
  await runRepositoryTestCommand(process.execPath,["test",join(this.repoRoot,"✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,env:repoTestArtifactEnvironment(this.repoRoot,"fem2d-surface-contract"),budgetMs:120000});
 }
}
/** 🔮️ Validates real neutral native output against the complete portable corpus and independent oracle. */
class NeutralSurfaceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("Neutral surface oracle accepts no implicit selection");
  await runRepositoryTestCommand(process.execPath,["test",join(this.repoRoot,"✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧪️tests/🔬️neutral/🟦️.ts")],{cwd:this.repoRoot,env:repoTestArtifactEnvironment(this.repoRoot,"fem2d-neutral-surface-oracle"),budgetMs:120000});
 }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test-surface-contract",SurfaceContractScript).register("test-neutral-surface",NeutralSurfaceScript), { invocation: original }));
