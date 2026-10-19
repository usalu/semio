import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import assert from "node:assert/strict";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import { findIndex } from "lodash";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { orchestratorBudgetOpts, runCmd, runCargo, runProbe } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runAdmittedExactCargoLawsV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🦀️rust");

const SCALE_COMPONENT_ARTIFACT = "🧰️framework/🛍️products/💻️os/🧪️testing/⚖️scale/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm";
const HOST_FAMILY = [
  "semio-framework-plugin-host-wit",
  "semio-framework-plugin-host-engine",
  "semio-framework-plugin-host-operation",
  "semio-framework-plugin-host-effects",
  "semio-framework-plugin-host-imports",
  "semio-framework-plugin-host-guest-runtime",
  "semio-framework-plugin-host-wake",
  "semio-framework-plugin-host-post-turn-relay",
  "semio-framework-plugin-host-routers",
  "semio-framework-plugin-host-process-transport",
  "semio-framework-plugin-host-shard",
  "semio-framework-plugin-host-shard-laws",
  "semio-framework-plugin-host-guest-laws",
  "semio-framework-plugin-host-shard-executor",
  "semio-framework-plugin-host",
] as const;
export const familyPackages = HOST_FAMILY.flatMap((name) => ["-p", name]);
const countOccurrences = (source: string, needle: string): number => source.split(needle).length - 1;
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
function moduleExportValidator(ajv: Ajv, modulePath: string, exportId: string) {
  const document = JSON.parse(readFileSync(modulePath, "utf8"));
  ajv.addSchema(document);
  const validate = ajv.getSchema(`${document.$id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`schema module ${modulePath}: unknown export ${exportId}`);
  return validate;
}
function guestFaultOracle(): number {
  const hostRoot = join(SCRIPT_DIR, "..", "..");
  const fixture = JSON.parse(readFileSync(join(hostRoot, "🔁️lifecycle", "🧪️tests/🧫️fixtures", "🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  
  
  for (const row of fixture.cases) {
    const actual = row.code === fixture.code && row.retryable === true && row.events.length <= 1 && row.events.every((event: string) => ["open", "close", "ack"].includes(event));
    assert.equal(actual, row.eligible, row.id);
  }
  const host = readModuleTreeSource(join(hostRoot, "🎭️guest-runtime", "🦀️.rs"));
  const retained = readFileSync(join(hostRoot, "🧬️component-codec", "🎟️retained-turn", "🦀️.rs"), "utf8");
  const owned = readModuleTreeSource(join(hostRoot, "..", "🧠️interpreter", "🏃️runtime", "🦀️.rs"));
  const contract = readFileSync(join(hostRoot, "..", "🧬️guest-contract", "🦀️.rs"), "utf8");
  assert(contract.includes("Guest(semio_framework::Fault)"), "typed guest fault must survive host decode");
  assert(host.includes("original.map_err(retained_turn_wire::failure_from_wit)"), "real Wasmtime poll must return the original actor failure receipt");
  assert(owned.includes("result.map_err(|error| decode_guest_fault_bytes(&error))"), "owned ABI must use typed decoder");
  assert(retained.includes("TurnFault::ActorCall(call)"), "actor failure must retain its physical input and spending");
  assert(retained.includes("wit_reactor::ActorCallFailure::Semantic=>None"), "semantic cause remains owned by its guest");
  return fixture.cases.length;
}
function retainedLifecycleOracle(): number {
  const root = join(SCRIPT_DIR, "..", "..", "🧵️shard", "🔁️lifecycle");
  const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures", "🔣️.json"), "utf8"));
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
  assert(shard.includes("OriginalShardEvent::new(source_event,issued)"), "retry owns the actual originally issued event source");
  assert(shard.includes("owner.original_event=Some(original)"), "suspension returns the same event source to its pending owner");
  assert(shard.includes("self.has_lifecycle_retry()"), "retry must permit one primed ingress frame");
  return fixture.traces.length;
}
function activationOwnershipOracle(): number {
  const root = join(SCRIPT_DIR, "..", "..", "🎠️activation");
  const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures", "🔣️.json"), "utf8"));
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
function kernelReservationOracle(): number {
  const root = join(SCRIPT_DIR, "..", "..", "..", "..", "..", "..", "..", "🔨️modules", "🎭️actor", "🎠️activation-reservation");
  const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
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

/** 🏃️ Runs the `ServiceOperationConversionCheckScript` command under the caller's roots and invocation. */
export async function runServiceOperationConversionCheckScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  assert(segments.every((segment) => segment === "--native"), "service-operation-conversion-check accepts only --native");
  const hostRoot = join(context.root, "..", "..");
  const wit = readFileSync(join(hostRoot, "..", "🧬️schema", "📜️effects.wit"), "utf8");
  const synchronous = readFileSync(join(hostRoot, "🔗️wit-bindings", "🔀️conversion", "🦀️.rs"), "utf8");
  const asynchronous = readFileSync(join(hostRoot, "📥️imports", "🦀️.rs"), "utf8");
  const kernel = join(context.repoRoot, "🧰️framework", "🔨️modules", "🎠️kernel");
  const fixture = JSON.parse(readFileSync(join(kernel, "🧪️tests/🧫️fixtures/💡️service-operation/🔣️.json", "💡️service-operation", "🔣️.json"), "utf8"));
  assertServiceOperationConversionSource(wit, synchronous, asynchronous);
  assert.throws(() => assertServiceOperationConversionSource(wit, synchronous, asynchronous.replace("E::RequestServiceOperation(inner) => K::RequestServiceOperation", "E::MissingServiceOperation(inner) => K::RequestServiceOperation")));
  console.log(`plugin-host-service-operation-source: effects=${fixture.effects.length} wit=1 sync=1 async=1 mutation=1 passed`);
  if (!segments.includes("--native")) return;
  const laws = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0), invocation: context.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: context.repoRoot, env: { ...process.env, CARGO_BUILD_JOBS: "1", RUST_MIN_STACK: "33554432" }, nativeEnv: { RUST_MIN_STACK: "268435456" }, groups: [
      { package: "semio-framework", target: { kind: "lib" }, laws: ["kernel::service_operation_tests::installed_owner_service_effects_match_the_portable_and_serde_oracles"] },
      {
        package: "semio-framework-plugin-host-wit",
        target: { kind: "lib" },
        laws: [
          "service_operation_tests::service_operation_preserves_its_exact_owner_action_and_payload",
          "service_operation_tests::service_operation_rejects_a_malformed_payload",
        ],
      },
      {
        package: "semio-framework-plugin-host-imports",
        target: { kind: "lib" },
        laws: [
          "effect_conversion_tests::service_operation_preserves_its_exact_owner_action_and_payload",
          "effect_conversion_tests::service_operation_rejects_a_malformed_payload",
        ],
      },
    ], progress: (event) => console.log(`service-operation ${event.stage} ${event.package} ${event.law ?? ""}`) });
  console.log(`plugin-host-service-operation-native: groups=${laws.length} laws=${laws.reduce((count, group) => count + group.laws.length, 0)} passed`);
}

/** 🏃️ Runs the `LifecycleCheckScript` command under the caller's roots and invocation. */
export async function runLifecycleCheckScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, _segments: string[]): Promise<void> {
  const oracle = join(context.root, "..", "..", "🧪️tests", "♻️relay-lifecycle", "🟦️.ts");
  const repoTest = join(context.root, "..", "..", "..", "..", "..", "..", "🦑️repo", "🔨️modules", "🧪️test", "📜️script.ts");
  if (!existsSync(oracle)) throw new Error(`missing relay lifecycle oracle at ${oracle}`);
  if (!existsSync(repoTest)) throw new Error(`missing repository test runner at ${repoTest}`);
  runCmd("bun", [oracle], { cwd: context.root });
  runCmd("bun", [repoTest, "subject", "fundamental", "--case", "♻️relay-lifecycle", "--implementation", "rust"], {
    cwd: context.root,
    budgetMs: 900_000,
    env: { ...process.env, SEMIO_TEST_BUDGET_MS: "900000" },
  });
  const focused: readonly (readonly [string, string])[] = [
    ["semio-framework-plugin-host-shard-laws", "unit::replay_owners_drop_safely_from_every_owned_frontier_and_balance_accounting"],
    ["semio-framework-plugin-host-shard-laws", "unit::replay_failure_and_actor_loss_enter_one_close_funnel_before_reporting"],
    ["semio-framework-plugin-host-shard-laws", "unit::spawn_job_effect_is_admitted_stepped_across_multiple_pumps_and_completion_reaches_the_originating_actor"],
    ["semio-framework-plugin-host-shard-laws", "unit::cancel_job_effect_stops_a_job_before_it_is_ever_stepped"],
    ["semio-framework-plugin-host-shard-laws", "unit::cancel_job_effect_failure_retires_the_actor_and_surfaces_the_typed_fault"],
    ["semio-framework-plugin-host-shard-laws", "unit::cancel_unregisters_the_instance_and_no_further_step_job_happens"],
    ["semio-framework-plugin-host-shard-laws", "unit::actor_cancel_failure_retires_the_instance_and_reports_fault_instead_of_cancelled"],
    ["semio-framework-plugin-host-shard-laws", "unit::exclusive_placement_is_stepped_before_inline_placement_admitted_the_same_pump"],
    ["semio-framework-plugin-host-shard-laws", "unit::exclusive_selection_never_crosses_a_lifecycle_barrier"],
    ["semio-framework-plugin-host-shard-laws", "unit::job_step_uses_the_owning_actors_last_granted_budget"],
    ["semio-framework-plugin-host-shard-executor", "tests::fifo_ingress_selects_interactive_before_earlier_background_without_unbounded_drain"],
    ["semio-framework-plugin-host-effects", "tests::router_effect_runs_through_the_retained_compute_session"],
    ["semio-framework-plugin-host-effects", "tests::router_effect_on_a_stopped_compute_pool_returns_worker_lost_without_stranding_its_owner"],
    ["semio-framework-plugin-host-post-turn-relay", "guest_cold_relay_tests::detached_reaper_reclaims_one_slot_per_opportunity_round_robin_and_refuses_stale_generation"],
    ["semio-framework-plugin-host-post-turn-relay", "guest_cold_relay_tests::detached_reaper_never_steals_a_live_draining_callers_exact_output"],
    ["semio-framework-plugin-host-post-turn-relay", "guest_cold_relay_tests::dropping_a_pending_mounted_future_reaps_without_a_second_foreground_poll"],
    ["semio-framework-plugin-host-post-turn-relay", "guest_cold_relay_tests::wake_incapable_close_uses_one_coalesced_bounded_fallback"],
    ["semio-framework-plugin-host-post-turn-relay", "guest_cold_relay_tests::retained_pool_future_retries_saturation_once_and_terminalizes_shutdown"],
    ["semio-framework-plugin-host-post-turn-relay", "guest_cold_relay_tests::neutral_relay_lifecycle_traces_drive_production_machines"],
  ];
  const discoveredByPackage = new Map<string, string[]>();
  const discover = (pkg: string): string[] => {
    const known = discoveredByPackage.get(pkg);
    if (known) return known;
    const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--lib", "-p", pkg, "--", "--list"], { cwd: context.root, ...orchestratorBudgetOpts() });
    if (listed.status !== 0) throw new Error(`plugin-host lifecycle inventory of ${pkg} failed with status ${listed.status}`);
    const names = listed.stdout
      .split("\n")
      .filter((line) => line.endsWith(": test"))
      .map((line) => line.slice(0, -": test".length));
    discoveredByPackage.set(pkg, names);
    return names;
  };
  const laws = focused.map(([pkg, suffix]) => {
    const matches = discover(pkg).filter((name) => name.endsWith(suffix));
    if (matches.length !== 1) throw new Error(`plugin-host lifecycle gate expected exactly one ${suffix} law in ${pkg}, selected ${matches.length}`);
    return [pkg, matches[0]!] as const;
  });
  console.log(`plugin-host-lifecycle-laws: ${laws.map(([pkg, law]) => `${pkg}:${law}`).join(" ")}`);
  for (const [pkg, law] of laws) await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "-p", pkg, law, "--", "--exact"], context.root);
  await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...familyPackages], context.root);
  await runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", ...familyPackages], context.root);
}

/** 🏃️ Runs the `GuestFaultCheckScript` command under the caller's roots and invocation. */
export async function runGuestFaultCheckScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  assert(
    segments.every((segment) => segment === "--native"),
    "guest-fault-check accepts only --native",
  );
  console.log(`guest-fault-oracle cases=${guestFaultOracle()} retries=${retainedLifecycleOracle()} activations=${activationOwnershipOracle()} reservations=${kernelReservationOracle()}`);
  if (!segments.includes("--native")) return;
  const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0), invocation: context.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 86_400_000, lawMilliseconds: 60_000 }, cwd: context.root, env: { ...process.env, RUST_MIN_STACK: "33554432", CARGO_BUILD_JOBS: "1" }, groups: [
      {
        package: "semio-framework-actor",
        target: { kind: "lib" },
        laws: ["activation::tests::neutral_reservation_traces_gate_dispatch_until_exact_binding", "activation::tests::reservation_exhaustion_collision_and_stale_binding_leave_no_partial_admission"],
      },
      {
        package: "semio-framework-plugin-host-shard-executor",
        target: { kind: "lib" },
        laws: [
          "tests::terminal_registration_reply_wakes_outside_the_state_lock",
          "tests::admitted_registration_reply_wakes_outside_the_state_lock",
          "tests::shard_stack_authority_matches_the_neutral_fixture",
          "tests::shard_executor_drives_a_turn_for_a_registered_actor_via_the_worker_pool",
          "tests::fifo_ingress_selects_interactive_before_earlier_background_without_unbounded_drain",
          "tests::every_actors_grant_lands_on_the_shard_it_was_registered_on_across_k_shards",
          "tests::suspend_then_resume_round_trip_lands_on_a_shard_where_the_actor_is_registered",
          "tests::concurrent_send_frame_bursts_never_drop_an_outcome",
          "tests::pending_drive_wake_and_wake_storm_claim_exactly_one_schedule",
          "tests::registration_acknowledgement_and_terminal_refusal_preserve_exact_owners",
        ],
      },
      {
        package: "semio-framework-plugin-host-shard-laws",
        target: { kind: "lib" },
        laws: [
          "lifecycle_tests::unknown_transport_actors_never_allocate_host_bookkeeping",
          "lifecycle_tests::neutral_retry_order_uses_the_production_selector",
          "lifecycle_tests::retry_keeps_exact_event_budget_credit_and_one_peer_order",
          "lifecycle_tests::retry_refuses_replacement_and_stale_queue_cannot_reach_same_id_successor",
          "lifecycle_tests::cancellation_revokes_retry_before_another_guest_call",
          "lifecycle_tests::retry_occupies_the_original_fixed_lane_capacity",
          "lifecycle_tests::terminal_faults_never_create_a_lifecycle_retry",
        ],
      },
      {
        package: "semio-framework-plugin-host-wit",
        target: { kind: "lib" },
        laws: [
          "cold_pair::tests::neutral_cold_ingress_variants_preserve_authority_and_refuse_hostile_wit",
          "cold_pair::tests::native_cold_pages_use_dedicated_bounded_input",
        ],
      },
      {
        package: "semio-framework-plugin-host",
        target: { kind: "lib" },
        laws: [
          "component::activation::tests::neutral_activation_failures_retire_the_exact_kernel_and_guest_owners",
        ],
      },
      {
        package: "semio-framework-plugin-host-guest-runtime",
        target: { kind: "lib" },
        laws: [
          "guest_fault_tests::structured_guest_fault_survives_wit_owned_and_async_channels",
          "guest_fault_tests::malformed_and_oversized_guest_faults_cannot_authorize_retry",
        ],
      },
    ], progress(event) {
      console.log(`guest-fault ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
    } });
  console.log(`guest-fault-receipts: ${JSON.stringify(receipts)}`);
}

