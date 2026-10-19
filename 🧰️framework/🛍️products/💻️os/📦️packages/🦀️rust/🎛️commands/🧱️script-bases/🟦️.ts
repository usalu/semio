/** 🧱️ Script bases of the os kernel package: the schema check shape, the exact cargo stage environments and the check, test and native test script classes the commands extend.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { createIssuedResourceControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { runAdmittedCargoTestsV1 as runCargoTestsV1, runAdmittedCargoPackageTestsV1, runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { scriptInvocationBudget } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { readCargoTestPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolve } from "node:path";
import { type ValidateFunction } from "ajv";
import { runCargo } from "@semio-tech/process/execution";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🧬️ A compiled owned-schema export, typed as a boolean runtime check so `assert` never narrows its validated subject to `unknown`. */
export type SchemaCheck = ((data: unknown) => boolean) & Pick<ValidateFunction, "errors">;



export function exactCargoStageEnvironments() {
  return {
    env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
    nativeEnv: { RUST_MIN_STACK: "268435456" },
  };
}

export class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

export class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
    if(segments[0]==="original-inference-caller"){
      await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,0),lawMilliseconds:600000},cwd:this.repoRoot,cargoArgs:segments.slice(1),groups:[{package:"semio-framework-os-kernel",target:{kind:"lib"},laws:["os_inference::component::tests::original_inferred_source_binding_preserves_actual_caller_before_unfunded_plan","os_inference::component::tests::original_inferred_graph_planning_quotes_each_borrowed_source_and_page"]},{package:"semio-framework-dsl-record",target:{kind:"test",name:"field_refusal_public"},laws:["retained_projection_measure_and_step_follow_original_payload_across_borrowed_headers"]},{package:"semio-framework-plugin",target:{kind:"lib"},laws:["tool_contract::original_inference_admission_tests::original_mounted_inference_admission_retains_source_and_quotes_actual_registry_events"]}]});return;
    }
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-os-kernel"], cwd: this.root, extraArgs: ["--features", "mutation-testing", ...rest] }, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
  }
}

export class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib", "--features", "sync,ureq", ...rest]);
  }
}
