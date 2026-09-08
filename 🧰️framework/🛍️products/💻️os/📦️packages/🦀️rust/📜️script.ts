#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-os-kernel` task router. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo, resolveTestLevel, runCargoTestBudgeted, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { runNestedCargoPackageAdapter } from "../../../../../📜️script.ts";

function exactCargoStageEnvironments() {
  return {
    env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
    nativeEnv: { RUST_MIN_STACK: "268435456" },
  };
}

/** 📜️ Verifies history-result publication and exact replay-owner retirement. */
class DatabaseHistoryCompletionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-history-completion-check accepts only --native");
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️fixtures/📜️history-completion");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
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
    console.log("[DEBUG] database-history-completion-check: AJV=1 sqlite-publication=" + fixture.publication.length);
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{
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
      }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log("database-history-completion-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);
      },
    });
    for (const receipt of receipts) console.log("database-history-completion-native-receipt: " + JSON.stringify(receipt));
  }
}

/** 📖️ Verifies catalog-root ownership independently of scalar capability opening. */
class DatabaseCatalogReadOwnershipCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-catalog-read-ownership-check accepts only --native");
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️fixtures/📖️catalog-read-ownership");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
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
    console.log("[DEBUG] database-catalog-read-ownership-check: AJV=1 sqlite-transfers=" + fixture.transfers.length + " sqlite-completion=" + fixture.completion.length + " sqlite-recovery=" + fixture.recovery.length);
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{
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
      }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log("database-catalog-read-ownership-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);
      },
    });
    for (const receipt of receipts) console.log("database-catalog-read-ownership-native-receipt: " + JSON.stringify(receipt));
  }
}

/** 📬️ Verifies retained capability completion handoff at every public waiter boundary. */
class DatabaseCapabilityCompletionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-capability-completion-check accepts only --native");
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️fixtures/📬️capability-completion");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
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
    console.log("database-capability-completion-check: AJV=1 sqlite-publication=" + fixture.publication.length + " sqlite-retirement=" + fixture.retirement.length + " sqlite-drive-ownership=" + fixture.driveOwnership.length + " sqlite-lease-completion=" + fixture.leaseCompletion.length);
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [
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
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log("database-capability-completion-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);
      },
    });
    for (const receipt of receipts) console.log("database-capability-completion-native-receipt: " + JSON.stringify(receipt));
  }
}

