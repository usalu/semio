/** 🛰️ Owns the authenticated Hub path for complete original document edits. */
import { AtomicDocumentAssembly, atomicBackbonePart, type AtomicDocumentRows } from "../../../../../../🔨️modules/📡️replication/🚪️io/💾️binary/🔗️causal/🧮️backbone/🧩️atomic-group/🟦️.ts";
import { decodeAtomicDocumentBackboneBatch } from "../../../../../../🔨️modules/📡️replication/🚪️io/💾️binary/🔗️causal/🧮️backbone/🟦️.ts";
import { encodeBackboneMessage, type DocumentBackboneTransportMessage } from "../../../../../../🔨️modules/🛰️backbone/🔀️message/🟦️.ts";
import { DOCUMENT_BACKBONE_RETENTION_LIMITS, decodeServerFrame, type WireAckStage, type WireFrontierSummary } from "@semio-tech/framework-replication";
import { hubCommandRejectionV1, hubTransientApplyRefusalV1 } from "../../../../🟦️";
import { closeHubSocketV1 } from "@semio-tech/framework/directory/client/socket-close";
import { artifacts, emitEvent, type ArtifactState } from "../🧾️document-state/🟦️.ts";
import { documentAwaitsBrowserActor, retainBackboneBeforeBrowserActor } from "../🎭️actor-lifecycle/🟦️.ts";
import { finishCatchupIfReady } from "../🚀️artifact-bootstrap/🟦️.ts";
import { requestDocumentActorRecoveryV1 } from "../📥️execution-target-read/🟦️.ts";
import { encodeClientAtomicDocumentPart, decodeServerAtomicDocumentPart } from "./📡️wire.ts";

type Row = ReturnType<typeof decodeAtomicDocumentBackboneBatch>[number];
type HostPart = Extract<DocumentBackboneTransportMessage, { kind: "atomic-part" }>;
type Group = Readonly<{ edit: AtomicDocumentRows<Row>; bytes: number; messages: number }>;
type SocketFlight = Readonly<{ id: number; socket: WebSocket }>;
type Outgoing = {
  readonly assembly: AtomicDocumentAssembly<Row>;
  readonly queue: Group[];
  heldBytes: number;
  heldMessages: number;
  flight: SocketFlight | null;
  retry: ReturnType<typeof setTimeout> | null;
};
type Incoming = { assembly: AtomicDocumentAssembly<Row>; frontier: WireFrontierSummary | null; nextOrdinal: number };
const outgoing = new WeakMap<ArtifactState, Outgoing>();
const incoming = new WeakMap<WebSocket, Incoming>();

function assertCurrent(state: ArtifactState, socket?: WebSocket): void {
  if (state.closed || artifacts.get(state.runtimeKey) !== state || (socket !== undefined && state.socket !== socket)) {
    throw new Error("atomic Hub group: retired document binding");
  }
}

export async function relayAtomicHubPart(state: ArtifactState, item: HostPart): Promise<void> {
  assertCurrent(state);
  const actor = state.actor;
  if (!state.hubActorReady || !actor || state.executionTargetLease?.fields().grant.write !== true) throw new Error("atomic Hub group: write authority");
  const scope = { documentId: state.config.documentId, actor, generation: state.openClientInstanceId };
  let slot = outgoing.get(state);
  if (!slot) {
    slot = { assembly: new AtomicDocumentAssembly(scope, decodeAtomicDocumentBackboneBatch), queue: [], heldBytes: 0, heldMessages: 0, flight: null, retry: null };
    outgoing.set(state, slot);
  }
  if (slot.heldMessages + 1 > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages ||
      slot.heldBytes + item.message.length > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes) {
    throw new Error("atomic Hub group: outbox capacity");
  }
  slot.heldBytes += item.message.length;
  slot.heldMessages += 1;
  state.pendingDocumentBackboneBytes += item.message.length;
  state.pendingDocumentBackboneMessages += 1;
  const sealed = await slot.assembly.admit(scope, item.part);
  assertCurrent(state);
  if (!sealed) return;
  const messages = sealed.parts.length;
  const bytes = sealed.parts.reduce((total, part) => total + encodeBackboneMessage({ kind: "mutations", envelopes: part }).byteLength, 0);
  slot.queue.push({ edit: sealed, bytes, messages });
  flushPendingAtomicHubGroup(state);
}

export function flushPendingAtomicHubGroup(state: ArtifactState): boolean {
  const slot = outgoing.get(state);
  const group = slot?.queue[0];
  if (!slot || !group) return false;
  const socket = state.socket;
  if (state.hubActorReady && state.actor !== group.edit.scope.actor) {
    retireAtomicHubDocument(state);
    requestDocumentActorRecoveryV1(state, "inbound-frame");
    return true;
  }
  if (!socket || socket.readyState !== WebSocket.OPEN || !state.hubActorReady ||
      slot.flight || slot.retry || state.pendingBatches.size > 0 || state.requiredTailFrontier || state.artifactBootstrap || state.artifactRebootstrapRequired) return true;
  const id = state.nextBatchId++;
  slot.flight = { id, socket };
  for (const part of group.edit.parts) socket.send(encodeClientAtomicDocumentPart(BigInt(id), part));
  return true;
}

