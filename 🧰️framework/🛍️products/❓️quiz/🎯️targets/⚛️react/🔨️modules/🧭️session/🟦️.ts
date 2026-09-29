/** 🧭️ The quiz client session: local-first, event-driven state of one learner on one device, and the controller that
 * turns learner intent into proctor commands and folds what comes back into that state.
 *
 * State only changes through {@link QuizClientEvent}s folded by the pure {@link evolveQuizState}. Slices are kept apart
 * by lifetime: persisted local-only (introduction seen, learner id, cached catalog, learner view, one record per run
 * view, the answer outbox), persisted shared (the proctor's events, reached only through commands), ephemeral shared
 * (the leaderboard, polled) and ephemeral local-only (the current step, notices, newly earned badges). The site's tabs
 * share the persisted local-only slices: what another tab stores is adopted without being written back — a run record
 * by {@link mergeRunViews} — so tabs never undo each other. Answers apply locally at once and travel through the
 * {@link Outbox}; interactive commands retry transient failures with jittered backoff under one command id and are
 * cancellable through their `AbortSignal`.
 *
 * @see ../../../../🧬️schema/🔣️.json — commands, events, views
 */

import { latestWins } from "@semio-tech/framework";
import type {
  Answer,
  BadgeAwardedEvent,
  CatalogView,
  Event,
  Id,
  Identity,
  IdentifyLearnerCommand,
  Leaderboard,
  LearnerView,
  Rejection,
  RunResult,
  RunSubmittedEvent,
  RunSummary,
  RunView,
  Slug,
  StartRunCommand,
  SubmitRunCommand,
} from "@semio-tech/quiz";
import { isRecord, type LocalChange, type LocalStore } from "../💾️persistence/🟦️.ts";
import { Outbox, type OutboxActivity } from "../📮️outbox/🟦️.ts";
import { ProctorUnavailable, RETRY_TIMING, isNotFound, newId, retryTransient, type CommandVerdict, type ProctorClient, type ProctorReachability, type RetryTiming } from "../🛂️proctor/🟦️.ts";

//#region 🧭️State
/** 🪧️ The screen the learner is on (ephemeral local-only). */
export type QuizStep =
  { readonly screen: "introduction" } | { readonly screen: "identity" } | { readonly screen: "home" } | { readonly screen: "run"; readonly run: Id } | { readonly screen: "results"; readonly run: Id } | { readonly screen: "leaderboard" };

/** 🧑‍🎓️ The learner this device acts as; the identity is known once registered here or loaded from the proctor. */
export interface QuizLearner {
  readonly id: Id;
  readonly identity?: Identity;
}

/** 📣️ A message the learner should read once. */
export type QuizNotice = { readonly kind: "rejection"; readonly rejection: Rejection } | { readonly kind: "refused"; readonly detail: string } | { readonly kind: "voided" };

/** 🧭️ Everything the client renders. */
export interface QuizState {
  readonly step: QuizStep;
  readonly introduced: boolean;
  readonly learner?: QuizLearner;
  readonly catalog?: CatalogView;
  readonly learnerView?: LearnerView;
  readonly runs: Readonly<Record<Id, RunView>>;
  readonly awards: Readonly<Record<Id, readonly Slug[]>>;
  readonly leaderboard?: { readonly board: Leaderboard; readonly at: number };
  readonly notice?: QuizNotice;
}

/** ⚡️ A fact of the client session. */
export type QuizClientEvent =
  | { readonly type: "introduction-read" }
  | { readonly type: "step-opened"; readonly step: QuizStep }
  | { readonly type: "catalog-loaded"; readonly catalog: CatalogView }
  | { readonly type: "learner-identified"; readonly learner: Id; readonly identity?: Identity }
  | { readonly type: "learner-forgotten" }
  | { readonly type: "learner-loaded"; readonly view: LearnerView }
  | { readonly type: "run-loaded"; readonly view: RunView }
  | { readonly type: "answer-given"; readonly run: Id; readonly task: Slug; readonly answer: Answer }
  | { readonly type: "run-submitted"; readonly run: Id; readonly result: RunResult; readonly badges: readonly Slug[]; readonly at: number }
  | { readonly type: "run-voided"; readonly run: Id }
  | { readonly type: "leaderboard-loaded"; readonly leaderboard: Leaderboard; readonly at: number }
  | { readonly type: "notice-raised"; readonly notice: QuizNotice }
  | { readonly type: "notice-dismissed" };

