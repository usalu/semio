#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 💤️ `@leutwiler/realparts-of-powers-z-n` task router: `repo ./📜️script.ts <deps|build|test> [lean|📯️notes|🏆️proof]`. */
import { BundleScript, ScriptRouter } from "../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{createIssuedResourceControlV1}from"../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withPrintExecutionV1}from"../../🧰️framework/🛍️products/📓️print/📦️artifacts/🎛️operation/🟦️.ts";
import{preparePrintMaterialV1}from"../../🧰️framework/🛍️products/📓️print/📦️artifacts/📥️material/📤️producer/🟦️.ts";
import { buildLean,buildPrintDocument,prepareLeanDependencies,verifyAxioms } from "./🟦️.ts";

class DepsScript extends BundleScript {
  async run(): Promise<void> {
    await prepareLeanDependencies(this.root,this.repoRoot);const phaseControl=createIssuedResourceControlV1(this.invocation,"print:leutwiler:prepare",0);await withPrintExecutionV1({workspace:this.repoRoot,invocation:this.invocation,phaseControl,kind:"document"},async execution=>{for(const kind of["tectonic","tex","fonts"]as const)await preparePrintMaterialV1(execution.operation,kind,execution.phaseControl);});
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const target=segments[0]??"lean";if(target==="lean"){await buildLean(this.root,this.repoRoot);return;}const phaseControl=createIssuedResourceControlV1(this.invocation,"print:leutwiler:build",0);await withPrintExecutionV1({workspace:this.repoRoot,invocation:this.invocation,phaseControl,kind:"document"},execution=>buildPrintDocument(execution,target));
  }
}

class TestScript extends BundleScript {
  async run(): Promise<void> {
    await verifyAxioms(this.root, this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("deps", DepsScript).register("build", BuildScript).register("test", TestScript);

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => router.run(process.argv.slice(2), original));
