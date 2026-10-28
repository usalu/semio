/** 🧭️ The course client session: local-first, event-driven state of one learner on one device, and the controller that
 * turns learner intent into proctor commands and folds what comes back into that state.
 *
 * State only changes through {@link CourseClientEvent}s folded by the pure {@link evolveCourseState}. Slices are kept apart
 * by lifetime: persisted local-only (introduction seen, learner id, cached course, learner view, one record per run
 * view, the answer outbox), persisted shared (the proctor's events, reached only through commands), ephemeral shared
 * (the leaderboard, polled) and ephemeral local-only (the current step and the trail of steps around it, notices, newly
 * earned badges, the quizzes whose crowd the learner asked to see before submitting). The site's tabs share the persisted local-only slices: what another tab stores is adopted without
 * being written back — a run record by {@link mergeRunViews} — so tabs never undo each other. Answers apply locally at
 * once and travel through the {@link Outbox}; interactive commands retry transient failures with jittered backoff under
 * one command id and are cancellable through their `AbortSignal`. A learner is registered by a command and recalled by
 * a read: asking who holds a pseudonym or name writes nothing, and whoever asks for a claimed one continues as its
 * holder. The learner finds the way with the trail: back and forward along the steps that still stand, up to the place
 * above, and a new trail with every new learner.
 *
 * A run starts at a challenge; starting a quiz at another challenge than its open run's voids that run. The device
 * keeps the time: an opening of a task and an answer carry the instant the learner acted by the session clock
 * ({@link CourseSession.now}), so a connection shortage never makes a timely answer late. An easy run's hints follow
 * every answer — at once from the deputy, else with the proctor's view of the run once its answers are delivered.
 *
 * With a {@link Deputy} — handed in with the options, or seated later by {@link CourseSession.attachDeputy} once a lazily fetched
 * material has arrived — the device also decides by itself. A command goes straight to the proctor unless the proctor
 * is known not to answer or something the deputy decided still waits in the outbox; then the deputy decides it, its events
 * fold into the held views at once and the command waits in the outbox for the proctor. What the deputy decided is
 * provisional: while such a command waits, the proctor's views are behind and are not adopted; once none waits they
 * are read again and replace what the device made up. A learner the deputy registered under a pseudonym or name the
 * proctor already knows continues as its holder, like anyone who enters a claimed handle.
 *
 * The session is a thin composer. Each collaborator owns its fields and an explicit narrow API, and only calls the ones
 * below it: {@link HeldState} (state, snapshot, listeners), {@link Lifecycle} (lifetime, releases, online),
 * {@link ProctorLink} (clock, retries, patience), {@link OutboxOverlay} (waiting answers over the run views),
 * {@link Deputising} (the deputy's views and decisions), {@link DecisionRouting} (who decides), {@link ProctorReads}
 * (reads and their order), {@link Registration}, {@link RunPlay}, {@link Settlement} (delivery and verdicts),
 * {@link Adoption} (other tabs and the outbox) and {@link Attention} (steps, crowd, board).
 *
 * @see ../../../../🧬️schema/🔣️.json — commands, events, views
 * @see ../🫡️deputy/🟦️.ts — the deciders on the device
 */

import { type Answer, type Challenge, type Id, type IdentityClaim, type RunView, type Slug } from "@semio-tech/course";
import type { Deputy } from "../🫡️deputy/🟦️.ts";
import { type LocalStore } from "../💾️persistence/🟦️.ts";
import { Outbox } from "../📮️outbox/🟦️.ts";
import { RETRY_TIMING, type ProctorClient } from "../🛂️proctor/🟦️.ts";
import { HeldState } from "./🗃️held-state/🟦️.ts";
import { DecisionRouting } from "./🏛️decision-routing/🟦️.ts";
import { Lifecycle } from "./🔌️lifecycle/🟦️.ts";
import { ProctorLink } from "./📡️proctor-link/🟦️.ts";
import { ProctorReads } from "./📖️proctor-reads/🟦️.ts";
import { Registration } from "./🙋️registration/🟦️.ts";
import { Attention } from "./🔭️attention/🟦️.ts";
import { RunPlay } from "./🎯️run-play/🟦️.ts";
import { Settlement } from "./📬️settlement/🟦️.ts";
import { Adoption } from "./🪟️adoption/🟦️.ts";
import { OutboxOverlay } from "./🧺️outbox-overlay/🟦️.ts";
import { Deputising } from "./🫡️deputising/🟦️.ts";
import { type BoardChoice, type CourseStep } from "./🧮️state/🟦️.ts";
import { DEPUTY_PATIENCE_MS, type CourseConnection, type DeputySeat, type CourseSessionOptions, type CourseSnapshot, type SessionFailure, type SubmissionPhase } from "./📋️session-model/🟦️.ts";

