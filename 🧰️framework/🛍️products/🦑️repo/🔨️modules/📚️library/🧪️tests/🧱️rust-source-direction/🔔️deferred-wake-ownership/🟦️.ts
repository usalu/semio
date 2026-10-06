import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import ts from "typescript";

interface Fixture { readonly schemaVersion: 1; readonly neutralFixture: string; readonly neutralRouter: string; readonly writerFixture: string; readonly writerRouter: string; readonly caseIds: readonly string[]; readonly neutralLaws: readonly string[]; readonly writerLaws: readonly string[]; readonly deniedNeutralKeys: readonly string[]; readonly writerMarkers: readonly string[]; readonly neutralMarkers: readonly string[] }
const root = resolve(import.meta.dir, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(new URL("../../../🧫️fixtures/🧱️rust-source-direction/🔔️deferred-wake-ownership/🔣️.json", import.meta.url), "utf8")) as Fixture;

const source = (path: string): string => readFileSync(resolve(root, path), "utf8");
const json = (path: string): any => JSON.parse(source(path));
function owner(path: string, name: string): ts.ClassDeclaration {
  const tree = ts.createSourceFile(path, source(path), ts.ScriptTarget.Latest, true);
  return tree.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === name) as ts.ClassDeclaration;
}
function nativeGroups(node: ts.Node, executor: string): unknown[] {
  const groups: unknown[] = [];
  const literal = (value: ts.Node): unknown => {
    if (ts.isStringLiteral(value)) return value.text;
    if (ts.isArrayLiteralExpression(value)) return value.elements.map(literal);
    if (ts.isObjectLiteralExpression(value)) return Object.fromEntries(value.properties.map(property => {
      if (!ts.isPropertyAssignment(property)) throw Error("Nonliteral native group property");
      return [property.name.getText(), literal(property.initializer)];
    }));
    throw Error("Nonliteral native group authority");
  };
  const visit = (child: ts.Node): void => {
    if (ts.isCallExpression(child) && ts.isIdentifier(child.expression) && child.expression.text === executor) {
      expect(child.arguments).toHaveLength(1);
      const options = child.arguments[0]!;
      if (!ts.isObjectLiteralExpression(options)) throw Error("Nonliteral native executor options");
      const roster = options.properties.filter(property => ts.isPropertyAssignment(property) && property.name.getText() === "groups") as ts.PropertyAssignment[];
      expect(roster).toHaveLength(1);
      if (!ts.isArrayLiteralExpression(roster[0]!.initializer)) throw Error("Nonliteral native executor groups");
      groups.push(...roster[0]!.initializer.elements.map(literal));
    }
    ts.forEachChild(child, visit);
  };
  visit(node);
  return groups;
}