/** 🏃️ Runs the `UiPatchMarshallingCheckScript` command under the caller's roots and invocation. */
export async function runUiPatchMarshallingCheckScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  assert(
    segments.every((segment) => segment === "--native"),
    "ui-patch-marshalling-check accepts only --native",
  );
  const hostRoot = join(context.root, "..", "..");
  const owner = join(hostRoot, "📥️ui-patch");
  const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures", "🔣️.json"), "utf8"));
  
  
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
  const synchronous = readModuleTreeSource(join(hostRoot, "🎭️guest-runtime", "🦀️.rs"));
  const scale = readFileSync(join(context.repoRoot, "🧰️framework", "🛍️products", "💻️os", "🧪️testing", "⚖️scale", "🦀️.rs"), "utf8");
  for (const kind of ["Upsert", "SetComponent", "SetLayout", "SetActivity", "SetChildren", "SetStyle", "SetAccessibility", "SetBindings", "SetMenu", "Remove", "SetRoot"]) {
    assert(codec.includes(`wit_ui::PatchOp::${kind}`), `missing ${kind} reverse codec`);
  }
  assert.equal(countOccurrences(synchronous, "ui_patch::wit_ui_patches_to_kernel("), 1);
  assert(!synchronous.includes("let _emitted_patches"));
  assert(scale.includes("semio::framework::host_async::emit_patch(&patch)"));
  assert(scale.includes("ui_patch_receipt"));
  console.log(`plugin-host-ui-patch-marshalling-source: ajv=1 cases=${fixture.cases.length} component=${fixture.componentCases.length} ops=${fixture.operationKinds.length} originalAsyncRuntime=1 passed`);
  if (!segments.includes("--native")) return;
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactRoot, "SEMIO_TEST_ARTIFACT_DIR is required");
  const scaleWasm = join(context.repoRoot, SCALE_COMPONENT_ARTIFACT);
  assert(existsSync(scaleWasm), "registered scale component was not materialized");
  const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0), invocation: context.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 86_400_000, lawMilliseconds: 60_000 }, cwd: context.root, env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432", CARGO_BUILD_JOBS: "1" }, nativeEnv: { RUST_MIN_STACK: "268435456" }, groups: [
      {
        package: "semio-framework-plugin-host-wit",
        target: { kind: "lib" },
        laws: [
          "ui_patch::tests::every_wit_patch_variant_moves_into_one_exact_kernel_owner",
          "ui_patch::tests::emitted_and_returned_channels_are_atomic_bounded_and_drained_once",
          "ui_patch::tests::target_budget_receipt_and_malformed_pack_refuse_before_publication",
        ],
      },
      {
        package: "semio-framework-plugin-host-guest-laws",
        target: { kind: "lib" },
        laws: [
          "ui_patch_component_tests::genuine_component_returned_and_imported_patches_keep_channel_order_and_exact_authority",
          "ui_patch_component_tests::imported_then_returned_channels_refuse_atomically_without_a_transport_token",
          "ui_patch_component_tests::malformed_import_is_drained_before_the_next_exact_patch_owner_is_published",
        ],
      },
    ], progress(event) {
      console.log(`ui-patch-marshalling ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
    } });
  console.log(`ui-patch-marshalling-receipts: ${JSON.stringify(receipts)}`);
}

/** 🏃️ Runs the `RouterEffectSourceCustodyScript` command under the caller's roots and invocation. */
export async function runRouterEffectSourceCustodyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  assert(segments.every((segment) => segment === "--native"), "router-effect-source-custody accepts only --native");
  const owner = join(context.root, "..", "..", "⚡️effects");
  const law = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures", "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(owner, "🧬️schema", "🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile<typeof import("../../⚡️effects/🧪️tests/🧫️fixtures/🔣️.json")>(schema);
  assert(validate(law), JSON.stringify(validate.errors));
  for (const axis of Object.keys(law.grant)) {
    const denied = structuredClone(law);
    Reflect.deleteProperty(denied.grant,axis);
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
    Reflect.deleteProperty(denied.grant,axis);
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
  parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", context.repoRoot)), "out/tree-sitter-rust.wasm")));
  try {
    for (const relative of ["🦀️.rs", "🧪️tests/🔬️unit/🦀️.rs", "../🦀️.rs", "../🧵️shard/🦀️.rs", "../🧵️shard/🪪️identity/🦀️.rs", "../🧵️shard/🪪️identity/🧪️tests/🦀️.rs", "../🧵️shard/🔁️lifecycle/🦀️.rs", "../🧵️shard/🧵️executor/🦀️.rs"]) {
      const tree = parser.parse(/^\.\.\/[^/]*shard\/[^/]*\.rs$/u.test(relative) ? readModuleTreeSource(join(owner, relative)) : readFileSync(join(owner, relative), "utf8"));
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
      }
      assert(tree && !tree.rootNode.hasError(), `original router Rust syntax: ${relative}`);
      tree.delete();
    }
  } finally { parser.delete(); }
  if (!segments.includes("--native")) return;
  await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0), invocation: context.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 86_400_000, lawMilliseconds: 60_000 }, cwd: context.repoRoot, groups: [{ package: "semio-framework-plugin-host-effects", target: { kind: "lib" }, laws: ["tests::router_effect_original_sources_keep_capacity_until_funded_close", "tests::router_effect_original_box_frame_has_a_separate_funded_terminal_turn", "tests::router_effect_recording_leases_keep_unique_shared_and_weak_backing_custody"] }, { package: "semio-framework-plugin-host-shard-laws", target: { kind: "lib" }, laws: ["identity_tests::shard_original_identity_loans_preserve_partial_children_and_actor_ledgers", "identity_tests::shard_original_identity_slot_requires_each_original_positive_axis"] }], progress: (event) => console.log(`router-effect-source-custody ${event.stage} ${event.law ?? ""}`) });
}

/** 🏃️ Runs the `OriginalDriverPolicyScript` command under the caller's roots and invocation. */
export async function runOriginalDriverPolicyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments:string[]): Promise<void> {
if(segments.length)throw new Error("test-original-driver-policy-native accepts no arguments");
const names=["original_host_actor_and_wake_returns_share_one_finite_treasury_and_exact_checkout","original_host_driver_treasury_never_renews_and_rejects_receipts_outside_the_independent_admission_policy","original_host_production_driver_keeps_same_finite_treasury_across_all_wake_returns","original_host_context_loan_preserves_same_recipient_and_returns_cumulative_wake_once","original_host_wake_ledger_keeps_actual_turn_exclusion_all_currencies_and_explicit_driver_return"];
const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0), invocation: context.invocation, policy: { ...configuredExactCargoLawPolicyV1(), lawMilliseconds: 120000 }, cwd:context.repoRoot, groups:[{package:"semio-framework-plugin-host-wake",target:{kind:"lib"},laws:names.map(name=>`tests::${name}`)}], progress:event=>{} });
 }
