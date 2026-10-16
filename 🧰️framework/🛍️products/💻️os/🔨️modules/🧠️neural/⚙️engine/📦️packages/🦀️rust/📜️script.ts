#!/usr/bin/env bun
import {runRepositoryExactCargoLaws as runExactCargoLaws,configuredRepositoryCargoPreparationPolicyV1} from "../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import {configuredExactCargoLawPolicyV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { readCargoTestPolicyV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 🧠️ Neural engine native and language-neutral lifecycle validation. */
import { runCargo } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

//#region 🧪️Validation
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if(segments[0]==="original-registry-admission-source"||segments[0]==="original-registry-admission-native"){
      if(segments.length!==1)throw Error("Original registry admission requires exact one cohort");const remaining=this.invocation.control.remainingMilliseconds();if(remaining===null||remaining<=0)throw Error("Original registry admission requires finite original deadline");
      await runOwnedCommand(process.execPath,["test",resolve(this.root,"../../📔️registry/🏗️admission/🧪️tests/🟦️.ts")],this.repoRoot,"neural:original-registry-admission-source",remaining,{env:process.env});
      if(segments[0]==="original-registry-admission-native"){const left=this.invocation.control.remainingMilliseconds();if(left===null||left<=0)throw Error("Original registry admission deadline expired");await runExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:left,listMilliseconds:left,lawMilliseconds:left},manifestPaths:{"semio-framework-os-kernel-neural-engine":resolve(this.root,"Cargo.toml")},cwd:this.repoRoot,groups:[{package:"semio-framework-os-kernel-neural-engine",target:{kind:"test",name:"registry_admission"},laws:["original_registry_issuer_reserves_two_shells_before_same_source_donation","original_registry_issuer_cancellation_returns_same_payload_after_born_shell_close"]}]});}return;
    }
    const { rest } = resolveTestLevel(segments);
    for (const crate of ["model", "registry", "budgeted", ""]) await runCargo(["test", "-p", `semio-framework-os-kernel-neural-engine${crate === "" ? "" : `-${crate}`}`, ...rest], this.repoRoot);
  }
}
class SourceTestScript extends BundleScript {
  async run(): Promise<void> { await import("../../🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts"); }
}
/** 🏷️ Validates direct lower type ownership under the actual Neural product owner. */
class TypeOwnershipTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-type-ownership");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🏷️type/🟦️.ts")], this.repoRoot, "neural:type:ownership", 15000);
 }
}
/** 🧭️ Validates original topology ordering against Graphlib through the shared schema. */
class TopologySourceTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-topology-source");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧭️topology/🧪️tests/🟦️.ts")], this.repoRoot, "neural:topology:source", 15000);
 }
}
/** 📥️ Validates original immutable input assembly against independent JSONPatch. */
class InputSourceTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-input-source");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../📥️input/🧪️tests/🟦️.ts")], this.repoRoot, "neural:input:source", 15000);
 }
}
/** ⏱️ Validates original boundary channel semantics with independent Graphlib and JSONPatch. */
class EvaluationSourceTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-evaluation-source");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../⏱️evaluation/🧪️tests/🟦️.ts")], this.repoRoot, "neural:evaluation:source", 15000);
 }
}
/** ✅️ Validates original finished output contracts against independent strict Ajv. */
class OutputSourceTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-output-source");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../📔️registry/✅️output/🧪️tests/🟦️.ts")], this.repoRoot, "neural:output:source", 15000);
 }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-source", SourceTestScript).register("test-type-ownership", TypeOwnershipTestScript).register("test-topology-source", TopologySourceTestScript).register("test-input-source", InputSourceTestScript).register("test-evaluation-source", EvaluationSourceTestScript).register("test-output-source", OutputSourceTestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
//#endregion 🧪️Validation
