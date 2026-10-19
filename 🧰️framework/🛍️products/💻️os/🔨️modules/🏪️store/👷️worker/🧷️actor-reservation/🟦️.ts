import { ReservationPollLanes } from "./🛤️poll-lanes/🟦️.ts";
import { ReservationTurnEffects } from "./📤️turn-effects/🟦️.ts";
import { ReservationUiPatches } from "./🩹️ui-patch-reconciliation/🟦️.ts";
import { ReservationDocumentBackbone } from "./📬️document-backbone/🟦️.ts";
import { ReservationColdPairTransfer } from "./🧊️cold-pair-transfer/🟦️.ts";
import { ReservationGuestActivation } from "./🚀️guest-activation/🟦️.ts";
import { ReservationGuestRetirement } from "./♻️guest-retirement/🟦️.ts";
import { ReservationSurfaceRendering } from "./🖼️surface-rendering/🟦️.ts";
import { ReservationActionDispatch } from "./🎯️action-dispatch/🟦️.ts";
import { ReservationLinkSuspension } from "./🔌️link-suspension/🟦️.ts";
import { ReservationSession } from "./🗃️state/🟦️.ts";
import type { ReservationHub } from "./🧩️hub/🟦️.ts";
import type { BrowserActorUiPatchResultV1 } from "@semio-tech/framework/browser-bundle/patch-handoff";
import type { BrowserActorEphemeralSnapshotV1 } from "@semio-tech/framework/browser-bundle/action-handoff/publication";
import type { BrowserActorActionRequestV1, BrowserActorActionResultV1 } from "@semio-tech/framework/browser-bundle/action-handoff";
import type { DocumentBrowserActorGrant } from "../🧩️actor-wire-helpers/🟦️.ts";
import type { VerifiedColdDocumentPair } from "../🧊️cold-pair/🟦️.ts";
import type { ArtifactState } from "../🧾️document-state/🟦️.ts";
import type { DocumentExecutionTargetLease } from "../🪪️execution-lease/🟦️.ts";

let documentBrowserActorGeneration = 0n;

/** 🧩️ Wires the collaborators of one {@link DocumentBrowserActorReservation} around its shell and its single private session; collaborators reach each other only through this hub, and only at call time, never while they are constructed. */
export class ReservationHubWiring implements ReservationHub {
  readonly lanes: ReservationPollLanes;
  readonly effects: ReservationTurnEffects;
  readonly patches: ReservationUiPatches;
  readonly backbone: ReservationDocumentBackbone;
  readonly coldPair: ReservationColdPairTransfer;
  readonly activation: ReservationGuestActivation;
  readonly retirement: ReservationGuestRetirement;
  readonly surfaces: ReservationSurfaceRendering;
  readonly actions: ReservationActionDispatch;
  readonly link: ReservationLinkSuspension;
  constructor(readonly shell: DocumentBrowserActorReservation, readonly session: ReservationSession) {
    this.lanes = new ReservationPollLanes(this);
    this.effects = new ReservationTurnEffects(this);
    this.patches = new ReservationUiPatches(this);
    this.backbone = new ReservationDocumentBackbone(this);
    this.coldPair = new ReservationColdPairTransfer(this);
    this.activation = new ReservationGuestActivation(this);
    this.retirement = new ReservationGuestRetirement(this);
    this.surfaces = new ReservationSurfaceRendering(this);
    this.actions = new ReservationActionDispatch(this);
    this.link = new ReservationLinkSuspension(this);
  }
}

/** 🧷️ This module owns the `browserActorReservation` slot of the artifact state. */
declare module "../🧾️document-state/🟦️.ts" {
  interface ArtifactOwnerSlots { browserActorReservation: DocumentBrowserActorReservation }
}

/** 🧷️ Owns one document's reserved child; only the live private lease and exchanged grant select it. */
export class DocumentBrowserActorReservation {
  readonly generation: bigint;
  private readonly hub: ReservationHubWiring;
  private readonly retire = () => this.close();

  constructor(state: ArtifactState, lease: DocumentExecutionTargetLease, grant: DocumentBrowserActorGrant) {
    if (documentBrowserActorGeneration === 0xffffffffffffffffn) throw new Error("document browser actor: generation exhausted");
    const session = new ReservationSession(state, lease, grant, this.retire, () => ++documentBrowserActorGeneration);
    this.generation = session.generation;
    this.hub = new ReservationHubWiring(this, session);
    session.artifact.docAbort.signal.addEventListener("abort", session.retire, { once: true });
    session.lease.retirement.addEventListener("abort", session.retire, { once: true });
  }

  get retirementOutcome(): Promise<"retired" | "unconfirmed"> | null {
    return this.hub.session.retirement;
  }

  /** 👥️ The guest's last published presence pack and interaction slice — what this human's presence heartbeat carries
   * for the actor-bound document (the Shell's local instance never sees it). `null` until the guest published one. */
  get ephemeralSnapshot(): BrowserActorEphemeralSnapshotV1 | null {
    return this.hub.session.lastEphemeral;
  }

  get suspended(): boolean { return this.hub.link.suspended; }

  reserve(): Promise<void> { return this.hub.activation.reserve(); }
  activate(socket: WebSocket): Promise<void> { return this.hub.activation.activate(socket); }
  installColdPair(owner: VerifiedColdDocumentPair, retainedBackbone?: readonly Uint8Array[]): Promise<void> { return this.hub.coldPair.installColdPair(owner, retainedBackbone); }
  refreshHostView(): void { return this.hub.surfaces.refreshHostView(); }
  bindApprovalUndoIfMounted(): void { return this.hub.surfaces.bindApprovalUndoIfMounted(); }
  receiveBackbone(bytes: Uint8Array): Promise<void> { return this.hub.backbone.receiveBackbone(bytes); }
  dispatchAction(raw: BrowserActorActionRequestV1): Promise<BrowserActorActionResultV1> { return this.hub.actions.dispatchAction(raw); }
  settleUiPatch(result: BrowserActorUiPatchResultV1): void { return this.hub.patches.settleUiPatch(result); }
  suspendLink(): boolean { return this.hub.link.suspendLink(); }
  adoptGrant(grant: DocumentBrowserActorGrant): void { return this.hub.link.adoptGrant(grant); }
  resumeLink(socket: WebSocket): boolean { return this.hub.link.resumeLink(socket); }
  close(): void { return this.hub.retirement.close(); }
}
