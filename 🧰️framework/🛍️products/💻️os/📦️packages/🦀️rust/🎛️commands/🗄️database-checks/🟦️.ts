/** 🗄️ Database commands: paged history stack, history completion, catalog read ownership, capability completion and shutdown checks.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { exactCargoStageEnvironments } from "../🧱️script-bases/🟦️.ts";

/** ↔️ Admits portable paged traversal and independently checks array/deque ordering. */
export class PagedHistoryStackScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length > 1 || (args.length && args[0] !== "--native")) throw Error("paged-history-stack-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/📸️paged-history-stack/🔣️.json"), "utf8"));
    
    
    
    for (const vector of fixture.vectors) {
      const values = Array.from({ length: vector.pushes }, (_, i) => String(i));
      if (vector.removeLogical !== null) values.splice(vector.removeLogical, 1);
      if (vector.replacement !== null) values.push(vector.replacement);
      assert.deepEqual(values, vector.forward);
      assert.deepEqual([...values].reverse(), vector.reverse);
      assert.deepEqual(vector.directions.map((direction: string) => (direction === "front" ? values.shift() : values.pop()) ?? null), vector.expected);
      assert.deepEqual(values, vector.remaining);
    }
    const editSchema = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧬️schema/🔣️.json"), "utf8"));
    const branchFixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🌿️branch-provenance/🔣️.json"), "utf8"));
    const branchAjv = semioSchemaAjvV1({ strict: true, allErrors: true });
    const editCheck = branchAjv.compile({ $defs: editSchema.$defs, $ref: "#/$defs/Edit" });
    for (const vector of branchFixture.vectors) {
      assert.equal(editCheck(vector.edit), vector.valid, vector.id);
      assert.equal(Object.hasOwn(vector.edit, "line") && (vector.edit.line === null || typeof vector.edit.line === "string"), vector.valid, vector.id);
    }
    const { testCanonicalEditFixtures } = await import("../../../../🔨️modules/🏪️store/🧪️tests/🧵️canonical-edit/🟦️.ts");
    const { storeCanonicalEditSealerSelfTests } = await import("../../../../🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts");
    testCanonicalEditFixtures();
    const canonical = storeCanonicalEditSealerSelfTests();
    console.log("paged-history-canonical-oracles: " + JSON.stringify(canonical));
    if (args[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 3_600_000, listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib", name: "semio_framework_os_kernel" }, laws: [
          "os_vcs::tests::paged_history_stack_traversal_follows_the_portable_deque_vectors",
          "os_vcs::tests::history_branch_provenance_follows_portable_required_wire_vectors",
        ] }, { package: "semio-framework-os-store-tests-09", target: { kind: "lib" }, laws: [
          "edit_digest_chains_match_the_neutral_vectors_and_extend_incrementally",
          "canonical_authority_final_unicode_strings_retire_under_single_byte_grants",
        ] }]});
      for (const receipt of receipts) console.log("paged-history-stack-native-receipt: " + JSON.stringify(receipt));
    }
    console.log(`paged-history-stack-check: vectors=${fixture.vectors.length} independent-array/Ajv=passed`);
  }
}

