#!/usr/bin/env bun




/** 🦀️ Awaited plugin SDK checks and exact-filter native regression tests. */
import { runCargo, runRepositoryCargoTests, runRepositoryExactCargoLaws, runRepositoryTestCommand, nextestArtifactLocation } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import assert from "node:assert/strict";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";


import { resolve } from "node:path";



//#region 🧪️RunnerSelection
/** 🎯️ Selects explicit build inventory or the existing budgeted test runner without interpreting filters. */
export function pluginTestInvocation(segments: string[]): { mode: "inventory" | "budgeted"; args: string[] } {
  const { rest } = resolveTestLevel(segments);
  const boundary = rest.indexOf("--");
  const options = rest.slice(0, boundary < 0 ? rest.length : boundary);
  const inventory = options.includes("--no-run");
  const targets = ["--lib", "--bins", "--bin", "--examples", "--example", "--tests", "--test", "--benches", "--bench", "--all-targets"];
  const args = options.some(option => targets.includes(option.split("=")[0])) ? rest : ["--lib", ...rest];
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
    console.log(`[DEBUG] plugin-runner-oracle cases=${await pluginTestRunnerSelfTests()}`);
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
      const receipts = await runRepositoryExactCargoLaws({
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
    else await runRepositoryCargoTests([], this.root, invocation.args);
  }
}

/** 🧩️ Checks composed child history labels against the independent neutral fixture twin. */
class ComposedChildHistorySourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test-composed-child-history-source accepts no arguments");
    const { composedChildHistoryOracle } = await import("../../🧪️tests/🧪️composed-child-history/🟦️.ts");
    console.log(`[DEBUG] composed-child-history-oracle cases=${composedChildHistoryOracle(this.repoRoot)}`);
  }
}

/** ♻️ Executes the original segmented-output owner through its actual allocator receipt law. */
class OutputRetirementNativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test-output-retirement-native accepts no arguments");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      env: process.env,
      groups: [{ package: "semio-framework-plugin", target: { kind: "lib" }, laws: ["segmented_download_contract::segmented_output_original_full_grant_physical_retirement"] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) { console.log(`[DEBUG] original-output-retirement ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`[DEBUG] original-output-retirement-receipt ${JSON.stringify(receipt)}`);
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
    if (segments.length) throw Error("test-fixture-channel-interfaces accepts no arguments");
    await runRepositoryTestCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🔬️app-declarations-fixture/🟦️.ts")], { cwd: this.repoRoot, budgetMs: 15_000 });
  }
}

/** 🤝️ Keeps actual host pump ownership and all retained native host witnesses in the plugin. */
class CooperativeHostCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("cooperative-host-check accepts only --native");
    await runRepositoryTestCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🤝️cooperative-pump/🟦️.ts")], { cwd: this.repoRoot, budgetMs: 15_000 });
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
      nativeEnv: { RUST_MIN_STACK: "268435456" },
      groups: [{ package: "semio-framework-plugin", target: { kind: "lib" }, laws: ["component::cooperative_pump_tests::cooperative_maintenance_live_host_revisits_queued_owner"] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) { console.log(`cooperative-host-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`cooperative-host-native-receipt: ${JSON.stringify(receipt)}`);
  }
}

