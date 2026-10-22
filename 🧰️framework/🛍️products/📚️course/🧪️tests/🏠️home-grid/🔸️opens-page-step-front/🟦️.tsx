/** 🔸️ The opens page step front tests of the 🏠️ home overview suite. */
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { HOME_GRID_TRACKS, HomeScreen, LEADERBOARD_POLL_MS, LeaderboardPage, homeCells, homeTrackTemplate, courseText, type CourseSession, type CourseState } from "@semio-tech/course-react";
import { Home, PAGES, PREFERENCES, QUIZZES, STATE, boardOf, cardHost, fixture, held, pane, stubSession } from "../../🛠️home-grid-kit/🟦️.tsx";
import { cards, moveMouseOver, weights } from "../🟣️shared/🟦️.tsx";

/** 🔹 Registers the opens page step front tests of the 🏠️ home overview suite. */
export function registerOpensPageStepFrontTests(): void {
  it("opens the page of the step in front whatever the address says, runs nothing, and closes it with Escape back to its card's heading link", async () => {
    window.history.replaceState(null, "", "#board");
    const session = stubSession();
    render(<Home session={session} initial={{ screen: "home", page: "heating" }} />);
    await act(async () => undefined);
    expect(session.open).not.toHaveBeenCalled();
    expect(window.location.hash).toBe("#board");
    expect(pane("board").hasAttribute("inert")).toBe(true);
    expect(pane("heating").hasAttribute("inert")).toBe(false);
    expect(within(pane("heating")).getByRole("region", { name: "Quiz heating" })).toBeTruthy();
    const tasks = within(pane("heating")).getByRole("region", { name: "Tasks in this quiz" });
    expect(
      within(tasks)
        .getAllByRole("listitem")
        .map((item) => item.textContent?.replace(/\s+/gu, "")),
    ).toEqual(["1TaskSorting"]);
    expect(cards()).toEqual([]);
    fireEvent.keyDown(document, { key: "Escape" });
    await act(async () => undefined);
    expect(session.open).toHaveBeenLastCalledWith({ screen: "home", page: "quizzes" });
    expect(cards()).toHaveLength(4);
    expect(document.activeElement).toBe(within(screen.getByRole("region", { name: "Quiz heating" })).getByRole("link", { name: "Quiz heating" }));
    expect(session.startRun).not.toHaveBeenCalled();
    expect(session.resumeRun).not.toHaveBeenCalled();
  });
  it("shows a quiz's page clear behind its card when its explore control is hovered or the card is focused, and never starts or resumes a run for it", () => {
    const session = stubSession();
    render(<Home initial={{ screen: "home", page: "quizzes" }} session={session} />);
    const card = screen.getByRole("region", { name: "Quiz heating" });
    fireEvent.pointerOver(card.querySelector("[data-layered-zoom]")!, { pointerType: "mouse" });
    expect(cardHost("heating").hasAttribute("data-revealed")).toBe(true);
    expect(card.hasAttribute("data-revealed")).toBe(true);
    expect(pane("heating").querySelector('[data-page="quiz:heating"]')).not.toBeNull();
    expect(
      within(pane("heating"))
        .queryAllByRole("button", { hidden: true })
        .map((button) => button.textContent?.trim()),
    ).toEqual(["Resume (Medium)", "Show it now"]);
    fireEvent.pointerOut(card, { pointerType: "mouse", relatedTarget: document.body });
    expect(cardHost("heating").hasAttribute("data-revealed")).toBe(false);
    const link = within(screen.getByRole("region", { name: "Quiz physics" })).getByRole("link", { name: "Quiz physics" });
    act(() => link.focus());
    expect(document.activeElement).toBe(link);
    expect(cardHost("physics").hasAttribute("data-revealed")).toBe(true);
    expect(pane("physics").querySelector('[data-page="quiz:physics"]')).not.toBeNull();
    expect(session.startRun).not.toHaveBeenCalled();
    expect(session.resumeRun).not.toHaveBeenCalled();
    expect(session.open).not.toHaveBeenCalled();
  });
  it("polls the leaderboard while home shows, whatever page is opened or revealed, and asks for the crowd of every quiz", async () => {
    vi.useFakeTimers();
    vi.spyOn(Math, "random").mockReturnValue(0.5);
    const session = stubSession();
    const { unmount } = render(<Home initial={{ screen: "home", page: "quizzes" }} session={session} />);
    await vi.advanceTimersByTimeAsync(0);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(1);
    expect(session.refreshCrowd.mock.calls.map(([quiz]) => quiz)).toEqual([...QUIZZES]);
    await vi.advanceTimersByTimeAsync(LEADERBOARD_POLL_MS);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(2);
    fireEvent.pointerOver(screen.getByRole("region", { name: "Quiz heating" }).querySelector("[data-layered-zoom]")!, { pointerType: "mouse" });
    await vi.advanceTimersByTimeAsync(LEADERBOARD_POLL_MS);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(3);
    fireEvent.keyDown(window, { key: "Escape" });
    fireEvent.click(within(screen.getByRole("region", { name: "Leaderboard" })).getByRole("button", { name: "Full leaderboard" }));
    expect(pane("board").hasAttribute("data-opened")).toBe(true);
    await vi.advanceTimersByTimeAsync(LEADERBOARD_POLL_MS);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(4);
    expect(session.refreshCrowd).toHaveBeenCalledTimes(QUIZZES.length);
    unmount();
    await vi.advanceTimersByTimeAsync(3 * LEADERBOARD_POLL_MS);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(4);
    const alone = stubSession();
    render(<LeaderboardPage session={alone as unknown as CourseSession} state={STATE} text={courseText("en")} locale="en" view={{ opened: true, revealed: false }} />);
    await vi.advanceTimersByTimeAsync(3 * LEADERBOARD_POLL_MS);
    expect(alone.refreshLeaderboard).not.toHaveBeenCalled();
    expect(screen.queryByRole("status")).toBeNull();
  });
  it("asks for the crowd of every quiz again whenever the polled leaderboard says that a run was submitted, and only then", () => {
    const session = stubSession();
    const home = (state: Partial<CourseState>) => <HomeScreen session={session as unknown as CourseSession} state={{ ...STATE, ...state }} text={courseText("en")} locale="en" preferences={PREFERENCES} onPreferences={() => undefined} />;
    const { rerender } = render(home({}));
    expect(session.refreshCrowd).toHaveBeenCalledTimes(QUIZZES.length);
    rerender(home({ leaderboards: held(boardOf(8, 6), 2) }));
    rerender(home({ board: { period: "daily", quiz: "heating" }, leaderboards: { ...held(boardOf(8, 6), 2), ...held({ ...boardOf(3, 1), period: "daily", quiz: "heating", submissions: 8 }, 3) } }));
    expect(session.refreshCrowd).toHaveBeenCalledTimes(QUIZZES.length);
    rerender(home({ submissions: 9, leaderboards: held({ ...boardOf(8, 6), submissions: 9 }, 4) }));
    expect(session.refreshCrowd.mock.calls.slice(QUIZZES.length).map(([quiz]) => quiz)).toEqual([...QUIZZES]);
  });
  it("keeps every page live behind the glass, each the size of the screen in its cell of the strip, and each card fixed in the same cell of the card grid", () => {
    render(<Home session={stubSession()} />);
    const root = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview]')!;
    expect([root.dataset.mode, root.dataset.pan]).toEqual(["strip", "pointer"]);
    const strip = root.querySelector<HTMLElement>("[data-layered-strip]")!;
    expect([strip.style.width, strip.style.height, strip.style.transform]).toEqual(["300%", "200%", "translate3d(0%, 0%, 0)"]);
    expect(root.querySelector<HTMLElement>(":scope > [data-layered-veil]")!.dataset.veil).toBe("whole");
    const cells = homeCells(PAGES, "desktop");
    for (const page of PAGES) {
      expect(pane(page).querySelector("[data-page]"), page).not.toBeNull();
      expect([Number(pane(page).style.gridColumn) - 1, Number(pane(page).style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
      const cell = cardHost(page).querySelector<HTMLElement>(".course-home-cell")!;
      expect([Number(cell.style.gridColumn) - 1, Number(cell.style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
    }
    const desktop = fixture.layouts.find((layout) => layout.grid === "desktop" && layout.mode === "strip")!;
    expect(HOME_GRID_TRACKS).toEqual({ columns: desktop.columns, rows: desktop.rows });
    const overlay = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-overlay].course-home-grid')!;
    expect(weights(overlay.style.getPropertyValue("--course-home-columns"))).toEqual(desktop.columns);
    expect(weights(overlay.style.getPropertyValue("--course-home-rows"))).toEqual(desktop.rows);
    expect(homeTrackTemplate([1, 1.5, 1], 3)).toBe("minmax(0, 1fr) minmax(0, 1.5fr) minmax(0, 1fr)");
    expect(homeTrackTemplate(undefined, 2)).toBe("minmax(0, 1fr) minmax(0, 1fr)");
  });
  it("pans the pages under the glass while the mouse is between the cards, glides to the page of a hovered explore control and shows it clear, as on play", () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval", "requestAnimationFrame", "cancelAnimationFrame", "performance", "Date"] });
    const session = stubSession();
    render(<Home session={session} />);
    const strip = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-strip]')!;
    const veil = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-veil]')!;
    const resting = cards().map((card) => card.outerHTML);
    moveMouseOver(1, 1);
    expect(strip.style.transform).toBe("translate3d(-33.3333%, -50%, 0)");
    moveMouseOver(0.5, 0);
    expect(strip.style.transform).toBe("translate3d(-33.3333%, 0%, 0)");
    expect(veil.dataset.veil).toBe("whole");
    expect(cards().map((card) => card.outerHTML)).toEqual(resting);
    const card = screen.getByRole("region", { name: "Leaderboard" });
    fireEvent.pointerOver(card.querySelector("[data-layered-zoom]")!, { pointerType: "mouse" });
    expect(veil.dataset.veil).toBe("whole");
    for (let spent = 0; spent < 600; spent += 10) act(() => void vi.advanceTimersByTime(10));
    expect(strip.style.transform).toBe("translate3d(-66.6667%, 0%, 0)");
    expect([veil.dataset.veil, veil.style.visibility]).toEqual(["clear", "hidden"]);
    moveMouseOver(1, 1);
    expect(strip.style.transform).toBe("translate3d(-66.6667%, 0%, 0)");
    fireEvent.pointerOut(card, { pointerType: "mouse", relatedTarget: document.body });
    expect([veil.dataset.veil, veil.style.visibility]).toEqual(["whole", "visible"]);
    expect(strip.style.transform).toBe("translate3d(-66.6667%, 0%, 0)");
    moveMouseOver(1, 1);
    expect(strip.style.transform).toBe("translate3d(-33.3333%, -50%, 0)");
    for (const page of PAGES) expect(pane(page).hasAttribute("inert"), page).toBe(true);
    expect(session.startRun).not.toHaveBeenCalled();
    expect(session.resumeRun).not.toHaveBeenCalled();
  });
}