/** 📜️ Verifies history-result publication and exact replay-owner retirement. */
export class DatabaseHistoryCompletionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-history-completion-check accepts only --native");
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🧫️fixtures/📜️history-completion");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    
    
    assert.deepEqual(fixture.publication.map((row: { name: string }) => row.name), ["before-poll", "before-registration", "after-pending"].flatMap(stage => ["success", "cancelled"].map(outcome => stage + "-" + outcome)));
    const { Database } = await import("bun:sqlite");
    const oracle = new Database(":memory:");
    try {
      oracle.run("CREATE TABLE handoff (payload TEXT, location TEXT, waiter INTEGER, wakes INTEGER, admission INTEGER, registry INTEGER)");
      for (const row of fixture.publication) {
        oracle.run("DELETE FROM handoff");
        const payload = row.outcome === "success" ? fixture.operations : { error: "Closed" };
        oracle.run("INSERT INTO handoff VALUES (?1, 'producer', 0, 0, 1, 1)", [JSON.stringify(payload)]);
        const snapshot = () => oracle.query("SELECT payload, location, waiter, wakes, admission, registry FROM handoff").get() as { payload: string; location: string; waiter: number; wakes: number; admission: number; registry: number };
        const publish = () => oracle.run("UPDATE handoff SET location = 'completion', wakes = wakes + waiter, waiter = 0 WHERE location = 'producer'");
        if (row.publication === "before-poll") publish();
        let firstReady = snapshot().location === "completion";
        if (!firstReady) {
          if (row.publication === "before-registration") publish();
          oracle.run("UPDATE handoff SET waiter = 1");
          firstReady = snapshot().location === "completion";
        }
        assert.equal(firstReady, row.firstReady, row.name);
        if (!firstReady) publish();
        assert.equal(oracle.run("UPDATE handoff SET location = 'consumer', waiter = 0 WHERE location = 'completion'").changes, 1, row.name);
        assert.equal(JSON.stringify(JSON.parse(snapshot().payload)) === JSON.stringify(payload), row.exactResult, row.name);
        assert.equal(snapshot().waiter === 0, row.waiterEmpty, row.name);
        assert.equal(snapshot().wakes, row.wakes, row.name);
        assert.equal(snapshot().waiter === 0, row.wakeLockReleased, row.name);
        oracle.run("UPDATE handoff SET location = 'empty', admission = 0, registry = 0 WHERE location = 'consumer'");
        assert.equal(snapshot().admission === 0, row.admissionReleased, row.name);
        assert.equal(snapshot().registry === 0, row.registryEmpty, row.name);
      }
    } finally {
      oracle.close();
    }
    console.log("database-history-completion-check: behavior-oracles=retained sqlite-publication=" + fixture.publication.length);
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [{
        package: "semio-framework-os-kernel-db",
        target: { kind: "lib", name: "db" },
        cargoArgs: ["--all-features"],
        laws: [
          "artifact_history_completion_interleavings_preserve_result_and_wake",
          "artifact_history_empty_and_two_batch_replay_are_deterministic",
          "artifact_history_empty_one_cap_plus_one_admission_returns_exact_request",
          "artifact_history_cancel_before_handoff_retires_full_reservation_before_credit_release",
          "artifact_history_public_terminal_close_releases_admission_only_after_roots_are_empty",
        ],
      }],progress(event) {
        console.log("database-history-completion-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);
      }});
    for (const receipt of receipts) console.log("database-history-completion-native-receipt: " + JSON.stringify(receipt));
  }
}