/** 🔐️ Checks fixed writer capabilities and retained local backend integration. */
class WalWriterAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-writer-authority-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️fixtures/🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧪️fixtures/🧬️.schema.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
    const remoteOwner = join(owner, "🧪️fixtures/🌐️remote-guard");
    const remoteFixture = JSON.parse(readFileSync(join(remoteOwner, "🔣️.json"), "utf8"));
    const validateRemote = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(remoteOwner, "🧬️.schema.json"), "utf8")));
    assert(validateRemote(remoteFixture), JSON.stringify(validateRemote.errors));
    assert.deepEqual(remoteFixture.mutations, ["create", "append", "sync", "seal", "truncateTail", "delete"]);
    const memoryOwner = join(owner, "..", "🧪️fixtures", "🧮️memory-backing");
    const memoryFixture = JSON.parse(readFileSync(join(memoryOwner, "🔣️.json"), "utf8"));
    const validateMemory = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(memoryOwner, "🧬️.schema.json"), "utf8")));
    assert(validateMemory(memoryFixture), JSON.stringify(validateMemory.errors));
    const poolUseOwner = join(owner, "..", "🧪️fixtures", "🔐️backend-pool-use");
    const poolUseFixture = JSON.parse(readFileSync(join(poolUseOwner, "🔣️.json"), "utf8"));
    const validatePoolUse = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(poolUseOwner, "🧬️.schema.json"), "utf8")));
    assert(validatePoolUse(poolUseFixture), JSON.stringify(validatePoolUse.errors));
    assert.deepEqual(
      poolUseFixture.cases.map((row: { name: string }) => row.name),
      [
        "registered-backend-blocks-shutdown",
        "task-derives-registered-pool",
        "dropped-facade-retains-use",
        "compaction-acquires-before-admission",
        "forged-kind-rejected-before-page-admission",
        "rollback-reserved-before-owner-transfer",
        "all-retirement-tiers-full-return-exact-executor",
        "committed-rollback-retains-use-until-terminal",
        "prepared-post-transfer-refusal-returns-close-owner",
      ],
    );
    const { Database } = await import("bun:sqlite");
    const openingOracle = new Database(":memory:");
    try {
      openingOracle.run("CREATE TABLE opening_owner (backend TEXT PRIMARY KEY, retained INTEGER NOT NULL, close_requested INTEGER NOT NULL)");
      for (const row of poolUseFixture.opening) {
        openingOracle.run("INSERT INTO opening_owner VALUES (?1, 1, 0)", [row.backend]);
        openingOracle.run("UPDATE opening_owner SET close_requested = 1 WHERE backend = ?1", [row.backend]);
        const requested = openingOracle.query("SELECT close_requested FROM opening_owner WHERE backend = ?1").get(row.backend) as { close_requested: number };
        assert.equal(!!requested.close_requested, row.closeRequestedBeforeRetry);
        openingOracle.run("DELETE FROM opening_owner WHERE backend = ?1 AND close_requested = 1", [row.backend]);
        const terminal = openingOracle.query("SELECT count(*) AS retained FROM opening_owner").get() as { retained: number };
        assert.equal(terminal.retained === 0, row.ledgerBaseline);
        assert.equal(terminal.retained === 0, row.poolShutdownAfterDrain);
        assert.equal(terminal.retained === 0, row.reopens);
      }
    } finally {
      openingOracle.close();
    }
    assert.deepEqual(memoryFixture.writerTable, { slots: fixture.capacity, separateBox: true });
    assert.deepEqual(memoryFixture.controllerCredit, { items: 1, controls: 1, bytesFormula: "wake-plus-two-usize" });
    assert.equal(memoryFixture.retainedPageResults, 44);
    assert.equal(memoryFixture.sameSlotReuseBeforeOldResultClose, true);
    assert.equal(memoryFixture.taskSlotReleasedWhileResultRetained, true);
    assert.equal(memoryFixture.droppedResultRetirement, "mounted-io-maintenance");
    const directoryOwner = join(owner, "..", "🧪️fixtures", "📁️directory-durability");
    const directoryFixture = JSON.parse(readFileSync(join(directoryOwner, "🔣️.json"), "utf8"));
    const validateDirectory = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(directoryOwner, "🧬️.schema.json"), "utf8")));
    assert(validateDirectory(directoryFixture), JSON.stringify(validateDirectory.errors));
    const openOwner = join(owner, "..", "..", "📝️wal", "🧪️fixtures", "🚪️open-rejection");
    const openFixture = JSON.parse(readFileSync(join(openOwner, "🔣️.json"), "utf8"));
    const validateOpen = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(openOwner, "🧬️.schema.json"), "utf8")));
    assert(validateOpen(openFixture), JSON.stringify(validateOpen.errors));
    let openOwnerState = "exact-writer-permit";
    assert.equal(openFixture.acquisition[1].owner, openOwnerState);
    openOwnerState = "exact-writer-release";
    assert.equal(openFixture.acquisition[2].owner, openOwnerState);
    assert.equal(openFixture.acquisition[2].releaseActivation, "first-explicit-close-poll");
    const exactRelease = Symbol("exact-writer-release");
    let retainedRelease: symbol | undefined = exactRelease;
    for (const [index, close] of openFixture.explicitClose.entries()) {
      assert.equal(close.attempt, index + 1);
      assert.equal(close.openCause, "original-open-error");
      if (close.outcome === "fault") assert.equal(retainedRelease, exactRelease);
      else retainedRelease = undefined;
      assert.equal(close.sameDocumentAcquire, retainedRelease ? "conflict" : "available");
    }
    assert.equal(openFixture.droppedRejection.behavior === "transfer" && !openFixture.droppedRejection.panics ? "backend-release-cell" : "discarded", openFixture.droppedRejection.location);
    const directoryNames = new Set(["segment", "marker"]);
    directoryNames.delete("segment");
    assert.equal(!directoryNames.has("segment") && directoryNames.has("marker") ? "not-found-with-retained-marker" : "invalid", directoryFixture.deleteFaultState);
    directoryNames.delete("marker");
    assert.equal(directoryNames.size, 0);
    assert(directoryFixture.delete.indexOf("segment-delete-parent-synced") < directoryFixture.delete.indexOf("marker-deleted"));
    const pinnedWriter = { operation: fixture.controllerBinding.operation, releaseRequested: true };
    assert.equal(pinnedWriter.releaseRequested && pinnedWriter.operation !== fixture.controllerBinding.contender ? "closed" : "ok", fixture.controllerBinding.fenceBeforeCallback);
    assert.equal(pinnedWriter.operation === fixture.controllerBinding.operation ? "ok" : "closed", fixture.controllerBinding.pinnedResume);
    assert.equal(fixture.controllerBinding.tasksAddedOnRelease, 0);
    assert.equal(fixture.controllerBinding.guardFactoryCallsOnConflict, 0);
    assert.equal(fixture.controllerBinding.faultRetryPreservesKey, true);
    const pendingWriters = new Map([
      ["fault", "retained"],
      ["healthy", "retained"],
    ]);
    const faultTrace = ["fault-retained"];
    pendingWriters.delete("healthy");
    faultTrace.push("healthy-terminal");
    assert.equal(pendingWriters.get("fault"), "retained");
    pendingWriters.delete("fault");
    faultTrace.push("fault-retry-terminal");
    assert.deepEqual(faultTrace, fixture.controllerFaults.coalesced);
    assert.equal(fixture.controllerFaults.outerPanic.executorTurns, 1);
    assert.equal(fixture.controllerFaults.outerPanic.wakesPerOwner, 1);
    const followerOwners = new Set(["document"]);
    assert.equal(followerOwners.has("document") ? "conflict" : "ok", fixture.replication.occupiedFollower);
    assert.deepEqual(fixture.replication.inventoryAfterConflict, []);
    const transfer = fixture.replication.snapshotTransfer;
    const sourceSnapshot = Buffer.concat(Array.from({ length: transfer.repetitions }, () => Buffer.from(transfer.pattern)));
    const copiedSnapshot = Buffer.from(sourceSnapshot);
    assert.equal(sourceSnapshot.length, transfer.bytes);
    sourceSnapshot.fill(0);
    assert.deepEqual(
      [...copiedSnapshot],
      Array.from({ length: transfer.bytes }, (_, index) => transfer.pattern[index % transfer.pattern.length]),
    );
    assert.equal(Symbol("source-result") === Symbol("independent-input"), transfer.sameOperation);
    const clusterSource = readFileSync(join(owner, "..", "..", "🌐️cluster", "🦀️.rs"), "utf8");
    assert(clusterSource.includes("db_io_copy_page_owner(&pages)") && clusterSource.includes("close_replication_pages(&mut pages)"), "snapshot replication must retire source result before a distinct write owner");
    const walSource = readFileSync(join(owner, "..", "..", "📝️wal", "🦀️.rs"), "utf8");
    for (const marker of ["struct ArtifactWalAcquiredRejected", "enum ArtifactWalOpenRejected", "retry_open(", "retry_close(", "open_acquired(", "into_open_rejected"])
      assert(walSource.includes(marker), "missing retained WAL-open owner primitive: " + marker);
    assert(!walSource.includes("release_failed_open"), "WAL open rejection must not await and flatten its writer release");
    const artifactSource = readFileSync(join(owner, "..", "..", "🗿️artifact", "🦀️.rs"), "utf8");
    for (const marker of ["enum ArtifactEngineOpenRejected", "RetainedWal", "has_retained_writer", "Future<Output = Result<ArtifactEngine<A, V>, ArtifactEngineOpenRejected>>"])
      assert(artifactSource.includes(marker), "missing engine retained-open propagation: " + marker);
    const engineSource = readFileSync(join(owner, "..", "..", "⚙️engine", "🦀️.rs"), "utf8");
    for (const marker of ["enum DatabaseDocumentOpenRejected", "Result<ArtifactHandle, DatabaseDocumentOpenRejected>", "rejected.retry_close().await"]) assert(engineSource.includes(marker), "missing database retained-open propagation: " + marker);
    for (const marker of ["enum ReplicationRejected", "ArtifactWal::open_acquired", "ReplicationRejected::WalOpen"]) assert(clusterSource.includes(marker), "missing cluster acquired-writer propagation: " + marker);
    for (const outcome of fixture.replication.releaseAfter) {
      const owner = new Set(["document"]);
      try {
        assert(["tail", "up-to-date", "snapshot", "leader-corrupt"].includes(outcome));
      } finally {
        owner.delete("document");
      }
      assert.equal(owner.size, 0);
    }
    for (const row of fixture.cases) {
      let generation = BigInt(row.firstGeneration);
      const live = new Map<string, bigint>();
      const owners = new Map<string, { document: string; generation: bigint }>();
      for (const step of row.steps) {
        let actual = "ok";
        const permit = owners.get(step.owner);
        if (step.action === "acquire") {
          if (live.has(step.document)) actual = "conflict";
          else if (generation === 0xffffffffffffffffn) actual = "exhausted";
          else {
            live.set(step.document, generation);
            owners.set(step.owner, { document: step.document, generation: generation++ });
          }
        } else if (step.backend !== 0 || permit?.document !== step.document || live.get(step.document) !== permit?.generation) actual = "fenced";
        else if (step.action === "release") live.delete(step.document);
        assert.equal(actual, step.expected, `${row.name}: ${JSON.stringify(step)}`);
      }
      assert.equal(live.size, 0, row.name);
    }
    assert.deepEqual(
      fixture.resultRetirement.terminal,
      Array.from({ length: fixture.resultRetirement.pages + 3 }, (_, index) => index === fixture.resultRetirement.pages + 2),
    );
    assert.deepEqual(
      fixture.guardRetirement.terminal,
      fixture.guardRetirement.stages.map((stage: string) => stage === "terminal"),
    );
    const rejected = new Set(Array.from({ length: fixture.backendPressure.capacity }, (_, index) => index));
    let retained = rejected.size === fixture.backendPressure.capacity;
    assert.equal(retained, fixture.backendPressure.retainedWhenFull);
    rejected.delete(0);
    if (rejected.size < fixture.backendPressure.capacity) retained = false;
    assert.equal(!retained, fixture.backendPressure.terminalAfterCapacityReturns);
    const closing = new Map([
      [fixture.fairRetirement.pinnedSlot, "pinned"],
      [fixture.fairRetirement.releasingSlot, "releasing"],
    ]);
    for (let cursor = 0; cursor < fixture.fairRetirement.maximumOpportunities; cursor++) if (closing.get(cursor) === "releasing") closing.delete(cursor);
    assert.equal(closing.has(fixture.fairRetirement.pinnedSlot), fixture.fairRetirement.pinnedRetained);
    assert.equal(!closing.has(fixture.fairRetirement.releasingSlot), fixture.fairRetirement.releasingRetired);
    for (const trace of [fixture.maintenanceFairness.continuouslyReady, fixture.maintenanceFairness.firstClassFaults])
      assert.deepEqual(
        trace,
        trace.map((_: unknown, index: number) => index % fixture.maintenanceFairness.classes.length),
      );
    let terminalEpoch = BigInt(fixture.releaseSignal.firstEpoch);
    const waits: bigint[] = [];
    for (const writer of fixture.releaseSignal.writers) {
      const required = terminalEpoch + 1n;
      assert.equal(terminalEpoch >= required, fixture.releaseSignal.requestCompletesRelease);
      terminalEpoch += 1n;
      waits.push(required);
      assert.equal(terminalEpoch.toString(), writer.terminalEpoch);
      assert.equal(
        waits.every((wait) => terminalEpoch >= wait),
        fixture.releaseSignal.oldWaitSurvivesReuse,
      );
    }
    console.log(
      `wal-writer-authority-independent-oracle: AJV=6 exact-u64=1 cases=${fixture.cases.length} mutations=${fixture.mutations.length} remote=${remoteFixture.cases.length} writer-slots=${memoryFixture.writerTable.slots} retained-result=1 directory-barriers=4 wal-open-owner=1 backend-pool-use=${poolUseFixture.cases.length} physical-opening=${poolUseFixture.opening.length}`,
    );
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    for (const marker of ["struct WalWriterPermit", "struct WalWriterTable", "struct WalFileWriterGuard", "try_lock()", "checked_add(1)", "active_operation", "fn release_step"])
      assert(source.includes(marker), `missing writer capability primitive: ${marker}`);
    const explicitRelease = source.slice(source.indexOf("pub fn release(mut self)"), source.indexOf("fn request_release(&self)"));
    assert(!explicitRelease.includes("self.request_release()"), "explicit retained release must stay dormant until its first poll");
    const storageSource = readFileSync(join(owner, "..", "🦀️.rs"), "utf8");
    for (const marker of ["WalWriterTable<WalFileWriterGuard>", "fn writer_sidecar", "DbIoTask::WalWriterAcquire", "fn pin_writer_operation", "finish_operation_if_pinned", ".semio-wal-writer"])
      assert(storageSource.includes(marker), `missing filesystem writer integration: ${marker}`);
    for (const marker of [
      "pool_use: Option<Arc<WorkerPoolUse>>",
      "fn db_io_backend_admit_operation",
      "Result<Arc<WorkerPool>, DbError>",
      "pub fn submit_db_io_task(task: DbIoTask)",
      "db_io_backend_control(owner.kind, slot, generation) != control",
      "struct DbIoBackendRollbackReservation",
      "struct DbIoBackendRegistrationRejected",
      "pub enum DbStorageOpenRejected",
      "pub async fn retry_close(self) -> Result<DbError, Self>",
      "register_db_io_backend_prepared_with_use",
      "Result<DbIoBackendControl, DbIoBackendRegistrationRejected>",
      "reserved: bool",
    ])
      assert(storageSource.includes(marker), `missing backend-owned WorkerPool use boundary: ${marker}`);
    assert(!storageSource.includes("pub fn submit_db_io_task(pool:"), "DB I/O tasks must derive the exact registered backend pool");
    const registrationSource = storageSource.slice(storageSource.indexOf("pub fn register_db_io_backend("), storageSource.indexOf("fn db_io_writer_release_lane_step"));
    assert(!registrationSource.includes("let _ = db_io_park_lost_owner"), "backend registration must not discard a saturated retirement owner");
    for (const marker of ["MemoryDbIoExecutor::backing_bytes()", "checked_add(writer::release::controller_credit())"]) assert(storageSource.includes(marker), `missing memory writer backing integration: ${marker}`);
    const sqliteSource = readFileSync(join(owner, "..", "🪶️sqlite", "🦀️.rs"), "utf8");
    for (const marker of ["WalWriterTable<SqliteWalWriterGuard>", "canonical_database", "fn physical_writer_sidecar", "DbIoTask::WalWriterAcquire", "fn pin_writer_operation", "finish_operation_if_pinned", ".semio-wal-writer"])
      assert(sqliteSource.includes(marker), `missing SQLite writer integration: ${marker}`);
    const releaseSource = readFileSync(join(owner, "🔔️release/🦀️.rs"), "utf8");
    for (const marker of [
      "struct WalWriterSignalCell",
      "requested: bool",
      "fn request_release(&mut self)",
      "impl Drop for WalWriterRelease",
      "deferred_fault_waiter",
      "defer_fault_notifications",
      "suspend_controller_for_refusal",
      "deferred_wake_pending_for_test",
    ])
      assert(releaseSource.includes(marker), `missing bounded deferred refusal primitive: ${marker}`);
    assert.equal((storageSource.match(/writer::release::notify_faults/g) ?? []).length, 0, "public DB handback paths must not invoke writer wakers directly");
    assert(storageSource.includes("wal_writer_mounted_stale_controller_defers_cross_key_wake_and_fences_retry_epoch"), "missing mounted stale-controller refusal law");
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [
        {
          package: "semio-framework-os-kernel-db",
          target: { kind: "lib", name: "db" },
          cargoArgs: ["--all-features"],
          laws: [
            "db_io_real_storage_open_drop_retires_queued_backend_and_allows_reopen",
            "db_io_real_storage_open_fault_drop_retires_registered_backend_without_retry",
            "wal_writer_table_matches_neutral_exact_scope_and_aba_rejection",
            "wal_writer_table_capacity_recycles_slots_without_reusing_generations",
            "wal_writer_file_lock_excludes_independent_instances_and_processes",
            "db_io_lost_result_lease_retains_every_page_and_final_handback",
            "wal_writer_release_retains_pinned_operation_and_faulted_guard",
            "db_io_lost_backend_retains_exact_owner_under_rejected_registry_pressure",
            "wal_writer_table_close_advances_other_guards_while_first_operation_is_pinned",
            "db_io_maintenance_rotates_ready_and_faulted_classes_without_starvation",
            "wal_writer_release_signal_preserves_exact_waits_across_writer_and_backend_reuse",
            "wal_writer_mounted_controller_fences_at_signal_and_wakes_outside_registry_without_tasks",
            "wal_writer_mounted_controller_fault_returns_exact_retry_owner_without_poisoning_other_writer",
            "wal_writer_mounted_stale_controller_defers_cross_key_wake_and_fences_retry_epoch",
            "wal_writer_mounted_controller_rerequests_after_async_executor_handback",
            "wal_writer_mounted_controller_coalesced_fault_does_not_strand_healthy_release",
            "wal_writer_mounted_controller_outer_panic_faults_waiters_once_and_stops",
            "db_io_memory_backend_heap_tables_have_exact_preflight_credit_and_terminal_return",
            "db_io_retained_page_results_survive_same_task_slot_reuse_and_return_exact_credit",
            "fs_storage_canonical_alias_writer_fences_all_six_mutations",
            "sqlite_wal_writer_real_database_alias_and_crash_are_exclusive",
            "fs_wal_directory_barriers_match_neutral_order_and_duplicate_create_is_atomic",
            "fs_wal_directory_faults_retain_seal_and_delete_order_until_explicit_retry",
            "fs_replacement_reports_failure_until_renamed_parent_is_synced",
            "fs_wal_reopen_repairs_unacknowledged_segment_namespace_before_header_ack",
            "replicate_document_fences_occupied_follower_before_inventory_or_up_to_date",
            "replicate_document_releases_follower_after_leader_replay_failure",
            "replicate_document_applies_missing_tail_commands_to_a_fresh_follower",
            "replicate_document_reports_up_to_date_once_a_follower_catches_up",
            "replicate_document_transfers_a_snapshot_when_the_follower_is_below_the_retained_floor",
            "artifact_wal_open_rejection_retains_exact_writer_for_close_or_same_owner_retry",
            "artifact_engine_create_rejection_propagates_exact_wal_release_owner",
            "database_document_mount_failure_terminalizes_authority_builder_wal_owner_before_fanout",
            "db_io_registered_backend_use_blocks_pool_shutdown_until_terminal_close",
            "db_io_backend_registration_saturation_returns_exact_executor_before_pool_use",
            "db_io_prepared_registration_failure_returns_exact_close_owner_after_submission_refusal",
            "db_io_task_uses_registered_backend_pool_not_caller_pool",
            "db_io_backend_drop_retains_pool_until_deferred_close_terminal",
            "db_io_forged_backend_kind_is_rejected_before_task_page_admission",
            "database_compaction_future_acquires_pool_use_before_admission",
          ],
        },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log(`wal-writer-authority-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    for (const receipt of receipts) console.log(`wal-writer-authority-native-receipt: ${JSON.stringify(receipt)}`);
  }
}

/** 🧾️ Proves the logical commit firewall with an independent neutral grammar evaluator. */
class WalCommittedTransactionsCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-committed-transactions-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️fixtures/🧾️committed-transactions/🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧪️fixtures/🧾️committed-transactions/🧬️.schema.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(fixture.historyProjection.expected, { entries: 1, operationIds: fixture.historyProjection.commands.map((command: any) => command.id), headSeq: fixture.historyProjection.commands.length, commitSeq: 1 });
    assert.equal(new Set(fixture.historyProjection.expected.operationIds).size, fixture.historyProjection.commands.length);
    for (const row of fixture.historyProjection.rejections) {
      const frontiers = row.records.flatMap((record: any, index: number) => (record.kind === "frontier" ? [index] : []));
      const accepted = row.records.every((record: any) => record.document === "current") && frontiers.length === 1 && frontiers[0] === row.records.length - 1;
      assert.equal(accepted, row.accepted, row.name);
    }
    for (const row of fixture.cases) {
      let next = 1n;
      let observed = false;
      let recoverAbort: string | null = null;
      const transactions: { id: string; kinds: string[] }[] = [];
      let error: string | null = null;
      try {
        for (const [index, segment] of row.segments.entries()) {
          if (index !== row.segments.length - 1 && segment.state !== "sealed") throw "corrupt";
          assert.deepEqual(
            [...segment.physicalCommitsAfter].sort((a: number, b: number) => a - b),
            segment.physicalCommitsAfter,
          );
          assert.equal(segment.physicalCommitsAfter.at(-1), segment.frames.length - 1);
          let current: { id: string; kinds: string[] } | null = null;
          for (const [ordinal, frame] of segment.frames.entries()) {
            if (frame.kind === "header") {
              if (ordinal !== 0 || current !== null) throw "corrupt";
              continue;
            }
            if (ordinal === 0) throw "corrupt";
            if (frame.kind === "begin") {
              if (current !== null || BigInt(frame.id) < next || (observed && BigInt(frame.id) !== next)) throw "corrupt";
              const payload = Buffer.alloc(8);
              payload.writeBigUInt64LE(BigInt(frame.id));
              const id = payload.readBigUInt64LE();
              if (id === 0xffffffffffffffffn) throw "sequence";
              next = id + 1n;
              observed = true;
              current = { id: id.toString(), kinds: [] };
            } else if (frame.kind === "commit" || frame.kind === "abort") {
              if (current === null || current.id !== frame.id || (frame.kind === "commit" && current.kinds.length !== frame.count)) throw "corrupt";
              if (frame.kind === "commit") transactions.push(current);
              current = null;
            } else {
              if (current === null) throw "corrupt";
              if (current.kinds.length === fixture.maximumRecords) throw "capacity";
              current.kinds.push(frame.kind);
            }
          }
          if (current !== null) {
            if (segment.state !== "active" || index !== row.segments.length - 1) throw "corrupt";
            recoverAbort = current.id;
          }
        }
      } catch (caught) {
        if (!["corrupt", "capacity", "sequence"].includes(String(caught))) throw caught;
        error = String(caught);
      }
      assert.deepEqual({ accepted: error === null, transactions: error === null ? transactions : [], nextTxId: error === null ? next.toString() : null, recoverAbort: error === null ? recoverAbort : null, error }, row.expected, row.name);
    }
    console.log(`wal-committed-transactions-independent-oracle: AJV=1 u64=1 vectors=${fixture.cases.length}`);
    const faults = JSON.parse(readFileSync(join(owner, "🧪️fixtures/🛑️fail-stop/🔣️.json"), "utf8"));
    const validateFaults = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧪️fixtures/🛑️fail-stop/🧬️.schema.json"), "utf8")));
    assert(validateFaults(faults), JSON.stringify(validateFaults.errors));
    assert.deepEqual(
      faults.cases.filter((row: any) => row.fault !== "successorAppendError").map((row: any) => [row.fault, row.expectedPhysicalSuffix]),
      [
        ["shortAppend", "torn"],
        ["appendError", "absent"],
        ["syncError", "complete"],
      ],
    );
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("struct WalTransactionGate") && source.includes("frames: [Option<WalRecordFrame>; 64]"), "logical admission must retain fixed frame spans, not owned decoded records");
    assert(source.includes("WalCommittedCursor") && source.includes("WalCommittedTransaction"), "materializers need one shared borrowed committed cursor");
    assert(source.includes("enum WalVerifiedFrameStep"), "verified replay must yield after each physical frame without repeating whole-frame CRC work");
    assert(source.includes("trait WalImmutableByteSource"), "History must share the authenticated frame source without borrowing another field across polls");
    assert(source.includes("struct WalAuthenticatedSource<S>") && source.includes("source: S"), "History authentication and committed spans must retain the same immutable source owner");
    assert(source.includes("recovered_abort_tx_id") && source.includes("wal recovery abort exceeds retained segment budget"), "active recovery must durably abort within the retained segment budget");
    const history = readFileSync(join(owner, "../🗿️artifact/🦀️.rs"), "utf8");
    assert(history.includes("WalAuthenticatedSource<HistoryPageSet>") && !history.includes("struct HistoryFrameCursor"), "History must consume authenticated committed spans, with no independent frame grammar");
    for (const check of ["history envelope document differs", "history frontier document differs", "history committed frontier is not terminal"]) assert(history.includes(check), `History admission is missing ${check}`);
    const decoder = JSON.parse(readFileSync(join(owner, "🧪️fixtures/📖️retained-decoder/🔣️.json"), "utf8"));
    const validateDecoder = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧪️fixtures/📖️retained-decoder/🧬️.schema.json"), "utf8")));
    assert(validateDecoder(decoder), JSON.stringify(validateDecoder.errors));
    const { default: leb } = await import("@webassemblyjs/leb128/lib/leb.js");
    for (const row of decoder.varints) {
      const bytes = Buffer.from(row.hex, "hex");
      let value: string | null = null;
      let consumed: number | null = null;
      const terminal = bytes.findIndex((byte) => byte < 128);
      if (terminal >= 0 && terminal < 10) {
        const number = bytes.subarray(0, terminal + 1).reduceRight((value, byte) => value * 128n + BigInt(byte & 127), 0n);
        if (number <= 0xffffffffffffffffn) {
          const storage = Buffer.alloc(8);
          storage.writeBigUInt64LE(number);
          if (Buffer.from(leb.encodeUIntBuffer(storage)).equals(bytes.subarray(0, terminal + 1))) {
            value = number.toString();
            consumed = terminal + 1;
          }
        }
      }
      assert.deepEqual({ value, consumed }, { value: row.value, consumed: row.consumed }, row.name);
    }
    assert(source.includes("fn wal_read_canonical_varint"), "retained readers must reject noncanonical and overflowing u64 fields");
    console.log(`wal-retained-decoder-independent-oracle: AJV=1 LEB128=1 vectors=${decoder.varints.length}`);
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib", name: "db" },
            cargoArgs: ["--all-features"],
            laws: [
              "wal_transaction_gate_matches_neutral_committed_spans",
              "wal_retained_decoder_fuel_resumes_exact_fragmented_bytes",
              "wal_retained_decoder_cancel_close_preserves_source_and_returns_owner",
              "wal_committed_cursor_cancel_resume_keeps_transaction_position",
              "wal_committed_cursor_unfinished_borrow_poison_and_cancelled_close",
              "artifact_open_ignores_neutral_aborted_command_snapshot_and_cas",
              "sync_replay_ignores_neutral_aborted_command_snapshot_and_cas",
              "cli_verify_checks_neutral_logical_commit_boundaries",
              "open_replays_the_wal_and_reconstructs_state_and_frontier_identically",
              "wal_retained_varints_match_neutral_exact_u64_and_atomic_interruption",
              "wal_committed_cursor_single_fuel_and_expired_turns_match_neutral_transactions",
              "sync_retained_reads_resume_neutral_varints_without_renewing_overall_deadline",
              "wal_immutable_source_fragmentation_matches_neutral_transactions",
              "artifact_history_replay_uses_neutral_committed_inventory_and_retires_every_owner",
              "artifact_history_replay_projects_real_committed_batch_and_cancels_owned_sources",
              "artifact_history_and_opener_reject_neutral_inner_documents_and_frontier_order",
              "wal_recovery_aborts_only_incomplete_active_transactions_idempotently",
              "wal_recovery_abort_fsync_survives_two_independent_filesystem_reopens",
              "wal_recovery_abort_faults_retry_without_duplicate_abort",
              "wal_recovery_abort_cancellation_has_one_durable_boundary",
              "wal_recovery_abort_capacity_exact_and_plus_one_preserves_source",
              "artifact_history_panic_at_each_phase_transition_retains_then_fault_retires",
              "db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects",
            ],
          },
        ],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
        listBudgetMs: 60_000,
        lawBudgetMs: 120_000,
        progress(event) {
          console.log(`wal-committed-transactions-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      for (const receipt of receipts) console.log(`wal-committed-transactions-native-receipt: ${JSON.stringify(receipt)}`);
    }
  }
}

