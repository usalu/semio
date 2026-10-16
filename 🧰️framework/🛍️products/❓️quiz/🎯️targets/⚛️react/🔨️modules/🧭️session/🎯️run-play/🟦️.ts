/** 🎯️ What the learner does in a run: starting it, opening a task of a timed one, giving answers and submitting. Answers apply locally at once and travel through the outbox; the other intents are commands decided by the proctor or the deputy.
 *
 * @see ../../📮️outbox/🟦️.ts — where answers wait for the proctor
 */

import { type Answer, type BadgeAwardedEvent, type Challenge, type Id, type OpenTaskCommand, type RunSubmittedEvent, type Slug, type StartRunCommand, type SubmitRunCommand } from "@semio-tech/quiz";
import { type Outbox } from "../../📮️outbox/🟦️.ts";
import { newId } from "../../🛂️proctor/🟦️.ts";
import { type HeldState } from "../🗃️held-state/🟦️.ts";
import { type DecisionRouting } from "../🏛️decision-routing/🟦️.ts";
import { type ProctorLink } from "../📡️proctor-link/🟦️.ts";
import { type ProctorReads } from "../📖️proctor-reads/🟦️.ts";
import { type Deputising } from "../🫡️deputising/🟦️.ts";
import { openRunOf, overdue } from "../🧮️state/🟦️.ts";
import { failureOf, type SessionFailure, type SubmissionPhase } from "../📋️session-model/🟦️.ts";

//#region 🎯️RunPlay
/** 🕹️ The run intents of one session. */
export class RunPlay {
  constructor(
    private readonly state: HeldState,
    private readonly link: ProctorLink,
    private readonly outbox: Outbox,
    private readonly routing: DecisionRouting,
    private readonly deputising: Deputising,
    private readonly reads: ProctorReads,
  ) {}

  /** 🚀️ Starts a run of `quiz` at `challenge` at the instant the learner started it by the session clock — the run's
   * start wherever and whenever it is decided — and opens it; an already open run of the quiz at that challenge is
   * resumed instead, and one at another challenge is voided by the start — the learner asked for that, so no notice says
   * it. */
  async startRun(quiz: Slug, challenge: Challenge, signal: AbortSignal): Promise<SessionFailure | undefined> {
    const learner = this.state.learnerId();
    if (learner === undefined) return { kind: "rejected", rejection: "unknown-learner" };
    const command: StartRunCommand = { type: "start-run", id: newId(), learner, run: newId(), quiz, challenge, at: this.link.now() };
    const { verdict, by } = await this.routing.decided(command, signal);
    if (verdict.kind === "rejected" && verdict.rejection === "run-open") {
      if (by === "proctor") this.reads.adoptLearnerView(await this.link.projected((current) => this.link.proctor.learner(learner, current), signal));
      const open = openRunOf(this.state.read(), quiz);
      return open === undefined ? failureOf(verdict) : this.resumeRun(open, signal);
    }
    if (verdict.kind !== "accepted") return failureOf(verdict);
    for (const event of verdict.events) {
      if (event.type !== "run-voided") continue;
      this.outbox.discard(event.run);
      this.state.dispatch({ type: "run-voided", run: event.run });
    }
    return this.resumeRun(command.run, signal);
  }

  /** ⏯️ Opens a run, loading it first when this device has not seen it yet. */
  async resumeRun(run: Id, signal: AbortSignal): Promise<undefined> {
    if (this.state.read().runs[run] === undefined) await this.reads.loadRun(run, signal);
    else void this.reads.loadRun(run);
    this.state.dispatch({ type: "step-opened", step: { screen: "run", run } });
    void this.reads.refreshLearner();
    return undefined;
  }

  /** ⏱️ Opens a task of a timed run: its clock starts at the instant the learner acted by the session clock. Decided
   * like a start — by the proctor, or by the deputy while the proctor is away, but only once the run's view is held: it
   * is loaded first when the device knows the run from its listing alone — and a task the held view already holds as
   * opened needs no command. Once it resolves without a failure, the held view holds the opening. `already-opened` is
   * success — the task is open elsewhere, so the run is read again when the held view lacks the opening. A revised quiz
   * voids the run; a run that closed is read again. */
  async openTask(run: Id, task: Slug, signal: AbortSignal): Promise<SessionFailure | undefined> {
    const learner = this.state.learnerId();
    if (learner === undefined) return { kind: "rejected", rejection: "unknown-learner" };
    if (this.state.read().runs[run] === undefined) await this.reads.loadRun(run, signal);
    const opened = this.state.read().runs[run]?.opened;
    if (opened !== undefined && Object.hasOwn(opened, task)) return undefined;
    const command: OpenTaskCommand = { type: "open-task", id: newId(), learner, run, task, at: this.link.now() };
    const { verdict, by } = await this.routing.decided(command, signal);
    if (verdict.kind === "accepted") {
      if (by === "deputy") return undefined;
      const event = verdict.events.find((candidate) => candidate.type === "task-opened" && candidate.run === run && candidate.task === task);
      if (event === undefined) await this.reads.loadRun(run, signal);
      else this.state.dispatch({ type: "task-opened", run, task, at: event.at });
      return undefined;
    }
    if (verdict.kind === "rejected" && verdict.rejection === "already-opened") {
      const held = this.state.read().runs[run]?.opened;
      if (held === undefined || !Object.hasOwn(held, task)) await this.reads.loadRun(run, signal);
      return undefined;
    }
    if (verdict.kind === "rejected" && verdict.rejection === "quiz-revised") this.reads.voided(run);
    if (verdict.kind === "rejected" && (verdict.rejection === "run-closed" || verdict.rejection === "unknown-run")) void this.reads.loadRun(run);
    return failureOf(verdict);
  }

