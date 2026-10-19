/** ⛰️ The challenges through the session: the sheet of each, switching voids the open run, the clock of an expert run, the hints of an easy run, points by challenge and stored runs restored only in the shape of the challenges.
 *
 * @see ../🧰️learner-journey-kit/🟦️.tsx — the shared quiz, proctor double and builders
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the client under test
 */
import { waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { CHALLENGES, challengeRules, hintsOf, type Answer, type Challenge, type Command, type SheetTask } from "@semio-tech/course";
import { isChallenge, keysHidden, localStore, memoryStorageOrigin, newId, openChallengeOf, openRunOf, restoreCourseState } from "@semio-tech/course-react";
import { QUIZ, COURSE, decoder, FakeProctor, sloppyAnswer, sessionOn, startedRun, everyItemTo, MATERIAL, perfectAnswer, clockedOn, guessedAnswer, roughAnswer, reversedSorting, taskOf } from "../🧰️learner-journey-kit/🟦️.tsx";

describe("⛰️ the challenges through the session", () => {
  it("starts a run at every challenge with the sheet of that challenge, and a start at the same challenge resumes it", { timeout: 30_000 }, async () => {
    for (const challenge of CHALLENGES) {
      const proctor = new FakeProctor();
      const session = sessionOn(proctor, memoryStorageOrigin().tab());
      const signal = new AbortController().signal;
      expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
      expect(await session.startRun(QUIZ.id, challenge, signal)).toBeUndefined();
      const { step, runs } = session.getSnapshot().state;
      if (step.screen !== "run") throw new Error("the run did not open");
      const view = runs[step.run]!;
      const rules = challengeRules(challenge);
      expect(view.sheet.challenge).toBe(challenge);
      expect(JSON.parse(decoder.decode(proctor.envelopes.find((envelope) => envelope.kind === "course.start-run")!.payload))).toMatchObject({ run: step.run, quiz: QUIZ.id, challenge });
      const sorting = taskOf(session, step.run, "sorting");
      const matching = taskOf(session, step.run, "matching");
      expect(sorting.kind === "sorting" && sorting.keys !== undefined, challenge).toBe(rules.keys);
      expect(matching.kind === "matching" && matching.dimensions.every((dimension) => dimension.cards !== undefined), challenge).toBe(rules.keys);
      expect(view.sheet.tasks.every((task) => (task.seconds !== undefined) === rules.timed), challenge).toBe(true);
      expect(view.opened, challenge).toEqual(rules.timed ? {} : undefined);
      expect(keysHidden(view), challenge).toBe(!rules.keys);
      expect(await session.startRun(QUIZ.id, challenge, signal)).toBeUndefined();
      expect(session.getSnapshot().state.step).toEqual({ screen: "run", run: step.run });
      expect(proctor.runOf(step.run)?.runs.map((run) => run.run)).toEqual([step.run]);
      session.stop();
    }
  });

  it("voids the open run when the learner starts the quiz at another challenge, without a notice, and drops what it queued", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const { run, tasks } = await startedRun(session);
    proctor.outage = true;
    session.answer(run, tasks[0]!.id, sloppyAnswer(tasks[0]!));
    expect(session.outbox.queued(run)).toHaveLength(1);
    expect(openChallengeOf(session.getSnapshot().state, QUIZ.id)).toBe("medium");
    expect(await session.startRun(QUIZ.id, "hard", new AbortController().signal)).toBeUndefined();
    const { step, runs, notice } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the new run did not open");
    expect(step.run).not.toBe(run);
    expect(runs[run]?.status).toBe("voided");
    expect(runs[step.run]?.sheet.challenge).toBe("hard");
    expect(notice).toBeUndefined();
    expect(session.outbox.queued(run)).toEqual([]);
    expect(openRunOf(session.getSnapshot().state, QUIZ.id)).toBe(step.run);
    expect(openChallengeOf(session.getSnapshot().state, QUIZ.id)).toBe("hard");
    expect(proctor.runOf(run)?.runs.map((candidate) => [candidate.run, candidate.challenge, candidate.status])).toEqual([
      [run, "medium", "voided"],
      [step.run, "hard", "open"],
    ]);
    proctor.outage = false;
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.runs.map((summary) => [summary.challenge, summary.status])).toEqual([
      ["hard", "open"],
      ["medium", "voided"],
    ]));
    session.stop();
  });

  it("keeps the clock of an expert run by the session clock: opens a task once, takes its answers in time up to the deadline, refuses one after it on the device without sending it and submits with tasks unanswered", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = clockedOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    expect(session.now()).toBe(proctor.now);
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const view = () => session.getSnapshot().state.runs[run]!;
    const [first, second] = view().sheet.tasks;
    expect(view().opened).toEqual({});

    session.answer(run, first!.id, guessedAnswer(first!));
    await session.outbox.settled(run);
    expect(session.getSnapshot().state.notice).toEqual({ kind: "rejection", rejection: "task-unopened" });
    await waitFor(() => expect(view().answers[first!.id]).toBeUndefined());
    session.dismissNotice();

    const before = proctor.now;
    expect(await session.openTask(run, first!.id, signal)).toBeUndefined();
    const opening = proctor.envelopes.filter((envelope) => envelope.kind === "course.open-task");
    expect(opening.map((envelope) => JSON.parse(decoder.decode(envelope.payload)))).toEqual([{ type: "open-task", id: opening[0]!.commandId, learner: session.getSnapshot().state.learner!.id, run, task: first!.id, at: before }]);
    expect(view().opened).toEqual({ [first!.id]: before });
    expect(proctor.runOf(run)?.runs[0]?.opened).toEqual({ [first!.id]: before });
    expect(await session.openTask(run, first!.id, signal)).toBeUndefined();
    expect(proctor.envelopes.filter((envelope) => envelope.kind === "course.open-task")).toHaveLength(1);

    const answeredAt = proctor.now;
    session.answer(run, first!.id, guessedAnswer(first!));
    await session.outbox.settled(run);
    expect(session.getSnapshot().state.notice).toBeUndefined();
    expect(proctor.answersOf(run)[first!.id]).toEqual(guessedAnswer(first!));
    expect(JSON.parse(decoder.decode(proctor.envelopes.filter((envelope) => envelope.kind === "course.record-answer").at(-1)!.payload))).toMatchObject({ task: first!.id, at: answeredAt });

    expect(await session.openTask(run, second!.id, signal)).toBeUndefined();
    const deadline = view().opened![second!.id]! + second!.seconds! * 1000;
    proctor.now = deadline;
    const lastInTime = guessedAnswer(second!);
    session.answer(run, second!.id, lastInTime);
    expect(view().answers[second!.id]).toEqual(lastInTime);
    await session.outbox.settled(run);
    expect(session.getSnapshot().state.notice).toBeUndefined();
    expect(session.now()).toBeGreaterThan(deadline);
    const sent = proctor.envelopes.length;
    session.answer(run, second!.id, roughAnswer(second!));
    expect(session.getSnapshot().state.notice).toEqual({ kind: "rejection", rejection: "time-up" });
    expect(view().answers[second!.id]).toEqual(lastInTime);
    expect(session.outbox.queued(run)).toEqual([]);
    expect(proctor.envelopes).toHaveLength(sent);
    expect(proctor.answersOf(run)[second!.id]).toEqual(lastInTime);
    session.dismissNotice();

    expect(await session.submit(run, signal, () => undefined)).toBeUndefined();
    const result = view().result!;
    expect(result.challenge).toBe("expert");
    expect(result.tasks.map((task) => task.score)).toEqual(view().sheet.tasks.map((task) => (task.id === first!.id || task.id === second!.id ? 1 : 0)));
    expect(result.score).toBeCloseTo(2 / view().sheet.tasks.length, 12);
    expect(result.points).toBeCloseTo(result.score * 400, 9);
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.best[QUIZ.id]).toEqual({ challenge: "expert", score: result.score, points: result.points }));
    session.stop();
  });

  it("opens the tasks of an expert run on the device while the proctor is away and delivers each opening before the answers of its task", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = clockedOn(proctor, memoryStorageOrigin().tab(), MATERIAL);
    const signal = new AbortController().signal;
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const [first, second] = session.getSnapshot().state.runs[run]!.sheet.tasks;
    proctor.away = true;
    const firstAt = proctor.now;
    expect(await session.openTask(run, first!.id, signal)).toBeUndefined();
    expect(session.getSnapshot().state.runs[run]?.opened).toEqual({ [first!.id]: firstAt });
    session.answer(run, first!.id, guessedAnswer(first!));
    proctor.now += 5_000;
    const secondAt = proctor.now;
    expect(await session.openTask(run, second!.id, signal)).toBeUndefined();
    session.answer(run, second!.id, roughAnswer(second!));
    session.answer(run, first!.id, roughAnswer(first!));
    expect(session.getSnapshot().state.runs[run]?.opened).toEqual({ [first!.id]: firstAt, [second!.id]: secondAt });
    expect(session.outbox.queued(run).map((command) => (command.type === "open-task" || command.type === "record-answer" ? `${command.type} ${command.task}` : command.type))).toEqual([
      `open-task ${first!.id}`,
      `open-task ${second!.id}`,
      `record-answer ${second!.id}`,
      `record-answer ${first!.id}`,
    ]);
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    const delivered = proctor.envelopes.filter((envelope) => envelope.kind === "course.open-task" || envelope.kind === "course.record-answer").map((envelope) => JSON.parse(decoder.decode(envelope.payload)) as Command);
    expect(delivered.map((command) => (command.type === "open-task" || command.type === "record-answer" ? `${command.type} ${command.task}` : command.type))).toEqual([`open-task ${first!.id}`, `open-task ${second!.id}`, `record-answer ${second!.id}`, `record-answer ${first!.id}`]);
    expect(proctor.runOf(run)?.runs[0]?.opened).toEqual({ [first!.id]: firstAt, [second!.id]: secondAt });
    expect(proctor.answersOf(run)).toEqual({ [first!.id]: roughAnswer(first!), [second!.id]: roughAnswer(second!) });
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

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
});