/** 🧹️ Proves compaction observes only logically committed WAL effects. */
class WalCommittedCompactionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-committed-compaction-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️fixtures/🧾️committed-effects/🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧪️fixtures/🧾️committed-effects/🧬️.schema.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(
      fixture.segments.map((row: any) => row.index),
      fixture.segments.map((_: any, index: number) => index),
    );
    assert.equal(fixture.segments.at(-1).state, "active");
    const committed = fixture.segments.flatMap((segment: any) =>
      segment.transactions.filter((transaction: any) => transaction.outcome === "commit").flatMap((transaction: any) => transaction.records.map((record: any) => ({ ...record, segment: segment.index }))),
    );
    const horizons = fixture.segments.map((segment: any) => ({
      segment: segment.index,
      head: committed.filter((record: any) => record.segment === segment.index && ["frontier", "snapshot"].includes(record.kind)).reduce((head: number | null, record: any) => (head === null ? record.headSeq : Math.max(head, record.headSeq)), null),
    }));
    const highest = fixture.segments.at(-1).index;
    const deletedSegments = horizons.filter((row: any) => row.segment !== highest && row.head !== null && row.head <= fixture.floorHeadSeq).map((row: any) => row.segment);
    const deletedPayloads =
      fixture.actor.payloadReclamation === "deferred-global-reference-authority"
        ? []
        : committed
            .filter((record: any) => record.kind === "payload" && deletedSegments.includes(record.segment) && !committed.some((live: any) => live.kind === "payload" && live.payload === record.payload && !deletedSegments.includes(live.segment)))
            .map((record: any) => record.payload);
    const allPayloads = new Set(fixture.segments.flatMap((segment: any) => segment.transactions.flatMap((transaction: any) => transaction.records.filter((record: any) => record.kind === "payload").map((record: any) => record.payload))));
    assert.deepEqual(
      {
        deletedSegments: deletedSegments.length,
        deletedPayloads: new Set(deletedPayloads).size,
        remainingSegments: fixture.segments.map((row: any) => row.index).filter((index: number) => !deletedSegments.includes(index)),
        retainedPayloads: [...allPayloads].filter((payload) => !deletedPayloads.includes(payload)),
      },
      fixture.expected,
    );
    assert.deepEqual(fixture.actor, {
      priority: "command",
      writerAuthority: "retained-artifact-wal",
      activeSegmentAuthority: "artifact-wal",
      leaseBefore: ["snapshot-floor", "wal-horizon", "wal-delete"],
      indexBudgetContinuation: "same-owned-future-cooperative-yield",
      payloadReclamation: "deferred-global-reference-authority",
      queuedSubmitDuring: "pending",
      queuedSubmitAfter: "accepted",
    });
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const committedCut = source.slice(source.indexOf("async fn committed_compaction_horizons"), source.indexOf("async fn retained_compaction_under_lease"));
    assert(committedCut.includes("replay_committed_document") && committedCut.includes("close_record_step") && committedCut.includes("transaction.finish()") && committedCut.includes("close_compaction_replay"));
    assert(source.slice(source.indexOf("async fn close_compaction_replay"), source.indexOf("async fn close_compaction_owner")).includes("close_owner_step"));
    assert(!committedCut.includes("WalReplayCursor") && !committedCut.includes("replay_document"));
    const liveCut = source.slice(source.indexOf("pub async fn retained_compaction_with_wal"), source.indexOf("async fn retained_compaction_under_lease"));
    assert(liveCut.indexOf("CompactionLease::acquire") < liveCut.indexOf("SnapshotFloor"));
    assert(!liveCut.includes("acquire_writer"));
    const underLeaseCut = source.slice(source.indexOf("async fn retained_compaction_under_lease"), source.indexOf("async fn retained_compaction_snapshot"));
    assert(!underLeaseCut.includes("loop {\n            let deadline") && underLeaseCut.includes("handle.compact(&mut control).await"));
    const walSource = readFileSync(join(owner, "..", "📝️wal", "🦀️.rs"), "utf8");
    assert(walSource.includes("delete_compacted_sealed_segment"));
    const artifactSource = readFileSync(join(owner, "..", "🗿️artifact", "🦀️.rs"), "utf8");
    assert(artifactSource.includes("ArtifactMessage::Compact") && artifactSource.includes("Priority::Command"));
    assert(source.includes("fn compaction_applies_only_committed_frontier_snapshot_and_payload_effects("));
    console.log("wal-committed-compaction-independent-oracle: abort effects excluded, global payloads retained, header-only highest preserved");
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        cargoArgs: ["--all-features"],
        groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib", name: "db" },
            laws: [
              "db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects",
              "db_compact::tests::document_compaction_retains_shared_and_private_cas_without_global_reference_authority",
              "db_engine::vcs_integration::retained_tests::vcs_store_keeps_exact_history_owners_through_changes_checkpoint_and_bounded_close",
              "db_engine::tests::compact_document_uses_live_actor_writer_and_restores_submits",
            ],
          },
        ],
        progress(event) {
          console.log(`wal-committed-compaction-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`wal-committed-compaction-native-receipts: ${JSON.stringify(receipts)}`);
    }
  }
}

/** 🚪️ Proves retained database shutdown keeps exact retry owners across interruption and error. */
class DatabaseShutdownCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-shutdown-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️fixtures/🚪️shutdown/🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧪️fixtures/🚪️shutdown/🧬️.schema.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(fixture.phases, ["authority", "versionGraph", "emit", "complete"]);
    assert.equal(fixture.maximumAuthorityStepsPerTurn, 1);
    for (const row of fixture.cases) {
      let authorityOwners = row.initial.authorityOwners;
      let externalAuthorityOwners = row.initial.externalAuthorityOwners;
      let graphStores = row.initial.graphStores;
      let closing = false;
      let cancelled = false;
      let injectedCloseError = false;
      let terminal = false;
      let emitCount = 0;
      let authorityRetained = false;
      let graphRetained = false;
      for (const action of row.trace) {
        if (action === "cancel") {
          cancelled = true;
          authorityRetained ||= authorityOwners > 0 || closing;
          graphRetained ||= graphStores > 0;
          continue;
        }
        if (action === "retry") {
          cancelled = false;
          continue;
        }
        if (action === "inject-close-error") {
          injectedCloseError = true;
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
            graphRetained ||= graphStores > 0;
          } else if (!closing) {
            closing = true;
          } else {
            closing = false;
            authorityOwners -= 1;
          }
        } else if (graphStores > 0) {
          if (injectedCloseError) {
            injectedCloseError = false;
            graphRetained = true;
          } else {
            graphStores -= 1;
          }
        } else if (emitCount === 0) {
          emitCount = 1;
        } else {
          terminal = true;
        }
      }
      assert.deepEqual({ terminal, authorityRetained, graphRetained, emitCount }, row.expected, row.name);
    }
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const artifact = readFileSync(join(owner, "..", "🗿️artifact", "🦀️.rs"), "utf8");
    const graph = readFileSync(join(owner, "..", "🕸️version-graph", "🦀️.rs"), "utf8");
    assert(source.includes("closing_authority: Option<") && source.includes("pub async fn shutdown_step(&mut self"));
    assert(source.includes("DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Authorities"));
    assert(source.includes("state.store = Some(store)") && source.includes("shutdown_graph_complete"));
    assert(!source.includes("pub async fn shutdown(self"));
    assert(artifact.includes("pub fn shutdown_step(&self) -> bool") && artifact.includes("handoff.terminal"));
    assert(graph.includes("VersionGraphShutdownStep") && graph.includes("fn shutdown_step(&self)"));
    console.log(`database-shutdown-independent-oracle: AJV=1 cases=${fixture.cases.length} retained-authority=1 retained-graph=1 terminal-ack=1`);
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        cargoArgs: ["--all-features"],
        groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib", name: "db" },
            laws: [
              "db_engine::vcs_integration::retained_tests::vcs_shutdown_error_reinstalls_exact_store_and_retry_reaches_terminal",
              "db_engine::tests::database_shutdown_cancellation_and_vcs_error_preserve_exact_retry_owners",
              "db_engine::tests::database_shutdown_shared_authority_blocks_without_closing_live_handle",
            ],
          },
        ],
        progress(event) {
          console.log(`database-shutdown-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`database-shutdown-native-receipts: ${JSON.stringify(receipts)}`);
    }
  }
}