test("closed deferred wake ownership corpus retains all original controller and scheduler witnesses", () => {
  
  expect(fixture["schemaVersion"]).toEqual(1);expect(fixture["neutralFixture"]).toEqual("🧰️framework/🔨️modules/⏳️async/🔔️deferred-wake/🧫️fixtures/🔣️.json");expect(fixture["neutralSchema"]).toEqual("🧰️framework/🔨️modules/⏳️async/🔔️deferred-wake/🧬️schema/🔣️.json");expect(fixture["neutralRouter"]).toEqual("🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📜️script.ts");expect(fixture["writerFixture"]).toEqual("🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🧫️fixtures/🔔️deferred-wake/🔣️.json");expect(fixture["writerSchema"]).toEqual("🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🧬️schema/🔔️deferred-wake/🔣️.json");expect(fixture["writerRouter"]).toEqual("🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts");expect(fixture["caseIds"]).toEqual(["cross-key-refusal","retry-supersedes-self","maintenance-capacity-independent","shutdown-drains-accepted","terminal-shutdown-restores-owner"]);expect(fixture["neutralLaws"]).toEqual(["deferred_wake::tests::worker_deferred_wake_matches_neutral_capacity_generation_and_shutdown_drain","native_pool::tests::worker_deferred_wake_native_never_runs_inline_and_shutdown_drains_accepted_owner","wasm_pool::cooperative_tests::worker_deferred_wake_cooperative_shutdown_requires_later_pump_to_drain"]);expect(fixture["writerLaws"]).toEqual(["db_io_real_storage_open_drop_retires_queued_backend_and_allows_reopen","db_io_real_storage_open_fault_drop_retires_registered_backend_without_retry","wal_writer_table_matches_neutral_exact_scope_and_aba_rejection","wal_writer_table_capacity_recycles_slots_without_reusing_generations","wal_writer_file_lock_excludes_independent_instances_and_processes","db_io_lost_result_lease_retains_every_page_and_final_handback","wal_writer_release_retains_pinned_operation_and_faulted_guard","db_io_lost_backend_retains_exact_owner_under_rejected_registry_pressure","wal_writer_table_close_advances_other_guards_while_first_operation_is_pinned","db_io_maintenance_rotates_ready_and_faulted_classes_without_starvation","wal_writer_release_signal_preserves_exact_waits_across_writer_and_backend_reuse","wal_writer_mounted_controller_fences_at_signal_and_wakes_outside_registry_without_tasks","wal_writer_mounted_controller_fault_returns_exact_retry_owner_without_poisoning_other_writer","wal_writer_mounted_stale_controller_defers_cross_key_wake_and_fences_retry_epoch","wal_writer_mounted_controller_rerequests_after_async_executor_handback","wal_writer_mounted_controller_coalesced_fault_does_not_strand_healthy_release","wal_writer_mounted_controller_outer_panic_faults_waiters_once_and_stops","db_io_memory_backend_heap_tables_have_exact_preflight_credit_and_terminal_return","db_io_retained_page_results_survive_same_task_slot_reuse_and_return_exact_credit","fs_storage_canonical_alias_writer_fences_all_six_mutations","sqlite_wal_writer_real_database_alias_and_crash_are_exclusive","fs_wal_directory_barriers_match_neutral_order_and_duplicate_create_is_atomic","fs_wal_directory_faults_retain_seal_and_delete_order_until_explicit_retry","fs_replacement_reports_failure_until_renamed_parent_is_synced","fs_wal_reopen_repairs_unacknowledged_segment_namespace_before_header_ack","replicate_document_fences_occupied_follower_before_inventory_or_up_to_date","replicate_document_releases_follower_after_leader_replay_failure","replicate_document_applies_missing_tail_commands_to_a_fresh_follower","replicate_document_reports_up_to_date_once_a_follower_catches_up","replicate_document_transfers_a_snapshot_when_the_follower_is_below_the_retained_floor","artifact_wal_open_rejection_retains_exact_writer_for_close_or_same_owner_retry","artifact_engine_create_rejection_propagates_exact_wal_release_owner","database_document_mount_failure_terminalizes_authority_builder_wal_owner_before_fanout","db_io_registered_backend_use_blocks_pool_shutdown_until_terminal_close","db_io_backend_registration_saturation_returns_exact_executor_before_pool_use","db_io_prepared_registration_failure_returns_exact_close_owner_after_submission_refusal","db_io_task_uses_registered_backend_pool_not_caller_pool","db_io_backend_drop_retains_pool_until_deferred_close_terminal","db_io_forged_backend_kind_is_rejected_before_task_page_admission","database_compaction_future_acquires_pool_use_before_admission"]);expect(fixture["deniedNeutralKeys"]).toEqual(["backendControls","writersPerBackend","waitersPerWriter","retainedFaults","retainedGuards","terminalEpochs","dbTasks","activeRequested"]);expect(fixture["writerMarkers"]).toEqual(["deferred_wake_ticket","defer_wake","restore_fault_waiter","defer_fault_notifications"]);
  
  for (const candidate of [{ ...fixture, unknown: true }, { ...fixture, caseIds: fixture.caseIds.slice(1) }, { ...fixture, writerLaws: fixture.writerLaws.slice(1) }, { ...fixture, neutralLaws: [...fixture.neutralLaws.slice(1), fixture.neutralLaws[1]] }, { ...fixture, neutralMarkers: [...fixture.neutralMarkers.slice(1), "changed-marker"] }]) {
    
    
  }
});

test("general deferred wake and specific writer have closed separate current fixture authority", () => {
  const neutral = json(fixture.neutralFixture), writer = json(fixture.writerFixture);
  for (const corpus of [neutral, writer]) expect(corpus.cases.map((row: { id: string }) => row.id)).toEqual([...fixture.caseIds]);
  for (const key of fixture.deniedNeutralKeys) expect(source(fixture.neutralFixture)).not.toContain(JSON.stringify(key));
  expect(Object.hasOwn(neutral, "refusal")).toBe(false);
  expect(Object.hasOwn(neutral, "retryEpoch")).toBe(false);
  expect(source(fixture.neutralRouter)).not.toContain("🛍️products");
  const lower = owner(fixture.neutralRouter, "WorkerDeferredWakeCheckScript"), higherOwner = owner(fixture.writerRouter, "WalWriterAuthorityCheckScript"), higher = higherOwner.getText();
  expect(nativeGroups(lower, "runExactCargoLaws")).toEqual([{ package: "semio-framework-async", target: { kind: "lib", name: "semio_framework_async" }, laws: [...fixture.neutralLaws] }]);
  expect(nativeGroups(higherOwner, "runRepositoryExactCargoLaws")).toEqual([{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, cargoArgs: ["--all-features"], laws: [...fixture.writerLaws] }]);
  expect(higher).toContain("writerDeferredWake.cases");
  expect(higher).toContain("writerDeferredWake.retryEpoch");
  expect(writer.runtimeMarkers.writer).toEqual([...fixture.writerMarkers]);
  expect(neutral.runtimeMarkers.async).toEqual([...fixture.neutralMarkers]);
  expect(Object.hasOwn(neutral.runtimeMarkers, "writer")).toBe(false);
  console.log(`[DEBUG] deferred-wake-owned-authorities neutral=5 writer=5 native=${fixture.neutralLaws.length}+${fixture.writerLaws.length}`);
});