/** 📖️ Verifies catalog-root ownership independently of scalar capability opening. */
export class DatabaseCatalogReadOwnershipCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-catalog-read-ownership-check accepts only --native");
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🧫️fixtures/📖️catalog-read-ownership");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    
    
    assert.deepEqual(fixture.transfers.map((row: { phase: string }) => row.phase), ["Handoff", "RetainWork", "Poll"]);
    assert.deepEqual(fixture.completion.map((row: { name: string }) => row.name), ["synchronous", "before-wake", "after-finalizer-check", "refused-finalizer"].flatMap(stage => ["pages", "backend-fault"].map(outcome => stage + "-" + outcome)));
    assert.deepEqual(fixture.recovery.map((row: { name: string }) => row.name), ["retry", "terminal-completion", "terminal-result", "spent-work"].flatMap(path => ["pages", "backend-fault"].map(outcome => path + "-" + outcome)));
    const { Database } = await import("bun:sqlite");
    const oracle = new Database(":memory:");
    try {
      oracle.run("CREATE TABLE ownership (active INTEGER, location TEXT, admission INTEGER, waiter INTEGER, releases INTEGER, payload TEXT)");
      const reset = (active: number, location: string, payload: unknown) => {
        oracle.run("DELETE FROM ownership");
        oracle.run("INSERT INTO ownership VALUES (?1, ?2, 1, 1, 0, ?3)", [active, location, JSON.stringify(payload)]);
      };
      const finalize = () => oracle.run("UPDATE ownership SET active = 4, admission = 0, releases = releases + 1 WHERE active = 0 AND location = 'empty' AND waiter = 0 AND admission = 1");
      for (const row of fixture.transfers) {
        reset(1, "stack", fixture.root);
        const close = oracle.run("UPDATE ownership SET admission = 0 WHERE active = 0 AND location = 'empty'").changes;
        assert.equal(close === 0, row.closeBlocked, row.name);
        const paused = oracle.query("SELECT admission, location FROM ownership").get() as { admission: number; location: string };
        assert.equal(paused.admission === 1, row.admissionRetained, row.name);
        assert.equal(paused.location === "stack", row.storageRetained, row.name);
        const submissions = oracle.run("UPDATE ownership SET active = 2 WHERE active = 0").changes;
        assert.equal(submissions, row.submissionsWhileActive, row.name);
        oracle.run("UPDATE ownership SET active = 0, location = 'empty', waiter = 0");
        finalize();
        assert.equal((oracle.query("SELECT admission = 0 AS empty FROM ownership").get() as { empty: number }).empty === 1, row.finalEmpty, row.name);
      }
      for (const row of fixture.completion) {
        const payload = row.outcome === "pages" ? fixture.root : { key: fixture.root.key, error: "fixture-root-fault" };
        reset(row.publication === "synchronous" ? 0 : 1, "completion", payload);
        assert.deepEqual(JSON.parse((oracle.query("SELECT payload FROM ownership").get() as { payload: string }).payload), payload, row.name);
        oracle.run("UPDATE ownership SET location = 'empty', waiter = 0");
        finalize();
        assert.equal((oracle.query("SELECT admission FROM ownership").get() as { admission: number }).admission === 1, row.admissionDuringPublication, row.name);
        const retirement = row.publication === "after-finalizer-check" || row.publication === "refused-finalizer";
        const submissions = retirement ? oracle.run("UPDATE ownership SET active = 2 WHERE active = 1").changes : 0;
        assert.equal(submissions, row.retirementSubmissions, row.name);
        oracle.run("UPDATE ownership SET active = 0 WHERE active IN (1, 2)");
        finalize();
        assert.equal(oracle.run("UPDATE ownership SET active = 2 WHERE active = 0").changes, row.lateSubmissions, row.name);
        const final = oracle.query("SELECT admission = 0 AND releases = 1 AS empty FROM ownership").get() as { empty: number };
        assert.equal(final.empty === 1, row.terminalEmpty, row.name);
      }
      oracle.run("CREATE TABLE recovery (generation INTEGER, checked_out INTEGER, admission INTEGER, deliveries INTEGER, payload TEXT)");
      for (const row of fixture.recovery) {
        oracle.run("DELETE FROM recovery");
        const payload = row.outcome === "pages" ? fixture.root : { key: fixture.root.key, error: "fixture-root-fault" };
        oracle.run("INSERT INTO recovery VALUES (1, 0, 1, 0, ?1)", [JSON.stringify(payload)]);
        if (row.path === "retry") {
          oracle.run("UPDATE recovery SET generation = generation + 1");
          assert.equal(oracle.run("UPDATE recovery SET deliveries = deliveries + 1 WHERE generation = 1").changes, row.staleRetrySubmissions, row.name);
        }
        if (row.path === "terminal-result" || row.path === "spent-work") {
          oracle.run("UPDATE recovery SET checked_out = 1");
          assert.equal(oracle.run("UPDATE recovery SET admission = 0 WHERE checked_out = 0").changes === 0, row.checkoutBlocksRetirement, row.name);
          oracle.run("UPDATE recovery SET checked_out = 0");
        } else {
          assert.equal(row.checkoutBlocksRetirement, false, row.name);
        }
        oracle.run("UPDATE recovery SET deliveries = deliveries + 1, admission = 0 WHERE checked_out = 0");
        const actual = oracle.query("SELECT deliveries, admission, payload FROM recovery").get() as { deliveries: number; admission: number; payload: string };
        assert.equal(actual.deliveries, 1, row.name);
        assert.deepEqual(JSON.parse(actual.payload), payload, row.name);
        assert.equal(actual.admission === 0, row.terminalEmpty, row.name);
      }
    } finally {
      oracle.close();
    }
    console.log("database-catalog-read-ownership-check: behavior-oracles=retained sqlite-transfers=" + fixture.transfers.length + " sqlite-completion=" + fixture.completion.length + " sqlite-recovery=" + fixture.recovery.length);
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [{
        package: "semio-framework-os-kernel-db",
        target: { kind: "lib", name: "db" },
        cargoArgs: ["--all-features"],
        laws: [
          "database_catalog_read_paused_transfers_exclude_successors_and_public_cleanup",
          "database_catalog_read_consumed_publication_preserves_exact_root_and_retires",
          "database_catalog_read_retry_and_terminal_resume_preserve_exact_root",
          "database_catalog_read_fixed_cap_plus_one_and_generation_aba",
          "database_catalog_read_success_returns_exact_storage_key_and_root",
          "database_catalog_read_controlled_wakes_coalesce_and_terminal_never_repolls",
          "database_catalog_read_publication_between_check_and_waker_registration_is_observed",
          "database_catalog_read_rejected_mount_retires_storage_and_key_on_distinct_grants",
          "database_catalog_read_cancel_stale_and_rejection_preserve_exact_storage_key",
          "database_catalog_read_terminal_result_drop_hands_back_exact_result",
        ],
      }],progress(event) {
        console.log("database-catalog-read-ownership-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);
      }});
    for (const receipt of receipts) console.log("database-catalog-read-ownership-native-receipt: " + JSON.stringify(receipt));
  }
}

