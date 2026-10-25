/** 🫡️ What the deputy does on the device: the learner as the held views describe it, the decisions it takes and folds into those views, the hints it tells and the device's own standing on the leaderboard. Without a deputy none of it happens.
 *
 * @see ../../🫡️deputy/🟦️.ts — the deciders on the device
 */

import { emptyLearnerState, evolveLearner, challengeRules, type Command, type Event, type HandleView, type Id, type LearnerState } from "@semio-tech/course";
import type { Deputy } from "../../🫡️deputy/🟦️.ts";
import { type Outbox } from "../../📮️outbox/🟦️.ts";
import { type DeputySeat } from "../📋️session-model/🟦️.ts";
import { type CommandVerdict } from "../../🛂️proctor/🟦️.ts";
import { type HeldState } from "../🗃️held-state/🟦️.ts";
import { type Lifecycle } from "../🔌️lifecycle/🟦️.ts";
import { type ProctorLink } from "../📡️proctor-link/🟦️.ts";

//#region 🫡️Deputising
/** 🪖️ The deputy of one session, if it has one yet, working on the held state and the outbox. */
export class Deputising {
  constructor(
    private readonly seat: DeputySeat,
    private readonly state: HeldState,
    private readonly outbox: Outbox,
    private readonly link: ProctorLink,
    private readonly lifecycle: Lifecycle,
  ) {}

  private get deputy(): Deputy | undefined {
    return this.seat.current;
  }

  /** 🙋️ Whether the session has a deputy. */
  get present(): boolean {
    return this.deputy !== undefined;
  }

  /** 🪑️ Seats the deputy of material that arrived after the session began: it tells the hints of the runs held and, where
   * no course was loaded or cached, its own stands in until the proctor's arrives. Does nothing when one sits already. */
  seatDeputy(deputy: Deputy): void {
    if (this.seat.current !== undefined) return;
    this.seat.current = deputy;
    if (this.state.read().course === undefined) this.state.dispatch({ type: "course-loaded", course: deputy.course }, false);
    this.rehintAll();
    this.state.notify();
  }

  /** 🔦️ Who holds `handle` as the deputy knows it: nobody. */
  handle(handle: string): HandleView {
    return this.deputy!.handle(handle);
  }

  /** 🗃️ The learner `learner` as the held views describe it; a learner this device does not act as has nothing held. */
  held(learner: Id): LearnerState {
    const { learner: acting, learnerView, runs } = this.state.read();
    return acting?.id === learner ? this.deputy!.state({ learner, identity: acting.identity, view: learnerView, runs }) : emptyLearnerState(learner);
  }

  /** 📥️ Makes `after` — the learner once `events` happened — what the device holds: the view of every run the events
   * touch, with the sheet the learner already sees, and the learner view. */
  fold(after: LearnerState, events: readonly Event[]): void {
    const deputy = this.deputy!;
    for (const run of new Set(events.flatMap((event) => (event.type === "learner-registered" ? [] : [event.run])))) {
      const view = deputy.runView(after, run);
      if (view !== undefined) this.state.dispatch({ type: "run-loaded", view: { ...view, sheet: this.state.read().runs[run]?.sheet ?? view.sheet } });
    }
    const view = deputy.learnerView(after);
    if (view !== undefined) this.state.dispatch({ type: "learner-loaded", view });
  }

  /** 🏷️ The deputy's decision of `command`: rejected like the proctor would, or accepted — then the command waits in
   * the outbox and its events are folded into the held views (a registration is folded by whoever adopts the learner). */
  deputise(command: Command): CommandVerdict {
    const before = command.type === "identify-learner" ? emptyLearnerState(command.learner) : this.held(command.learner);
    const taken = this.deputy!.decide(before, command, this.link.now());
    if ("rejection" in taken) return { kind: "rejected", rejection: taken.rejection };
    this.outbox.enqueue(command);
    if (command.type !== "identify-learner") this.fold(taken.events.reduce(evolveLearner, before), taken.events);
    return { kind: "accepted", events: taken.events };
  }

  /** 💡️ Has the deputy tell the hints of `run` anew when it is an open run that hints and they differ from those held —
   * whenever the answers held may have changed. Without a deputy, or for a run whose held sheet the material does not
   * deal, the hints held stay as they are: whatever changed an answer already dropped the hints of its former one. */
  rehint(run: Id, persist = true): void {
    const learner = this.state.learnerId();
    const view = this.state.read().runs[run];
    if (this.deputy === undefined || learner === undefined || view?.status !== "open" || !challengeRules(view.sheet.challenge).hints) return;
    const hints = this.deputy.hints(this.held(learner), run);
    if (hints !== undefined && JSON.stringify(hints) !== JSON.stringify(view.hints ?? {})) this.state.dispatch({ type: "run-hinted", run, hints }, persist);
  }

  /** 💭️ Has the deputy tell the hints of every run held anew. */
  rehintAll(): void {
    for (const run of Object.keys(this.state.read().runs)) this.rehint(run);
  }

  /** 🥈️ Shows the device's own standing on the leaderboard the learner looks at; a standing the proctor sent stays. */
  ownStanding(): void {
    const learner = this.state.learnerId();
    if (this.deputy === undefined || learner === undefined || this.lifecycle.signal.aborted) return;
    this.state.dispatch({ type: "leaderboard-loaded", leaderboard: this.deputy.leaderboard(this.held(learner), this.state.read().board, this.link.now()), at: this.link.now(), local: true });
  }
}
//#endregion 🫡️Deputising
