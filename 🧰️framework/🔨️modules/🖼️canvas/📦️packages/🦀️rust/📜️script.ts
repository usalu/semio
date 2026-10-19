
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { stageAdmittedArtifactTreesV1 } from "../../../🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts";
import { readBuildMode } from "../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { runtimeAssetStageTrees } from "../../🔌️runtime-assets/🚚️staging/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTestScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🖼️ Executes every General Canvas owning package target: the producer crates bottom up, then the facade. */
class TestScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
/** 📦️ Stages the runtime asset directories (compiler fonts, label face, icon pack) as provenance-marked trees every production and play bundle copies, and the guest subset the package asset route serves. */
class StageRuntimeAssetsScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("stage-runtime-assets takes no arguments");
  const mode=readBuildMode(process.env.SEMIO_BUILD_MODE),phase=scriptCompilerAdmissionControlV1(this.invocation,"canvas:runtime-assets",60000);
  await stageAdmittedArtifactTreesV1({workspace:this.repoRoot,invocation:this.invocation,phase,mode,trees:productsRoot=>runtimeAssetStageTrees(productsRoot,mode)});
 }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test",TestScript).register("stage-runtime-assets",StageRuntimeAssetsScript), { invocation: original, defaultCommand: "test" }));