/** 🌱️ The state a device starts with, from its persisted local-only slices. */
export function initialQuizState(persisted: Pick<QuizState, "introduced" | "learner" | "catalog" | "learnerView" | "runs">): QuizState {
  const step: QuizStep = !persisted.introduced ? { screen: "introduction" } : persisted.learner === undefined ? { screen: "identity" } : { screen: "home" };
  return { ...persisted, step, awards: {} };
}

/** 🔒️ A cached open run the learner view lists as closed (e.g. submitted on another device): closed here too; its
 * result arrives with the next run view. */
function closedRun(view: RunView, summary: RunSummary): RunView {
  return { ...view, status: summary.status, ...(summary.submittedAt === undefined ? {} : { submittedAt: summary.submittedAt }) };
}

function withRun(state: QuizState, run: Id, change: (view: RunView) => RunView): QuizState {
  const view = state.runs[run];
  return view === undefined ? state : { ...state, runs: { ...state.runs, [run]: change(view) } };
}

/** 🧮️ Folds one client event into the state. Pure. */
export function evolveQuizState(state: QuizState, event: QuizClientEvent): QuizState {
  switch (event.type) {
    case "introduction-read":
      return { ...state, introduced: true, step: state.learner === undefined ? { screen: "identity" } : { screen: "home" } };
    case "step-opened":
      return { ...state, step: event.step };
    case "catalog-loaded":
      return { ...state, catalog: event.catalog };
    case "learner-identified": {
      const same = state.learner?.id === event.learner;
      return {
        ...state,
        learner: { id: event.learner, identity: event.identity ?? (same ? state.learner?.identity : undefined) },
        learnerView: same ? state.learnerView : undefined,
        runs: same ? state.runs : {},
        awards: same ? state.awards : {},
        step: { screen: "home" },
      };
    }
    case "learner-forgotten":
      return { ...state, learner: undefined, learnerView: undefined, runs: {}, awards: {}, step: { screen: "identity" } };
    case "learner-loaded": {
      if (state.learner?.id !== event.view.learner) return state;
      const closed = event.view.runs.filter((summary) => summary.status !== "open" && state.runs[summary.run]?.status === "open");
      const runs = closed.length === 0 ? state.runs : { ...state.runs, ...Object.fromEntries(closed.map((summary) => [summary.run, closedRun(state.runs[summary.run]!, summary)])) };
      return { ...state, learnerView: event.view, learner: { id: event.view.learner, identity: event.view.identity }, runs };
    }
    case "run-loaded":
      if (state.learner?.id !== event.view.learner) return state;
      return { ...state, runs: { ...state.runs, [event.view.run]: event.view } };
    case "answer-given":
      return withRun(state, event.run, (view) => ({ ...view, answers: { ...view.answers, [event.task]: event.answer } }));
    case "run-submitted": {
      const submitted = withRun(state, event.run, (view) => ({ ...view, status: "submitted", result: event.result, submittedAt: event.at }));
      return { ...submitted, awards: { ...state.awards, [event.run]: event.badges }, step: { screen: "results", run: event.run } };
    }
    case "run-voided": {
      const voided = withRun(state, event.run, (view) => ({ ...view, status: "voided" }));
      const onRun = (state.step.screen === "run" || state.step.screen === "results") && state.step.run === event.run;
      return onRun ? { ...voided, step: { screen: "home" } } : voided;
    }
    case "leaderboard-loaded":
      return { ...state, leaderboard: { board: event.leaderboard, at: event.at } };
    case "notice-raised":
      return { ...state, notice: event.notice };
    case "notice-dismissed":
      return { ...state, notice: undefined };
  }
}

/** 🏅️ The badges `run` earned: announced by its submission or listed in the learner view. */
export function runAwards(state: QuizState, run: Id): readonly Slug[] {
  const listed = (state.learnerView?.badges ?? []).filter((award) => award.run === run).map((award) => award.badge);
  return [...new Set([...(state.awards[run] ?? []), ...listed])];
}

