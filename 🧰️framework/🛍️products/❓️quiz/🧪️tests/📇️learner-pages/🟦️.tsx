/** 📇️ The learner's own pages: switching identity asks first — an anonymous learner is told the progress cannot be
 * recovered and has to say so explicitly, a pseudonymous or named one is told how to come back —, the runs table names
 * every column with a header cell, missing values are spoken, and a badge shows whether it is earned as an icon and in
 * words, never by colour or opacity alone. Names are computed by `dom-accessibility-api` (through `@testing-library`).
 *
 * @see https://www.w3.org/WAI/WCAG22/Understanding/error-prevention-legal-financial-data.html — confirm what cannot be undone
 * @see https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html
 */

import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { learnerTag, type CatalogView, type Identity, type LearnerView } from "@semio-tech/quiz";
import { BadgesCard, BadgesPage, LearnerCard, LearnerPage, QUIZ_LOCALES, quizText, type QuizSession, type QuizState } from "@semio-tech/quiz-react";

const LEARNER = "a".repeat(32);
const text = (en: string, de: string) => ({ en, de });

const CATALOG: CatalogView = {
  id: "pages",
  title: text("Pages catalog", "Seitenkatalog"),
  introduction: { title: text("How it works", "So geht es"), paragraphs: [text("Classify, sort and match.", "Klassifiziere, sortiere und ordne zu.")] },
  quizzes: [{ id: "heating", emoji: "🔥", title: text("Heating", "Heizen"), description: text("About heating.", "Über das Heizen."), tasks: [{ id: "u-values", kind: "matching", title: text("U-values", "U-Werte") }] }],
  badges: [
    { id: "heating-expert", emoji: "🔥", label: text("Heating expert", "Heiz-Profi"), description: text("All right in heating.", "Alles richtig beim Heizen.") },
    { id: "all-done", emoji: "🏁", label: text("All done", "Alles erledigt"), description: text("Every quiz once.", "Jedes Quiz einmal.") },
  ],
};

function state(identity: Identity): QuizState {
  const learnerView: LearnerView = {
    learner: LEARNER,
    identity,
    runs: [
      { run: "1".repeat(32), quiz: "heating", status: "open", startedAt: 10 },
      { run: "2".repeat(32), quiz: "heating", status: "submitted", score: 0.997, startedAt: 20, submittedAt: 30 },
    ],
    badges: [{ badge: "heating-expert", run: "2".repeat(32), at: 30 }],
    best: { heating: 0.997 },
    total: 99.7,
  };
  return { step: { screen: "home" }, trail: { back: [], forward: [] }, introduced: true, learner: { id: LEARNER, identity }, catalog: CATALOG, learnerView, runs: {}, awards: {}, board: { period: "all-time" }, leaderboards: {}, crowds: {}, asked: [] };
}

function stubSession() {
  return { open: vi.fn(), resumeRun: vi.fn(async () => undefined), forgetLearner: vi.fn() };
}

const ANONYMOUS: Identity = { kind: "anonymous" };
const NAMED: Identity = { kind: "pseudonym", handle: "Ada" };
const act = (): void => undefined;

describe("📇️ switching identity", () => {
  it("tells an anonymous learner that the progress cannot be recovered and forgets only on the explicit answer", async () => {
    const session = stubSession();
    const user = userEvent.setup();
    const name = `Anonymous #${learnerTag(LEARNER)}`;
    render(
      <div className="quiz-app">
        <main>
          <LearnerCard session={session as unknown as QuizSession} state={state(ANONYMOUS)} text={quizText("en")} locale="en" revealed={false} onOpen={() => undefined} />
        </main>
      </div>,
    );
    const opener = screen.getByRole("button", { name: "Switch identity" });
    await user.click(opener);
    expect(session.forgetLearner).not.toHaveBeenCalled();
    const dialog = screen.getByRole("alertdialog", { name: "Switch identity?" });
    expect(dialog.getAttribute("aria-modal")).toBe("true");
    expect(document.getElementById(dialog.getAttribute("aria-describedby") ?? "")?.textContent).toBe(
      `You are anonymous, so this browser holds the only key to your progress. If you switch, your runs, scores and badges stay on the leaderboard as “${name}”, but nobody – you included – can ever continue them.`,
    );
    expect(document.querySelector("main")?.hasAttribute("inert")).toBe(true);
    expect(document.activeElement).toBe(within(dialog).getByRole("button", { name: "Keep this identity" }));
    await user.keyboard("{Enter}");
    expect(screen.queryByRole("alertdialog")).toBeNull();
    expect(document.querySelector("main")?.hasAttribute("inert")).toBe(false);
    expect(document.activeElement).toBe(opener);
    expect(session.forgetLearner).not.toHaveBeenCalled();

    await user.click(opener);
    await user.keyboard("{Escape}");
    expect(screen.queryByRole("alertdialog")).toBeNull();
    expect(session.forgetLearner).not.toHaveBeenCalled();

    await user.click(opener);
    await user.click(within(screen.getByRole("alertdialog")).getByRole("button", { name: "Switch and give up this progress" }));
    expect(session.forgetLearner).toHaveBeenCalledTimes(1);
  });

  it("tells a pseudonymous learner how to come back, on the card and on the profile page, in both languages", async () => {
    const told = { en: "Your progress stays saved. To come back, enter “Ada” again when you are asked how you want to appear – on this or any other device.", de: "Dein Fortschritt bleibt gespeichert. Um zurückzukommen, gib „Ada“ erneut ein, wenn du gefragt wirst, wie du erscheinen möchtest – auf diesem oder einem anderen Gerät." };
    const labels = { en: { open: "Switch identity", title: "Switch identity?", keep: "Keep this identity" }, de: { open: "Identität wechseln", title: "Identität wechseln?", keep: "Identität behalten" } };
    for (const locale of QUIZ_LOCALES) {
      const session = stubSession();
      const user = userEvent.setup();
      const { unmount } = render(<LearnerPage session={session as unknown as QuizSession} state={state(NAMED)} text={quizText(locale)} locale={locale} view={{ opened: true, revealed: false }} busy={false} act={act} />);
      await user.click(screen.getByRole("button", { name: labels[locale].open }));
      const dialog = screen.getByRole("alertdialog", { name: labels[locale].title });
      expect(document.getElementById(dialog.getAttribute("aria-describedby") ?? "")?.textContent).toBe(told[locale]);
      expect(
        within(dialog)
          .getAllByRole("button")
          .map((button) => button.textContent?.trim()),
      ).toEqual([labels[locale].keep, labels[locale].open]);
      await user.tab();
      await user.tab();
      expect(dialog.contains(document.activeElement)).toBe(true);
      await user.click(within(dialog).getByRole("button", { name: labels[locale].open }));
      expect(session.forgetLearner).toHaveBeenCalledTimes(1);
      unmount();
    }
  });
});

