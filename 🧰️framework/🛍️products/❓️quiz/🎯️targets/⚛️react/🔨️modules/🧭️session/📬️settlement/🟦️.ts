/** 📬️ The outbox's two ways into the session: how a queued command is delivered to the proctor — a registration only once the proctor knows its learner — and what the session does with the verdict of every command once it is settled.
 *
 * @see ../../📮️outbox/🟦️.ts — the queue that calls both
 */

import { challengeRules, type Command, type Id, type Identity, type IdentifyLearnerCommand } from "@semio-tech/quiz";
import { commandRun, type Outbox } from "../../📮️outbox/🟦️.ts";
import { ProctorThrottled, ProctorUnavailable, RETRY_AFTER_MAX_MS, isNotFound, signUpsSpent, type CommandVerdict } from "../../🛂️proctor/🟦️.ts";
import { type HeldState } from "../🗃️held-state/🟦️.ts";
import { type DecisionRouting } from "../🏛️decision-routing/🟦️.ts";
import { type ProctorLink } from "../📡️proctor-link/🟦️.ts";
import { type ProctorReads } from "../📖️proctor-reads/🟦️.ts";
import { type Deputising } from "../🫡️deputising/🟦️.ts";

//#region 📬️Settlement
/** 🚛️ The delivery and settlement of one session's commands. It owns the runs whose view must be read again once the proctor has heard of every decision about them. */
export class Settlement {
  private readonly unconfirmed = new Set<Id>();

  constructor(
    private readonly state: HeldState,
    private readonly link: ProctorLink,
    private readonly outbox: Outbox,
    private readonly routing: DecisionRouting,
    private readonly reads: ProctorReads,
    private readonly deputising: Deputising,
  ) {}

  /** 🚚️ Delivers one queued command to the proctor. A spent sign-up allowance and a full roster are waited out, never
   * given up on: the learner goes on playing on the device meanwhile. A registration is only delivered once the
   * proctor knows its learner ({@link Settlement.enrolled}), so the commands behind it find that learner. */
  async deliver(command: Command, signal: AbortSignal): Promise<CommandVerdict> {
    let verdict: CommandVerdict;
    try {
      verdict = await this.link.proctor.command(command, signal);
    } catch (error) {
      if (signUpsSpent(error)) throw new ProctorThrottled(error.retryAfterMs ?? RETRY_AFTER_MAX_MS);
      throw error;
    }
    return command.type === "identify-learner" ? this.enrolled(command, verdict, signal) : verdict;
  }

  /** ⚖️ What the proctor decided about `command` once it is settled: an answer or a decision. */
  settled(command: Command, verdict: CommandVerdict): void {
    if (command.type === "record-answer") this.answerSettled(command.run, verdict);
    else this.decisionSettled(command, verdict);
  }

  /** 🎫️ The verdict of a delivered registration once the proctor knows the learner. A handle that turned out to be
   * claimed is recalled: the device continues as its holder ({@link Settlement.recalled}) before the next command leaves. A
   * registration the proctor already holds is accepted. The proctor relays a registration under a handle to its learner
   * a moment later, so the learner view is awaited — when it stays missing, the delivery counts as not answered. */
  private async enrolled(command: IdentifyLearnerCommand, verdict: CommandVerdict, signal: AbortSignal): Promise<CommandVerdict> {
    let learner = command.learner;
    if (verdict.kind === "refused") return verdict;
    if (verdict.kind === "rejected") {
      if (verdict.rejection === "roster-full") throw new ProctorThrottled(RETRY_AFTER_MAX_MS);
      if (verdict.rejection === "handle-claimed" && command.identity.kind !== "anonymous") {
        const { holder } = await this.link.proctor.handle(command.identity.handle, signal);
        if (holder === undefined) throw new ProctorUnavailable("the holder of the handle is not known yet");
        if (holder.learner !== learner) this.recalled(learner, holder.learner, holder.identity);
        learner = holder.learner;
      } else if (verdict.rejection !== "learner-exists") return verdict;
    }
    try {
      await this.link.projected((current) => this.link.proctor.learner(learner, current), signal);
    } catch (error) {
      throw isNotFound(error) ? new ProctorUnavailable("the learner is not enrolled yet") : error;
    }
    return { kind: "accepted", events: verdict.kind === "accepted" ? verdict.events : [] };
  }

