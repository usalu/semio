import type { ReservationSession } from "../🗃️state/🟦️.ts";
import type { ReservationHub } from "../🧩️hub/🟦️.ts";
import { createShardCommandIngressPages } from "@semio-tech/framework-actor/shard-client";
import { parseBrowserActorHostEffectBytesV1, type BrowserActorActionRequestV1, type BrowserActorActionResultV1, parseBrowserActorActionRequestV1, BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION, parseBrowserActorHistoryPatchBytesV1, browserActorGuestRefusalReasonV1, browserActorAdmissionRefusalReasonV1 } from "@semio-tech/framework/browser-bundle/action-handoff";
import { documentActorRecoveryStepV1 } from "../../🚑️actor-recovery/🟦️.ts";
import { type BrowserActorActionPublication, BrowserActorGuestRefusalV1, browserActorActionDisposition, browserActorAppCommandV1, browserActorCommandIngressStatus, browserActorUiIntentV1 } from "../../🧩️actor-wire-helpers/🟦️.ts";
import { requestDocumentActorRecoveryV1 } from "../../📥️execution-target-read/🟦️.ts";
import { documentCatchingUpV1 } from "../../🛰️hub-relay/🟦️.ts";

declare module "../🧩️hub/🟦️.ts" {
  interface ReservationCollaborators { actions: ReservationActionDispatch }
}

/** 🎯️ Applies one authenticated action in issue order on the turn lane. */
export class ReservationActionDispatch {
  private readonly session: ReservationSession;
  constructor(private readonly hub: ReservationHub) { this.session = hub.session; }

  /** ⏳️ Resolves once no patch offer awaits the main thread and no host view refresh is outstanding — poll-free, by
   * wrapping the promises those two already own. It MUST run outside the turn lane: `refreshHostView` settles by
   * running its own turn on that lane, so a turn that awaited `viewRefresh` from inside would wait on the turn
   * queued behind itself. A patch deadline rejects through here (the action falls to `action-refused: <why>`). */
  private async awaitUiQuiescence(): Promise<void> {
    while (this.session.pendingUiPatch !== null || this.session.viewRefresh !== null) {
      if (this.session.pendingUiPatch !== null) await this.session.pendingUiPatch.settled;
      else await this.session.viewRefresh;
    }
  }