/** 🏃️ The open run of `quiz`, if the learner has one; a run this device already saw closed is not open, whatever a
 * lagging learner view still says. */
export function openRunOf(state: QuizState, quiz: Slug): Id | undefined {
  const closedHere = (run: Id): boolean => state.runs[run] !== undefined && state.runs[run].status !== "open";
  const listed = state.learnerView?.runs.find((summary) => summary.quiz === quiz && summary.status === "open" && !closedHere(summary.run))?.run;
  return listed ?? Object.values(state.runs).find((view) => view.quiz === quiz && view.status === "open" && view.learner === state.learner?.id)?.run;
}

/** 🏁️ The latest submitted run of `quiz`, from the learner view or from what this device submitted since. */
export function lastSubmittedRunOf(state: QuizState, quiz: Slug): Id | undefined {
  const listed = state.learnerView?.runs.find((summary) => summary.quiz === quiz && summary.status === "submitted");
  const here = Object.values(state.runs)
    .filter((view) => view.quiz === quiz && view.status === "submitted" && view.learner === state.learner?.id)
    .sort((left, right) => (right.submittedAt ?? 0) - (left.submittedAt ?? 0))[0];
  if (here === undefined) return listed?.run;
  return listed === undefined || (here.submittedAt ?? 0) >= (listed.submittedAt ?? 0) ? here.run : listed.run;
}

/** 🔀️ One run from this tab's copy and another tab's: a closed status (with its result) wins over an open one; while
 * both are open the answers are their union, the other tab's — written last — winning per task. */
export function mergeRunViews(mine: RunView | undefined, theirs: RunView): RunView {
  if (mine === undefined || theirs.status !== "open") return theirs;
  if (mine.status !== "open") return mine;
  return { ...theirs, answers: { ...mine.answers, ...theirs.answers } };
}
//#endregion 🧭️State

//#region 💾️Persistence
function restoredLearner(value: unknown): QuizLearner | undefined {
  if (!isRecord(value) || typeof value.id !== "string") return undefined;
  return isRecord(value.identity) && typeof value.identity.kind === "string" ? { id: value.id, identity: value.identity as unknown as Identity } : { id: value.id };
}

function restoredCatalog(value: unknown): CatalogView | undefined {
  return isRecord(value) && Array.isArray(value.quizzes) && Array.isArray(value.badges) ? (value as unknown as CatalogView) : undefined;
}

function restoredLearnerView(value: unknown): LearnerView | undefined {
  return isRecord(value) && typeof value.learner === "string" && Array.isArray(value.runs) && Array.isArray(value.badges) ? (value as unknown as LearnerView) : undefined;
}

function restoredRun(value: unknown, run: Id): RunView | undefined {
  return isRecord(value) && value.run === run && typeof value.learner === "string" && isRecord(value.sheet) && isRecord(value.answers) && typeof value.status === "string" ? (value as unknown as RunView) : undefined;
}

function restoredRuns(store: LocalStore, learner: Id | undefined): Readonly<Record<Id, RunView>> {
  const runs: Record<Id, RunView> = {};
  if (learner === undefined) return runs;
  for (const [run, value] of store.records("runs")) {
    const view = restoredRun(value, run);
    if (view?.learner === learner) runs[run] = view;
  }
  return runs;
}

/** 💾️ The persisted local-only slices and run records of `store` for its learner, dropping anything unreadable. */
export function restoreQuizState(store: LocalStore): QuizState {
  const learner = restoredLearner(store.read("learner"));
  return initialQuizState({
    introduced: store.read("introduced") === true,
    learner,
    catalog: restoredCatalog(store.read("catalog")),
    learnerView: restoredLearnerView(store.read("learner-view")),
    runs: restoredRuns(store, learner?.id),
  });
}