/** 🪢️ Proves one generation-fenced retained mount owner serves every concurrent document opener. */
class DocumentMountSingleFlightCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("document-mount-single-flight-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🚪️document-mount");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️fixtures/🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧪️fixtures/🧬️.schema.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
    for (const row of fixture.cases) {
      let activeGeneration: bigint | undefined;
      let waiters = 0;
      let ready = false;
      let terminalCleanup = true;
      let emitting = false;
      let events = 0;
      let fanoutPrepared = false;
      let registryUnlocked = true;
      let ownerPollActive = false;
      for (const step of row.steps) {
        const generation = BigInt(step.generation);
        if (step.action === "install") {
          assert.equal(activeGeneration, undefined, row.name);
          activeGeneration = generation;
          waiters = 1;
          ready = false;
        } else if (step.action === "join") {
          assert.equal(generation, activeGeneration, row.name);
          waiters += 1;
        } else if (step.action === "cancel-waiter") {
          assert.equal(generation, activeGeneration, row.name);
          waiters -= 1;
        } else if (step.action === "catalog-published" || step.action === "shutdown-interrupted") {
          assert.equal(generation, activeGeneration, row.name);
        } else if (step.action === "emit-begin") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!emitting && events === 0, row.name);
          emitting = true;
        } else if (step.action === "emit-complete") {
          assert.equal(generation, activeGeneration, row.name);
          assert(events === 0, row.name);
          emitting = false;
          events = 1;
        } else if (step.action === "fill-waiters") {
          assert.equal(generation, activeGeneration, row.name);
          waiters = fixture.capacity.waitersPerDocument;
        } else if (step.action === "reject-over-capacity") {
          assert.equal(generation, activeGeneration, row.name);
          assert.equal(waiters, fixture.capacity.waitersPerDocument, row.name);
        } else if (step.action === "release-slot") {
          assert.equal(generation, activeGeneration, row.name);
          waiters -= 1;
        } else if (step.action === "close-fault") {
          assert.equal(generation, activeGeneration, row.name);
          terminalCleanup = false;
        } else if (step.action === "resume-cleanup") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!terminalCleanup, row.name);
        } else if (step.action === "explicit-retirement-retry") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!terminalCleanup, row.name);
        } else if (step.action === "close-terminal") {
          assert.equal(generation, activeGeneration, row.name);
          terminalCleanup = true;
          activeGeneration = undefined;
        } else if (step.action === "prepare-fanout") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!fanoutPrepared && registryUnlocked, row.name);
          fanoutPrepared = true;
          registryUnlocked = false;
        } else if (step.action === "unlock-registry") {
          assert.equal(generation, activeGeneration, row.name);
          assert(fanoutPrepared && !registryUnlocked, row.name);
          registryUnlocked = true;
        } else if (step.action === "wake-waiter") {
          assert.equal(generation, activeGeneration, row.name);
          assert(fanoutPrepared && registryUnlocked, row.name);
          fanoutPrepared = false;
        } else if (step.action === "owner-poll-active") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!ownerPollActive, row.name);
          ownerPollActive = true;
        } else if (step.action === "request-drive") {
          assert.equal(generation, activeGeneration, row.name);
          assert(ownerPollActive, row.name);
        } else if (step.action === "owner-poll-release") {
          assert.equal(generation, activeGeneration, row.name);
          assert(ownerPollActive, row.name);
          ownerPollActive = false;
        } else if (step.action === "retry") {
          assert.equal(activeGeneration, undefined, row.name);
          activeGeneration = generation;
          waiters = 1;
          events = 0;
        } else if (step.action === "ready") {
          assert.equal(generation, activeGeneration, row.name);
          assert(terminalCleanup && !emitting && events === 1, row.name);
          ready = true;
        }
      }
      assert.equal(ready ? 1 : 0, row.expected.actors, row.name);
      assert.equal(row.expected.owners, 1, row.name);
      assert.equal(events, row.expected.events, row.name);
      assert.equal(waiters, row.expected.waitersReleased, row.name);
      assert.equal(terminalCleanup, row.expected.terminalBeforeFanout, row.name);
      assert(!fanoutPrepared && registryUnlocked && !ownerPollActive, row.name);
    }
    for (const row of fixture.driverCases) {
      let state = "idle";
      let wakeRequested = false;
      let resumeRequested = false;
      let queued = 0;
      let maximumQueued = 0;
      let cleanupParked = false;
      let cleanupResumes = 0;
      for (const step of row.steps) {
        if (step === "request" || step === "request-resume") {
          wakeRequested = true;
          resumeRequested ||= step === "request-resume";
          if (state === "idle") {
            state = "queued";
            queued = 1;
            maximumQueued = Math.max(maximumQueued, queued);
          }
        } else if (step === "poll-start") {
          assert.equal(state, "queued", row.name);
          state = "polling";
          queued = 0;
          wakeRequested = false;
        } else if (step === "cleanup-fault") {
          assert.equal(state, "polling", row.name);
          cleanupParked = true;
        } else if (step === "cleanup-resume") {
          assert(cleanupParked && resumeRequested, row.name);
          cleanupParked = false;
          resumeRequested = false;
          cleanupResumes += 1;
        } else if (step === "hard-submit-rejected") {
          assert.equal(state, "queued", row.name);
          state = "non-runnable";
        } else if (step === "poll-pending") {
          assert.equal(state, "polling", row.name);
          if (wakeRequested) wakeRequested = false;
          else state = "idle";
        } else if (step === "complete") {
          state = "terminal";
        }
      }
      assert.equal(state, row.expected.state, row.name);
      assert.equal(maximumQueued, row.expected.maximumQueuedPollers, row.name);
      assert.equal(cleanupResumes, row.expected.cleanupResumes, row.name);
    }
    const source = readFileSync(join(owner, "..", "🦀️.rs"), "utf8");
    for (const marker of [
      "enum DatabaseDocumentMountSlot",
      "Opening {",
      "struct DatabaseDocumentMountOwner",
      "DatabaseDocumentMountWork",
      "DATABASE_DOCUMENT_MOUNT_WAITERS",
      "fn take_generation",
      "pub async fn ensure_document",
      "retained_mount_rejection",
      "DatabaseDocumentMountWait",
      "impl Drop for DatabaseDocumentMountWait",
    ])
      assert(source.includes(marker), `missing retained document-mount marker: ${marker}`);
    assert(!source.includes("open_artifacts: Mutex<HashMap<String, Arc<db_artifact::ArtifactAuthority>>>") && source.includes("open_artifacts: Arc<Mutex<DatabaseDocumentMountRegistry>>"));
    const retainedMount = source.slice(source.indexOf("async fn run_document_mount("), source.indexOf("async fn mount_document("));
    assert(retainedMount.indexOf("emit.emit(") >= 0 && retainedMount.indexOf("emit.emit(") < retainedMount.indexOf("Ok(DatabaseDocumentMountReply"));
    assert(!source.slice(source.indexOf("struct DatabaseDocumentMountReply"), source.indexOf("struct DatabaseDocumentMountWaiter")).includes("emission:"));
    const complete = source.slice(source.indexOf("fn complete(&self, result: Result<DatabaseDocumentMountReply, DbError>)"), source.indexOf("//#region 🔖️Database"));
    assert(complete.includes("let mut fanout: [Option<(") && complete.includes("DATABASE_DOCUMENT_MOUNT_WAITERS"));
    assert(complete.indexOf("drop(authority);") < complete.indexOf("for (reply, outcome) in fanout.into_iter().flatten()"));
    assert(complete.indexOf("self.terminal.store(true") < complete.indexOf("for (reply, outcome) in fanout.into_iter().flatten()"));
    assert(source.includes("self.registry.try_lock().is_ok()"));
    const mountOwner = source.slice(source.indexOf("impl DatabaseDocumentMountOwner"), source.indexOf("//#region 🔖️Database"));
    assert(
      mountOwner.includes("DatabaseDocumentMountDriver::Idle") &&
        mountOwner.includes("DatabaseDocumentMountDriver::Queued") &&
        mountOwner.includes("DatabaseDocumentMountDriver::Polling") &&
        mountOwner.includes("DatabaseDocumentMountDriver::NonRunnable"),
    );
    assert(mountOwner.includes("resume_requested") && mountOwner.includes("wake_requested"));
    assert(mountOwner.includes("Arc::downgrade(self)") && !mountOwner.includes("let owner = self.clone();\n        self.submit_exact"));
    const databaseOpen = source.slice(source.indexOf("async fn open_with("), source.indexOf("fn document_engine_config("));
    assert.equal(databaseOpen.match(/acquire_use\(\)/g)?.length, 1);
    for (const marker of ["DatabaseCapabilityOpenFuture::try_prepare_with_use", "DatabaseCatalogReadFuture::try_prepare_with_use", "DatabaseCatalogBootstrapFuture::try_prepare_with_use"])
      assert(databaseOpen.includes(marker), `database open minted an untracked pool use instead of retaining ${marker}`);
    const catalogPublication = source.slice(source.indexOf("async fn publish_mount_catalog("), source.indexOf("async fn run_open_document_mount("));
    assert(catalogPublication.includes("DatabaseCreateCatalogFuture::try_prepare_with_use(pool, pool_use"));
    const catalogDriveStart = source.indexOf("fn drive_one(self: Arc<Self>, generation: u64)", source.indexOf("//#region 🔖️CreateDocumentCatalogCas"));
    const catalogDrive = source.slice(catalogDriveStart, source.indexOf("fn drive_claimed(self: &Arc<Self>, generation: u64)", catalogDriveStart));
    assert(!catalogDrive.includes("release_success()"));
    const catalogTerminal = source.slice(source.indexOf("pub struct DatabaseCreateCatalogTerminalHandle"), source.indexOf("pub fn take_database_create_catalog_terminal"));
    assert(catalogTerminal.includes("drive_explicit_close_one()"), "explicit catalog terminal cleanup must make one bounded turn without a wall-clock callback");
    const helloStart = source.indexOf("pub fn hello_retained(");
    const hello = source.slice(helloStart, source.indexOf("pub async fn hello(", helloStart));
    assert(hello.includes("DatabaseSyncHelloFuture::try_submit_with_use") && !hello.includes("DatabaseSyncHelloFuture::try_submit("));
    const requestDrive = mountOwner.slice(mountOwner.indexOf("fn request_drive(self: &Arc<Self>"), mountOwner.indexOf("fn resume_parked("));
    assert(!requestDrive.includes("self.work.try_lock()") && !requestDrive.includes("self.work.lock()"));
    const artifact = readFileSync(join(owner, "..", "..", "🗿️artifact", "🦀️.rs"), "utf8");
    for (const marker of [
      "enum ArtifactRunnerDriver",
      "RunnableIdle",
      "Queued",
      "PollingWake",
      "Parked",
      "ClosingReady",
      "ClosingPollingWake",
      "ClosingParked",
      "Terminal",
      "fn park_terminal_job",
      "struct ArtifactRunnerClosePoll",
      "struct ArtifactRunnerPoll",
      "struct ArtifactRunnerRetirementReservation",
      "WorkerMaintenanceStep::Retire",
      "impl Drop for ArtifactAuthority",
      "pub fn close(mut self) -> Result<(), Self>",
      "pub fn resume(mut self) -> Result<(), Self>",
    ]) {
      assert(artifact.includes(marker), `missing artifact terminal-authority marker: ${marker}`);
    }
    for (const marker of ["close_error", "WorkerMaintenanceStep::Fault", "artifact_engine_close_fault_retains_exact_runner_until_explicit_maintenance_retry"]) {
      assert(artifact.includes(marker), `missing retained artifact close-fault marker: ${marker}`);
    }
    const artifactSchedule = artifact.slice(artifact.indexOf("fn schedule(self: &Arc<Self>)"), artifact.indexOf("fn submit_exact(self: &Arc<Self>"));
    assert(artifactSchedule.includes("compare_exchange") && artifactSchedule.includes("ArtifactRunnerDriver::RunnableIdle as u8") && artifactSchedule.includes("ArtifactRunnerDriver::Queued as u8"));
    assert(!artifactSchedule.includes("scheduled.compare_exchange"));
    const graph = readFileSync(join(owner, "..", "..", "🕸️version-graph", "🦀️.rs"), "utf8");
    assert(graph.includes("fn emit(&self, event: EmitEvent) -> impl Future<Output = ()> + Send;"));
    const hub = readFileSync(join(this.repoRoot, "🌎️hub/📦️packages/🦀️rust/🚀️bin.rs"), "utf8");
    const ensure = hub.slice(hub.indexOf("async fn ensure_document(&self"), hub.indexOf("fn bearer("));
    assert(ensure.includes("self.db.ensure_document(id).await") && ensure.includes("rejected.retry_close().await"));
    assert(!ensure.includes("self.db.create_document") && !ensure.includes("self.db.document(id)"));
    for (const law of [
      "database_document_mount_coalesces_join_drives_without_shared_pool_starvation",
      "database_document_mount_cleanup_fault_consumes_racing_resume_request_exactly_once",
      "database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer",
    ]) {
      assert(source.includes(`fn ${law}(`), `missing exact mount driver law ${law}`);
    }
    console.log(`document-mount-single-flight-independent-oracle: AJV=1 cases=${fixture.cases.length} waiters=${fixture.capacity.waitersPerDocument} owner-futures=${fixture.capacity.ownerFuturesPerDocument}`);
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [
        {
          package: "semio-framework-os-kernel-db",
          target: { kind: "lib", name: "db" },
          cargoArgs: ["--all-features"],
          laws: [
            "db_engine::tests::database_concurrent_ensure_mounts_one_actor_and_one_writer",
            "db_engine::tests::database_published_opening_joins_without_actor_overwrite",
            "db_engine::tests::database_cancelled_ensure_waiter_does_not_cancel_mount_owner",
            "db_engine::tests::database_document_mount_failure_waiters_share_terminal_cleanup_and_retry_generation",
            "db_engine::tests::database_mount_owner_emits_before_ready_and_survives_elected_waiter_cancellation",
            "db_engine::tests::database_mount_waiter_capacity_rejects_33_and_reuses_one_cancelled_slot",
            "db_engine::tests::database_document_mount_fanout_wakes_only_after_registry_unlock_and_internal_owner_handoff",
            "db_engine::tests::database_shutdown_interrupt_retains_waiterless_opening_owner_until_ready",
            "db_engine::tests::database_document_mount_unlock_fault_parks_exact_owner_until_controlled_shutdown_resume",
            "db_engine::tests::database_document_mount_coalesces_join_drives_without_shared_pool_starvation",
            "db_engine::tests::database_document_mount_cleanup_fault_consumes_racing_resume_request_exactly_once",
            "db_engine::tests::database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack",
            "db_engine::tests::database_worker_pool_use_is_admitted_before_the_first_storage_probe",
            "db_engine::tests::database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer",
            "db_engine::tests::database_create_catalog_resolved_drop_retains_use_until_terminal_drain",
            "db_artifact::tests::artifact_runner_terminal_authority_latch_preserves_external_job_and_one_resume",
            "db_artifact::tests::artifact_runner_closing_poll_waits_for_retained_wake_before_next_turn",
            "db_artifact::tests::artifact_runner_terminal_close_returns_exact_cursor_until_retained_wake",
            "db_artifact::tests::artifact_runner_terminal_resume_refusal_returns_exact_cursor_for_close",
            "db_artifact::tests::artifact_authority_drop_transfers_parked_terminal_job_to_registered_close_owner",
            "db_artifact::tests::artifact_runner_retirement_panic_retains_exact_cursor_until_explicit_retry",
            "db_artifact::tests::artifact_engine_close_fault_retains_exact_runner_until_explicit_maintenance_retry",
            "db_artifact::tests::artifact_authority_drop_reuses_registered_retirement_slot_beyond_capacity",
          ],
        },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 180_000,
      progress(event) {
        console.log(`document-mount-single-flight-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`document-mount-single-flight-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

/** 🗺️ Proves the fixed owned Map decision envelope with independent AJV and SHA-256 oracles. */
class DurableOwnedGroupDecisionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("durable-owned-group-decision-check accepts only --native");
    const { testDurableOwnedGroupDecisionFixture } = await import("../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️script.ts");
    testDurableOwnedGroupDecisionFixture();
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [
          {
            package: "semio-framework-os-kernel",
            target: { kind: "lib" },
            laws: [
              "durable_group::tests::durable_owned_group_decision_matches_neutral_canonical_hash_and_bounds",
              "durable_group::tests::durable_group_journal_record_projects_edits_only_after_all_three_bound_outcomes_verify",
              "durable_group::tests::durable_store_prepared_outcome_derives_and_verifies_exact_unbound_bytes",
              "durable_group::tests::durable_owned_group_decision_rejects_forged_identity_commitment_and_capacity",
              "durable_group::tests::durable_decision_rejects_deflate_expansion_before_document_body_allocation",
              "durable_group::tests::durable_store_owned_three_member_bind_and_base_recovery_retain_exact_private_owners",
              "durable_group::tests::durable_store_private_committed_record_recovers_all_three_stores_without_reappending_journal",
              "durable_group::tests::durable_json_carriers_preserve_numeric_kinds_and_reject_control_and_resource_excess",
              "durable_group::tests::durable_store_group_journal_commit_flips_one_shared_root_then_adopts_exactly_once",
              "durable_group::tests::durable_store_group_cancellation_waits_for_trusted_absence_then_restores_all_old_roots",
              "durable_group::tests::durable_store_group_stage_error_retains_abort_owner_until_every_root_is_empty",
              "durable_group::tests::durable_store_group_uncertain_journal_error_retries_same_owner_without_rebegin_or_visibility_change",
              "durable_group::tests::durable_store_group_rejects_foreign_anchor_receipt_before_visibility_and_aborts_only_after_absence",
              "durable_group::tests::durable_map_fixed_host_slot_retains_every_live_owner_across_request_error_until_terminal_handoff",
              "durable_group::tests::durable_map_fixed_host_slot_cancellation_after_uncertain_io_waits_for_trusted_absence",
            ],
          },
        ],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
        listBudgetMs: 60_000,
        lawBudgetMs: 120_000,
        progress(event) {
          console.log(`durable-owned-group-decision-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      for (const receipt of receipts) console.log(`durable-owned-group-decision-native-receipt: ${JSON.stringify(receipt)}`);
    }
  }
}

/** 🧾️ Proves the typed Store-decision journal boundary and exact physical WAL reservation. */
class DurableGroupJournalCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("durable-group-journal-check accepts only --native");
    const artifactOwner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact");
    const fixtureOwner = join(artifactOwner, "🧪️fixtures/📓️durable-group-journal");
    const fixture = JSON.parse(readFileSync(join(fixtureOwner, "🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(fixtureOwner, "🧬️.schema.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.equal(new Set(fixture.cases.map((row: any) => row.id)).size, fixture.cases.length);
    for (const row of fixture.committedDecisionWitnessCases) {
      const events = row.recordKinds.filter((kind: string) => kind === "event").length;
      const expected = row.transaction === "aborted" || events === 0 ? "ignored" : row.replayDocument === "foreign" || events !== 1 || row.recordKinds.length !== 1 ? "rejected" : "witness";
      assert.equal(row.expected, expected);
    }
    assert.deepEqual(fixture.committedRecovery.authority, {
      consumer: "db-wal-witness",
      recordEscape: false,
      receiptSource: "derived",
      cancelAfterCommit: false,
      errorSurface: "retaining-fault",
      terminalHandoff: "stores-and-ack-once",
    });
    assert.equal(new Set(fixture.committedRecovery.cases.map((row: any) => row.id)).size, fixture.committedRecovery.cases.length);
    for (const row of fixture.committedRecovery.cases) {
      const expected = row.frontier === "base" ? ["complete", 3] : row.frontier === "post" ? ["already-applied", 0] : ["fault", 0];
      assert.deepEqual([row.expected, row.mutations], expected);
      assert.equal(row.returnedStoreOwners, 3);
    }
    const storeOwner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group");
    const storeFixture = JSON.parse(readFileSync(join(storeOwner, "🧪️fixtures/🔣️.json"), "utf8"));
    assert.equal(createHash("sha256").update(storeFixture.expected.unsignedJson).digest("hex"), fixture.record.decisionSha256);
    assert.equal(storeFixture.expected.anchorSha256, fixture.record.anchorSha256);
    const varintBytes = (value: number) => {
      let bytes = 1;
      while (value >= 128) {
        value = Math.floor(value / 128);
        bytes += 1;
      }
      return bytes;
    };
    const frameBytes = (payload: number) => varintBytes(payload + 2) + payload + 10;
    const fieldBytes = (value: string) => varintBytes(Buffer.byteLength(value)) + Buffer.byteLength(value);
    const successorHeaderBytes = 32 + frameBytes(fieldBytes(fixture.record.document) + 41) + 75;
    const transactionBytes = (eventBytes: number) => frameBytes(8) + frameBytes(eventBytes) + frameBytes(12) + 75;
    assert.equal(successorHeaderBytes, fixture.limits.successorHeaderBytes);
    assert.equal(transactionBytes(fixture.limits.storeEventBytes), fixture.limits.storeMaximumTransactionBytes);
    assert.equal(successorHeaderBytes + transactionBytes(fixture.limits.storeEventBytes), fixture.limits.storeMaximumSegmentBytes);
    let maximumEventBytes = fixture.limits.walSegmentBytes;
    while (successorHeaderBytes + transactionBytes(maximumEventBytes) > fixture.limits.walSegmentBytes) maximumEventBytes -= 1;
    assert.equal(maximumEventBytes, fixture.limits.maximumEventBytes);
    assert(successorHeaderBytes + transactionBytes(fixture.limits.storeEventBytes) <= fixture.limits.walSegmentBytes);
    assert(successorHeaderBytes + transactionBytes(maximumEventBytes + 1) > fixture.limits.walSegmentBytes);
    const storeSource = readFileSync(join(storeOwner, "🦀️.rs"), "utf8");
    const artifactSource = readFileSync(join(artifactOwner, "🦀️.rs"), "utf8");
    const walSource = readFileSync(join(artifactOwner, "../📝️wal/🦀️.rs"), "utf8");
    const engineSource = readFileSync(join(artifactOwner, "../⚙️engine/🦀️.rs"), "utf8");
    assert(storeSource.includes("pub struct DurableOwnedGroupJournalRecordV1") && storeSource.includes("DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&canonical_pack)"));
    assert(storeSource.includes("pub enum DurableOwnedGroupJournalAdvanceV1") && storeSource.includes("Rejected(String)"));
    const append = artifactSource.slice(artifactSource.indexOf("async fn append_durable_group_decision("), artifactSource.indexOf("pub(crate) async fn compact_retained", artifactSource.indexOf("async fn append_durable_group_decision(")));
    assert(append.includes("WalRecord::Event") && append.includes("DurabilityClass::Fsync"));
    assert(append.indexOf("preflight_submit") < append.indexOf("self.wal.submit"));
    assert(append.includes("ArtifactDurableGroupJournalAppendV1::Absent") && append.includes("ArtifactDurableGroupJournalAppendV1::Rejected"));
    const witness = artifactSource.slice(artifactSource.indexOf("struct ArtifactCommittedDurableGroupDecisionV1"), artifactSource.indexOf("//#endregion 🔖️Receipt"));
    const lowerRecovery = storeSource.slice(
      storeSource.indexOf("impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapRecoveryHostV1"),
      storeSource.indexOf("impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapRecoveryHostV1"),
    );
    assert(witness.includes("WalCommittedTransaction") && witness.includes("record_count != 1") && witness.includes("transaction.finish()?"));
    for (const marker of [
      "ArtifactCommittedDurableGroupRecoveryV1",
      "ArtifactCommittedDurableGroupRecoveryStateV1::Admitting",
      "ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault",
      "into_store_owned_recovery",
      "restore_untrusted_committed_record",
      "acknowledge_restoration(&receipt)",
      "take_terminal",
      "take_rejected_terminal",
    ])
      assert(witness.includes(marker), `missing committed Store recovery marker ${marker}`);
    assert(!witness.includes("pub fn begin_store_owned_recovery"));
    assert(!witness.includes("fn into_record(") && !witness.includes("fn record(&self)"), "a committed WAL witness has no raw record escape");
    assert(!witness.includes("fn cancel("), "committed Store recovery remains non-cancellable after WAL visibility");
    assert(
      lowerRecovery.includes("pub fn advance(&mut self, grant: super::ArtifactStoreOneItemGrant) -> DurableOwnedMapRecoveryAdvanceV1") && lowerRecovery.includes("DurableOwnedMapRecoveryAdvanceV1::Fault(error)"),
      "lower Store recovery faults must retain their exact host instead of exposing Result/? owner loss",
    );
    assert(lowerRecovery.includes("pub fn capture_snapshot(&self) -> Option<"), "lower Store recovery observation must not expose a Result/? owner-loss path");
    const sink = artifactSource.slice(artifactSource.indexOf("struct ArtifactDurableGroupJournalSinkV1"), artifactSource.indexOf("type ArtifactBuildFuture"));
    assert(sink.includes("NotSubmitted") && sink.includes("Awaiting") && sink.includes("Failed") && sink.includes("Committed") && sink.includes("terminal_is_empty"));
    assert(!sink.includes("block_on"));
    assert(walSource.includes("pub(crate) fn preflight_submit(&self, records: &WalRecordBatch)") && walSource.includes("wal transaction exceeds readable segment"));
    assert(engineSource.includes("pub fn durable_group_journal_sink(&self, now_ms: u64)"));
    const laws = [
      "db_artifact::tests::document_authority_durable_group_journal_commits_one_exact_fsync_event",
      "db_artifact::tests::committed_durable_group_decision_accepts_only_one_exact_event_transaction",
      "db_artifact::tests::committed_durable_group_recovery_consumes_wal_witness_and_returns_exact_three_stores_on_pre_mutation_rejection",
      "db_artifact::tests::document_authority_durable_group_journal_cancellation_before_handoff_is_absent",
      "db_artifact::tests::document_authority_durable_group_journal_rejects_hash_before_mailbox",
    ];
    for (const law of laws) assert(artifactSource.includes(`fn ${law.split("::").at(-1)}(`), `missing exact native law ${law}`);
    console.log(
      `durable-group-journal-independent-oracle: AJV=1 cases=${fixture.cases.length} witnesses=${fixture.committedDecisionWitnessCases.length} recovery=${fixture.committedRecovery.cases.length} max-event=${maximumEventBytes} store-margin=${fixture.limits.walSegmentBytes - fixture.limits.storeMaximumSegmentBytes}`,
    );
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, cargoArgs: ["--all-features"], laws }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 180_000,
      progress(event) {
        console.log(`durable-group-journal-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`durable-group-journal-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

/** 🚑️ Proves exact WAL commit boundaries with independent CRC/LEB128 and schema oracles. */
class WalRecoveryCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-recovery-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️fixtures/🚑️recovery/🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧪️fixtures/🚑️recovery/🧬️.schema.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
    const failStop = JSON.parse(readFileSync(join(owner, "🧪️fixtures/🛑️fail-stop/🔣️.json"), "utf8"));
    const validateFailStop = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧪️fixtures/🛑️fail-stop/🧬️.schema.json"), "utf8")));
    assert(validateFailStop(failStop), JSON.stringify(validateFailStop.errors));
    assert.deepEqual(
      failStop.cases.map((row: any) => [row.name, row.fault, row.expectedPhysicalSuffix]),
      [
        ["short-append", "shortAppend", "torn"],
        ["append-error", "appendError", "absent"],
        ["sync-error", "syncError", "complete"],
        ["successor-append-error", "successorAppendError", "complete"],
      ],
    );
    const { default: crc } = await import("crc-32/crc32c.js");
    const leb = await import("@webassemblyjs/leb128");
    const { blake3Hex } = await import("../../🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts");
    const { inspectRetainedSprNeutral } = await import(join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts"));
    const checksum = (bytes: Uint8Array) => crc.buf(bytes) >>> 0;
    const fragmented = Buffer.from(Array.from({ length: 49152 }, (_, index) => (index * 17 + 3) % 251));
    for (const row of fixture.fragmentCopies) assert.equal(checksum(fragmented.subarray(row.offset, row.offset + row.length)), row.crc32c);
    const hash = (bytes: Buffer) => Buffer.from(blake3Hex(bytes), "hex");
    const u64 = (value: number) => {
      const bytes = Buffer.alloc(8);
      bytes.writeBigUInt64LE(BigInt(value));
      return bytes;
    };
    const frame = (kind: number, payload: Buffer) => {
      const body = Buffer.concat([Buffer.from([kind, 2]), payload]);
      const size = Buffer.from(leb.encodeU32(body.length));
      const trailer = Buffer.alloc(8);
      trailer.writeUInt32LE(checksum(body));
      trailer.writeUInt32LE(size.length + body.length + 8, 4);
      return Buffer.concat([size, body, trailer]);
    };
    const header = Buffer.alloc(32);
    Buffer.from([137, 83, 80, 82, 13, 10, 26, 10]).copy(header);
    header.writeUInt16LE(1, 8);
    header.writeUInt32LE(1, 12);
    header.writeUInt32LE(checksum(header.subarray(0, 20)), 20);
    let bytes = header;
    let chain = hash(header);
    let previousOffset = 0;
    const batches = [
      [frame(64, Buffer.concat([Buffer.from([1, 100]), u64(0), Buffer.from([0])]))],
      ...fixture.commands.map((command: string, index: number) => [frame(65, u64(index + 1)), frame(68, Buffer.from(command)), frame(66, Buffer.concat([u64(index + 1), Buffer.from([1, 0, 0, 0])]))]),
    ];
    for (const [index, records] of batches.entries()) {
      const payload = Buffer.alloc(64);
      const recordsLength = records.reduce((sum: number, value: Buffer) => sum + value.length, 0);
      chain = hash(Buffer.concat([chain, ...records.map(hash)]));
      payload.writeBigUInt64LE(BigInt(index + 1));
      payload.writeBigUInt64LE(BigInt(previousOffset), 8);
      payload.writeBigUInt64LE(BigInt(recordsLength), 16);
      payload.writeUInt32LE(records.length, 24);
      chain.copy(payload, 32);
      previousOffset = bytes.length + recordsLength;
      bytes = Buffer.concat([bytes, ...records, frame(12, payload)]);
      assert.equal(bytes.length, fixture.commitEnds[index]);
    }
    for (const row of fixture.cuts) {
      const prefix = bytes.subarray(0, row.cut);
      const span = row.cut < 32 ? { end: 0, sequence: 0 } : inspectRetainedSprNeutral(prefix, checksum, hash);
      assert.equal(span.end, row.trustedEnd);
      assert.equal(Math.max(129, span.end), row.recoveredEnd);
      assert.equal(Math.max(1, span.sequence), row.nextTxId);
    }
    const expectedAccepted = new Set(["missing", "highest-sealed", "successor-empty", "successor-partial", "successor-header", "compacted-clean"]);
    assert.equal(new Set(fixture.lifecycle.map((row: any) => row.name)).size, fixture.lifecycle.length);
    for (const row of fixture.lifecycle) assert.equal(row.accepted, expectedAccepted.has(row.name), row.name);
    console.log(`wal-recovery-independent-oracle: ${fixture.cuts.length} exact CRC/hash-chain prefixes, ${fixture.lifecycle.length} lifecycle rows`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const open = source.slice(source.indexOf("pub async fn open(storage:", source.indexOf("impl ArtifactWal")), source.indexOf("pub async fn document(&self)", source.indexOf("impl ArtifactWal")));
    assert(!open.includes("delete_segment"), "recovery must not delete a committed segment");
    assert(source.includes("resume_verified") && source.includes("RetainedSprVerification"), "full verification and exact writer resume are required");
    const replayClose = source.slice(
      source.indexOf("pub fn close_owner_step(&mut self)", source.indexOf("impl<'storage, S: db_storage::WalStorage> WalReplayCursor")),
      source.indexOf("pub async fn close_step(&mut self)", source.indexOf("impl<'storage, S: db_storage::WalStorage> WalReplayCursor")),
    );
    assert(replayClose.includes("pages.close_step()") && replayClose.includes("segments.close_step()"), "replay close must retire retained page and list owners");
    assert(!replayClose.includes("control.grant()"), "terminal replay close must remain available after cancellation");
    const artifactClose = source.slice(source.indexOf("pub fn close_step(&mut self)", source.indexOf("impl ArtifactWal")), source.indexOf("//#endregion 🔖️ArtifactWal"));
    assert(artifactClose.includes("self.active.close_step()") && artifactClose.includes("self.active.terminal_is_empty()"), "artifact WAL must expose explicit terminal owner retirement");
    const segmentClose = source.slice(source.indexOf("fn close_step(&mut self)", source.indexOf("impl SegmentWriter")), source.indexOf("//#endregion 🔖️Segment"));
    assert(segmentClose.indexOf("self.writer.take()") < segmentClose.indexOf("buf.close_step()"), "segment close must relinquish the retained writer before retiring its page buffer");
    assert(segmentClose.includes("force_flush is required before close"), "segment close must reject pending records");
    const segmentFlush = source.slice(source.indexOf("async fn commit_and_flush", source.indexOf("impl SegmentWriter")), source.indexOf("async fn tip_chain_hash", source.indexOf("impl SegmentWriter")));
    assert(segmentFlush.includes("new_len != expected_len") && segmentFlush.includes("self.flushed_len = new_len"), "WAL flush must verify the exact appended length before acknowledging it");
    assert(segmentFlush.indexOf("self.flushed_len = new_len") < segmentFlush.indexOf("storage.sync"), "a failed sync must retain knowledge that its append already landed");
    assert(segmentFlush.includes("self.poison()"), "every uncertain post-commit failure must poison the live writer");
    const rotate = source.slice(source.indexOf("async fn rotate", source.indexOf("impl ArtifactWal")), source.indexOf("pub fn close_step", source.indexOf("impl ArtifactWal")));
    assert(rotate.indexOf("storage.seal") < rotate.indexOf("self.active.poison()"), "a sealed segment must poison its old live writer before successor creation");
    const laws = [
      "db_wal::tests::wal_recovery_preserves_neutral_committed_prefixes",
      "db_wal::tests::wal_recovery_matches_neutral_lifecycle_without_prefix_replacement",
      "db_wal::retained_tests::wal_replay_cancellation_remains_set_while_close_reaches_terminal_empty",
      "db_wal::retained_tests::artifact_wal_repeated_open_close_is_page_budget_neutral",
      "db_wal::retained_tests::artifact_wal_close_rejects_pending_records_and_closed_writes",
      "db_wal::retained_tests::artifact_wal_short_append_is_fail_stop_until_reopen",
      "db_wal::retained_tests::artifact_wal_append_error_is_fail_stop_until_reopen",
      "db_wal::retained_tests::artifact_wal_sync_error_is_fail_stop_until_reopen",
      "db_wal::retained_tests::artifact_wal_successor_failure_after_seal_is_fail_stop_until_reopen",
      "db_testkit::tests::fault_storage_fail_nth_sync_fails_once_after_the_preceding_append",
    ];
    laws.push(
      ...[
        "single_segment_write_commit_flush_recovers_cleanly",
        "group_commit_batches_until_policy_threshold_then_commits",
        "fsync_durability_forces_immediate_commit_regardless_of_policy",
        "torn_tail_is_recovered_by_truncating_only_the_uncommitted_suffix",
        "recovery_resumes_next_tx_id_and_accepts_further_submits",
        "multi_segment_rotation_chains_prev_hash_and_replay_spans_segments",
        "recovery_rejects_a_torn_non_active_sealed_segment",
        "empty_document_open_creates_a_fresh_wal",
      ].map((law) => `db_wal::tests::${law}`),
    );
    const testkitSource = readFileSync(join(owner, "../🧪️testkit/🦀️.rs"), "utf8");
    for (const law of laws) assert((law.startsWith("db_testkit::") ? testkitSource : source).includes(`fn ${law.split("::").at(-1)}(`), `missing exact native law ${law}`);
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, laws }],
        progress(event) {
          console.log(`wal-recovery ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`wal-recovery-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`wal-recovery-check: ${fixture.cuts.length + fixture.lifecycle.length + fixture.fragmentCopies.length + failStop.cases.length} checks clean`);
  }
}

/** 📏️ Proves complete transaction reservations fit the shared readable storage span. */
class WalCapacityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-capacity-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️fixtures/📏️capacity/🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧪️fixtures/📏️capacity/🧬️.schema.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
    const leb = await import("@webassemblyjs/leb128");
    const frame = (payload: number) => leb.encodeU32(payload + 2).length + payload + 10;
    const txn = (payload: number) => frame(8) + frame(payload) + frame(12);
    for (const row of fixture.cases) {
      const lengths = [129];
      const segments: number[] = [];
      let pending = false;
      for (let index = 0; index < 3; index++) {
        if (lengths.at(-1)! + txn(fixture.payloadBytes) + 75 > fixture.maxSegmentBytes) {
          if (pending) lengths[lengths.length - 1] += 75;
          lengths.push(161);
          pending = false;
        }
        segments.push(lengths.length - 1);
        lengths[lengths.length - 1] += txn(fixture.payloadBytes);
        pending = true;
        if (row.durability === "fsync") {
          lengths[lengths.length - 1] += 75;
          pending = false;
        }
      }
      if (pending) lengths[lengths.length - 1] += 75;
      assert.deepEqual(segments, row.segments);
      assert.deepEqual(lengths, row.lengths);
    }
    assert.equal(129 + txn(fixture.exactPayloadBytes) + 75, fixture.maxSegmentBytes);
    assert.equal(129 + txn(fixture.oversizedPayloadBytes) + 75, fixture.maxSegmentBytes + 1);
    console.log("wal-capacity-independent-oracle: Fsync/grouped rotation and exact/one-over capacity confirmed by LEB128");
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("fn wal_transaction_frame_bytes("), "missing transaction byte preflight before writes");
    assert(source.includes("const DEFAULT_MAX_SEGMENT_BYTES: u64 = db_storage::DB_IO_MAX_READ_BYTES;"), "WAL and storage must share one byte ceiling");
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, laws: ["db_wal::tests::wal_capacity_preflight_matches_neutral_memory_and_filesystem_boundaries"] }],
        progress(event) {
          console.log(`wal-capacity ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`wal-capacity-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log("wal-capacity-check: 6 checks clean");
  }
}

//#region 🔎️ScalarWireSource
class ScalarWireSourceScript extends BundleScript {
  async run(): Promise<void> {
    const { testScalarRecordWireFixture } = await import("../../🔨️modules/🎒️pack/🔎️scalar-witness/📜️script.ts");
    testScalarRecordWireFixture();
  }
}
//#endregion 🔎️ScalarWireSource

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

/** 🪪️ Executes the native outer opening-attempt wire law without broadening the browser patch contract. */
class DocumentOpeningAttemptNativeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("document-opening-attempt-native-check accepts no arguments");
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [
        {
          package: "semio-framework-os-kernel",
          target: { kind: "lib" },
          cargoArgs: ["--features", "sync"],
          laws: ["os_store::sync::tests::document_opening_attempt_wire_preserves_outer_owner_without_widening_actor_messages"],
        },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log(`document-opening-attempt-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`document-opening-attempt-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-os-kernel"], this.repoRoot, ["--lib", "--features", "sync,ureq", ...rest]);
  }
}

class DirectoryRuntimeSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-directory-runtime-source accepts no arguments");
    const { testDirectoryRuntimeIdentityFixture } = await import("../../🔨️modules/📇️directory/🔌️client/🪪️runtime/📜️script.ts");
    testDirectoryRuntimeIdentityFixture();
  }
}

/** 📃️ Proves the shared event-page envelope against an independent JSON Schema and SHA-256 oracle. */
export async function directoryEventPageContractOracle(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📃️event-page-v1.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8"));
  const validator = new Ajv2020({ strict: false, allErrors: true, discriminator: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/DirectoryEventPageV1" });
  assert(validator(fixture.valid), JSON.stringify(validator.errors));
  assert.equal(new TextEncoder().encode(fixture.canonicalUnsigned).length, 474);
  assert.equal(createHash("sha256").update(fixture.canonicalUnsigned).digest("hex"), fixture.expectedReceiptSha256);
  const contract = await import("../../🔨️modules/📇️directory/🧬️schema/🟦️.ts");
  const parsed = await contract.parseDirectoryEventPageV1(JSON.stringify(fixture.valid));
  assert.deepEqual(parsed, fixture.valid);
  const setPath = (value: any, path: string, replacement: unknown): any => {
    const copy = structuredClone(value);
    const parts = path.split(".");
    let parent = copy;
    for (const part of parts.slice(0, -1)) parent = parent[Number.isInteger(Number(part)) ? Number(part) : part];
    parent[parts.at(-1)!] = replacement;
    return copy;
  };
  for (const hostile of fixture.hostileMutations) await assert.rejects(() => contract.parseDirectoryEventPageV1(JSON.stringify(setPath(fixture.valid, hostile.path, hostile.value))), undefined, hostile.name);
  const canonical = JSON.stringify(fixture.valid);
  await assert.rejects(() => contract.parseDirectoryEventPageV1(`${canonical} `), undefined, "trailing-byte");
  await assert.rejects(() => contract.parseDirectoryEventPageV1(canonical.replace('{"schema":', '{"schema":"duplicate","schema":')), undefined, "duplicate-key");
  const rust = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"), "utf8");
  assert(rust.includes("pub struct DirectoryEventPageV1") && rust.includes("pub fn receipt_matches(&self) -> bool"), "Rust event-page contract missing");
  return 5 + fixture.hostileMutations.length + fixture.rawHostiles.length;
}

/** 🔌️ Proves both directory clients preserve one canonical event-page response and its bounded header. */
export async function directoryEventPageClientOracle(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📃️event-page-v1.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8"));
  const validator = new Ajv2020({ strict: false, allErrors: true, discriminator: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/DirectoryEventPageV1" });
  const canonical = JSON.stringify(fixture.valid);
  const accept = (raw: string, after: number) => {
    if (!Number.isSafeInteger(after) || after < 0 || new TextEncoder().encode(raw).byteLength > 65_536) throw new Error("client admission");
    const parsed = JSON.parse(raw);
    if (JSON.stringify(parsed) !== raw || !validator(parsed)) throw new Error("client admission");
    if (parsed.afterSeqExclusive !== after) throw new Error("frontier substitution");
    const { receiptSha256, ...unsigned } = parsed;
    if (createHash("sha256").update(JSON.stringify(unsigned)).digest("hex") !== receiptSha256) throw new Error("receipt substitution");
    return { canonicalJson: raw, throughSeqInclusive: parsed.throughSeqInclusive, receiptSha256: parsed.receiptSha256 };
  };
  const page = accept(canonical, 3);
  assert.equal(page.canonicalJson, canonical);
  assert.equal(page.throughSeqInclusive, fixture.valid.throughSeqInclusive);
  assert.equal(page.receiptSha256, fixture.expectedReceiptSha256);
  assert.throws(() => accept("x".repeat(65_537), 0));
  assert.throws(() => accept(canonical, 4));
  assert.throws(() => accept(canonical, -1));
  assert.throws(() => accept(`${canonical} `, 3));
  const typescript = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🟦️.ts"), "utf8");
  const method = typescript.slice(typescript.indexOf("async eventPage("), typescript.indexOf("stream(since", typescript.indexOf("async eventPage(")));
  assert(
    method.includes("response.text()") && method.includes("parseDirectoryEventPageV1(canonicalJson)") && method.includes("page.afterSeqExclusive !== after") && !method.includes("response.json()"),
    "TypeScript canonical page transport is incomplete",
  );
  assert(typescript.includes("streamAcknowledged(since:") && typescript.includes("acknowledge: (through: number)"), "TypeScript acknowledged directory frontier is missing");
  const rust = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8");
  assert(rust.includes("pub async fn event_page") && rust.includes("CanonicalDirectoryEventPageV1") && rust.includes("DIRECTORY_EVENT_PAGE_MAX_BYTES"), "Rust canonical page transport is incomplete");
  assert(rust.includes("pub fn stream_acknowledged") && rust.includes("pub fn acknowledge(&mut self, through: u64)"), "Rust acknowledged directory frontier is missing");
  assert(rust.includes("pub struct DirectoryEventPageBootstrapV1") && rust.includes("pub enum DirectoryBootstrapTransition"), "Rust directory bootstrap owner is missing");
  return 11;
}

class DirectoryEventPageContractCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-event-page-contract-check accepts only --native");
    const checks = await directoryEventPageContractOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["os_directory::schema::tests::directory_event_page_v1_matches_language_neutral_receipt_and_rejects_hostiles"] }],
        progress(event) {
          console.log(`directory-event-page-contract ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`directory-event-page-contract-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-event-page-contract-check: checks=${checks} clean`);
  }
}

class DirectoryEventPageClientCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-event-page-client-check accepts only --native");
    const checks = await directoryEventPageClientOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["os_directory::client::tests::directory_event_page_preserves_canonical_bytes_bounds_and_cancels_before_io"] }],
        progress(event) {
          console.log(`directory-event-page-client ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`directory-event-page-client-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-event-page-client-check: checks=${checks} clean`);
  }
}

/** 🧭️ Proves fetch, exact Home ACK, next-page, and live-cursor ordering independently of either shell. */
export function directoryEventPageBootstrapOracle(repoRoot: string): number {
  const trace = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🚀️event-page-bootstrap-v1.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🔗️event-page-bootstrap-v1.schema.json"), "utf8"));
  const validator = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validator(trace), JSON.stringify(validator.errors));
  let cursor = trace.initialAfter;
  let pending: any = null;
  let live = false;
  const present = (page: any): void => {
    assert(!pending && !live && page.afterSeqExclusive === cursor && page.throughSeqInclusive >= cursor);
    pending = page;
  };
  const acknowledge = (page: any, epoch = trace.bootstrapEpoch): "fetch" | "live" => {
    assert(pending && epoch === trace.bootstrapEpoch);
    for (const key of ["receiptSha256", "sessionBindingSha256", "authorizationGeneration", "throughSeqInclusive"]) assert.equal(page[key], pending[key]);
    cursor = pending.throughSeqInclusive;
    const hasMore = pending.hasMore;
    pending = null;
    live = !hasMore;
    return hasMore ? "fetch" : "live";
  };
  present(trace.pages[0]);
  assert.throws(() => present(trace.pages[1]), undefined, "page 2 before ACK");
  assert.throws(() => acknowledge({ ...trace.pages[0], receiptSha256: "d".repeat(64) }), undefined, "forged ACK");
  assert.equal(cursor, trace.initialAfter);
  assert.equal(acknowledge(trace.pages[0]), "fetch");
  present(trace.pages[1]);
  assert.throws(() => acknowledge(trace.pages[1], trace.bootstrapEpoch + 1), undefined, "stale epoch");
  assert.equal(acknowledge(trace.pages[1]), "live");
  for (const wakeup of trace.wakeups) assert(wakeup > cursor && cursor === trace.expectedSocketSince);
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "utf8");
  assert(worker.includes("DirectoryEventPageBootstrapV1") && worker.includes("directory-bootstrap-ack") && worker.includes("directory-event-page"), "browser worker bootstrap owner missing");
  return 11;
}

class DirectoryEventPageBootstrapCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length) throw new Error("directory-event-page-bootstrap-check accepts no arguments");
    console.log(`directory-event-page-bootstrap-check: checks=${directoryEventPageBootstrapOracle(this.repoRoot)} clean`);
  }
}

/** 🚦️ Proves every WAL backend exposes the same read-only active/sealed contract. */
export function walSegmentStateOracle(repoRoot: string): number {
  const storageRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db");
  const core = readFileSync(join(storageRoot, "🗄️storage/🦀️.rs"), "utf8");
  const sqlite = readFileSync(join(storageRoot, "🗄️storage/🪶️sqlite/🦀️.rs"), "utf8");
  const postgres = readFileSync(join(storageRoot, "🗄️storage/🐘️postgres/🦀️.rs"), "utf8");
  const neo4j = readFileSync(join(storageRoot, "🗄️storage/🌐️neo4j/🦀️.rs"), "utf8");
  const testkit = readFileSync(join(storageRoot, "🧪️testkit/🦀️.rs"), "utf8");
  assert(core.includes("pub enum WalSegmentState") && core.includes("Active,") && core.includes("Sealed,"));
  assert(core.includes("async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError>"));
  assert(core.includes("WalState { backend: DbIoBackendControl") && core.includes("WalSegmentState(WalSegmentState)"));
  assert(core.includes("Err(error) if error.kind() == std::io::ErrorKind::NotFound => WalSegmentState::Active") && core.includes("Err(error) => return Err(io_err(error))"));
  assert(sqlite.includes("SELECT sealed FROM wal_segment") && sqlite.includes("Err(DbError::Corrupt") && sqlite.includes("DbIoTask::WalState"));
  assert(postgres.includes("POSTGRES_WAL_STATE_QUERY") && postgres.includes("fetch_optional") && !postgres.slice(postgres.indexOf("const POSTGRES_WAL_STATE_QUERY"), postgres.indexOf("const POSTGRES_WAL_STATE_QUERY") + 240).includes("FOR UPDATE"));
  assert(neo4j.includes("const CYPHER_WAL_STATE") && neo4j.includes("RETURN n.sealed AS sealed") && !neo4j.slice(neo4j.indexOf("const CYPHER_WAL_STATE"), neo4j.indexOf("const CYPHER_WAL_STATE") + 240).includes("bytes"));
  assert(testkit.includes("async fn segment_state") && testkit.includes("fault_storage_segment_state_is_observational_and_counter_neutral"));
  return 12;
}

class WalSegmentStateCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("wal-segment-state-check accepts only --native");
    const checks = walSegmentStateOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        cargoArgs: ["--all-features"],
        groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib" },
            laws: [
              "memory_storage_satisfies_wal_storage_laws",
              "fs_storage_satisfies_wal_storage_laws",
              "fs_storage_stale_seal_marker_does_not_resurrect_missing_segment",
              "memory_storage_db_backend_accessors_and_capabilities",
              "wal_segment_state_observes_active_sealed_and_missing_rows",
              "wal_segment_state_decoder_rejects_non_boolean_storage_values",
              "wal_segment_state_query_and_mapper_are_read_only_and_byte_neutral",
              "wal_cypher_statements_reference_the_expected_label_and_keys",
              "fault_storage_segment_state_is_observational_and_counter_neutral",
            ],
          },
        ],
        progress(event) {
          console.log(`wal-segment-state ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`wal-segment-state-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`wal-segment-state-check: checks=${checks} clean`);
  }
}

class CodecSendSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-codec-send-source accepts no arguments");
    const { testNativeCodecSendFixture } = await import("../../🔨️modules/🏪️store/📦️codec/🧵️send/📜️script.ts");
    testNativeCodecSendFixture();
  }
}

class BackboneDetachSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-backbone-detach-source accepts no arguments");
    const { testBackboneDetachFixture } = await import("../../🔨️modules/🏪️store/🔗️backbone/✂️detach/📜️script.ts");
    testBackboneDetachFixture();
  }
}

class MemberDialectSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-member-dialect-source accepts no arguments");
    const { testMemberDialectFixture } = await import("../../🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/📜️script.ts");
    testMemberDialectFixture();
    const { testFixtureProjectionRetirement } = await import("../../🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/📜️script.ts");
    testFixtureProjectionRetirement();
  }
}

class MemberDialectCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { testMemberDialectFixture } = await import("../../🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/📜️script.ts");
    testMemberDialectFixture();
    const { testFixtureProjectionRetirement } = await import("../../🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/📜️script.ts");
    testFixtureProjectionRetirement();
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      cargoArgs: segments,
      buildBudgetMs: 3_600_000,
      groups: [
        {
          package: "semio-framework-schema",
          target: { kind: "lib" },
          laws: [
            "artifact_composition_fields_derive_emits_expected_slot_tables",
            "artifact_composition_fields_default_to_empty_for_leaf_artifacts",
            "artifact_composition_projection_walks_aliases_nested_options_and_cancels",
            "artifact_composition_projection_real_child_alias_has_fixed_admission_bounds",
          ],
        },
        {
          package: "semio-framework-os-kernel",
          target: { kind: "lib" },
          laws: [
            "member_factory_closed_dialect_matches_neutral_admission_corpus",
            "member_factory_closed_dialect_rejects_identity_and_owner_substitution",
            "member_factory_closed_dialect_graph_admission_matches_neutral_corpus",
            "member_factory_closed_dialect_graph_sync_preserves_prior_state_on_rejection",
            "member_factory_closed_dialect_parent_projection_matches_neutral_corpus",
            "initial_child_identity_matches_neutral_coordinates_and_blake3",
          ],
        },
        {
          package: "semio-framework-plugin",
          target: { kind: "lib" },
          laws: [
            "fixture_projection_retires_exact_tree_before_return_error_or_panic",
            "member_factory_parent_snapshot_restore_matches_neutral_corpus",
            "member_factory_closed_dialect_open_failure_retains_pin_and_drains_exact_member",
            "member_factory_closed_dialect_register_rejects_pin_without_mutating_member",
            "member_factory_closed_dialect_fresh_register_and_restore_publish_exact_parent_owner",
          ],
        },
      ],
    });
    console.log(`[DEBUG] exact member admission laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed across ${receipts.length} verified test executables`);
  }
}

//#region 🧩️JCO Package Adapter
class GenerateJcoPackageAdapterScript extends BundleScript {
  run(): void {
    runNestedCargoPackageAdapter(this.repoRoot, "generate");
  }
}
class PreviewGeneratedScript extends BundleScript {
  run(): void {
    runNestedCargoPackageAdapter(this.repoRoot, "preview");
  }
}
class CheckJcoPackageAdapterScript extends BundleScript {
  run(): void {
    runNestedCargoPackageAdapter(this.repoRoot, "check");
  }
}
//#endregion 🧩️JCO Package Adapter

const router = new ScriptRouter(import.meta.dir)
  .register("check", CheckScript)
  .register("test", TestScript)
  .register("document-opening-attempt-native-check", DocumentOpeningAttemptNativeCheckScript)
  .register("test-scalar-wire-source", ScalarWireSourceScript)
  .register("generate-jco-package-adapter", GenerateJcoPackageAdapterScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-jco-package-adapter", CheckJcoPackageAdapterScript)
  .register("test-native", NativeTestScript)
  .register("test-directory-runtime-source", DirectoryRuntimeSourceScript)
  .register("directory-event-page-contract-check", DirectoryEventPageContractCheckScript)
  .register("directory-event-page-client-check", DirectoryEventPageClientCheckScript)
  .register("directory-event-page-bootstrap-check", DirectoryEventPageBootstrapCheckScript)
  .register("wal-segment-state-check", WalSegmentStateCheckScript)
  .register("test-codec-send-source", CodecSendSourceScript)
  .register("test-backbone-detach-source", BackboneDetachSourceScript)
  .register("test-member-dialect-source", MemberDialectSourceScript)
  .register("member-dialect-check", MemberDialectCheckScript);

router.register("wal-recovery-check", WalRecoveryCheckScript);
router.register("wal-capacity-check", WalCapacityCheckScript);
router.register("wal-committed-transactions-check", WalCommittedTransactionsCheckScript);
router.register("wal-writer-authority-check", WalWriterAuthorityCheckScript);
router.register("database-history-completion-check", DatabaseHistoryCompletionCheckScript);
router.register("database-catalog-read-ownership-check", DatabaseCatalogReadOwnershipCheckScript);
router.register("database-capability-completion-check", DatabaseCapabilityCompletionCheckScript);
router.register("wal-committed-compaction-check", WalCommittedCompactionCheckScript);
router.register("database-shutdown-check", DatabaseShutdownCheckScript);
router.register("document-mount-single-flight-check", DocumentMountSingleFlightCheckScript);
router.register("durable-owned-group-decision-check", DurableOwnedGroupDecisionCheckScript);
router.register("durable-group-journal-check", DurableGroupJournalCheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
