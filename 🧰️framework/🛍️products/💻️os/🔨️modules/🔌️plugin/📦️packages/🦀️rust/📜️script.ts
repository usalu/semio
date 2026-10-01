#!/usr/bin/env bun
import { sourceFreshnessOracle } from "../../🧪️tests/🔬️source-freshness/🟦️.ts";
import { schemaDocumentAuthorityOracle } from "../../🏗️builder/🧪️tests/🧾️document-authority/🟦️.ts";
import { extensionRetirementOracle } from "../../🧪️tests/🔬️extension-retirement/🟦️.ts";
import { createPluginRunnerTests } from "../../🧪️tests/🏃️runner-self-tests/🟦️.ts";
/** 🦀️ Awaited plugin SDK checks and exact-filter native regression tests. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo, runCargoTestBudgeted, runExactCargoLaws, nextestArtifactLocation } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parseArgs } from "node:util";
import Ajv from "ajv";

//#region 🧪️RunnerSelection
/** 🎯️ Selects explicit build inventory or the existing budgeted test runner without interpreting filters. */
export function pluginTestInvocation(segments: string[]): { mode: "inventory" | "budgeted"; args: string[] } {
  const { rest } = resolveTestLevel(segments);
  const boundary = rest.indexOf("--");
  const inventory = rest.slice(0, boundary < 0 ? rest.length : boundary).includes("--no-run");
  return inventory ? { mode: "inventory", args: ["test", "--manifest-path", "Cargo.toml", "--lib", ...rest] } : { mode: "budgeted", args: ["--lib", ...rest] };
}
//#endregion 🧪️RunnerSelection

import { artifactAdmissionOracle, completionRejectionOracle } from "../../🧪️tests/🧪️artifact-admission-and-completion-oracles/🟦️.ts";
import { coldDocumentPairIngressOracle, documentBackboneBindingOracle, guestLifecycleOracle, issuedPatchOracle } from "../../🧪️tests/🧪️reactor-contract-oracles/🟦️.ts";
import { declaredVerbVerdictOracle, declaredBridgeArgumentOracle } from "../../🧪️tests/⚖️declared-verb-verdicts/🟦️.ts";
import { mediaOwnerContextOracle } from "../../🧪️tests/🎞️media-owner-context/🟦️.ts";
import { agentLaneCarriageOracle, agentLanePreviewVerdictOracle } from "../../🧪️tests/🤖️agent-lane-preview/🟦️.ts";
import { timeTravelScenarioOracle } from "../../🧪️tests/🧪️time-travel/🟦️.ts";
import { supersedeLedgerOracle } from "../../🧪️tests/🧪️supersede-ledger/🟦️.ts";
import { historyAlternativesOracle } from "../../🧪️tests/🧪️history-alternatives/🟦️.ts";
import { historyLabelReloadOracle } from "../../🧪️tests/🧪️history-label-reload/🟦️.ts";

