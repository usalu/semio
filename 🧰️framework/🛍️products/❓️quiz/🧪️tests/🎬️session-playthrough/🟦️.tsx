/** 🎬️ A whole learner session through the public API of `QuizSession` alone (no screen): the introduction, a registration, a run, answers, the submission with its phases, results, board, crowd, navigation, a reload, an expert run on the session clock with a cancelled submission, and the same session with a deputy while the proctor is away.
 *
 * @see ../🧰️learner-journey-kit/🟦️.tsx — the shared quiz, proctor double and builders
 * @see ../../🎯️targets/⚛️react/🔨️modules/🧭️session/🟦️.ts — the session under test
 */
import { waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { memoryStorageOrigin, type QuizSession, type QuizState, type QuizStep, type SubmissionPhase } from "@semio-tech/quiz-react";
import { CATALOG, FakeProctor, QUIZ, clockedOn, deputisedOn, guessedAnswer, perfectAnswer, sessionOn } from "../🧰️learner-journey-kit/🟦️.tsx";

const label = (step: QuizStep): string => (step.screen === "home" ? `home${step.page === undefined ? "" : `#${step.page}`}` : step.screen);

const sent = (proctor: FakeProctor): string[] => proctor.envelopes.map((envelope) => envelope.kind.replace("quiz.", ""));

const SAVED: SubmissionPhase[] = [
  { phase: "saving", done: 0, total: 3 },
  { phase: "saving", done: 1, total: 3 },
  { phase: "saving", done: 2, total: 3 },
  { phase: "saving", done: 3, total: 3 },
  { phase: "submitting" },
  { phase: "results" },
];

function watched(session: QuizSession): { readonly steps: string[]; readonly changes: () => number; readonly stop: () => void } {
  const steps: string[] = [label(session.getSnapshot().state.step)];
  let changes = 0;
  const stop = session.subscribe(() => {
    changes += 1;
    const next = label(session.getSnapshot().state.step);
    if (steps.at(-1) !== next) steps.push(next);
  });
  return { steps, changes: () => changes, stop };
}

function openRun(state: QuizState): string {
  if (state.step.screen !== "run") throw new Error("the run did not open");
  return state.step.run;
}

describe("🎬️ a whole session through the public API", () => {
  it("plays the household quiz from the introduction to the results, finds its way along the trail and resumes after a reload", { timeout: 60_000 }, async () => {
    const proctor = new FakeProctor();
    const origin = memoryStorageOrigin();
    const session = sessionOn(proctor, origin.tab());
    const watch = watched(session);
    const first = session.getSnapshot();
    expect(session.getSnapshot()).toBe(first);
    expect(first.connection.deputy).toBe(false);
    expect(first.state.step).toEqual({ screen: "introduction" });
    await waitFor(() => expect(session.getSnapshot().state.catalog?.id).toBe(CATALOG.id));
    expect(session.getSnapshot()).not.toBe(first);
    session.readIntroduction();
    expect(session.getSnapshot().state.introduced).toBe(true);
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "pseudonym", handle: "Ada" }, signal)).toBeUndefined();
    const learner = session.getSnapshot().state.learner!;
    expect(learner.identity).toEqual({ kind: "pseudonym", handle: "Ada" });
    expect(watch.steps).toEqual(["introduction", "identity", "home"]);
    session.open({ screen: "home", page: "board" });
    expect(await session.refreshLeaderboard()).toEqual({ answered: true });
    expect(Object.keys(session.getSnapshot().state.leaderboards)).toEqual(["all-time"]);
    session.chooseBoard({ period: "weekly" });
    await waitFor(() => expect(Object.keys(session.getSnapshot().state.leaderboards).sort()).toEqual(["all-time", "weekly"]));
    session.chooseBoard({ period: "all-time" });
    expect(await session.startRun(QUIZ.id, "medium", signal)).toBeUndefined();
    const run = openRun(session.getSnapshot().state);
    const tasks = session.getSnapshot().state.runs[run]!.sheet.tasks;
    expect(tasks.map((task) => task.id).sort()).toEqual(["climates", "lamps", "masses"]);
    for (const task of tasks) session.answer(run, task.id, perfectAnswer(task));
    expect(Object.keys(session.getSnapshot().state.runs[run]!.answers).sort()).toEqual(["climates", "lamps", "masses"]);
    session.askCrowd(QUIZ.id);
    expect(session.getSnapshot().state.asked).toEqual([QUIZ.id]);
    session.unaskCrowd(QUIZ.id);
    expect(session.getSnapshot().state.asked).toEqual([]);
    session.askCrowd(QUIZ.id);
    const phases: SubmissionPhase[] = [];
    expect(await session.submit(run, signal, (phase) => phases.push(phase))).toBeUndefined();
    const state = session.getSnapshot().state;
    expect(phases).toEqual(SAVED);
    expect(state.runs[run]!.status).toBe("submitted");
    expect(state.awards[run]).toEqual(["all-done", "flawless"]);
    expect(state.asked).toEqual([]);
    expect(Object.keys(state.crowds)).toEqual([QUIZ.id]);
    expect(state.step).toEqual({ screen: "results", run });
    expect(sent(proctor)).toEqual(["identify-learner", "start-run", "record-answer", "record-answer", "record-answer", "submit-run"]);
    expect(session.getSnapshot().connection.pending).toBe(0);
    expect(watch.changes()).toBeGreaterThan(0);
    session.back();
    expect(label(session.getSnapshot().state.step)).toBe("home#board");
    session.forward();
    expect(session.getSnapshot().state.step).toEqual({ screen: "results", run });
    session.up();
    expect(label(session.getSnapshot().state.step)).toBe("home#household");
    expect(watch.steps).toEqual(["introduction", "identity", "home", "home#board", "run", "results", "home#board", "results", "home#household"]);
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.runs.find((summary) => summary.run === run)?.status).toBe("submitted"));
    await session.refreshCatalog();
    await session.refreshLearner();
    await session.refreshCrowd(QUIZ.id);
    expect(session.getSnapshot().state.learnerView?.badges.map((award) => award.badge)).toEqual(["all-done", "flawless"]);
    expect(Number.isFinite(session.now())).toBe(true);
    const calls = watch.changes();
    watch.stop();
    session.dismissNotice();
    session.stop();
    expect(watch.changes()).toBe(calls);
    const reloaded = sessionOn(proctor, origin.tab());
    expect(reloaded.getSnapshot().state.step).toEqual({ screen: "home" });
    expect(reloaded.getSnapshot().state.learner).toEqual(learner);
    expect(Object.keys(reloaded.getSnapshot().state.runs)).toEqual([run]);
    reloaded.forgetLearner();
    expect(reloaded.getSnapshot().state.step).toEqual({ screen: "identity" });
    expect(reloaded.getSnapshot().state.learner).toBeUndefined();
    reloaded.stop();
  });

  it("plays an expert run on the session clock: every opening once, a submission cancelled and a submission through", { timeout: 60_000 }, async () => {
    const proctor = new FakeProctor();
    const session = clockedOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const run = openRun(session.getSnapshot().state);
    const tasks = session.getSnapshot().state.runs[run]!.sheet.tasks;
    for (const task of tasks) expect(await session.openTask(run, task.id, signal)).toBeUndefined();
    expect(await session.openTask(run, tasks[0]!.id, signal)).toBeUndefined();
    expect(Object.keys(session.getSnapshot().state.runs[run]!.opened ?? {}).sort()).toEqual(["climates", "lamps", "masses"]);
    expect(sent(proctor).filter((kind) => kind === "open-task")).toHaveLength(3);
    for (const task of tasks) session.answer(run, task.id, guessedAnswer(task));
    const cancelled = new AbortController();
    cancelled.abort(new Error("cancelled"));
    await expect(session.submit(run, cancelled.signal, () => undefined)).rejects.toThrow("cancelled");
    expect(session.getSnapshot().state.runs[run]!.status).toBe("open");
    expect(sent(proctor)).not.toContain("submit-run");
    const phases: SubmissionPhase[] = [];
    expect(await session.submit(run, signal, (phase) => phases.push(phase))).toBeUndefined();
    expect(phases).toEqual(SAVED);
    expect(session.getSnapshot().state.runs[run]!.status).toBe("submitted");
    expect(session.getSnapshot().state.step).toEqual({ screen: "results", run });
    expect(sent(proctor).filter((kind) => kind === "record-answer")).toHaveLength(3);
    expect(sent(proctor).at(-1)).toBe("submit-run");
    session.stop();
  });

  it("decides on the device while the proctor is away and hands every decision over, in order, once it is back", { timeout: 60_000 }, async () => {
    const proctor = new FakeProctor();
    const session = deputisedOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    proctor.away = true;
    expect(await session.identify({ kind: "pseudonym", handle: "Grace" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "easy", signal)).toBeUndefined();
    const run = openRun(session.getSnapshot().state);
    for (const task of session.getSnapshot().state.runs[run]!.sheet.tasks) session.answer(run, task.id, perfectAnswer(task));
    const phases: SubmissionPhase[] = [];
    expect(await session.submit(run, signal, (phase) => phases.push(phase))).toBeUndefined();
    expect(phases.map((phase) => phase.phase)).toEqual(["submitting", "results"]);
    expect(session.getSnapshot().state.runs[run]?.status).toBe("submitted");
    expect(session.getSnapshot().state.step).toEqual({ screen: "results", run });
    expect(session.getSnapshot().connection).toMatchObject({ deputy: true, pending: 6, online: true });
    expect(sent(proctor)).toEqual([]);
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.getSnapshot().connection.pending).toBe(0), { timeout: 20_000 });
    expect(sent(proctor)).toEqual(["identify-learner", "start-run", "record-answer", "record-answer", "record-answer", "submit-run"]);
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.runs.map((summary) => summary.status)).toEqual(["submitted"]));
    expect(await session.refreshLeaderboard()).toEqual({ answered: true });
    session.stop();
  });
});