describe("📇️ the runs table", () => {
  it("names every column with a header cell, the actions one for assistive technology only, and speaks what is missing", () => {
    render(<LearnerPage session={stubSession() as unknown as QuizSession} state={state(NAMED)} text={quizText("en")} locale="en" view={{ opened: true, revealed: false }} busy={false} act={act} />);
    const table = screen.getByRole("table", { name: "Your runs" });
    const head = within(table).getAllByRole("row")[0]!;
    expect([...head.children].map((cell) => cell.tagName)).toEqual(["TH", "TH", "TH", "TH", "TH", "TH"]);
    expect(within(head).getAllByRole("columnheader").map((cell) => cell.textContent)).toEqual(["Quiz", "Status", "Score", "Started", "Submitted", "Actions"]);
    expect(head.lastElementChild?.querySelector(".sr-only")?.textContent).toBe("Actions");
    const open = within(table).getAllByRole("row")[2]!;
    const cells = within(open).getAllByRole("cell");
    expect(cells[0]?.textContent).toBe("In progress");
    expect([...cells[1]!.children].map((part) => [part.getAttribute("aria-hidden"), part.className, part.textContent])).toEqual([
      ["true", "", "–"],
      [null, "sr-only", "No score"],
    ]);
    expect(cells[3]?.querySelector(".sr-only")?.textContent).toBe("Not submitted");
  });
});

describe("📇️ badges", () => {
  it("shows on the card whether a badge is earned as an icon and says it in words, not by colour or opacity alone", () => {
    render(<BadgesCard state={state(NAMED)} text={quizText("en")} locale="en" revealed={false} onOpen={() => undefined} />);
    const tiles = within(screen.getByRole("region", { name: "Badges" })).getAllByRole("listitem");
    expect(tiles.map((tile) => tile.querySelector(".sr-only")?.textContent)).toEqual(["Heating expert: Earned", "All done: Not yet earned"]);
    expect(tiles.map((tile) => tile.getAttribute("data-state"))).toEqual(["earned", "locked"]);
    expect(tiles.map((tile) => tile.querySelector("[data-badge-mark]")?.getAttribute("data-badge-mark"))).toEqual(["check", "lock"]);
    for (const tile of tiles) {
      expect(tile.querySelector("[data-badge-mark]")?.getAttribute("aria-hidden")).toBe("true");
      expect(tile.className).not.toMatch(/opacity-/u);
    }
    expect(tiles[0]?.className).toContain("border-solid");
    expect(tiles[1]?.className).toContain("border-dashed");
  });

  it("shows the same state on the badges page beside the words", () => {
    render(<BadgesPage state={state(NAMED)} text={quizText("en")} locale="en" view={{ opened: true, revealed: false }} />);
    const badges = within(screen.getByRole("region", { name: "Badges" })).getAllByRole("listitem");
    expect(badges.map((badge) => badge.getAttribute("data-state"))).toEqual(["earned", "locked"]);
    expect(badges.map((badge) => badge.querySelector("[data-badge-mark]")?.getAttribute("data-badge-mark"))).toEqual(["check", "lock"]);
    expect(within(badges[1]!).getByText("Not yet earned")).toBeTruthy();
    expect(within(badges[0]!).getByText(/^Earned on /u)).toBeTruthy();
  });
});
