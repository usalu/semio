import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/framework/process/compiler-capacity";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/framework/process/cargo-test";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/framework/process/cargo-laws";

/** 🏃️ Runs the `OriginalOwnershipScript` command under the caller's roots and invocation. */
export async function runOriginalOwnershipScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments:string[]): Promise<void> {
if(segments.length)throw new Error("test-original-ownership accepts no arguments");
if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw new Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned generated output");
const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),cwd:context.repoRoot,env:process.env,groups:[{package:"semio-s-artifact-stdio-contract-part21",target:{kind:"lib"},laws:[
  "controlled::tests::part21_original_decoder_returns_every_success_and_cancelled_prefix_to_actual_recipient",
  "controlled::tests::part21_rejected_nested_child_preserves_owned_parent_text_and_siblings",
  "controlled::tests::part21_cohort_retirement_releases_original_capacity_without_literal_byte_credit",
 ]},{package:"semio-s-artifact-stdio-contract-editing",target:{kind:"lib"},laws:[
  "tests::retained_copy_original_backing_uses_shared_four_axis_allocator_law",
 ]}],invocation:context.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:Number(process.env.SEMIO_BUILD_BUDGET_MS??3_600_000),listMilliseconds:60_000,lawMilliseconds:120_000},progress(event){}});
for(const receipt of receipts){}
 }