  /** 🪪️ The learner this device registered as `from` is the learner `to` of the proctor: every queued command and
   * everything held moves to that id, and the learner is told when it is the one on screen. */
  private recalled(from: Id, to: Id, identity: Identity): void {
    this.outbox.reassign(from, to);
    if (this.state.learnerId() !== from) return;
    this.state.dispatch({ type: "learner-recalled", from, to, identity });
    if (identity.kind !== "anonymous") this.state.dispatch({ type: "notice-raised", notice: { kind: "recalled", handle: identity.handle } });
  }

  /** 📭️ What the proctor decided about a command the deputy had decided. An accepted one needs nothing now — a run the
   * proctor voided with it is voided here too. A run the proctor would not start never existed for it, and a run whose
   * task the proctor would not open because the quiz was revised is void for it like one whose answer met the revision:
   * either is voided on the device. A task the proctor already holds as opened (`already-opened`) is open, as the
   * device wants it. Any other refusal is told. Once the device is no longer ahead, the proctor's views are
   * read again: the learner, and every run the proctor heard of meanwhile. */
  private decisionSettled(command: Exclude<Command, { readonly type: "record-answer" }>, verdict: CommandVerdict): void {
    const run = commandRun(command);
    if (run !== undefined) this.unconfirmed.add(run);
    if (verdict.kind === "accepted") {
      for (const event of verdict.events) {
        if (event.type !== "run-voided") continue;
        if (event.run === run) this.reads.voided(event.run);
        else {
          this.outbox.discard(event.run);
          this.state.dispatch({ type: "run-voided", run: event.run });
        }
      }
    } else if (command.type === "start-run" || (command.type === "open-task" && verdict.kind === "rejected" && verdict.rejection === "quiz-revised")) {
      this.unconfirmed.delete(command.run);
      this.reads.voided(command.run);
    } else if (verdict.kind === "refused") this.state.dispatch({ type: "notice-raised", notice: { kind: "refused", detail: verdict.detail } });
    else if ((command.type !== "submit-run" || verdict.rejection !== "run-closed") && (command.type !== "open-task" || verdict.rejection !== "already-opened")) this.state.dispatch({ type: "notice-raised", notice: { kind: "rejection", rejection: verdict.rejection } });
    if (this.routing.ahead()) return;
    void this.reads.refreshLearner();
    for (const confirmed of this.unconfirmed) void this.reads.loadRun(confirmed);
    this.unconfirmed.clear();
  }

  /** 📪️ What the proctor decided about an answer. Without a deputy, the hints of a run that hints come with the
   * proctor's view of it: it is read again once none of its answers waits any more. An answer refused because its
   * task's time was up or its clock never started is told, and the run is read again, so the device shows what the
   * proctor recorded — at once, whatever the deputy decided about other runs, or, while the proctor's view of this run
   * lacks a decision of the deputy, once that decision is delivered. */
  private answerSettled(run: Id, verdict: CommandVerdict): void {
    if (verdict.kind === "accepted") {
      const view = this.state.read().runs[run];
      if (!this.deputising.present && view?.status === "open" && challengeRules(view.sheet.challenge).hints && this.outbox.queued(run).length === 0) void this.reads.loadRun(run);
      return;
    }
    if (verdict.kind === "refused") return this.state.dispatch({ type: "notice-raised", notice: { kind: "refused", detail: verdict.detail } });
    if (verdict.rejection === "quiz-revised") return this.reads.voided(run);
    if (verdict.rejection === "run-closed" || verdict.rejection === "unknown-run") return void this.reads.loadRun(run);
    this.state.dispatch({ type: "notice-raised", notice: { kind: "rejection", rejection: verdict.rejection } });
    if (verdict.rejection !== "time-up" && verdict.rejection !== "task-unopened") return;
    if (this.routing.behind(run)) this.unconfirmed.add(run);
    else void this.reads.loadRun(run);
  }
}
//#endregion 📬️Settlement
