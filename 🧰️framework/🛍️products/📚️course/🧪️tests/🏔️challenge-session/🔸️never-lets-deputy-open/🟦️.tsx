/** 🔸️ The never lets deputy open tests of the ⛰️ the challenges through the session suite. */
import { waitFor } from "@testing-library/react";
import { expect, it } from "vitest";
import { hintsOf, type Answer, type Challenge, type SheetTask } from "@semio-tech/course";
import { isChallenge, localStore, memoryStorageOrigin, newId, restoreCourseState } from "@semio-tech/course-react";
import { QUIZ, COURSE, FakeProctor, sessionOn, everyItemTo, MATERIAL, perfectAnswer, clockedOn, guessedAnswer, reversedSorting, taskOf } from "../../🧰️learner-journey-kit/🟦️.tsx";

/** 🔹 Registers the never lets deputy open tests of the ⛰️ the challenges through the session suite. */
export function registerNeverLetsDeputyOpenTests(): void {
  it("never lets the deputy open a task or submit a run it knows from its listing alone: it loads the run from the proctor first", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const handle = `Clock ${newId().slice(0, 6)}`;
    const first = clockedOn(proctor, memoryStorageOrigin().tab(), MATERIAL);
    const signal = new AbortController().signal;
    await waitFor(() => expect(first.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await first.identify({ kind: "pseudonym", handle }, signal)).toBeUndefined();
    expect(await first.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const { step } = first.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const [task] = first.getSnapshot().state.runs[run]!.sheet.tasks;
    expect(await first.openTask(run, task!.id, signal)).toBeUndefined();
    const opened = first.getSnapshot().state.runs[run]!.opened;
    first.stop();

    const other = clockedOn(proctor, memoryStorageOrigin().tab(), MATERIAL);
    await waitFor(() => expect(other.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await other.identify({ kind: "pseudonym", handle }, signal)).toBeUndefined();
    await waitFor(() => expect(other.getSnapshot().state.learnerView?.runs.map((summary) => [summary.run, summary.challenge, summary.status])).toEqual([[run, "expert", "open"]]));
    expect(other.getSnapshot().state.runs[run]).toBeUndefined();
    proctor.away = true;
    for (const attempt of [(cancel: AbortSignal) => other.openTask(run, task!.id, cancel), (cancel: AbortSignal) => other.submit(run, cancel, () => undefined)]) {
      const controller = new AbortController();
      const waiting = attempt(controller.signal);
      await new Promise((resolve) => setTimeout(resolve, 60));
      controller.abort(new Error("the learner gave up waiting"));
      await expect(waiting).rejects.toThrow("the learner gave up waiting");
      expect(other.outbox.queued()).toEqual([]);
      expect(other.getSnapshot().state.runs[run]?.status ?? "open").toBe("open");
    }
    proctor.away = false;
    expect(await other.openTask(run, task!.id, signal)).toBeUndefined();
    expect(other.getSnapshot().state.runs[run]?.opened).toEqual(opened);
    expect(proctor.envelopes.filter((envelope) => envelope.kind === "course.open-task")).toHaveLength(1);
    other.stop();
  });
  it("tells the hints of an easy run after an answer once the proctor's view of it arrives, and none on medium", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "easy", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const sorting = taskOf(session, run, "sorting");
    const source = QUIZ.tasks.find((task) => task.id === sorting.id)!;
    const reversed = reversedSorting(sorting);
    const expected = hintsOf(source, sorting, reversed);
    expect(expected.map((hint) => hint.kind)).toEqual(["compare", "compare"]);
    expect(session.getSnapshot().state.runs[run]?.hints).toBeUndefined();
    session.answer(run, sorting.id, reversed);
    expect(session.getSnapshot().state.runs[run]?.hints).toBeUndefined();
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.hints).toEqual({ [sorting.id]: expected }));
    session.answer(run, sorting.id, perfectAnswer(sorting));
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.hints).toBeUndefined());

    expect(await session.startRun(QUIZ.id, "medium", signal)).toBeUndefined();
    const medium = session.getSnapshot().state.step;
    if (medium.screen !== "run") throw new Error("the run did not open");
    const queried = proctor.runQueries;
    session.answer(medium.run, taskOf(session, medium.run, "sorting").id, reversedSorting(taskOf(session, medium.run, "sorting")));
    await session.outbox.settled(medium.run);
    expect(proctor.runQueries).toBe(queried);
    expect(session.getSnapshot().state.runs[medium.run]?.hints).toBeUndefined();
    session.stop();
  });
  it("tells the hints of an easy run at once from the deputy, also while the proctor is away", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.away = true;
    const session = clockedOn(proctor, memoryStorageOrigin().tab(), MATERIAL);
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "easy", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const sorting = taskOf(session, step.run, "sorting");
    const reversed = reversedSorting(sorting);
    session.answer(step.run, sorting.id, reversed);
    expect(session.getSnapshot().state.runs[step.run]?.hints).toEqual({ [sorting.id]: hintsOf(QUIZ.tasks.find((task) => task.id === sorting.id)!, sorting, reversed) });
    const classifying = taskOf(session, step.run, "classification");
    if (classifying.kind !== "classification") throw new Error("not the classification");
    session.answer(step.run, classifying.id, everyItemTo(classifying, 0));
    const doubted = hintsOf(QUIZ.tasks.find((task) => task.id === classifying.id)!, classifying, everyItemTo(classifying, 0));
    expect(doubted.length).toBeGreaterThan(0);
    expect(session.getSnapshot().state.runs[step.run]?.hints?.[classifying.id]).toEqual(doubted);
    session.answer(step.run, sorting.id, perfectAnswer(sorting));
    expect(Object.keys(session.getSnapshot().state.runs[step.run]?.hints ?? {})).toEqual([classifying.id]);
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    await waitFor(() => expect(session.getSnapshot().state.runs[step.run]?.hints).toEqual({ [classifying.id]: doubted }));
    session.stop();
  });
  it("scores a hard run's guesses at its par and keeps the best run by points across challenges", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    const played = async (challenge: Challenge, answer: (task: SheetTask) => Answer): Promise<string> => {
      expect(await session.startRun(QUIZ.id, challenge, signal)).toBeUndefined();
      const { step, runs } = session.getSnapshot().state;
      if (step.screen !== "run") throw new Error("the run did not open");
      for (const task of runs[step.run]!.sheet.tasks) session.answer(step.run, task.id, answer(task));
      expect(await session.submit(step.run, signal, () => undefined)).toBeUndefined();
      return step.run;
    };
    const hard = await played("hard", guessedAnswer);
    expect(session.getSnapshot().state.runs[hard]?.result).toMatchObject({ challenge: "hard", score: 1, points: 300 });
    const easy = await played("easy", perfectAnswer);
    expect(session.getSnapshot().state.runs[easy]?.result).toMatchObject({ challenge: "easy", score: 1, points: 100 });
    await waitFor(() => expect(session.getSnapshot().state.learnerView).toMatchObject({ best: { [QUIZ.id]: { challenge: "hard", score: 1, points: 300 } }, total: 300 }));
    expect(session.getSnapshot().state.learnerView?.runs.map((summary) => [summary.challenge, summary.points])).toEqual([
      ["easy", 100],
      ["hard", 300],
    ]);
    session.stop();
  });
  it("restores a stored run only with the challenge of its sheet", () => {
    const area = memoryStorageOrigin().tab();
    const store = localStore(area, COURSE.id);
    const learner = "a".repeat(32);
    store.write("introduced", true);
    store.write("learner", { id: learner, identity: { kind: "anonymous" } });
    const sheet = { quiz: QUIZ.id, seed: 1, challenge: "hard", title: QUIZ.title, description: QUIZ.description, tasks: [] };
    const view = { run: "b".repeat(32), learner, quiz: QUIZ.id, status: "open", sheet, answers: {}, startedAt: 1 };
    store.put("runs", view.run, view);
    const { challenge: _, ...unchallenged } = sheet;
    store.put("runs", "c".repeat(32), { ...view, run: "c".repeat(32), sheet: unchallenged });
    store.put("runs", "d".repeat(32), { ...view, run: "d".repeat(32), sheet: { ...sheet, challenge: "lenient" } });
    expect(Object.keys(restoreCourseState(store).runs)).toEqual([view.run]);
    expect(isChallenge("expert")).toBe(true);
    expect([undefined, "Expert", "level", 3].some(isChallenge)).toBe(false);
  });
  it("restores a stored learner view only in the shape of the challenges: every run with its challenge, every best with the challenge, score and points of its run", () => {
    const store = localStore(memoryStorageOrigin().tab(), COURSE.id);
    const learner = "a".repeat(32);
    store.write("introduced", true);
    store.write("learner", { id: learner, identity: { kind: "anonymous" } });
    const summary = { run: "b".repeat(32), quiz: QUIZ.id, challenge: "hard", status: "submitted", startedAt: 1, score: 0.9, points: 270, submittedAt: 2 };
    const current = { learner, identity: { kind: "anonymous" }, runs: [summary], badges: [], best: { [QUIZ.id]: { challenge: "hard", score: 0.9, points: 270 } }, total: 270 };
    store.write("learner-view", current);
    expect(restoreCourseState(store).learnerView).toEqual(current);
    const { challenge: _, ...unchallenged } = summary;
    const { best: __, ...bestless } = current;
    for (const old of [{ ...current, best: { [QUIZ.id]: 0.9 } }, { ...current, runs: [unchallenged] }, { ...current, best: { [QUIZ.id]: { score: 0.9, points: 270 } } }, { ...current, best: { [QUIZ.id]: { challenge: "hard", score: 0.9 } } }, bestless]) {
      store.write("learner-view", old);
      expect(restoreCourseState(store).learnerView, JSON.stringify(old)).toBeUndefined();
    }
  });
}
