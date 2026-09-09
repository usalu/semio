import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";

/** 🪪️ Validates neutral admission laws with independent AJV predicates and pins the real reducer. */
export function guestLifecycleOracle(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../⚛️reactor/🚪️lifetime/🧫️fixtures/🧵️production.json", import.meta.url), "utf8"));
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
  const fixture = JSON.parse(readFileSync(new URL("../../⚛️reactor/📨️pending/🧫️fixtures/🩹️receipt.json", import.meta.url), "utf8"));
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


export async function coldDocumentPairIngressOracle(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(new URL("../../⚛️reactor/📥️cold-pair/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
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
  const actorFixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🧫️fixtures/🔣️.json"), "utf8"));
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


export function documentBackboneBindingOracle(repoRoot: string): number {
  const fixture = JSON.parse(readFileSync(new URL("../../📡️backbone/🔗️binding/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
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
  for (const marker of ["ArtifactActorMsg::DocumentBackbone", "ArtifactEvent::DocumentBackbone", "DocumentBackboneRetentionV1", "decode_document_backbone_message_exact", "semio_framework_async::oneshot::channel", "pool.submit_at(pool.now_ms(), semio_framework_async::Lane::Io, admission_job)"]) assert(sync.includes(marker), marker);
  for (const marker of ["tokio::runtime::Handle::try_current()", "self.io_reactor.as_ref().map(tokio::runtime::Handle::enter)"]) assert(sync.includes(marker), marker);
  for (const marker of ["struct ArtifactReadinessWake", "connection.read.poll_next_unpin(&mut context)", "future.as_mut().poll(&mut context)"]) assert(sync.includes(marker), marker);
  for (const marker of ["readiness_requested.store(true", "self.readiness_requested.swap(false", "self.drive_phase = ArtifactDrivePhase::ConnectResult", "if self.semio_hub.is_some() {\n                            self.drive_phase = ArtifactDrivePhase::Hub", "self.drive_phase = ArtifactDrivePhase::Hub;\n                        self.on_hub_message(message).await"]) assert(sync.includes(marker), marker);
  assert(sync.includes("self.semio_hub.is_some() || self.connect_future.is_some() || self.reconnect_at.is_some_and"), "connect admission must retain live sockets and respect backoff");
  const handoff = sync.slice(sync.indexOf("fn release_scheduled_after_turn_with"), sync.indexOf("fn arm_deadline"));
  assert(handoff.indexOf("self.scheduled.store(false") < handoff.indexOf("self.wake_requested.swap(false"), "readiness handoff must release scheduled ownership before consuming a concurrent wake");
  const cancellation = sync.slice(sync.indexOf("fn cancel(self: &Arc<Self>)"), sync.indexOf("fn request_close"));
  assert(!cancellation.includes("scheduled.store(false"), "cancellation must not steal scheduled ownership from an inflight turn");
  assert(!sync.includes("tokio::task::spawn_blocking"), "native actor must not depend on an ambient Tokio runtime for socket admission");
  for (const marker of ["plugin_handle_document_backbone_binding", "plugin_receive_document_backbone", "plugin_drain_document_backbones"]) assert(reactor.includes(marker), marker);
  const retireBranch = component.slice(component.indexOf("DocumentBackboneBindingDecisionV1::Retire(receipt)"), component.indexOf("pub async fn plugin_receive_document_backbone"));
  assert(retireBranch.indexOf("owner.begin_retire()") < retireBranch.indexOf("plugin_detach_backbone(runtime, command.instance_id).await"), "retire must synchronously close ingress before detach awaits");
  assert(!retireBranch.includes("document_backbone_effects"), "retire must discard stale data before emitting its sole receipt");
  assert.deepEqual(fixture.dataLimits, { hotMessageBytes: 262144, snapshotMessageBytes: 4194304, pendingBytes: 1048576, pendingMessages: 64, snapshotTransport: "cold-pair" });
  assert.deepEqual(batchFixture.retention, { maximumBytes: 1048576, maximumMessages: 64 });
  return fixture.cases.length + fixture.codec.golden.length + fixture.codec.hostile.length + fixture.hostile.length + batchFixture.cases.length;
}

