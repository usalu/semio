#!/usr/bin/env bun
/** 🧬️ Shared stdio artifact contract package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runContributionChecks } from "../../🧪️tests/📇️contributions/🟦️.ts";
import { runDefinitionHierarchyChecks } from "../../🧪️tests/🪜️definition-hierarchy/🟦️.ts";
import {runOriginalCopyCloseChecks} from "../../✏️editing/♻️close/🧪️tests/🟦️.ts";
import { runCompositionContributionChecks } from "../../🧩️composition/🧪️tests/🟦️.ts";
import {runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import {configuredExactCargoLawPolicyV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import {BundleScript} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";

/** ♻️ Executes original decoder/copy custody with one actual executable and exact physical laws. */
class OriginalOwnershipScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-original-ownership accepts no arguments");
  if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned generated output");
  console.log(`[DEBUG] Stdio original independent neutral twin assertions=${runOriginalCopyCloseChecks()}`);
  const receipts=await runRepositoryExactCargoLaws({preparation: configuredRepositoryCargoPreparationPolicyV1(),cwd:this.repoRoot,env:process.env,groups:[{package:"semio-s-artifact-stdio-contract-part21",target:{kind:"lib"},laws:[
    "controlled::tests::part21_original_decoder_returns_every_success_and_cancelled_prefix_to_actual_recipient",
    "controlled::tests::part21_rejected_nested_child_preserves_owned_parent_text_and_siblings",
    "controlled::tests::part21_cohort_retirement_releases_original_capacity_without_literal_byte_credit",
   ]},{package:"semio-s-artifact-stdio-contract-editing",target:{kind:"lib"},laws:[
    "tests::retained_copy_original_backing_uses_shared_four_axis_allocator_law",
   ]}],invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:Number(process.env.SEMIO_BUILD_BUDGET_MS??3_600_000),listMilliseconds:60_000,lawMilliseconds:120_000},progress(event){console.log(`[DEBUG] Stdio original ownership ${event.stage}: ${event.law??""} artifacts=${event.artifactDir}`);}});
  for(const receipt of receipts)console.log(`[DEBUG] Stdio original ownership receipt ${JSON.stringify(receipt)}`);
 }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-contract", { packages: ["semio-s-artifact-stdio-contract", "semio-s-artifact-stdio-contract-wire", "semio-s-artifact-stdio-contract-window-kit", "semio-s-artifact-stdio-contract-assembly", "semio-s-artifact-stdio-contract-editing", "semio-s-artifact-stdio-contract-editing-patch", "semio-s-artifact-stdio-contract-editing-details", "semio-s-artifact-stdio-contract-part21", "semio-s-artifact-stdio-contract-list-delta", "semio-s-artifact-stdio-contract-media-export", "semio-s-artifact-stdio-contract-native-print", "semio-s-artifact-stdio-contract-testing"], commands: {"test-original-ownership":OriginalOwnershipScript}, twins: [{name:"original-owner-close",run:runOriginalCopyCloseChecks},{ name: "definition-hierarchy", run: runDefinitionHierarchyChecks }, { name: "contribution-removal", run: runContributionChecks }, { name: "composition-selection", run: runCompositionContributionChecks }] });
