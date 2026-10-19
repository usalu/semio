/** 🎫️ The challenge on the quiz cards and pages of the layered home. */
import { act, cleanup, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useState, type ReactElement } from "react";
import { describe, expect, it } from "vitest";
import { type Challenge, type Leaderboard, type LeaderboardRow } from "@semio-tech/course";
import { HOME_GRID_ROW_HEIGHT_PX, HomeScreen, LeaderboardPage, challengeOf, courseText, type CoursePreferences, type CourseSession } from "@semio-tech/course-react";
import { Home, LEARNER_VIEW, PREFERENCES, QUIZZES, STATE, fixture, held, pane, row, stubSession } from "../🛠️home-grid-kit/🟦️.tsx";

describe("⛰️ the challenge on the quiz cards and pages", () => {
  /** 🎛️ Home over a session double whose preferences really change, so the page's chooser follows the learner. */
  function Choosing(props: { readonly session: ReturnType<typeof stubSession>; readonly page: string; readonly challenge: Challenge; readonly remembered?: CoursePreferences["challenges"]; readonly locale?: "en" | "de"; readonly chosen?: (preferences: CoursePreferences) => void }): ReactElement {
    const [preferences, setPreferences] = useState<CoursePreferences>({ ...PREFERENCES, challenges: { ...props.remembered, [props.page]: props.challenge } });
    const locale = props.locale ?? "en";
    return (
      <HomeScreen
        session={props.session as unknown as CourseSession}
        state={{ ...STATE, step: { screen: "home", page: props.page } }}
        text={courseText(locale)}
        locale={locale}
        preferences={preferences}
        onPreferences={(next) => {
          props.chosen?.(next);
          setPreferences(next);
        }}
      />
    );
  }

  it("starts every quiz card at the challenge remembered for its own quiz, medium where none is, and resumes an open run at its own, naming both", async () => {
    const session = stubSession();
    const user = userEvent.setup();
    render(<Home initial={{ screen: "home", page: "quizzes" }} session={session} preferences={{ ...PREFERENCES, challenges: { cooling: "hard", heating: "expert" } }} />);
    const actions = (name: string): readonly string[] =>
      within(screen.getByRole("region", { name }))
        .getAllByRole("button")
        .map((button) => button.textContent?.trim() ?? "");
    expect(actions("Quiz physics")).toEqual(["Start (Medium)"]);
    expect(actions("Quiz cooling")).toEqual(["View last result", "Again (Hard)"]);
    expect(actions("Quiz heating")).toEqual(["Resume (Medium)"]);
    expect(actions("Quiz demand")).toEqual(["View last result", "Resume (Expert)"]);
    await user.click(within(screen.getByRole("region", { name: "Quiz physics" })).getByRole("button", { name: "Start (Medium)" }));
    expect(session.startRun).toHaveBeenCalledWith("physics", "medium", expect.any(AbortSignal));
    await act(async () => undefined);
    await user.click(within(screen.getByRole("region", { name: "Quiz cooling" })).getByRole("button", { name: "Again (Hard)" }));
    expect(session.startRun).toHaveBeenLastCalledWith("cooling", "hard", expect.any(AbortSignal));
    await act(async () => undefined);
    await user.click(within(screen.getByRole("region", { name: "Quiz heating" })).getByRole("button", { name: "Resume (Medium)" }));
    expect(session.resumeRun).toHaveBeenCalledWith("1".repeat(32), expect.any(AbortSignal));
    expect(session.startRun).toHaveBeenCalledTimes(2);
  });

  for (const locale of ["en", "de"] as const) {
    it(`keeps every quiz card as compact as before in ${locale}: no chooser, at most two actions of one short line and short facts`, () => {
      const longest = { ...LEARNER_VIEW, best: { cooling: { challenge: "expert" as const, score: 0.9997, points: 399.9 }, demand: { challenge: "expert" as const, score: 0.0003, points: 0.1 } } };
      render(<HomeScreen session={stubSession() as unknown as CourseSession} state={{ ...STATE, step: { screen: "home", page: "quizzes" }, learnerView: longest }} text={courseText(locale)} locale={locale} preferences={{ ...PREFERENCES, challenges: Object.fromEntries(QUIZZES.map((quiz) => [quiz, "expert" as const])) }} onPreferences={() => undefined} />);
      for (const quiz of QUIZZES) {
        const card = document.querySelector<HTMLElement>(`.course-home-grid section[data-card="quiz:${quiz}"]`)!;
        expect(card.querySelector("fieldset, input"), quiz).toBeNull();
        const actions = within(card).getAllByRole("button").map((button) => button.textContent?.trim() ?? "");
        expect(actions.length, quiz).toBeLessThanOrEqual(2);
        for (const action of actions) expect(action.length, action).toBeLessThanOrEqual(20);
        for (const fact of card.querySelectorAll("li")) expect((fact.textContent ?? "").length, fact.textContent ?? "").toBeLessThanOrEqual(34);
      }
      expect(HOME_GRID_ROW_HEIGHT_PX).toBe(fixture.heights.rowHeightPx);
    });
  }

  it("offers the four challenges on a quiz's page as radios, each saying what it asks and its most points, and remembers the choice", async () => {
    const session = stubSession();
    const chosen: CoursePreferences[] = [];
    const user = userEvent.setup();
    render(<Choosing session={session} page="physics" challenge="medium" chosen={(next) => chosen.push(next)} />);
    await act(async () => undefined);
    const group = within(pane("physics")).getByRole("group", { name: "Challenge" });
    const radios = within(group).getAllByRole("radio");
    expect(radios.map((radio) => radio.getAttribute("value"))).toEqual(["easy", "medium", "hard", "expert"]);
    expect(radios.map((radio) => (radio as HTMLInputElement).checked)).toEqual([false, true, false, false]);
    const named = ["Easy", "Medium", "Hard", "Expert"].map((name) => within(group).getByRole("radio", { name }));
    expect(named).toEqual(radios);
    const described = radios.map((radio) => document.getElementById(radio.getAttribute("aria-describedby") ?? "")?.textContent ?? "");
    expect(described.map((line) => /Most points: (\d+)$/u.exec(line)?.[1])).toEqual(["100", "200", "300", "400"]);
    expect(described[0]).toMatch(/hint/u);
    expect(described[3]).toMatch(/clock/u);
    expect(within(pane("physics")).getByRole("button", { name: "Start (Medium)" })).toBeTruthy();
    radios[3]!.focus();
    await user.keyboard(" ");
    expect(chosen.at(-1)?.challenges.physics).toBe("expert");
    expect((radios[3] as HTMLInputElement).checked).toBe(true);
    await user.click(within(group).getByRole("radio", { name: "Easy" }));
    expect(chosen.map((preferences) => preferences.challenges.physics)).toEqual(["expert", "easy"]);
    expect(chosen.at(-1)).toEqual({ ...PREFERENCES, challenges: { physics: "easy" } });
    await user.click(within(pane("physics")).getByRole("button", { name: "Start (Easy)" }));
    expect(session.startRun).toHaveBeenCalledWith("physics", "easy", expect.any(AbortSignal));
  });

  it("remembers the challenge per quiz: expert chosen on one quiz's page leaves every other quiz at its own", async () => {
    const chosen: CoursePreferences[] = [];
    const user = userEvent.setup();
    render(<Choosing session={stubSession()} page="physics" challenge="medium" remembered={{ cooling: "easy" }} chosen={(next) => chosen.push(next)} />);
    await act(async () => undefined);
    await user.click(within(within(pane("physics")).getByRole("group", { name: "Challenge" })).getByRole("radio", { name: "Expert" }));
    const remembered = chosen.at(-1)!;
    expect(remembered.challenges).toEqual({ cooling: "easy", physics: "expert" });
    expect(QUIZZES.map((quiz) => challengeOf(remembered, quiz))).toEqual(QUIZZES.map((quiz) => (quiz === "physics" ? "expert" : quiz === "cooling" ? "easy" : "medium")));
    expect(within(pane("physics")).getByRole("button", { name: "Start (Expert)" })).toBeTruthy();
  });

  it("says the chooser and its lines in German", async () => {
    render(<Choosing session={stubSession()} page="physics" challenge="hard" locale="de" />);
    await act(async () => undefined);
    const group = within(pane("physics")).getByRole("group", { name: "Herausforderung" });
    expect(within(group).getAllByRole("radio").map((radio) => radio.parentElement?.querySelector("label")?.textContent)).toEqual(["Leicht", "Mittel", "Schwer", "Experte"]);
    expect(within(group).getByRole("radio", { name: "Schwer" })).toHaveProperty("checked", true);
    expect(within(group).getByText(/Du schätzt sie\. Höchstpunktzahl: 300$/u)).toBeTruthy();
    expect(within(pane("physics")).getByRole("button", { name: "Starten (Schwer)" })).toBeTruthy();
  });

  it("asks before another challenge discards the open run, keeps it on the safe answer and starts anew only when told", async () => {
    const session = stubSession();
    const user = userEvent.setup();
    render(<Choosing session={session} page="heating" challenge="medium" />);
    await act(async () => undefined);
    const page = pane("heating");
    expect(within(page).getByText("Your open run is on Medium.")).toBeTruthy();
    expect(within(page).getAllByRole("button").map((button) => button.textContent?.trim())).toContain("Resume (Medium)");
    expect(within(page).queryByRole("button", { name: /^Start/u })).toBeNull();
    await user.click(within(page).getByRole("radio", { name: "Hard" }));
    const primary = within(page).getByRole("button", { name: "Start (Hard)" });
    expect(within(page).getByRole("button", { name: "Resume (Medium)" })).toBeTruthy();
    await user.click(primary);
    const dialog = screen.getByRole("alertdialog", { name: "Discard the open run?" });
    expect(document.getElementById(dialog.getAttribute("aria-describedby") ?? "")?.textContent).toBe("Your run on Medium is still open. Starting on Hard discards it with its answers.");
    expect(document.activeElement).toBe(within(dialog).getByRole("button", { name: "Keep the open run" }));
    await user.click(within(dialog).getByRole("button", { name: "Keep the open run" }));
    expect(screen.queryByRole("alertdialog")).toBeNull();
    expect(session.startRun).not.toHaveBeenCalled();
    await user.click(within(page).getByRole("button", { name: "Start (Hard)" }));
    await user.keyboard("{Escape}");
    expect(screen.queryByRole("alertdialog")).toBeNull();
    expect(session.startRun).not.toHaveBeenCalled();
    await user.click(within(page).getByRole("button", { name: "Start (Hard)" }));
    await user.click(within(screen.getByRole("alertdialog")).getByRole("button", { name: "Discard and start" }));
    expect(session.startRun).toHaveBeenCalledWith("heating", "hard", expect.any(AbortSignal));
    expect(session.resumeRun).not.toHaveBeenCalled();
  });

  it("asks before discarding the open run in German, with the open run's line, the dialog and both answers", async () => {
    const session = stubSession();
    const user = userEvent.setup();
    render(<Choosing session={session} page="heating" challenge="medium" locale="de" />);
    await act(async () => undefined);
    const page = pane("heating");
    expect(within(page).getByText("Dein offener Durchgang ist auf Mittel.")).toBeTruthy();
    await user.click(within(page).getByRole("radio", { name: "Schwer" }));
    await user.click(within(page).getByRole("button", { name: "Starten (Schwer)" }));
    const dialog = screen.getByRole("alertdialog", { name: "Offenen Durchgang verwerfen?" });
    expect(document.getElementById(dialog.getAttribute("aria-describedby") ?? "")?.textContent).toBe("Dein Durchgang auf Mittel ist noch offen. Wenn du auf Schwer startest, wird er mit seinen Antworten verworfen.");
    expect(within(dialog).getAllByRole("button").map((button) => button.textContent?.trim())).toEqual(expect.arrayContaining(["Offenen Durchgang behalten", "Verwerfen und starten"]));
    expect(document.activeElement).toBe(within(dialog).getByRole("button", { name: "Offenen Durchgang behalten" }));
    expect(document.body.textContent).not.toMatch(/\{\{|quiz\.[a-z]+\.[a-zA-Z]+/u);
    await user.click(within(dialog).getByRole("button", { name: "Verwerfen und starten" }));
    expect(session.startRun).toHaveBeenCalledWith("heating", "hard", expect.any(AbortSignal));
  });

  it("offers no crowd on a quiz's page while a run of it that hides the keys is open, even after a submission", async () => {
    render(<Choosing session={stubSession()} page="demand" challenge="medium" />);
    await act(async () => undefined);
    expect(within(pane("demand")).queryByRole("region", { name: "What everyone answered" })).toBeNull();
    expect(within(pane("demand")).getByText("Your open run is on Expert.")).toBeTruthy();
    cleanup();
    render(<Choosing session={stubSession()} page="cooling" challenge="medium" />);
    await act(async () => undefined);
    expect(within(pane("cooling")).getByRole("region", { name: "What everyone answered" })).toBeTruthy();
  });

  it("shows each quiz's best on the leaderboard page as points with its challenge and sorts the column by points", async () => {
    const rowOf = (rank: number, tag: string, handle: string, best: LeaderboardRow["best"]): LeaderboardRow => ({ ...row(rank, tag, handle), best });
    const board: Leaderboard = {
      period: "all-time",
      rows: [rowOf(1, "00000001", "Bo", { cooling: { challenge: "easy", score: 1, points: 100 } }), rowOf(2, "00000002", "Cy", { cooling: { challenge: "expert", score: 0.5, points: 200 } })],
      learners: 2,
      submissions: 2,
    };
    const user = userEvent.setup();
    render(<LeaderboardPage session={stubSession() as unknown as CourseSession} state={{ ...STATE, leaderboards: held(board, 1) }} text={courseText("en")} locale="en" view={{ opened: true, revealed: false }} />);
    const table = screen.getByRole("table", { name: "Leaderboard" });
    const cooling = (): readonly string[] => [...table.querySelectorAll('tbody td[data-label="Quiz cooling"]')].map((cell) => cell.textContent ?? "");
    expect(cooling()).toEqual(["100 (Easy)", "200 (Expert)"]);
    await user.click(within(table).getByRole("button", { name: "Quiz cooling" }));
    expect(within(table).getByRole("columnheader", { name: "Quiz cooling" }).getAttribute("aria-sort")).toBe("descending");
    expect(cooling()).toEqual(["200 (Expert)", "100 (Easy)"]);
  });
});
