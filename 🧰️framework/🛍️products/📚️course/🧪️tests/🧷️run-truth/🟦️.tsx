/** 🧷️ What the device shows of a run stays true to its answers: hints never outlive the answer they were given for, another tab's copy merges, a refused answer is read again and an answer is judged at the instant of its own last edit.
 *
 * @see ../🧰️learner-journey-kit/🟦️.tsx — the shared quiz, proctor double and builders
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the client under test
 */
import { waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { decodeQueryEnvelope, type HttpTransport } from "@semio-tech/framework-server";
import { hintsOf, type Answer, type Hint, type Query, type Quiz, type RecordAnswerCommand, type RunView } from "@semio-tech/course";
import { ProctorClient, CourseSession, evolveCourseState, initialCourseState, localStore, memoryStorageOrigin, mergeRunViews, newId, openRunOf, type CourseMaterial } from "@semio-tech/course-react";
import { text, QUIZ, COURSE, decoder, FakeProctor, TIMING, sessionOn, classification, everyItemTo, MATERIAL, deputisedOn, perfectAnswer, clockedOn, guessedAnswer, roughAnswer, reversedSorting, taskOf } from "../🧰️learner-journey-kit/🟦️.tsx";

/** ✋️ A transport to `proctor` that, while `hold.on`, keeps the answer to every read of a run back until its release in
 * `held` is called — the releases in the order the reads were asked. */
function heldTransport(proctor: FakeProctor): { readonly transport: HttpTransport; readonly hold: { on: boolean }; readonly held: (() => void)[] } {
  const hold = { on: false };
  const held: (() => void)[] = [];
  const transport: HttpTransport = {
    send: async (request) => {
      if (request.path !== "/queries") return proctor.transport.send(request);
      const body = JSON.parse(typeof request.body === "string" ? request.body : decoder.decode(request.body)) as unknown;
      const read = request.path === "/queries" && (JSON.parse(decoder.decode(decodeQueryEnvelope(body).arguments)) as Query).type === "run";
      const kept = hold.on && read;
      const response = await proctor.transport.send(request);
      if (kept) await new Promise<void>((release) => held.push(release));
      return response;
    },
  };
  return { transport, hold, held };
}

/** 🍳️ A second quiz beside the household one, so that two runs of different quizzes can be open at once. */
const KITCHEN: Quiz = { ...QUIZ, id: "kitchen", emoji: "🍳", title: text("Kitchen physics", "Küchenphysik") };

/** 🗄️ The household course with the kitchen quiz beside it. */
const TWO_QUIZZES: CourseMaterial = { course: { ...COURSE, quizzes: ["household.json", "kitchen.json"] }, quizzes: [QUIZ, KITCHEN] };

/** ⏱️ The run view of an open timed run of the household quiz with `answers` and `extra` members, for the pure folds. */
function heldRun(answers: RunView["answers"], extra: Partial<RunView> = {}): RunView {
  return { run: "b".repeat(32), learner: "a".repeat(32), quiz: QUIZ.id, status: "open", sheet: { quiz: QUIZ.id, seed: 1, challenge: "expert", title: QUIZ.title, description: QUIZ.description, tasks: [] }, answers, startedAt: 1, ...extra };
}

describe("🧷️ what the device shows of a run stays true to its answers", () => {
  it("drops the hints of a task the moment its answer changes and shows those of the new answer only once the proctor's view of it arrives (no deputy)", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "easy", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const view = (): RunView => session.getSnapshot().state.runs[run]!;
    const sorting = taskOf(session, run, "sorting");
    const classifying = classification(view().sheet.tasks);
    const misplaced = { [classifying.id]: hintsOf(QUIZ.tasks.find((task) => task.id === classifying.id)!, classifying, everyItemTo(classifying, 0)) };
    expect(misplaced[classifying.id]!.length).toBeGreaterThan(0);
    session.answer(run, sorting.id, reversedSorting(sorting));
    session.answer(run, classifying.id, everyItemTo(classifying, 0));
    await waitFor(() => expect(view().hints).toEqual({ ...misplaced, [sorting.id]: hintsOf(QUIZ.tasks.find((task) => task.id === sorting.id)!, sorting, reversedSorting(sorting)) }));
    proctor.outage = true;
    session.answer(run, sorting.id, perfectAnswer(sorting));
    expect(view().hints).toEqual(misplaced);
    await session.loadRun(run);
    expect(view().answers[sorting.id]).toEqual(perfectAnswer(sorting));
    expect(view().hints).toEqual(misplaced);
    proctor.outage = false;
    session.outbox.wake();
    await waitFor(() => expect(proctor.answersOf(run)[sorting.id]).toEqual(perfectAnswer(sorting)));
    await waitFor(() => expect(session.outbox.queued()).toEqual([]));
    expect(view().hints).toEqual(misplaced);
    session.stop();
  });

  it("has the deputy tell the hints of the answers shown, also when a read of the run comes back while a newer answer waits", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = deputisedOn(proctor, memoryStorageOrigin().tab());
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "easy", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const view = (): RunView => session.getSnapshot().state.runs[run]!;
    const sorting = taskOf(session, run, "sorting");
    const classifying = classification(view().sheet.tasks);
    const misplaced = { [classifying.id]: hintsOf(QUIZ.tasks.find((task) => task.id === classifying.id)!, classifying, everyItemTo(classifying, 0)) };
    expect(misplaced[classifying.id]!.length).toBeGreaterThan(0);
    session.answer(run, sorting.id, reversedSorting(sorting));
    session.answer(run, classifying.id, everyItemTo(classifying, 0));
    await session.outbox.settled(run);
    expect(view().hints).toEqual({ ...misplaced, [sorting.id]: hintsOf(QUIZ.tasks.find((task) => task.id === sorting.id)!, sorting, reversedSorting(sorting)) });
    proctor.outage = true;
    session.answer(run, sorting.id, perfectAnswer(sorting));
    expect(view().hints).toEqual(misplaced);
    const queried = proctor.runQueries;
    await session.loadRun(run);
    expect(proctor.runQueries).toBeGreaterThan(queried);
    expect(proctor.answersOf(run)[sorting.id]).toEqual(reversedSorting(sorting));
    expect(view().answers[sorting.id]).toEqual(perfectAnswer(sorting));
    expect(view().hints).toEqual(misplaced);
    proctor.outage = false;
    session.stop();
  });

  it("never lets a read of the run asked for before a newer answer bring back the older answer or its hints, and drops a read that a later one overtook", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const { transport, hold, held } = heldTransport(proctor);
    const session = new CourseSession({ proctor: new ProctorClient(() => transport, COURSE.id), store: localStore(memoryStorageOrigin().tab(), COURSE.id), timing: TIMING });
    session.start();
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "easy", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const view = (): RunView => session.getSnapshot().state.runs[run]!;
    const sorting = taskOf(session, run, "sorting");
    session.answer(run, sorting.id, reversedSorting(sorting));
    await waitFor(() => expect(Object.keys(view().hints ?? {})).toEqual([sorting.id]));
    hold.on = true;
    const stale = session.loadRun(run);
    await waitFor(() => expect(held).toHaveLength(1));
    session.answer(run, sorting.id, perfectAnswer(sorting));
    await session.outbox.settled(run);
    await waitFor(() => expect(held).toHaveLength(2));
    held[0]!();
    await stale;
    expect(view().answers[sorting.id]).toEqual(perfectAnswer(sorting));
    expect(view().hints).toBeUndefined();
    held[1]!();
    const classifying = classification(view().sheet.tasks);
    const older = session.loadRun(run);
    await waitFor(() => expect(held).toHaveLength(3));
    proctor.seed({ type: "record-answer", id: newId(), learner: session.getSnapshot().state.learner!.id, run, task: classifying.id, answer: everyItemTo(classifying, 1), at: proctor.now });
    const newer = session.loadRun(run);
    await waitFor(() => expect(held).toHaveLength(4));
    held[3]!();
    await newer;
    expect(view().answers[classifying.id]).toEqual(everyItemTo(classifying, 1));
    const doubted = { [classifying.id]: hintsOf(QUIZ.tasks.find((task) => task.id === classifying.id)!, classifying, everyItemTo(classifying, 1)) };
    expect(doubted[classifying.id]!.length).toBeGreaterThan(0);
    expect(view().hints).toEqual(doubted);
    held[2]!();
    await older;
    expect(view().answers).toEqual({ [sorting.id]: perfectAnswer(sorting), [classifying.id]: everyItemTo(classifying, 1) });
    expect(view().hints).toEqual(doubted);
    hold.on = false;
    session.stop();
  });

  it("merges another tab's copy of an open run: the union of the answers, each task with the hints of the copy whose answer it keeps, every opening at its earliest instant", () => {
    const order = (...items: string[]): Answer => ({ kind: "sorting", order: items });
    const doubt = (item: string, other: string): Hint => ({ kind: "compare", item, other, factor: 2, verdict: "under" });
    const mine = heldRun({ a: order("x", "y"), b: order("y", "x") }, { opened: { a: 5, c: 9 }, hints: { a: [doubt("x", "y")], b: [doubt("y", "x")] } });
    const theirs = heldRun({ a: order("y", "x") }, { opened: { b: 7, c: 3 }, hints: { a: [doubt("y", "x")] } });
    expect(mergeRunViews(mine, theirs)).toEqual({ ...theirs, answers: { a: order("y", "x"), b: order("y", "x") }, opened: { a: 5, b: 7, c: 3 }, hints: { a: [doubt("y", "x")], b: [doubt("y", "x")] } });
    expect(mergeRunViews(heldRun({ a: order("x") }), heldRun({}))).toStrictEqual(heldRun({ a: order("x") }));
    const submitted = { ...theirs, status: "submitted" as const };
    expect(mergeRunViews(mine, submitted)).toBe(submitted);
    const voided = { ...mine, status: "voided" as const };
    expect(mergeRunViews(voided, theirs)).toBe(voided);
  });

  it("keeps no hints on a run once it is submitted, voided or closed by its listing — the openings of a timed run stay, as in the proctor's view — and drops only those of a task whose answer changes", () => {
    const learner = "a".repeat(32);
    const order = (...items: string[]): Answer => ({ kind: "sorting", order: items });
    const view = heldRun({ a: order("x", "y"), b: order("y", "x") }, { opened: { a: 2, b: 3 }, hints: { a: [{ kind: "group", item: "x", other: "y", together: true }], b: [{ kind: "category", item: "y", category: "c" }] } });
    const held = { ...initialCourseState({ introduced: true, learner: { id: learner }, runs: { [view.run]: view } }), step: { screen: "run", run: view.run } as const };
    const result = { quiz: QUIZ.id, challenge: "expert" as const, score: 1, points: 400, tasks: [] };
    const listed = { learner, identity: { kind: "anonymous" as const }, runs: [{ run: view.run, quiz: QUIZ.id, challenge: "expert" as const, status: "submitted" as const, startedAt: 1, score: 1, points: 400, submittedAt: 4 }], badges: [], best: {}, total: 0 };
    for (const event of [{ type: "run-submitted", run: view.run, result, badges: [], at: 4 }, { type: "run-voided", run: view.run }, { type: "learner-loaded", view: listed }] as const) {
      const closed = evolveCourseState(held, event).runs[view.run]!;
      expect(closed.status, event.type).not.toBe("open");
      expect(Object.hasOwn(closed, "hints"), event.type).toBe(false);
      expect(closed.opened, event.type).toEqual({ a: 2, b: 3 });
    }
    expect(evolveCourseState(held, { type: "answer-given", run: view.run, task: "a", answer: order("y", "x") }).runs[view.run]!.hints).toEqual({ b: [{ kind: "category", item: "y", category: "c" }] });
    const once = evolveCourseState(held, { type: "answer-given", run: view.run, task: "a", answer: order("y", "x") });
    expect(Object.hasOwn(evolveCourseState(once, { type: "answer-given", run: view.run, task: "b", answer: order("x", "y") }).runs[view.run]!, "hints")).toBe(false);
  });

  it("reads a run again at once after the proctor refused one of its answers, while the deputy's start of a run of another quiz still waits", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor(TWO_QUIZZES);
    const session = clockedOn(proctor, memoryStorageOrigin().tab(), TWO_QUIZZES);
    const signal = new AbortController().signal;
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const [first] = session.getSnapshot().state.runs[run]!.sheet.tasks;
    proctor.outage = true;
    session.answer(run, first!.id, guessedAnswer(first!));
    expect(session.getSnapshot().state.runs[run]!.answers[first!.id]).toEqual(guessedAnswer(first!));
    proctor.away = true;
    expect(await session.startRun(KITCHEN.id, "medium", signal)).toBeUndefined();
    const other = session.getSnapshot().state.step;
    if (other.screen !== "run" || other.run === run) throw new Error("the kitchen run did not open");
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["record-answer", "start-run"]);
    proctor.outage = false;
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    expect(session.getSnapshot().state.notice).toEqual({ kind: "rejection", rejection: "task-unopened" });
    await waitFor(() => expect(session.getSnapshot().state.runs[run]!.answers[first!.id]).toBeUndefined(), { timeout: 10_000 });
    expect(proctor.runOf(run)?.runs.map((candidate) => [candidate.quiz, candidate.status])).toEqual([
      [QUIZ.id, "open"],
      [KITCHEN.id, "open"],
    ]);
    session.stop();
  });

  it("voids a run on the device when the proctor refuses an opening the deputy decided because the quiz was revised meanwhile", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = clockedOn(proctor, memoryStorageOrigin().tab(), MATERIAL);
    const signal = new AbortController().signal;
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const [first] = session.getSnapshot().state.runs[run]!.sheet.tasks;
    proctor.away = true;
    expect(await session.openTask(run, first!.id, signal)).toBeUndefined();
    expect(session.outbox.queued(run).map((command) => command.type)).toEqual(["open-task"]);
    proctor.revise(QUIZ.id);
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.status).toBe("voided"), { timeout: 10_000 });
    expect(session.getSnapshot().state.notice).toEqual({ kind: "voided" });
    expect(session.outbox.queued()).toEqual([]);
    expect(session.getSnapshot().state.step).toEqual({ screen: "home" });
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.runs.map((summary) => summary.run)).toEqual([run]));
    expect(openRunOf(session.getSnapshot().state, QUIZ.id)).toBeUndefined();
    session.stop();
  });

  it("delivers an answer replaced while it waited with the instant of its own last edit, which is what the deciders judge", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = clockedOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    expect(await session.identify({ kind: "anonymous" }, signal)).toBeUndefined();
    expect(await session.startRun(QUIZ.id, "expert", signal)).toBeUndefined();
    const { step } = session.getSnapshot().state;
    if (step.screen !== "run") throw new Error("the run did not open");
    const run = step.run;
    const [first] = session.getSnapshot().state.runs[run]!.sheet.tasks;
    expect(await session.openTask(run, first!.id, signal)).toBeUndefined();
    proctor.away = true;
    session.answer(run, first!.id, roughAnswer(first!));
    await waitFor(() => expect(session.getSnapshot().connection.activity).toBe("retrying"));
    proctor.now += 2_000;
    const lastAt = proctor.now;
    session.answer(run, first!.id, guessedAnswer(first!));
    expect(session.outbox.queued(run)).toEqual([expect.objectContaining({ type: "record-answer", task: first!.id, answer: guessedAnswer(first!), at: lastAt })]);
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    const delivered = proctor.envelopes.filter((envelope) => envelope.kind === "course.record-answer").map((envelope) => JSON.parse(decoder.decode(envelope.payload)) as RecordAnswerCommand);
    expect(delivered.map((command) => [command.answer, command.at])).toEqual([[guessedAnswer(first!), lastAt]]);
    expect(proctor.answersOf(run)[first!.id]).toEqual(guessedAnswer(first!));
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });
});
