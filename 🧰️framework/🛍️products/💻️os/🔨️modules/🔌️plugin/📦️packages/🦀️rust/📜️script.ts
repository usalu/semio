#!/usr/bin/env bun
import { createPluginRunnerTests } from "../../🧪️tests/🏃️runner-self-tests/🟦️.ts";
/** 🦀️ Awaited plugin SDK checks and exact-filter native regression tests. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo, runCargoTestBudgeted, runExactCargoLaws } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
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

/** 🪪️ Independent admission oracle: structural AJV validation and separately decoded owner grammar. */
export function artifactAdmissionOracle(repoRoot?: string): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🏗️builder/🧪️tests/🪪️artifact-admission/🧪️fixture/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🏗️builder/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validate = ajv.getSchema(`${schema.$id}#/$defs/ArtifactAdmissionV1`)!;
  assert(validate(fixture), JSON.stringify(validate.errors));
  const canonical = ajv.compile({ type: "string", pattern: "^s\\.[a-z0-9]+(?:-[a-z0-9]+)*\\.[a-z0-9]+(?:-[a-z0-9]+)*$" });
  const segment = (value: string) => value.length > 0 && !value.startsWith("-") && !value.endsWith("-") && !value.includes("--") && [...value].every((char) => "abcdefghijklmnopqrstuvwxyz0123456789-".includes(char));
  assert.equal(new Set(fixture.cases.map((row: { id: string }) => row.id)).size, fixture.cases.length);
  for (const row of fixture.cases) {
    const parts = row.kind.split(".");
    const valid = parts.length === 3 && parts[0] === "s" && parts.slice(1).every(segment);
    assert.equal(canonical(row.kind), valid, row.id);
    assert.equal(row.package, `semio:${row.plugin}`, row.id);
    const code = !valid ? "plugin-assembly.artifact-kind" : parts[1] !== row.plugin ? "plugin-assembly.artifact-owner" : "accepted";
    assert.equal(code, row.code, row.id);
  }
  if (repoRoot) {
    assert.equal(new Set(fixture.firstParty.map((row: { kind: string }) => row.kind)).size, 39);
    for (const row of fixture.firstParty) {
      assert.equal(row.kind.split(".")[1], row.plugin);
      const definition = readFileSync(resolve(repoRoot, row.definition), "utf8");
      const root = readFileSync(resolve(repoRoot, row.root), "utf8");
      assert(definition.includes(`ArtifactDefinition::new(ArtifactIdentity::parse("${row.kind}")`), row.definition);
      assert(root.includes(`.package_id("semio:${row.plugin}")`), row.root);
      const roots = [...definition.matchAll(/ArtifactDefinition::new\(ArtifactIdentity::parse\("([^"]+)"/g)].map((match) => match[1]);
      assert.deepEqual(roots, [row.kind]);
      const capabilityIds = [...definition.matchAll(/^\s*\("(s\.[^"]+)",\s*"(?:standard|profile|schema|inference|grammar|codec|localization|resource|representation|mutation)"/gm)].map((match) => match[1]!);
      assert(
        capabilityIds.every((kind) => kind.startsWith(`${row.kind}.`)),
        row.definition,
      );
    }
  }
  return fixture.cases.length;
}

/** ♻️ Independently models completion admission and pins every migrated caller to its retained close owner. */
export function completionRejectionOracle(repoRoot?: string): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧪️tests/⏳️completion/🧪️fixture/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧪️tests/⏳️completion/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const law = fixture.ownerReturningRejection;
  assert.equal(validate({ ...fixture, ownerReturningRejection: { ...law, noImplicitRetry: false } }), false, "schema must reject implicit completion retries");
  assert.equal(validate({ ...fixture, ownerReturningRejection: { ...law, cases: law.cases.slice(1) } }), false, "schema must retain all admission states");
  assert.equal(validate({ ...fixture, ownerReturningRejection: { ...law, reservedCallers: law.reservedCallers.slice(1) } }), false, "schema must retain every reserved Puzzle5d caller");
  assert.equal(new Set(law.cases.map((row: { id: string }) => row.id)).size, law.cases.length);
  for (const row of law.cases) {
    const rejected = row.busy || row.cell !== "empty";
    const outcome = rejected ? "rejected" : "accepted";
    const finalCell = rejected ? row.cell : "submitted";
    assert.equal(outcome, row.outcome, row.id);
    assert.equal(finalCell, row.finalCell, row.id);
    assert.equal(rejected, row.submittedOwnerReturned, row.id);
  }
  if (repoRoot) {
    assert.equal(new Set(law.callers.map((row: { family: string }) => row.family)).size, law.callers.length);
    for (const row of law.callers) {
      const source = readFileSync(resolve(repoRoot, row.source), "utf8").split("#[cfg(test)]", 1)[0]!;
      const retain = source.indexOf("self.pending_completion_rejection = Some(rejected)");
      const guard = source.indexOf(row.terminalGuard);
      const close = source.indexOf("emit.close_child_one(maximum_items, maximum_bytes)");
      assert(retain >= 0, `${row.family} loses the returned completion owner`);
      assert(guard >= 0, `${row.family} can replay after terminal completion rejection`);
      assert(close > retain, `${row.family} lacks child-first rejection retirement`);
      assert.equal(source.includes(row.legacyLossToken), false, `${row.family} retained the lossy is_err handoff`);
    }
    assert.equal(new Set(law.reservedCallers.map((row: { family: string }) => row.family)).size, law.reservedCallers.length);
    for (const row of law.reservedCallers) {
      const source = readFileSync(resolve(repoRoot, row.source), "utf8");
      const region = source.split(row.sourceStart, 2)[1]?.split(row.sourceEnd, 1)[0];
      assert(region, `${row.family} source region is absent`);
      const guard = region.indexOf(row.terminalGuard);
      const prepare = region.indexOf("commit.prepare");
      const retain = region.indexOf(row.retainedOwner);
      const close = region.indexOf(row.incrementalClose);
      assert(guard >= 0 && guard < prepare, `${row.family} can replay after a rejected completion`);
      assert(prepare >= 0 && prepare < retain, `${row.family} does not retain the exact returned owner`);
      assert(close > retain, `${row.family} does not retire the returned owner incrementally`);
      assert.equal(region.includes(row.legacyLossToken), false, `${row.family} retained the lossy completion handoff`);
    }
  }
  return law.cases.length + law.callers.length + law.reservedCallers.length;
}

//#region 🎯️Tasks
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    console.log(`[DEBUG] plugin-runner-oracle cases=${pluginTestRunnerSelfTests()}`);
    console.log(`artifact-admission-oracle cases=${artifactAdmissionOracle(this.repoRoot)} firstParty=39`);
    console.log(`completion-rejection-oracle assertions=${completionRejectionOracle(this.repoRoot)}`);
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

/** 🪪️ Validates neutral admission laws with independent AJV predicates and pins the real reducer. */
export function guestLifecycleOracle(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../⚛️reactor/🚪️lifetime/🧫️fixture/🧵️production.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../⚛️reactor/🚪️lifetime/🧬️schema/🧵️production.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  const validate = ajv.compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const admit = ajv.compile({ type: "object", required: ["exact", "live", "capacity"], properties: { exact: { const: true }, live: { const: true }, capacity: { const: true } } });
  for (const row of fixture.cases) {
    assert.equal(row.exact && row.live && row.capacity, row.accepted, row.id);
    assert.equal(admit(row), row.accepted, row.id);
  }
  const reactor = readFileSync(new URL("../../⚛️reactor/🦀️.rs", import.meta.url), "utf8");
  assert(reactor.includes("pub use turn::poll_kernel;"), "production reducer must be native-testable");
  const turn = readFileSync(new URL("../../⚛️reactor/🔄️turn/🦀️.rs", import.meta.url), "utf8");
  assert(!turn.includes("plugin_destroy_app(runtime"), "exact captured close must not admit twice");
  for (const token of ["guest_lifetimes", "stage_ack", "finish_turn", "record_close_admission"]) assert(turn.includes(token), token);
  const owner = readFileSync(new URL("../../⚛️reactor/🚪️lifetime/🦀️.rs", import.meta.url), "utf8");
  for (const token of ["PluginInstanceCloseLease<PA>", "lease.is_retired()", "release_reactor_close", "GuestLifecycleCell<NativeLifetimeOwner<PA>>"]) assert(owner.includes(token), token);
  return fixture.cases.length;
}

/** 🧾️ Cross-checks exact issued patch acknowledgement against independent AJV constants. */
export function issuedPatchOracle(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../⚛️reactor/📨️pending/🧫️fixture/🩹️receipt.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../⚛️reactor/📨️pending/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validateReceipt = ajv.getSchema(`${schema.$id}#/$defs/PendingPatchReceiptV1`)!;
  assert(validateReceipt(fixture), JSON.stringify(validateReceipt.errors));
  const accept = ajv.compile({ type: "object", required: ["ack", "committed", "pending", "live"], properties: { ack: { const: fixture.issued }, committed: { const: true }, pending: { const: true }, live: { const: true } } });
  for (const row of fixture.cases) {
    const exact = Object.keys(fixture.issued).every((key) => fixture.issued[key] === row.ack[key]);
    assert.equal(exact && row.committed && row.pending && row.live, row.accepted, row.id);
    assert.equal(accept(row), row.accepted, row.id);
  }
  const pending = readFileSync(new URL("../../⚛️reactor/📨️pending/🦀️.rs", import.meta.url), "utf8");
  for (const token of ["IssuedPatchAck", "stage_emission", "commit_emission", "apply_issued_ack", "apply_issued_rejection"]) assert(pending.includes(token), token);
  return fixture.cases.length;
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

async function coldDocumentPairIngressOracle(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(new URL("../../⚛️reactor/📥️cold-pair/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../⚛️reactor/📥️cold-pair/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(validate({ ...fixture, hostile: fixture.hostile.slice(1) }), false, "cold pair corpus must retain every hostile row");
  const pattern = (length: number, row: { multiplier: number; addend: number }) => Uint8Array.from({ length }, (_, index) => (index * row.multiplier + row.addend) & 255);
  const pack = pattern(fixture.exact.packLength, fixture.exact.packPattern);
  const spr = pattern(fixture.exact.sprLength, fixture.exact.sprPattern);
  const pairs = [[pack, fixture.exact.packSha256], [spr, fixture.exact.sprSha256], [Buffer.concat([pack, spr]), fixture.exact.aggregateSha256]] as const;
  for (const [bytes, expected] of pairs) {
    assert.equal(createHash("sha256").update(bytes).digest("hex"), expected);
    assert.equal(Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex"), expected);
  }
  const kernel = readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🎠️kernel/📥️cold-pair/🦀️.rs"), "utf8");
  const ingress = readFileSync(new URL("../../⚛️reactor/📥️cold-pair/🦀️.rs", import.meta.url), "utf8");
  const actorFixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🧪️fixture/🔣️.json"), "utf8"));
  const actorSchema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🧬️schema/🔣️.json"), "utf8"));
  const validateActorStatus = new Ajv({ strict: true, allErrors: true }).compile(actorSchema);
  for (const row of actorFixture.statusRows) assert(validateActorStatus(row), JSON.stringify(validateActorStatus.errors));
  for (const row of [actorFixture.hostileRows[0], actorFixture.hostileRows[1], actorFixture.hostileRows[4]]) assert.equal(validateActorStatus(row), false, "actor cold status structural hostile must fail AJV");
  const actorCold = readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🦀️.rs"), "utf8");
  for (const marker of ["COLD_PAIR_PAGE_MAXIMUM_BYTES", "COLD_PAIR_MAXIMUM_BYTES", "COLD_PAIR_MAXIMUM_PAGES", "ColdDocumentPairHeader", "ColdPairIngressStatus"]) assert(kernel.includes(marker), marker);
  for (const marker of ["ColdDocumentPairIngressRegistry", "cold-pair.not-live", "cold-pair.slot-collision", "try_reserve_exact", "reserved_bytes", "live != Some(header.lifetime)", "preflight_close_instance", "advance_close_one", "close_step", "files.spr[start..].fill(0)", "bounded terminal close before teardown"]) assert(ingress.includes(marker), marker);
  for (const marker of ["ColdDocumentPairFrontier", "ColdDocumentPairCursor", "ColdDocumentPairApplied", "InvalidColdPair", "COLD_PAIR_FAULT_MAXIMUM_BYTES"]) assert(actorCold.includes(marker), marker);
  return fixture.hostile.length + actorFixture.hostileRows.length;
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

function documentBackboneBindingOracle(repoRoot: string): number {
  const fixture = JSON.parse(readFileSync(new URL("../../📡️backbone/🔗️binding/🧪️fixture/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../📡️backbone/🔗️binding/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const reduce = (row: any) => {
    const generation = BigInt(row.initial.generation),
      commandGeneration = BigInt(row.command.bindingGeneration),
      currentUri = row.initial.uri,
      operation = row.command.operation;
    if (operation === "bind") {
      if (currentUri !== null && generation === commandGeneration && currentUri === row.command.uri) return { operation: "bound", code: undefined, generation, uri: currentUri };
      if (currentUri !== null && generation === commandGeneration) return { operation: "refused", code: "plugin.document-backbone.binding-collision", generation, uri: currentUri };
      if (currentUri !== null && commandGeneration > generation) return { operation: "refused", code: "plugin.document-backbone.binding-live", generation, uri: currentUri };
      if (currentUri === null && commandGeneration > generation) return { operation: "bound", code: undefined, generation: commandGeneration, uri: row.command.uri };
      return { operation: "refused", code: "plugin.document-backbone.stale-generation", generation, uri: currentUri };
    }
    if (currentUri !== null && generation === commandGeneration && currentUri === row.command.uri) return { operation: "retired", code: undefined, generation, uri: null };
    if (currentUri !== null && generation === commandGeneration) return { operation: "refused", code: "plugin.document-backbone.binding-collision", generation, uri: currentUri };
    return { operation: "refused", code: "plugin.document-backbone.stale-generation", generation, uri: currentUri };
  };
  for (const row of fixture.cases) {
    const outcome = reduce(row);
    assert.equal(outcome.operation, row.receipt.operation, row.id);
    assert.equal(outcome.code, row.receipt.code, row.id);
    assert.equal(outcome.generation.toString(), row.final.generation, row.id);
    assert.equal(outcome.uri, row.final.uri, row.id);
  }
  for (const row of fixture.hostile) assert.equal(validate({ ...fixture, cases: [{ ...fixture.cases[0], command: row.value }] }), false, row.id);
  const binding = readFileSync(new URL("../../📡️backbone/🔗️binding/🦀️.rs", import.meta.url), "utf8");
  const batchFixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/📡️replication/🔗️causal/🧫️fixtures/🧮️document-backbone-batch-v1/🔣️.json"), "utf8"));
  const batchSchema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/📡️replication/🔗️causal/🧬️schema/🧮️document-backbone-batch-v1/🔣️.json"), "utf8"));
  const validateBatch = new Ajv({ strict: true, allErrors: true }).compile(batchSchema);
  assert(validateBatch(batchFixture), JSON.stringify(validateBatch.errors));
  const store = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"), "utf8");
  const causal = readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs"), "utf8");
  const sync = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs"), "utf8");
  const component = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  const reactor = readFileSync(new URL("../../⚛️reactor/🔄️turn/🦀️.rs", import.meta.url), "utf8");
  for (const marker of ["DocumentBackboneBindingStateV1", "binding-noncanonical", "binding-live", "stale-generation"]) assert(binding.includes(marker), marker);
  for (const marker of ["ActorBackboneChannelOwner", "attach_hot_backbone", "decode_hot_backbone_message_exact", "hot backbone transport refuses snapshots"]) assert(store.includes(marker), marker);
  for (const marker of ["decode_document_backbone_envelopes_exact_with_limits", "nonminimal-varint", "DOCUMENT_BACKBONE_PENDING_MAXIMUM_BYTES"]) assert(causal.includes(marker), marker);
  for (const marker of ["ArtifactActorMsg::DocumentBackbone", "ArtifactEvent::DocumentBackbone", "DocumentBackboneRetentionV1", "decode_document_backbone_message_exact"]) assert(sync.includes(marker), marker);
  for (const marker of ["plugin_handle_document_backbone_binding", "plugin_receive_document_backbone", "plugin_drain_document_backbones"]) assert(reactor.includes(marker), marker);
  const retireBranch = component.slice(component.indexOf("DocumentBackboneBindingDecisionV1::Retire(receipt)"), component.indexOf("pub async fn plugin_receive_document_backbone"));
  assert(retireBranch.indexOf("owner.begin_retire()") < retireBranch.indexOf("plugin_detach_backbone(runtime, command.instance_id).await"), "retire must synchronously close ingress before detach awaits");
  assert(!retireBranch.includes("document_backbone_effects"), "retire must discard stale data before emitting its sole receipt");
  assert.deepEqual(fixture.dataLimits, { hotMessageBytes: 262144, snapshotMessageBytes: 4194304, pendingBytes: 1048576, pendingMessages: 64, snapshotTransport: "cold-pair" });
  assert.deepEqual(batchFixture.retention, { maximumBytes: 1048576, maximumMessages: 64 });
  return fixture.cases.length + fixture.codec.golden.length + fixture.codec.hostile.length + fixture.hostile.length + batchFixture.cases.length;
}

class DocumentBackboneBindingCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    assert(segments.every((segment) => segment === "--native"), "document-backbone-binding-check accepts only --native");
    const rows = documentBackboneBindingOracle(this.repoRoot);
    console.log(`document-backbone-binding-oracle: ajv=1 rows=${rows} hot=256KiB snapshot=4MiB pending=64/1MiB`);
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

const router = new ScriptRouter(import.meta.dir)
  .register("document-backbone-binding-check", DocumentBackboneBindingCheckScript)
  .register("cold-document-pair-ingress-check", ColdDocumentPairIngressCheckScript)
  .register("guest-lifecycle-check", GuestLifecycleCheckScript)
  .register("check", CheckScript)
  .register("test", TestScript)
  .register("test-codec-send-source", CodecSendSourceScript)
  .register("artifact-admission-check", ArtifactAdmissionCheckScript);
const createPluginRunnerTestsInstance = createPluginRunnerTests({ Ajv, assert, parseArgs, pluginTestInvocation, readFileSync }, { directory: import.meta.dir, url: import.meta.url });
export const pluginTestRunnerSelfTests = createPluginRunnerTestsInstance.pluginTestRunnerSelfTests;


if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
//#endregion 🎯️Tasks
