#!/usr/bin/env bun
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
/** 🖥️ Runs owned plugin-host checks and exact native test filters. */
const SCALE_COMPONENT_ARTIFACT = "🧰️framework/🛍️products/💻️os/🧪️testing/⚖️scale/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm";
import assert from "node:assert/strict";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import findIndex from "lodash-es/findIndex.js";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { orchestratorBudgetOpts, runCargo, runCmd, runProbe, runRepositoryExactCargoLaws } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

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

/** 💡️ Verifies owner-addressed operations preserve their coordinates and binary payload. */
function assertServiceOperationConversionSource(wit: string, synchronous: string, asynchronous: string): void {
  assert.equal(countOccurrences(wit, "request-service-operation(request-service-operation-effect)"), 1);
  assert.equal(countOccurrences(synchronous, "E::RequestServiceOperation(inner) => Effect::RequestServiceOperation"), 1);
  assert.equal(countOccurrences(asynchronous, "E::RequestServiceOperation(inner) => K::RequestServiceOperation"), 1);
  for (const source of [synchronous, asynchronous]) {
    const arm = source.slice(source.indexOf("E::RequestServiceOperation(inner)"));
    for (const field of ["owner", "service_id", "action"]) assert(arm.slice(0, arm.indexOf("},")).includes(`${field}: inner.${field}`));
    assert(arm.slice(0, arm.indexOf("},")).includes("payload: decode_dsl(&inner.payload).await.ok_or_else("));
  }
}

class ServiceOperationConversionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    assert(segments.every((segment) => segment === "--native"), "service-operation-conversion-check accepts only --native");
    const hostRoot = join(import.meta.dir, "..", "..");
    const wit = readFileSync(join(hostRoot, "..", "🧬️schema", "📜️.wit"), "utf8");
    const synchronous = readFileSync(join(hostRoot, "🦀️.rs"), "utf8");
    const asynchronous = readFileSync(join(hostRoot, "📥️imports", "🦀️.rs"), "utf8");
    const kernel = join(this.repoRoot, "🧰️framework", "🔨️modules", "🎠️kernel");
    const fixture = JSON.parse(readFileSync(join(kernel, "🧫️fixtures", "💡️service-operation", "🔣️.json"), "utf8"));
    assertServiceOperationConversionSource(wit, synchronous, asynchronous);
    assert.throws(() => assertServiceOperationConversionSource(wit, synchronous, asynchronous.replace("E::RequestServiceOperation(inner) => K::RequestServiceOperation", "E::MissingServiceOperation(inner) => K::RequestServiceOperation")));
    console.log(`plugin-host-service-operation-source: effects=${fixture.effects.length} wit=1 sync=1 async=1 mutation=1 passed`);
    if (!segments.includes("--native")) return;
    const laws = await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, env: { ...process.env, CARGO_BUILD_JOBS: "1", RUST_MIN_STACK: "33554432" }, nativeEnv: { RUST_MIN_STACK: "268435456" }, groups: [
        { package: "semio-framework", target: { kind: "lib" }, laws: ["kernel::service_operation_tests::installed_owner_service_effects_match_the_portable_and_serde_oracles"] },
        { package: "semio-framework-plugin-host", target: { kind: "lib" }, laws: [
          "component::service_operation_tests::service_operation_preserves_its_exact_owner_action_and_payload",
          "component::service_operation_tests::service_operation_rejects_a_malformed_payload",
          "component::imports::effect_conversion_tests::service_operation_preserves_its_exact_owner_action_and_payload",
          "component::imports::effect_conversion_tests::service_operation_rejects_a_malformed_payload",
        ] },
      ], progress: (event) => console.log(`service-operation ${event.stage} ${event.package} ${event.law ?? ""}`) });
    console.log(`plugin-host-service-operation-native: groups=${laws.length} laws=${laws.reduce((count, group) => count + group.laws.length, 0)} passed`);
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
  
  
  for (const row of fixture.cases) {
    const actual = row.code === fixture.code && row.retryable === true && row.events.length <= 1 && row.events.every((event: string) => ["open", "close", "ack"].includes(event));
    assert.equal(actual, row.eligible, row.id);
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
  assert(shard.includes("retryable_lifecycle_turn(&fault, events)"));
  assert(shard.includes("self.has_lifecycle_retry()"), "retry must permit one primed ingress frame");
  return fixture.traces.length;
}

