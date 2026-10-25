/** 🫡️ The deputy while the proctor is away, a proctor of another contract and a screen that fails.
 *
 * @see ../🧰️learner-journey-kit/🟦️.tsx — the shared quiz, proctor double and builders
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the client under test
 */
import { StrictMode, type ReactElement } from "react";
import cancellation from "../../🧫️fixtures/🛑️cancellation/🔣️.json";
import { ProctorLink } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🔨️modules/🧭️session/📡️proctor-link/🟦️.ts";
import { act, render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { type HttpTransport } from "@semio-tech/framework-server";
import { WIRE_VERSION, learnerTag, runView, type RunView } from "@semio-tech/course";
import { AGREEMENT_RECHECK_MS, CourseApp, Deputy, ProctorClient, ProctorUnavailable, COURSE_LOCALES, CourseSession, ScreenBoundary, connectionMessage, keysHidden, localStore, memoryStorageOrigin, newId, openRunOf, courseText, shownLeaderboard, waitingMessage, type CourseMaterial, type CourseMaterialSource, type StorageArea } from "@semio-tech/course-react";
import { QUIZ, COURSE, FakeProctor, QUIET_PRESENCE, sloppyAnswer, TIMING, app, dismissWalkthrough, openQuizGrid, startedRun, MATERIAL, deputisedOn, perfectAnswer, playedRun, WHOLE_RUN } from "../🧰️learner-journey-kit/🟦️.tsx";

describe("🫡️ the deputy while the proctor is away", () => {
  it("never admits an already cancelled request into its patience window", async () => {
    const proctor = new FakeProctor();
    const link = new ProctorLink(new ProctorClient(() => proctor.transport, COURSE.id), TIMING, 1000, Date.now);
    const operation = vi.fn(async () => "committed");
    const controller = new AbortController();
    const reason = new Error("the learner cancelled");
    controller.abort(reason);
    await expect(link.patient(operation, controller.signal)).rejects.toBe(reason);
    expect(operation).toHaveBeenCalledTimes(cancellation.admission.operationCalls);
  });

  for (const scenario of cancellation.scenarios) {
    it(`keeps the run and persisted answers when ${scenario.command} is cancelled ${scenario.cancelAt} while offline`, async () => {
      const proctor = new FakeProctor();
      proctor.away = true;
      const area = memoryStorageOrigin().tab();
      const session = deputisedOn(proctor, area);
      try {
        const { run, tasks } = await startedRun(session);
        session.answer(run, tasks[0]!.id, perfectAnswer(tasks[0]!));
        const before = session.outbox.queued();
        const persisted = Object.fromEntries(area.keys().map(key => [key, area.get(key)]));
        const controller = new AbortController();
        const reason = new Error("the learner cancelled");
        if (scenario.cancelAt === "before") controller.abort(reason);
        const operation = scenario.command === "start-run"
          ? session.startRun(QUIZ.id, "expert", controller.signal)
          : session.submit(run, controller.signal, phase => { if (phase.phase === scenario.cancelAt) controller.abort(reason); });
        await expect(operation).rejects.toBe(reason);
        expect(session.getSnapshot().state.runs[run]?.status).toBe(scenario.runStatus);
        expect(session.outbox.queued()).toHaveLength(before.length + scenario.addedCommands);
        expect(session.outbox.queued()).toEqual(before);
        expect(Object.fromEntries(area.keys().map(key => [key, area.get(key)]))).toEqual(persisted);
      } finally {
        session.stop();
      }
    });
  }

  it("registers, plays and scores a first visit on the device, keeps it over a reload and hands it to the proctor once it answers", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.away = true;
    const origin = memoryStorageOrigin();
    const signal = new AbortController().signal;
    const session = deputisedOn(proctor, origin.tab());
    expect(session.getSnapshot().state.course).toEqual(proctor.view);
    expect(session.getSnapshot().state.step).toEqual({ screen: "home" });
    session.readIntroduction();

    expect(await session.identify({ kind: "pseudonym", handle: " Grace   Hopper " }, signal)).toBeUndefined();
    const learner = session.getSnapshot().state.learner!;
    expect(learner.identity).toEqual({ kind: "pseudonym", handle: "Grace Hopper" });
    expect(session.getSnapshot().state.learnerView).toMatchObject({ learner: learner.id, identity: learner.identity, runs: [], badges: [], total: 0 });

    const run = await playedRun(session);
    const phases: string[] = [];
    expect(await session.submit(run, signal, (phase) => phases.push(phase.phase))).toBeUndefined();
    expect(phases).toEqual(["submitting", "results"]);
    const played = session.getSnapshot().state;
    expect(played.step).toEqual({ screen: "results", run });
    expect(played.runs[run]).toMatchObject({ learner: learner.id, status: "submitted", result: { quiz: QUIZ.id, score: 1 } });
    expect(played.learnerView).toMatchObject({ total: 200, best: { household: { challenge: "medium", score: 1, points: 200 } }, runs: [{ run, challenge: "medium", status: "submitted", score: 1, points: 200 }] });
    expect(played.learnerView?.badges.map((award) => award.badge)).toEqual(["all-done", "flawless"]);
    expect(played.awards[run]).toEqual(["all-done", "flawless"]);
    expect(played.notice).toBeUndefined();
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN]);
    expect(proctor.envelopes).toEqual([]);
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "unreachable", deputy: true, pending: 6 }));

    await session.refreshLeaderboard();
    expect(shownLeaderboard(session.getSnapshot().state)).toMatchObject({ local: true, board: { learners: 1, rows: [{ rank: 1, tag: learnerTag(learner.id), total: 200, badges: ["all-done", "flawless"] }] } });
    expect(await session.startRun(QUIZ.id, "medium", signal)).toBeUndefined();
    const again = session.getSnapshot().state.step;
    expect(again.screen === "run" && again.run !== run).toBe(true);
    session.stop();

    const reloaded = deputisedOn(proctor, origin.tab());
    expect(reloaded.getSnapshot().state.step).toEqual({ screen: "home" });
    expect(reloaded.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(reloaded.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN, "start-run"]);
    proctor.away = false;
    reloaded.reconnect();
    await waitFor(() => expect(reloaded.outbox.queued()).toEqual([]), { timeout: 10_000 });
    const known = proctor.runOf(run)!;
    expect(known).toMatchObject({ learner: learner.id, identity: learner.identity });
    expect(known.runs.map((candidate) => candidate.status)).toEqual(["submitted", "open"]);
    expect(known.runs[0]?.result?.score).toBe(1);
    expect(known.badges.map((award) => award.badge)).toEqual(["all-done", "flawless"]);
    expect(new Set(proctor.envelopes.map((envelope) => envelope.commandId)).size).toBe(7);
    await waitFor(() => expect(reloaded.getSnapshot().state.runs[run]?.submittedAt).toBe(known.runs[0]?.submittedAt));
    await waitFor(() => expect(reloaded.getSnapshot().state.learnerView?.badges.map((award) => award.at)).toEqual(known.badges.map((award) => award.at)));
    await reloaded.refreshLeaderboard();
    expect(shownLeaderboard(reloaded.getSnapshot().state)?.local).toBeUndefined();
    expect(shownLeaderboard(reloaded.getSnapshot().state)?.board.rows.map((row) => row.tag)).toEqual([learnerTag(learner.id)]);
    expect(reloaded.getSnapshot().state.notice).toBeUndefined();
    expect(reloaded.getSnapshot().connection).toMatchObject({ reachability: "reachable", pending: 0 });
    reloaded.stop();
  });

  it("leaves every decision to the proctor while it answers", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = deputisedOn(proctor, memoryStorageOrigin().tab());
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, new AbortController().signal)).toBeUndefined();
    const run = await playedRun(session);
    expect(session.outbox.queued().every((command) => command.type === "record-answer")).toBe(true);
    const phases: string[] = [];
    expect(await session.submit(run, new AbortController().signal, (phase) => phases.push(phase.phase))).toBeUndefined();
    expect(phases[0]).toBe("saving");
    expect(proctor.envelopes.map((envelope) => envelope.kind)).toEqual(["course.identify-learner", ...WHOLE_RUN.map((type) => `course.${type}`)]);
    expect(session.outbox.queued()).toEqual([]);
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", submittedAt: proctor.runOf(run)?.runs[0]?.submittedAt });
    session.stop();
  });

  it("decides a submission on the device when the proctor goes away in the middle of a run", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = deputisedOn(proctor, memoryStorageOrigin().tab());
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    const { run, tasks } = await startedRun(session);
    session.answer(run, tasks[0]!.id, perfectAnswer(tasks[0]!));
    await session.outbox.settled(run);
    proctor.away = true;
    for (const task of tasks.slice(1)) session.answer(run, task.id, perfectAnswer(task));
    expect(await session.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["record-answer", "record-answer", "submit-run"]);
    expect(proctor.runOf(run)?.runs[0]?.status).toBe("open");
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } }));
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.submittedAt).toBe(proctor.runOf(run)?.runs[0]?.submittedAt));
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("does not keep a learner waiting for a proctor that says nothing: the deputy decides once the patience is spent", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    let silent = false;
    const transport: HttpTransport = { send: (request) => (silent ? new Promise<never>((_, reject) => setTimeout(() => reject(new ProctorUnavailable("the request timed out")), 400)) : proctor.transport.send(request)) };
    const session = new CourseSession({ proctor: new ProctorClient(() => transport, COURSE.id), store: localStore(memoryStorageOrigin().tab(), COURSE.id), deputy: new Deputy(MATERIAL), patienceMs: 40, timing: TIMING });
    session.start();
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    silent = true;
    const began = Date.now();
    expect(await session.identify({ kind: "pseudonym", handle: "Patient Learner" }, new AbortController().signal)).toBeUndefined();
    expect(Date.now() - began).toBeLessThan(350);
    const run = await playedRun(session);
    expect(await session.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN]);
    expect(proctor.envelopes).toEqual([]);
    silent = false;
    await waitFor(() => expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } }), { timeout: 10_000 });
    await waitFor(() => expect(session.outbox.queued()).toEqual([]));
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("continues as the holder when the proctor already knows the pseudonym the device registered by itself", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const holder = "e".repeat(32);
    const earlier = "f".repeat(32);
    proctor.seed({ type: "identify-learner", id: newId(), learner: holder, identity: { kind: "pseudonym", handle: "Grace Hopper" } });
    proctor.seed({ type: "start-run", id: newId(), learner: holder, run: earlier, quiz: QUIZ.id, challenge: "medium", at: proctor.now });
    for (const task of runView(proctor.runOf(earlier)!, earlier, proctor.quizzes)!.sheet.tasks) proctor.seed({ type: "record-answer", id: newId(), learner: holder, run: earlier, task: task.id, answer: sloppyAnswer(task), at: proctor.now });
    proctor.seed({ type: "submit-run", id: newId(), learner: holder, run: earlier });
    proctor.away = true;
    const area = memoryStorageOrigin().tab();
    const session = deputisedOn(proctor, area);
    expect(await session.identify({ kind: "pseudonym", handle: "grace hopper" }, new AbortController().signal)).toBeUndefined();
    const provisional = session.getSnapshot().state.learner!.id;
    expect(provisional).not.toBe(holder);
    const run = await playedRun(session);
    expect(await session.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    expect(session.getSnapshot().state.learner).toEqual({ id: holder, identity: { kind: "pseudonym", handle: "Grace Hopper" } });
    expect(session.getSnapshot().state.notice).toEqual({ kind: "recalled", handle: "Grace Hopper" });
    expect(session.getSnapshot().state.step).toEqual({ screen: "results", run });
    expect(localStore(area, COURSE.id).read("learner")).toMatchObject({ id: holder });
    expect(proctor.runOf(run)?.learner).toBe(holder);
    expect(proctor.runOf(run)?.runs.map((candidate) => [candidate.run, candidate.status])).toEqual([
      [earlier, "submitted"],
      [run, "submitted"],
    ]);
    expect(proctor.envelopes.some((envelope) => envelope.target.id === provisional)).toBe(false);
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.runs.map((summary) => summary.run)).toEqual([run, earlier]));
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ learner: holder, status: "submitted", result: { score: 1 } });
    session.stop();
  });

  it("voids a run of the device that the proctor would not start, and says so", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = deputisedOn(proctor, memoryStorageOrigin().tab());
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, new AbortController().signal)).toBeUndefined();
    const learner = session.getSnapshot().state.learner!.id;
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.learner).toBe(learner));
    proctor.away = true;
    const elsewhere = "9".repeat(32);
    proctor.seed({ type: "start-run", id: newId(), learner, run: elsewhere, quiz: QUIZ.id, challenge: "medium", at: proctor.now });
    const run = await playedRun(session);
    expect(session.outbox.queued(run).map((command) => command.type)).toEqual(WHOLE_RUN.slice(0, 4));
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    expect(session.getSnapshot().state.runs[run]?.status).toBe("voided");
    expect(session.getSnapshot().state.notice).toEqual({ kind: "voided" });
    expect(session.getSnapshot().state.step).toEqual({ screen: "home" });
    expect(proctor.runOf(run)).toBeUndefined();
    expect(proctor.envelopes.filter((envelope) => envelope.kind === "course.record-answer")).toEqual([]);
    await waitFor(() => expect(openRunOf(session.getSnapshot().state, QUIZ.id)).toBe(elsewhere));
    session.stop();
  });

  it("keeps the queue of a run the deputy submitted when another tab adopts it, and both tabs deliver it once", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.away = true;
    const origin = memoryStorageOrigin();
    const left = deputisedOn(proctor, origin.tab());
    const right = deputisedOn(proctor, origin.tab());
    expect(await left.identify({ kind: "anonymous" }, new AbortController().signal)).toBeUndefined();
    const run = await playedRun(left);
    expect(await left.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    await waitFor(() => expect(right.getSnapshot().state.runs[run]?.status).toBe("submitted"));
    expect(right.getSnapshot().state.learner?.id).toBe(left.getSnapshot().state.learner?.id);
    for (const tab of [left, right]) expect(tab.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN]);
    proctor.away = false;
    left.reconnect();
    right.reconnect();
    await waitFor(() => expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } }), { timeout: 10_000 });
    await waitFor(() => {
      for (const tab of [left, right]) expect(tab.outbox.queued()).toEqual([]);
    });
    expect(proctor.runOf(run)?.runs).toHaveLength(1);
    for (const tab of [left, right]) expect(tab.getSnapshot().state.notice).toBeUndefined();
    left.stop();
    right.stop();
  });

  it("shows the course at once, says that everything is saved on the device and marks the leaderboard as this device's own", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.away = true;
    const user = userEvent.setup();
    render(app(proctor, memoryStorageOrigin().tab(), MATERIAL));
    await screen.findByRole("heading", { level: 1, name: "Kurs" });
    expect((await screen.findAllByText("Kursserver nicht erreichbar – alles wird auf diesem Gerät gespeichert")).map((element) => element.closest("[role=status]") !== null)).toEqual([false, true]);
    await dismissWalkthrough(user);
    await user.click(screen.getByRole("button", { name: "Anmelden" }));
    await screen.findByRole("dialog", { name: "Wie möchtest du erscheinen?" });
    await user.click(screen.getByRole("button", { name: "Weiter" }));
    await openQuizGrid(user, "de");
    const quizCard = await screen.findByRole("region", { name: "Haushaltsphysik" });
    await screen.findByText("Kursserver nicht erreichbar – auf diesem Gerät gespeichert, noch zu senden: 1");
    await waitFor(() => expect(document.querySelector("[data-board-local]")?.textContent).toBe("Nur dieses Gerät – die anderen erscheinen, sobald der Kursserver antwortet"));
    await user.click(within(quizCard).getByRole("button", { name: "Starten (Mittel)" }));
    await screen.findByRole("heading", { level: 1, name: "Haushaltsphysik" });
    expect(screen.queryByRole("alert")).toBeNull();
    proctor.away = false;
    await screen.findByText("Alle Antworten gespeichert", {}, { timeout: 10_000 });
    expect(proctor.envelopes.map((envelope) => envelope.kind)).toEqual(["course.identify-learner", "course.start-run"]);
    await waitFor(() => expect(document.querySelector("[data-board-local]")).toBeNull());
  });

  for (const locale of COURSE_LOCALES) {
    it(`shows before its lazily fetched material has arrived and seats the deputy once it has, in ${locale}`, { timeout: 30_000 }, async () => {
      const proctor = new FakeProctor();
      proctor.away = true;
      const text = courseText(locale);
      const arrivals: ((material: CourseMaterial) => void)[] = [];
      const requested = vi.fn<CourseMaterialSource>(() => new Promise((resolve) => arrivals.push(resolve)));
      render(
        <StrictMode>
          <CourseApp proctor="" tenant={COURSE.id} material={requested} presence={QUIET_PRESENCE} transport={() => proctor.transport} storage={memoryStorageOrigin().tab()} languages={[locale]} timing={TIMING} />
        </StrictMode>,
      );
      await screen.findByRole("heading", { level: 1, name: text("course.app.loading") });
      expect(requested).toHaveBeenCalled();
      expect(screen.queryByRole("heading", { level: 1, name: locale === "en" ? "Course" : "Kurs" })).toBeNull();
      expect(screen.queryAllByText(text("course.connection.local"))).toEqual([]);
      act(() => arrivals.forEach((arrive) => arrive(MATERIAL)));
      await screen.findByRole("heading", { level: 1, name: locale === "en" ? "Course" : "Kurs" });
      expect((await screen.findAllByText(text("course.connection.local"))).length).toBeGreaterThan(0);
    });
  }

  for (const arrival of ["before the learner signs in", "while the learner signs in", "after the learner signs in"] as const) {
    it(`registers anonymously and starts a run with the proctor as ever when the lazily fetched material arrives ${arrival}`, { timeout: 30_000 }, async () => {
      const proctor = new FakeProctor();
      const user = userEvent.setup();
      const arrivals: ((material: CourseMaterial) => void)[] = [];
      const arrive = () => act(() => arrivals.forEach((resolve) => resolve(MATERIAL)));
      render(
        <StrictMode>
          <CourseApp proctor="" tenant={COURSE.id} material={() => new Promise((resolve) => arrivals.push(resolve))} presence={QUIET_PRESENCE} transport={() => proctor.transport} storage={memoryStorageOrigin().tab()} languages={["en"]} timing={TIMING} />
        </StrictMode>,
      );
      await screen.findByRole("heading", { level: 1, name: "Course" });
      await dismissWalkthrough(user);
      if (arrival === "before the learner signs in") arrive();
      await user.click(screen.getByRole("button", { name: "Sign in" }));
      await screen.findByRole("dialog", { name: "How do you want to appear?" });
      const signing = user.click(screen.getByRole("button", { name: "Continue" }));
      if (arrival === "while the learner signs in") arrive();
      await signing;
      await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
      if (arrival === "after the learner signs in") arrive();
      await openQuizGrid(user, "en");
      const quizCard = await screen.findByRole("region", { name: "Household physics" });
      await user.click(within(quizCard).getByRole("button", { name: "Start (Medium)" }));
      await screen.findByRole("heading", { level: 1, name: "Household physics" });
      expect(screen.queryByRole("dialog")).toBeNull();
      expect(proctor.envelopes.map((envelope) => envelope.kind)).toEqual(["course.identify-learner", "course.start-run"]);
    });
  }

  it("goes on without a deputy when the lazily fetched material cannot be fetched", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    render(
      <StrictMode>
        <CourseApp proctor="" tenant={COURSE.id} material={() => Promise.reject(new Error("chunk gone"))} presence={QUIET_PRESENCE} transport={() => proctor.transport} storage={memoryStorageOrigin().tab()} languages={["en"]} timing={TIMING} />
      </StrictMode>,
    );
    await screen.findByRole("heading", { level: 1, name: "Course" });
    expect(screen.queryAllByText(courseText("en")("course.connection.local"))).toEqual([]);
  });
});

