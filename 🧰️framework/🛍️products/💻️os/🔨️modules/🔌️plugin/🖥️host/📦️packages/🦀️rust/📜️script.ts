#!/usr/bin/env bun
/** 🖥️ Runs owned plugin-host checks and exact native test filters. */
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

/** 🪪️ Pins typed guest-fault transport and independently checks lifecycle replay eligibility. */
function guestFaultOracle(): number {
  const hostRoot = join(import.meta.dir, "..", "..");
  const fixture = JSON.parse(readFileSync(join(hostRoot, "🔁️lifecycle", "🧫️fixture", "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(hostRoot, "🔁️lifecycle", "🧬️schema.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  const validate = ajv.compile(schema);
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
  const fixture = JSON.parse(readFileSync(join(root, "🧫️fixture", "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(root, "🧬️schema.json"), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
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

class GuestFaultCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    assert(
      segments.every((segment) => segment === "--native"),
      "guest-fault-check accepts only --native",
    );
    console.log(`guest-fault-oracle cases=${guestFaultOracle()} retries=${retainedLifecycleOracle()}`);
    if (!segments.includes("--native")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: "33554432", CARGO_BUILD_JOBS: "1" },
      groups: [
        {
          package: "semio-framework-plugin-host",
          target: { kind: "lib" },
          laws: ["component::guest_fault_tests::structured_guest_fault_survives_wit_owned_and_async_channels", "component::guest_fault_tests::malformed_and_oversized_guest_faults_cannot_authorize_retry"],
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

const router = new ScriptRouter(import.meta.dir)
  .register("check", CheckScript)
  .register("test", TestScript)
  .register("inference-proposal-conversion-check", InferenceProposalConversionCheckScript)
  .register("lifecycle-check", LifecycleCheckScript)
  .register("guest-fault-check", GuestFaultCheckScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
//#endregion 🎯️Tasks
