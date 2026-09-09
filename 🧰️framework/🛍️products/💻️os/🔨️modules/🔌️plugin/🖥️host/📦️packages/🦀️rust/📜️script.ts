#!/usr/bin/env bun
/** 🖥️ Runs owned plugin-host checks and exact native test filters. */
import { SCALE_COMPONENT_ARTIFACT } from "../../../../../🧪️testkit/⚖️scale/🟦️.ts";
import assert from "node:assert/strict";
import Ajv from "ajv";
import findIndex from "lodash-es/findIndex.js";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, orchestratorBudgetOpts, runBundleScriptMain, runCargo, runCmd, runProbe, runExactCargoLaws } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

//#region 🎯️Tasks
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }
}

const countOccurrences = (source: string, needle: string): number => source.split(needle).length - 1;

/** 💡️ Verifies the closed inference-proposal intent is mapped by both host execution modes. */
function assertInferenceProposalConversionSource(wit: string, synchronous: string, asynchronous: string): void {
  assert.equal(countOccurrences(wit, "request-inference-proposal(request-inference-proposal-effect)"), 1);
  assert.equal(countOccurrences(synchronous, "E::RequestInferenceProposal(inner) => Effect::RequestInferenceProposal"), 1);
  assert.equal(countOccurrences(asynchronous, "E::RequestInferenceProposal(inner) => K::RequestInferenceProposal"), 1);
  assert.equal(countOccurrences(synchronous, "wit_effects::InferenceProposalKind::GisMapBoundsRegion => semio_framework::kernel::InferenceProposalKind::GisMapBoundsRegion"), 1);
  assert.equal(countOccurrences(asynchronous, "wit_effects::InferenceProposalKind::GisMapBoundsRegion => semio_framework::kernel::InferenceProposalKind::GisMapBoundsRegion"), 1);
}

class InferenceProposalConversionCheckScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    const hostRoot = join(import.meta.dir, "..", "..");
    const wit = readFileSync(join(hostRoot, "..", "🧬️schema", "📜️.wit"), "utf8");
    const synchronous = readFileSync(join(hostRoot, "🦀️.rs"), "utf8");
    const asynchronous = readFileSync(join(hostRoot, "📥️imports", "🦀️.rs"), "utf8");
    assertInferenceProposalConversionSource(wit, synchronous, asynchronous);
    assert.throws(() => assertInferenceProposalConversionSource(wit, synchronous, asynchronous.replace("E::RequestInferenceProposal(inner) => K::RequestInferenceProposal", "E::MissingInferenceProposal(inner) => K::RequestInferenceProposal")));
    console.log("plugin-host-inference-proposal-conversion-source: wit=1 sync=1 async=1 mutation=1 passed");
  }
}

class LifecycleCheckScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    const oracle = join(this.root, "..", "..", "🧪️tests", "♻️relay-lifecycle", "🟦️.ts");
    const repoTest = join(this.root, "..", "..", "..", "..", "..", "..", "🦑️repo", "🔨️modules", "🧪️test", "📜️script.ts");
    if (!existsSync(oracle)) throw new Error(`missing relay lifecycle oracle at ${oracle}`);
    if (!existsSync(repoTest)) throw new Error(`missing repository test runner at ${repoTest}`);
    runCmd("bun", [oracle], { cwd: this.root });
    runCmd("bun", [repoTest, "subject", "fundamental", "--case", "♻️relay-lifecycle", "--implementation", "rust"], {
      cwd: this.root,
      budgetMs: 900_000,
      env: { ...process.env, SEMIO_TEST_BUDGET_MS: "900000" },
    });
    const focused = [
      "component::shard::tests::replay_owners_drop_safely_from_every_owned_frontier_and_balance_accounting",
      "component::shard::tests::replay_failure_and_actor_loss_enter_one_close_funnel_before_reporting",
      "component::shard::tests::spawn_job_effect_is_admitted_stepped_across_multiple_pumps_and_completion_reaches_the_originating_actor",
      "component::shard::tests::cancel_job_effect_stops_a_job_before_it_is_ever_stepped",
      "component::shard::tests::cancel_job_effect_failure_retires_the_actor_and_surfaces_the_typed_fault",
      "component::shard::tests::cancel_unregisters_the_instance_and_no_further_step_job_happens",
      "component::shard::tests::actor_cancel_failure_retires_the_instance_and_reports_fault_instead_of_cancelled",
      "component::shard::tests::exclusive_placement_is_stepped_before_inline_placement_admitted_the_same_pump",
      "component::shard::tests::exclusive_selection_never_crosses_a_lifecycle_barrier",
      "component::shard::tests::job_step_uses_the_owning_actors_last_granted_budget",
      "component::shard::executor::tests::fifo_ingress_selects_interactive_before_earlier_background_without_unbounded_drain",
      "component::effects::tests::router_effect_runs_through_the_retained_compute_session",
      "component::effects::tests::router_effect_on_a_stopped_compute_pool_returns_worker_lost_without_stranding_its_owner",
      "component::guest_cold_relay_tests::detached_reaper_reclaims_one_slot_per_opportunity_round_robin_and_refuses_stale_generation",
      "component::guest_cold_relay_tests::detached_reaper_never_steals_a_live_draining_callers_exact_output",
      "component::guest_cold_relay_tests::dropping_a_pending_mounted_future_reaps_without_a_second_foreground_poll",
      "component::guest_cold_relay_tests::wake_incapable_close_uses_one_coalesced_bounded_fallback",
      "component::guest_cold_relay_tests::retained_pool_future_retries_saturation_once_and_terminalizes_shutdown",
      "component::guest_cold_relay_tests::neutral_relay_lifecycle_traces_drive_production_machines",
    ];
    const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--lib", "--", "--list"], { cwd: this.root, ...orchestratorBudgetOpts() });
    if (listed.status !== 0) throw new Error(`plugin-host lifecycle inventory failed with status ${listed.status}`);
    const discovered = listed.stdout
      .split("\n")
      .filter((line) => line.endsWith(": test"))
      .map((line) => line.slice(0, -": test".length));
    const laws = focused.map((suffix) => {
      const matches = discovered.filter((name) => name.endsWith(suffix));
      if (matches.length !== 1) throw new Error(`plugin-host lifecycle gate expected exactly one ${suffix} law, selected ${matches.length}`);
      return matches[0]!;
    });
    console.log(`plugin-host-lifecycle-laws: ${laws.join(" ")}`);
    for (const law of laws) await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", law, "--", "--exact"], this.root);
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib"], this.root);
    await runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features"], this.root);
  }
}