function persistQuizState(store: LocalStore, before: QuizState, after: QuizState): void {
  if (before.introduced !== after.introduced) store.write("introduced", after.introduced);
  if (before.learner !== after.learner) after.learner === undefined ? store.remove("learner") : store.write("learner", after.learner);
  if (before.catalog !== after.catalog) store.write("catalog", after.catalog);
  if (before.learnerView !== after.learnerView) after.learnerView === undefined ? store.remove("learner-view") : store.write("learner-view", after.learnerView);
  if (before.runs === after.runs) return;
  for (const [run, view] of Object.entries(after.runs)) if (before.runs[run] !== view) store.put("runs", run, view);
  for (const run of Object.keys(before.runs)) if (!Object.hasOwn(after.runs, run)) store.drop("runs", run);
}
//#endregion 💾️Persistence

//#region 🎮️Session
/** 🚫️ Why an interactive command did not go through. */
export type SessionFailure = { readonly kind: "rejected"; readonly rejection: Rejection } | { readonly kind: "refused"; readonly detail: string };

/** 📊️ The progress of a submission. */
export type SubmissionPhase = { readonly phase: "saving"; readonly done: number; readonly total: number } | { readonly phase: "submitting" } | { readonly phase: "results" };

/** 📶️ What the learner sees of the connection. */
export interface QuizConnection {
  readonly reachability: ProctorReachability;
  readonly online: boolean;
  readonly pending: number;
  readonly activity: OutboxActivity;
}

/** 📸️ One consistent render input: the state and the connection. */
export interface QuizSnapshot {
  readonly state: QuizState;
  readonly connection: QuizConnection;
}

/** ⚙️ What a {@link QuizSession} talks to. */
export interface QuizSessionOptions {
  readonly proctor: ProctorClient;
  readonly store: LocalStore;
  readonly timing?: RetryTiming;
  readonly now?: () => number;
}

/** ⏳️ How long a view this client itself caused may still be missing from the proctor's projections. */
export const PROJECTION_GRACE_MS = 10_000;

function stoppedLifetime(): AbortController {
  const lifetime = new AbortController();
  lifetime.abort();
  return lifetime;
}

