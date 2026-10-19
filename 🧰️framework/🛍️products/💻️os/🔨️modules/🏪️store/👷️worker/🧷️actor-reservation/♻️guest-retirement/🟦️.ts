import type { ReservationSession } from "../🗃️state/🟦️.ts";
import type { ReservationHub } from "../🧩️hub/🟦️.ts";
import { type ActorInstanceLifetime, type ActorInstanceCloseRequest, actorInstanceLifetimeEquals, type ActorInstanceLifecycleReceipt } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts";
import type { BrowserActorChildValue } from "@semio-tech/framework-plugin/browser-bundle/child";
import { DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT, type DocumentBrowserActorChild, browserActorCloseReceipt, browserActorTurnResult, wipeBrowserActorValue } from "../../🧩️actor-wire-helpers/🟦️.ts";

declare module "../🧩️hub/🟦️.ts" {
  interface ReservationCollaborators { retirement: ReservationGuestRetirement }
}

/** ♻️ Closes the reservation and retires the guest instance through the lifecycle close protocol. */
export class ReservationGuestRetirement {
  private readonly session: ReservationSession;
  constructor(private readonly hub: ReservationHub) { this.session = hub.session; }

  private async retireGuest(child: DocumentBrowserActorChild, lifetime: ActorInstanceLifetime): Promise<"retired" | "unconfirmed"> {
    const request: ActorInstanceCloseRequest = { kind: "close", lifetime, requestSequence: 2 },
      assertRetained = () => {
        if (this.session.child !== child || this.session.lifetime === null || !actorInstanceLifetimeEquals(this.session.lifetime, lifetime)) throw new Error("document browser actor: stale retirement owner");
      };
    let result: BrowserActorChildValue | null = null;
    try {
      result = await this.hub.lanes.invokePoll(child, [{ tag: "instance-close", val: { lifetime: { ...lifetime }, requestSequence: BigInt(request.requestSequence) } }], null, assertRetained, true);
      let receipt = browserActorCloseReceipt(result, request, null);
      if (receipt?.kind !== "accepted" || this.hub.patches.captureUiPatch(result, lifetime) !== null || ((browserActorTurnResult(result).effects as readonly unknown[] | undefined)?.length ?? 0) !== 0) throw new Error("document browser actor: close not accepted");
      wipeBrowserActorValue(result);
      result = null;
      const accepted: ActorInstanceLifecycleReceipt = receipt;
      result = await this.hub.lanes.invokePoll(
        child,
        [{ tag: "instance-lifecycle-ack", val: { tag: "accepted", val: { lifetime: { ...accepted.lifetime }, requestSequence: BigInt(accepted.requestSequence), closeGeneration: accepted.closeGeneration } } }],
        null,
        assertRetained,
        true,
      );
      for (let turn = 0; turn < DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT; turn += 1) {
        receipt = browserActorCloseReceipt(result, request, accepted);
        if (this.hub.patches.captureUiPatch(result, lifetime) !== null || ((browserActorTurnResult(result).effects as readonly unknown[] | undefined)?.length ?? 0) !== 0) throw new Error("document browser actor: close emitted work");
        if (receipt?.kind === "retired") {
          const retired = receipt;
          wipeBrowserActorValue(result);
          result = await this.hub.lanes.invokePoll(
            child,
            [{ tag: "instance-lifecycle-ack", val: { tag: "retired", val: { lifetime: { ...retired.lifetime }, requestSequence: BigInt(retired.requestSequence), closeGeneration: retired.closeGeneration } } }],
            null,
            assertRetained,
            true,
          );
          if (browserActorCloseReceipt(result, request, retired) !== null || this.hub.patches.captureUiPatch(result, lifetime) !== null || ((browserActorTurnResult(result).effects as readonly unknown[] | undefined)?.length ?? 0) !== 0) throw new Error("document browser actor: retired ACK not terminal");
          return "retired";
        }
        if (receipt !== null && receipt.kind !== "accepted") throw new Error("document browser actor: invalid close progress");
        wipeBrowserActorValue(result);
        result = await this.hub.lanes.invokePoll(child, [], null, assertRetained, true);
      }
      return "unconfirmed";
    } catch {
      return "unconfirmed";
    } finally {
      if (result !== null) wipeBrowserActorValue(result);
    }
  }

  close(): void {
    if (this.session.closed) return;
    this.session.closed = true;
    this.session.artifact.docAbort.signal.removeEventListener("abort", this.session.retire);
    this.session.lease.retirement.removeEventListener("abort", this.session.retire);
    if (this.session.pendingUiPatch !== null) {
      const pending = this.session.pendingUiPatch;
      this.session.pendingUiPatch = null;
      clearTimeout(pending.timer);
      pending.reject(new Error("document browser actor: closed with pending patch"));
    }
    const child = this.session.child,
      lifetime = this.session.lifetime;
    this.session.coldOwner = null;
    this.session.coldApplied = null;
    this.session.renderedUiPatch = false;
    this.session.renderedSurfaceRevisions.clear();
    this.session.acknowledgedUiRevision = 0;
    this.session.mountedUiRevision = 0;
    this.session.renderedViewState = null;
    this.session.lastActionSequence = 0;
    this.session.pendingBackboneBeforeBinding = [];
    this.session.pendingBackboneBeforeBindingBytes = 0;
    this.session.documentBackboneReady = false;
    if (this.session.artifact.browserActorReservation === this.hub.shell) this.session.artifact.browserActorReservation = null;
    if (child === null || lifetime === null) {
      const outcome = child === null ? "retired" as const : "unconfirmed" as const;
      this.session.abort.abort();
      child?.close(outcome === "retired" ? "retired" : "retirement unconfirmed");
      this.session.child = null;
      this.session.lifetime = null;
      this.session.documentBinding = null;
      this.session.retirement = Promise.resolve(outcome);
      return;
    }
    const bindingRetirement = this.session.documentBinding?.port.retire() ?? Promise.resolve();
    this.session.retirement = bindingRetirement.then(() => this.hub.lanes.enqueueTurn(() => this.retireGuest(child, lifetime), true), () => "unconfirmed" as const).then(outcome => {
      this.session.abort.abort();
      child?.close(outcome === "retired" ? "retired" : "retirement unconfirmed");
      this.session.child = null;
      this.session.lifetime = null;
      this.session.documentBinding = null;
      return outcome;
    });
  }
}
