#!/usr/bin/env bun
/** 🏃️ Shared playbook artifact package router. */
import { runArtifactRustPackageMain, runAdmittedCargoPackageTestsV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 📖️ Runs the actual selected child producer over the portable shared form law. */
class FlowFormsScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("test-flow-forms accepts no arguments");
  await runAdmittedCargoPackageTestsV1(["semio-framework-artifact-playbook-playbook"],this.root,this.invocation,["--lib","flow_forms::tests"]);
 }
}
if (process.argv[2] === "generation-root-policy") {
  const { proceduralGenerationRootSelfTests } = await import("../../🧬️generation/🧪️tests/🔬️procedural-generation-root/🟦️.ts");
} else {
  await runArtifactRustPackageMain(import.meta.dir, "semio-framework-artifact-playbook-playbook",{commands:{"test-flow-forms":FlowFormsScript}});
}