//#region 🎯️Tasks
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    console.log(`plugin-runner-oracle cases=${pluginTestRunnerSelfTests()}`);
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
    if (segments.length === 1 && segments[0] === "--retained-child-close-exact") {
      const receipts = await runExactCargoLaws({
        cwd: this.root,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [
          {
            package: "semio-framework-plugin",
            target: { kind: "lib" },
            laws: ["app::plugin_builder_contract_tests::retained_command_child_emit_prepublication_close_and_rejected_handoff_are_bounded"],
          },
        ],
        progress(event) {
          console.log(`retained-child-close ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`retained-child-close-receipts: ${JSON.stringify(receipts)}`);
      return;
    }
    const invocation = pluginTestInvocation(segments);
    if (invocation.mode === "inventory") await runCargo(invocation.args, this.root);
    else await runCargoTestBudgeted([], this.root, invocation.args);
  }
}

class CodecSendSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-codec-send-source accepts no arguments");
    const { testPluginCodecCallerSource } = await import("../../🧪️tests/🔣️codec-caller-source/🟦️.ts");
    testPluginCodecCallerSource(this.repoRoot);
  }
}

/** 🧾️ Verifies explicit shared schema authority through the actual plugin assembly boundary. */
class SchemaDocumentAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("Unsupported schema document authority argument");
    console.log(`schema-document-authority oracle=${schemaDocumentAuthorityOracle()}`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: "268435456" },
      groups: [{ package: "semio-framework-plugin", target: { kind: "lib" }, laws: ["builder::schema_document_authority_tests::schema_document_authority_follows_portable_owner_corpus"] }],
      progress(event) { console.log(`schema-document-authority ${event.stage}: ${event.law ?? ""}`); },
    });
    console.log(`schema-document-authority native-laws=${receipts.length}`);
  }
}

class ArtifactAdmissionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    assert(
      segments.every((segment) => segment === "--oracle-only"),
      "unsupported artifact admission check argument",
    );
    console.log(`artifact-admission-oracle cases=${artifactAdmissionOracle(this.repoRoot)} firstParty=39`);
    if (segments.includes("--oracle-only")) return;
    const laws = [
      "strict_artifact_identity_all_builder_channels_reject_before_publication",
      "strict_artifact_identity_mixed_channels_publish_nothing",
      "strict_artifact_identity_matches_independent_neutral_fixture",
      "strict_artifact_identity_owned_tree_and_definition_channels_publish",
    ];
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: "268435456" },
      groups: [{ package: "semio-framework-plugin", target: { kind: "lib" }, laws }],
      progress(event) {
        console.log(`artifact-admission ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`artifact-admission-laws: ${JSON.stringify(receipts)}`);
  }
}


class GuestLifecycleCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    assert(
      segments.every((segment) => segment === "--native"),
      "guest-lifecycle-check accepts only --native",
    );
    console.log(`guest-lifecycle-oracle cases=${guestLifecycleOracle()} patch-cases=${issuedPatchOracle()}`);
    if (!segments.includes("--native")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: "33554432", CARGO_BUILD_JOBS: "1" },
      groups: [
        {
          package: "semio-framework-plugin",
          target: { kind: "lib" },
          laws: [
            "component::plugin_runtime::runtime_cleanup_fault_vector_tests::non_fault_status_reprs_never_collide_with_a_fault_variant",
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
            "component::reactor::pending::instance_lifetime_patch_close_tests::guest_instance_lifecycle_pending_patch_handback_preserves_rejected_owner_and_exact_bytes",
            "component::reactor::patches::tests::issued_obsolete_reconcile_feedback_retires_only_the_old_pending_owner",
            "component::reactor::instance_lifetime::tests::guest_instance_lifecycle_ack_fault_keeps_exact_receipt_and_owner",
            "component::reactor::instance_lifetime::tests::guest_instance_lifecycle_terminal_release_work_is_measured_and_never_repeated_after_late_clock",
            "component::reactor::instance_lifetime::tests::guest_instance_lifecycle_same_activation_reopen_rejects_old_authority",
            "component::plugin_runtime::plugin_builder_contract_tests::reactor_native_lifecycle_retains_exact_close_until_ack",
            "component::plugin_runtime::plugin_builder_contract_tests::reactor_native_lifecycle_rejects_foreign_and_colliding_owners",
            "component::plugin_runtime::plugin_builder_contract_tests::reactor_native_lifecycle_output_failure_preserves_ack_and_owner",
            "component::plugin_runtime::plugin_builder_contract_tests::reactor_output_fault_returns_real_patch_and_preserves_other_lifecycle_ack",
          ],
        },
      ],
      buildBudgetMs: 86_400_000,
      lawBudgetMs: 60_000,
      progress(event) {
        console.log(`guest-lifecycle ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`guest-lifecycle-receipts: ${JSON.stringify(receipts)}`);
  }
}


class ColdDocumentPairIngressCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    assert(segments.every((segment) => segment === "--native"), "cold-document-pair-ingress-check accepts only --native");
    const hostile = await coldDocumentPairIngressOracle(this.repoRoot);
    console.log(`cold-document-pair-ingress-oracle: ajv=1 sha256=3 webcrypto=3 hostile=${hostile} limits=64KiB/64/4MiB`);
    if (!segments.includes("--native")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: "33554432", CARGO_BUILD_JOBS: "1" },
      nativeEnv: { RUST_MIN_STACK: "268435456", CARGO_BUILD_RUSTFLAGS: "-Z threads=1" },
      groups: [
        {
          package: "semio-framework-actor",
          target: { kind: "lib" },
          laws: [
            "cold_pair_tests::cold_pair_ingress_status_pack_round_trips_every_exact_variant",
            "cold_pair_tests::cold_pair_ingress_neutral_fixture_has_exact_semantic_receipts_and_hostiles",
            "cold_pair_tests::cold_pair_ingress_decode_refuses_noncanonical_authority_before_publication",
          ],
        },
        {
          package: "semio-framework-plugin",
          target: { kind: "lib" },
          laws: [
          "component::reactor::cold_pair::tests::cold_pair_ingress_streams_the_exact_four_mibibyte_pair_and_loads_once",
          "component::reactor::cold_pair::tests::cold_pair_ingress_rechecks_live_and_rejects_hostile_pages_without_displacement",
          "component::reactor::cold_pair::tests::cold_pair_ingress_keeps_the_structural_owner_across_load_cancel_and_bounded_close",
          "component::reactor::cold_pair::tests::cold_pair_ingress_final_live_fence_rejects_post_await_revocation",
          "component::reactor::cold_pair::tests::cold_pair_ingress_charges_aggregate_reserved_capacity_until_final_close",
          "component::reactor::cold_pair::tests::cold_pair_ingress_is_an_exact_retained_native_close_participant",
          "component::reactor::cold_pair::tests::cold_pair_header_requires_an_active_checkpoint_frontier_and_exact_hashes",
          ],
        },
      ],
      buildBudgetMs: 86_400_000,
      lawBudgetMs: 60_000,
      progress(event) { console.log(`cold-document-pair-ingress ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
    });
    console.log(`cold-document-pair-ingress-receipts: ${JSON.stringify(receipts)}`);
  }
}


class DocumentBackboneBindingCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    assert(segments.every((segment) => segment === "--native"), "document-backbone-binding-check accepts only --native");
    const rows = documentBackboneBindingOracle(this.repoRoot);
    console.log(`document-backbone-binding-oracle: ajv=1 rows=${rows} hot=256KiB genesis=4MiB pending=64/1MiB`);
    if (!segments.includes("--native")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: "33554432", CARGO_BUILD_JOBS: "1" },
      nativeEnv: { RUST_MIN_STACK: "268435456", CARGO_BUILD_RUSTFLAGS: "-Z threads=1" },
      groups: [
        {
          package: "semio-framework-replication",
          target: { kind: "lib", name: "protocol" },
          laws: ["causal::tests::document_backbone_batch_fixture_is_exact_bounded_and_u64_safe"],
        },
        {
          package: "semio-framework-os-kernel",
          target: { kind: "lib", name: "semio_framework_os_kernel" },
          cargoArgs: ["--features", "sync"],
          laws: [
            "os_store::sync::tests::document_backbone_mailbox_and_retention_are_exact_bounded_and_terminal",
            "os_store::sync::tests::actor_tests::raw_document_backbone_reaches_hub_once_and_returns_one_canonical_event",
            "os_store::sync::native_actor::retained_turn_fixtures::retained_readiness_wake_after_turn_release_is_observed_once",
            "os_store::sync::native_actor::retained_turn_fixtures::cancellation_cannot_complete_before_an_inflight_turn_returns_its_exact_owner",
          ],
        },
        {
          package: "semio-framework-plugin",
          target: { kind: "lib" },
          laws: [
            "component::document_backbone_binding::tests::document_backbone_op_binary_is_exact_canonical_and_bounded",
            "component::document_backbone_binding::tests::document_backbone_binding_reducer_preserves_generation_and_live_owner",
          ],
        },
      ],
      buildBudgetMs: 86_400_000,
      lawBudgetMs: 60_000,
      progress(event) { console.log(`document-backbone-binding ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
    });
    console.log(`document-backbone-binding-receipts: ${JSON.stringify(receipts)}`);
  }
}

/** 🏛️ Verifies worker-owned command publication and immutable presence authority. */
class CanonicalArchitectureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some((segment) => segment !== "--oracle-only")) throw new Error("canonical-architecture accepts only --oracle-only");
    console.log(`declared-bridge-argument-oracle assertions=${declaredBridgeArgumentOracle()}`);
    console.log(`media-owner-context-oracle assertions=${mediaOwnerContextOracle()}`);
    console.log(`extension-retirement-oracle cases=${extensionRetirementOracle(this.repoRoot)}`);
    console.log(`source-freshness-oracle cases=${await sourceFreshnessOracle(this.repoRoot,nextestArtifactLocation(this.repoRoot).directory)}`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
      nativeEnv: { RUST_MIN_STACK: "268435456" },
      groups: [{
        package: "semio-framework-plugin",
        target: { kind: "lib" },
        laws: [
          "media_export_request_context_preserves_exact_supplied_owner",
          "declared_bridge_required_arguments_match_neutral_schemas",
          "language_neutral_action_collections_agree_with_json_pointer_oracle",
          "a_command_reaches_both_ephemeral_lanes_without_touching_history",
          "a_command_that_emits_nothing_ephemeral_leaves_both_lanes_untouched",
          "peer_presence_capture_is_one_arc_and_retirement_waits_for_then_drains_the_exact_root",
          "peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority",
          "extension_bundle_resource_retirement_preserves_zero_cancel_progress_and_terminal_owner",
          "extension_bundle_resource_retirement_replacement_retains_old_and_backpressured_candidates",
          "extension_bundle_resource_retirement_rejects_false_terminal_and_exceeded_grants",
          "extension_bundle_resource_retirement_runs_through_suspend_and_exported_poll",
          "extension_bundle_resource_retirement_refuses_implicit_live_drop_and_disposes_cold_explicitly",
          "extension_bundle_resource_retirement_admits_exact_backing_and_boxed_shell_allocations",
        ],
      }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) { console.log(`canonical-architecture-command ${event.stage}: ${event.law ?? ""}`); },
    });
    console.log(`canonical-architecture-command receipts=${receipts.length}`);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("canonical-architecture", CanonicalArchitectureScript)
  .register("document-backbone-binding-check", DocumentBackboneBindingCheckScript)
  .register("cold-document-pair-ingress-check", ColdDocumentPairIngressCheckScript)
  .register("guest-lifecycle-check", GuestLifecycleCheckScript)
  .register("check", CheckScript)
  .register("test", TestScript)
  .register("test-codec-send-source", CodecSendSourceScript)
  .register("artifact-admission-check", ArtifactAdmissionCheckScript)
  .register("schema-document-authority-check", SchemaDocumentAuthorityCheckScript);
const createPluginRunnerTestsInstance = createPluginRunnerTests({ Ajv, assert, parseArgs, pluginTestInvocation, readFileSync }, { directory: import.meta.dir, url: import.meta.url });
export const pluginTestRunnerSelfTests = createPluginRunnerTestsInstance.pluginTestRunnerSelfTests;


if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
//#endregion 🎯️Tasks
