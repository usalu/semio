/** 🚶️ The whole learner journey from the introduction through a submitted run to the leaderboard and a reload that resumes in the chosen language, several tabs, cancellation and other devices, and presence through the whole app.
 *
 * @see ../🧰️learner-journey-kit/🟦️.tsx — the shared quiz, proctor double and builders
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the client under test
 */
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeAll, describe, expect, it } from "vitest";
import { type HttpTransport } from "@semio-tech/framework-server";
import { handleActorId, learnerTag, type RecordAnswerCommand } from "@semio-tech/course";
import { DEFAULT_BOARD, ProctorClient, ProctorUnavailable, CourseApp, CourseSession, lastSubmittedRunOf, learnerName, localStore, memoryStorageOrigin, newId, openRunOf, overallLeaderboard, courseText, shownLeaderboard, type PresenceConnect, type PresenceSocket } from "@semio-tech/course-react";
import { quizStylesheet as stylesheet } from "@semio-tech/course-react/stylesheet";
import { QUIZ, COURSE, decoder, PROJECTION_LAG, FakeProctor, sloppyAnswer, seedAnonymousRival, TIMING, QUIET_PRESENCE, navbar, app, dismissWalkthrough, openQuizGrid, answerCurrentTask, sessionOn, startedRun, classification, everyItemTo } from "../🧰️learner-journey-kit/🟦️.tsx";

beforeAll(async () => {
  await Promise.all([import("@semio-tech/course-react/modules/run/.tsx"), import("@semio-tech/course-react/modules/results/.tsx")]);
}, 180_000);

