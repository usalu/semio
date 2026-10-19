/** 🧺️ The run views the device holds against what still waits in the outbox: answers waiting lie on top of any view, a closed run sheds its queue, and a state restored from the store starts that way. */

import { type Id, type RunView } from "@semio-tech/course";
import type { Deputy } from "../../🫡️deputy/🟦️.ts";
import { type LocalStore } from "../../💾️persistence/🟦️.ts";
import { commandRun, type Outbox } from "../../📮️outbox/🟦️.ts";
import { restoreCourseState } from "../💾️restore/🟦️.ts";
import { evolveCourseState, unhinted, type CourseState } from "../🧮️state/🟦️.ts";
import { decision } from "../📋️session-model/🟦️.ts";

//#region 🧺️OutboxOverlay
/** 🪡️ The outbox of one session seen from the run views: `deputy` is the one whose course stands in for a course never loaded. */
export class OutboxOverlay {
  constructor(
    private readonly outbox: Outbox,
    private readonly store: LocalStore,
    private readonly deputy: Deputy | undefined,
  ) {}

  /** 📝️ `view` with the answers still waiting in the outbox on top — they are newer than anything the proctor has — and
   * without the hints of the tasks whose answer that changes. */
  withPending(view: RunView): RunView {
    if (view.status !== "open") return view;
    const pending = this.outbox.pendingAnswers(view.run);
    const changed = Object.keys(pending).filter((task) => JSON.stringify(view.answers[task]) !== JSON.stringify(pending[task]));
    return changed.length === 0 ? view : unhinted({ ...view, answers: { ...view.answers, ...pending } }, (task) => changed.includes(task));
  }

  /** 🗑️ Drops what is queued of a run that closed — unless a decision about that run still waits: then the proctor has
   * not heard of its closing yet and needs every command of it in order. */
  retire(run: Id): void {
    if (!this.outbox.waiting((command) => decision(command) && commandRun(command) === run)) this.outbox.discard(run);
  }

  /** 🌱️ The state the store holds for its learner, with the deputy's course where none was loaded, the queues of closed runs dropped, the run records the fold changed written back and the waiting answers on top. */
  restore(): CourseState {
    const held = restoreCourseState(this.store);
    const stored = held.course === undefined && this.deputy !== undefined ? { ...held, course: this.deputy.course } : held;
    const state = stored.learnerView === undefined ? stored : evolveCourseState(stored, { type: "learner-loaded", view: stored.learnerView });
    for (const summary of state.learnerView?.runs ?? []) if (summary.status !== "open") this.retire(summary.run);
    for (const [run, view] of Object.entries(state.runs)) if (stored.runs[run] !== view) this.store.put("runs", run, view);
    return { ...state, runs: Object.fromEntries(Object.entries(state.runs).map(([run, view]) => [run, this.withPending(view)])) };
  }
}
//#endregion 🧺️OutboxOverlay