/** 🎠️ Checks failed activation ownership with a separate declarative validator. */
function activationOwnershipOracle(): number {
  const root = join(import.meta.dir, "..", "..", "🎠️activation");
  const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures", "🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  const validateStage = moduleExportValidator(ajv, join(root, "🧬️schema", "🔣️.json"), "ActivationStage");
  
  
  for (const row of fixture.cases) {
    assert(validateStage(row.stage), JSON.stringify(validateStage.errors));
    const actual = { actorRetained: row.stage === "complete", instantiations: ["instantiate", "register", "complete"].includes(row.stage) ? 1 : 0, drops: row.stage === "register" ? 1 : 0, admitted: row.stage === "complete" };
    const expected = { actorRetained: row.actorRetained, instantiations: row.instantiations, drops: row.drops, admitted: row.admitted };
    assert.deepEqual(actual, expected, row.id);

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
    const receipts = await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 86_400_000, lawMilliseconds: 60_000 }, cwd: this.root, env: { ...process.env, RUST_MIN_STACK: "33554432", CARGO_BUILD_JOBS: "1" }, groups: [
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
      ], progress(event) {
        console.log(`guest-fault ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      } });
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
    
    
    assert.equal(new Set(fixture.operationKinds).size, 11);
    for (const row of fixture.cases) {
      const count = row.emitted + row.returned;
      const accepted = !(row.emitted > 0 && row.returned > 0) && count <= fixture.maximumPatches && row.receipt === (count > 0);
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
    const receipts = await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 86_400_000, lawMilliseconds: 60_000 }, cwd: this.root, env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432", CARGO_BUILD_JOBS: "1" }, nativeEnv: { RUST_MIN_STACK: "268435456" }, groups: [
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
      ], progress(event) {
        console.log(`ui-patch-marshalling ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      } });
    console.log(`ui-patch-marshalling-receipts: ${JSON.stringify(receipts)}`);
  }
}

class RouterEffectSourceCustodyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    assert(segments.every((segment) => segment === "--native"), "router-effect-source-custody accepts only --native");
    const owner = join(import.meta.dir, "..", "..", "⚡️effects");
    const law = JSON.parse(readFileSync(join(owner, "🧫️fixtures", "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(owner, "🧬️schema", "🔣️.json"), "utf8"));
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    assert(validate(law), JSON.stringify(validate.errors));
    for (const axis of Object.keys(law.grant)) {
      const denied = structuredClone(law);
      delete denied.grant[axis];
      assert(!validate(denied), `missing original ${axis} authority`);
    }
    assert(law.driveGrant.maximumCapacityBytes > law.grant.maximumCapacityBytes);
    assert(law.identityPolicy.maximumBytes > law.identityPolicy.grant.maximumCapacityBytes);
    const identityOwner = join(owner, "..", "🧵️shard", "🪪️identity");
    const identityPolicy = JSON.parse(readFileSync(join(identityOwner, "⚙️configuration", "🔣️.json"), "utf8"));
    const identitySchema = JSON.parse(readFileSync(join(identityOwner, "🧬️schema", "🔣️.json"), "utf8"));
    const identityValidate = new Ajv({ strict: true, allErrors: true }).compile(identitySchema);
    assert(identityValidate(identityPolicy), JSON.stringify(identityValidate.errors));
    assert.deepEqual(identityPolicy, law.identityPolicy);
    for (const axis of Object.keys(identityPolicy.grant)) {
      const denied = structuredClone(identityPolicy);
      delete denied.grant[axis];
      assert(!identityValidate(denied), `missing original actor ${axis} authority`);
    }
    for (const row of law.sources) {
      const bytes = new TextEncoder().encode(row.text);
      const oracle = Buffer.from(row.text, "utf8");
      assert.deepEqual([...bytes], [...oracle]);
      assert(row.capacity > bytes.length);
      assert.equal(new TextDecoder("utf8", { fatal: true }).decode(bytes), row.text);
      assert.equal(JSON.parse(JSON.stringify(row.text)), row.text);
      assert(row.capacity <= law.grant.maximumReleaseBytes);
    }
    await Parser.init();
    const parser = new Parser();
    parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", this.repoRoot)), "out/tree-sitter-rust.wasm")));
    try {
      for (const relative of ["🦀️.rs", "🧪️tests/🔬️unit/🦀️.rs", "../🦀️.rs", "../🧵️shard/🦀️.rs", "../🧵️shard/🪪️identity/🦀️.rs", "../🧵️shard/🪪️identity/🧪️tests/🦀️.rs", "../🧵️shard/🔁️lifecycle/🦀️.rs", "../🧵️shard/🧵️executor/🦀️.rs"]) {
        const tree = parser.parse(readFileSync(join(owner, relative), "utf8"));
        if (relative === "../🦀️.rs") {
          assert(tree);
          for (const name of ["GuestRelayWakeAuthority", "guest_relay_lifecycle_probe_session", "exercise_abandoned_relay_lifecycle_trace", "exercise_live_relay_lifecycle_trace", "exercise_stale_relay_lifecycle_trace", "test_relay_wake_authority", "run_job_on_worker", "apply_emit_ops"]) {
            const node = tree.rootNode.descendantsOfType(["function_item", "struct_item"]).find((node) => node.childForFieldName("name")?.text === name);
            assert(node && !node.hasError(), `original main Host owning span syntax: ${name}`);
            const original = parser.parse(node.text);
            assert(original && !original.rootNode.hasError(), `original main Host owning span parse: ${name}`);
            original.delete();
          }
          tree.delete();
          continue;
        }
        if (relative === "../🧵️shard/🦀️.rs") {
          assert(tree);
          const nodes = tree.rootNode.descendantsOfType(["struct_item", "impl_item", "function_item"]).filter((node) => node.childForFieldName("type")?.text === "ShardLoop" || ["ShardLoop", "test_identity_issuer"].includes(node.childForFieldName("name")?.text ?? ""));
          assert(nodes.length >= 3, "original ShardLoop custody definitions are present");
          for (const node of nodes) {
            assert(!node.hasError(), "original ShardLoop custody syntax");
            const original = parser.parse(node.text);
            assert(original && !original.rootNode.hasError(), "original ShardLoop custody parse");
            original.delete();
          }
          tree.delete();
          continue;
        }
        if (tree?.rootNode.hasError()) {
          const failures: { line: number; kind: string; text: string }[] = [];
          const pending = [tree.rootNode];
          while (pending.length && failures.length < 8) {
            const node = pending.pop()!;
            if (node.type === "ERROR" || node.isMissing()) failures.push({ line: node.startPosition.row + 1, kind: node.type, text: node.text.slice(0, 200) });
            else if (node.hasError()) pending.push(...node.children.toReversed());
          }
          console.log("[DEBUG] original router Rust parse refusal", relative, failures);
        }
        assert(tree && !tree.rootNode.hasError(), `original router Rust syntax: ${relative}`);
        tree.delete();
      }
    } finally { parser.delete(); }
    console.log(`[DEBUG] router effect source custody: strict schema, five independent grants, UTF8 Buffer oracle, ${law.sources.length} original capacities, Rust completeOwningFiles=6 mainHostOwningSpans=8 originalShardLoopDefinitions=true wholeAsyncClosureOwnersRustcPending=true, independent actor policy, native pending`);
    if (!segments.includes("--native")) return;
    await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 86_400_000, lawMilliseconds: 60_000 }, cwd: this.repoRoot, groups: [{ package: "semio-framework-plugin-host", target: { kind: "lib" }, laws: ["component::effects::tests::router_effect_original_sources_keep_capacity_until_funded_close", "component::effects::tests::router_effect_original_box_frame_has_a_separate_funded_terminal_turn", "component::effects::tests::router_effect_recording_leases_keep_unique_shared_and_weak_backing_custody", "component::shard::identity::tests::shard_original_identity_loans_preserve_partial_children_and_actor_ledgers", "component::shard::identity::tests::shard_original_identity_slot_requires_each_original_positive_axis"] }], progress: (event) => console.log(`router-effect-source-custody ${event.stage} ${event.law ?? ""}`) });
  }
}

class SqliteObservationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("sqlite-observation-check has a fixed native observation contract");
    await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [{ package: "semio-framework-plugin-host", target: { kind: "lib" }, laws: ["component::shared_wasmtime_engine_tests::sqlite_observation_uses_the_host_pool_clock_and_cancellation_without_an_external_reactor"] }], progress: (event) => console.log(`sqlite-observation ${event.stage} ${event.law ?? ""}`) });
  }
}

class OwnedInstanceCheckScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw Error("owned-instance-check has a fixed native fixture contract");
    await runCargo(["test","--manifest-path","Cargo.toml","-p","semio-framework-plugin-host","--lib","owned_instance_open_tests","--","--nocapture"],this.repoRoot);
  }
}

class CountComponentCheckScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw Error("count-component-check has an exact three-law contract");
    if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("count-component-check requires the task artifact root");
    await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), lawMilliseconds: 300000 }, cwd:this.repoRoot, groups:[{package:"semio-framework-plugin-host",target:{kind:"lib"},laws:[
      "component::owned_instance_open_tests::count_component_tests::count_component_full_i32_both_native_encodings_use_real_wasm_and_independent_sqlite",
      "component::owned_instance_open_tests::count_component_tests::count_component_cancellation_occurs_during_real_export_and_import_interpretation",
      "component::owned_instance_open_tests::count_component_tests::count_component_selected_compiled_refusal_owners_preserve_all_eight_causes_and_full_nul_diagnostics",
    ]}], progress:event=>console.log(`count-component ${event.stage} ${event.law??""}`) });
  }
}

/** 🏦️ Runs the exact genuine finite-driver law cohort through the repository compiler authority. */
class OriginalDriverPolicyScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-original-driver-policy-native accepts no arguments");
  const names=["original_host_actor_and_wake_returns_share_one_finite_treasury_and_exact_checkout","original_host_driver_treasury_never_renews_and_rejects_receipts_outside_the_independent_admission_policy","original_host_production_driver_keeps_same_finite_treasury_across_all_wake_returns","original_host_context_loan_preserves_same_recipient_and_returns_cumulative_wake_once","original_host_wake_ledger_keeps_actual_turn_exclusion_all_currencies_and_explicit_driver_return"];
  const receipts=await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), lawMilliseconds: 120000 }, cwd:this.repoRoot, groups:[{package:"semio-framework-plugin-host",target:{kind:"lib"},laws:names.map(name=>`component::original_wake_receipts::tests::${name}`)}], progress:event=>console.log(`[DEBUG] original Host driver ${event.stage} ${event.law??""}`) });
  console.log(`[DEBUG] original Host driver exact groups=${receipts.length} laws=${names.length}`);
 }
}

const router = new ScriptRouter(import.meta.dir)
  .register("check", CheckScript)
  .register("test", TestScript)
  .register("test-original-driver-policy-native", OriginalDriverPolicyScript)
  .register("owned-instance-check", OwnedInstanceCheckScript)
  .register("count-component-check", CountComponentCheckScript)
  .register("sqlite-observation-check", SqliteObservationCheckScript)
  .register("router-effect-source-custody", RouterEffectSourceCustodyScript)
  .register("service-operation-conversion-check", ServiceOperationConversionCheckScript)
  .register("lifecycle-check", LifecycleCheckScript)
  .register("guest-fault-check", GuestFaultCheckScript)
  .register("ui-patch-marshalling-check", UiPatchMarshallingCheckScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "check" }) }));
//#endregion 🎯️Tasks
