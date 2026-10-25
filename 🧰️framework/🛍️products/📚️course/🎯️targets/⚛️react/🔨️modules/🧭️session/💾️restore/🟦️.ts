/** 💾️ The persisted local-only slices and run records of a store: restoring a state from them and writing a change back. */

import { type CourseView, type Id, type Identity, type LearnerView, type RunView } from "@semio-tech/course";
import { isCourseView, isLearnerView, isRecord, isRunView, type LocalStore } from "../../💾️persistence/🟦️.ts";
import { initialCourseState, type CourseLearner, type CourseState } from "../🧮️state/🟦️.ts";

//#region 💾️Persistence
export function restoredLearner(value: unknown): CourseLearner | undefined {
  if (!isRecord(value) || typeof value.id !== "string") return undefined;
  return isRecord(value.identity) && typeof value.identity.kind === "string" ? { id: value.id, identity: value.identity as unknown as Identity } : { id: value.id };
}

export function restoredCourse(value: unknown): CourseView | undefined {
  return isCourseView(value) ? value : undefined;
}

export function restoredLearnerView(value: unknown): LearnerView | undefined {
  return isLearnerView(value) ? value : undefined;
}

export function restoredRun(value: unknown, run: Id): RunView | undefined {
  return isRunView(value, run) ? value : undefined;
}

export function restoredRuns(store: LocalStore, learner: Id | undefined): Readonly<Record<Id, RunView>> {
  const runs: Record<Id, RunView> = {};
  if (learner === undefined) return runs;
  for (const [run, value] of store.records("runs")) {
    const view = restoredRun(value, run);
    if (view?.learner === learner) runs[run] = view;
  }
  return runs;
}

/** 🗄️ The persisted local-only slices and run records of `store` for its learner, dropping anything unreadable. */
export function restoreCourseState(store: LocalStore): CourseState {
  const learner = restoredLearner(store.read("learner"));
  return initialCourseState({
    introduced: store.read("introduced") === true,
    learner,
    course: restoredCourse(store.read("course")),
    learnerView: restoredLearnerView(store.read("learner-view")),
    runs: restoredRuns(store, learner?.id),
  });
}

export function persistCourseState(store: LocalStore, before: CourseState, after: CourseState): void {
  if (before.introduced !== after.introduced) store.write("introduced", after.introduced);
  if (before.learner !== after.learner) after.learner === undefined ? store.remove("learner") : store.write("learner", after.learner);
  if (before.course !== after.course) store.write("course", after.course);
  if (before.learnerView !== after.learnerView) after.learnerView === undefined ? store.remove("learner-view") : store.write("learner-view", after.learnerView);
  if (before.runs === after.runs) return;
  for (const [run, view] of Object.entries(after.runs)) if (before.runs[run] !== view) store.put("runs", run, view);
  for (const run of Object.keys(before.runs)) if (!Object.hasOwn(after.runs, run)) store.drop("runs", run);
}
//#endregion 💾️Persistence