/** 🧬️ Resolves one named export of a schema module document through the repository draft-07 dialect. */
function moduleExportValidator(ajv: Ajv, modulePath: string, exportId: string) {
  const document = JSON.parse(readFileSync(modulePath, "utf8"));
  ajv.addSchema(document);
  const validate = ajv.getSchema(`${document.$id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`schema module ${modulePath}: unknown export ${exportId}`);
  return validate;
}

/** 🪪️ Pins typed guest-fault transport and independently checks lifecycle replay eligibility. */
function guestFaultOracle(): number {
  const hostRoot = join(import.meta.dir, "..", "..");
  const fixture = JSON.parse(readFileSync(join(hostRoot, "🔁️lifecycle", "🧫️fixtures", "🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  const validate = moduleExportValidator(ajv, join(hostRoot, "🔁️lifecycle", "🧬️schema", "🔣️.json"), "ReactorTurnLifecycleV1");
  assert(validate(fixture), JSON.stringify(validate.errors));
  const eligible = ajv.compile({
    type: "object",
    required: ["code", "retryable", "events"],
    properties: { code: { const: fixture.code }, retryable: { const: true }, events: { type: "array", maxItems: 1, items: { enum: ["open", "close", "ack"] } } },
  });
  for (const row of fixture.cases) {
    const actual = row.code === fixture.code && row.retryable === true && row.events.length <= 1 && row.events.every((event: string) => ["open", "close", "ack"].includes(event));
    assert.equal(actual, row.eligible, row.id);
    assert.equal(eligible(row), actual, row.id);
  }
  const host = readFileSync(join(hostRoot, "🦀️.rs"), "utf8");
  const runtime = readFileSync(join(hostRoot, "⏳️runtime", "🦀️.rs"), "utf8");
  assert(host.includes("Guest(semio_framework::Fault)"), "typed guest fault must survive host decode");
  assert(host.includes("poll_result.map_err(decode_guest_plugin_error)?"), "real Wasmtime poll must use typed decoder");
  assert(host.includes("result.map_err(|error| decode_guest_fault_bytes(&error))"), "owned ABI must use typed decoder");
  assert(runtime.includes("Ok(Err(fault)) => Err(super::decode_guest_plugin_error(fault))"), "async component poll must preserve typed guest fault");
  assert(!runtime.includes("Sender<Result<KernelTurnResult, String>>"), "poll oneshot must not stringify faults");
  return fixture.cases.length;
}

/** 📨️ Checks neutral retry ordering against an independent collection implementation. */
function retainedLifecycleOracle(): number {
  const root = join(import.meta.dir, "..", "..", "🧵️shard", "🔁️lifecycle");
  const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures", "🔣️.json"), "utf8"));
  const validate = moduleExportValidator(new Ajv({ strict: true, allErrors: true }), join(root, "🧬️schema", "🔣️.json"), "ShardLifecycleV1");
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const trace of fixture.traces) {
    const pending = [...trace.queued] as string[];
    const result: string[] = [];
    let yieldPeer = true;
    while (pending.length) {
      const retry = pending.indexOf(trace.retry);
      const peer = findIndex(pending, (item: string) => item[0] !== trace.retry[0]);
      const selected = retry < 0 ? 0 : yieldPeer && peer >= 0 ? peer : retry;
      result.push(pending.splice(selected, 1)[0]!);
      if (retry >= 0) yieldPeer = false;
    }
    assert.deepEqual(result, trace.expected, trace.id);
  }
  assert(existsSync(join(root, "🦀️.rs")), "native retained authority owner must be mounted");
  const owner = readFileSync(join(root, "🦀️.rs"), "utf8");
  const shard = readFileSync(join(root, "..", "🦀️.rs"), "utf8");
  assert(owner.includes("struct AdmittedAuthority"));
  assert(owner.includes("struct ShardActorAllocation"));
  assert(shard.includes("retryable_lifecycle_turn(&fault, &events)"));
  assert(shard.includes("self.has_lifecycle_retry()"), "retry must permit one primed ingress frame");
  return fixture.traces.length;
}

/** 🎠️ Checks failed activation ownership with a separate declarative validator. */
function activationOwnershipOracle(): number {
  const root = join(import.meta.dir, "..", "..", "🎠️activation");
  const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures", "🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  const validate = moduleExportValidator(ajv, join(root, "🧬️schema", "🔣️.json"), "ActivationAdmissionV1");
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const row of fixture.cases) {
    const actual = { actorRetained: row.stage === "complete", instantiations: ["instantiate", "register", "complete"].includes(row.stage) ? 1 : 0, drops: row.stage === "register" ? 1 : 0, admitted: row.stage === "complete" };
    const expected = { actorRetained: row.actorRetained, instantiations: row.instantiations, drops: row.drops, admitted: row.admitted };
    assert.deepEqual(actual, expected, row.id);
    const independent = ajv.compile({ type: "object", required: Object.keys(expected), properties: Object.fromEntries(Object.entries(expected).map(([key, value]) => [key, { const: value }])) });
    assert(independent(actual), row.id);
  }
  assert(existsSync(join(root, "🦀️.rs")), "shared activation ownership boundary must be mounted");
  const osRoot = join(root, "..", "..", "..", "..");
  for (const facade of [join(osRoot, "🖥️host", "🎠️activation", "🦀️.rs"), join(osRoot, "🔨️modules", "📺️renderer", "🧑‍🎨engine", "🎯️targets", "🧊️wgpu", "🎠️runtime", "🦀️.rs")]) {
    assert.equal(countOccurrences(readFileSync(facade, "utf8"), "semio_framework_plugin_host::activation::install_actor("), 1, facade);
  }
  return fixture.cases.length;
}

/** 🎟️ Validates reservation event traces against a separate collection-based transition model. */
function kernelReservationOracle(): number {
  const root = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..", "🔨️modules", "🎭️actor", "🎠️activation-reservation");
  const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures", "🔣️.json"), "utf8"));
  const validate = moduleExportValidator(new Ajv({ strict: true, allErrors: true }), join(root, "🧬️schema", "🔣️.json"), "ActivationReservation");
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const row of fixture.traces) {
    let live = false,
      active = false,
      queued = false,
      grants = 0;
    for (const event of row.events) {
      if (event === "reserve") live = true;
      if (event === "submit") queued = true;
      if (event === "bind") active = true;
      if (event === "abort") live = active = queued = false;
      if (event === "tick" && live && active && queued) {
        grants++;
        queued = false;
      }
    }
    const bind = findIndex(row.events, (event: string) => event === "bind");
    const granted = bind >= 0 && findIndex(row.events, (event: string, index: number) => index > bind && event === "tick") >= 0;
    assert.equal(grants, Number(granted), row.id);
    assert.equal(grants, row.grants, row.id);
    assert.equal(live, row.actorRetained, row.id);
  }
  assert(existsSync(join(root, "🦀️.rs")), "owned Kernel activation reservation must be mounted");
  return fixture.traces.length;
}

class GuestFaultCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    assert(
      segments.every((segment) => segment === "--native"),
      "guest-fault-check accepts only --native",
    );
    console.log(`guest-fault-oracle cases=${guestFaultOracle()} retries=${retainedLifecycleOracle()} activations=${activationOwnershipOracle()} reservations=${kernelReservationOracle()}`);
    if (!segments.includes("--native")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: "33554432", CARGO_BUILD_JOBS: "1" },
      groups: [
        {
          package: "semio-framework-actor",
          target: { kind: "lib" },
          laws: ["activation::tests::neutral_reservation_traces_gate_dispatch_until_exact_binding", "activation::tests::reservation_exhaustion_collision_and_stale_binding_leave_no_partial_admission"],
        },
        {
          package: "semio-framework-plugin-host",
          target: { kind: "lib" },
          laws: [
            "component::shard::executor::tests::terminal_registration_reply_wakes_outside_the_state_lock",
            "component::shard::executor::tests::admitted_registration_reply_wakes_outside_the_state_lock",
            "component::shard::executor::tests::shard_stack_authority_matches_the_neutral_fixture",
            "component::shard::executor::tests::shard_executor_drives_a_turn_for_a_registered_actor_via_the_worker_pool",
            "component::shard::executor::tests::fifo_ingress_selects_interactive_before_earlier_background_without_unbounded_drain",
            "component::shard::executor::tests::every_actors_grant_lands_on_the_shard_it_was_registered_on_across_k_shards",
            "component::shard::executor::tests::suspend_then_resume_round_trip_lands_on_a_shard_where_the_actor_is_registered",
            "component::shard::executor::tests::concurrent_send_frame_bursts_never_drop_an_outcome",
            "component::shard::executor::tests::pending_drive_wake_and_wake_storm_claim_exactly_one_schedule",
            "component::shard::executor::tests::registration_acknowledgement_and_terminal_refusal_preserve_exact_owners",
            "component::shard::lifecycle::tests::unknown_transport_actors_never_allocate_host_bookkeeping",
            "component::cold_pair::tests::neutral_cold_ingress_variants_preserve_authority_and_refuse_hostile_wit",
            "component::cold_pair::tests::native_cold_pages_use_dedicated_bounded_input",
            "component::activation::tests::neutral_activation_failures_retire_the_exact_kernel_and_guest_owners",
            "component::guest_fault_tests::structured_guest_fault_survives_wit_owned_and_async_channels",
            "component::guest_fault_tests::malformed_and_oversized_guest_faults_cannot_authorize_retry",
            "component::shard::lifecycle::tests::neutral_retry_order_uses_the_production_selector",
            "component::shard::lifecycle::tests::retry_keeps_exact_event_budget_credit_and_one_peer_order",
            "component::shard::lifecycle::tests::retry_refuses_replacement_and_stale_queue_cannot_reach_same_id_successor",
            "component::shard::lifecycle::tests::cancellation_revokes_retry_before_another_guest_call",
            "component::shard::lifecycle::tests::retry_occupies_the_original_fixed_lane_capacity",
            "component::shard::lifecycle::tests::terminal_faults_never_create_a_lifecycle_retry",
          ],
        },
      ],
      buildBudgetMs: 86_400_000,
      lawBudgetMs: 60_000,
      progress(event) {
        console.log(`guest-fault ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`guest-fault-receipts: ${JSON.stringify(receipts)}`);
  }
}