/** 🧩️ A started session with a deputy whose proctor client keeps time by `clock`, so a test decides when a proctor of
 * another contract is asked again; without `deputy` the proctor decides everything. */
function agreeingOn(proctor: FakeProctor, area: StorageArea, clock: { now: number }, deputy = true): CourseSession {
  const client = new ProctorClient(() => proctor.transport, COURSE.id, () => clock.now);
  const session = new CourseSession({ proctor: client, store: localStore(area, COURSE.id), ...(deputy ? { deputy: new Deputy(MATERIAL) } : {}), timing: TIMING });
  session.start();
  return session;
}

describe("🧩️ a proctor of another contract", () => {
  it("never sends a command or a query to a proctor of an older contract: the device decides, keeps everything and delivers it once the proctor agrees", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.contract = WIRE_VERSION - 1;
    const clock = { now: 1_760_000_000_000 };
    const signal = new AbortController().signal;
    const session = agreeingOn(proctor, memoryStorageOrigin().tab(), clock);
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "incompatible", contract: "older", deputy: true }));
    expect(connectionMessage(session.getSnapshot().connection, courseText("en"))).toEqual({ message: "Course server not reachable – everything is saved on this device", tone: "alert" });
    session.readIntroduction();
    expect(await session.identify({ kind: "pseudonym", handle: "Ada Lovelace" }, signal)).toBeUndefined();
    const run = await playedRun(session);
    expect(await session.submit(run, signal, () => undefined)).toBeUndefined();
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN]);
    await session.refreshLeaderboard();
    expect(shownLeaderboard(session.getSnapshot().state)?.local).toBe(true);
    expect(proctor.requests.every((request) => request === "GET /instance")).toBe(true);
    expect(proctor.instances).toBe(1);
    expect(session.getSnapshot().state.notice).toBeUndefined();

    proctor.contract = WIRE_VERSION;
    clock.now += AGREEMENT_RECHECK_MS;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(proctor.runOf(run)?.identity).toEqual({ kind: "pseudonym", handle: "Ada Lovelace" });
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "reachable", pending: 0 }));
    expect(session.getSnapshot().connection.contract).toBeUndefined();
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("takes a proctor replaced by one of an older contract in the middle of a run for one that is away: nothing it refuses is lost or told", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const clock = { now: 1_760_000_000_000 };
    const session = agreeingOn(proctor, memoryStorageOrigin().tab(), clock);
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    const { run, tasks } = await startedRun(session);
    session.answer(run, tasks[0]!.id, perfectAnswer(tasks[0]!));
    await session.outbox.settled(run);
    proctor.contract = WIRE_VERSION - 1;
    for (const task of tasks.slice(1)) session.answer(run, task.id, perfectAnswer(task));
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "incompatible", contract: "older" }));
    expect(await session.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["record-answer", "record-answer", "submit-run"]);
    expect(session.getSnapshot().state.notice).toBeUndefined();
    expect(proctor.runOf(run)?.runs[0]?.status).toBe("open");

    proctor.contract = WIRE_VERSION;
    clock.now += AGREEMENT_RECHECK_MS;
    session.reconnect();
    await waitFor(() => expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } }), { timeout: 10_000 });
    await waitFor(() => expect(session.outbox.queued()).toEqual([]));
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.submittedAt).toBe(proctor.runOf(run)?.runs[0]?.submittedAt));
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("waits with every answer for a proctor of a newer contract without a deputy, and says that the page is out of date", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const clock = { now: 1_760_000_000_000 };
    const session = agreeingOn(proctor, memoryStorageOrigin().tab(), clock, false);
    const { run, tasks } = await startedRun(session);
    proctor.contract = WIRE_VERSION + 1;
    for (const task of tasks) session.answer(run, task.id, perfectAnswer(task));
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "incompatible", contract: "newer", deputy: false, pending: tasks.length }));
    for (const locale of COURSE_LOCALES) expect(connectionMessage(session.getSnapshot().connection, courseText(locale)).message).toBe(courseText(locale)("course.connection.outdated"));
    expect(waitingMessage(session.getSnapshot().connection, undefined, courseText("de"))).toEqual({ message: "Diese Seite ist älter als der Kursserver – lade sie neu, dann geht alles, was auf diesem Gerät liegt, an den Server.", retry: false, reload: true });
    expect(session.getSnapshot().state.notice).toBeUndefined();
    expect(proctor.answersOf(run)).toEqual({});
    proctor.contract = WIRE_VERSION;
    clock.now += AGREEMENT_RECHECK_MS;
    session.reconnect();
    await waitFor(() => expect(Object.keys(proctor.answersOf(run))).toHaveLength(tasks.length), { timeout: 10_000 });
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "reachable", pending: 0 }));
    session.stop();
  });

  it("reads an answer that is no JSON — a network's sign-in page — as a shortage: it waits, tells nothing and loses nothing", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const clock = { now: 1_760_000_000_000 };
    const session = agreeingOn(proctor, memoryStorageOrigin().tab(), clock, false);
    const { run, tasks } = await startedRun(session);
    proctor.unreadable = true;
    session.answer(run, tasks[0]!.id, perfectAnswer(tasks[0]!));
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "unreachable", pending: 1 }));
    expect(session.getSnapshot().connection.contract).toBeUndefined();
    expect(session.getSnapshot().state.notice).toBeUndefined();
    proctor.unreadable = false;
    session.reconnect();
    await waitFor(() => expect(proctor.answersOf(run)[tasks[0]!.id]).toEqual(perfectAnswer(tasks[0]!)), { timeout: 10_000 });
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "reachable", pending: 0 }));
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("never adopts a view that is not of the contract — a sheet without its challenge — and shows the one it holds until a readable one arrives", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const clock = { now: 1_760_000_000_000 };
    const session = agreeingOn(proctor, memoryStorageOrigin().tab(), clock, false);
    const { run } = await startedRun(session);
    const held = session.getSnapshot().state.runs[run]!;
    const asked = proctor.instances;
    proctor.mangle = (view, query) => (query.type === "run" ? { ...(view as RunView), sheet: { ...(view as RunView).sheet, challenge: undefined } } : view);
    const loading = session.loadRun(run);
    await waitFor(() => expect(proctor.asked("run").length).toBeGreaterThan(2));
    expect(proctor.instances).toBeGreaterThan(asked);
    expect(session.getSnapshot().state.runs[run]).toBe(held);
    expect(() => keysHidden(session.getSnapshot().state.runs[run]!)).not.toThrow();
    proctor.mangle = undefined;
    expect((await loading)?.sheet.challenge).toBe("medium");
    expect(session.getSnapshot().connection.reachability).toBe("reachable");
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("shows a page older than the proctor as out of date with a reload, and the device goes on deciding", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.contract = WIRE_VERSION + 1;
    const user = userEvent.setup();
    render(app(proctor, memoryStorageOrigin().tab(), MATERIAL));
    await screen.findByRole("heading", { level: 1, name: "Kurs" });
    const outdated = await screen.findByRole("alert");
    expect(outdated.textContent).toContain("Diese Seite ist älter als der Kursserver");
    expect(within(outdated).getByRole("button", { name: "Seite neu laden" })).toBeTruthy();
    expect(screen.getAllByText("Seite veraltet – alles bleibt auf diesem Gerät, bis du sie neu lädst").length).toBeGreaterThan(0);
    await dismissWalkthrough(user);
    await user.click(screen.getByRole("button", { name: "Anmelden" }));
    await screen.findByRole("dialog", { name: "Wie möchtest du erscheinen?" });
    await user.click(screen.getByRole("button", { name: "Weiter" }));
    await openQuizGrid(user, "de");
    const quizCard = await screen.findByRole("region", { name: "Haushaltsphysik" });
    await user.click(within(quizCard).getByRole("button", { name: "Starten (Mittel)" }));
    await screen.findByRole("heading", { level: 1, name: "Haushaltsphysik" });
    expect(proctor.envelopes).toEqual([]);
    expect(proctor.requests.every((request) => request === "GET /instance")).toBe(true);
  });
});

describe("🧯️ a screen that fails", () => {
  it("shows what happened in place of the screen, keeps everything else and shows the screen again on request", async () => {
    let failing = true;
    function Fragile(): ReactElement {
      if (failing) throw new Error("the sheet has no challenge");
      return <h1>Run</h1>;
    }
    const user = userEvent.setup();
    const quiet = vi.spyOn(console, "error").mockImplementation(() => undefined);
    try {
      render(
        <ScreenBoundary text={courseText("en")}>
          <Fragile />
        </ScreenBoundary>,
      );
      const alert = await screen.findByRole("alert");
      expect(alert.textContent).toContain("Something went wrong while showing this screen. Everything you did is kept.");
      expect(within(alert).getByText("the sheet has no challenge")).toBeTruthy();
      expect(within(alert).getByRole("button", { name: "Reload page" })).toBeTruthy();
      failing = false;
      await user.click(within(alert).getByRole("button", { name: "Try again now" }));
      expect(await screen.findByRole("heading", { level: 1, name: "Run" })).toBeTruthy();
      expect(screen.queryByRole("alert")).toBeNull();
    } finally {
      quiet.mockRestore();
    }
  });
});
