/** 🟢️ The shared declarations of the 🐾️ pet companions suite tests. */
import type { CourseView, RunResult, RunView, Sheet, SheetClassificationTask, SheetMatchingTask, SheetSortingTask } from "@semio-tech/course";
import { initialCourseState, type CourseSession, type CourseState, type CourseStep } from "@semio-tech/course-react";
import sheets from "../../../🧫️fixtures/🃏️sheet-assembly/🔣️.json";
import { COURSE, text } from "../../🪛️pet-companions-kit/🟦️.tsx";

export const SHEET = sheets.sheets.find((vector) => vector.id === "icons-1")!.sheet as unknown as Sheet;

export const OPEN_RUN: RunView = { run: "run-1", learner: "learner-1", quiz: SHEET.quiz, status: "open", sheet: SHEET, answers: {}, startedAt: 1_000_000 };

export const HOME_COURSE: CourseView = {
  ...COURSE,
  quizzes: [
    { ...COURSE.quizzes[0]!, tasks: [{ id: "u-values", kind: "classification", title: text("U-values", "U-Werte") }, { id: "heating-load-and-demand", kind: "sorting", title: text("Heating load", "Heizlast") }] },
    { ...COURSE.quizzes[1]!, tasks: [{ id: "air-change-rates", kind: "matching", title: text("Air changes", "Luftwechsel") }] },
  ],
};

/** 🧾️ The run of `SHEET` once submitted, with a result for each of its three tasks; the true order of its sorting is
 * the reverse of the sheet's. */
export function submittedRun(): RunView {
  const [sources, appliances, masses] = SHEET.tasks as readonly [SheetClassificationTask, SheetMatchingTask, SheetSortingTask];
  const category = sources.categories[0]!.id;
  const result: RunResult = {
    quiz: SHEET.quiz,
    challenge: "medium",
    score: 0.5,
    points: 100,
    tasks: [
      { kind: "classification", task: sources.id, score: 1, items: sources.items.map((item) => ({ item: item.id, assigned: category, correct: category, credit: 1 })) },
      { kind: "matching", task: appliances.id, score: 0, dimensions: appliances.dimensions.map((dimension) => ({ dimension: dimension.id, score: 0, items: appliances.items.map((item) => ({ item: item.id, correct: 1 })) })) },
      { kind: "sorting", task: masses.id, score: 0, items: masses.items.map((item, index) => ({ item: item.id, value: 10 ** index, position: index, rank: masses.items.length - 1 - index })) },
    ],
  };
  return { ...OPEN_RUN, status: "submitted", result, submittedAt: 1_100_000 };
}

/** 🏠️ The overview of a learner who knows the course, on `step`. */
export function homeAt(step: CourseStep): CourseState {
  return { ...initialCourseState({ introduced: true, learner: { id: "a".repeat(32), identity: { kind: "pseudonym", handle: "Ada" } }, course: HOME_COURSE, learnerView: undefined, runs: {} }), step };
}

/** 🛋️ A session double for the screens: everything it is asked does nothing and answers at once. */
export function idleSession(): CourseSession {
  const done = async (): Promise<undefined> => undefined;
  return { now: () => 2_000_000, open: () => undefined, chooseBoard: () => undefined, startRun: done, resumeRun: done, loadRun: done, forgetLearner: () => undefined, refreshLeaderboard: async () => ({ answered: true }), refreshCrowd: done, openTask: done, answer: () => undefined, askCrowd: () => undefined, unaskCrowd: () => undefined, submit: done } as unknown as CourseSession;
}

/** 🔑️ The tag, the parent's tag and the topic key of every element on screen that pets may play with. */
export function marked(selector = "[data-pet-prop]"): readonly (readonly [string, string | undefined, string | undefined])[] {
  return [...document.querySelectorAll<HTMLElement>(selector)].map((element) => [element.tagName, element.parentElement?.tagName, element.dataset.petProp]);
}
