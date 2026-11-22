#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { join } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { withScratchDirectoryV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧹️scratch/🟦️.ts";
import { nativeCommandPolicyV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { runRepositoryCommand, repoTestArtifactEnvironment } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
/** 📐️ Executes the complete schema-first neutral region surface corpus under an owned finite child. */
class SurfaceContractScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("Surface contract accepts no implicit selection");
  const io=join(this.repoRoot,"✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📦️packages/🦀️rust");
  await withScratchDirectoryV1("fem-surface-contract",async directory=>{
   const results=join(directory,"surface.json");
   await runRepositoryCommand(this.invocation,"cargo",["test","--offline","--manifest-path",join(io,"Cargo.toml"),"--lib","surface_corpus_exports_obj_and_stl_beside_the_neutral_surface","--","--nocapture"],io,"fem2d-surface-export-test",nativeCommandPolicyV1({maximumBytes:134217728,maximumLines:65536}),{env:{...process.env,SEMIO_FEM_SURFACE_RESULTS:results}});
   await runRepositoryTestCommand(process.execPath,["test",join(this.repoRoot,"✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,env:repoTestArtifactEnvironment(this.repoRoot,"fem2d-surface-contract",{...process.env,SEMIO_FEM_SURFACE_RESULTS:results}),budgetMs:120000});
  },{root:this.repoRoot});
 }
}
/** 🔮️ Validates real neutral native output against the complete portable corpus and independent oracle. */
class NeutralSurfaceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("Neutral surface oracle accepts no implicit selection");
  const native=join(this.root,"../🦀️rust");
  await withScratchDirectoryV1("fem-neutral-surface",async directory=>{
   const results=join(directory,"neutral.json");
   await runRepositoryCommand(this.invocation,"cargo",["test","--offline","--manifest-path",join(native,"Cargo.toml"),"--lib","surface_tests::complete_portable_region_surface_corpus","--","--exact","--nocapture"],native,"fem-mesh-surface-test",nativeCommandPolicyV1({maximumBytes:134217728,maximumLines:65536}),{env:{...process.env,SEMIO_FEM_NEUTRAL_RESULTS:results}});
   await runRepositoryTestCommand(process.execPath,["test",join(this.repoRoot,"✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧪️tests/🔬️neutral/🟦️.ts")],{cwd:this.repoRoot,env:repoTestArtifactEnvironment(this.repoRoot,"fem2d-neutral-surface-oracle",{...process.env,SEMIO_FEM_NEUTRAL_RESULTS:results}),budgetMs:120000});
  },{root:this.repoRoot});
 }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test-surface-contract",SurfaceContractScript).register("test-neutral-surface",NeutralSurfaceScript), { invocation: original }));