describe("🚶️ learner journey", () => {
  it("goes from the introduction through a submitted run to the leaderboard and resumes after a reload", { timeout: 60_000 }, async () => {
    const proctor = new FakeProctor();
    const rival = seedAnonymousRival(proctor);
    const origin = memoryStorageOrigin();
    const user = userEvent.setup();
    render(app(proctor, origin.tab()));

    await screen.findByRole("heading", { level: 1, name: "Kurs" });
    await waitFor(() => expect(document.documentElement.lang).toBe("de"));
    await waitFor(() => expect(document.querySelector('[data-slot="introduction-info-box-title"]')?.textContent).toBe("Willkommen im Testkatalog"), { timeout: 20_000 });
    await dismissWalkthrough(user);
    await user.click(within(navbar()).getByRole("button", { name: "English" }));
    expect(screen.getByRole("heading", { level: 1, name: "Course" })).toBeTruthy();
    expect(within(navbar()).getByRole("button", { name: "English" }).getAttribute("aria-pressed")).toBe("true");
    await user.click(within(screen.getByRole("region", { name: "Settings" })).getByRole("button", { name: "Open" }));
    expect(
      within(document.querySelector<HTMLElement>('[data-layered-pane="prefs"][data-opened]')!)
        .getByRole("button", { name: "English" })
        .getAttribute("aria-pressed"),
    ).toBe("true");
    await user.click(within(navbar()).getByRole("button", { name: "Course overview" }));
    await user.click(screen.getByRole("button", { name: "Sign in" }));

    await screen.findByRole("dialog", { name: "How do you want to appear?" });
    expect(screen.getByText(/There are no passwords/u)).toBeTruthy();
    await user.click(screen.getByRole("radio", { name: "Pseudonym" }));
    await user.type(screen.getByRole("textbox", { name: "Your pseudonym" }), "  Ada   Lovelace ");
    await user.click(screen.getByRole("button", { name: "Continue" }));

    const learnerCard = await screen.findByRole("region", { name: "Ada Lovelace" });
    expect(within(learnerCard).getByText("Points: 0")).toBeTruthy();
    expect(within(learnerCard).getByText("0 of 1 quizzes played")).toBeTruthy();
    const identify = proctor.envelopes.find((envelope) => envelope.kind === "course.identify-learner")!;
    expect(identify.principal).toEqual({ kind: "anonymous" });
    expect(identify.target).toEqual({ tenant: COURSE.id, kind: "course-handle", id: handleActorId("ada lovelace") });
    expect(JSON.parse(decoder.decode(identify.payload)).identity).toEqual({ kind: "pseudonym", handle: "  Ada   Lovelace " });
    expect(within(screen.getByRole("region", { name: "Badges" })).getByText("0 of 2 badges earned")).toBeTruthy();
    await openQuizGrid(user, "en");
    const quizCard = screen.getByRole("region", { name: "Household physics" });
    expect(within(quizCard).getByText("Not attempted yet")).toBeTruthy();

    await user.click(within(quizCard).getByRole("button", { name: "Start (Medium)" }));
    await screen.findByRole("heading", { level: 1, name: "Household physics" });
    await waitFor(() => expect(proctor.lagged).toBe(2 * PROJECTION_LAG));
    expect(JSON.parse(decoder.decode(proctor.envelopes.find((envelope) => envelope.kind === "course.start-run")!.payload))).toMatchObject({ quiz: QUIZ.id, challenge: "medium" });
    expect(screen.queryByRole("alert")).toBeNull();
    expect(screen.getByText("0 of 3 tasks complete")).toBeTruthy();
    const submit = screen.getByRole("button", { name: "Submit quiz" });
    expect((submit as HTMLButtonElement).disabled).toBe(false);
    expect(submit.getAttribute("aria-disabled")).toBe("false");
    await user.click(submit);
    const early = screen.getByRole("alertdialog", { name: "Submit this quiz?" });
    expect(within(early).getByText("Not answered or incomplete, scored as missed:")).toBeTruthy();
    await user.click(within(early).getByRole("button", { name: "Keep working" }));
    expect(screen.queryByRole("alertdialog")).toBeNull();
    expect(document.activeElement).toBe(submit);

    const taskButtons = within(screen.getByRole("navigation", { name: "Tasks" })).getAllByRole("button");
    expect(taskButtons).toHaveLength(3);
    for (const [index, button] of taskButtons.entries()) {
      await user.click(button);
      expect(button.getAttribute("aria-current")).toBe("step");
      if (index === 2) proctor.outage = true;
      await answerCurrentTask(user);
    }
    const lost = await screen.findAllByText(/^Connection lost – retrying\. Answers kept on this device: \d$/u);
    expect(lost.map((element) => element.closest("[role=status]") !== null)).toEqual([false, true]);
    proctor.outage = false;
    await screen.findByText("All answers saved", {}, { timeout: 5000 });
    expect(document.querySelector("[data-connection-announcer]")?.textContent).toBe("Connection restored");
    expect(screen.getByText("3 of 3 tasks complete")).toBeTruthy();

    const answers = proctor.envelopes.filter((envelope) => envelope.kind === "course.record-answer");
    for (const envelope of answers) {
      expect(envelope.idempotencyKey).toBe(envelope.commandId);
      expect(envelope.principal.kind).toBe("user");
      expect(envelope.target.kind).toBe("course-learner");
    }
    const attempts = new Map<string, number>();
    for (const envelope of answers) attempts.set(envelope.commandId, (attempts.get(envelope.commandId) ?? 0) + 1);
    expect(Math.max(...attempts.values())).toBeGreaterThan(1);

    expect(screen.getByRole("button", { name: "Submit quiz" }).getAttribute("aria-disabled")).toBe("false");
    await user.click(screen.getByRole("button", { name: "Submit quiz" }));
    const dialog = screen.getByRole("alertdialog", { name: "Submit this quiz?" });
    expect(document.activeElement).toBe(within(dialog).getByRole("button", { name: "Submit now" }));
    expect(dialog.closest(".course-app")).not.toBeNull();
    expect(document.querySelector("#course-main")?.hasAttribute("inert")).toBe(true);
    await user.keyboard("{Escape}");
    expect(screen.queryByRole("alertdialog")).toBeNull();
    expect(document.querySelector("#course-main")?.hasAttribute("inert")).toBe(false);
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Submit quiz" }));
    await user.keyboard("{Enter}");
    expect(document.activeElement).toBe(within(screen.getByRole("alertdialog", { name: "Submit this quiz?" })).getByRole("button", { name: "Submit now" }));
    await user.click(within(screen.getByRole("alertdialog")).getByRole("button", { name: "Submit now" }));

    await screen.findByRole("heading", { level: 1, name: "Results: Household physics" });
    expect(document.title).toBe("Results: Household physics · Test course");
    expect(screen.getByText("Your score: 100%").closest("p")?.textContent).toBe("Your score: 100%");
    const newBadges = screen.getByRole("region", { name: "New badges" });
    expect(within(newBadges).getByText("All done")).toBeTruthy();
    expect(within(newBadges).getByText("Flawless")).toBeTruthy();
    const climates = screen.getByRole("region", { name: /Climates/u });
    const [answered, everyone] = within(climates).getAllByRole("table");
    const desertRow = within(answered!).getByRole("rowheader", { name: "Desert" }).closest("tr")!;
    expect(
      within(desertRow)
        .getAllByRole("cell")
        .map((cell) => cell.textContent),
    ).toEqual(["Hot and dry", "Hot and dry", "✓ Correct (100%)", "Little rain, much sun."]);
    expect(everyone!.getAttribute("aria-label")).toBe("What everyone answered: Climates");
    await waitFor(() => expect(everyone!.querySelector('[data-crowd-item="desert"] > [data-key="hot-dry"] > .sr-only')?.textContent).toBe("100% (2 of 2), your answer, correct"));
    const scored = screen.getByRole("list", { name: "How everyone scored: Household physics" });
    expect(scored.closest("figure")?.getAttribute("data-runs")).toBe("2");
    expect(within(scored).getAllByRole("listitem").at(-1)?.hasAttribute("data-own")).toBe(true);
    expect(within(climates).getByRole("list", { name: "How everyone scored: Climates" })).toBeTruthy();
    const masses = screen.getByRole("region", { name: /Masses/u });
    expect(
      within(masses)
        .getAllByRole("cell")
        .map((cell) => cell.textContent),
    ).toContain("500\u00a0kg");
    expect(within(masses).getByRole("heading", { name: "Correct order" })).toBeTruthy();
    const lamps = screen.getByRole("region", { name: /Lamps/u });
    expect(within(lamps).getByRole("heading", { name: "Power: 100%" })).toBeTruthy();

    await user.click(screen.getByRole("button", { name: "Open the leaderboard" }));
    const table = await screen.findByRole("table", { name: "Leaderboard" });
    expect(window.location.hash).toBe("#board");
    const description = screen.getByText(/^Learners with a submitted quiz, ranked by total points/u);
    expect(table.parentElement?.contains(description)).toBe(false);
    expect(table.getAttribute("aria-describedby")).toBe(description.id);
    const rows = () => within(table).getAllByRole("row").slice(1);
    await waitFor(() => expect(rows()).toHaveLength(2));
    const me = rows()[0]!;
    expect(me.getAttribute("aria-current")).toBe("true");
    expect(within(me).getByRole("rowheader").textContent).toBe("Ada Lovelace (you)");
    expect(within(rows()[1]!).getByRole("rowheader").textContent).toBe(`Anonymous #${learnerTag(rival)}`);
    await user.click(within(table).getByRole("button", { name: /^Learner/u }));
    expect(
      within(table)
        .getByRole("columnheader", { name: /Learner/u })
        .getAttribute("aria-sort"),
    ).toBe("ascending");
    await user.click(within(table).getByRole("button", { name: /^Learner/u }));
    expect(
      within(table)
        .getByRole("columnheader", { name: /Learner/u })
        .getAttribute("aria-sort"),
    ).toBe("descending");
    expect(within(rows()[0]!).getByRole("rowheader").textContent).toBe(`Anonymous #${learnerTag(rival)}`);

    const page = table.closest<HTMLElement>("[data-card='leaderboard']")!;
    const headings = () => within(screen.getByRole("table", { name: "Leaderboard" })).getAllByRole("columnheader").map((heading) => heading.textContent?.trim());
    const pressed = (group: string) =>
      within(within(page).getByRole("group", { name: group }))
        .getAllByRole("button")
        .map((button) => `${button.textContent?.trim()}${button.getAttribute("aria-pressed") === "true" ? " ✓" : ""}`);
    expect(pressed("Period")).toEqual(["Today", "This week", "This month", "All time ✓"]);
    expect(pressed("Category")).toEqual(["All ✓", expect.stringMatching(/Household physics$/u)]);
    expect(headings()).toEqual(["Rank", "Learner", "Total", "Household physics", "Badges", "Runs", "Last submission"]);
    expect(within(page).queryByText(/^Counts the quizzes submitted from /u)).toBeNull();
    await user.click(within(page).getByRole("button", { name: "Today" }));
    await within(page).findByText(/^Counts the quizzes submitted from .+ until .+\.$/u);
    expect(pressed("Period")).toEqual(["Today ✓", "This week", "This month", "All time"]);
    const today = await screen.findByRole("table", { name: "Leaderboard" });
    expect(within(today).getAllByRole("rowheader").map((name) => name.textContent)).toEqual([`Anonymous #${learnerTag(rival)}`, "Ada Lovelace (you)"]);
    expect(within(today).getByRole("columnheader", { name: /Learner/u }).getAttribute("aria-sort")).toBe("descending");
    await user.click(within(within(page).getByRole("group", { name: "Category" })).getByRole("button", { name: /Household physics$/u }));
    await waitFor(() => expect(headings()).toEqual(["Rank", "Learner", "Points", "Badges", "Runs", "Last submission"]));
    expect(pressed("Category")).toEqual(["All", expect.stringMatching(/Household physics ✓$/u)]);
    expect(proctor.asked("leaderboard").slice(-1)).toEqual([{ type: "leaderboard", period: "daily", quiz: QUIZ.id, learner: expect.any(String) }]);
    await user.click(within(page).getByRole("button", { name: "All time" }));
    await user.click(within(page).getByRole("button", { name: "All" }));
    await waitFor(() => expect(pressed("Period").concat(pressed("Category"))).toEqual(["Today", "This week", "This month", "All time ✓", "All ✓", expect.stringMatching(/Household physics$/u)]));
    expect(headings()).toEqual(["Rank", "Learner", "Total", "Household physics", "Badges", "Runs", "Last submission"]);
    await user.click(screen.getByRole("button", { name: "Course overview" }));
    expect(window.location.hash).toBe("");
    await openQuizGrid(user, "en");
    await screen.findByRole("region", { name: "Household physics" });

    cleanup();
    render(app(proctor, origin.tab()));
    await screen.findByRole("heading", { level: 1, name: "Quizzes" });
    await user.click(within(navbar()).getByRole("button", { name: "Course overview" }));
    await screen.findByRole("region", { name: "Ada Lovelace" });
    expect(screen.getByRole("heading", { level: 1, name: "Course" })).toBeTruthy();
    expect(within(screen.getByRole("region", { name: "Badges" })).getByText("2 of 2 badges earned")).toBeTruthy();
    await openQuizGrid(user, "en");
    const card = screen.getByRole("region", { name: "Household physics" });
    await within(card).findByText("Best: 200 of 200 (Medium)");
    expect(within(card).getByRole("button", { name: "Again (Medium)" })).toBeTruthy();
    expect(within(card).getByText(/^Earned here: /u).textContent).toContain("All done");
  });

  it("asks a device to identify again once the proctor does not know its learner beyond the projection grace", async () => {
    const proctor = new FakeProctor();
    const store = localStore(memoryStorageOrigin().tab(), COURSE.id);
    store.write("introduced", true);
    store.write("learner", { id: "e".repeat(32), identity: { kind: "anonymous" } });
    let clock = 0;
    const session = new CourseSession({ proctor: new ProctorClient(() => proctor.transport, COURSE.id), store, timing: TIMING, now: () => (clock += 4000) });
    expect(session.getSnapshot().state.step).toEqual({ screen: "home" });
    session.start();
    await waitFor(() => expect(session.getSnapshot().state.step).toEqual({ screen: "identity" }));
    expect(session.getSnapshot().state.notice).toEqual({ kind: "rejection", rejection: "unknown-learner" });
    expect(store.read("learner")).toBeUndefined();
    session.stop();
  });

  it("asks for the leaderboard the learner chose, shows one it holds at once and keeps the overall one fresh while another is looked at", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    seedAnonymousRival(proctor);
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    expect(await session.identify({ kind: "pseudonym", handle: "Board Chooser" }, new AbortController().signal)).toBeUndefined();
    await waitFor(() => expect(session.getSnapshot().state.course).toBeDefined());
    const state = () => session.getSnapshot().state;
    const boards = () => proctor.asked("leaderboard").map(({ period, quiz }) => (quiz === undefined ? period : `${period}/${quiz}`));
    const today = { period: "daily", quiz: QUIZ.id } as const;
    expect(state().board).toEqual(DEFAULT_BOARD);
    expect(await session.refreshLeaderboard()).toEqual({ answered: true });
    expect(boards()).toEqual(["all-time"]);
    expect(shownLeaderboard(state())?.board).toMatchObject({ period: "all-time", learners: 1, submissions: 1 });
    expect(state().submissions).toBe(1);

    session.chooseBoard(today);
    expect(state().board).toEqual(today);
    expect(shownLeaderboard(state())).toBeUndefined();
    await waitFor(() => expect(shownLeaderboard(state())?.board).toMatchObject({ ...today, learners: 1, submissions: 1 }));
    expect(shownLeaderboard(state())?.board.window).toMatchObject({ from: expect.any(Number), until: expect.any(Number) });
    await session.refreshLeaderboard();
    expect(boards()).toEqual(["all-time", `daily/${QUIZ.id}`, `daily/${QUIZ.id}`]);

    session.chooseBoard(today);
    session.chooseBoard({ period: "weekly", quiz: "no-such-quiz" });
    expect(state().board).toEqual({ period: "weekly" });
    await waitFor(() => expect(shownLeaderboard(state())?.board.period).toBe("weekly"));
    await session.refreshLeaderboard();
    expect(boards().slice(3)).toEqual(["weekly", "weekly"]);

    session.chooseBoard(DEFAULT_BOARD);
    expect(shownLeaderboard(state())?.board.period).toBe("all-time");
    session.chooseBoard(today);
    expect(shownLeaderboard(state())?.board).toMatchObject(today);
    await session.refreshLeaderboard();
    await session.refreshLeaderboard();
    expect(new Set(boards().slice(5))).toEqual(new Set(["all-time", `daily/${QUIZ.id}`]));

    seedAnonymousRival(proctor, "a".repeat(32), "b".repeat(32));
    const before = boards().length;
    expect(await session.refreshLeaderboard()).toEqual({ answered: true });
    expect(boards().slice(before)).toEqual([`daily/${QUIZ.id}`, "all-time"]);
    expect(overallLeaderboard(state())?.board).toMatchObject({ period: "all-time", learners: 2, submissions: 2 });
    expect(shownLeaderboard(state())?.board).toMatchObject({ ...today, learners: 2 });
    expect(state().submissions).toBe(2);
    const settled = boards().length;
    expect(await session.refreshLeaderboard()).toEqual({ answered: true });
    expect(boards().slice(settled)).toEqual([`daily/${QUIZ.id}`]);
    session.stop();
  });
});