/** 📬️ Verifies retained capability completion handoff at every public waiter boundary. */
export class DatabaseCapabilityCompletionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-capability-completion-check accepts only --native");
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🧫️fixtures/📬️capability-completion");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    
    
    assert.deepEqual(
      fixture.publication.map((row: { name: string }) => row.name),
      ["before-poll", "before-registration", "after-pending"].flatMap((stage) => ["success", "fault"].map((outcome) => stage + "-" + outcome)),
    );
    assert.deepEqual(
      fixture.retirement.map((row: { name: string }) => row.name),
      ["consumed-before-wake-success", "consumed-before-wake-fault"],
    );
    assert.deepEqual(fixture.driveOwnership.map((row: { phase: string }) => row.phase), ["Handoff", "RetainWork", "Poll"]);
    assert.deepEqual(fixture.leaseCompletion.map((row: { publication: string }) => row.publication), ["synchronous", "active-publisher", "after-finalizer-check", "after-finalizer-check-refused"]);
    const { Database } = await import("bun:sqlite");
    const oracle = new Database(":memory:");
    try {
      oracle.run("CREATE TABLE handoff (id INTEGER PRIMARY KEY, completion TEXT, waiter INTEGER NOT NULL, wakes INTEGER NOT NULL, admission INTEGER NOT NULL)");
      const snapshot = () => oracle.query("SELECT completion, waiter, wakes, admission FROM handoff WHERE id = 1").get() as { completion: string | null; waiter: number; wakes: number; admission: number };
      const reset = () => {
        oracle.run("DELETE FROM handoff");
        oracle.run("INSERT INTO handoff VALUES (1, NULL, 0, 0, 1)");
      };
      const publish = (outcome: string) => oracle.run("UPDATE handoff SET completion = ?1, wakes = wakes + waiter, waiter = 0 WHERE id = 1", [outcome]);
      const consume = () => oracle.run("UPDATE handoff SET completion = NULL, waiter = 0, admission = 0 WHERE id = 1 AND completion IS NOT NULL");
      for (const row of fixture.publication) {
        reset();
        if (row.publication === "before-poll") publish(row.outcome);
        let firstReady = snapshot().completion !== null;
        if (!firstReady) {
          if (row.publication === "before-registration") publish(row.outcome);
          oracle.run("UPDATE handoff SET waiter = 1 WHERE id = 1");
          firstReady = snapshot().completion !== null;
        }
        assert.equal(firstReady, row.firstReady, row.name);
        if (!firstReady) publish(row.outcome);
        assert.equal(snapshot().completion, row.outcome, row.name);
        consume();
        assert.deepEqual(snapshot(), { completion: null, waiter: 0, wakes: row.wakes, admission: 0 }, row.name);
      }
      for (const row of fixture.retirement) {
        reset();
        oracle.run("UPDATE handoff SET waiter = 1, completion = ?1 WHERE id = 1", [row.outcome]);
        consume();
        assert.equal(snapshot().admission === 0, row.terminalBeforePublisherWake, row.name);
        oracle.run("UPDATE handoff SET wakes = wakes + waiter, waiter = 0 WHERE id = 1");
        assert.equal(snapshot().wakes, row.wakes, row.name);
      }
      oracle.run("CREATE TABLE ownership (active INTEGER NOT NULL, location TEXT NOT NULL, admission INTEGER NOT NULL, releases INTEGER NOT NULL)");
      for (const row of fixture.driveOwnership) {
        oracle.run("DELETE FROM ownership");
        oracle.run("INSERT INTO ownership VALUES (1, 'stack', 1, 0)");
        const close = () => oracle.run("UPDATE ownership SET admission = 0, releases = releases + 1 WHERE active = 0 AND location = 'empty' AND admission = 1").changes;
        assert.equal(close() === 0, row.closeBlocked, row.name);
        const paused = oracle.query("SELECT admission, location FROM ownership").get() as { admission: number; location: string };
        assert.equal(paused.admission === 1, row.admissionRetained, row.name);
        assert.equal(paused.location === "stack", row.storageRetained, row.name);
        oracle.run("UPDATE ownership SET location = 'terminal', active = 0");
        assert.equal(close(), 0, row.name);
        oracle.run("UPDATE ownership SET location = 'empty'");
        assert.equal(close(), 1, row.name);
        assert.equal(close(), 0, row.name);
        assert.equal((oracle.query("SELECT releases FROM ownership").get() as { releases: number }).releases, row.finalReleases, row.name);
      }
      for (const row of fixture.leaseCompletion) {
        oracle.run("DELETE FROM ownership");
        oracle.run("INSERT INTO ownership VALUES (?1, 'completion', 1, 0)", [row.publication === "synchronous" ? 0 : 1]);
        const checkedBeforeConsumption = row.publication.startsWith("after-finalizer-check");
        const readyAtFirstCheck = (oracle.query("SELECT location = 'empty' AS ready FROM ownership").get() as { ready: number }).ready === 1;
        oracle.run("UPDATE ownership SET location = 'empty'");
        const finalize = () => oracle.run("UPDATE ownership SET active = 4, admission = 0, releases = releases + 1 WHERE location = 'empty' AND active = 0 AND admission = 1");
        finalize();
        assert.equal((oracle.query("SELECT admission FROM ownership").get() as { admission: number }).admission === 1, row.admissionDuringPublication, row.name);
        let retirementSubmissions = 0;
        if (checkedBeforeConsumption && !readyAtFirstCheck) {
          oracle.run("UPDATE ownership SET active = active | 2 WHERE active = 1");
          oracle.run("UPDATE ownership SET active = active & ~1 WHERE active = 3");
          retirementSubmissions = (oracle.query("SELECT active = 2 AS queued FROM ownership").get() as { queued: number }).queued;
          oracle.run("UPDATE ownership SET active = 0 WHERE active = 2");
        } else {
          oracle.run("UPDATE ownership SET active = 0 WHERE active = 1");
        }
        assert.equal(retirementSubmissions, row.retirementSubmissions, row.name);
        finalize();
        assert.equal(oracle.run("UPDATE ownership SET active = 2 WHERE active = 0").changes, row.lateSubmissions, row.name);
        assert.equal((oracle.query("SELECT releases FROM ownership").get() as { releases: number }).releases, 1, row.name);
      }
    } finally {
      oracle.close();
    }
    console.log("database-capability-completion-check: behavior-oracles=retained sqlite-publication=" + fixture.publication.length + " sqlite-retirement=" + fixture.retirement.length + " sqlite-drive-ownership=" + fixture.driveOwnership.length + " sqlite-lease-completion=" + fixture.leaseCompletion.length);
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [
        {
          package: "semio-framework-os-kernel-db",
          target: { kind: "lib", name: "db" },
          cargoArgs: ["--all-features"],
          laws: [
            "database_capability_open_paused_transfer_blocks_public_close",
            "database_capability_open_lease_successors_and_active_publication_retire_once",
            "database_capability_open_completion_interleavings_preserve_result_and_wake",
            "database_capability_open_consumed_completion_retires_before_publisher_wake",
            "database_capability_open_fixed_admission_cap_plus_one_and_generation_aba",
            "database_capability_open_success_returns_exact_storage_owner_and_scalar",
            "database_capability_open_cancel_and_stale_generation_retain_exact_owner_for_public_close",
            "database_capability_open_saturation_and_shutdown_keep_retry_job_and_public_terminal",
            "database_capability_open_poll_publication_precedes_wake_rearm_at_every_boundary",
            "database_capability_open_post_ready_cancel_and_stale_retain_public_exact_result",
            "database_capability_open_rejection_take_retry_and_close_preserve_exact_storage",
            "database_capability_open_terminal_result_take_resume_and_checked_out_drop_handback",
            "database_capability_open_retry_contention_is_one_compare_exchange_per_callback",
            "database_catalog_read_publication_between_check_and_waker_registration_is_observed",
            "database_catalog_bootstrap_publication_race_and_queue_pressure_keep_exact_successor",
            "database_create_catalog_publication_check_register_recheck_has_no_lost_wake",
            "open_at_creates_a_fresh_zero_touch_database_with_an_empty_catalog",
          ],
        },
      ],progress(event) {
        console.log("database-capability-completion-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);
      }});
    for (const receipt of receipts) console.log("database-capability-completion-native-receipt: " + JSON.stringify(receipt));
  }
}