export function retireAtomicHubSocket(state: ArtifactState, socket: WebSocket): void {
  incoming.get(socket)?.assembly.retire();
  incoming.delete(socket);
  const slot = outgoing.get(state);
  if (slot?.flight?.socket === socket) slot.flight = null;
}

export function retireAtomicHubDocument(state: ArtifactState): void {
  if (state.socket) retireAtomicHubSocket(state, state.socket);
  const slot = outgoing.get(state);
  if (!slot) return;
  slot.assembly.retire();
  if (slot.retry !== null) clearTimeout(slot.retry);
  outgoing.delete(state);
  state.pendingDocumentBackboneBytes -= slot.heldBytes;
  state.pendingDocumentBackboneMessages -= slot.heldMessages;
}

export function acknowledgeAtomicHubGroup(state: ArtifactState, id: number, stages: readonly WireAckStage[]): boolean {
  const slot = outgoing.get(state);
  if (!slot || slot.flight?.id !== id || slot.flight.socket !== state.socket) return false;
  const applied = stages.find(stage => typeof stage !== "string" && "Applied" in stage);
  if (applied === undefined || typeof applied === "string" || !("Applied" in applied)) return true;
  const report = applied.Applied.outcome;
  const rejected = typeof report === "object" && "Rejected" in report ? report.Rejected : null;
  slot.flight = null;
  if (rejected && hubTransientApplyRefusalV1(rejected.messages)) {
    slot.retry = setTimeout(() => { slot.retry = null; flushPendingAtomicHubGroup(state); }, 500);
    return true;
  }
  const group = slot.queue.shift()!;
  slot.heldBytes -= group.bytes;
  slot.heldMessages -= group.messages;
  state.pendingDocumentBackboneBytes -= group.bytes;
  state.pendingDocumentBackboneMessages -= group.messages;
  if (report !== "Accepted") {
    if (rejected) emitEvent(state, { kind: "commandOutcome", batchId: id, outcome: hubCommandRejectionV1(rejected) });
    requestDocumentActorRecoveryV1(state, "inbound-frame");
  } else {
    emitEvent(state, { kind: "commandOutcome", batchId: id, outcome: { kind: "accepted" } });
  }
  return true;
}

export async function deliverAtomicHubPacket(state: ArtifactState, socket: WebSocket, bytes: Uint8Array): Promise<boolean> {
  const packet = decodeServerAtomicDocumentPart(bytes);
  if (!packet) return false;
  assertCurrent(state, socket);
  if (state.artifactBootstrap || state.artifactRebootstrapRequired) throw new Error("atomic Hub group: bootstrap frontier");
  const scope = { documentId: state.config.documentId, actor: state.pendingSocketActorId ?? state.actor, generation: state.openClientInstanceId };
  let slot = incoming.get(socket);
  try {
    if (!slot) {
      slot = { assembly: new AtomicDocumentAssembly(scope, decodeAtomicDocumentBackboneBatch, true), frontier: null, nextOrdinal: 0 };
      incoming.set(socket, slot);
    }
    if (atomicBackbonePart(packet.part)?.ordinal !== slot.nextOrdinal++) throw new Error("atomic Hub group: physical order");
    if (packet.frontier.length !== 0) {
      const { lane, frame } = decodeServerFrame(packet.frontier);
      if (lane !== "command" || typeof frame !== "object" || !("Ack" in frame) || frame.Ack.batch_id !== 0 || frame.Ack.stages.length !== 0 || frame.Ack.frontier.document_id !== scope.documentId) {
        throw new Error("atomic Hub group: frontier binding");
      }
      slot.frontier = frame.Ack.frontier;
    }
    const sealed = await slot.assembly.admit(scope, packet.part);
    assertCurrent(state, socket);
    if (sealed === undefined) return true;
    const frontier = slot.frontier;
    if (frontier?.head_edit_id !== sealed.editId) throw new Error("atomic Hub group: edit frontier");
    incoming.delete(socket);
    const reservation = state.browserActorReservation;
    for (const envelopes of sealed.parts) {
      assertCurrent(state, socket);
      const message = encodeBackboneMessage({ kind: "mutations", envelopes });
      if (reservation) await reservation.receiveBackbone(message);
      else if (documentAwaitsBrowserActor(state)) retainBackboneBeforeBrowserActor(state, message);
      else emitEvent(state, { kind: "documentBackbone", message });
    }
    assertCurrent(state, socket);
    if (state.frontier === null || frontier.last_commit_seq >= state.frontier.last_commit_seq) state.frontier = frontier;
    finishCatchupIfReady(state);
    return true;
  } catch (error) {
    retireAtomicHubSocket(state, socket);
    closeHubSocketV1(socket, "actorMismatch");
    throw error;
  }
}
