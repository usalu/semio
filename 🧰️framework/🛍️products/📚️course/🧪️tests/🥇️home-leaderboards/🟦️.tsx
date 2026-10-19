/** 🥇️ The leaderboard of the new contract on the layered home and its four leaderboards with their categories. */
import { cleanup, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { LEADERBOARD_TOP, type Leaderboard } from "@semio-tech/course";
import { HomeScreen, LeaderboardCard, LeaderboardPage, boardKey, evolveCourseState, leaderboardColumns, nextSort, shownLeaderboard, courseText, textPresentation, type CourseSession, type CourseState } from "@semio-tech/course-react";
import { COURSE, LEARNER, PREFERENCES, QUIZZES, STATE, boardOf, fixture, held, stubSession } from "../🛠️home-grid-kit/🟦️.tsx";

describe("🏆️ the leaderboard of the new contract", () => {
  it("is sent with at most a hundred rows", () => {
    expect(fixture.board.sentRows).toBe(LEADERBOARD_TOP);
  });

  for (const vector of fixture.board.page) {
    it(`shows the page ${vector.id}: the sent rows, the own row with its real rank and the number of ranked learners`, () => {
      const board = boardOf(vector.ranks, vector.mine);
      render(<LeaderboardPage session={stubSession() as unknown as CourseSession} state={{ ...STATE, leaderboards: held(board, 1) }} text={courseText("en")} locale="en" view={{ opened: true, revealed: false }} />);
      const table = screen.getByRole("table", { name: "Leaderboard" });
      const sent = [...table.querySelectorAll("tbody:not([data-board-own]) > tr")];
      expect(sent).toHaveLength(vector.rows);
      expect(sent.map((line) => line.querySelector("td")?.textContent)).toEqual(Array.from({ length: vector.rows }, (_, index) => String(index + 1)));
      const apart = table.querySelector("tbody[data-board-own]");
      expect(apart === null ? null : Number(apart.querySelector('tr[aria-current="true"] > td')?.textContent)).toBe(vector.apart);
      if (apart !== null) {
        expect(within(apart as HTMLElement).getByRole("rowheader", { name: "Ada (you)" })).toBeTruthy();
        expect(apart.querySelector('th[scope="rowgroup"]')?.textContent).toBe("Your place");
        expect(apart.querySelector('tr[aria-hidden="true"]')).not.toBeNull();
      }
      expect(table.querySelectorAll('tr[aria-current="true"]')).toHaveLength(vector.mine === null ? 0 : 1);
      const count = document.querySelector("[data-board-count]")!;
      expect([...count.children].map((part) => part.textContent)).toEqual(vector.shown === null ? [vector.count] : [vector.count, vector.shown]);
      expect(screen.getByText("Just for fun: names are not verified, and anyone can play under any pseudonym.")).toBeTruthy();
    });
  }

  it("says the learner's real rank among all ranked learners on the learner's card, also far below the sent rows", () => {
    const session = stubSession() as unknown as CourseSession;
    render(<HomeScreen session={session} state={{ ...STATE, leaderboards: held(boardOf(250, 180), 1) }} text={courseText("en")} locale="en" preferences={PREFERENCES} onPreferences={() => undefined} />);
    expect(within(screen.getByRole("region", { name: "Ada" })).getByText("Rank 180 of 250")).toBeTruthy();
    const card = within(screen.getByRole("region", { name: "Leaderboard" })).getByRole("table");
    expect(
      within(card)
        .getAllByRole("row")
        .slice(1)
        .map((line) => within(line).queryAllByRole("cell")[0]?.textContent),
    ).toEqual(["1", "2", "3", "4", "5", "180"]);
  });

  it("keeps the very same state while a polled leaderboard says what the last one said, so nothing renders", () => {
    const first = evolveCourseState(STATE, { type: "leaderboard-loaded", leaderboard: boardOf(250, 180), at: 10 });
    expect(shownLeaderboard(first)?.at).toBe(10);
    const again = evolveCourseState(first, { type: "leaderboard-loaded", leaderboard: boardOf(250, 180), at: 20 });
    expect(again).toBe(first);
    const moved = evolveCourseState(again, { type: "leaderboard-loaded", leaderboard: boardOf(250, 179), at: 30 });
    expect(moved).not.toBe(again);
    expect(shownLeaderboard(moved)?.at).toBe(30);
    expect(shownLeaderboard(moved)?.board.own?.rank).toBe(179);
  });
});

describe("🗓️ the four leaderboards and their categories", () => {
  const TODAY = { from: Date.UTC(2026, 9, 2), until: Date.UTC(2026, 9, 3) };
  const heating: Leaderboard = { ...boardOf(3, 2), period: "daily", quiz: "heating", window: TODAY, submissions: 8 };
  const page = (state: Partial<CourseState>, session = stubSession()) => {
    render(<LeaderboardPage session={session as unknown as CourseSession} state={{ ...STATE, ...state }} text={courseText("en")} locale="en" view={{ opened: true, revealed: false }} />);
    return session;
  };
  const choices = (group: string): string[] =>
    within(screen.getByRole("group", { name: group }))
      .getAllByRole("button")
      .map((button) => `${button.textContent?.trim()}${button.getAttribute("aria-pressed") === "true" ? " ✓" : ""}`);
  const headings = (table: HTMLElement): (string | undefined)[] => within(table).getAllByRole("columnheader").map((heading) => heading.textContent?.trim());
  const ranks = (table: HTMLElement): (string | null | undefined)[] => [...table.querySelectorAll("tbody:not([data-board-own]) > tr:not([aria-hidden])")].map((line) => line.querySelector("td")?.textContent);

  it("offers the four periods and every quiz as a category on the page, the chosen ones pressed", async () => {
    const session = page({});
    expect(choices("Period")).toEqual(["Today", "This week", "This month", "All time ✓"]);
    expect(choices("Category")).toEqual(["All ✓", ...QUIZZES.map((quiz) => `${textPresentation(COURSE.quizzes.find((entry) => entry.id === quiz)!.emoji)} Quiz ${quiz}`)]);
    const user = userEvent.setup();
    await user.click(screen.getByRole("button", { name: "This week" }));
    expect(session.chooseBoard).toHaveBeenLastCalledWith({ period: "weekly" });
    await user.click(within(screen.getByRole("group", { name: "Category" })).getByRole("button", { name: /Quiz cooling$/u }));
    expect(session.chooseBoard).toHaveBeenLastCalledWith({ period: "all-time", quiz: "cooling" });
    expect(session.refreshLeaderboard).not.toHaveBeenCalled();
  });

  it("keeps the other half of the choice when one half is chosen, and drops the category with “All”", async () => {
    const session = page({ board: { period: "daily", quiz: "heating" }, leaderboards: held(heating, 1) });
    expect(choices("Period")).toEqual(["Today ✓", "This week", "This month", "All time"]);
    expect(choices("Category").filter((choice) => choice.endsWith("✓"))).toEqual([expect.stringMatching(/Quiz heating ✓$/u)]);
    const user = userEvent.setup();
    await user.click(screen.getByRole("button", { name: "This month" }));
    expect(session.chooseBoard).toHaveBeenLastCalledWith({ period: "monthly", quiz: "heating" });
    await user.click(within(screen.getByRole("group", { name: "Category" })).getByRole("button", { name: /Quiz demand$/u }));
    expect(session.chooseBoard).toHaveBeenLastCalledWith({ period: "daily", quiz: "demand" });
    await user.click(screen.getByRole("button", { name: "All" }));
    expect(session.chooseBoard).toHaveBeenLastCalledWith({ period: "daily" });
  });

  it("shows the leaderboard of one quiz with its points in place of the total and the bests, and says what it counts in the learner's time zone", () => {
    page({ board: { period: "daily", quiz: "heating" }, leaderboards: { ...STATE.leaderboards, ...held(heating, 1) } });
    const table = screen.getByRole("table", { name: "Leaderboard" });
    expect(headings(table)).toEqual(["Rank", "Learner", "Points", "Badges", "Runs", "Last submission"]);
    expect(ranks(table)).toEqual(["1", "2", "3"]);
    expect(within(table).getAllByRole("row")[1]!.querySelectorAll("td, th")).toHaveLength(6);
    const counted = document.querySelector("[data-board-window]")?.textContent ?? "";
    const instant = (at: number): string => new Intl.DateTimeFormat("en", { dateStyle: "medium", timeStyle: "short" }).format(at);
    expect(counted).toBe(`Counts the quizzes submitted from ${instant(TODAY.from)} until ${instant(TODAY.until)}.`);
    expect([...document.querySelector("[data-board-count]")!.children].map((part) => part.textContent)).toEqual(["Learners in total: 3"]);
  });

  it("shows every quiz's best on the leaderboards of every quiz and no window on the all-time one", () => {
    page({});
    expect(headings(screen.getByRole("table", { name: "Leaderboard" }))).toEqual(["Rank", "Learner", "Total", ...QUIZZES.map((quiz) => `Quiz ${quiz}`), "Badges", "Runs", "Last submission"]);
    expect(document.querySelector("[data-board-window]")).toBeNull();
    expect(leaderboardColumns(COURSE, courseText("en"), "en").map((column) => column.key)).toEqual(["rank", "learner", "total", ...QUIZZES.map((quiz) => `best:${quiz}`), "badges", "runs", "last-activity"]);
    expect(leaderboardColumns(COURSE, courseText("de"), "de", "heating").map((column) => column.label)).toEqual(["Rang", "Lernende", "Punkte", "Abzeichen", "Durchgänge", "Letzte Abgabe"]);
  });

  it("keeps the choice on the page while a leaderboard it does not hold yet is fetched, and says when one is empty", () => {
    page({ board: { period: "weekly" } });
    expect(choices("Period")).toEqual(["Today", "This week ✓", "This month", "All time"]);
    expect(screen.getByRole("status").textContent).toBe("Loading from the course server…");
    expect(screen.queryByRole("table")).toBeNull();
    cleanup();
    const week = { from: Date.UTC(2026, 8, 28), until: Date.UTC(2026, 9, 5) };
    page({ board: { period: "weekly" }, leaderboards: held({ period: "weekly", window: week, rows: [], learners: 0, submissions: 8 }, 1) });
    expect(document.querySelector("[data-board-empty]")?.textContent).toBe("No quiz submitted in this period and category yet.");
    expect(document.querySelector("[data-board-window]")).not.toBeNull();
    expect(choices("Category")).toHaveLength(QUIZZES.length + 1);
    cleanup();
    page({ leaderboards: held({ period: "all-time", rows: [], learners: 0, submissions: 0 }, 1) });
    expect(document.querySelector("[data-board-empty]")?.textContent).toBe("No submitted quizzes yet.");
  });

  it("sorts every leaderboard by the column heading chosen last, and by rank where that column is not shown", async () => {
    const session = stubSession() as unknown as CourseSession;
    const view = (state: Partial<CourseState>) => <LeaderboardPage session={session} state={{ ...STATE, ...state }} text={courseText("en")} locale="en" view={{ opened: true, revealed: false }} />;
    const { rerender } = render(view({}));
    const user = userEvent.setup();
    const table = () => screen.getByRole("table", { name: "Leaderboard" });
    const sorted = () => within(table()).getAllByRole("columnheader").flatMap((heading) => (heading.getAttribute("aria-sort") === null ? [] : [`${heading.textContent?.trim()} ${heading.getAttribute("aria-sort")}`]));
    expect(sorted()).toEqual(["Rank ascending"]);
    await user.click(within(table()).getByRole("button", { name: "Total" }));
    expect(sorted()).toEqual(["Total descending"]);
    expect(ranks(table())).toEqual(["1", "2", "3", "4", "5", "6", "7", "8"]);
    await user.click(within(table()).getByRole("button", { name: "Total" }));
    expect(sorted()).toEqual(["Total ascending"]);
    expect(ranks(table())).toEqual(["8", "7", "6", "5", "4", "3", "2", "1"]);
    rerender(view({ board: { period: "daily", quiz: "heating" }, leaderboards: held(heating, 1) }));
    expect(sorted()).toEqual(["Points ascending"]);
    expect(ranks(table())).toEqual(["3", "2", "1"]);
    rerender(view({}));
    await user.click(within(table()).getByRole("button", { name: "Quiz cooling" }));
    expect(sorted()).toEqual(["Quiz cooling descending"]);
    rerender(view({ board: { period: "daily", quiz: "heating" }, leaderboards: held(heating, 1) }));
    expect(sorted()).toEqual(["Rank ascending"]);
    expect(ranks(table())).toEqual(["1", "2", "3"]);
    await user.click(within(table()).getByRole("button", { name: "Learner" }));
    expect(sorted()).toEqual(["Learner ascending"]);
    expect(within(table()).getAllByRole("rowheader").map((name) => name.textContent)).toEqual(["Ada (you)", "Learner 1", "Learner 3"]);
  });

  it("turns the order of a column around when it is chosen again and starts every other column where more is better", () => {
    const rank = { key: "rank", direction: "ascending" } as const;
    expect(nextSort(rank, "rank")).toEqual({ key: "rank", direction: "descending" });
    expect(nextSort(nextSort(rank, "rank"), "rank")).toEqual(rank);
    expect(nextSort(rank, "learner")).toEqual({ key: "learner", direction: "ascending" });
    for (const key of ["total", "badges", "runs", "last-activity", "best:heating"] as const) expect(nextSort(rank, key)).toEqual({ key, direction: "descending" });
  });

  it("offers the periods on the card, names the category chosen on the page and sorts its excerpt by a heading", async () => {
    const session = stubSession();
    const card = (state: Partial<CourseState>) => <LeaderboardCard session={session as unknown as CourseSession} state={{ ...STATE, ...state }} text={courseText("en")} locale="en" revealed={false} onOpen={session.open} />;
    const { rerender } = render(card({}));
    const region = screen.getByRole("region", { name: "Leaderboard" });
    expect(choices("Period")).toEqual(["Today", "This week", "This month", "All time ✓"]);
    expect(within(region).queryByRole("group", { name: "Category" })).toBeNull();
    expect(region.querySelector("[data-board-category]")).toBeNull();
    const user = userEvent.setup();
    await user.click(within(region).getByRole("button", { name: "Today" }));
    expect(session.chooseBoard).toHaveBeenLastCalledWith({ period: "daily" });
    const table = within(region).getByRole("table");
    expect(headings(table)).toEqual(["Rank", "Learner", "Total", "Badges"]);
    const order = () =>
      within(table)
        .getAllByRole("row")
        .slice(1)
        .map((line) => within(line).queryAllByRole("cell")[0]?.textContent);
    expect(order()).toEqual(["1", "2", "3", "4", "5", "7"]);
    await user.click(within(table).getByRole("button", { name: "Rank" }));
    expect(order()).toEqual(["5", "4", "3", "2", "1", "7"]);
    expect(within(table).getByRole("columnheader", { name: /Rank/u }).getAttribute("aria-sort")).toBe("descending");
    expect(session.open, "a control of the card never opens its page").not.toHaveBeenCalled();
    rerender(card({ board: { period: "daily", quiz: "heating" }, leaderboards: held(heating, 1) }));
    expect(region.querySelector("[data-board-category]")?.textContent).toMatch(/^Category: .*Quiz heating$/u);
    expect(headings(within(region).getByRole("table"))).toEqual(["Rank", "Learner", "Points", "Badges"]);
    rerender(card({ board: { period: "monthly" } }));
    expect(within(region).queryByRole("table")).toBeNull();
    expect(within(region).getByText("Loading from the course server…")).toBeTruthy();
    rerender(card({ board: { period: "monthly" }, leaderboards: held({ period: "monthly", window: TODAY, rows: [], learners: 0, submissions: 8 }, 1) }));
    expect(within(region).getByText("No quiz submitted in this period and category yet.")).toBeTruthy();
  });

  it("holds the last answer of every leaderboard looked at, each under its own period and quiz", () => {
    const chosen = evolveCourseState(STATE, { type: "board-chosen", board: { period: "daily", quiz: "heating" } });
    expect(chosen.board).toEqual({ period: "daily", quiz: "heating" });
    expect(shownLeaderboard(chosen)).toBeUndefined();
    expect(evolveCourseState(chosen, { type: "board-chosen", board: { period: "daily", quiz: "heating" } })).toBe(chosen);
    const answered = evolveCourseState(chosen, { type: "leaderboard-loaded", leaderboard: heating, at: 40 });
    expect(shownLeaderboard(answered)).toEqual({ board: heating, at: 40 });
    expect(Object.keys(answered.leaderboards).sort()).toEqual(["all-time", "daily/heating"]);
    expect(answered.leaderboards["all-time"]).toBe(STATE.leaderboards["all-time"]);
    const back = evolveCourseState(answered, { type: "board-chosen", board: { period: "all-time" } });
    expect(shownLeaderboard(back)).toBe(STATE.leaderboards["all-time"]);
    expect(boardKey({ period: "weekly" })).toBe("weekly");
    expect(boardKey({ period: "monthly", quiz: "cooling" })).toBe("monthly/cooling");
  });

  it("remembers how many runs the last answer counted, whatever leaderboard it was", () => {
    expect(STATE.submissions).toBe(8);
    const same = evolveCourseState(STATE, { type: "leaderboard-loaded", leaderboard: heating, at: 40 });
    expect(same.submissions).toBe(8);
    const more = evolveCourseState(same, { type: "leaderboard-loaded", leaderboard: { ...heating, submissions: 9 }, at: 50 });
    expect([more.submissions, shownLeaderboard(more)?.board.submissions]).toEqual([9, 8]);
    const overall = evolveCourseState(more, { type: "leaderboard-loaded", leaderboard: boardOf(8, 7), at: 60 });
    expect(overall.submissions).toBe(8);
    expect(overall.leaderboards).toBe(more.leaderboards);
    expect(evolveCourseState(overall, { type: "leaderboard-loaded", leaderboard: boardOf(8, 7), at: 70 })).toBe(overall);
  });

  it("takes no quiz the course does not list as a category, and forgets every leaderboard with the learner", () => {
    expect(evolveCourseState(STATE, { type: "board-chosen", board: { period: "weekly", quiz: "plumbing" } }).board).toEqual({ period: "weekly" });
    const chosen = evolveCourseState(STATE, { type: "board-chosen", board: { period: "weekly", quiz: "demand" } });
    const shrunk = evolveCourseState(chosen, { type: "course-loaded", course: { ...COURSE, quizzes: COURSE.quizzes.filter((quiz) => quiz.id !== "demand") } });
    expect(shrunk.board).toEqual({ period: "weekly" });
    expect(evolveCourseState(chosen, { type: "course-loaded", course: COURSE }).board).toBe(chosen.board);
    expect(evolveCourseState(chosen, { type: "learner-identified", learner: LEARNER }).leaderboards).toBe(chosen.leaderboards);
    expect(evolveCourseState(chosen, { type: "learner-identified", learner: "f".repeat(32) }).leaderboards).toEqual({});
    expect(evolveCourseState(chosen, { type: "learner-forgotten" }).leaderboards).toEqual({});
    expect(evolveCourseState(chosen, { type: "learner-forgotten" }).board).toEqual({ period: "weekly", quiz: "demand" });
  });
});