/** 🧾️ Verifies explicit shared schema authority through the actual plugin assembly boundary. */
class SchemaDocumentAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { schemaDocumentAuthorityOracle } = await import("../../🏗️builder/🧪️tests/🧾️document-authority/🟦️.ts");
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("Unsupported schema document authority argument");
    console.log(`schema-document-authority oracle=${schemaDocumentAuthorityOracle()}`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runRepositoryExactCargoLaws({
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
    const receipts = await runRepositoryExactCargoLaws({
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
    const { coldDocumentPairIngressOracle, documentBackboneBindingOracle, guestLifecycleOracle, issuedPatchOracle } = await import("../../🧪️tests/🧪️reactor-contract-oracles/🟦️.ts");
    assert(
      segments.every((segment) => segment === "--native"),
      "guest-lifecycle-check accepts only --native",
    );
    console.log(`guest-lifecycle-oracle cases=${guestLifecycleOracle()} patch-cases=${issuedPatchOracle()}`);
    if (!segments.includes("--native")) return;
    const receipts = await runRepositoryExactCargoLaws({
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
    const { coldDocumentPairIngressOracle, documentBackboneBindingOracle, guestLifecycleOracle, issuedPatchOracle } = await import("../../🧪️tests/🧪️reactor-contract-oracles/🟦️.ts");
    assert(segments.every((segment) => segment === "--native"), "cold-document-pair-ingress-check accepts only --native");
    const hostile = await coldDocumentPairIngressOracle(this.repoRoot);
    console.log(`cold-document-pair-ingress-oracle: ajv=1 sha256=3 webcrypto=3 hostile=${hostile} limits=64KiB/64/4MiB`);
    if (!segments.includes("--native")) return;
    const receipts = await runRepositoryExactCargoLaws({
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
    const { coldDocumentPairIngressOracle, documentBackboneBindingOracle, guestLifecycleOracle, issuedPatchOracle } = await import("../../🧪️tests/🧪️reactor-contract-oracles/🟦️.ts");
    assert(segments.every((segment) => segment === "--native"), "document-backbone-binding-check accepts only --native");
    const rows = documentBackboneBindingOracle(this.repoRoot);
    console.log(`document-backbone-binding-oracle: ajv=1 rows=${rows} hot=256KiB genesis=4MiB pending=64/1MiB`);
    if (!segments.includes("--native")) return;
    const receipts = await runRepositoryExactCargoLaws({
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
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
      nativeEnv: { RUST_MIN_STACK: "268435456" },
      groups: [{
        package: "semio-framework-plugin",
        target: { kind: "lib" },
        laws: [
          "artifact_inference_context_preserves_execution_and_registry_identity",
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

/** 🧩️ Proves this owner's complete canonical command ingress consumer contract. */
class CommandIngressConsumerScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-command-ingress-consumer accepts no arguments");
  const {runBudgetedTestCommand}=await import("../../../../../../🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"),{testLevelBudgetMs}=await import("../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts"),source=resolve(this.root,"../../🏛️ownership/📥️command-ingress/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}

/** 🪶️ Verifies the plugin-owned native SQLite snapshot laws. */
class SnapshotSqliteAdmissionScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
 if(segments.length)throw Error("test-snapshot-sqlite-admission accepts no arguments");
 await runRepositoryCargoTests(["semio-framework-plugin"],this.repoRoot,["--lib","sqlite_snapshot_"]);
 }
}

/** ⚠️ Preserves intrinsic SQLite provider refusals across the guest codec boundary. */
class SnapshotSqliteRefusalScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  const [mode,...rest]=segments;
  if(rest.length || !["source","native","host-native"].includes(mode))throw Error("test-snapshot-sqlite-refusal requires source, native or host-native");
  if(mode!=="source"){
   await runRepositoryCargoTests([mode==="native"?"semio-framework-plugin":"semio-framework-plugin-host"],this.repoRoot,["--lib","sqlite_snapshot_guest_refusal_"]);
   return;
  }
  const {runBudgetedTestCommand}=await import("../../../../../../🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts");
  const {testLevelBudgetMs}=await import("../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🧬️schema/🪶️sqlite/⚠️refusal/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}

/** ♻️ Verifies actual retained operation and localized history allocation authorities. */
class RetainedMetadataScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw Error("test-retained-metadata accepts no arguments");
    if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
    const receipts=await runRepositoryExactCargoLaws({cwd:this.root,env:process.env,groups:[{package:"semio-framework-plugin",target:{kind:"lib"},laws:["retained_command::metadata_retirement::tests::retained_command_metadata_preserves_original_history_and_physical_authority"]}],artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,buildBudgetMs:Number(process.env.SEMIO_BUILD_BUDGET_MS??3_600_000),listBudgetMs:60_000,lawBudgetMs:120_000,progress(event){console.log(`[DEBUG] retained-metadata ${event.stage}: ${event.law??""}`);}});
    console.log(`[DEBUG] retained-metadata receipts=${receipts.length}`);
  }
}

/** ♻️ Checks original raw backing receipts independently of logical copy work. */
class RetainedRawScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||!["source","native"].includes(segments[0]))throw Error("test-retained-raw requires source or native");
  const {runBudgetedTestCommand}=await import("../../../../../../🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🧵️retained-command/🧪️tests/♻️raw-allocation-close/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🎟️admission/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🪟️mounted/♻️frontier/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:120_000,throwOnFailure:true});
  if(segments[0]==="source")return;
  if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  const receipts=await runRepositoryExactCargoLaws({cwd:this.root,env:process.env,groups:[{package:"semio-framework-plugin",target:{kind:"lib"},laws:["app::plugin_builder_contract_tests::retained_latest_wins_raw_capacity_is_not_initialized_byte_retirement","retained_command::admission::tests::retained_raw_admission_has_no_failure_destructor_or_undeclared_allocation"]}],artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,buildBudgetMs:3_600_000,listBudgetMs:60_000,lawBudgetMs:120_000,progress(event){console.log(`[DEBUG] retained-raw ${event.stage}`);}});
  console.log(`[DEBUG] retained-raw receipts=${receipts.length}`);
 }
}

/** 📬️ Verifies exact window mutation wrappers and their independently issued payload custody. */
class WindowMutationScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||!["source","native"].includes(segments[0]))throw Error("test-window-mutation requires source or native");
  const {runBudgetedTestCommand}=await import("../../../../../../🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"),file=resolve(this.root,"../../🪟️window/📬️mutation/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,["test",file],{cwd:this.repoRoot,budgetMs:120000,throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file],{cwd:this.repoRoot,budgetMs:120000,throwOnFailure:true});
  if(segments[0]==="source")return;
  if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  const receipts=await runRepositoryExactCargoLaws({cwd:this.root,env:process.env,groups:[{package:"semio-framework-plugin",target:{kind:"lib"},laws:["window_mutation::tests::window_original_mutation_wrappers_have_exact_physical_custody"]}],artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,buildBudgetMs:3_600_000,listBudgetMs:60_000,lawBudgetMs:120000,progress(event){console.log(`[DEBUG] window-mutation ${event.stage}`);}});
  console.log(`[DEBUG] window-mutation nativeReceipts=${receipts.length}`);
 }
}
const router = new ScriptRouter(import.meta.dir).register("test-window-mutation",WindowMutationScript).register("test-retained-raw",RetainedRawScript).register("test-retained-metadata",RetainedMetadataScript).register("test-snapshot-sqlite-refusal",SnapshotSqliteRefusalScript).register("test-snapshot-sqlite-admission",SnapshotSqliteAdmissionScript).register("test-command-ingress-consumer",CommandIngressConsumerScript)
  .register("canonical-architecture", CanonicalArchitectureScript)
  .register("cooperative-host-check", CooperativeHostCheckScript)
  .register("document-backbone-binding-check", DocumentBackboneBindingCheckScript)
  .register("cold-document-pair-ingress-check", ColdDocumentPairIngressCheckScript)
  .register("guest-lifecycle-check", GuestLifecycleCheckScript)
  .register("check", CheckScript)
  .register("test", TestScript)
  .register("test-composed-child-history-source", ComposedChildHistorySourceScript)
  .register("test-output-retirement-native", OutputRetirementNativeScript)
  .register("test-fixture-channel-interfaces", FixtureChannelInterfacesScript)
  .register("test-codec-send-source", CodecSendSourceScript)
  .register("artifact-admission-check", ArtifactAdmissionCheckScript)
  .register("schema-document-authority-check", SchemaDocumentAuthorityCheckScript);
export async function pluginTestRunnerSelfTests(): Promise<number> {
  const { createPluginRunnerTests } = await import("../../🧪️tests/🏃️runner-self-tests/🟦️.ts");
  const [{ default: Ajv }, { parseArgs }, { readFileSync }] = await Promise.all([import("ajv"), import("node:util"), import("node:fs")]);
  return createPluginRunnerTests({ Ajv, assert, parseArgs, pluginTestInvocation, readFileSync }, { directory: import.meta.dir, url: import.meta.url }).pluginTestRunnerSelfTests();
}


if (import.meta.main) await runScriptMain(router, { defaultCommand: "check" });
//#endregion 🎯️Tasks