//#region 🎮️Session
/** 🎮️ One learner session against one proctor tenant — an external store for the UI and the controller of its intents. */
export class CourseSession {
  readonly proctor: ProctorClient;
  readonly outbox: Outbox;
  readonly refreshCourse: ProctorReads["refreshCourse"];
  readonly refreshLearner: ProctorReads["refreshLearner"];
  readonly refreshLeaderboard: ProctorReads["refreshLeaderboard"];
  readonly getSnapshot: () => CourseSnapshot;
  readonly subscribe: (listener: () => void) => () => void;
  private readonly store: LocalStore;
  private readonly link: ProctorLink;
  private readonly lifecycle = new Lifecycle();
  private readonly state: HeldState;
  private readonly deputising: Deputising;
  private readonly reads: ProctorReads;
  private readonly registration: Registration;
  private readonly play: RunPlay;
  private readonly settlement: Settlement;
  private readonly adoption: Adoption;
  private readonly attention: Attention;

  constructor(options: CourseSessionOptions) {
    const clock = options.now ?? Date.now;
    const timing = options.timing ?? RETRY_TIMING;
    const seat: DeputySeat = { current: options.deputy };
    this.proctor = options.proctor;
    this.store = options.store;
    this.link = new ProctorLink(options.proctor, timing, options.patienceMs ?? DEPUTY_PATIENCE_MS, clock);
    this.outbox = new Outbox({ send: (command, signal) => this.settlement.deliver(command, signal), store: options.store, timing, now: clock, onSettled: (command, verdict) => this.settlement.settled(command, verdict) });
    const overlay = new OutboxOverlay(this.outbox, options.store, seat);
    const connection = (): CourseConnection => {
      const status = this.outbox.status();
      const reachability = options.proctor.reachability();
      const contract = reachability === "incompatible" ? options.proctor.contract() : undefined;
      return { reachability, online: this.lifecycle.isOnline(), pending: status.pending, activity: status.activity, deputy: seat.current !== undefined, ...(contract === undefined ? {} : { contract }) };
    };
    this.state = new HeldState(options.store, connection, overlay.restore());
    this.deputising = new Deputising(seat, this.state, this.outbox, this.link, this.lifecycle);
    this.deputising.rehintAll();
    const routing = new DecisionRouting(this.state, this.link, this.lifecycle, this.outbox, this.deputising);
    this.reads = new ProctorReads(this.state, this.link, this.lifecycle, this.outbox, overlay, this.deputising, routing);
    this.registration = new Registration(this.state, this.link, routing, this.deputising, this.reads);
    this.play = new RunPlay(this.state, this.link, this.outbox, routing, this.deputising, this.reads);
    this.settlement = new Settlement(this.state, this.link, this.outbox, routing, this.reads, this.deputising);
    this.adoption = new Adoption(this.state, options.store, overlay, this.deputising, this.reads);
    this.attention = new Attention(this.state, this.reads, clock);
    this.refreshCourse = this.reads.refreshCourse;
    this.refreshLearner = this.reads.refreshLearner;
    this.refreshLeaderboard = this.reads.refreshLeaderboard;
    this.getSnapshot = this.state.snapshot;
    this.subscribe = this.state.subscribe;
  }

  /** 🕰️ The session's clock in milliseconds since the Unix epoch: the instant an answer or an opening is stamped with
   * and the one every countdown is measured against. */
  now(): number {
    return this.link.now();
  }

  /** ▶️ Starts delivering answers, loading the course and the learner, and following the browser's connectivity and
   * what the site's other tabs store. */
  start(): void {
    this.lifecycle.begin();
    this.lifecycle.own(
      this.outbox.subscribe(() => {
        this.adoption.overlayPending();
        this.state.notify();
      }),
    );
    this.lifecycle.own(this.store.watch((change) => this.adoption.remote(change)));
    let reached = this.proctor.reachability();
    this.lifecycle.own(
      this.proctor.subscribe(() => {
        const before = reached;
        reached = this.proctor.reachability();
        if (reached === "reachable" && before !== "throttled") this.outbox.wake();
        this.state.notify();
      }),
    );
    this.lifecycle.watchBrowser((online) => {
      if (online) this.outbox.wake();
      this.state.notify();
    });
    this.outbox.start();
    void this.refreshCourse();
    void this.refreshLearner();
    this.state.notify();
  }