describe("🚶️ several tabs, cancellation and other devices", () => {
  it("keeps both tabs' unsynced answers of one run and delivers them all once the proctor is back", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const origin = memoryStorageOrigin();
    const area = origin.tab();
    const left = sessionOn(proctor, origin.tab());
    const { run, tasks } = await startedRun(left);
    const right = sessionOn(proctor, origin.tab());
    expect(right.getSnapshot().state.runs[run]?.status).toBe("open");
    const [first, second] = tasks;
    proctor.outage = true;
    left.answer(run, first!.id, sloppyAnswer(first!));
    right.answer(run, second!.id, sloppyAnswer(second!));
    await waitFor(() => {
      for (const session of [left, right]) {
        expect(Object.keys(session.getSnapshot().state.runs[run]?.answers ?? {}).sort()).toEqual([first!.id, second!.id].sort());
        expect(session.outbox.queued(run)).toHaveLength(2);
      }
    });
    expect(area.keys().filter((key) => key.includes(".outbox/"))).toHaveLength(2);
    const reloaded = new CourseSession({ proctor: new ProctorClient(() => proctor.transport, COURSE.id), store: localStore(origin.tab(), COURSE.id), timing: TIMING });
    expect(Object.keys(reloaded.getSnapshot().state.runs[run]?.answers ?? {}).sort()).toEqual([first!.id, second!.id].sort());
    expect(reloaded.outbox.queued(run)).toHaveLength(2);
    proctor.outage = false;
    left.outbox.wake();
    right.outbox.wake();
    await waitFor(() => expect(Object.keys(proctor.answersOf(run)).sort()).toEqual([first!.id, second!.id].sort()));
    await waitFor(() => expect(area.keys().filter((key) => key.includes(".outbox/"))).toHaveLength(0));
    left.stop();
    right.stop();
  });

  it("reconciles a cancelled submission the proctor already committed and shows its results", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const { run, tasks } = await startedRun(session);
    for (const task of tasks) session.answer(run, task.id, sloppyAnswer(task));
    await session.outbox.settled(run);
    proctor.holdSubmissions();
    const controller = new AbortController();
    const submitting = session.submit(run, controller.signal, () => undefined);
    await waitFor(() => expect(proctor.submissions).toBe(1));
    controller.abort(new Error("cancelled by the learner"));
    await expect(submitting).rejects.toThrow("cancelled by the learner");
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.status).toBe("submitted"));
    expect(session.getSnapshot().state.runs[run]?.result?.quiz).toBe(QUIZ.id);
    expect(session.getSnapshot().state.step).toEqual({ screen: "results", run });
    proctor.releaseSubmissions();
    session.stop();
  });

  it("lets the proctor's run view win over delivered answers and keeps only undelivered ones on top", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const { run, tasks } = await startedRun(session);
    const learner = session.getSnapshot().state.learner!.id;
    const task = classification(tasks);
    session.answer(run, task.id, everyItemTo(task, 0));
    await session.outbox.settled(run);
    proctor.seed({ type: "record-answer", id: newId(), learner, run, task: task.id, answer: everyItemTo(task, 1), at: proctor.now });
    await session.loadRun(run);
    expect(session.getSnapshot().state.runs[run]?.answers[task.id]).toEqual(everyItemTo(task, 1));
    proctor.outage = true;
    session.answer(run, task.id, everyItemTo(task, 0));
    await session.loadRun(run);
    expect(session.getSnapshot().state.runs[run]?.answers[task.id]).toEqual(everyItemTo(task, 0));
    proctor.outage = false;
    session.stop();
  });

  it("refreshes a cached run from the proctor whenever it is opened, showing the cached view at once", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = sessionOn(proctor, memoryStorageOrigin().tab());
    const { run, tasks } = await startedRun(session);
    const learner = session.getSnapshot().state.learner!.id;
    const task = classification(tasks);
    session.open({ screen: "home" });
    proctor.seed({ type: "record-answer", id: newId(), learner, run, task: task.id, answer: everyItemTo(task, 1), at: proctor.now });
    const queried = proctor.runQueries;
    session.open({ screen: "run", run });
    expect(session.getSnapshot().state.step).toEqual({ screen: "run", run });
    expect(session.getSnapshot().state.runs[run]?.answers[task.id]).toBeUndefined();
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.answers[task.id]).toEqual(everyItemTo(task, 1)));
    expect(proctor.runQueries).toBeGreaterThan(queried);
    session.stop();
  });

  it("shows the connection state on a first visit while the proctor cannot be reached, and loads on retry", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    let up = false;
    const transport: HttpTransport = {
      send: async (request) => {
        if (!up) throw new ProctorUnavailable("connection refused");
        return proctor.transport.send(request);
      },
    };
    const user = userEvent.setup();
    render(<CourseApp proctor="" tenant={COURSE.id} presence={QUIET_PRESENCE} transport={() => transport} storage={memoryStorageOrigin().tab()} languages={["en"]} timing={{ minMs: 60_000, maxMs: 60_000 }} />);
    await screen.findByText("The course server cannot be reached right now – trying again.");
    expect(screen.getByRole("heading", { level: 1, name: "Loading…" })).toBeTruthy();
    expect(screen.getAllByText("Connection lost – retrying").map((element) => element.closest("[role=status]") !== null)).toEqual([false, true]);
    expect(screen.queryByText("All answers saved")).toBeNull();
    up = true;
    await user.click(screen.getByRole("button", { name: "Try again now" }));
    await screen.findByRole("heading", { level: 1, name: "Course" });
    expect(screen.getByText("All answers saved")).toBeTruthy();
  });

  it("offers the way to the overview while the course has not arrived yet and hides the logo from assistive technology", async () => {
    const area = memoryStorageOrigin().tab();
    const store = localStore(area, COURSE.id);
    store.write("introduced", true);
    store.write("learner", { id: "f".repeat(32), identity: { kind: "anonymous" } });
    const silent: HttpTransport = { send: () => new Promise<never>(() => undefined) };
    render(<CourseApp proctor="" tenant={COURSE.id} presence={QUIET_PRESENCE} transport={() => silent} storage={area} languages={["en"]} timing={TIMING} logo={'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1 1"><path d="M0 0h1v1z"/></svg>'} />);
    expect(within(navbar()).getByRole("button", { name: "Course overview" }).getAttribute("aria-disabled")).toBe("true");
    const brand = navbar().querySelector<HTMLElement>("[data-course-brand]")!;
    expect(brand.textContent).toBe("");
    expect(brand.querySelector("img")?.closest('[aria-hidden="true"]')).not.toBeNull();
    expect(document.title).toBe("Loading…");
  });

  it("says it is connecting — never connected — until the proctor first answers", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    let open: () => void = () => undefined;
    const opened = new Promise<void>((resolve) => {
      open = resolve;
    });
    const transport: HttpTransport = {
      send: async (request) => {
        await opened;
        return proctor.transport.send(request);
      },
    };
    render(<CourseApp proctor="" tenant={COURSE.id} presence={QUIET_PRESENCE} transport={() => transport} storage={memoryStorageOrigin().tab()} languages={["en"]} timing={TIMING} />);
    expect(await screen.findAllByText("Connecting to the course server…")).toHaveLength(2);
    expect(screen.queryByText("All answers saved")).toBeNull();
    open();
    await screen.findByRole("heading", { level: 1, name: "Course" });
    expect(screen.queryByText("Connecting to the course server…")).toBeNull();
    expect(screen.getByText("All answers saved")).toBeTruthy();
  });

  it("lets long texts wrap at phone widths instead of clipping them", () => {
    expect(stylesheet).toMatch(/\.course-app \{[^}]*overflow-wrap: break-word;[^}]*\}/u);
    expect(stylesheet).toMatch(/\.course-app :where\(p, h1, h2, h3, h4, li, label, legend, summary, figcaption, dd, dt\) \{\s*overflow-wrap: anywhere;/u);
    expect(stylesheet).toMatch(/\.course-home-entry \{[^}]*inline-size: min\(100%, 36rem\);/u);
  });

  it("closes a cached run another device submitted as soon as the learner view says so, dropping its stale answers", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const deviceA = memoryStorageOrigin();
    const handle = `Device ${newId().slice(0, 6)}`;
    localStore(deviceA.tab(), COURSE.id).write("introduced", true);
    const first = sessionOn(proctor, deviceA.tab());
    const { run, tasks } = await startedRun(first, handle);
    proctor.outage = true;
    first.answer(run, tasks[0]!.id, sloppyAnswer(tasks[0]!));
    first.stop();
    expect(first.outbox.queued(run)).toHaveLength(1);
    const deviceB = sessionOn(proctor, memoryStorageOrigin().tab());
    const signal = new AbortController().signal;
    expect(await deviceB.identify({ kind: "pseudonym", handle }, signal)).toBeUndefined();
    await deviceB.resumeRun(run, signal);
    proctor.outage = false;
    for (const task of tasks) deviceB.answer(run, task.id, sloppyAnswer(task));
    await deviceB.outbox.settled(run);
    expect(await deviceB.submit(run, signal, () => undefined)).toBeUndefined();
    deviceB.stop();
    proctor.outage = true;
    const reloaded = sessionOn(proctor, deviceA.tab());
    expect(openRunOf(reloaded.getSnapshot().state, QUIZ.id)).toBe(run);
    await waitFor(() => expect(reloaded.getSnapshot().state.runs[run]?.status).toBe("submitted"));
    expect(openRunOf(reloaded.getSnapshot().state, QUIZ.id)).toBeUndefined();
    expect(lastSubmittedRunOf(reloaded.getSnapshot().state, QUIZ.id)).toBe(run);
    expect(reloaded.outbox.queued(run)).toEqual([]);
    const learner = reloaded.getSnapshot().state.learner!.id;
    reloaded.stop();
    const cached = localStore(deviceA.tab(), COURSE.id);
    cached.put("runs", run, { ...(cached.record("runs", run) as object), status: "open" });
    const stale: RecordAnswerCommand = { type: "record-answer", id: newId(), learner, run, task: tasks[1]!.id, answer: sloppyAnswer(tasks[1]!), at: 1 };
    cached.put("outbox", stale.id, { command: stale, queuedAt: 1 });
    const painted = new CourseSession({ proctor: new ProctorClient(() => proctor.transport, COURSE.id), store: cached, timing: TIMING });
    expect(painted.getSnapshot().state.runs[run]?.status).toBe("submitted");
    expect(openRunOf(painted.getSnapshot().state, QUIZ.id)).toBeUndefined();
    expect(painted.outbox.queued()).toEqual([]);
    expect(cached.record("outbox", stale.id)).toBeUndefined();
    expect((cached.record("runs", run) as { readonly status: string }).status).toBe("submitted");
    proctor.outage = false;
    render(<CourseApp proctor="" tenant={COURSE.id} presence={QUIET_PRESENCE} transport={() => proctor.transport} storage={deviceA.tab()} languages={["en"]} timing={TIMING} />);
    await openQuizGrid(userEvent.setup(), "en");
    const card = await screen.findByRole("region", { name: "Household physics" });
    expect(within(card).queryByRole("button", { name: /^Resume/u })).toBeNull();
    expect(within(card).queryByText("In progress")).toBeNull();
    expect(within(card).getByRole("button", { name: "Again (Medium)" })).toBeTruthy();
    expect(within(card).getByRole("button", { name: "View last result" })).toBeTruthy();
  });

  it("names anonymous learners by their public tag, never by any part of their id", () => {
    const learner = newId();
    const tag = learnerTag(learner);
    expect(tag).toMatch(/^[0-9a-f]{8}$/u);
    expect(learnerName({ kind: "anonymous" }, tag, courseText("en"))).toBe(`Anonymous #${tag}`);
    expect(learnerName({ kind: "anonymous" }, tag, courseText("de"))).toBe(`Anonym #${tag}`);
    expect(learnerName({ kind: "pseudonym", handle: "Ada" }, tag, courseText("de"))).toBe("Ada");
  });
});

