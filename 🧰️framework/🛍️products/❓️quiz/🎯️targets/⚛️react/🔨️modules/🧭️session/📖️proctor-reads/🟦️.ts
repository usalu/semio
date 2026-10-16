/** 📖️ What the session reads from the proctor and how it keeps the held views true to it: the catalog, the learner view, the crowd, the leaderboard and the run views, each retried through connection shortages and projection lag, and the follow-up of a run that closed. */

import { latestWins } from "@semio-tech/framework";
import { type Id, type LearnerView, type RunView, type Slug } from "@semio-tech/quiz";
import { coalescingKey, type Outbox } from "../../📮️outbox/🟦️.ts";
import { ProctorThrottled, isNotFound } from "../../🛂️proctor/🟦️.ts";
import { type HeldState } from "../🗃️held-state/🟦️.ts";
import { type DecisionRouting } from "../🏛️decision-routing/🟦️.ts";
import { type Lifecycle } from "../🔌️lifecycle/🟦️.ts";
import { type ProctorLink } from "../📡️proctor-link/🟦️.ts";
import { type OutboxOverlay } from "../🧺️outbox-overlay/🟦️.ts";
import { type Deputising } from "../🫡️deputising/🟦️.ts";
import { DEFAULT_BOARD, boardKey, keptFresh, overallLeaderboard } from "../🧮️state/🟦️.ts";
import { describe, type RefreshOutcome } from "../📋️session-model/🟦️.ts";

//#region 📖️ProctorReads
/** 🛰️ The reads of one session. It owns the tickets that order the reads of a run and the stamps of the answers this device gave, so a read overtaken by a later one, or older than an answer, never wins. */
export class ProctorReads {
  private edits = 0;
  private readonly edited = new Map<string, number>();
  private reads = 0;
  private readonly adopted = new Map<Id, number>();

  constructor(
    private readonly state: HeldState,
    private readonly link: ProctorLink,
    private readonly lifecycle: Lifecycle,
    private readonly outbox: Outbox,
    private readonly overlay: OutboxOverlay,
    private readonly deputising: Deputising,
    private readonly routing: DecisionRouting,
  ) {}

  /** 📚️ Reloads the catalog, retrying through connection shortages; concurrent calls collapse into one. */
  readonly refreshCatalog = latestWins(async (): Promise<void> => {
    const catalog = await this.background((signal) => this.link.proctor.catalog(this.state.learnerId(), signal));
    if (catalog !== undefined) this.state.dispatch({ type: "catalog-loaded", catalog });
  });

  /** 🔃️ Reloads the learner view, retrying through connection shortages and projection lag; concurrent calls collapse
   * into one. A learner the proctor still does not know afterwards is forgotten on this device and asked to identify.
   * While the device is ahead of the proctor nothing is read: the proctor has not heard everything yet. */
  readonly refreshLearner = latestWins(async (): Promise<void> => {
    const learner = this.state.learnerId();
    if (learner === undefined || this.routing.ahead()) return;
    const signal = this.lifecycle.signal;
    try {
      const view = await this.link.projected((lifetime) => this.link.proctor.learner(learner, lifetime), signal);
      if (!this.routing.ahead()) this.adoptLearnerView(view);
    } catch (error) {
      if (signal.aborted || this.state.learnerId() !== learner || this.routing.ahead()) return;
      if (!isNotFound(error)) return this.state.dispatch({ type: "notice-raised", notice: { kind: "refused", detail: describe(error) } });
      this.outbox.forget(learner);
      this.state.dispatch({ type: "learner-forgotten" });
      this.state.dispatch({ type: "notice-raised", notice: { kind: "rejection", rejection: "unknown-learner" } });
    }
  });

  /** 📈️ Asks for the leaderboard the learner looks at once; polling repeats it, so a failure only keeps the last known
   * standings and tells the poller to slow down. While the learner looks at another leaderboard than the overall one,
   * that one — which says the learner's rank on the profile — is asked for too whenever the answer says that runs were
   * submitted since it was held. When the proctor does not answer — or not within the learner's patience — a deputy
   * fills in the device's own standing for every leaderboard the proctor never answered here. */
  readonly refreshLeaderboard = latestWins(async (): Promise<RefreshOutcome> => {
    const spent = setTimeout(() => this.deputising.ownStanding(), this.link.patienceMs);
    try {
      const { board, learner } = this.state.read();
      const leaderboard = await this.link.proctor.leaderboard(board, learner?.id, this.lifecycle.signal);
      this.state.dispatch({ type: "leaderboard-loaded", leaderboard, at: this.link.now() });
      if (boardKey(board) !== boardKey(DEFAULT_BOARD) && overallLeaderboard(this.state.read())?.board.submissions !== leaderboard.submissions)
        this.state.dispatch({ type: "leaderboard-loaded", leaderboard: await this.link.proctor.leaderboard(DEFAULT_BOARD, learner?.id, this.lifecycle.signal), at: this.link.now() });
      return { answered: true };
    } catch (error) {
      this.deputising.ownStanding();
      return error instanceof ProctorThrottled && error.retryAfterMs !== undefined ? { answered: false, retryAfterMs: error.retryAfterMs } : { answered: false };
    } finally {
      clearTimeout(spent);
    }
  });

