import type { ReservationSession } from "../🗃️state/🟦️.ts";
import type { ReservationHub } from "../🧩️hub/🟦️.ts";
import { ActorDocumentBindingV1, documentBackboneEffectV1, encodeDocumentBackboneControlV1 } from "@semio-tech/framework/backbone/binding";
import type { BrowserActorChildValue } from "@semio-tech/framework-plugin/browser-bundle/child";
import { parseDocumentBackboneMessage, BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, parseInboundDocumentBackboneMessage, decodeBackboneMessage } from "../../../../../🟦️";
import { DOCUMENT_BACKBONE_RETENTION_LIMITS } from "@semio-tech/framework-replication";
import { type DocumentBrowserActorChild, browserActorColdStatus, hubBinding, wipeBrowserActorValue } from "../../🧩️actor-wire-helpers/🟦️.ts";
import { documentBrowserActorLease } from "../../🎭️actor-lifecycle/🟦️.ts";
import { emitExecutionTargetStatus } from "../../📥️execution-target-read/🟦️.ts";
import { handleLocalMsg } from "../../♻️artifact-lifecycle/🟦️.ts";

declare module "../🧩️hub/🟦️.ts" {
  interface ReservationCollaborators { backbone: ReservationDocumentBackbone }
}

/** 📬️ Binds the document backbone port of the child and admits, retains and delivers the inbound backbone messages. */
export class ReservationDocumentBackbone {
  private readonly session: ReservationSession;
  constructor(private readonly hub: ReservationHub) { this.session = hub.session; }

  assertDocumentOwnerCurrent(): void {
    if (this.session.closed || this.session.artifact.browserActorReservation !== this.hub.shell || documentBrowserActorLease(this.session.artifact) !== this.session.lease || this.session.lease.browserActorGrant() !== this.session.grant) throw new Error("actor-document-port.stale");
    this.session.lease.assertBrowserActorCurrent();
  }

  private currentDocumentSource() {
    return { runtimeKey: this.session.artifact.runtimeKey, clientInstanceId: this.session.artifact.openClientInstanceId, scope: { ...this.session.scope } } as const;
  }

  async bindDocumentBackbone(child: DocumentBrowserActorChild): Promise<void> {
    if (this.session.documentBinding !== null) return this.session.documentBinding.bind();
    const owner = { ...this.currentDocumentSource(), actorId: this.session.grant.actorId, activationGeneration: this.session.generation, instanceId: 0 } as const,
      binding = new ActorDocumentBindingV1(owner, ++this.session.documentBindingGeneration, {
        current: () => {
          try { this.assertDocumentOwnerCurrent(); return true; } catch { return false; }
        },
        assertActive: () => this.assertDocumentOwnerCurrent(),
        limits: { messageBytes: BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, pendingBytes: DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes, pendingMessages: DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages },
        send: payload => {
          const kind = documentBackboneEffectV1(payload);
          if (kind === "remote-ingest-receipt") return;
          const work = this.session.localEffectTail.then(async () => {
            if (this.session.localEffectFailure !== null) throw this.session.localEffectFailure;
            const priorBytes = this.session.artifact.pendingDocumentBackboneBytes,
              priorMessages = this.session.artifact.pendingDocumentBackboneMessages;
            await handleLocalMsg(this.session.artifact, { kind: "documentBackbone", message: payload });
            if (this.session.artifact.pendingDocumentBackboneMessages !== priorMessages + 1 || this.session.artifact.pendingDocumentBackboneBytes !== priorBytes + payload.byteLength) throw new Error("actor-document-port.admission-refused");
          });
          this.session.localEffectTail = work.then(
            () => {},
            error => { this.session.localEffectFailure = error instanceof Error ? error : new Error("actor-document-port.admission-refused"); },
          );
        },
        exchange: async command => {
          const exchange = async () => {
            let result: BrowserActorChildValue | null = null;
            try {
              result = await this.hub.lanes.invokePoll(
                child,
                [{ tag: "message", val: { source: { tag: "shell", val: 0 }, payload: encodeDocumentBackboneControlV1(command) } }],
                null,
                () => {
                  if (this.session.child !== child || this.session.lifetime === null) throw new Error("actor-document-control.stale-child");
                  if (command.operation !== "retire") this.assertDocumentOwnerCurrent();
                },
                command.operation === "retire",
              );
              if (browserActorColdStatus(result).kind !== "idle" || this.hub.patches.captureUiPatch(result, this.session.lifetime!) !== null) throw new Error("actor-document-control.unexpected-turn-output");
              const routed = await this.hub.effects.routeTurnEffects(result, "control");
              if (routed.mutations !== 0) throw new Error("actor-document-control.data-before-receipt");
              return routed.receipts;
            } finally {
              if (result !== null) wipeBrowserActorValue(result);
            }
          };
          return command.operation === "retire" ? this.hub.lanes.enqueueTurn(exchange, true) : exchange();
        },
        deliver: async payload => {
          await this.hub.lanes.enqueueTurn(async () => {
            let result: BrowserActorChildValue | null = null;
            try {
              result = await this.hub.lanes.invokePoll(child, [{ tag: "message", val: { source: { tag: "backbone", val: binding.port.uri }, payload } }], null, () => this.assertDocumentOwnerCurrent());
              if (browserActorColdStatus(result).kind !== "idle") throw new Error("actor-document-port.unexpected-cold-ingress");
              await this.hub.patches.driveTurnResult(result, child, () => this.assertDocumentOwnerCurrent());
              result = null;
              await this.hub.surfaces.refreshDocumentSurfaces(child, () => this.assertDocumentOwnerCurrent(), false);
            } finally {
              if (result !== null) wipeBrowserActorValue(result);
            }
          });
        },
      });
    this.session.documentBinding = binding;
    try {
      await binding.bind();
    } catch (error) {
      if (this.session.documentBinding === binding) this.session.documentBinding = null;
      void binding.port.retire().catch(() => {});
      throw error;
    }
  }

