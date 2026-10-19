#!/usr/bin/env bun
import{scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1}from"@semio-tech/process/compiler-capacity";
import {runBudgetedTestCommand} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { runRepositoryCommand } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts";
import { readCargoTestPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";

import { configuredExactCargoLawPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";




/** 🦀️ Awaited plugin SDK checks and exact-filter native regression tests. */
import { runCargo, resolveTypescriptCompilerV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runAdmittedCargoPackageTestsV1, runAdmittedExactCargoLawsV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { nextestArtifactLocation } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter, scriptInvocationBudget } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import assert from "node:assert/strict";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";


import { join, resolve } from "node:path";
import { readFileSync } from "node:fs";
import { semioSchemaAjvV1 } from "../../../../../../🔨️modules/🧬️schema/📦️packages/🟦️typescript/🔮️oracles/✅️validator/🟦️.ts";



//#region 🧪️RunnerSelection
/** 🎯️ Selects explicit build inventory or the existing budgeted test runner without interpreting filters. */
export function pluginTestInvocation(segments: string[]): { mode: "inventory" | "budgeted"; args: string[] } {
  const { rest } = resolveTestLevel(segments);
  const boundary = rest.indexOf("--");
  const options = rest.slice(0, boundary < 0 ? rest.length : boundary);
  const inventory = options.includes("--no-run");
  const targets = ["--lib", "--bins", "--bin", "--examples", "--example", "--tests", "--test", "--benches", "--bench", "--all-targets"];
  const args = options.some(option => targets.includes(option.split("=",1).at(0)??"")) ? rest : ["--lib", ...rest];
  return inventory ? { mode: "inventory", args: ["test", "--manifest-path", "Cargo.toml", ...args] } : { mode: "budgeted", args };
}
//#endregion 🧪️RunnerSelection













//#region 🎯️Tasks
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { artifactAdmissionOracle, completionRejectionOracle } = await import("../../🧪️tests/🧪️artifact-admission-and-completion-oracles/🟦️.ts");
    const { declaredVerbVerdictOracle, declaredBridgeArgumentOracle } = await import("../../🧪️tests/⚖️declared-verb-verdicts/🟦️.ts");
    const { mediaOwnerContextOracle } = await import("../../🧪️tests/🎞️media-owner-context/🟦️.ts");
    const { agentLaneCarriageOracle, agentLanePreviewVerdictOracle } = await import("../../🧪️tests/🤖️agent-lane-preview/🟦️.ts");
    const { timeTravelScenarioOracle } = await import("../../🧪️tests/🧪️time-travel/🟦️.ts");
    const { supersedeLedgerOracle } = await import("../../🧪️tests/🧪️supersede-ledger/🟦️.ts");
    const { historyAlternativesOracle } = await import("../../🧪️tests/🧪️history-alternatives/🟦️.ts");
    const { historyLabelReloadOracle } = await import("../../🧪️tests/🧪️history-label-reload/🟦️.ts");
    const { folderReloadRouteOracle } = await import("../../🧪️tests/🧪️folder-reload-route/🟦️.ts");
    const { composedChildHistoryOracle } = await import("../../🧪️tests/🧪️composed-child-history/🟦️.ts");
    const { checkpointActorOracle } = await import("../../⚛️reactor/📸️checkpoint/🧪️tests/🔬️unit/🟦️.ts");
    await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../🧵️retained-command/🏭️factory/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🏭️producer/🧪️tests/🟦️.ts"),resolve(this.root,"../../👷️job/🏛️reserved/⚙️worker/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/👷️job/🎟️request/♻️retirement/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/👷️job/🎟️request/🔒️command/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/🎞️media/🏭️construction/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🔄️full-operation/🏭️producer/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🧰️framework/🔨️modules/🎯️action-bus/🚪️admission/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});
    await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../🎞️media/⚠️turn/🧪️tests/🟦️.ts"),resolve(this.root,"../../🖥️host/⚡️effects/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧪️tests/🧪️history-edit-acceptance/🧪️tests/📥️action/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});
    console.log(`artifact-admission-oracle cases=${artifactAdmissionOracle(this.repoRoot)} firstParty=39`);
    console.log(`completion-rejection-oracle assertions=${completionRejectionOracle(this.repoRoot)}`);
    console.log(`declared-verb-verdict-oracle cases=${declaredVerbVerdictOracle()}`);
    console.log(`declared-bridge-argument-oracle assertions=${declaredBridgeArgumentOracle()}`);
    console.log(`media-owner-context-oracle assertions=${mediaOwnerContextOracle()}`);
    console.log(`agent-lane-preview-verdict-oracle cases=${agentLanePreviewVerdictOracle()}`);
    console.log(`agent-lane-carriage-oracle cases=${agentLaneCarriageOracle()}`);
    console.log(`time-travel-scenario-oracle cases=${timeTravelScenarioOracle(this.repoRoot)}`);
    console.log(`supersede-ledger-oracle cases=${supersedeLedgerOracle(this.repoRoot)}`);
    console.log(`history-alternatives-oracle cases=${historyAlternativesOracle(this.repoRoot)}`);
    console.log(`history-label-reload-oracle cases=${historyLabelReloadOracle(this.repoRoot)}`);
    console.log(`folder-reload-route-oracle steps=${folderReloadRouteOracle(this.repoRoot)}`);
    console.log(`composed-child-history-oracle cases=${composedChildHistoryOracle(this.repoRoot)}`);
    console.log(`checkpoint-actor-oracle cases=${checkpointActorOracle(this.repoRoot)}`);
    if (segments.length === 1 && segments[0] === "--retained-child-close-exact") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.root, env: { ...process.env, RUST_MIN_STACK: "268435456" }, groups: [
          { package: "semio-framework-plugin-tests-14", target: { kind: "lib" }, laws: ["plugin_builder_contract_tests_4::retained_command_child_emit_prepublication_close_and_rejected_handoff_are_bounded"],
          },
        ], progress(event) {
          console.log(`retained-child-close ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        } });
      console.log(`retained-child-close-receipts: ${JSON.stringify(receipts)}`);
      return;
    }
    const invocation = pluginTestInvocation(segments);
    if (invocation.mode === "inventory") await runCargo(invocation.args, this.root);
    else await runAdmittedCargoPackageTestsV1([], this.root, this.invocation, invocation.args);
  }
}

/** 🎟️ Runs the strict original-job corpus and its actual native registry custody laws. */
class OriginalJobNativeScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("test-original-job-native accepts no arguments");
  const {extensionInvokeCallerOracle}=await import("../../🧩️extension/🧪️tests/📨️invoke/🟦️.ts");extensionInvokeCallerOracle(this.repoRoot);
  const laws=["media_original_domain_pending_publication_and_cancelled_close","media_original_dispatch_and_artifact_projection_preserves_operands","media_original_publication_parts_complete_physical_close","original_job_registry_cold_admission_denial_and_saturation_preserve_sources","original_job_kind_lookup_preserves_denied_frontiers_and_whole_sources","original_job_cancellation_preserves_denied_and_foreign_sources","original_job_body_failure_retains_exact_owner_and_error_until_paid_close","original_job_cancelled_context_publishes_terminal_reply_after_paid_close","original_job_body_failure_cancellation_signals_same_failed_owner","original_job_source_close_uses_exact_one_unit_caller_work","original_job_failure_projection_uses_exact_one_unit_caller_work"];
  await runAdmittedCargoPackageTestsV1([],this.root,this.invocation,["--offline","--lib","-E",laws.map(law=>`test(${law})`).join(" | "),"--","--nocapture"]);
 }
}

/** 🎞️ Verifies neutral media source laws with independent oracles and strict checked input. */
class MediaTurnSourceScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("test-media-turn-source accepts no arguments");
  const sources=[resolve(this.root,"../../🎞️media/⚠️turn/🧪️tests/🟦️.ts"),resolve(this.root,"../../♻️cancellation/🎟️issuer/🧪️tests/🟦️.ts")];
  await runRepositoryTestCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",...sources],{cwd:this.repoRoot,budgetMs:120000});
  await runRepositoryTestCommand(process.execPath,["test",...sources],{cwd:this.repoRoot,budgetMs:15000});
 }
}
/** 🎞️ Executes original media laws against the complete production library and actual heap witness. */
class MediaTurnNativeScript extends MediaTurnSourceScript{
 async run(segments:string[]):Promise<void>{
  await super.run(segments);
  await runAdmittedCargoPackageTestsV1(["semio-framework-plugin-tests-03"],this.root,this.invocation,["--offline","--test","media-turn","--features","media-turn-testing","--","--nocapture"]);
 }
}

/** 🎟️ Executes original body/shared-frame admission and physical return through the production library. */
class ReservedProducerNativeScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("test-reserved-producer-native accepts no arguments");
  await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../📜️declaration/👷️job/🎟️birth/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});
  await runAdmittedCargoPackageTestsV1([],this.root,this.invocation,["--offline","--test","reserved-producer","--features","reserved-producer-testing","--","--nocapture"]);
 }
}

/** 📥️ Checks actual original Opening, response, intent and window admission source contracts. */
class OriginalReactorReceivingSourceScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length)throw new Error("test-original-reactor-receiving-source accepts no arguments");
  const {runOwnedCommand}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
  const laws=["⚛️reactor/🚪️lifetime/🏗️opening/🧪️tests/🟦️.ts","⚛️reactor/🔄️turn/📥️receiving/🧪️tests/🟦️.ts","🪟️window/🫧️transient/🔁️document-replacement/🧪️tests/🟦️.ts"].map(path=>resolve(this.root,"../..",path));
  const remaining=()=>{const value=scriptInvocationBudget(this.invocation,0);if(value===null||value<=0)throw new Error("Original receiving verification requires its finite actual caller");return value};
  await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(this.repoRoot),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun,node",...laws],this.repoRoot,"plugin:original-receiving:strict",remaining(),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",...laws],this.repoRoot,"plugin:original-receiving:source",remaining(),{signal:this.invocation.control.signal});
 }
}

/** 📨️ Checks unchanged caller authority and explicit pending extension payload custody. */
class ExtensionInvokeSourceScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw new Error("test-extension-invoke-source accepts no arguments");
    const {extensionInvokeCallerOracle}=await import("../../🧩️extension/🧪️tests/📨️invoke/🟦️.ts");
    extensionInvokeCallerOracle(this.repoRoot);
  }
}

/** 🧩️ Checks composed child history labels against the independent neutral fixture twin. */
class ComposedChildHistorySourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-composed-child-history-source accepts no arguments");
    const { composedChildHistoryOracle } = await import("../../🧪️tests/🧪️composed-child-history/🟦️.ts");
  }
}

/** ♻️ Executes the original segmented-output owner through its actual allocator receipt law. */
class OutputRetirementNativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-output-retirement-native accepts no arguments");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw new Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,env: process.env,groups: [{ package: "semio-framework-plugin-tests-09", target: { kind: "lib" }, laws: ["segmented_download_contract::segmented_output_original_full_grant_physical_retirement"] }],progress(event) {  }});
    for (const receipt of receipts) {}
  }
}

class CodecSendSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-codec-send-source accepts no arguments");
    const { testPluginCodecCallerSource } = await import("../../🧪️tests/🔣️codec-caller-source/🟦️.ts");
    testPluginCodecCallerSource(this.repoRoot);
  }
}

/** 🏗️ Pins concrete fixture interfaces to their original implementation and neutral imports. */
class FixtureChannelInterfacesScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-fixture-channel-interfaces accepts no arguments");
    await runRepositoryTestCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🔬️app-declarations-fixture/🟦️.ts")], { cwd: this.repoRoot, budgetMs: 15_000 });
  }
}

/** 🧾️ Verifies explicit shared schema authority through the actual plugin assembly boundary. */
class SchemaDocumentAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { schemaDocumentAuthorityOracle } = await import("../../🏗️builder/🧪️tests/🧾️document-authority/🟦️.ts");
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("Unsupported schema document authority argument");
    console.log(`schema-document-authority oracle=${schemaDocumentAuthorityOracle()}`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.root, env: { ...process.env, RUST_MIN_STACK: "268435456" }, groups: [{ package: "semio-framework-plugin-tests-15", target: { kind: "lib" }, laws: ["builder_test_hosts::builder_unit_1::schema_document_authority_tests::schema_document_authority_follows_portable_owner_corpus"] }], progress(event) { console.log(`schema-document-authority ${event.stage}: ${event.law ?? ""}`); } });
    console.log(`schema-document-authority native-laws=${receipts.length}`);
  }
}

class ArtifactAdmissionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { artifactAdmissionOracle, completionRejectionOracle } = await import("../../🧪️tests/🧪️artifact-admission-and-completion-oracles/🟦️.ts");
    assert(
      segments.every((segment) => segment === "--oracle-only"),
      "unsupported artifact admission check argument",
    );
    console.log(`artifact-admission-oracle cases=${artifactAdmissionOracle(this.repoRoot)} firstParty=39`);
    console.log(`completion-rejection-oracle assertions=${completionRejectionOracle(this.repoRoot)}`);
    if (segments.includes("--oracle-only")) return;
    const laws = [
      "strict_artifact_identity_all_builder_channels_reject_before_publication",
      "strict_artifact_identity_mixed_channels_publish_nothing",
      "strict_artifact_identity_matches_independent_neutral_fixture",
      "strict_artifact_identity_owned_tree_and_definition_channels_publish",
    ];
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.root, env: { ...process.env, RUST_MIN_STACK: "268435456" }, groups: [{ package: "semio-framework-plugin", target: { kind: "lib" }, laws }], progress(event) {
        console.log(`artifact-admission ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      } });
    console.log(`artifact-admission-laws: ${JSON.stringify(receipts)}`);
  }
}


class GuestLifecycleCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { coldDocumentPairIngressOracle, documentBackboneBindingOracle, guestLifecycleOracle, issuedPatchOracle } = await import("../../🧪️tests/🧪️reactor-contract-oracles/🟦️.ts");
    assert(
      segments.every((segment) => segment === "--native"),
      "guest-lifecycle-check accepts only --native",
    );
    console.log(`guest-lifecycle-oracle cases=${guestLifecycleOracle()} patch-cases=${issuedPatchOracle()}`);
    if (!segments.includes("--native")) return;
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 86_400_000, lawMilliseconds: 60_000 }, cwd: this.root, env: { ...process.env, RUST_MIN_STACK: "33554432", CARGO_BUILD_JOBS: "1" }, groups: [
        { package: "semio-framework-plugin-tests-01", target: { kind: "lib" }, laws: ["runtime_instances_test_hosts::runtime_cleanup_fault_vector_tests::non_fault_status_reprs_never_collide_with_a_fault_variant"],
        }, { package: "semio-framework-plugin-tests-10", target: { kind: "lib" }, laws: ["plugin_builder_contract_tests_2::instance_lifetime_close_does_not_publish_terminal_before_watchdog", "plugin_builder_contract_tests_2::instance_lifetime_close_deadline_resume_never_reenters_completed_work", "plugin_builder_contract_tests_2::instance_lifetime_close_late_physical_step_retains_its_exact_outcome", "plugin_builder_contract_tests_2::instance_lifetime_close_deadline_submit_refusal_preserves_candidate", "plugin_builder_contract_tests_2::instance_lifetime_close_optional_monotonic_clock_rejects_missing_and_backward_authority", "plugin_builder_contract_tests_2::instance_lifetime_close_fault_outcome_dominates_complete_progress"],
        }, { package: "semio-framework-plugin-reactor-lifecycle", target: { kind: "lib" }, laws: ["issued_receipt_tests::reactor_issued_patch_ack_and_rejection_match_neutral_exact_tuple", "issued_receipt_tests::reactor_issued_parallel_patch_slots_and_duplicate_ack_remain_independent", "issued_receipt_tests::reactor_uncommitted_patch_handback_preserves_exact_slot_and_retry", "issued_receipt_tests::reactor_acknowledged_patch_slots_retire_without_instance_close", "instance_lifetime_patch_close_tests::guest_instance_lifecycle_pending_patch_handback_preserves_rejected_owner_and_exact_bytes", "instance_lifetime::tests::guest_instance_lifecycle_ack_fault_keeps_exact_receipt_and_owner", "instance_lifetime::tests::guest_instance_lifecycle_terminal_release_work_is_measured_and_never_repeated_after_late_clock", "instance_lifetime::tests::guest_instance_lifecycle_same_activation_reopen_rejects_old_authority"],
        }, { package: "semio-framework-plugin-reactor-patches", target: { kind: "lib" }, laws: ["tests::issued_obsolete_reconcile_feedback_retires_only_the_old_pending_owner"],
        }, { package: "semio-framework-plugin-tests-11", target: { kind: "lib" }, laws: ["plugin_builder_contract_tests_6::reactor_native_lifecycle_retains_exact_close_until_ack", "plugin_builder_contract_tests_6::reactor_native_lifecycle_rejects_foreign_and_colliding_owners", "plugin_builder_contract_tests_6::reactor_native_lifecycle_output_failure_preserves_ack_and_owner", "plugin_builder_contract_tests_6::reactor_output_fault_returns_real_patch_and_preserves_other_lifecycle_ack"],
        },
      ], progress(event) {
        console.log(`guest-lifecycle ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      } });
    console.log(`guest-lifecycle-receipts: ${JSON.stringify(receipts)}`);
  }
}


class ColdDocumentPairIngressCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { coldDocumentPairIngressOracle, documentBackboneBindingOracle, guestLifecycleOracle, issuedPatchOracle } = await import("../../🧪️tests/🧪️reactor-contract-oracles/🟦️.ts");
    assert(segments.every((segment) => segment === "--native"), "cold-document-pair-ingress-check accepts only --native");
    const hostile = await coldDocumentPairIngressOracle(this.repoRoot);
    console.log(`cold-document-pair-ingress-oracle: ajv=1 sha256=3 webcrypto=3 hostile=${hostile} limits=64KiB/64/4MiB`);
    if (!segments.includes("--native")) return;
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 86_400_000, lawMilliseconds: 60_000 }, cwd: this.root, env: { ...process.env, RUST_MIN_STACK: "33554432", CARGO_BUILD_JOBS: "1" }, nativeEnv: { RUST_MIN_STACK: "268435456", CARGO_BUILD_RUSTFLAGS: "-Z threads=1" }, groups: [
        {
          package: "semio-framework-actor",
          target: { kind: "lib" },
          laws: [
            "cold_pair_tests::cold_pair_ingress_status_pack_round_trips_every_exact_variant",
            "cold_pair_tests::cold_pair_ingress_neutral_fixture_has_exact_semantic_receipts_and_hostiles",
            "cold_pair_tests::cold_pair_ingress_decode_refuses_noncanonical_authority_before_publication",
          ],
        },
        { package: "semio-framework-plugin-reactor-surfaces", target: { kind: "lib" }, laws: ["tests::cold_pair_ingress_streams_the_exact_four_mibibyte_pair_and_loads_once", "tests::cold_pair_ingress_rechecks_live_and_rejects_hostile_pages_without_displacement", "tests::cold_pair_ingress_keeps_the_structural_owner_across_load_cancel_and_bounded_close", "tests::cold_pair_ingress_final_live_fence_rejects_post_await_revocation", "tests::cold_pair_ingress_charges_aggregate_reserved_capacity_until_final_close", "tests::cold_pair_ingress_is_an_exact_retained_native_close_participant", "tests::cold_pair_header_requires_an_active_checkpoint_frontier_and_exact_hashes"],
        },
      ], progress(event) { console.log(`cold-document-pair-ingress ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); } });
    console.log(`cold-document-pair-ingress-receipts: ${JSON.stringify(receipts)}`);
  }
}


class DocumentBackboneBindingCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { coldDocumentPairIngressOracle, documentBackboneBindingOracle, guestLifecycleOracle, issuedPatchOracle } = await import("../../🧪️tests/🧪️reactor-contract-oracles/🟦️.ts");
    assert(segments.every((segment) => segment === "--native"), "document-backbone-binding-check accepts only --native");
    const rows = documentBackboneBindingOracle(this.repoRoot);
    console.log(`document-backbone-binding-oracle: ajv=1 rows=${rows} hot=256KiB genesis=4MiB pending=64/1MiB`);
    if (!segments.includes("--native")) return;
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 86_400_000, lawMilliseconds: 60_000 }, cwd: this.root, env: { ...process.env, RUST_MIN_STACK: "33554432", CARGO_BUILD_JOBS: "1" }, nativeEnv: { RUST_MIN_STACK: "268435456", CARGO_BUILD_RUSTFLAGS: "-Z threads=1" }, groups: [
        {
          package: "semio-framework-replication",
          target: { kind: "lib", name: "protocol" },
          laws: ["causal::tests::document_backbone_batch_fixture_is_exact_bounded_and_u64_safe"],
        },
        {
          package: "semio-framework-os-store-sync-tests-1",
          target: { kind: "lib" },
          laws: [
            "sync_tests::document_backbone_mailbox_and_retention_are_exact_bounded_and_terminal",
            "sync_tests::actor_tests::raw_document_backbone_reaches_hub_once_and_returns_one_canonical_event",
          ],
        },
        {
          package: "semio-framework-os-store-sync-native",
          target: { kind: "lib" },
          laws: [
            "native_actor::retained_turn_fixtures::retained_readiness_wake_after_turn_release_is_observed_once",
            "native_actor::retained_turn_fixtures::cancellation_cannot_complete_before_an_inflight_turn_returns_its_exact_owner",
          ],
        },
        { package: "semio-framework-plugin-actor-host", target: { kind: "lib" }, laws: ["document_backbone_binding::tests::document_backbone_op_binary_is_exact_canonical_and_bounded", "document_backbone_binding::tests::document_backbone_binding_reducer_preserves_generation_and_live_owner"],
        },
      ], progress(event) { console.log(`document-backbone-binding ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); } });
    console.log(`document-backbone-binding-receipts: ${JSON.stringify(receipts)}`);
  }
}

/** 🏛️ Verifies worker-owned command publication and immutable presence authority. */
class CanonicalArchitectureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { sourceFreshnessOracle } = await import("../../🧪️tests/🔬️source-freshness/🟦️.ts");
    const { extensionRetirementOracle } = await import("../../🧪️tests/🔬️extension-retirement/🟦️.ts");
    const { declaredVerbVerdictOracle, declaredBridgeArgumentOracle } = await import("../../🧪️tests/⚖️declared-verb-verdicts/🟦️.ts");
    const { mediaOwnerContextOracle } = await import("../../🧪️tests/🎞️media-owner-context/🟦️.ts");
    if (segments.some((segment) => segment !== "--oracle-only")) throw new Error("canonical-architecture accepts only --oracle-only");
    console.log(`declared-bridge-argument-oracle assertions=${declaredBridgeArgumentOracle()}`);
    console.log(`media-owner-context-oracle assertions=${mediaOwnerContextOracle()}`);
    console.log(`extension-retirement-oracle cases=${extensionRetirementOracle(this.repoRoot)}`);
    console.log(`source-freshness-oracle cases=${await sourceFreshnessOracle(this.repoRoot,nextestArtifactLocation(this.repoRoot).directory)}`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },nativeEnv: { RUST_MIN_STACK: "268435456" },groups: [{ package: "semio-framework-plugin-extension", target: { kind: "lib" }, laws: ["artifact_inference::artifact_inference_service_tests::artifact_inference_context_preserves_execution_and_registry_identity"],
      }, { package: "semio-framework-plugin-surface-fixture", target: { kind: "lib" }, laws: ["surface::media_owner_context::media_export_request_context_preserves_exact_supplied_owner"],
      }, { package: "semio-framework-plugin-declared-verdict", target: { kind: "lib" }, laws: ["declared_verb_verdict_tests::declared_bridge_required_arguments_match_neutral_schemas"],
      }, { package: "semio-framework-plugin-testing-acceptance", target: { kind: "lib" }, laws: ["ui_action_laws::tests::language_neutral_action_collections_agree_with_json_pointer_oracle"],
      }, { package: "semio-framework-plugin-tests-14", target: { kind: "lib" }, laws: ["plugin_builder_contract_tests_4::a_command_reaches_both_ephemeral_lanes_without_touching_history"],
      }, { package: "semio-framework-plugin-tests-11", target: { kind: "lib" }, laws: ["plugin_builder_contract_tests_6::a_command_that_emits_nothing_ephemeral_leaves_both_lanes_untouched"],
      }, { package: "semio-framework-plugin-tests-09", target: { kind: "lib" }, laws: ["plugin_builder_contract_tests_5::peer_presence_capture_is_one_arc_and_retirement_waits_for_then_drains_the_exact_root", "plugin_builder_contract_tests_5::peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority"],
      }, { package: "semio-framework-plugin-tests-06", target: { kind: "lib" }, laws: ["extension_test_hosts::extension_retirement_tests::extension_bundle_resource_retirement_preserves_zero_cancel_progress_and_terminal_owner", "extension_test_hosts::extension_retirement_tests::extension_bundle_resource_retirement_replacement_retains_old_and_backpressured_candidates", "extension_test_hosts::extension_retirement_tests::extension_bundle_resource_retirement_rejects_false_terminal_and_exceeded_grants", "extension_test_hosts::extension_retirement_tests::extension_bundle_resource_retirement_runs_through_suspend_and_exported_poll", "extension_test_hosts::extension_retirement_tests::extension_bundle_resource_retirement_refuses_implicit_live_drop_and_disposes_cold_explicitly", "extension_test_hosts::extension_retirement_tests::extension_bundle_resource_retirement_admits_exact_backing_and_boxed_shell_allocations"],
      }],progress(event) { console.log(`canonical-architecture-command ${event.stage}: ${event.law ?? ""}`); }});
    console.log(`canonical-architecture-command receipts=${receipts.length}`);
  }
}

/** 🧩️ Proves this owner's complete canonical command ingress consumer contract. */
class CommandIngressConsumerScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("test-command-ingress-consumer accepts no arguments");
  const {runBudgetedTestCommand}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts"),{testLevelBudgetMs}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts"),source=resolve(this.root,"../../🏛️ownership/📥️command-ingress/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}

/** 🪶️ Verifies the plugin-owned native SQLite snapshot laws. */
class SnapshotSqliteAdmissionScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
 if(segments.length)throw new Error("test-snapshot-sqlite-admission accepts no arguments");
 await runAdmittedCargoPackageTestsV1(["semio-framework-plugin-abi", "semio-framework-plugin-host-guest-laws", "semio-framework-plugin-host-test-component", "semio-framework-plugin-tests-18"], this.repoRoot, this.invocation, ["--lib","sqlite_snapshot_"]);
 }
}

/** ⚠️ Preserves intrinsic SQLite provider refusals across the guest codec boundary. */
class SnapshotSqliteRefusalScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  const [mode,...rest]=segments;
  if(rest.length || mode===undefined || !["source","native","host-native"].includes(mode))throw new Error("test-snapshot-sqlite-refusal requires source, native or host-native");
  if(mode==="host-native"){
   const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1() },cwd:this.repoRoot,env:process.env,groups:[{ package: "semio-framework-plugin-host-guest-laws", target: { kind: "lib" }, laws: ["sqlite_refusal_tests::sqlite_snapshot_guest_refusal_host_preserves_causes_diagnostics_and_actual_vm_cancellation"] }, { package: "semio-framework-plugin-host-guest-runtime", target: { kind: "lib" }, laws: ["snapshot_call::metadata::tests::original_snapshot_scalar_metadata_borrows_actual_json_without_heap", "snapshot_call::metadata::tests::original_snapshot_scalar_metadata_cancellation_keeps_source_and_ceiling", "snapshot_call::tests::original_snapshot_pending_fuel_preserves_original_completed_ceiling"] },{package:"semio-framework-plugin-interpreter",target:{kind:"lib"},laws:["operation::input::tests::original_snapshot_input_cancellation_retains_actual_json_backing"]}],progress(event){}});
   return;
  }
  if(mode==="native"){
   await runAdmittedCargoPackageTestsV1(["semio-framework-plugin-host-guest-laws", "semio-framework-plugin-tests-18"], this.repoRoot, this.invocation, ["--lib","sqlite_snapshot_guest_refusal_"]);
   return;
  }
  const {runOwnedCommand}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
  const remaining=()=>{const milliseconds=scriptInvocationBudget(this.invocation,0);if(milliseconds!==null&&milliseconds<1)throw new Error("Original snapshot source deadline exhausted");return milliseconds===null?0:Math.floor(milliseconds);};
  const source=resolve(this.root,"../../🧬️schema/🪶️sqlite/⚠️refusal/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"snapshot:refusal:types",remaining(),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"snapshot:refusal:source",remaining(),{signal:this.invocation.control.signal});
 }
}

/** ♻️ Verifies actual retained operation and localized history allocation authorities. */
class RetainedMetadataScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw new Error("test-retained-metadata accepts no arguments");
    if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw new Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
    const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS??3_600_000), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd:this.root,env:process.env,groups:[{ package: "semio-framework-plugin-retained-command", target:{kind:"lib"}, laws: ["metadata_retirement::tests::retained_command_metadata_preserves_original_history_and_physical_authority"]}],progress(event){}});
  }
}

/** ♻️ Checks original raw backing receipts independently of logical copy work. */
class RetainedRawScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||segments[0]===undefined||!["source","native"].includes(segments[0]))throw new Error("test-retained-raw requires source or native");
  const {runBudgetedTestCommand}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🧵️retained-command/🧪️tests/♻️raw-allocation-close/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🎟️admission/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🪟️mounted/♻️frontier/🧪️tests/🟦️.ts"),resolve(this.root,"../../♻️lifecycle/🏪️stores/🧪️tests/🟦️.ts"),resolve(this.root,"../../♻️cancellation/📦️lease/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:120_000,throwOnFailure:true});
  const lease=resolve(this.root,"../../♻️cancellation/📦️lease/🧪️tests/🟦️.ts");await runBudgetedTestCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",lease,resolve(this.root,"../../🧵️retained-command/🪟️mounted/♻️frontier/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:120000,throwOnFailure:true});
  if(segments[0]==="source")return;
  if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw new Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 3_600_000, listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd:this.root,env:process.env,groups:[{ package: "semio-framework-plugin-tests-12", target:{kind:"lib"}, laws: ["plugin_builder_contract_tests_7::retained_latest_wins_raw_capacity_is_not_initialized_byte_retirement"]}, { package: "semio-framework-plugin-retained-command", target:{kind:"lib"}, laws: ["admission::tests::retained_raw_admission_has_no_failure_destructor_or_undeclared_allocation"]}],progress(event){}});
 }
}

/** 🚨️ Checks that a mounted retained operation names the structural refusal it retains, from one corpus read by the Rust law and its TypeScript twin. */
class RetainedStructuralRefusalScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||segments[0]===undefined||!["source","native"].includes(segments[0]))throw new Error("test-retained-structural-refusal requires source or native");
  const {runBudgetedTestCommand}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🧵️retained-command/🪟️mounted/🚨️structural-refusal/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:120_000,throwOnFailure:true});
  if(segments[0]==="source")return;
  if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw new Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 3_600_000, listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd:this.root,env:process.env,groups:[{ package: "semio-framework-plugin-tests-09", target:{kind:"lib"}, laws: ["job_tool_run_host_2::tool_run_tests_2::a_mounted_operation_reports_the_structural_refusal_it_retains"]}],progress(event){console.log(`[TRACE] retained-structural-refusal ${event.stage}`);}});
  console.log(`[TRACE] retained-structural-refusal receipts=${receipts.length}`);
 }
}

/** 📬️ Verifies exact window mutation wrappers and their independently issued payload custody. */
class WindowMutationScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||segments[0]===undefined||!["source","native"].includes(segments[0]))throw new Error("test-window-mutation requires source or native");
  const {runBudgetedTestCommand}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts"),file=resolve(this.root,"../../🪟️window/📬️mutation/🧪️tests/🟦️.ts"),prepared=resolve(this.root,"../../../🏪️store/📦️prepared/♻️retirement/🧪️tests/🟦️.ts"),read=resolve(this.root,"../../../🏪️store/🔗️read/♻️retirement/🧪️tests/🔬️unit/🟦️.ts"),refresh=resolve(this.root,"../../🪟️window/📬️mutation/🔄️refresh/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,["test",file,prepared,read,refresh],{cwd:this.repoRoot,budgetMs:120000,throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file,prepared,read,refresh],{cwd:this.repoRoot,budgetMs:120000,throwOnFailure:true});
  if(segments[0]==="source")return;
  if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw new Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 3_600_000, listMilliseconds: 60_000, lawMilliseconds: 120000 },cwd:this.root,env:process.env,groups:[{ package: "semio-framework-plugin-tests-19", target:{kind:"lib"}, laws: ["window_mutation_tests::tests::window_original_mutation_wrappers_have_exact_physical_custody"]}],progress(event){}});
 }
}
/** 🗂️ Qualifies original paged window registry order and independently funded retirement. */
class WindowRegistryScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||segments[0]===undefined||!["source","native"].includes(segments[0]))throw new Error("test-window-registry requires source or native");
  if(segments[0]==="native"){
   await runAdmittedCargoPackageTestsV1(["semio-framework-plugin-tests-19"], this.repoRoot, this.invocation, ["--lib","window_config_paged_registry","--","--nocapture"]);
   return;
  }
  const file=resolve(this.root,"../../🪟️window/🎚️config/🗂️registry/🧪️tests/🟦️.ts");
  await runRepositoryTestCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file],{cwd:this.repoRoot,budgetMs:120000});
  await runRepositoryTestCommand(process.execPath,["test",file],{cwd:this.repoRoot,budgetMs:120000});
 }
}

/** 👷️ Drives the original reserved job's complete independent currency law. */
class ReservedRetirementScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||segments[0]===undefined||!["source","native"].includes(segments[0]))throw new Error("test-reserved-retirement requires source or native");
  const {runBudgetedTestCommand}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts");
  const original=resolve(this.root,"../../👷️job/🧾️bytes/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",original],{cwd:this.repoRoot,budgetMs:120000,throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,["test",original],{cwd:this.repoRoot,budgetMs:120000,throwOnFailure:true});
  if(segments[0]==="source")return;
  if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw new Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 3600000, listMilliseconds: 60000, lawMilliseconds: 120000 },cwd:this.root,env:process.env,groups:[{ package: "semio-framework-plugin-tests-12", target:{kind:"lib"}, laws: ["plugin_builder_contract_tests_7::original_reserved_job_currency_law_retains_spare_backing_until_exact_release"]}]});
 }
}

/** 🏇️ Executes the original runtime scheduling, acknowledgement and terminal witnesses in place. */
class MountedReceivingScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-mounted-receiving accepts no arguments");
    const artifactDir = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactDir) throw new Error("mounted receiving requires explicit owned artifact storage");
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { version:1, buildMilliseconds:300000, listMilliseconds:60000, lawMilliseconds:60000, transport:{maximumBuildOutputBytes:268435456,maximumNativeOutputBytes:8388608} },cwd: this.root,env: process.env,nativeEnv: { RUST_MIN_STACK: "268435456" },groups: [{ package: "semio-framework-plugin", target: { kind: "lib" }, cargoArgs: ["--offline"], laws: [
          "component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_does_not_publish_terminal_before_watchdog",
          "component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_deadline_resume_never_reenters_completed_work",
          "component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_late_physical_step_retains_its_exact_outcome",
          "component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_deadline_submit_refusal_preserves_candidate",
          "component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_optional_monotonic_clock_rejects_missing_and_backward_authority",
          "component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_fault_outcome_dominates_complete_progress",
          "component::reactor::pending::issued_receipt_tests::reactor_issued_patch_ack_and_rejection_match_neutral_exact_tuple",
          "component::reactor::pending::issued_receipt_tests::reactor_issued_parallel_patch_slots_and_duplicate_ack_remain_independent",
          "component::reactor::pending::issued_receipt_tests::reactor_uncommitted_patch_handback_preserves_exact_slot_and_retry",
          "component::reactor::pending::issued_receipt_tests::reactor_acknowledged_patch_slots_retire_without_instance_close",
          "component::plugin_runtime::plugin_builder_contract_tests::a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta",
          "component::plugin_runtime::plugin_builder_contract_tests::task_admission_defers_the_user_factory_until_actor_execution",
          "component::plugin_runtime::plugin_builder_contract_tests::spawn_task_quota_gate_faults_the_n_plus_1th_task_and_never_silently_drops_it",
          "component::plugin_runtime::plugin_builder_contract_tests::key_dedupe_cancels_the_previously_live_task_under_the_same_key",
          "component::plugin_runtime::plugin_builder_contract_tests::instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot",
          "component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume"
        ] }]});
      for (const receipt of receipts) {}
  }
}

/** 🧬️ Checks the original Window domain preparation fixture against independent SQLite semantics. */
class WindowPreparationSourceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length!==0)throw new Error("test-window-preparation-source takes no arguments");
  const file=resolve(this.root,"../../🪟️window/🎚️config/🧬️preparation/🧪️tests/🟦️.ts");
  await runRepositoryTestCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file],{cwd:this.repoRoot,budgetMs:120000});
  await runRepositoryTestCommand(process.execPath,["test",file],{cwd:this.repoRoot,budgetMs:120000});
 }
}

/** ⚖️ Original child-error receipt, source custody and actual cause pointers. */
class OriginalFailureDebitSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("original failure debit takes no arguments");const sources=[resolve(this.root,"../../⚛️reactor/🎟️context/🌉️call/⚖️failure-debit/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧪️tests/🔬️app-artifact-inference-wire/🟦️.ts"),resolve(this.root,"../../🧪️testing/🎟️caller/🧪️tests/🟦️.ts")];const execute=(args:string[])=>runRepositoryTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});await execute([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",...sources]);await execute(["test",...sources]);}
}
/** 🦀️ Owning exact original cause/debit and registered job entry laws. */
class OriginalFailureDebitNative extends OriginalFailureDebitSource{
 async run(args:string[]):Promise<void>{await super.run(args);await runAdmittedCargoPackageTestsV1([],this.root,this.invocation,["--offline","--lib","-E","test(original_guest_failure_debit_) | test(original_guest_job_loan_retains_whole_value_error_and_actual_context) | test(original_extension_) | test(original_inference_gateway_failure_all_owned_causes) | test(original_plugin_fixture_) | test(original_fixture_command_projection_matches_the_closed_unit_and_data_cases) | test(original_receiving_cancelled_before_first_loan_retires_same_inputs_and_source) | test(erased_dispatch_clone_release_preserves_unique_bounded_job_disposal_authority) | test(original_query_output_width_does_not_price_or_renew_its_caller_receiving_grant) | test(test_config_original_record_receiving_preserves_nullable_selection)","--","--nocapture"]);}
}
/** 📬️ Checks every original Store command receiving ordinal and partial envelope custody. */
class OriginalCommandReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("original command receiving takes no arguments");const sources=[resolve(this.root,"../../../🏪️store/📬️command/🛬️receiving/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/🧪️tests/🟦️.ts")];const execute=(args:string[])=>runRepositoryTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});await execute([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",...sources]);await execute(["test",...sources]);}
}
/** 🦀️ Runs the owning lower Store command and envelope receiving laws. */
class OriginalCommandReceivingNative extends OriginalCommandReceivingSource{
 async run(args:string[]):Promise<void>{await super.run(args);await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"],this.repoRoot,this.invocation,["--offline","--lib","-E","test(original_command_receiving_)","--","--nocapture"]);}
}
/** 🪢️ Checks the original lifecycle context and whole paired cause against independent neutral oracles. */
class OriginalLifecycleSource extends BundleScript{async run():Promise<void>{await runRepositoryTestCommand(process.execPath,["test",resolve(import.meta.dir,"../../🏛️ownership/⚠️failure/🧪️tests/🟦️.ts"),resolve(import.meta.dir,"../../🧪️tests/🧾️document-archive-load-legs/🟦️.ts")],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});}}
/** 🦀️ Runs actual lifecycle failure and context source preservation laws. */
class OriginalLifecycleNative extends OriginalLifecycleSource{async run():Promise<void>{await super.run();await runAdmittedCargoPackageTestsV1(["semio-framework-plugin-ownership", "semio-framework-plugin-tests-11"],this.repoRoot,this.invocation,["--offline","--lib","-E","test(original_lifecycle_)","--","--nocapture"]);}}

/** 📤️ Original bounded output and exact paid acknowledgement against independent neutral semantics. */
class OriginalPagedOutputSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("test-original-paged-output accepts no arguments");const source=resolve(this.root,"../../⚛️reactor/🎟️context/🌉️call/📤️output/🧪️tests/🟦️.ts");const execute=(args:string[])=>runRepositoryTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});await execute([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",source]);await execute(["test",source]);}
}
/** 🦀️ Same original output pointer, physical span receipts, denial and fully paid close. */
class OriginalPagedOutputNative extends OriginalPagedOutputSource{
 async run(args:string[]):Promise<void>{await super.run(args);await runAdmittedCargoPackageTestsV1([],this.root,this.invocation,["--offline","--lib","-E","test(original_job_output_paged_source_identity_cpu_denials_system_and_paid_ack)","--","--nocapture"]);}
}
/** 🧬️ Original typed inference container and exact scalar grammar positions use independent source oracles. */
class OriginalInferenceCloseSource extends BundleScript {
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("original inference close takes no arguments");const sources=[resolve(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/📬️outcome/🧪️tests/🟦️.ts"),resolve(this.root,"../../⚛️reactor/🎟️context/🌉️call/📤️failure/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/👷️job/🧪️inference-close/🟦️.ts"),resolve(this.repoRoot,"🧰️framework/🔨️modules/⚠️diagnostic/📤️wire/👣️cursor/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🧪️tests/🟦️.ts")];sources.push(resolve(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🧪️tests/🎟️caller/🟦️.ts"));sources.push(resolve(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🎟️original/🧪️tests/🟦️.ts"));sources.push(resolve(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/📤️output/🧪️tests/🟦️.ts"));sources.push(resolve(this.repoRoot,"🧰️framework/🔨️modules/⚠️diagnostic/📤️wire/👣️cursor/🧩️domain/🧪️tests/🟦️.ts"));sources.push(resolve(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🧪️tests/🔬️unit/🟦️.ts"));const execute=(args:string[])=>runRepositoryTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});await execute([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",...sources]);await execute(["test",...sources]);}
}
/** 🦀️ Same inference source and canonical scalar wire, real System receipts and terminal physical close. */
class OriginalInferenceCloseNative extends OriginalInferenceCloseSource {
 async run(args:string[]):Promise<void>{await super.run(args);await runAdmittedCargoPackageTestsV1(["semio-framework-os-inference"],this.repoRoot,this.invocation,["--offline","--lib","-E","test(original_inferred_source_scope_refuses_foreign_root_with_same_business_identity) | test(original_inferred_scope_refuses_foreign_clock_binding_on_same_parent_and_function) | test(original_inferred_values_same_source_donor_denials_partial_cancel_and_full_paid_close) | test(original_stepped_driver_matches_neutral_dag_for_each_finite_turn) | test(original_stepped_cache_option_preserves_values_without_unfunded_cache_publication) | test(original_stepped_cancel_retains_finished_rows_and_closes_pending_before_source) | test(a_failing_compute_is_an_error_naming_the_entity_and_keeps_the_values_before_it)","--","--nocapture"]);await runAdmittedCargoPackageTestsV1([],this.root,this.invocation,["--offline","--lib","-E","test(original_inference_session_same_source_full_paid_close_and_terminal_drop_zero) | test(original_inference_session_second_arc_alias_same_payload_paid_extract_full_close) | test(original_entry_full_failure_borrowed_publication_same_source_denials_and_paid_close)","--","--nocapture"]);}
}
/** 🎟️ Original queued worker source and exact paid alias/admission use independent neutral oracles. */
class OriginalPendingWorkerSource extends BundleScript {
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("original pending worker takes no arguments");const source=resolve(this.root,"../../👷️job/🚪️session/🎟️pending/🧪️tests/🟦️.ts");const donor=resolve(this.root,"../../🧵️retained-command/🔗️read/🧪️tests/🟦️.ts");const execute=(args:string[])=>runRepositoryTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});await execute([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",source,donor]);await execute(["test",source,donor]);}
}
/** 🦀️ Actual original pending source, System receipts, all cancel prefixes and full terminal close. */
class OriginalPendingWorkerNative extends OriginalPendingWorkerSource {
 async run(args:string[]):Promise<void>{await super.run(args);await runAdmittedCargoPackageTestsV1([],this.root,this.invocation,["--offline","--lib","-E","test(original_reserved_pending_worker_same_source_denials_admission_cancel_and_paid_close)","--","--nocapture"]);}
}
/** 🎟️ Tests original media decoded custody with an independent neutral wire oracle. */
class OriginalMediaCustodySource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("original media custody accepts no arguments");const source=resolve(this.root,"../../⚛️reactor/🏃️runtime/🎞️command/📥️custody/🧪️tests/🟦️.ts");const execute=(args:string[])=>runRepositoryTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});await execute([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",source]);await execute(["test",source]);}
}
/** 🦀️ Runs the actual paid original decoded owner and complete original cause under one finite caller. */
class OriginalMediaCustodyNative extends OriginalMediaCustodySource{
 async run(args:string[]):Promise<void>{await super.run(args);await runAdmittedCargoPackageTestsV1([],this.root,this.invocation,["--offline","--lib","-E","test(original_failed_media_decoded_source_same_cause_denials_and_full_paid_close) | test(mounted_original_producer_shell_denials_unchanged_child_census_and_paid_terminal_release)","--","--nocapture"]);}
}
/** 🧯️ Original fixture completion custody and unchanged caller fuel use independent neutral oracles. */
class OriginalFixtureCloseSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("original fixture close takes no arguments");const sources=[resolve(this.root,"../../🧪️tests/🧬️mutation-fixtures-job-close/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🧪️tests/🔬️unit/⛽️fuel/🟦️.ts"),resolve(this.root,"../../📬️completion/⚠️fault/🧪️tests/🟦️.ts")];const execute=(args:string[])=>runRepositoryTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});await execute([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",...sources]);await execute(["test",...sources]);}
}


/** 📷️ Checks canonical window presence and the existing native declared-interaction law. */
class CameraPresenceContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1 || !["source", "native"].includes(segments[0]!)) throw new Error("test-camera-presence requires source or native");
    const file = resolve(this.root, "../../👥️presence/📷️camera/🧪️tests/🟦️.ts");
    await runRepositoryCommand(this.invocation,process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file],this.repoRoot,"presence-source-types",{version:1,maximumElapsedMilliseconds:120000,output:{maximumBytes:8388608,maximumLines:65536}});
    const { testCameraPresenceContract } = await import("../../👥️presence/📷️camera/🧪️tests/🟦️.ts");
    if (segments[0] === "source") return;
    const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(native.manifestPath),manifest);const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:native.buildBudgetMs,lawMilliseconds:native.assertionBudgets[native.level]},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:native.rustMinStack},manifestPaths:{"semio-framework-plugin":manifest},groups:[{package:"semio-framework-plugin",target:{kind:"lib",name:"semio_framework_plugin"},cargoArgs:["--locked"],laws:["component::plugin_runtime::plugin_builder_contract_tests::ephemeral_snapshot_carries_encoded_interaction_from_declared_broadcast_specs"]}]});
  }
}
/** 🏭️ Verifies real producer/source custody corpora against independent schema and physical-effect oracles. */
class OriginalProducerSourceScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw new Error("test-original-producer-source accepts no arguments");await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../🧵️retained-command/🏭️producer/🧪️tests/🟦️.ts"),resolve(this.root,"../../👷️job/🏛️reserved/⚙️worker/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/👷️job/🎟️request/♻️retirement/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/👷️job/🎟️request/🔒️command/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/🎞️media/🏭️construction/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🔄️full-operation/🏭️producer/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🏭️factory/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🧰️framework/🔨️modules/🎯️action-bus/🚪️admission/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});}}

const router = new ScriptRouter(import.meta.dir).register("test-original-fixture-close-source",OriginalFixtureCloseSource).register("test-original-media-custody-source",OriginalMediaCustodySource).register("test-original-media-custody-native",OriginalMediaCustodyNative).register("test-original-command-receiving-source",OriginalCommandReceivingSource).register("test-original-command-receiving-native",OriginalCommandReceivingNative).register("test-original-pending-worker-source",OriginalPendingWorkerSource).register("test-original-pending-worker-native",OriginalPendingWorkerNative).register("test-original-inference-close-source",OriginalInferenceCloseSource).register("test-original-inference-close-native",OriginalInferenceCloseNative).register("test-original-paged-output-source",OriginalPagedOutputSource).register("test-original-paged-output-native",OriginalPagedOutputNative).register("test-original-lifecycle-source",OriginalLifecycleSource).register("test-original-lifecycle-native",OriginalLifecycleNative).register("test-original-failure-debit-source",OriginalFailureDebitSource).register("test-original-failure-debit-native",OriginalFailureDebitNative).register("test-window-mutation",WindowMutationScript).register("test-retained-raw",RetainedRawScript).register("test-retained-structural-refusal",RetainedStructuralRefusalScript).register("test-retained-metadata",RetainedMetadataScript).register("test-snapshot-sqlite-refusal",SnapshotSqliteRefusalScript).register("test-snapshot-sqlite-admission",SnapshotSqliteAdmissionScript).register("test-command-ingress-consumer",CommandIngressConsumerScript)
  .register("test-window-registry",WindowRegistryScript)
  .register("test-window-preparation-source",WindowPreparationSourceScript)
  .register("canonical-architecture", CanonicalArchitectureScript)
  .register("document-backbone-binding-check", DocumentBackboneBindingCheckScript)
  .register("cold-document-pair-ingress-check", ColdDocumentPairIngressCheckScript)
  .register("guest-lifecycle-check", GuestLifecycleCheckScript)
  .register("test-mounted-receiving", MountedReceivingScript)
  .register("check", CheckScript)
  .register("test", TestScript)
  .register("test-reserved-retirement", ReservedRetirementScript)
  .register("test-composed-child-history-source", ComposedChildHistorySourceScript)
  .register("test-extension-invoke-source", ExtensionInvokeSourceScript)
  .register("test-original-job-native", OriginalJobNativeScript)
  .register("test-media-turn-source",MediaTurnSourceScript).register("test-media-turn-native",MediaTurnNativeScript)
  .register("test-reserved-producer-native",ReservedProducerNativeScript)
  .register("test-original-reactor-receiving-source",OriginalReactorReceivingSourceScript)
  .register("test-output-retirement-native", OutputRetirementNativeScript)
  .register("test-fixture-channel-interfaces", FixtureChannelInterfacesScript)
  .register("test-codec-send-source", CodecSendSourceScript)
  .register("artifact-admission-check", ArtifactAdmissionCheckScript)
  .register("schema-document-authority-check", SchemaDocumentAuthorityCheckScript).register("test-camera-presence", CameraPresenceContractScript);
export async function pluginTestRunnerSelfTests(): Promise<number> {
  const { createPluginRunnerTests } = await import("../../🧪️tests/🏃️runner-self-tests/🟦️.ts");
  const [{ default: Ajv }, { parseArgs }, { readFileSync }] = await Promise.all([import("ajv"), import("node:util"), import("node:fs")]);
  return createPluginRunnerTests({ Ajv, assert, parseArgs, pluginTestInvocation, readFileSync }, { directory: import.meta.dir, url: import.meta.url }).pluginTestRunnerSelfTests();
}


/** 📥️ Verifies accepted remote prefixes against SQLite and the mounted plugin runtime. */
class RemoteIngestPrefixScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!["source","native"].includes(args[0]!))throw new Error("test-remote-ingest-prefix source|native");
  const owner=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📥️remote-prefix"),law=JSON.parse(readFileSync(join(owner,"🔣️.json"),"utf8"));
  const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(JSON.parse(readFileSync(join(owner,"🧬️schema/🔣️.json"),"utf8")));assert.equal(validate(law),true,JSON.stringify(validate.errors));
  const {Database}=await import("bun:sqlite"),oracle=new Database(":memory:");
  try{oracle.exec("CREATE TABLE accepted(id TEXT PRIMARY KEY,payload TEXT NOT NULL CHECK(payload='valid'),amount INTEGER NOT NULL)");for(const row of law.cases){this.invocation.control.signal.throwIfAborted();oracle.exec("DELETE FROM accepted");let refused=false,notifications=0;for(const step of row.steps){try{const result=oracle.query("INSERT INTO accepted VALUES('original',?,1) ON CONFLICT DO NOTHING").run(step==="malformed"?"malformed":"valid");notifications+=Number(result.changes);}catch{refused=true;break;}}const value=oracle.query("SELECT COALESCE(SUM(amount),0) AS count FROM accepted").get()as{count:number};assert.equal(refused,row.refused,row.name);assert.equal(value.count,row.count,row.name);assert.equal(notifications,row.notifications,row.name);}}
  finally{oracle.close();}
  if(args[0]==="native")await runAdmittedCargoPackageTestsV1(["semio-framework-plugin"],this.root,this.invocation,["--lib","accepted_remote_prefix_notifies_before_later_refusal","--no-fail-fast","--success-output","immediate"]);
 }
}

router.register("test-remote-ingest-prefix",RemoteIngestPrefixScript);
/** 🏗️ Verifies central builder retirement with neutral SQL receipts and the native owner. */
class SnapshotBuilderOwnershipScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length !== 1 || !["source", "native"].includes(args[0]!)) throw new Error("test-snapshot-builder-ownership source|native");
    await runRepositoryTestCommand(process.execPath, ["test", resolve(this.root, "../../📜️declaration/🪟️surface/🏗️snapshot-builder/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: 15_000 });
    if (args[0] === "native") {
      const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");
      assert.equal(resolve(native.manifestPath),manifest);
      const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,0)},cwd:this.repoRoot,env:process.env,nativeEnv:{RUST_MIN_STACK:native.rustMinStack},manifestPaths:{"semio-framework-plugin":manifest},groups:[{package:"semio-framework-plugin",target:{kind:"test",name:"snapshot_builder_ownership"},cargoArgs:["--locked"],laws:["fixture::snapshot_builder_retires_original_owners_on_success_and_refusal"]}]});
      assert.equal(receipts[0]!.assertions,1);
    }
  }
}
router.register("test-snapshot-builder-ownership", SnapshotBuilderOwnershipScript);
/** 📬️ Checks returned completion owners through the neutral lifecycle and real typed close. */
class CompletionRejectionCloseScript extends BundleScript {
  async run(args:string[]):Promise<void>{
    if(args.length!==1||!["source","native"].includes(args[0]!))throw new Error("test-completion-rejection-close source|native");
    await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../🧵️retained-command/🧪️tests/♻️completion-rejection/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});
    const {completionRejectionOracle}=await import("../../🧪️tests/🧪️artifact-admission-and-completion-oracles/🟦️.ts");
    if(args[0]==="native")await runAdmittedCargoPackageTestsV1(["semio-framework-plugin"],this.root,this.invocation,["--lib","retained_rejected_completion_closes_original_artifact_and_ephemeral_owners","--no-fail-fast"]);
  }
}
router.register("test-completion-rejection-close",CompletionRejectionCloseScript);
/** 🧾️ Verifies original snapshot preparation receipt admission. */
class SnapshotReadPreparationScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!["source","native"].includes(args[0]!))throw new Error("test-snapshot-read-preparation source|native");
  await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../📜️declaration/👁️view/📸️preparation/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});
  await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../⏪️time-travel/♻️retirement/🧪️tests/🟦️.ts"),"-t","original ZST"],{cwd:this.repoRoot,budgetMs:15000});
  if(args[0]==="native"){const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(native.manifestPath),manifest);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,env:process.env,manifestPaths:{"semio-framework-plugin":manifest},nativeEnv:{RUST_MIN_STACK:native.rustMinStack},groups:[{package:"semio-framework-plugin",target:{kind:"test",name:"mounted_snapshot_preparation"},cargoArgs:["--locked","--features","artifact-app-testing"],laws:["receipts::mounted_snapshot_preparation_records_original_receipts_before_readiness","mounted_snapshot_preparation_boundary_uses_original_context_and_readiness","mounted_pending_source_version_compares_complete_original_authority"]}]});}
 }
}
router.register("test-snapshot-read-preparation",SnapshotReadPreparationScript);
/** 🎞️ Verifies original writer-owned and borrowed native tick sources through paid payload publication. */
class OriginalTickPayloadSourcesScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!["source","native"].includes(args[0]!))throw new Error("test-original-tick-payload-sources source|native");
  const file=resolve(this.root,"../../⏯️tool-run/🎞️tick/📦️payload/🧪️tests/🟦️.ts"),sourcePolicy={version:1 as const,maximumElapsedMilliseconds:120000,output:{maximumBytes:1048576,maximumLines:4096}};
  await runRepositoryCommand(this.invocation,process.execPath,["test",file],this.repoRoot,"original-tick-payload-sources",sourcePolicy);
  await runRepositoryCommand(this.invocation,process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file],this.repoRoot,"original-tick-payload-source-types",sourcePolicy);
  if(args[0]==="source")return;const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(native.manifestPath),manifest);
  await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,env:process.env,manifestPaths:{"semio-framework-plugin":manifest},groups:[{package:"semio-framework-plugin",target:{kind:"test",name:"original_tick_payload_sources"},cargoArgs:["--locked"],laws:["original_payload::tests::original_tick_payload_preserves_same_context_authority_and_cancellation"]}]});
 }
}
router.register("test-original-tick-payload-sources",OriginalTickPayloadSourcesScript);

/** 📮️ Verifies persistent Reactor original page custody with independent portable ABI and event-order oracles. */
class ResultPublicationScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!["source","native"].includes(args[0]!))throw new Error("test-result-publication source|native");
  const file=resolve(this.root,"../../⚛️reactor/🏃️runtime/📮️publication/🧪️tests/🟦️.ts"),policy={version:1 as const,maximumElapsedMilliseconds:120000,output:{maximumBytes:1048576,maximumLines:4096}};
  await runRepositoryCommand(this.invocation,process.execPath,["test",file],this.repoRoot,"result-publication-source",policy);
  await runRepositoryCommand(this.invocation,process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file],this.repoRoot,"result-publication-source-types",policy);
  if(args[0]==="source")return;const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(native.manifestPath),manifest);
  await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,0)},cwd:this.repoRoot,env:process.env,manifestPaths:{"semio-framework-plugin":manifest},groups:[{package:"semio-framework-plugin",target:{kind:"lib"},cargoArgs:["--locked"],laws:["component::plugin_runtime::result_publication::tests::native_page_batch_preserves_eight_original_loans_through_funded_receiving","component::plugin_runtime::result_publication::tests::native_page_batch_cancellation_preserves_original_source_and_partial_destination_custody","component::plugin_runtime::result_publication::tests::native_page_publication_cancel_after_birth_keeps_destination_before_ack","component::plugin_runtime::plugin_builder_contract_tests::concurrent_typed_operations_hand_every_presentable_page_to_one_turn","component::plugin_runtime::plugin_builder_contract_tests::original_result_batch_retains_eight_under_separate_original_input","component::plugin_runtime::plugin_builder_contract_tests::retained_operation_continues_after_command_admission_until_publication_and_retirement","component::plugin_runtime::plugin_builder_contract_tests::retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires","component::plugin_runtime::plugin_builder_contract_tests::a_many_mutation_publication_folds_every_mutation_and_is_never_terminated_as_stalled"]}]});
 }
}
router.register("test-result-publication",ResultPublicationScript);
/** 🏠️ Validates genuine Window catalog source admission, paired issuer identity and caller-funded closure. */
class OriginalWindowCatalogScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length!==1||!['source','native'].includes(args[0]!))throw new Error('test-original-window-catalog source|native');const {originalWindowCatalogOracle}=await import('../../🪟️window/🎚️config/🏠️catalog/🧪️tests/🟦️.ts');originalWindowCatalogOracle();if(args[0]==='source')return;const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,'Cargo.toml');await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:240000,listMilliseconds:30000,lawMilliseconds:30000},cwd:this.repoRoot,env:process.env,manifestPaths:{'semio-framework-plugin':manifest},groups:[{package:'semio-framework-plugin',target:{kind:'lib'},cargoArgs:['--locked'],laws:['component::window_config::original_catalog_tests::original_window_catalog_sources_and_tickets_match_physical_original_grants']}]});}}
router.register('test-original-window-catalog',OriginalWindowCatalogScript);
/** 🫴️ Validates exact original artifact document source receiving against independent codecs and native custody laws. */
class OriginalDocumentReceivingScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!["source","native"].includes(args[0]!))throw new Error("test-original-document-receiving source|native");
  const file=resolve(this.root,"../../💡️inference/🫴️receiving/🧪️tests/🟦️.ts"),policy={version:1 as const,maximumElapsedMilliseconds:120000,output:{maximumBytes:1048576,maximumLines:4096}};
  await runRepositoryCommand(this.invocation,process.execPath,["test",file],this.repoRoot,"original-document-receiving-source",policy);
  if(args[0]==="source")return;
  const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(native.manifestPath),manifest);
  await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,env:process.env,manifestPaths:{"semio-framework-plugin":manifest},groups:[{package:"semio-framework-plugin",target:{kind:"lib"},cargoArgs:["--locked"],laws:["component::artifact_inference::document_receiving::tests::artifact_document_original_pair_receiving_preserves_exact_sources_and_physical_receipts","component::artifact_inference::document_receiving::tests::artifact_document_original_receiving_refusal_cancellation_and_foreign_source_retain_partial_custody"]}]});
 }
}
router.register("test-original-document-receiving",OriginalDocumentReceivingScript);
if(import.meta.main) router.register("test-original-producer-source",OriginalProducerSourceScript);
/** 🏛️ Verifies genuine Reserved registry body bytes, original CPU and physical source custody. */
class ReservedRegistryScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!["source","native"].includes(args[0]!))throw new Error("test-reserved-registry source|native");
  const file=resolve(this.root,"../../⚛️reactor/💼️jobs/🏛️reserved/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",file],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});
  await runBudgetedTestCommand(process.execPath,["test",file],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});
  if(args[0]==="source")return;
  await runAdmittedCargoPackageTestsV1(["semio-framework-plugin"],this.root,this.invocation,["--offline","--lib","-E","test(reserved_registry_original_body_bytes_and_actual_receipts_share_one_source) | test(reserved_registry_original_body_work_denials_preserve_input_and_last_unit_progress) | test(reserved_registry_original_partial_output_cancel_keeps_backings_until_same_source_close) | test(reserved_registry_original_expired_clock_closes_partial_output_without_renewal) | test(reserved_registry_original_expired_clock_closes_failed_and_bounded_slot_owners) | test(reserved_registry_original_global_close_rejects_foreign_source_and_keeps_expired_clock) | test(reserved_registry_original_child_release_denial_preserves_last_work_unit)","--","--nocapture"]);
 }
}
router.register("test-reserved-registry",ReservedRegistryScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "check" }));
//#endregion 🎯️Tasks