/** 🚪️ Proves retained database shutdown keeps exact retry owners across interruption and shared authority. */
export class DatabaseShutdownCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-shutdown-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🚪️shutdown/🔣️.json"), "utf8"));
    
    
    assert.deepEqual(fixture.phases, ["authority", "emit", "complete"]);
    assert.equal(fixture.maximumAuthorityStepsPerTurn, 1);
    for (const row of fixture.cases) {
      let authorityOwners = row.initial.authorityOwners;
      let externalAuthorityOwners = row.initial.externalAuthorityOwners;
      let closing = false;
      let cancelled = false;
      let terminal = false;
      let emitCount = 0;
      let authorityRetained = false;
      for (const action of row.trace) {
        if (action === "cancel") {
          cancelled = true;
          authorityRetained ||= authorityOwners > 0 || closing;
          continue;
        }
        if (action === "retry") {
          cancelled = false;
          continue;
        }
        if (action === "release-shared-owner") {
          externalAuthorityOwners = 0;
          continue;
        }
        if (cancelled || terminal) continue;
        if (authorityOwners > 0) {
          if (externalAuthorityOwners > 0) {
            authorityRetained = true;
          } else if (!closing) {
            closing = true;
          } else {
            closing = false;
            authorityOwners -= 1;
          }
        } else if (emitCount === 0) {
          emitCount = 1;
        } else {
          terminal = true;
        }
      }
      assert.deepEqual({ terminal, authorityRetained, emitCount }, row.expected, row.name);
    }
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const artifact = readFileSync(join(owner, "..", "🗿️artifact", "🦀️.rs"), "utf8");
    assert(source.includes("closing_authority: Option<") && source.includes("pub async fn shutdown_step(&mut self"));
    assert(source.includes("DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Authorities"));
    assert(source.includes("shutdown_emit_started") && !source.includes("shutdown_graph_complete"));
    assert(!source.includes("pub async fn shutdown(self"));
    assert(artifact.includes("pub fn shutdown_step(&self) -> bool") && artifact.includes("handoff.terminal"));
    console.log(`database-shutdown-independent-oracle: behavior-oracles=retained cases=${fixture.cases.length} retained-authority=1 terminal-ack=1`);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, ...exactCargoStageEnvironments(), cargoArgs: ["--all-features"], groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib", name: "db" },
            laws: [
              "db_engine::tests::database_shutdown_cancellation_preserves_exact_retry_owners",
              "db_engine::tests::database_shutdown_shared_authority_blocks_without_closing_live_handle",
            ],
          },
        ], progress(event) {
          console.log(`database-shutdown-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        } });
      console.log(`database-shutdown-native-receipts: ${JSON.stringify(receipts)}`);
    }
  }
}
