/** 🙋️ How a device becomes a learner: asking who holds a pseudonym or name, recalling its holder or registering a free one, and waiting out a spent sign-up allowance. */

import { emptyLearnerState, type HandleView, type Id, type Identity, type IdentityClaim, type IdentifyLearnerCommand } from "@semio-tech/quiz";
import { SIGN_UP_ALLOWANCE, errorRejection, newId, signUpsSpent } from "../../🛂️proctor/🟦️.ts";
import { type HeldState } from "../🗃️held-state/🟦️.ts";
import { type DecisionRouting } from "../🏛️decision-routing/🟦️.ts";
import { type ProctorLink } from "../📡️proctor-link/🟦️.ts";
import { type ProctorReads } from "../📖️proctor-reads/🟦️.ts";
import { type Deputising } from "../🫡️deputising/🟦️.ts";
import { IDENTIFY_ATTEMPTS, failureOf, registration, type Decided, type SessionFailure } from "../📋️session-model/🟦️.ts";

//#region 🙋️Registration
/** 🪪️ The registration of one session. It owns the instant until which the proctor takes no further sign-up from this address. */
export class Registration {
  private signUpsFrom = 0;

  constructor(
    private readonly state: HeldState,
    private readonly link: ProctorLink,
    private readonly routing: DecisionRouting,
    private readonly deputising: Deputising,
    private readonly reads: ProctorReads,
  ) {}

  /** 🤝️ Makes this device a learner. An anonymous learner is registered under a fresh id. For a pseudonym or name the
   * proctor is asked who holds it: a claimed one is recalled — this device continues as its holder, nothing is written
   * — and a free one is registered; when someone else claims it in between, the proctor is asked again. When the
   * proctor says that the sign-up allowance of this address is spent, the sign-up is not sent again: the answer is the
   * `waiting` failure, and until the wait the proctor named is over no further sign-up leaves this device — a recall,
   * which is a read, still does. While the proctor is away a deputy registers the learner on the device: it knows no
   * holder of any handle, so the proctor settles later whose the handle is. */
  async identify(identity: IdentityClaim, signal: AbortSignal): Promise<SessionFailure | undefined> {
    for (let attempt = 0; attempt < IDENTIFY_ATTEMPTS; attempt += 1) {
      let display: string | undefined;
      if (identity.kind !== "anonymous") {
        let held: HandleView;
        try {
          held = await this.routing.holding(identity.handle, signal);
        } catch (error) {
          const rejection = errorRejection(error);
          if (rejection === undefined) throw error;
          return { kind: "rejected", rejection };
        }
        if (held.holder !== undefined) return this.adopt(held.holder.learner, held.holder.identity);
        display = held.display;
      }
      const learner = newId();
      const outcome = await this.register(learner, identity, signal);
      if ("kind" in outcome) return outcome;
      const { verdict, by } = outcome;
      if (verdict.kind === "accepted") {
        const registered = verdict.events.find(registration);
        return this.adopt(registered?.learner ?? learner, registered?.identity ?? (identity.kind === "anonymous" ? identity : { kind: identity.kind, handle: display ?? identity.handle }), by);
      }
      if (verdict.kind !== "rejected" || verdict.rejection !== (identity.kind === "anonymous" ? "learner-exists" : "handle-claimed")) return failureOf(verdict);
    }
    return { kind: "rejected", rejection: identity.kind === "anonymous" ? "learner-exists" : "handle-claimed" };
  }

  private async register(learner: Id, identity: IdentityClaim, signal: AbortSignal): Promise<Decided | Extract<SessionFailure, { readonly kind: "waiting" }>> {
    const left = this.signUpsFrom - this.link.now();
    if (left > 0) return { kind: "waiting", allowance: SIGN_UP_ALLOWANCE, retryAfterMs: left };
    const command: IdentifyLearnerCommand = { type: "identify-learner", id: newId(), learner, identity };
    try {
      return await this.routing.decided(command, signal);
    } catch (error) {
      if (!signUpsSpent(error)) throw error;
      this.signUpsFrom = this.link.now() + (error.retryAfterMs ?? 0);
      return error.retryAfterMs === undefined ? { kind: "waiting", allowance: SIGN_UP_ALLOWANCE } : { kind: "waiting", allowance: SIGN_UP_ALLOWANCE, retryAfterMs: error.retryAfterMs };
    }
  }

  private adopt(learner: Id, registered: Identity, by: Decided["by"] = "proctor"): undefined {
    this.state.dispatch({ type: "learner-identified", learner, identity: registered });
    if (by === "deputy") this.deputising.fold({ ...emptyLearnerState(learner), identity: registered }, []);
    void this.reads.refreshLearner();
    return undefined;
  }
}
//#endregion 🙋️Registration
