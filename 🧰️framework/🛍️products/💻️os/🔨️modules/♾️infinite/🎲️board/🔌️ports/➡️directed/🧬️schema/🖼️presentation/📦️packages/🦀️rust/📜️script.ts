#!/usr/bin/env repo
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runAdmittedCargoPackageTestsV1 } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { runSourceScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

/** 🧬️ Validates the portable corpus with the independent JSON schema oracle. */
class SourceScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runSourceScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

/** 🦀️ Runs the defining owner's portable law through original repository execution. */
class NativeScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw new Error("test-native accepts no arguments");
  await runAdmittedCargoPackageTestsV1(["semio-framework-os-infinite-directed-presentation"],this.root,this.invocation,["--lib"]);
 }
}
const router=new ScriptRouter(import.meta.dirname).register("test-source",SourceScript).register("test-native",NativeScript);
if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));
