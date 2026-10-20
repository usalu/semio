/** 🔶️ The starts run challenge sheet tests of the ⛰️ the challenges through the session suite. */
import { waitFor } from "@testing-library/react";
import { expect, it } from "vitest";
import { CHALLENGES, challengeRules, type Command } from "@semio-tech/course";
import { keysHidden, memoryStorageOrigin, openChallengeOf, openRunOf } from "@semio-tech/course-react";
import { QUIZ, decoder, FakeProctor, sloppyAnswer, sessionOn, startedRun, MATERIAL, clockedOn, guessedAnswer, roughAnswer, taskOf } from "../../🧰️learner-journey-kit/🟦️.tsx";

/** 🔷 Registers the starts run challenge sheet tests of the ⛰️ the challenges through the session suite. */
export function registerStartsRunChallengeSheetTests(): void {
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
}
