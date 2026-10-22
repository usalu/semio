#!/usr/bin/env repo
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runAdmittedCargoPackageTestsV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { runFormSourceScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

/** 🧪️ Runs the original extension contribution laws with their defining Flow owner. */
class ExtensionRegistryScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw new Error("test-extension-registry accepts no arguments");
  await runAdmittedCargoPackageTestsV1(["semio-framework-os-flow-operators"],this.root,this.invocation,["--lib","registry::contribution_tests"]);
 }
}
/** 📝️ Runs original widget loans against the portable independent producer oracle. */
class FormProducerScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw new Error("test-form-producer accepts no arguments");
  await runAdmittedCargoPackageTestsV1(["semio-framework-os-flow-vcs"],this.root,this.invocation,["--lib","forms::tests"]);
 }
}
/** 🧬️ Validates the entire portable original field-loan corpus with the independent schema oracle. */
class FormSourceScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runFormSourceScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router=new ScriptRouter(import.meta.dirname).register("test-extension-registry",ExtensionRegistryScript).register("test-form-producer",FormProducerScript).register("test-form-source",FormSourceScript);
if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));
