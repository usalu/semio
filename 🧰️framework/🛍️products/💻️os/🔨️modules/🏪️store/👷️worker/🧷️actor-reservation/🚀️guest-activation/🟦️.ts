import type { ReservationSession } from "../🗃️state/🟦️.ts";
import type { ReservationHub } from "../🧩️hub/🟦️.ts";
import type { ActorInstanceLifetime, ActorInstanceOpenRequest } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts";
import { type BrowserActorChildValue, reserveBrowserActorChild } from "@semio-tech/framework-plugin/browser-bundle/child";
import type { DocumentExecutionTargetProgressV1 } from "@semio-tech/framework/directory/schema";
import { type DocumentBrowserActorChild, browserActorCapturedReceipt, browserActorColdStatus, hubBinding, wipeBrowserActorValue } from "../../🧩️actor-wire-helpers/🟦️.ts";
import { documentBrowserActorLease } from "../../🎭️actor-lifecycle/🟦️.ts";
import { emitExecutionTargetStatus } from "../../📥️execution-target-read/🟦️.ts";

declare module "../🧩️hub/🟦️.ts" {
  interface ReservationCollaborators { activation: ReservationGuestActivation }
}

/** 🚀️ Reserves the child, activates it on the exact socket Session and opens the guest instance. */
export class ReservationGuestActivation {
  private readonly session: ReservationSession;
  constructor(private readonly hub: ReservationHub) { this.session = hub.session; }

  async reserve(): Promise<void> {
    const fields = this.session.lease.fields();
    if (fields.browserActor.kind !== "closed-browser-actor") throw new Error("document browser actor: unavailable");
    const child = await reserveBrowserActorChild({ actorId: this.session.grant.actorId, activationGeneration: this.session.generation, bundleSha256: fields.browserActor.sha256, bundleByteLength: fields.browserActor.byteLength }, this.session.abort.signal);
    this.session.child = child;
    if (this.session.closed || this.session.artifact.browserActorReservation !== this.hub.shell || documentBrowserActorLease(this.session.artifact) !== this.session.lease || this.session.lease.browserActorGrant() !== this.session.grant || Date.now() >= this.session.grant.reserveBeforeMs) {
      this.hub.retirement.close();
      throw new Error("document browser actor: stale reservation");
    }
  }

  activate(socket: WebSocket): Promise<void> {
    if (this.session.activation) {
      if (this.session.socket !== socket) return Promise.reject(new Error("document browser actor: activation socket mismatch"));
      return this.session.activation;
    }
    this.session.socket = socket;
    this.session.activation = (async () => {
      const assertCurrent = () => {
        if (
          this.session.closed ||
          this.session.artifact.socket !== socket ||
          !this.session.artifact.hubActorReady ||
          this.session.artifact.actor !== this.session.grant.actorId ||
          this.session.artifact.pendingSocketActorId !== null ||
          this.session.artifact.browserActorReservation !== this.hub.shell ||
          documentBrowserActorLease(this.session.artifact) !== this.session.lease ||
          this.session.lease.browserActorGrant() !== this.session.grant
        )
          throw new Error("document browser actor: stale Session");
        this.session.lease.assertBrowserActorCurrent();
      };
      assertCurrent();
      const child = this.session.child,
        binding = hubBinding(this.session.artifact.config);
      if (!child || !binding) throw new Error("document browser actor: unavailable child");
      const stage = (name: DocumentExecutionTargetProgressV1["stage"], completedBytes = 0): void => {
        if (this.session.mountedUiRevision === 0) emitExecutionTargetStatus(this.session.artifact, binding, "verifying", { stage: name, completedBytes, totalBytes: 1 });
      };
      await this.session.lease.activateBrowserActor(
        child,
        this.session.abort.signal,
        assertCurrent,
        (progress) => {
          assertCurrent();
          emitExecutionTargetStatus(this.session.artifact, binding, "verifying", progress);
        },
        (progress) => emitExecutionTargetStatus(this.session.artifact, binding, "retrying", progress),
      );
      assertCurrent();
      stage("actor-open");
      this.session.lifetime = await this.openGuest(child, assertCurrent);
      assertCurrent();
      stage("actor-cold");
      const owner = this.session.artifact.verifiedColdPair;
      if (owner) await this.hub.coldPair.transferColdPair(owner, child, binding, assertCurrent);
      stage("actor-view");
      this.hub.surfaces.refreshHostView();
      stage("actor-ready", 1);
    })();
    return this.session.activation;
  }

  private async openGuest(child: DocumentBrowserActorChild, assertCurrent: () => void): Promise<ActorInstanceLifetime> {
    const fields = this.session.lease.fields();
    const request: ActorInstanceOpenRequest = { kind: "open", activationGeneration: this.session.generation, instanceId: 0, requestSequence: 1 };
    const config = new Uint8Array(0),
      quotas = new Uint8Array(0);
    let result: BrowserActorChildValue | null = null;
    try {
      result = await this.hub.lanes.invokePoll(
        child,
        [
          {
              tag: "instance-open",
              val: {
                instance: request.instanceId,
                activationGeneration: request.activationGeneration,
                requestSequence: BigInt(request.requestSequence),
                appId: fields.surface.appId,
                actor: this.session.grant.actorId,
                config,
                assets: [],
                capabilities: [],
                quotas,
              },
          },
        ],
        null,
        assertCurrent,
      );
      assertCurrent();
      const captured = browserActorCapturedReceipt(result, request);
      if (browserActorColdStatus(result, true).kind !== "idle") throw new Error("document browser actor: cold ingress before ACK");
      if (this.hub.patches.captureUiPatch(result, captured.lifetime) !== null) throw new Error("document browser actor: patch before lifecycle ACK");
      const acknowledged = await this.hub.lanes.invokePoll(child, [{ tag: "instance-lifecycle-ack", val: { tag: "captured", val: { lifetime: { ...captured.lifetime }, requestSequence: BigInt(captured.requestSequence) } } }], null, assertCurrent);
      wipeBrowserActorValue(result);
      result = acknowledged;
      assertCurrent();
      if (browserActorColdStatus(acknowledged).kind !== "idle") throw new Error("document browser actor: cold ingress during ACK");
      if (this.hub.patches.captureUiPatch(acknowledged, captured.lifetime) !== null) throw new Error("document browser actor: patch before cold load");
      return captured.lifetime;
    } finally {
      if (config.byteLength) config.fill(0);
      if (quotas.byteLength) quotas.fill(0);
      if (result !== null) wipeBrowserActorValue(result);
    }
  }
}