function describe(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function failureOf(verdict: Exclude<CommandVerdict, { readonly kind: "accepted" }>): SessionFailure {
  return verdict.kind === "rejected" ? { kind: "rejected", rejection: verdict.rejection } : { kind: "refused", detail: verdict.detail };
}

function rosterEvent(event: Event): event is Extract<Event, { readonly type: "learner-registered" | "learner-recalled" }> {
  return event.type === "learner-registered" || event.type === "learner-recalled";
}

/** 🎮️ One learner session against one proctor tenant — an external store for the UI and the controller of its intents. */
export class QuizSession {
  readonly proctor: ProctorClient;
  readonly outbox: Outbox;
  private readonly store: LocalStore;
  private readonly timing: RetryTiming;
  private readonly now: () => number;
  private state: QuizState;
  private online = true;
  private snapshot: QuizSnapshot;
  private lifetime = stoppedLifetime();
  private readonly listeners = new Set<() => void>();
  private readonly teardown: (() => void)[] = [];

  /** 📚️ Reloads the catalog, retrying through connection shortages; concurrent calls collapse into one. */
  readonly refreshCatalog = latestWins(async (): Promise<void> => {
    const catalog = await this.background((signal) => this.proctor.catalog(this.state.learner?.id, signal));
    if (catalog !== undefined) this.dispatch({ type: "catalog-loaded", catalog });
  });

  /** 🧑‍🎓️ Reloads the learner view, retrying through connection shortages and projection lag; concurrent calls collapse
   * into one. A learner the proctor still does not know afterwards is forgotten on this device and asked to identify. */
  readonly refreshLearner = latestWins(async (): Promise<void> => {
    const learner = this.state.learner?.id;
    if (learner === undefined) return;
    const signal = this.lifetime.signal;
    try {
      this.adoptLearnerView(await this.projected((lifetime) => this.proctor.learner(learner, lifetime), signal));
    } catch (error) {
      if (signal.aborted || this.state.learner?.id !== learner) return;
      if (!isNotFound(error)) return this.dispatch({ type: "notice-raised", notice: { kind: "refused", detail: describe(error) } });
      this.outbox.forget(learner);
      this.dispatch({ type: "learner-forgotten" });
      this.dispatch({ type: "notice-raised", notice: { kind: "rejection", rejection: "unknown-learner" } });
    }
  });

  /** 🏆️ Asks for the leaderboard once; polling repeats it, so a failure only keeps the last known standings. */
  readonly refreshLeaderboard = latestWins(async (): Promise<void> => {
    try {
      const leaderboard = await this.proctor.leaderboard(this.state.learner?.id, this.lifetime.signal);
      this.dispatch({ type: "leaderboard-loaded", leaderboard, at: this.now() });
    } catch {
      return;
    }
  });

  constructor(options: QuizSessionOptions) {
    this.proctor = options.proctor;
    this.store = options.store;
    this.timing = options.timing ?? RETRY_TIMING;
    this.now = options.now ?? Date.now;
    this.outbox = new Outbox({ send: (command, signal) => this.proctor.command(command, signal), store: options.store, timing: this.timing, now: this.now, onSettled: (command, verdict) => this.answerSettled(command.run, verdict) });
    this.state = this.restored();
    this.snapshot = this.freeze();
  }

  /** 📸️ The current snapshot; the same object until something changes. */
  getSnapshot = (): QuizSnapshot => this.snapshot;

  /** 🔔️ Calls `listener` on every change; returns the unsubscribe function. */
  subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  /** ▶️ Starts delivering answers, loading the catalog and the learner, and following the browser's connectivity and
   * what the site's other tabs store. */
  start(): void {
    this.lifetime = new AbortController();
    this.teardown.push(
      this.outbox.subscribe(() => {
        this.overlayPending();
        this.notify();
      }),
    );
    this.teardown.push(this.store.watch((change) => this.remote(change)));
    this.teardown.push(
      this.proctor.subscribe(() => {
        if (this.proctor.reachability() === "reachable") this.outbox.wake();
        this.notify();
      }),
    );
    if (typeof window !== "undefined") {
      const online = (): void => this.connectivity(true);
      const offline = (): void => this.connectivity(false);
      window.addEventListener("online", online);
      window.addEventListener("offline", offline);
      this.teardown.push(
        () => window.removeEventListener("online", online),
        () => window.removeEventListener("offline", offline),
      );
      this.online = typeof navigator === "undefined" || navigator.onLine !== false;
    }
    this.outbox.start();
    void this.refreshCatalog();
    void this.refreshLearner();
    this.notify();
  }

  /** ⏹️ Stops every background activity; persisted slices stay for the next {@link start}. */
  stop(): void {
    this.lifetime.abort();
    this.outbox.stop();
    for (const release of this.teardown.splice(0)) release();
  }

  /** 🔌️ Asks the proctor again now instead of waiting out the current backoff: restarts the catalog and learner loads
   * and wakes the outbox. Does nothing while the session is stopped. */
  reconnect(): void {
    if (this.lifetime.signal.aborted) return;
    this.lifetime.abort();
    this.lifetime = new AbortController();
    this.outbox.wake();
    void this.refreshCatalog();
    void this.refreshLearner();
  }

  /** 👋️ The learner has read the introduction. */
  readIntroduction(): void {
    this.dispatch({ type: "introduction-read" });
  }

  /** 🪧️ Opens a screen and refreshes what it shows from the proctor; a cached run shows at once, also offline. */
  open(step: QuizStep): void {
    this.dispatch({ type: "step-opened", step });
    if (step.screen === "home") void this.refreshLearner();
    if (step.screen === "leaderboard") void this.refreshLeaderboard();
    if (step.screen === "run" || step.screen === "results") void this.loadRun(step.run);
  }

  /** 🙈️ Dismisses the current notice. */
  dismissNotice(): void {
    this.dispatch({ type: "notice-dismissed" });
  }

  /** 🔁️ Forgets the learner on this device; their progress stays with the proctor. */
  forgetLearner(): void {
    this.dispatch({ type: "learner-forgotten" });
  }

  /** 🙋️ Registers a new learner or recalls the one holding the handle. A replay whose events were already delivered
   * is asked once more under a fresh command id, which recalls whoever holds the handle by then. A recalled learner
   * shows the handle just entered until their learner view arrives with the handle as registered. */
  async identify(identity: Identity, signal: AbortSignal): Promise<SessionFailure | undefined> {
    const learner = newId();
    const once = (): Promise<CommandVerdict> => {
      const command: IdentifyLearnerCommand = { type: "identify-learner", id: newId(), learner, identity };
      return retryTransient(() => this.proctor.command(command, signal), this.timing, signal);
    };
    let verdict = await once();
    if (verdict.kind === "accepted" && !verdict.events.some(rosterEvent)) verdict = await once();
    if (verdict.kind !== "accepted") return failureOf(verdict);
    const event = verdict.events.find(rosterEvent);
    this.dispatch({ type: "learner-identified", learner: event?.learner ?? learner, identity: event?.type === "learner-registered" ? event.identity : identity });
    void this.refreshLearner();
    return undefined;
  }

  /** ▶️ Starts a run of `quiz` and opens it; an already open run of the quiz is resumed instead. */
  async startRun(quiz: Slug, signal: AbortSignal): Promise<SessionFailure | undefined> {
    const learner = this.state.learner?.id;
    if (learner === undefined) return { kind: "rejected", rejection: "unknown-learner" };
    const command: StartRunCommand = { type: "start-run", id: newId(), learner, run: newId(), quiz };
    const verdict = await retryTransient(() => this.proctor.command(command, signal), this.timing, signal);
    if (verdict.kind === "rejected" && verdict.rejection === "run-open") {
      const view = await this.projected((current) => this.proctor.learner(learner, current), signal);
      this.adoptLearnerView(view);
      const open = openRunOf(this.state, quiz);
      return open === undefined ? failureOf(verdict) : this.resumeRun(open, signal);
    }
    if (verdict.kind !== "accepted") return failureOf(verdict);
    for (const event of verdict.events) {
      if (event.type !== "run-voided") continue;
      this.outbox.discard(event.run);
      this.dispatch({ type: "run-voided", run: event.run });
    }
    return this.resumeRun(command.run, signal);
  }

  /** ⏯️ Opens a run, loading it first when this device has not seen it yet. */
  async resumeRun(run: Id, signal: AbortSignal): Promise<undefined> {
    if (this.state.runs[run] === undefined) await this.loadRun(run, signal);
    else void this.loadRun(run);
    this.dispatch({ type: "step-opened", step: { screen: "run", run } });
    void this.refreshLearner();
    return undefined;
  }

  /** ✍️ Records an answer: applied locally at once, delivered through the outbox. */
  answer(run: Id, task: Slug, answer: Answer): void {
    const learner = this.state.learner?.id;
    const view = this.state.runs[run];
    if (learner === undefined || view === undefined || view.status !== "open") return;
    this.dispatch({ type: "answer-given", run, task, answer });
    this.outbox.enqueue({ type: "record-answer", id: newId(), learner, run, task, answer });
  }

  /** 📨️ Submits a run once all its answers are delivered, reporting each phase; `signal` cancels between and during
   * phases. A cancellation can race the proctor committing the submission, so every cancelled submission is
   * reconciled: the run view is read again and, if the proctor did submit it, adopted with its result. */
  async submit(run: Id, signal: AbortSignal, onPhase: (phase: SubmissionPhase) => void): Promise<SessionFailure | undefined> {
    try {
      return await this.submitting(run, signal, onPhase);
    } catch (error) {
      if (signal.aborted) void this.loadRun(run);
      throw error;
    }
  }

  private async submitting(run: Id, signal: AbortSignal, onPhase: (phase: SubmissionPhase) => void): Promise<SessionFailure | undefined> {
    const learner = this.state.learner?.id;
    if (learner === undefined) return { kind: "rejected", rejection: "unknown-learner" };
    await this.outbox.settled(run, signal, (done, total) => onPhase({ phase: "saving", done, total }));
    if (this.state.runs[run]?.status === "voided") return { kind: "rejected", rejection: "quiz-revised" };
    onPhase({ phase: "submitting" });
    const command: SubmitRunCommand = { type: "submit-run", id: newId(), learner, run };
    const verdict = await retryTransient(() => this.proctor.command(command, signal), this.timing, signal);
    if (verdict.kind === "refused" || (verdict.kind === "rejected" && verdict.rejection !== "run-closed")) return failureOf(verdict);
    onPhase({ phase: "results" });
    const events = verdict.kind === "accepted" ? verdict.events : [];
    const submitted = events.find((event): event is RunSubmittedEvent => event.type === "run-submitted" && event.run === run);
    if (events.some((event) => event.type === "run-voided" && event.run === run)) {
      this.voided(run);
      return { kind: "rejected", rejection: "quiz-revised" };
    }
    if (submitted !== undefined) {
      const badges = events.filter((event): event is BadgeAwardedEvent => event.type === "badge-awarded" && event.run === run).map((event) => event.badge);
      this.dispatch({ type: "run-submitted", run, result: submitted.result, badges, at: submitted.at });
    } else {
      const view = await this.loadRun(run, signal);
      if (view?.status !== "submitted") return { kind: "rejected", rejection: view?.status === "voided" ? "quiz-revised" : "run-closed" };
      this.dispatch({ type: "step-opened", step: { screen: "results", run } });
    }
    void this.refreshLearner();
    return undefined;
  }

  /** 🏃️ Loads a run view, waiting out projection lag. The proctor's view wins — including answers another device gave
   * — except for answers still waiting in the outbox, which are newer than anything the proctor has. A run that closed
   * meanwhile is followed: to its results when submitted, home with a notice when voided. `signal` makes it
   * interactive, else it runs in the background. */
  async loadRun(run: Id, signal?: AbortSignal): Promise<RunView | undefined> {
    const learner = this.state.learner?.id;
    if (learner === undefined) return undefined;
    const read = (current: AbortSignal): Promise<RunView> => this.proctor.run(run, learner, current);
    const loaded = signal === undefined ? await this.background(read) : await this.projected(read, signal);
    if (loaded === undefined) return undefined;
    const cached = this.state.runs[run];
    const view = this.withPending(loaded);
    this.dispatch({ type: "run-loaded", view });
    this.followClosure(cached, view);
    return view;
  }

  private withPending(view: RunView): RunView {
    return view.status === "open" ? { ...view, answers: { ...view.answers, ...this.outbox.pendingAnswers(view.run) } } : view;
  }

  private overlayPending(): void {
    for (const view of Object.values(this.state.runs)) {
      if (view.status !== "open") continue;
      const pending = Object.entries(this.outbox.pendingAnswers(view.run));
      if (pending.some(([task, answer]) => JSON.stringify(view.answers[task]) !== JSON.stringify(answer))) this.dispatch({ type: "run-loaded", view: this.withPending(view) }, false);
    }
  }

  private followClosure(before: RunView | undefined, after: RunView | undefined, refresh = true): void {
    if (before?.status !== "open" || after === undefined || after.status === "open") return;
    const onRun = this.state.step.screen === "run" && this.state.step.run === after.run;
    this.outbox.discard(after.run);
    if (after.status === "voided") {
      if (onRun) this.dispatch({ type: "step-opened", step: { screen: "home" } });
      this.dispatch({ type: "notice-raised", notice: { kind: "voided" } });
    } else if (onRun) this.dispatch({ type: "step-opened", step: { screen: "results", run: after.run } });
    if (refresh) void this.refreshLearner();
  }

  private adoptLearnerView(view: LearnerView, persist = true): void {
    const before = this.state.runs;
    this.dispatch({ type: "learner-loaded", view }, persist);
    for (const summary of view.runs) this.followClosure(before[summary.run], this.state.runs[summary.run], false);
  }

  private remote(change: LocalChange): void {
    if (change.kind === "cleared") return this.adoptStore();
    if (change.kind === "record") {
      if (change.collection === "runs") this.adoptRun(change.id);
      return;
    }
    switch (change.slice) {
      case "introduced":
        if (this.store.read("introduced") === true && !this.state.introduced) this.dispatch({ type: "introduction-read" }, false);
        return;
      case "learner":
        return this.adoptLearner();
      case "catalog": {
        const catalog = restoredCatalog(this.store.read("catalog"));
        if (catalog !== undefined) this.dispatch({ type: "catalog-loaded", catalog }, false);
        return;
      }
      case "learner-view": {
        const view = restoredLearnerView(this.store.read("learner-view"));
        if (view !== undefined) this.adoptLearnerView(view, false);
        return;
      }
      case "preferences":
        return;
    }
  }

  private adoptLearner(): void {
    const stored = restoredLearner(this.store.read("learner"));
    if (stored?.id === this.state.learner?.id) return;
    if (stored === undefined) return this.dispatch({ type: "learner-forgotten" }, false);
    this.dispatch({ type: "learner-identified", learner: stored.id, identity: stored.identity }, false);
    for (const view of Object.values(restoredRuns(this.store, stored.id))) this.dispatch({ type: "run-loaded", view: this.withPending(view) }, false);
    void this.refreshLearner();
  }

  private adoptRun(run: Id): void {
    const stored = restoredRun(this.store.record("runs", run), run);
    if (stored === undefined || stored.learner !== this.state.learner?.id) return;
    const cached = this.state.runs[run];
    const view = this.withPending(mergeRunViews(cached, stored));
    this.dispatch({ type: "run-loaded", view }, false);
    this.followClosure(cached, view);
  }

  private adoptStore(): void {
    this.state = this.restored();
    this.notify();
  }

  private restored(): QuizState {
    const stored = restoreQuizState(this.store);
    const state = stored.learnerView === undefined ? stored : evolveQuizState(stored, { type: "learner-loaded", view: stored.learnerView });
    for (const summary of state.learnerView?.runs ?? []) if (summary.status !== "open") this.outbox.discard(summary.run);
    for (const [run, view] of Object.entries(state.runs)) if (stored.runs[run] !== view) this.store.put("runs", run, view);
    return { ...state, runs: Object.fromEntries(Object.entries(state.runs).map(([run, view]) => [run, this.withPending(view)])) };
  }

  private answerSettled(run: Id, verdict: CommandVerdict): void {
    if (verdict.kind === "accepted") return;
    if (verdict.kind === "refused") return this.dispatch({ type: "notice-raised", notice: { kind: "refused", detail: verdict.detail } });
    if (verdict.rejection === "quiz-revised") return this.voided(run);
    if (verdict.rejection === "run-closed" || verdict.rejection === "unknown-run") return void this.loadRun(run);
    this.dispatch({ type: "notice-raised", notice: { kind: "rejection", rejection: verdict.rejection } });
  }

  private voided(run: Id): void {
    this.outbox.discard(run);
    this.dispatch({ type: "run-voided", run });
    this.dispatch({ type: "notice-raised", notice: { kind: "voided" } });
    void this.refreshLearner();
  }

  private projected<T>(call: (signal: AbortSignal) => Promise<T>, signal: AbortSignal): Promise<T> {
    const until = this.now() + PROJECTION_GRACE_MS;
    return retryTransient(
      async () => {
        try {
          return await call(signal);
        } catch (error) {
          if (isNotFound(error) && this.now() < until) throw new ProctorUnavailable("not projected yet");
          throw error;
        }
      },
      this.timing,
      signal,
    );
  }

  private async background<T>(call: (signal: AbortSignal) => Promise<T>): Promise<T | undefined> {
    const signal = this.lifetime.signal;
    try {
      return await this.projected(call, signal);
    } catch (error) {
      if (!signal.aborted) this.dispatch({ type: "notice-raised", notice: { kind: "refused", detail: describe(error) } });
      return undefined;
    }
  }

  private connectivity(online: boolean): void {
    this.online = online;
    if (online) this.outbox.wake();
    this.notify();
  }

  private dispatch(event: QuizClientEvent, persist = true): void {
    const before = this.state;
    this.state = evolveQuizState(before, event);
    if (this.state === before) return;
    if (persist) persistQuizState(this.store, before, this.state);
    this.notify();
  }

  private notify(): void {
    this.snapshot = this.freeze();
    for (const listener of [...this.listeners]) listener();
  }

  private freeze(): QuizSnapshot {
    const status = this.outbox.status();
    return { state: this.state, connection: { reachability: this.proctor.reachability(), online: this.online, pending: status.pending, activity: status.activity } };
  }
}
//#endregion 🎮️Session
