import type { ReservationSession } from "../🗃️state/🟦️.ts";
import type { ReservationHub } from "../🧩️hub/🟦️.ts";
import type { DocumentBrowserActorGrant } from "../../🧩️actor-wire-helpers/🟦️.ts";

declare module "../🧩️hub/🟦️.ts" {
  interface ReservationCollaborators { link: ReservationLinkSuspension }
}

/** 🔌️ Keeps a mounted child through a short link loss and resumes it on the re-admitted socket. */
export class ReservationLinkSuspension {
  private readonly session: ReservationSession;
  constructor(hub: ReservationHub) { this.session = hub.session; }

  /** 🔌️ The link this child answers to is the document's current socket, or it is suspended by a short link loss. */
  linkCurrent(): boolean {
    return this.session.linkSuspended || this.session.artifact.socket === this.session.socket;
  }

  /** 🔌️ Keeps a MOUNTED child through a short link loss (cold pair applied, surface painted): actions keep applying and
   * their backbone effects queue in the outbox. A child still activating is not resumable and returns `false`. */
  suspendLink(): boolean {
    if (this.session.closed || this.session.child === null || this.session.coldApplied === null || !this.session.renderedUiPatch || !this.session.documentBackboneReady || this.session.linkSuspended) return false;
    this.session.linkSuspended = true;
    this.session.socket = null;
    return true;
  }

  get suspended(): boolean {
    return this.session.linkSuspended && !this.session.closed;
  }

  /** 🔁️ Adopts the re-admitted grant of the same hub actor (see `DocumentExecutionTargetLease.readmitBrowserActor`). */
  adoptGrant(grant: DocumentBrowserActorGrant): void {
    if (!this.session.linkSuspended || grant.actorId !== this.session.grant.actorId) throw new Error("document browser actor: grant actor mismatch");
    this.session.grant = grant;
  }

  /** 🔁️ Binds the socket whose `Session` re-admitted this child's actor; the missed tail and the queued outbox then
   * travel through the ordinary resume path (`Welcome` None/Tail, `Commands`, `Session` flush). */
  resumeLink(socket: WebSocket): boolean {
    if (!this.suspended || this.session.lease.browserActorGrant() !== this.session.grant) return false;
    this.session.linkSuspended = false;
    this.session.socket = socket;
    return true;
  }
}
