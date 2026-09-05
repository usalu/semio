import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";

const sha256 = (bytes: Uint8Array | string): string => createHash("sha256").update(bytes).digest("hex");
const bytes = (hex: string): Buffer => {
  assert.match(hex, /^(?:[0-9a-f]{2})+$/);
  return Buffer.from(hex, "hex");
};
const revision = (hex: string): number[] => [...bytes(hex)];

export function testDurableOwnedGroupDecisionFixture(): void {
  const owner = import.meta.dir;
  const fixture = JSON.parse(readFileSync(join(owner, "🧪️fixtures/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(owner, "🧪️fixtures/🧬️.schema.json"), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const member = (role: "parent" | "drawing" | "value") => {
    const row = fixture.members[role];
    const callerRecovery = bytes(row.callerRecoveryPackHex);
    assert.equal(sha256(callerRecovery), row.callerRecoveryPackSha256, `${role} hostile caller recovery hash`);
    assert.equal(row.callerRecoveryExpected, "reject");
    assert.notEqual(callerRecovery.subarray(0, 8).toString("hex"), "8953454d0d0a1a0a", `${role} caller bytes are not a typed Store recovery Pack`);
    assert.equal(sha256(bytes(row.unboundOutcomePackHex)), row.unboundOutcomeSha256, `${role} Store-owned unbound outcome pack hash`);
    return {
      role: row.role,
      reference: row.reference,
      owner: row.owner,
      expectedGeneration: row.expectedGeneration,
      expectedRevision: revision(row.expectedRevisionHex),
      recoverySchema: row.recoverySchema,
      unboundOutcomeSha256: row.unboundOutcomeSha256,
    };
  };
  const unsigned = { schema: fixture.schemas.decision, anchor: fixture.anchor, parent: member("parent"), drawing: member("drawing"), value: member("value") };
  const unsignedJson = JSON.stringify(unsigned);
  assert.equal(JSON.stringify(fixture.anchor), JSON.stringify(unsigned.anchor));
  assert.equal(sha256(JSON.stringify(fixture.anchor)), fixture.expected.anchorSha256);
  assert.equal(unsignedJson, fixture.expected.unsignedJson);
  assert.equal(sha256(unsignedJson), fixture.expected.decisionSha256);
  assert(Buffer.byteLength(unsignedJson) <= fixture.limits.structuralIdentityBytes);
  assert.equal(fixture.limits.eventBytes + fixture.limits.walFramingReserveBytes, fixture.limits.walSegmentBytes);
  assert(3 * fixture.limits.recoveryPackBytes + fixture.limits.structuralIdentityBytes <= fixture.limits.eventBytes);
  const varintBytes = (value: number): number => value < 1 << 7 ? 1 : value < 1 << 14 ? 2 : value < 1 << 21 ? 3 : value < 1 << 28 ? 4 : 5;
  const sprFrameBytes = (payload: number): number => varintBytes(payload + 2) + payload + 10;
  const exactWholeSegmentBytes = 129 + sprFrameBytes(8) + sprFrameBytes(fixture.limits.eventBytes) + sprFrameBytes(12) + 75;
  assert.equal(exactWholeSegmentBytes, 491779);
  assert(exactWholeSegmentBytes <= fixture.limits.walSegmentBytes);
  assert.deepEqual(fixture.cases.map((row: any) => row.id), [
    "image-free-create-region-commits",
    "same-decision-replay-is-idempotent",
    "forged-owned-child-rejected-before-journal",
    "bound-edit-derivations-are-not-an-unsigned-preimage",
    "only-none-to-decision-group-binds",
    "tampered-outcome-or-commitment-rejected",
    "mixed-recovery-is-corrupt-not-compensated",
    "capacity-and-base-fence",
  ]);
  assert.deepEqual(fixture.cases[2].variants, ["drawing-child-id", "drawing-dialect", "drawing-owner-slot", "drawing-owner-parent"]);
  assert.deepEqual(fixture.cases[7].variants, ["recovery-pack-162001", "event-pack-491521", "changed-expected-generation", "changed-expected-revision"]);
  assert.deepEqual(fixture.carrier.numericCases.map((row: any) => [row.id, row.kind, row.expected]), [
    ["u64-max", "uint", "admit"],
    ["i64-min", "int", "admit"],
    ["finite-float", "float", "admit"],
  ]);
  assert.equal(BigInt(fixture.carrier.numericCases[0].canonical), 18446744073709551615n);
  assert.equal(BigInt(fixture.carrier.numericCases[1].canonical), -9223372036854775808n);
  assert.equal(Number(fixture.carrier.numericCases[2].canonical), 1.5);
  assert([...fixture.carrier.escapedControlIdentity].some((scalar) => /\p{Cc}/u.test(scalar)));
  assert.equal(fixture.carrier.escapedControlExpected, "reject");
  assert.deepEqual(fixture.publication.commitPhases, [
    "staging-parent", "staging-drawing", "staging-value", "starting-journal", "journal",
    "publishing-parent-lease", "publishing-drawing-lease", "publishing-value-lease",
    "adopting-parent", "adopting-drawing", "adopting-value",
    "clearing-parent", "clearing-drawing", "clearing-value", "awaiting-ack", "closing-journal", "complete",
  ]);
  assert.deepEqual(fixture.publication.pendingRoots, [0, 0, 0]);
  assert.deepEqual(fixture.publication.committedRoots, [7, 11, 13]);
  assert.deepEqual([fixture.publication.cancellationResolution, fixture.publication.ambiguousJournalError, fixture.publication.acknowledgement], ["absent", "retry", "required"]);
  const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
  for (const marker of ["DurableOwnedThreeMemberDecisionV1", "DurableOwnedThreeMemberUnsignedV1", "DurableUnboundOneItemOutcomeV1", "DurableBoundOneItemOutcomeV1", "DurableStorePreparedOutcomeV1", "DurableOwnedThreeStorePreparedV1", "DurableOwnedThreeStoreBoundV1", "DurableOwnedThreeStoreCommitV1", "DurableOwnedMapCommitOperationV1", "DurableOwnedMapCommitOwnersV1", "DurableOwnedGroupJournalSinkV1", "capture_store_owned_three_snapshot", "durable_unbound_outcome", "verify_inverse", "bind_store_owned", "recover_store_owned", "begin_retained_commit", "mount_map", "take_terminal_owners", "validate_json_budget", "next_clock_canonical_json", "edit_without_group_canonical_json", "parse_canonical_json", "decode_canonical_pack", "admit_map", "DURABLE_OWNED_GROUP_EVENT_MAX_BYTES", "max_total_alloc: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES", "manifest.doc_frame_count != 1", "durable_owned_group_decision_matches_neutral_canonical_hash_and_bounds", "durable_store_prepared_outcome_derives_and_verifies_exact_unbound_bytes", "durable_store_owned_three_member_bind_and_base_recovery_retain_exact_private_owners", "durable_store_group_journal_commit_flips_one_shared_root_then_adopts_exactly_once", "durable_store_group_cancellation_waits_for_trusted_absence_then_restores_all_old_roots", "durable_store_group_stage_error_retains_abort_owner_until_every_root_is_empty", "durable_store_group_uncertain_journal_error_retries_same_owner_without_rebegin_or_visibility_change", "durable_store_group_rejects_foreign_anchor_receipt_before_visibility_and_aborts_only_after_absence", "durable_map_mounted_operation_retains_every_live_owner_across_request_error_until_terminal_handoff", "durable_json_carriers_preserve_numeric_kinds_and_reject_control_and_resource_excess", "durable_decision_rejects_deflate_expansion_before_document_body_allocation"]) assert(source.includes(marker), `missing durable decision marker ${marker}`);
  assert(!source.includes("pub(crate) fn seal("), "callers cannot construct durable decisions from supplied hashes or recovery packs");
  const unsignedCut = source.slice(source.indexOf("struct DurableOwnedThreeMemberUnsignedV1"), source.indexOf("pub struct DurableOwnedThreeMemberDecisionV1"));
  for (const forbidden of ["recovery_pack", "recovery_pack_sha256", "post_generation", "post_revision", "edit_digest", "prefix", "seal"]) assert(!unsignedCut.includes(forbidden), `unsigned preimage contains ${forbidden}`);
  console.log(`durable-owned-group-decision-oracle: AJV=1 SHA256=1 roles=3 cases=${fixture.cases.length} event=${fixture.limits.eventBytes} whole=${exactWholeSegmentBytes} member=${fixture.limits.recoveryPackBytes}`);
}