  /** ✍️ Records an answer given now by the session clock: applied locally at once, delivered through the outbox — its
   * instant is the one the deciders judge, also once a later edit of the same task replaced it in the queue. An answer
   * to a task whose time is up by the session clock is refused here as the deciders would refuse it: nothing changes and
   * nothing is sent, and the learner is told. The hints of the task's former answer go at once; on a run that hints, a
   * deputy tells those of the new answer right away, else they arrive with the proctor's view of the run once its
   * answers are delivered. */
  answer(run: Id, task: Slug, answer: Answer): void {
    const learner = this.state.learnerId();
    const view = this.state.read().runs[run];
    if (learner === undefined || view === undefined || view.status !== "open") return;
    const at = this.link.now();
    if (overdue(view, task, at)) return this.state.dispatch({ type: "notice-raised", notice: { kind: "rejection", rejection: "time-up" } });
    this.state.dispatch({ type: "answer-given", run, task, answer });
    this.reads.noteEdit(run, task);
    this.outbox.enqueue({ type: "record-answer", id: newId(), learner, run, task, answer, at });
    this.deputising.rehint(run);
  }

  /** 📨️ Submits a run once all its answers are delivered, reporting each phase; `signal` cancels between and during
   * phases. A cancellation can race the proctor committing the submission, so every cancelled submission is
   * reconciled: the run view is read again and, if the proctor did submit it, adopted with its result. */
  async submit(run: Id, signal: AbortSignal, onPhase: (phase: SubmissionPhase) => void): Promise<SessionFailure | undefined> {
    try {
      return await this.submitting(run, signal, onPhase);
    } catch (error) {
      if (signal.aborted) void this.reads.loadRun(run);
      throw error;
    }
  }

  private async submitting(run: Id, signal: AbortSignal, onPhase: (phase: SubmissionPhase) => void): Promise<SessionFailure | undefined> {
    const learner = this.state.learnerId();
    if (learner === undefined) return { kind: "rejected", rejection: "unknown-learner" };
    const delivered = this.routing.direct() && (await this.routing.saved(run, signal, onPhase));
    if (this.state.read().runs[run]?.status === "voided") return { kind: "rejected", rejection: "quiz-revised" };
    onPhase({ phase: "submitting" });
    const command: SubmitRunCommand = { type: "submit-run", id: newId(), learner, run };
    const verdict = delivered || !this.routing.decidable(command) ? (await this.routing.decided(command, signal)).verdict : this.deputising.deputise(command);
    if (verdict.kind === "refused" || (verdict.kind === "rejected" && verdict.rejection !== "run-closed")) return failureOf(verdict);
    onPhase({ phase: "results" });
    const events = verdict.kind === "accepted" ? verdict.events : [];
    const submitted = events.find((event): event is RunSubmittedEvent => event.type === "run-submitted" && event.run === run);
    if (events.some((event) => event.type === "run-voided" && event.run === run)) {
      this.reads.voided(run);
      return { kind: "rejected", rejection: "quiz-revised" };
    }
    if (submitted !== undefined) {
      const badges = events.filter((event): event is BadgeAwardedEvent => event.type === "badge-awarded" && event.run === run).map((event) => event.badge);
      this.state.dispatch({ type: "run-submitted", run, result: submitted.result, badges, at: submitted.at });
    } else {
      const view = await this.reads.loadRun(run, signal);
      if (view?.status !== "submitted") return { kind: "rejected", rejection: view?.status === "voided" ? "quiz-revised" : "run-closed" };
      this.state.dispatch({ type: "step-opened", step: { screen: "results", run } });
    }
    void this.reads.refreshLearner();
    const quiz = this.state.read().runs[run]?.quiz;
    if (quiz !== undefined) void this.reads.refreshCrowd(quiz);
    return undefined;
  }
}
//#endregion 🎯️RunPlay