  /** 📥️ Delivers the backbone retained before the binding (a catch-up tail among it) in arrival order — reporting
   * `catching-up` progress per delivered message, which the mounted surface clears — then opens the
   * port for live traffic. The guest answers each ingested batch on the bound port (its `remote-ingest-receipt`), so the
   * bound port accepts the guest's sends while the retained backlog drains. */
  async flushPendingBackbone(): Promise<void> {
    const binding = this.session.documentBinding;
    if (binding === null) throw new Error("actor-document-port.unbound");
    this.session.backboneDraining = true;
    try {
      const hub = hubBinding(this.session.artifact.config);
      let delivered = 0;
      while (this.session.pendingBackboneBeforeBinding.length > 0) {
        const queued = this.session.pendingBackboneBeforeBinding;
        this.session.pendingBackboneBeforeBinding = [];
        this.session.pendingBackboneBeforeBindingBytes = 0;
        const retained = delivered + queued.length;
        for (const payload of queued) {
          if (hub !== null) emitExecutionTargetStatus(this.session.artifact, hub, "catching-up", { stage: "catch-up", completedBytes: delivered, totalBytes: retained });
          if (!(await binding.port.receive(this.currentDocumentSource(), payload))) throw new Error("actor-document-port.stale-queued-message");
          delivered += 1;
        }
      }
    } finally {
      this.session.backboneDraining = false;
    }
    this.session.documentBackboneReady = true;
    this.hub.surfaces.publishMountedIfReady();
  }

  retainBackboneBeforeBinding(bytes: Uint8Array): void {
    const message = this.admitInboundBackbone(bytes);
    if (this.session.pendingBackboneBeforeBinding.length >= DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages || message.byteLength > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes - this.session.pendingBackboneBeforeBindingBytes) throw new Error("actor-document-port.capacity");
    this.session.pendingBackboneBeforeBinding.push(message);
    this.session.pendingBackboneBeforeBindingBytes += message.byteLength;
  }

  /** 🛂️ One canonical inbound message for this document's actor: a mutation batch scoped to this document, or the
   * retraction of history transitions the hub refused (`BackboneMessage::Retract`). */
  private admitInboundBackbone(bytes: Uint8Array): Uint8Array {
    const message = parseInboundDocumentBackboneMessage(bytes);
    if (decodeBackboneMessage(message).kind === "retract") return message;
    if (parseDocumentBackboneMessage(message).envelopes.some(envelope => envelope.document_id !== this.session.artifact.config.documentId)) throw new Error("actor-document-port.scope");
    return message;
  }

  async receiveBackbone(bytes: Uint8Array): Promise<void> {
    this.assertDocumentOwnerCurrent();
    const binding = this.session.documentBinding;
    if (binding === null || !this.session.documentBackboneReady) {
      this.retainBackboneBeforeBinding(bytes);
      return;
    }
    if (!(await binding.port.receive(this.currentDocumentSource(), this.admitInboundBackbone(bytes)))) throw new Error("actor-document-port.stale");
  }
}
