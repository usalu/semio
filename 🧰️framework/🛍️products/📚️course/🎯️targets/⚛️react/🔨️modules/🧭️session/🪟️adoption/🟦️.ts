/** 🪟️ What the session adopts without being asked: what the site's other tabs store (taken in and never written back, so tabs never undo each other) and what still waits in the outbox, laid over the run views held.
 *
 * @see ../../💾️persistence/🟦️.ts — the store the tabs share
 */

import { type Id } from "@semio-tech/course";
import { type LocalChange, type LocalStore } from "../../💾️persistence/🟦️.ts";
import { type HeldState } from "../🗃️held-state/🟦️.ts";
import { restoredCourse, restoredLearner, restoredLearnerView, restoredRun, restoredRuns } from "../💾️restore/🟦️.ts";
import { type ProctorReads } from "../📖️proctor-reads/🟦️.ts";
import { type OutboxOverlay } from "../🧺️outbox-overlay/🟦️.ts";
import { type Deputising } from "../🫡️deputising/🟦️.ts";
import { mergeRunViews } from "../🧮️state/🟦️.ts";

//#region 🪟️Adoption
/** 🤲️ The adoption of one session. */
export class Adoption {
  constructor(
    private readonly state: HeldState,
    private readonly store: LocalStore,
    private readonly overlay: OutboxOverlay,
    private readonly deputising: Deputising,
    private readonly reads: ProctorReads,
  ) {}

  /** 🧲️ Takes in a change another tab made to the store: a cleared store restores everything, a run record is merged into the held run, any other slice replaces its own part of the state. */
  remote(change: LocalChange): void {
    if (change.kind === "cleared") return this.adoptStore();
    if (change.kind === "record") {
      if (change.collection === "runs") this.adoptRun(change.id);
      return;
    }
    switch (change.slice) {
      case "introduced":
        if (this.store.read("introduced") === true && !this.state.read().introduced) this.state.dispatch({ type: "introduction-read" }, false);
        return;
      case "learner":
        return this.adoptLearner();
      case "course": {
        const course = restoredCourse(this.store.read("course"));
        if (course !== undefined) this.state.dispatch({ type: "course-loaded", course }, false);
        return;
      }
      case "learner-view": {
        const view = restoredLearnerView(this.store.read("learner-view"));
        if (view !== undefined) this.reads.adoptLearnerView(view, false);
        return;
      }
      case "preferences":
        return;
    }
  }

  /** 🧅️ Lays the answers still waiting in the outbox over every open run held, and has the deputy tell their hints anew. */
  overlayPending(): void {
    for (const view of Object.values(this.state.read().runs)) {
      const overlaid = this.overlay.withPending(view);
      if (overlaid === view) continue;
      this.state.dispatch({ type: "run-loaded", view: overlaid }, false);
      this.deputising.rehint(view.run, false);
    }
  }

  private adoptLearner(): void {
    const stored = restoredLearner(this.store.read("learner"));
    if (stored?.id === this.state.learnerId()) return;
    if (stored === undefined) return this.state.dispatch({ type: "learner-forgotten" }, false);
    this.state.dispatch({ type: "learner-identified", learner: stored.id, identity: stored.identity }, false);
    for (const view of Object.values(restoredRuns(this.store, stored.id))) {
      this.state.dispatch({ type: "run-loaded", view: this.overlay.withPending(view) }, false);
      this.deputising.rehint(view.run, false);
    }
    void this.reads.refreshLearner();
  }

  private adoptRun(run: Id): void {
    const stored = restoredRun(this.store.record("runs", run), run);
    if (stored === undefined || stored.learner !== this.state.learnerId()) return;
    const cached = this.state.read().runs[run];
    const view = this.overlay.withPending(mergeRunViews(cached, stored));
    this.state.dispatch({ type: "run-loaded", view }, false);
    this.deputising.rehint(run, false);
    this.reads.followClosure(cached, view);
  }

  private adoptStore(): void {
    this.state.reset(this.overlay.restore());
    this.deputising.rehintAll();
    this.state.notify();
  }
}
//#endregion 🪟️Adoption