  /** 🎯️ Applies one authenticated action in issue order: the turn lane is the FIFO (L5 — an early input queues, it is
   * never refused as `action-busy`), a patch still being acknowledged is awaited rather than refused, and the
   * action is accepted against any revision this lifetime painted. */
  async dispatchAction(raw: BrowserActorActionRequestV1): Promise<BrowserActorActionResultV1> {
    const request = parseBrowserActorActionRequestV1(raw);
    let invoked = false;
    try {
      await this.awaitUiQuiescence();
      return await this.hub.lanes.enqueueTurn(async () => {
        if (this.session.pendingUiPatch !== null) await this.session.pendingUiPatch.settled;
        const fields = this.session.lease.fields();
        this.hub.backbone.assertDocumentOwnerCurrent();
        const intent = request.payload.kind === "ui-intent" ? browserActorUiIntentV1(request) : null,
          command = intent === null ? browserActorAppCommandV1(request, fields, this.session.windowKeys) : null;
        const painted = request.surfaceRevision >= 1 && request.surfaceRevision <= (this.session.renderedSurfaceRevisions.get(intent?.surfaceKey ?? command!.surfaceKey) ?? 0);
        if (
          request.scope.spaceId !== fields.scope.spaceId ||
          request.scope.documentId !== fields.scope.documentId ||
          request.verifiedSurfaceId !== fields.surface.surfaceId ||
          request.appChannelVersion !== BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION ||
          request.activationGeneration !== this.session.generation.toString() ||
          request.instanceId !== 0 ||
          !painted ||
          request.actionSequence <= this.session.lastActionSequence
        ) throw new Error("action-owner-mismatch");
        if (
          documentCatchingUpV1({
            bound: this.session.documentBinding !== null,
            backboneReady: this.session.documentBackboneReady,
            bootstrap: this.session.artifact.artifactBootstrap !== null,
            rebootstrap: this.session.artifact.artifactRebootstrapRequired,
            tail: this.session.artifact.requiredTailFrontier !== null,
            coldApplied: this.session.coldApplied !== null,
            coldTransfer: this.session.coldTransfer !== null,
          })
        ) throw new Error("action-catching-up");
        const child = this.session.child;
        if (child === null) throw new Error("action-child-unavailable");
        this.session.lastActionSequence = request.actionSequence;
        const publication: BrowserActorActionPublication = { kind: request.payload.kind, sequence: request.actionSequence, frames: 0, commit: null, refusals: [], hostEffects: [], historyPatches: [] };
        let mutationCount = 0;
        if (intent !== null) {
          invoked = true;
          const result = await this.hub.lanes.invokePoll(child, [{ tag: "ui-intent", val: { instance: 0, intent: intent.bytes } }], null, () => this.hub.backbone.assertDocumentOwnerCurrent());
          mutationCount = await this.hub.patches.driveTurnResult(result, child, () => this.hub.backbone.assertDocumentOwnerCurrent(), publication);
        } else {
          const pages = createShardCommandIngressPages({ owner: 0n, generation: this.session.generation, commandIndex: 0, commandCount: 1, instance: 0, seq: BigInt(request.actionSequence), command: command!.bytes });
          let terminal = "idle";
          for (const page of pages) {
            invoked = true;
            const result = await this.hub.lanes.invokeCommandPage(child, page, () => this.hub.backbone.assertDocumentOwnerCurrent());
            terminal = browserActorCommandIngressStatus(result);
            mutationCount += await this.hub.patches.driveTurnResult(result, child, () => this.hub.backbone.assertDocumentOwnerCurrent(), publication);
            if (terminal === "backpressure" || terminal === "fault") throw new Error("action-command-ingress-refused");
          }
          for (let turn = 0; terminal !== "command-complete" && turn < 1_024; turn += 1) {
            const result = await this.hub.lanes.invokeCommandPage(child, null, () => this.hub.backbone.assertDocumentOwnerCurrent());
            terminal = browserActorCommandIngressStatus(result);
            mutationCount += await this.hub.patches.driveTurnResult(result, child, () => this.hub.backbone.assertDocumentOwnerCurrent(), publication);
            if (terminal === "backpressure" || terminal === "fault") throw new Error("action-command-ingress-refused");
          }
          if (terminal !== "command-complete") throw new Error("action-command-ingress-unconfirmed");
        }
        if (publication.refusals.length !== 0) throw new BrowserActorGuestRefusalV1(publication.refusals[0] ?? null);
        if (publication.frames !== 1) throw new Error("action-publication-mismatch");
        await this.hub.surfaces.refreshDocumentSurfaces(child, () => this.hub.backbone.assertDocumentOwnerCurrent(), command !== null || mutationCount > 0);
        this.session.artifact.actorRecovery = documentActorRecoveryStepV1(this.session.artifact.actorRecovery, { kind: "applied", atMs: Date.now() }).memory;
        return browserActorActionDisposition(request, "guest-applied", mutationCount, publication.commit, parseBrowserActorHostEffectBytesV1(publication.hostEffects), undefined, parseBrowserActorHistoryPatchBytesV1(publication.historyPatches));
      });
    } catch (error) {
      const explicitRefusal = error instanceof BrowserActorGuestRefusalV1;
      if (invoked && !explicitRefusal) {
        this.hub.retirement.close();
        requestDocumentActorRecoveryV1(this.session.artifact, "action-unconfirmed");
      }
      const reason = explicitRefusal ? browserActorGuestRefusalReasonV1(error.detail) : error instanceof Error && /^(action-owner-mismatch|action-catching-up|action-child-unavailable)$/u.test(error.message) ? error.message : invoked ? "action-state-unconfirmed" : browserActorAdmissionRefusalReasonV1(error instanceof Error ? error.message : String(error));
      return browserActorActionDisposition(request, "rejected", 0, null, [], reason);
    }
  }
}