  /** ⏹️ Stops every background activity; persisted slices stay for the next {@link start}. */
  stop(): void {
    this.lifecycle.abort();
    this.outbox.stop();
    this.lifecycle.release();
  }

  /** 🔌️ Asks the proctor again now instead of waiting out the current backoff: restarts the course and learner loads
   * and wakes the outbox. Does nothing while the session is stopped. */
  reconnect(): void {
    if (!this.lifecycle.restart()) return;
    this.outbox.wake();
    void this.refreshCourse();
    void this.refreshLearner();
  }

  /** 🫡️ Seats the deputy of material that arrived after the session began ({@link Deputising.seatDeputy}); from then on
   * the device decides while the proctor is away. Does nothing when a deputy sits already. */
  attachDeputy(deputy: Deputy): void {
    this.deputising.seatDeputy(deputy);
  }

  /** 👋️ The learner has read the introduction. */
  readIntroduction(): void {
    this.state.dispatch({ type: "introduction-read" });
  }

  /** 🙈️ Dismisses the current notice. */
  dismissNotice(): void {
    this.state.dispatch({ type: "notice-dismissed" });
  }

  /** 🧽️ Forgets the learner on this device; their progress stays with the proctor. */
  forgetLearner(): void {
    this.state.dispatch({ type: "learner-forgotten" });
  }

  /** 🚪️ Opens a screen and refreshes what it shows ({@link Attention.open}). */
  open(step: CourseStep, instead = false): void {
    this.attention.open(step, instead);
  }

  /** ◀️ Goes back along the trail to the nearest step that still stands. */
  back(): void {
    this.attention.back();
  }

  /** ⏭️ Goes forward again along the trail to the nearest step that still stands. */
  forward(): void {
    this.attention.forward();
  }

  /** 🔼️ Opens the place above the step in front, if there is one. */
  up(): void {
    this.attention.up();
  }

  /** 👆️ The learner chooses the leaderboard to look at ({@link Attention.chooseBoard}). */
  chooseBoard(board: BoardChoice): void {
    this.attention.chooseBoard(board);
  }

  /** 👀️ The learner asks to see what the others answered in `quiz` before submitting ({@link Attention.askCrowd}). */
  askCrowd(quiz: Slug): void {
    this.attention.askCrowd(quiz);
  }

  /** 🫣️ The learner hides the others of `quiz` again until submitting. */
  unaskCrowd(quiz: Slug): void {
    this.attention.unaskCrowd(quiz);
  }

  /** 👥️ Asks for the submitted crowd of `quiz` once ({@link ProctorReads.refreshCrowd}). */
  refreshCrowd(quiz: Slug): Promise<void> {
    return this.reads.refreshCrowd(quiz);
  }

  /** 🙋️ Makes this device a learner ({@link Registration.identify}). */
  identify(identity: IdentityClaim, signal: AbortSignal): Promise<SessionFailure | undefined> {
    return this.registration.identify(identity, signal);
  }

  /** 🚀️ Starts a run of `quiz` at `challenge` and opens it ({@link RunPlay.startRun}). */
  startRun(quiz: Slug, challenge: Challenge, signal: AbortSignal): Promise<SessionFailure | undefined> {
    return this.play.startRun(quiz, challenge, signal);
  }

  /** ⏯️ Opens a run, loading it first when this device has not seen it yet. */
  resumeRun(run: Id, signal: AbortSignal): Promise<undefined> {
    return this.play.resumeRun(run, signal);
  }

  /** ⏱️ Opens a task of a timed run ({@link RunPlay.openTask}). */
  openTask(run: Id, task: Slug, signal: AbortSignal): Promise<SessionFailure | undefined> {
    return this.play.openTask(run, task, signal);
  }

  /** ✍️ Records an answer given now by the session clock ({@link RunPlay.answer}). */
  answer(run: Id, task: Slug, answer: Answer): void {
    this.play.answer(run, task, answer);
  }

  /** 📨️ Submits a run once all its answers are delivered, reporting each phase ({@link RunPlay.submit}). */
  submit(run: Id, signal: AbortSignal, onPhase: (phase: SubmissionPhase) => void): Promise<SessionFailure | undefined> {
    return this.play.submit(run, signal, onPhase);
  }

  /** 📖️ Loads a run view, waiting out projection lag ({@link ProctorReads.loadRun}). */
  loadRun(run: Id, signal?: AbortSignal): Promise<RunView | undefined> {
    return this.reads.loadRun(run, signal);
  }
}
//#endregion 🎮️Session
