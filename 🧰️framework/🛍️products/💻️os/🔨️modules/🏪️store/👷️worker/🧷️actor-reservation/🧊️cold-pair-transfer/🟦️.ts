import type { ReservationSession } from "../🗃️state/🟦️.ts";
import type { ReservationHub } from "../🧩️hub/🟦️.ts";
import type { BrowserActorChildValue } from "@semio-tech/framework-plugin/browser-bundle/child";
import type { PersistenceBinding } from "../../../../../🟦️";
import { type ColdPairIngressStatus, coldDocumentPairCursorEquals } from "@semio-tech/framework-actor/cold-pair";
import { type DocumentBrowserActorChild, browserActorColdStatus, browserActorRecord, coldPairIngressStatusDiagnostic, hubBinding, retryColdPairBackpressure, settleColdPairLoading, wipeBrowserActorValue } from "../../🧩️actor-wire-helpers/🟦️.ts";
import type { VerifiedColdDocumentPair } from "../../🧊️cold-pair/🟦️.ts";
import { documentBrowserActorLease } from "../../🎭️actor-lifecycle/🟦️.ts";
import { emitExecutionTargetStatus } from "../../📥️execution-target-read/🟦️.ts";

declare module "../🧩️hub/🟦️.ts" {
  interface ReservationCollaborators { coldPair: ReservationColdPairTransfer }
}

/** 🧊️ Installs the verified cold document pair into the live child page by page and applies it. */
export class ReservationColdPairTransfer {
  private readonly session: ReservationSession;
  constructor(private readonly hub: ReservationHub) { this.session = hub.session; }

  async installColdPair(owner: VerifiedColdDocumentPair, retainedBackbone: readonly Uint8Array[] = []): Promise<void> {
    if (this.session.closed) throw new Error("document browser actor: closed reservation");
    this.session.documentBackboneReady = false;
    for (const message of retainedBackbone) this.hub.backbone.retainBackboneBeforeBinding(message);
    if (!this.session.child || !this.session.socket || !this.session.activation) return;
    await this.session.activation;
    if (!this.session.lifetime) return;
    await this.session.viewRefresh;
    const binding = hubBinding(this.session.artifact.config),
      socket = this.session.socket;
    if (!binding) throw new Error("document browser actor: missing hub binding");
    const assertCurrent = () => {
      if (this.session.closed || this.session.socket !== socket || !this.hub.link.linkCurrent() || this.session.artifact.browserActorReservation !== this.hub.shell || documentBrowserActorLease(this.session.artifact) !== this.session.lease) throw new Error("document browser actor: stale cold transfer");
      owner.assertCurrent();
    };
    await this.transferColdPair(owner, this.session.child, binding, assertCurrent);
    if (!this.session.documentBackboneReady) await this.hub.backbone.flushPendingBackbone();
    this.hub.surfaces.refreshHostView();
  }

  transferColdPair(owner: VerifiedColdDocumentPair, child: DocumentBrowserActorChild, binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>, assertCurrent: () => void): Promise<void> {
    if (this.session.coldApplied === owner) return Promise.resolve();
    if (this.session.coldOwner === owner && this.session.coldTransfer) return this.session.coldTransfer;
    if (this.session.coldTransfer) return Promise.reject(new Error("document browser actor: cold transfer already active"));
    const lifetime = this.session.lifetime;
    if (!lifetime) return Promise.reject(new Error("document browser actor: guest not live"));
    this.session.coldOwner = owner;
    this.session.coldApplied = null;
    this.session.renderedUiPatch = false;
    this.session.renderedSurfaceRevisions.clear();
    this.session.acknowledgedUiRevision = 0;
    this.session.mountedUiRevision = 0;
    this.session.lastActionSequence = 0;
    this.session.documentBackboneReady = false;
    this.session.coldTransfer = this.hub.lanes.enqueueTurn(async () => {
      for (let pageIndex = 0; pageIndex < owner.pageCount; pageIndex += 1) {
        const sendPage = async (): Promise<BrowserActorChildValue> => {
          assertCurrent();
          const page = owner.page(lifetime, pageIndex);
          const pageRecord = browserActorRecord(page, "document browser actor: invalid cold page");
          const bytes = pageRecord.bytes;
          if (!(bytes instanceof Uint8Array)) throw new Error("document browser actor: invalid cold bytes");
          try {
            const sent = await this.hub.lanes.invokePoll(child, [], page, assertCurrent);
            if (bytes.byteLength !== 0) {
              wipeBrowserActorValue(sent);
              throw new Error("document browser actor: cold page ownership not transferred");
            }
            return sent;
          } finally {
            if (bytes.byteLength) bytes.fill(0);
          }
        };
        let result: BrowserActorChildValue | null = null;
        try {
          result = await retryColdPairBackpressure(sendPage, (value) => browserActorColdStatus(value), lifetime, () => {
            assertCurrent();
            return this.hub.lanes.invokePoll(child, [], null, assertCurrent);
          }, wipeBrowserActorValue);
          assertCurrent();
          if (pageIndex + 1 === owner.pageCount) {
            const cursor = { lifetime, transferGeneration: owner.transferGeneration, pageIndex, pageCount: owner.pageCount };
            const loadingStatus = (value: BrowserActorChildValue): ColdPairIngressStatus => {
              const answered = browserActorColdStatus(value);
              if (answered.kind === "loading" && this.hub.patches.captureUiPatch(value, lifetime) !== null) throw new Error("document browser actor: patch before cold pair applied");
              return answered;
            };
            result = await settleColdPairLoading(result, loadingStatus, cursor, () => {
              assertCurrent();
              return this.hub.lanes.invokePoll(child, [], null, assertCurrent);
            }, wipeBrowserActorValue);
            assertCurrent();
            const status = browserActorColdStatus(result);
            if (status.kind !== "applied") throw new Error(`document browser actor: cold pair not applied (${coldPairIngressStatusDiagnostic(status)})`);
            owner.assertApplied(status, lifetime);
            this.session.coldApplied = owner;
            await this.hub.backbone.bindDocumentBackbone(child);
            await this.hub.patches.reconcileUiPatches(result, child, assertCurrent);
            await this.hub.surfaces.renderSurface(child, assertCurrent);
          } else {
            const status = browserActorColdStatus(result);
            const expected = { lifetime, transferGeneration: owner.transferGeneration, pageIndex, pageCount: owner.pageCount };
            if (status.kind !== "pageAccepted" || !coldDocumentPairCursorEquals(status.cursor, expected))
              throw new Error(`document browser actor: invalid page receipt (page ${pageIndex + 1}/${owner.pageCount} answered ${coldPairIngressStatusDiagnostic(status)})`);
            if (this.hub.patches.captureUiPatch(result, lifetime) !== null) throw new Error("document browser actor: patch before cold pair applied");
          }
        } finally {
          if (result !== null) wipeBrowserActorValue(result);
        }
      }
      assertCurrent();
      this.session.coldApplied = owner;
      this.hub.surfaces.bindApprovalUndoIfMounted();
      if (!this.session.renderedUiPatch) emitExecutionTargetStatus(this.session.artifact, binding, "renderer-unavailable");
    }).then(() => this.hub.backbone.flushPendingBackbone()).finally(() => {
      if (this.session.coldOwner === owner) this.session.coldOwner = null;
      this.session.coldTransfer = null;
    });
    return this.session.coldTransfer;
  }
}