describe("👥️ presence through the whole app", () => {
  it("joins the roster and the page's room once identified, shows the others and follows the learner into a quiz", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const sockets: { readonly url: string; readonly sent: unknown[]; closed: boolean; readonly socket: PresenceSocket }[] = [];
    const presence: PresenceConnect = (url) => {
      const record = { url, sent: [] as unknown[], closed: false, socket: undefined as unknown as PresenceSocket };
      const socket: PresenceSocket = {
        readyState: 1,
        onopen: null,
        onmessage: null,
        onclose: null,
        onerror: null,
        send: (data) => record.sent.push(JSON.parse(data)),
        close: () => {
          record.closed = true;
        },
      };
      sockets.push(Object.assign(record, { socket }));
      return socket;
    };
    const open = (scope: string) => {
      const found = sockets.filter((entry) => !entry.closed && entry.url.split("?")[0] === `ws://localhost:3000/scopes/${encodeURIComponent(scope)}/presence/ws`);
      expect(found, sockets.map((entry) => `${entry.url} ${entry.closed ? "closed" : "open"}`).join("\n")).toHaveLength(1);
      return found[0]!;
    };
    const deliver = (entry: { readonly socket: PresenceSocket }, frame: unknown) => act(() => entry.socket.onmessage?.(new MessageEvent("message", { data: JSON.stringify(frame) })));
    const user = userEvent.setup();
    render(<CourseApp proctor="" tenant={COURSE.id} presence={presence} transport={() => proctor.transport} storage={memoryStorageOrigin().tab()} languages={["en"]} timing={TIMING} />);
    await screen.findByRole("heading", { level: 1, name: "Course" });
    expect(sockets).toEqual([]);
    await dismissWalkthrough(user);
    await user.click(screen.getByRole("button", { name: "Sign in" }));
    await user.click(await screen.findByRole("radio", { name: "Pseudonym" }));
    await user.type(screen.getByRole("textbox", { name: "Your pseudonym" }), "Ada");
    await user.click(screen.getByRole("button", { name: "Continue" }));
    await screen.findByRole("region", { name: "Ada" });

    const roster = open(COURSE.id);
    const home = open(`${COURSE.id}/home`);
    expect(roster.url.endsWith("?surface=course")).toBe(true);
    const mira = "d".repeat(32);
    await deliver(roster, {
      type: "welcome",
      session: "s-me",
      colour: 0,
      roster: [{ session: "s-mira", colour: 4, surface: "course", state: { active: true, identity: { handle: "Mira", kind: "pseudonym" }, place: { quiz: QUIZ.id, screen: "run" }, tag: learnerTag(mira) } }],
    });
    await waitFor(() => expect(roster.sent.at(-1)).toMatchObject({ type: "state", state: { identity: { kind: "pseudonym", handle: "Ada" }, place: { screen: "home" }, active: true } }));
    expect(document.querySelector("[data-presence-footer]")?.textContent).toContain("Online: 1");
    await deliver(roster, { type: "batch", entries: [{ session: "s-me", colour: 0, surface: "course", state: (roster.sent.at(-1) as { readonly state: unknown }).state }], left: [] });
    expect(document.querySelector("[data-presence-footer]")?.textContent).toContain("Online: 2");
    await openQuizGrid(user, "en");
    expect(within(screen.getByRole("region", { name: "Household physics" })).getByText("Learning now: 1")).toBeTruthy();
    await user.click(within(navbar()).getByRole("button", { name: "Course overview" }));

    await deliver(home, { type: "welcome", session: "r-me", colour: 0, roster: [] });
    const watched = ["introduction", "leaderboard", "badges", "learner", "preferences", `quiz/${QUIZ.id}`, `quiz/${QUIZ.id}/thinking`].map((room) => `${COURSE.id}/${room}`);
    await waitFor(() =>
      expect(home.sent).toEqual([
        { type: "state", state: { tag: (roster.sent.at(-1) as { readonly state: { readonly tag: string } }).state.tag, place: { screen: "home" } } },
        { type: "watch", scopes: watched, intervalMs: 250 },
      ]),
    );
    await deliver(roster, { type: "batch", entries: [{ session: "s-mira", colour: 4, surface: "course", state: { active: true, identity: { handle: "Mira", kind: "pseudonym" }, place: { screen: "leaderboard" }, tag: learnerTag(mira) } }], left: [] });
    await deliver(home, {
      type: "watched",
      scope: `${COURSE.id}/leaderboard`,
      entries: [{ session: "w-mira", colour: 4, surface: "leaderboard", state: { cursor: { anchor: "leaderboard", x: 0.5, y: 0.5 }, place: { screen: "leaderboard" }, tag: learnerTag(mira) } }],
      left: [],
      snapshot: true,
    });
    const inBoard = await waitFor(() => {
      const mark = document.querySelector<HTMLElement>(`[data-layered-pane="board"] [data-pane-peers="${COURSE.id}/leaderboard"] [data-peer="cursor"]`);
      expect(mark).not.toBeNull();
      return mark!;
    });
    expect(inBoard.textContent).toBe("Mira");
    expect(inBoard.closest("[aria-hidden]")).not.toBeNull();
    await openQuizGrid(user, "en");
    await deliver(home, { type: "watched", scope: `${COURSE.id}/quiz/${QUIZ.id}/thinking`, entries: [{ session: "t-mira", colour: 4, surface: "thinking", state: { answers: {}, tag: learnerTag(mira) } }], left: [] });
    await waitFor(() => expect(document.querySelector(`[data-layered-pane="${QUIZ.id}"] [data-crowd-gate]`)?.getAttribute("data-crowd-gate")).toBe("locked"));
    expect(document.querySelector(`[data-layered-pane="${QUIZ.id}"] [data-crowd-figure], [data-layered-pane="${QUIZ.id}"] [data-crowd-dot]`)).toBeNull();
    await deliver(home, { type: "watched", scope: `${COURSE.id}/leaderboard`, entries: [], left: ["w-mira"] });
    expect(document.querySelector("[data-pane-peers]")).toBeNull();
    await user.click(within(navbar()).getByRole("button", { name: "Course overview" }));
    await deliver(roster, { type: "batch", entries: [{ session: "s-mira", colour: 4, surface: "course", state: { active: true, identity: { handle: "Mira", kind: "pseudonym" }, place: { screen: "home" }, tag: learnerTag(mira) } }], left: [] });
    await deliver(home, { type: "batch", entries: [{ session: "r-mira", colour: 4, surface: "home", state: { cursor: { anchor: "home:learner", x: 0.5, y: 0.5 }, place: { screen: "home" }, tag: learnerTag(mira) } }], left: [] });
    const peer = await waitFor(() => {
      const mark = document.querySelector<HTMLElement>('[data-presence-layer] [data-peer="cursor"]');
      expect(mark).not.toBeNull();
      return mark!;
    });
    expect(peer.textContent).toBe("Mira");
    expect(peer.closest("[aria-hidden]")).not.toBeNull();
    await deliver(home, { type: "batch", entries: [], left: ["r-mira"] });
    expect(document.querySelector("[data-presence-layer]")).toBeNull();

    await openQuizGrid(user, "en");
    await user.click(within(screen.getByRole("region", { name: "Household physics" })).getByRole("link", { name: "Household physics" }));
    await waitFor(() => expect(home.closed).toBe(true));
    const page = open(`${COURSE.id}/quiz/${QUIZ.id}`);
    expect(page.url.endsWith("?surface=quiz")).toBe(true);
    await waitFor(() => expect((roster.sent.at(-1) as { readonly state: { readonly place: unknown } }).state.place).toEqual({ screen: "quiz", quiz: QUIZ.id }));
    fireEvent.keyDown(document, { key: "Escape" });
    await screen.findByRole("heading", { name: "Quizzes" });
    await user.click(within(navbar()).getByRole("button", { name: "Course overview" }));
    await screen.findByRole("region", { name: "Settings" });
    await waitFor(() => expect(page.closed).toBe(true));
    const back = open(`${COURSE.id}/home`);

    await user.click(within(screen.getByRole("region", { name: "Settings" })).getByRole("button", { name: "Open" }));
    await waitFor(() => expect(back.closed).toBe(true));
    await waitFor(() => expect((roster.sent.at(-1) as { readonly state: { readonly place: unknown } }).state.place).toEqual({ screen: "preferences" }));
    expect(sockets.filter((entry) => !entry.closed).map((entry) => entry.url.split("?")[0])).toEqual([`ws://localhost:3000/scopes/${COURSE.id}/presence/ws`, `ws://localhost:3000/scopes/${COURSE.id}%2Fpreferences/presence/ws`]);
    await user.click(screen.getByRole("checkbox", { name: "Show others' cursors" }));
    fireEvent.keyDown(document, { key: "Escape" });
    await openQuizGrid(user, "en");
    await screen.findByRole("region", { name: "Household physics" });
    const again = open(`${COURSE.id}/home`);
    await deliver(again, { type: "welcome", session: "r-me-2", colour: 0, roster: [{ session: "r-mira", colour: 4, surface: "home", state: { cursor: { anchor: "home:learner", x: 0.5, y: 0.5 }, tag: learnerTag(mira) } }] });
    expect(document.querySelector("[data-presence-layer]")).toBeNull();

    await user.click(within(screen.getByRole("region", { name: "Household physics" })).getByRole("button", { name: "Start (Medium)" }));
    await screen.findByRole("heading", { level: 1, name: "Household physics" });
    await waitFor(() => expect(again.closed).toBe(true));
    const room = open(`${COURSE.id}/quiz/${QUIZ.id}`);
    expect(room.url.endsWith("?surface=run")).toBe(true);
    const task = screen.getByRole("heading", { level: 2, name: /Climates|Masses|Lamps/u }).closest("section")!;
    expect(task.getAttribute("data-presence-anchor")).toMatch(/^task:(climates|masses|lamps)$/u);
    await waitFor(() => expect((roster.sent.at(-1) as { readonly state: { readonly place: unknown } }).state.place).toEqual({ screen: "run", quiz: QUIZ.id, task: task.getAttribute("data-presence-anchor")!.slice("task:".length) }));

    const thinking = open(`${COURSE.id}/quiz/${QUIZ.id}/thinking`);
    expect(thinking.url.endsWith("?surface=thinking")).toBe(true);
    const miraThinks = {
      answers: {
        climates: { kind: "classification", assignments: { desert: "hot-dry", fjord: "hot-dry" } },
        lamps: { kind: "matching", values: { power: { floodlight: 2000, led: 8 } } },
        masses: { kind: "sorting", order: ["mouse", "cat", "horse"] },
      },
      tag: learnerTag(mira),
    };
    await deliver(thinking, { type: "welcome", session: "t-me", colour: 0, roster: [{ session: "t-mira", colour: 4, surface: "thinking", state: miraThinks }] });
    await waitFor(() => expect(thinking.sent.at(-1)).toEqual({ type: "state", state: { answers: {}, tag: (roster.sent.at(-1) as { readonly state: { readonly tag: string } }).state.tag } }));
    expect(task.querySelector("[data-crowd-gate]")?.getAttribute("data-crowd-gate")).toBe("locked");
    expect(task.querySelector("[data-crowd-figure], [data-crowd-dot]")).toBeNull();
    await user.click(within(task).getByRole("button", { name: "Show it now" }));
    await waitFor(() => expect(task.querySelector<HTMLElement>('[data-crowd-figure="answers"]')?.dataset.thinkers).toBe("1"));
    expect(within(task).getByRole("button", { name: "Hide it again" })).toBeTruthy();
    const sentences = [...task.querySelectorAll("[data-crowd-item] > [data-live] > .sr-only")].map((sentence) => sentence.textContent);
    expect(sentences.length).toBeGreaterThan(0);
    for (const sentence of sentences) expect(sentence).toMatch(/^0% \(0 of 0\), thinking this now: 1$/u);
    expect(task.querySelectorAll("[data-crowd-dot]").length).toBe(sentences.length);
    await user.click(within(task).getByRole("button", { name: "Hide it again" }));
    await waitFor(() => expect(task.querySelector("[data-crowd-figure]")).toBeNull());
  });
});