/** 🩹️ Validates the one-owner reverse WIT patch bridge and its two native host insertions. */
class UiPatchMarshallingCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    assert(
      segments.every((segment) => segment === "--native"),
      "ui-patch-marshalling-check accepts only --native",
    );
    const hostRoot = join(import.meta.dir, "..", "..");
    const owner = join(hostRoot, "📥️ui-patch");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures", "🔣️.json"), "utf8"));
    const validate = moduleExportValidator(new Ajv({ strict: true, allErrors: true }), join(owner, "🧬️schema", "🔣️.json"), "NativeUiPatchMarshallingV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.equal(new Set(fixture.operationKinds).size, 11);
    for (const row of fixture.cases) {
      const count = row.emitted + row.returned;
      const accepted = count <= 1 && row.receipt === (count === 1);
      assert.equal(accepted ? "accepted" : "rejected", row.expected, row.id);
    }
    const componentExpected = new Map([
      ["returned", ["accepted", "returned", 201]],
      ["imported", ["accepted", "emitted", 101]],
      ["both-channels", ["rejected", "both", null]],
      ["malformed-then-recovered", ["rejected", "emitted", 302]],
    ]);
    for (const row of fixture.componentCases) {
      const expected = componentExpected.get(row.id);
      assert(expected, row.id);
      assert.equal(row.first, expected[0], row.id);
      assert.equal(row.channel, expected[1], row.id);
      assert.equal(row.root, expected[2], row.id);
    }
    const codec = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const synchronous = readFileSync(join(hostRoot, "🦀️.rs"), "utf8");
    const asynchronous = readFileSync(join(hostRoot, "⏳️runtime", "🦀️.rs"), "utf8");
    const scale = readFileSync(join(this.repoRoot, "🧰️framework", "🛍️products", "💻️os", "🧫️fixtures", "⚖️scale", "🦀️.rs"), "utf8");
    for (const kind of ["Upsert", "SetComponent", "SetLayout", "SetActivity", "SetChildren", "SetStyle", "SetAccessibility", "SetBindings", "SetMenu", "Remove", "SetRoot"]) {
      assert(codec.includes(`wit_ui::PatchOp::${kind}`), `missing ${kind} reverse codec`);
    }
    assert.equal(countOccurrences(synchronous, "ui_patch::wit_ui_patches_to_kernel("), 1);
    assert.equal(countOccurrences(asynchronous, "super::ui_patch::wit_ui_patches_to_kernel("), 1);
    assert(!synchronous.includes("let _emitted_patches"));
    assert(!asynchronous.includes("let (emitted, _patches)"));
    assert(scale.includes("semio::framework::host_async::emit_patch(&patch)"));
    assert(scale.includes("ui_patch_receipt"));
    console.log(`plugin-host-ui-patch-marshalling-source: ajv=1 cases=${fixture.cases.length} component=${fixture.componentCases.length} ops=${fixture.operationKinds.length} sync=1 async=1 passed`);
    if (!segments.includes("--native")) return;
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    assert(artifactRoot, "SEMIO_TEST_ARTIFACT_DIR is required");
    const scaleWasm = join(this.repoRoot, SCALE_COMPONENT_ARTIFACT);
    assert(existsSync(scaleWasm), "registered scale component was not materialized");
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432", CARGO_BUILD_JOBS: "1", SEMIO_UI_PATCH_SCALE_WASM: scaleWasm },
      nativeEnv: { RUST_MIN_STACK: "268435456" },
      groups: [
        {
          package: "semio-framework-plugin-host",
          target: { kind: "lib" },
          laws: [
            "component::ui_patch::tests::every_wit_patch_variant_moves_into_one_exact_kernel_owner",
            "component::ui_patch::tests::emitted_and_returned_channels_are_atomic_bounded_and_drained_once",
            "component::ui_patch::tests::target_budget_receipt_and_malformed_pack_refuse_before_publication",
            "component::ui_patch_component_tests::genuine_component_returned_and_imported_patches_keep_channel_order_and_exact_authority",
            "component::ui_patch_component_tests::imported_then_returned_channels_refuse_atomically_without_a_transport_token",
            "component::ui_patch_component_tests::malformed_import_is_drained_before_the_next_exact_patch_owner_is_published",
          ],
        },
      ],
      buildBudgetMs: 86_400_000,
      lawBudgetMs: 60_000,
      progress(event) {
        console.log(`ui-patch-marshalling ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`ui-patch-marshalling-receipts: ${JSON.stringify(receipts)}`);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("check", CheckScript)
  .register("test", TestScript)
  .register("inference-proposal-conversion-check", InferenceProposalConversionCheckScript)
  .register("lifecycle-check", LifecycleCheckScript)
  .register("guest-fault-check", GuestFaultCheckScript)
  .register("ui-patch-marshalling-check", UiPatchMarshallingCheckScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
//#endregion 🎯️Tasks