  /** 👥️ Asks for the submitted crowd of `quiz` once; a failure keeps the last known crowd. */
  async refreshCrowd(quiz: Slug): Promise<void> {
    try {
      this.state.dispatch({ type: "crowd-loaded", crowd: await this.link.proctor.crowd(quiz, this.state.learnerId(), this.lifecycle.signal) });
    } catch {
      return;
    }
  }

  /** ✍️ Stamps that the learner changed the answer to `task` of `run` on this device just now. */
  noteEdit(run: Id, task: Slug): void {
    this.edited.set(coalescingKey({ run, task }), (this.edits += 1));
  }

  /** 🧾️ Loads a run view, waiting out projection lag. The proctor's view wins — including answers another device gave
   * — except for the answers it cannot hold yet, with their hints: those still waiting in the outbox, and those this
   * device gave of a task that had one waiting when the read was asked for or that it answered since. A read overtaken
   * by a later one of the same run is dropped. A run that closed meanwhile is followed: to its results when submitted,
   * home with a notice when voided. `signal` makes it interactive, else it runs in the background. While the proctor's
   * view of the run lacks a decision of the deputy ({@link DecisionRouting.behind}) the held view is the answer. */
  async loadRun(run: Id, signal?: AbortSignal): Promise<RunView | undefined> {
    const learner = this.state.learnerId();
    if (learner === undefined) return undefined;
    if (this.routing.behind(run)) return this.state.read().runs[run];
    const ticket = (this.reads += 1);
    const edits = this.edits;
    const unsettled = Object.keys(this.outbox.pendingAnswers(run));
    const read = (current: AbortSignal): Promise<RunView> => this.link.proctor.run(run, learner, current);
    const loaded = signal === undefined ? await this.background(read) : await this.link.projected(read, signal);
    if (loaded === undefined || this.state.learnerId() !== learner) return undefined;
    if (this.routing.behind(run) || (this.adopted.get(run) ?? 0) > ticket) return this.state.read().runs[run];
    this.adopted.set(run, ticket);
    const cached = this.state.read().runs[run];
    const fresh = (task: Slug): boolean => unsettled.includes(task) || (this.edited.get(coalescingKey({ run, task })) ?? 0) > edits;
    const view = this.overlay.withPending(cached === undefined ? loaded : keptFresh(loaded, cached, fresh));
    this.state.dispatch({ type: "run-loaded", view });
    this.deputising.rehint(run);
    this.followClosure(cached, view);
    return this.state.read().runs[run] ?? view;
  }

  /** 🧲️ Takes in the learner view the proctor sent (or another tab stored, with `persist` off) and follows every run it shows closed. */
  adoptLearnerView(view: LearnerView, persist = true): void {
    const before = this.state.read().runs;
    this.state.dispatch({ type: "learner-loaded", view }, persist);
    for (const summary of view.runs) this.followClosure(before[summary.run], this.state.read().runs[summary.run], false);
  }

  /** 🏁️ Follows a run that closed between `before` and `after`: its queue is dropped, a voided run sends the learner home with a notice, a submitted one to its results; the learner view is read again unless `refresh` is off. */
  followClosure(before: RunView | undefined, after: RunView | undefined, refresh = true): void {
    if (before?.status !== "open" || after === undefined || after.status === "open") return;
    const { step } = this.state.read();
    const onRun = step.screen === "run" && step.run === after.run;
    this.overlay.retire(after.run);
    if (after.status === "voided") {
      if (onRun) this.state.dispatch({ type: "step-opened", step: { screen: "home" } });
      this.state.dispatch({ type: "notice-raised", notice: { kind: "voided" } });
    } else if (onRun) this.state.dispatch({ type: "step-opened", step: { screen: "results", run: after.run } });
    if (refresh) void this.refreshLearner();
  }

  /** 💥️ Voids `run` on this device: its queue is dropped, the learner is told and the learner view is read again. */
  voided(run: Id): void {
    this.outbox.discard(run);
    this.state.dispatch({ type: "run-voided", run });
    this.state.dispatch({ type: "notice-raised", notice: { kind: "voided" } });
    void this.refreshLearner();
  }

  private async background<T>(call: (signal: AbortSignal) => Promise<T>): Promise<T | undefined> {
    const signal = this.lifecycle.signal;
    try {
      return await this.link.projected(call, signal);
    } catch (error) {
      if (!signal.aborted) this.state.dispatch({ type: "notice-raised", notice: { kind: "refused", detail: describe(error) } });
      return undefined;
    }
  }
}
//#endregion 📖️ProctorReads
