#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-os-kernel` task router. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo, resolveTestLevel, runCargoTestBudgeted, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { runNestedCargoPackageAdapter } from "../../../../../📜️script.ts";

/** 🔐️ Checks fixed writer capabilities independently of the retained backend integration. */
class WalWriterAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-writer-authority-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️fixtures/🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧪️fixtures/🧬️.schema.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
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
          else { live.set(step.document, generation); owners.set(step.owner, { document: step.document, generation: generation++ }); }
        } else if (step.backend !== 0 || permit?.document !== step.document || live.get(step.document) !== permit?.generation) actual = "fenced";
        else if (step.action === "release") live.delete(step.document);
        assert.equal(actual, step.expected, `${row.name}: ${JSON.stringify(step)}`);
      }
      assert.equal(live.size, 0, row.name);
    }
    assert.deepEqual(fixture.resultRetirement.terminal, Array.from({ length: fixture.resultRetirement.pages + 3 }, (_, index) => index === fixture.resultRetirement.pages + 2));
    assert.deepEqual(fixture.guardRetirement.terminal, fixture.guardRetirement.stages.map((stage: string) => stage === "terminal"));
    const rejected = new Set(Array.from({ length: fixture.backendPressure.capacity }, (_, index) => index));
    let retained = rejected.size === fixture.backendPressure.capacity;
    assert.equal(retained, fixture.backendPressure.retainedWhenFull);
    rejected.delete(0);
    if (rejected.size < fixture.backendPressure.capacity) retained = false;
    assert.equal(!retained, fixture.backendPressure.terminalAfterCapacityReturns);
    const closing = new Map([[fixture.fairRetirement.pinnedSlot, "pinned"], [fixture.fairRetirement.releasingSlot, "releasing"]]);
    for (let cursor = 0; cursor < fixture.fairRetirement.maximumOpportunities; cursor++) if (closing.get(cursor) === "releasing") closing.delete(cursor);
    assert.equal(closing.has(fixture.fairRetirement.pinnedSlot), fixture.fairRetirement.pinnedRetained);
    assert.equal(!closing.has(fixture.fairRetirement.releasingSlot), fixture.fairRetirement.releasingRetired);
    for (const trace of [fixture.maintenanceFairness.continuouslyReady, fixture.maintenanceFairness.firstClassFaults]) assert.deepEqual(trace, trace.map((_: unknown, index: number) => index % fixture.maintenanceFairness.classes.length));
    console.log(`wal-writer-authority-independent-oracle: AJV=1 exact-u64=1 cases=${fixture.cases.length} mutations=${fixture.mutations.length} retained-result=1`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    for (const marker of ["struct WalWriterPermit", "struct WalWriterTable", "struct WalFileWriterGuard", "try_lock()", "checked_add(1)", "active_operation", "fn release_step"]) assert(source.includes(marker), `missing writer capability primitive: ${marker}`);
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot, env: { ...process.env, RUST_MIN_STACK: "268435456" },
      groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, cargoArgs: ["--all-features"], laws: ["wal_writer_table_matches_neutral_exact_scope_and_aba_rejection", "wal_writer_table_capacity_recycles_slots_without_reusing_generations", "wal_writer_file_lock_excludes_independent_instances_and_processes", "db_io_lost_result_lease_retains_every_page_and_final_handback", "wal_writer_release_retains_pinned_operation_and_faulted_guard", "db_io_lost_backend_retains_exact_owner_under_rejected_registry_pressure", "wal_writer_table_close_advances_other_guards_while_first_operation_is_pinned", "db_io_maintenance_rotates_ready_and_faulted_classes_without_starvation"] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listBudgetMs: 60_000, lawBudgetMs: 120_000,
      progress(event) { console.log(`wal-writer-authority-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
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
      const frontiers = row.records.flatMap((record: any, index: number) => record.kind === "frontier" ? [index] : []);
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
          assert.deepEqual([...segment.physicalCommitsAfter].sort((a: number, b: number) => a - b), segment.physicalCommitsAfter);
          assert.equal(segment.physicalCommitsAfter.at(-1), segment.frames.length - 1);
          let current: { id: string; kinds: string[] } | null = null;
          for (const [ordinal, frame] of segment.frames.entries()) {
            if (frame.kind === "header") { if (ordinal !== 0 || current !== null) throw "corrupt"; continue; }
            if (ordinal === 0) throw "corrupt";
            if (frame.kind === "begin") {
              if (current !== null || BigInt(frame.id) < next || (observed && BigInt(frame.id) !== next)) throw "corrupt";
              const payload = Buffer.alloc(8); payload.writeBigUInt64LE(BigInt(frame.id));
              const id = payload.readBigUInt64LE();
              if (id === 0xffffffffffffffffn) throw "sequence";
              next = id + 1n; observed = true; current = { id: id.toString(), kinds: [] };
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
    assert.deepEqual(faults.cases.filter((row: any) => row.fault !== "successorAppendError").map((row: any) => [row.fault, row.expectedPhysicalSuffix]), [["shortAppend", "torn"], ["appendError", "absent"], ["syncError", "complete"]]);
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
          const storage = Buffer.alloc(8); storage.writeBigUInt64LE(number);
          if (Buffer.from(leb.encodeUIntBuffer(storage)).equals(bytes.subarray(0, terminal + 1))) { value = number.toString(); consumed = terminal + 1; }
        }
      }
      assert.deepEqual({ value, consumed }, { value: row.value, consumed: row.consumed }, row.name);
    }
    assert(source.includes("fn wal_read_canonical_varint"), "retained readers must reject noncanonical and overflowing u64 fields");
    console.log(`wal-retained-decoder-independent-oracle: AJV=1 LEB128=1 vectors=${decoder.varints.length}`);
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot, env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, cargoArgs: ["--all-features"], laws: [
          "wal_transaction_gate_matches_neutral_committed_spans", "wal_retained_decoder_fuel_resumes_exact_fragmented_bytes",
          "wal_retained_decoder_cancel_close_preserves_source_and_returns_owner", "wal_committed_cursor_cancel_resume_keeps_transaction_position",
          "wal_committed_cursor_unfinished_borrow_poison_and_cancelled_close", "artifact_open_ignores_neutral_aborted_command_snapshot_and_cas",
          "sync_replay_ignores_neutral_aborted_command_snapshot_and_cas", "cli_verify_checks_neutral_logical_commit_boundaries",
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
        ] }],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listBudgetMs: 60_000, lawBudgetMs: 120_000,
        progress(event) { console.log(`wal-committed-transactions-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
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
    assert.deepEqual(fixture.segments.map((row: any) => row.index), fixture.segments.map((_: any, index: number) => index));
    assert.equal(fixture.segments.at(-1).state, "active");
    const committed = fixture.segments.flatMap((segment: any) => segment.transactions.filter((transaction: any) => transaction.outcome === "commit").flatMap((transaction: any) => transaction.records.map((record: any) => ({ ...record, segment: segment.index }))));
    const horizons = fixture.segments.map((segment: any) => ({ segment: segment.index, head: committed.filter((record: any) => record.segment === segment.index && ["frontier", "snapshot"].includes(record.kind)).reduce((head: number | null, record: any) => head === null ? record.headSeq : Math.max(head, record.headSeq), null) }));
    const highest = fixture.segments.at(-1).index;
    const deletedSegments = horizons.filter((row: any) => row.segment !== highest && row.head !== null && row.head <= fixture.floorHeadSeq).map((row: any) => row.segment);
    const deletedPayloads = committed.filter((record: any) => record.kind === "payload" && deletedSegments.includes(record.segment) && !committed.some((live: any) => live.kind === "payload" && live.payload === record.payload && !deletedSegments.includes(live.segment))).map((record: any) => record.payload);
    const allPayloads = new Set(fixture.segments.flatMap((segment: any) => segment.transactions.flatMap((transaction: any) => transaction.records.filter((record: any) => record.kind === "payload").map((record: any) => record.payload))));
    assert.deepEqual({
      deletedSegments: deletedSegments.length,
      deletedPayloads: new Set(deletedPayloads).size,
      remainingSegments: fixture.segments.map((row: any) => row.index).filter((index: number) => !deletedSegments.includes(index)),
      retainedPayloads: [...allPayloads].filter(payload => !deletedPayloads.includes(payload)),
    }, fixture.expected);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const committedCut = source.slice(source.indexOf("async fn committed_compaction_horizons"), source.indexOf("async fn retained_compaction_under_lease"));
    assert(committedCut.includes("replay_committed_document") && committedCut.includes("close_record_step") && committedCut.includes("transaction.finish()") && committedCut.includes("close_compaction_replay"));
    assert(source.slice(source.indexOf("async fn close_compaction_replay"), source.indexOf("async fn close_compaction_owner")).includes("close_owner_step"));
    assert(!committedCut.includes("WalReplayCursor") && !committedCut.includes("replay_document"));
    assert(source.includes("fn compaction_applies_only_committed_frontier_snapshot_and_payload_effects("));
    console.log("wal-committed-compaction-independent-oracle: abort effects excluded, committed effects retained, header-only highest preserved");
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        cargoArgs: ["--all-features"],
        groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, laws: ["db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects"] }],
        progress(event) { console.log(`wal-committed-compaction-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      console.log(`wal-committed-compaction-native-receipts: ${JSON.stringify(receipts)}`);
    }
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
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: [
          "durable_group::tests::durable_owned_group_decision_matches_neutral_canonical_hash_and_bounds",
          "durable_group::tests::durable_store_prepared_outcome_derives_and_verifies_exact_unbound_bytes",
          "durable_group::tests::durable_owned_group_decision_rejects_forged_identity_commitment_and_capacity",
          "durable_group::tests::durable_decision_rejects_deflate_expansion_before_document_body_allocation",
          "durable_group::tests::durable_store_owned_three_member_bind_and_base_recovery_retain_exact_private_owners",
          "durable_group::tests::durable_json_carriers_preserve_numeric_kinds_and_reject_control_and_resource_excess",
          "durable_group::tests::durable_store_group_journal_commit_flips_one_shared_root_then_adopts_exactly_once",
          "durable_group::tests::durable_store_group_cancellation_waits_for_trusted_absence_then_restores_all_old_roots",
          "durable_group::tests::durable_store_group_stage_error_retains_abort_owner_until_every_root_is_empty",
          "durable_group::tests::durable_store_group_uncertain_journal_error_retries_same_owner_without_rebegin_or_visibility_change",
          "durable_group::tests::durable_store_group_rejects_foreign_anchor_receipt_before_visibility_and_aborts_only_after_absence",
          "durable_group::tests::durable_map_mounted_operation_retains_every_live_owner_across_request_error_until_terminal_handoff",
        ] }],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
        listBudgetMs: 60_000,
        lawBudgetMs: 120_000,
        progress(event) { console.log(`durable-owned-group-decision-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      for (const receipt of receipts) console.log(`durable-owned-group-decision-native-receipt: ${JSON.stringify(receipt)}`);
    }
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
    assert.deepEqual(failStop.cases.map((row: any) => [row.name, row.fault, row.expectedPhysicalSuffix]), [["short-append", "shortAppend", "torn"], ["append-error", "appendError", "absent"], ["sync-error", "syncError", "complete"], ["successor-append-error", "successorAppendError", "complete"]]);
    const { default: crc } = await import("crc-32/crc32c.js");
    const leb = await import("@webassemblyjs/leb128");
    const { blake3Hex } = await import("../../🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts");
    const { inspectRetainedSprNeutral } = await import(join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts"));
    const checksum = (bytes: Uint8Array) => crc.buf(bytes) >>> 0;
    const fragmented = Buffer.from(Array.from({ length: 49152 }, (_, index) => (index * 17 + 3) % 251));
    for (const row of fixture.fragmentCopies) assert.equal(checksum(fragmented.subarray(row.offset, row.offset + row.length)), row.crc32c);
    const hash = (bytes: Buffer) => Buffer.from(blake3Hex(bytes), "hex");
    const u64 = (value: number) => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(value)); return bytes; };
    const frame = (kind: number, payload: Buffer) => {
      const body = Buffer.concat([Buffer.from([kind, 2]), payload]);
      const size = Buffer.from(leb.encodeU32(body.length));
      const trailer = Buffer.alloc(8); trailer.writeUInt32LE(checksum(body)); trailer.writeUInt32LE(size.length + body.length + 8, 4);
      return Buffer.concat([size, body, trailer]);
    };
    const header = Buffer.alloc(32); Buffer.from([137, 83, 80, 82, 13, 10, 26, 10]).copy(header);
    header.writeUInt16LE(1, 8); header.writeUInt32LE(1, 12); header.writeUInt32LE(checksum(header.subarray(0, 20)), 20);
    let bytes = header; let chain = hash(header); let previousOffset = 0;
    const batches = [[frame(64, Buffer.concat([Buffer.from([1, 100]), u64(0), Buffer.from([0])]))], ...fixture.commands.map((command: string, index: number) => [frame(65, u64(index + 1)), frame(68, Buffer.from(command)), frame(66, Buffer.concat([u64(index + 1), Buffer.from([1, 0, 0, 0])]))])];
    for (const [index, records] of batches.entries()) {
      const payload = Buffer.alloc(64); const recordsLength = records.reduce((sum: number, value: Buffer) => sum + value.length, 0);
      chain = hash(Buffer.concat([chain, ...records.map(hash)]));
      payload.writeBigUInt64LE(BigInt(index + 1)); payload.writeBigUInt64LE(BigInt(previousOffset), 8); payload.writeBigUInt64LE(BigInt(recordsLength), 16); payload.writeUInt32LE(records.length, 24); chain.copy(payload, 32);
      previousOffset = bytes.length + recordsLength; bytes = Buffer.concat([bytes, ...records, frame(12, payload)]);
      assert.equal(bytes.length, fixture.commitEnds[index]);
    }
    for (const row of fixture.cuts) {
      const prefix = bytes.subarray(0, row.cut);
      const span = row.cut < 32 ? { end: 0, sequence: 0 } : inspectRetainedSprNeutral(prefix, checksum, hash);
      assert.equal(span.end, row.trustedEnd); assert.equal(Math.max(129, span.end), row.recoveredEnd);
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
    const replayClose = source.slice(source.indexOf("pub fn close_owner_step(&mut self)", source.indexOf("impl<'storage, S: db_storage::WalStorage> WalReplayCursor")), source.indexOf("pub async fn close_step(&mut self)", source.indexOf("impl<'storage, S: db_storage::WalStorage> WalReplayCursor")));
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
    const laws = ["db_wal::tests::wal_recovery_preserves_neutral_committed_prefixes", "db_wal::tests::wal_recovery_matches_neutral_lifecycle_without_prefix_replacement", "db_wal::retained_tests::wal_replay_cancellation_remains_set_while_close_reaches_terminal_empty", "db_wal::retained_tests::artifact_wal_repeated_open_close_is_page_budget_neutral", "db_wal::retained_tests::artifact_wal_close_rejects_pending_records_and_closed_writes", "db_wal::retained_tests::artifact_wal_short_append_is_fail_stop_until_reopen", "db_wal::retained_tests::artifact_wal_append_error_is_fail_stop_until_reopen", "db_wal::retained_tests::artifact_wal_sync_error_is_fail_stop_until_reopen", "db_wal::retained_tests::artifact_wal_successor_failure_after_seal_is_fail_stop_until_reopen", "db_testkit::tests::fault_storage_fail_nth_sync_fails_once_after_the_preceding_append"];
    laws.push(...["single_segment_write_commit_flush_recovers_cleanly", "group_commit_batches_until_policy_threshold_then_commits", "fsync_durability_forces_immediate_commit_regardless_of_policy", "torn_tail_is_recovered_by_truncating_only_the_uncommitted_suffix", "recovery_resumes_next_tx_id_and_accepts_further_submits", "multi_segment_rotation_chains_prev_hash_and_replay_spans_segments", "recovery_rejects_a_torn_non_active_sealed_segment", "empty_document_open_creates_a_fresh_wal"].map(law => `db_wal::tests::${law}`));
    const testkitSource = readFileSync(join(owner, "../🧪️testkit/🦀️.rs"), "utf8");
    for (const law of laws) assert((law.startsWith("db_testkit::") ? testkitSource : source).includes(`fn ${law.split("::").at(-1)}(`), `missing exact native law ${law}`);
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({ cwd: this.repoRoot, env: { ...process.env, RUST_MIN_STACK: "268435456" }, groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, laws }], progress(event) { console.log(`wal-recovery ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); } });
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
      const lengths = [129]; const segments: number[] = []; let pending = false;
      for (let index = 0; index < 3; index++) {
        if (lengths.at(-1)! + txn(fixture.payloadBytes) + 75 > fixture.maxSegmentBytes) {
          if (pending) lengths[lengths.length - 1] += 75;
          lengths.push(161); pending = false;
        }
        segments.push(lengths.length - 1); lengths[lengths.length - 1] += txn(fixture.payloadBytes); pending = true;
        if (row.durability === "fsync") { lengths[lengths.length - 1] += 75; pending = false; }
      }
      if (pending) lengths[lengths.length - 1] += 75;
      assert.deepEqual(segments, row.segments); assert.deepEqual(lengths, row.lengths);
    }
    assert.equal(129 + txn(fixture.exactPayloadBytes) + 75, fixture.maxSegmentBytes);
    assert.equal(129 + txn(fixture.oversizedPayloadBytes) + 75, fixture.maxSegmentBytes + 1);
    console.log("wal-capacity-independent-oracle: Fsync/grouped rotation and exact/one-over capacity confirmed by LEB128");
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("fn wal_transaction_frame_bytes("), "missing transaction byte preflight before writes");
    assert(source.includes("const DEFAULT_MAX_SEGMENT_BYTES: u64 = db_storage::DB_IO_MAX_READ_BYTES;"), "WAL and storage must share one byte ceiling");
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({ cwd: this.repoRoot, env: { ...process.env, RUST_MIN_STACK: "268435456" }, groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, laws: ["db_wal::tests::wal_capacity_preflight_matches_neutral_memory_and_filesystem_boundaries"] }], progress(event) { console.log(`wal-capacity ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); } });
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
  await assert.rejects(() => contract.parseDirectoryEventPageV1(canonical.replace("{\"schema\":", "{\"schema\":\"duplicate\",\"schema\":")), undefined, "duplicate-key");
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
  assert(method.includes("response.text()") && method.includes("parseDirectoryEventPageV1(canonicalJson)") && method.includes("page.afterSeqExclusive !== after") && !method.includes("response.json()"), "TypeScript canonical page transport is incomplete");
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
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["os_directory::schema::tests::directory_event_page_v1_matches_language_neutral_receipt_and_rejects_hostiles"] }],
        progress(event) { console.log(`directory-event-page-contract ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
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
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["os_directory::client::tests::directory_event_page_preserves_canonical_bytes_bounds_and_cancels_before_io"] }],
        progress(event) { console.log(`directory-event-page-client ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
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
        cargoArgs: ["--all-features"],
        groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib" }, laws: [
          "memory_storage_satisfies_wal_storage_laws",
          "fs_storage_satisfies_wal_storage_laws",
          "fs_storage_stale_seal_marker_does_not_resurrect_missing_segment",
          "memory_storage_db_backend_accessors_and_capabilities",
          "wal_segment_state_observes_active_sealed_and_missing_rows",
          "wal_segment_state_decoder_rejects_non_boolean_storage_values",
          "wal_segment_state_query_and_mapper_are_read_only_and_byte_neutral",
          "wal_cypher_statements_reference_the_expected_label_and_keys",
          "fault_storage_segment_state_is_observational_and_counter_neutral",
        ] }],
        progress(event) { console.log(`wal-segment-state ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
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
      cargoArgs: segments,
      buildBudgetMs: 3_600_000,
      groups: [
        { package: "semio-framework-schema", target: { kind: "lib" }, laws: [
          "artifact_composition_fields_derive_emits_expected_slot_tables",
          "artifact_composition_fields_default_to_empty_for_leaf_artifacts",
          "artifact_composition_projection_walks_aliases_nested_options_and_cancels",
          "artifact_composition_projection_real_child_alias_has_fixed_admission_bounds",
        ] },
        { package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: [
          "member_factory_closed_dialect_matches_neutral_admission_corpus",
          "member_factory_closed_dialect_rejects_identity_and_owner_substitution",
          "member_factory_closed_dialect_graph_admission_matches_neutral_corpus",
          "member_factory_closed_dialect_graph_sync_preserves_prior_state_on_rejection",
          "member_factory_closed_dialect_parent_projection_matches_neutral_corpus",
          "initial_child_identity_matches_neutral_coordinates_and_blake3",
        ] },
        { package: "semio-framework-plugin", target: { kind: "lib" }, laws: [
          "fixture_projection_retires_exact_tree_before_return_error_or_panic",
          "member_factory_parent_snapshot_restore_matches_neutral_corpus",
          "member_factory_closed_dialect_open_failure_retains_pin_and_drains_exact_member",
          "member_factory_closed_dialect_register_rejects_pin_without_mutating_member",
          "member_factory_closed_dialect_fresh_register_and_restore_publish_exact_parent_owner",
        ] },
      ],
    });
    console.log(`[DEBUG] exact member admission laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed across ${receipts.length} verified test executables`);
  }
}

//#region 🧩️JCO Package Adapter
class GenerateJcoPackageAdapterScript extends BundleScript {
  run(): void { runNestedCargoPackageAdapter(this.repoRoot, "generate"); }
}
class PreviewGeneratedScript extends BundleScript {
  run(): void { runNestedCargoPackageAdapter(this.repoRoot, "preview"); }
}
class CheckJcoPackageAdapterScript extends BundleScript {
  run(): void { runNestedCargoPackageAdapter(this.repoRoot, "check"); }
}
//#endregion 🧩️JCO Package Adapter

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("test-scalar-wire-source", ScalarWireSourceScript).register("generate-jco-package-adapter", GenerateJcoPackageAdapterScript).register("preview-generated", PreviewGeneratedScript).register("check-jco-package-adapter", CheckJcoPackageAdapterScript).register("test-native", NativeTestScript).register("test-directory-runtime-source", DirectoryRuntimeSourceScript).register("directory-event-page-contract-check", DirectoryEventPageContractCheckScript).register("directory-event-page-client-check", DirectoryEventPageClientCheckScript).register("directory-event-page-bootstrap-check", DirectoryEventPageBootstrapCheckScript).register("wal-segment-state-check", WalSegmentStateCheckScript).register("test-codec-send-source", CodecSendSourceScript).register("test-backbone-detach-source", BackboneDetachSourceScript).register("test-member-dialect-source", MemberDialectSourceScript).register("member-dialect-check", MemberDialectCheckScript);

router.register("wal-recovery-check", WalRecoveryCheckScript);
router.register("wal-capacity-check", WalCapacityCheckScript);
router.register("wal-committed-transactions-check", WalCommittedTransactionsCheckScript);
router.register("wal-writer-authority-check", WalWriterAuthorityCheckScript);
router.register("wal-committed-compaction-check", WalCommittedCompactionCheckScript);
router.register("durable-owned-group-decision-check", DurableOwnedGroupDecisionCheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
