/** 🔶️ The replays language agnostic hierarchy tests of the 🏠️ home overview suite. */
import { cleanup } from "@testing-library/react";
import { type LayeredMode } from "@semio-tech/ui-react/chrome";
import { type HomeGrid } from "@semio-tech/course-react";
import { produce } from "immer";
import { FollowProvider, addressPages, parentPage, pagesWithin, previewCamera } from "@semio-tech/course-react";
import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, it, vi } from "vitest";
import { learnerTag } from "@semio-tech/course";
import { HomeScreen, boardExcerpt, homeCells, homePages, courseText, type CourseSession } from "@semio-tech/course-react";
import { COURSE, Home, LEARNER, PAGES, PREFERENCES, QUIZZES, STATE, atViewport, boardOf, fixture, pane, stubSession } from "../../🛠️home-grid-kit/🟦️.tsx";
import followedLayouts from "../../../🧫️fixtures/👀️follow/🔣️.json";
import { DESKTOP_CELLS_OF_THE_REFERENCE, TABLET_CELLS_OF_THE_REFERENCE, cards } from "../🟣️harness/🟦️.tsx";

/** 🔷 Registers the replays language agnostic hierarchy tests of the 🏠️ home overview suite. */
export function registerReplaysLanguageAgnosticHierarchyTests(): void {
  it.each([375, 900])("brings each guided introduction card into the swipe viewport at %s px", (width) => {
    atViewport(width, 300);
    const props = { session: stubSession() as unknown as CourseSession, state: STATE, text: courseText("en"), locale: "en" as const, preferences: PREFERENCES, onPreferences: () => undefined };
    const { rerender } = render(<HomeScreen {...props} introductionPage="board" />);
    expect(document.querySelector('[data-grid-active] > [data-layered-overview]')?.getAttribute("data-layered-resting")).toBe("board");
    rerender(<HomeScreen {...props} introductionPage="prefs" />);
    expect(document.querySelector('[data-grid-active] > [data-layered-overview]')?.getAttribute("data-layered-resting")).toBe("prefs");
    for (const page of PAGES) expect(document.querySelector(`[data-element-alias="course.home.${page}"]`)).not.toBeNull();
  });
  for (const vector of followedLayouts.layouts) it(`mirrors the peer's ${vector.grid} ${vector.mode} layout and settled ${vector.swipe} page from a desktop observer`, () => {
    atViewport(1440, 1200);
    const captured = vi.fn();
    const noop = () => undefined;
    const session = stubSession() as unknown as CourseSession;
    const presentation = { layout: { grid: vector.grid as HomeGrid, mode: vector.mode as LayeredMode }, swipe: vector.swipe };
    render(<FollowProvider following="alice" presentation={presentation} onCamera={noop} onRevealed={noop} onSubmission={noop} onSort={noop} onTaskPage={noop} onLayout={captured} onSwipe={captured}><HomeScreen session={session} state={STATE} text={courseText("en")} locale="en" preferences={PREFERENCES} onPreferences={noop} /></FollowProvider>);
    const root = document.querySelector<HTMLElement>('[data-course-grid="root"] > [data-layered-overview]')!;
    expect(root.parentElement?.hasAttribute("data-grid-active")).toBe(true);
    expect(document.querySelectorAll("[data-grid-active]")).toHaveLength(1);
    expect(root.hasAttribute("data-interactive")).toBe(false);
    expect(root.dataset.mode).toBe(vector.mode);
    const columns = vector.grid === "tablet" ? 2 : 3;
    const rows = Math.ceil(PAGES.length / columns);
    if (vector.mode === "swipe") {
      const offset = /translate3d\(([^%]+)%, ([^%]+)%, 0\)/u.exec(root.querySelector<HTMLElement>(":scope > [data-layered-strip]")!.style.transform)!;
      expect(-Number(offset[1]) * columns / 100).toBeCloseTo(vector.column);
      expect(-Number(offset[2]) * rows / 100).toBeCloseTo(vector.row);
      const overlay = root.querySelector<HTMLElement>(":scope > [data-layered-overlay]")!;
      const cells = [...overlay.querySelectorAll<HTMLElement>(":scope > [data-layered-cell]")];
      expect(cells).toHaveLength(PAGES.length);
      expect([...new Set(cells.map((cell) => Number(cell.style.gridColumn) - 1))].sort()).toEqual(Array.from({ length: columns }, (_, index) => index));
      expect([...new Set(cells.map((cell) => Number(cell.style.gridRow) - 1))].sort()).toEqual(Array.from({ length: rows }, (_, index) => index));
    }
    expect(captured).not.toHaveBeenCalled();
  });
  it("replays language-agnostic hierarchy children, parents and addresses", () => {
    const course = { ...COURSE, quizzes: COURSE.quizzes.map((quiz, index) => ({ ...quiz, id: `q${index}` })), topics: fixture.hierarchy.topics };
    for (const vector of fixture.hierarchy.scopes) expect(pagesWithin(vector.page ?? undefined, course), vector.page ?? "root").toEqual(vector.pages);
    for (const vector of fixture.hierarchy.parents) expect(parentPage(vector.page, course), vector.page).toBe(vector.parent ?? undefined);
    expect(addressPages(course)).toEqual([...PAGES, "energy", "parametric", "q0", "q1", "q2", "q3"]);
    expect(pagesWithin("quizzes", COURSE)).toEqual(QUIZZES);
    expect(parentPage("physics", COURSE)).toBe("quizzes");
  });
  it("keeps the active grid free of an inherited camera and gives every grid behind it that pointer", () => {
    const pointer = { x: 0.25, y: 0.75 };
    const followed = { x: 1, y: 0 };
    const cases = [
      { active: true, followed: undefined, inherited: pointer, expected: undefined },
      { active: true, followed, inherited: pointer, expected: followed },
      { active: false, followed: undefined, inherited: pointer, expected: pointer },
      { active: false, followed, inherited: undefined, expected: undefined },
    ] as const;
    for (const item of cases) {
      const actual = previewCamera(item.active, item.followed, item.inherited);
      const oracle = produce({ camera: item.active ? item.followed : item.inherited }, (draft) => draft);
      expect(actual).toEqual(item.expected);
      expect(actual).toEqual(oracle.camera);
    }
  });
  it("opens nested topic grids while keeping parents mounted and empty topics as accessible placeholders", async () => {
    const session = stubSession();
    const course = { ...COURSE, topics: fixture.hierarchy.topics!.map((topic) => ({ ...topic, quizzes: topic.id === "energy" ? [...QUIZZES] : [] })) };
    render(<Home session={session} course={course} />);
    const user = userEvent.setup();
    await user.click(within(screen.getByRole("region", { name: "Quizzes" })).getByRole("button", { name: "Explore" }));
    expect(cards().map((card) => card.dataset.card)).toEqual(["topic:energy", "topic:parametric"]);
    expect(pane("quizzes").hasAttribute("data-opened")).toBe(true);
    await user.click(within(screen.getByRole("region", { name: "Energy" })).getByRole("button", { name: "Explore" }));
    expect(cards().map((card) => card.dataset.card)).toEqual(QUIZZES.map((id) => `quiz:${id}`));
    expect(pane("quizzes").hasAttribute("data-opened")).toBe(true);
    expect(pane("energy").hasAttribute("data-opened")).toBe(true);
    await user.keyboard("{Escape}");
    expect(session.open).toHaveBeenLastCalledWith({ screen: "home", page: "quizzes" });
    await user.click(within(screen.getByRole("region", { name: "Parametric Design" })).getByRole("button", { name: "Explore" }));
    expect(pane("parametric").hasAttribute("data-opened")).toBe(true);
    expect(within(pane("parametric")).getByText("Quizzes for this topic are coming soon.")).toBeTruthy();
    expect(within(pane("parametric")).queryByRole("button")).toBeNull();
    expect(session.startRun).not.toHaveBeenCalled();
  });
  for (const order of fixture.orders) {
    it(`orders the pages of a course with ${order.quizzes.length} quizzes`, () => {
      expect(homePages()).toEqual(order.pages);
    });
  }
  for (const vector of fixture.cells) {
    it(`lays the pages of ${vector.quizzes} quizzes on the ${vector.grid} strip where their cards sit`, () => {
      const order = fixture.orders.find((candidate) => candidate.quizzes.length === vector.quizzes)!;
      expect(homeCells(order.pages, vector.grid)).toEqual(vector.cells);
    });
  }
  it("keeps the strip cells of the five root pages those of the reference layout", () => {
    const pages = homePages();
    expect(homeCells(pages, "desktop")).toEqual(DESKTOP_CELLS_OF_THE_REFERENCE);
    expect(homeCells(pages, "tablet")).toEqual(TABLET_CELLS_OF_THE_REFERENCE);
  });
  for (const excerpt of fixture.excerpts) {
    it(`cuts the leaderboard excerpt ${excerpt.id}`, () => {
      const board = boardOf(excerpt.ranks, excerpt.mine);
      const cut = boardExcerpt({ ...board, rows: [...board.rows].reverse() }, excerpt.mine === null ? undefined : learnerTag(LEARNER));
      expect(cut.top.map((entry) => entry.rank)).toEqual(excerpt.top);
      expect(cut.own?.rank ?? null).toBe(excerpt.own);
    });
  }
  it("lays five named card sections in reading order over their immediate pages, each heading linking its page", () => {
    render(<Home session={stubSession()} />);
    const shown = cards();
    expect(shown.map((card) => card.getAttribute("data-card"))).toEqual(["learner", "quizzes", "board", "badges", "preferences"]);
    const names = shown.map((card) => document.getElementById(card.getAttribute("aria-labelledby") ?? "")?.textContent);
    expect(names).toEqual(["Ada", "Quizzes", "Leaderboard", "Badges", "Settings"]);
    for (const [index, card] of shown.entries()) {
      expect(screen.getByRole("region", { name: names[index]! })).toBe(card);
      expect(within(card).getByRole("link", { name: names[index]! }).getAttribute("href")).toBe(`#${PAGES[index]}`);
      expect(card.className.split(" ")).toContain("pointer-events-auto");
    }
    expect([...document.querySelectorAll('[data-grid-active] > [data-layered-overview] > [data-layered-strip] > [data-layered-pane]')].map((element) => element.getAttribute("data-layered-pane"))).toEqual(PAGES);
    for (const page of PAGES) {
      expect(pane(page).hasAttribute("inert")).toBe(true);
      expect(pane(page).getAttribute("aria-hidden")).toBe("true");
    }
    expect(screen.getAllByRole("heading", { level: 1 }).map((heading) => heading.textContent)).toEqual(["Course"]);
    expect(screen.getAllByRole("heading", { level: 2 })).toHaveLength(5);
    for (const button of screen.getAllByRole("button")) expect(button.textContent?.trim() || button.getAttribute("aria-label")).toBeTruthy();
  });
  it("offers each quiz exactly its start, resume or play-again action, and the last result only after a submission", () => {
    render(<Home initial={{ screen: "home", page: "quizzes" }} session={stubSession()} />);
    const actions = (name: string): readonly string[] =>
      within(screen.getByRole("region", { name }))
        .getAllByRole("button")
        .map((button) => button.textContent?.trim() ?? "");
    expect(actions("Quiz physics")).toEqual(["Start (Medium)"]);
    expect(actions("Quiz heating")).toEqual(["Resume (Medium)"]);
    expect(actions("Quiz cooling")).toEqual(["View last result", "Again (Medium)"]);
    expect(actions("Quiz demand")).toEqual(["View last result", "Resume (Expert)"]);
    expect(within(screen.getByRole("region", { name: "Quiz cooling" })).getByText("Best: 299.1 of 300 (Hard)")).toBeTruthy();
    expect(within(screen.getByRole("region", { name: "Quiz demand" })).getByText("Best: 50 of 100 (Easy)")).toBeTruthy();
    expect(within(screen.getByRole("region", { name: "Quiz cooling" })).getByText("Earned here: 🔥 Heating expert")).toBeTruthy();
    expect(within(screen.getByRole("region", { name: "Quiz heating" })).getByText("In progress")).toBeTruthy();
    cleanup();
    render(<Home session={stubSession()} />);
    expect(within(screen.getByRole("region", { name: "Ada" })).getByText("Rank 7 of 8")).toBeTruthy();
    expect(within(screen.getByRole("region", { name: "Badges" })).getByText("1 of 2 badges earned")).toBeTruthy();
  });
  it("shows the top five, a gap and the own row in the leaderboard card, and opens the leaderboard page in its place", async () => {
    const session = stubSession();
    render(<Home session={session} />);
    const board = screen.getByRole("region", { name: "Leaderboard" });
    const table = within(board).getByRole("table", { name: "The five highest-ranked learners and you, by total points" });
    const body = within(table).getAllByRole("row").slice(1);
    expect(body.map((line) => within(line).getAllByRole("cell")[0]?.textContent)).toEqual(["1", "2", "3", "4", "5", "7"]);
    expect(table.querySelectorAll('tbody > tr[aria-hidden="true"]')).toHaveLength(1);
    const own = body.at(-1)!;
    expect(own.getAttribute("aria-current")).toBe("true");
    expect(within(own).getByRole("rowheader").textContent).toBe("Ada (you)");
    await userEvent.setup().click(within(board).getByRole("button", { name: "Full leaderboard" }));
    expect(session.open).toHaveBeenLastCalledWith({ screen: "home", page: "board" });
    expect(window.location.hash).toBe("");
    expect(cards()).toEqual([]);
    expect(pane("board").hasAttribute("inert")).toBe(false);
    expect(screen.getByRole("table", { name: "Leaderboard" })).toBeTruthy();
    expect(session.refreshLeaderboard).toHaveBeenCalled();
  });
}
