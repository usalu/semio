/** 🏠️ The layered home (design §16–§17): the pages of the overview in reading order and the cell of each (shared
 * vectors), the nine card sections over the glass with the leaderboard fifth and each heading linking its page, every
 * page live behind the cards at rest with its card in its cell, opening a page by its card or the step in front and
 * closing it with Escape (the address and the navbar's way back are the client's, `🚏️navigation`), revealing a page behind its hovered card without running any command, the leaderboard polled and the crowd of
 * every quiz asked for while home shows and again with every new submission, the others inside the pages behind the cards, the actions of every quiz card by
 * run state, and the tracks of the live grid the card layer shares at the design system's breakpoints — read from the
 * stylesheet and the overview's track lists as parsed by `lightningcss` (the CSS engine Tailwind itself runs on) as the
 * third-party oracle.
 *
 * @see ../../🧫️fixtures/🏠️home-grid/🔣️.json
 * @see ../../../../🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx — the layered overview and its hooks
 */

import { act, cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { transform } from "lightningcss";
import { useState, type ReactElement } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { UI_MOBILE_MAX_WIDTH_PX, UI_TABLET_MAX_WIDTH_PX } from "@semio-tech/ui-react/chrome";
import { LEADERBOARD_TOP, learnerTag, roomScope, type CatalogView, type CursorState, type Leaderboard, type LeaderboardRow, type LearnerView } from "@semio-tech/quiz";
import {
  EMPTY_PRESENCE_VIEW,
  Glyph,
  HOME_CHROME_HEIGHT_PX,
  HOME_GRID_ROW_HEIGHT_PX,
  HOME_GRID_TRACKS,
  HomeScreen,
  LEADERBOARD_POLL_MS,
  LeaderboardCard,
  LeaderboardPage,
  PresenceProvider,
  TEXT_SIZES,
  boardExcerpt,
  boardKey,
  evolveQuizState,
  leaderboardColumns,
  nextSort,
  shownLeaderboard,
  homeCells,
  homeGridMinHeight,
  homeLayoutQueries,
  homePages,
  homeTrackTemplate,
  quizText,
  textPresentation,
  type PeerCursor,
  type PresenceView,
  type QuizPreferences,
  type QuizSession,
  type QuizState,
  type QuizStep,
} from "@semio-tech/quiz-react";
import stylesheet from "../../🎯️targets/⚛️react/🎨️.css?raw";
import grid from "../../🧫️fixtures/🏠️home-grid/🔣️.json";

interface Fixture {
  readonly breakpoints: { readonly mobileMaxPx: number; readonly tabletMaxPx: number };
  readonly layouts: readonly { readonly width: number; readonly layout: string; readonly columns: readonly number[]; readonly rows: readonly number[] | null }[];
  readonly heights: { readonly rowHeightPx: number; readonly chromeHeightPx: number; readonly minimum: { readonly desktop: number; readonly tablet: number }; readonly vectors: readonly { readonly width: number; readonly height: number; readonly scale: number; readonly layout: string }[] };
  readonly orders: readonly { readonly quizzes: readonly string[]; readonly pages: readonly string[] }[];
  readonly cells: readonly { readonly quizzes: number; readonly layout: string; readonly cells: Readonly<Partial<Record<string, { readonly column: number; readonly row: number }>>> }[];
  readonly excerpts: readonly { readonly id: string; readonly ranks: number; readonly mine: number | null; readonly top: readonly number[]; readonly own: number | null }[];
  readonly board: { readonly sentRows: number; readonly page: readonly { readonly id: string; readonly ranks: number; readonly mine: number | null; readonly rows: number; readonly apart: number | null; readonly count: string; readonly shown: string | null }[] };
}

const fixture: Fixture = grid;
const SNOW_EMOJI = String.fromCodePoint(0x2744, 0xfe0f);
const SNOW_TEXT = String.fromCodePoint(0x2744, 0xfe0e);
const LEARNER = "a".repeat(32);
const text = (en: string, de: string) => ({ en, de });
const PREFERENCES: QuizPreferences = { theme: "system", textSize: "normal", showCursors: true, others: "submitted", animateIcons: true, pets: "calm", petsLiveliness: "calm", petsChosen: false };
const PAGES = ["learner", "physics", "intro", "heating", "board", "cooling", "badges", "demand", "prefs"];
const DESKTOP_CELLS_OF_THE_REFERENCE = {
  learner: { column: 0, row: 0 },
  q0: { column: 1, row: 0 },
  intro: { column: 2, row: 0 },
  q1: { column: 0, row: 1 },
  board: { column: 1, row: 1 },
  q2: { column: 2, row: 1 },
  badges: { column: 0, row: 2 },
  q3: { column: 1, row: 2 },
  prefs: { column: 2, row: 2 },
};
const TABLET_CELLS_OF_THE_REFERENCE = {
  learner: { column: 0, row: 0 },
  q0: { column: 1, row: 0 },
  intro: { column: 0, row: 1 },
  q1: { column: 1, row: 1 },
  board: { column: 0, row: 2 },
  q2: { column: 0, row: 3 },
  badges: { column: 1, row: 3 },
  q3: { column: 0, row: 4 },
  prefs: { column: 1, row: 4 },
};

function row(rank: number, tag: string, handle: string): LeaderboardRow {
  return { rank, tag, identity: { kind: "pseudonym", handle }, total: 1000 - rank * 10, reachedAt: rank, best: {}, badges: [], runs: 1, lastActivity: rank };
}

function rows(ranks: number, mine: number | null): readonly LeaderboardRow[] {
  return Array.from({ length: ranks }, (_, index) => (index + 1 === mine ? row(index + 1, learnerTag(LEARNER), "Ada") : row(index + 1, String(index + 1).padStart(8, "0"), `Learner ${index + 1}`)));
}

/** 🏆️ The all-time leaderboard as the proctor answers a learner of rank `mine` among `ranks` ranked learners: the top
 * rows, the counts, and the own row when ranked. */
function boardOf(ranks: number, mine: number | null): Leaderboard {
  const all = rows(ranks, mine);
  const own = mine === null ? undefined : all[mine - 1];
  return { period: "all-time", rows: all.slice(0, LEADERBOARD_TOP), learners: ranks, submissions: ranks, ...(own === undefined ? {} : { own }) };
}

/** 🗃️ The leaderboards a client holds once the proctor answered `board`. */
function held(board: Leaderboard, at: number): QuizState["leaderboards"] {
  return { [boardKey(board)]: { board, at } };
}

//#region 🏗️State
const QUIZZES = ["physics", "heating", "cooling", "demand"] as const;

const CATALOG: CatalogView = {
  id: "grid",
  title: text("Grid catalog", "Rasterkatalog"),
  introduction: { title: text("How the quizzes work", "So funktionieren die Quizze"), paragraphs: [text("Classify, sort and match.", "Klassifiziere, sortiere und ordne zu.")] },
  quizzes: QUIZZES.map((id, index) => ({
    id,
    emoji: ["🧲", "🔥", SNOW_EMOJI, "📊"][index]!,
    title: text(`Quiz ${id}`, `Quiz ${id}`),
    description: text(`About ${id}.`, `Über ${id}.`),
    tasks: [{ id: `${id}-task`, kind: "sorting", title: text("Task", "Aufgabe") }],
  })),
  badges: [
    { id: "heating-expert", emoji: "🔥", label: text("Heating expert", "Heizprofi"), description: text("All right in heating.", "Alles richtig beim Heizen.") },
    { id: "all-done", emoji: "🏁", label: text("All done", "Alles erledigt"), description: text("Every quiz once.", "Jedes Quiz einmal.") },
  ],
};

const LEARNER_VIEW: LearnerView = {
  learner: LEARNER,
  identity: { kind: "pseudonym", handle: "Ada" },
  runs: [
    { run: "1".repeat(32), quiz: "heating", status: "open", startedAt: 10 },
    { run: "2".repeat(32), quiz: "cooling", status: "submitted", score: 0.997, startedAt: 20, submittedAt: 30 },
    { run: "3".repeat(32), quiz: "demand", status: "submitted", score: 0.5, startedAt: 40, submittedAt: 50 },
    { run: "4".repeat(32), quiz: "demand", status: "open", startedAt: 60 },
  ],
  badges: [{ badge: "heating-expert", run: "2".repeat(32), at: 30 }],
  best: { cooling: 0.997, demand: 0.5 },
  total: 149.7,
};

const STATE: QuizState = {
  step: { screen: "home" },
  trail: { back: [], forward: [] },
  introduced: true,
  learner: { id: LEARNER, identity: { kind: "pseudonym", handle: "Ada" } },
  catalog: CATALOG,
  learnerView: LEARNER_VIEW,
  runs: {},
  awards: {},
  crowds: {},
  asked: [],
  board: { period: "all-time" },
  leaderboards: held(boardOf(8, 7), Date.UTC(2026, 8, 29, 12, 0)),
  submissions: 8,
};

function stubSession() {
  return {
    open: vi.fn(),
    chooseBoard: vi.fn(),
    startRun: vi.fn(async () => undefined),
    resumeRun: vi.fn(async () => undefined),
    forgetLearner: vi.fn(),
    refreshLeaderboard: vi.fn(async () => ({ answered: true as const })),
    refreshCrowd: vi.fn(async (quiz: string) => void quiz),
    loadRun: vi.fn(async () => undefined),
  };
}
//#endregion 🏗️State

//#region 🎨️Tracks
/** 🎨️ One declaration of the card layer or its cells in the stylesheet, with the minimum viewport width its media query
 * asks for (0 outside one): a track list as its track count, a variable as `var(--name)`, anything else as its name. */
interface LayerRule {
  readonly minWidth: number;
  readonly selector: string;
  readonly property: string;
  readonly value: number | string;
}

type Declaration = {
  readonly property: string;
  readonly value: { readonly items?: readonly unknown[]; readonly propertyId?: { readonly property: string }; readonly value?: readonly { readonly type: string; readonly value?: { readonly name?: { readonly ident: string } } }[] };
};

const SPACING = ["gap", "row-gap", "column-gap", "padding", "padding-top", "padding-right", "padding-bottom", "padding-left", "padding-inline", "padding-block", "margin"];

function selectorText(selector: readonly { readonly type: string; readonly name?: string; readonly value?: string; readonly operation?: { readonly value: string } }[]): string {
  return selector.map((part) => (part.type === "class" ? `.${part.name}` : part.type === "combinator" ? ">" : part.type === "attribute" ? `[${part.name}=${part.operation?.value}]` : part.type)).join("");
}

function declared(declaration: Declaration): LayerRule["value"] {
  const variable = Array.isArray(declaration.value.value) ? declaration.value.value.find((token) => token.type === "var")?.value?.name?.ident : undefined;
  if (declaration.property === "unparsed" && variable !== undefined) return `var(${variable})`;
  return declaration.value.items?.length ?? declaration.property;
}

/** 🎨️ Every declaration of `.quiz-home-grid` and `.quiz-home-cell` with the minimum viewport width it applies from. */
function layerRules(css: string): readonly LayerRule[] {
  const found: LayerRule[] = [];
  const collect = (rules: readonly { readonly type: string; readonly value: never }[], minWidth: number): void => {
    for (const rule of rules as readonly {
      readonly type: string;
      readonly value: { readonly selectors?: readonly never[][]; readonly declarations?: { readonly declarations: readonly Declaration[] }; readonly query?: never; readonly rules?: never[] };
    }[]) {
      if (rule.type === "media") {
        const query = rule.value.query as unknown as {
          readonly mediaQueries: readonly { readonly condition: { readonly value: { readonly name: string; readonly operator: string; readonly value: { readonly value: { readonly value: { readonly unit: string; readonly value: number } } } } } }[];
        };
        const feature = query.mediaQueries[0]?.condition.value;
        if (feature?.name === "width") {
          if (feature.operator !== "greater-than-equal" || feature.value.value.value.unit !== "px") throw new Error(`unexpected width query ${JSON.stringify(query)}`);
          collect(rule.value.rules ?? [], feature.value.value.value.value);
        } else {
          const before = found.length;
          collect(rule.value.rules ?? [], Number.POSITIVE_INFINITY);
          if (found.length !== before) throw new Error(`card layer rules under a media query that is not about width: ${JSON.stringify(query)}`);
        }
        continue;
      }
      if (rule.type !== "style") continue;
      for (const selector of rule.value.selectors ?? []) {
        const name = selectorText(selector);
        if (!name.startsWith(".quiz-home-grid") && !name.startsWith(".quiz-home-cell")) continue;
        for (const declaration of rule.value.declarations?.declarations ?? []) found.push({ minWidth, selector: name, property: declaration.value.propertyId?.property ?? declaration.property, value: declared(declaration) });
      }
    }
  };
  transform({
    filename: "🎨️.css",
    code: new TextEncoder().encode(css),
    visitor: {
      StyleSheet(sheet) {
        collect(sheet.rules as never, 0);
      },
    },
  });
  return found;
}

/** 🔳️ The tracks the card layer takes at `width`: counted, or the overview's variables. */
function layerAt(rules: readonly LayerRule[], width: number): Readonly<Record<string, number | string>> {
  const tracks: Record<string, number | string> = {};
  for (const rule of rules) {
    if (width < rule.minWidth || rule.selector !== ".quiz-home-grid") continue;
    if (rule.property === "grid-template-columns") tracks.columns = rule.value;
    if (rule.property === "grid-template-rows") tracks.rows = rule.value;
  }
  return tracks;
}

/** ⚖️ The fr weights of a CSS track list, as parsed by lightningcss (which keeps them as 32-bit floats, hence the
 * rounding to 10⁻⁶). */
function weights(template: string): readonly number[] {
  const found: number[] = [];
  transform({
    filename: "tracks.css",
    code: new TextEncoder().encode(`.tracks { grid-template-columns: ${template}; }`),
    visitor: {
      Declaration: {
        "grid-template-columns"(declaration) {
          const value = declaration.value as { readonly items?: readonly { readonly type: string; readonly value: { readonly type: string; readonly value?: number; readonly max?: { readonly type: string; readonly value: number } } }[] };
          for (const item of value.items ?? []) {
            const size = item.value.type === "min-max" ? item.value.max : item.value;
            if (item.type !== "track-size" || size?.type !== "flex") throw new Error(`not a flexible track: ${JSON.stringify(item)}`);
            found.push(Math.round((size.value as number) * 1e6) / 1e6);
          }
        },
      },
    },
  });
  return found;
}

/** 📱️ Emulates a viewport `width` px wide and `height` px high (by default high enough for every grid) for the size
 * media queries (`min-`/`max-` `width`/`height` in px, joined by `and`); every other query is false. */
function atViewport(width: number, height = 1200): void {
  vi.stubGlobal("matchMedia", (query: string) => ({
    matches: query.split(" and ").every((part) => {
      const found = /^\((min|max)-(width|height):\s*(\d+)px\)$/u.exec(part.trim());
      const size = found?.[2] === "height" ? height : width;
      return found !== null && (found[1] === "min" ? size >= Number(found[3]) : size <= Number(found[3]));
    }),
    media: query,
    onchange: null,
    addEventListener: () => undefined,
    removeEventListener: () => undefined,
    addListener: () => undefined,
    removeListener: () => undefined,
    dispatchEvent: () => false,
  }));
}
//#endregion 🎨️Tracks

//#region 🥞️Overview
/** 🥞️ Home over a session double whose `open` really moves the step, so the overview's open page follows it. */
function Home(props: { readonly session: ReturnType<typeof stubSession>; readonly initial?: QuizStep; readonly preferences?: QuizPreferences }): ReactElement {
  const [step, setStep] = useState<QuizStep>(props.initial ?? { screen: "home" });
  const [session] = useState(() => ({
    ...props.session,
    open: (next: QuizStep) => {
      props.session.open(next);
      setStep(next);
    },
  }));
  return <HomeScreen session={session as unknown as QuizSession} state={{ ...STATE, step }} text={quizText("en")} locale="en" preferences={props.preferences ?? PREFERENCES} onPreferences={() => undefined} />;
}

/** 🖱️ Gives the overview a size and moves the mouse to the fractions `fx`, `fy` of it, then lets the pan settle. */
function moveMouseOver(fx: number, fy: number): void {
  const root = document.querySelector<HTMLElement>("[data-layered-overview]")!;
  vi.spyOn(root, "getBoundingClientRect").mockReturnValue({ left: 0, top: 0, width: 1400, height: 850, right: 1400, bottom: 850, x: 0, y: 0, toJSON: () => ({}) });
  act(() => void window.dispatchEvent(new PointerEvent("pointermove", { clientX: fx * 1400, clientY: fy * 850, pointerType: "mouse" })));
  for (let spent = 0; spent < 3000; spent += 10) act(() => void vi.advanceTimersByTime(10));
}

function peer(anchor: string): PeerCursor {
  return { session: `w-${anchor}`, tag: "0badc0de", label: "Mira K.", colour: 3, cursor: { anchor, x: 0.5, y: 0.5 }, focus: undefined, drag: undefined };
}

function cards(): readonly HTMLElement[] {
  return [...document.querySelectorAll<HTMLElement>(".quiz-home-grid section[data-card]")];
}

function pane(page: string): HTMLElement {
  return document.querySelector<HTMLElement>(`[data-layered-pane="${page}"]`)!;
}

function cardHost(page: string): HTMLElement {
  return document.querySelector<HTMLElement>(`[data-layered-card="${page}"]`)!;
}
//#endregion 🥞️Overview

afterEach(() => {
  window.history.replaceState(null, "", window.location.pathname);
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

describe("🏠️ home overview", () => {
  for (const order of fixture.orders) {
    it(`orders the pages of a catalog with ${order.quizzes.length} quizzes`, () => {
      expect(homePages(order.quizzes)).toEqual(order.pages);
    });
  }

  for (const vector of fixture.cells) {
    it(`lays the pages of ${vector.quizzes} quizzes on the ${vector.layout} strip where their cards sit`, () => {
      const order = fixture.orders.find((candidate) => candidate.quizzes.length === vector.quizzes)!;
      expect(homeCells(order.pages, vector.layout as "desktop" | "tablet")).toEqual(vector.cells);
    });
  }

  it("keeps the strip cells of the nine pages those of the reference layout", () => {
    const nine = homePages(["q0", "q1", "q2", "q3"]);
    expect(homeCells(nine, "desktop")).toEqual(DESKTOP_CELLS_OF_THE_REFERENCE);
    expect(homeCells(nine, "tablet")).toEqual(TABLET_CELLS_OF_THE_REFERENCE);
  });

  for (const excerpt of fixture.excerpts) {
    it(`cuts the leaderboard excerpt ${excerpt.id}`, () => {
      const board = boardOf(excerpt.ranks, excerpt.mine);
      const cut = boardExcerpt({ ...board, rows: [...board.rows].reverse() }, excerpt.mine === null ? undefined : learnerTag(LEARNER));
      expect(cut.top.map((entry) => entry.rank)).toEqual(excerpt.top);
      expect(cut.own?.rank ?? null).toBe(excerpt.own);
    });
  }

  it("lays nine named card sections in reading order over their nine pages, the leaderboard fifth, each heading linking its page", () => {
    render(<Home session={stubSession()} />);
    const shown = cards();
    expect(shown.map((card) => card.getAttribute("data-card"))).toEqual(["learner", "quiz:physics", "introduction", "quiz:heating", "board", "quiz:cooling", "badges", "quiz:demand", "preferences"]);
    const names = shown.map((card) => document.getElementById(card.getAttribute("aria-labelledby") ?? "")?.textContent);
    expect(names).toEqual(["Ada", "Quiz physics", "How it works", "Quiz heating", "Leaderboard", "Quiz cooling", "Badges", "Quiz demand", "Settings"]);
    for (const [index, card] of shown.entries()) {
      expect(screen.getByRole("region", { name: names[index]! })).toBe(card);
      expect(within(card).getByRole("link", { name: names[index]! }).getAttribute("href")).toBe(`#${PAGES[index]}`);
      expect(card.className.split(" ")).toContain("pointer-events-auto");
    }
    expect([...document.querySelectorAll("[data-layered-pane]")].map((element) => element.getAttribute("data-layered-pane"))).toEqual(PAGES);
    for (const page of PAGES) {
      expect(pane(page).hasAttribute("inert")).toBe(true);
      expect(pane(page).getAttribute("aria-hidden")).toBe("true");
    }
    expect(screen.getAllByRole("heading", { level: 1 }).map((heading) => heading.textContent)).toEqual(["Quizzes"]);
    expect(screen.getAllByRole("heading", { level: 2 })).toHaveLength(9);
    for (const button of screen.getAllByRole("button")) expect(button.textContent?.trim() || button.getAttribute("aria-label")).toBeTruthy();
  });

  it("offers each quiz exactly its start, resume or play-again action, and the last result only after a submission", () => {
    render(<Home session={stubSession()} />);
    const actions = (name: string): readonly string[] =>
      within(screen.getByRole("region", { name }))
        .getAllByRole("button")
        .map((button) => button.textContent?.trim() ?? "");
    expect(actions("Quiz physics")).toEqual(["Start quiz"]);
    expect(actions("Quiz heating")).toEqual(["Resume quiz"]);
    expect(actions("Quiz cooling")).toEqual(["View last result", "Start again"]);
    expect(actions("Quiz demand")).toEqual(["View last result", "Resume quiz"]);
    expect(within(screen.getByRole("region", { name: "Quiz cooling" })).getByText("Best score: 99.7%")).toBeTruthy();
    expect(within(screen.getByRole("region", { name: "Quiz cooling" })).getByText("Earned here: 🔥 Heating expert")).toBeTruthy();
    expect(within(screen.getByRole("region", { name: "Quiz heating" })).getByText("In progress")).toBeTruthy();
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
    expect(session.open).toHaveBeenLastCalledWith({ screen: "home" });
    expect(cards()).toHaveLength(9);
    expect(document.activeElement).toBe(within(screen.getByRole("region", { name: "Quiz heating" })).getByRole("link", { name: "Quiz heating" }));
    expect(session.startRun).not.toHaveBeenCalled();
    expect(session.resumeRun).not.toHaveBeenCalled();
  });

  it("shows a quiz's page clear behind its hovered or focused card and never starts or resumes a run for it", async () => {
    const session = stubSession();
    const user = userEvent.setup();
    render(<Home session={session} />);
    const card = screen.getByRole("region", { name: "Quiz heating" });
    fireEvent.pointerOver(card, { pointerType: "mouse" });
    expect(cardHost("heating").hasAttribute("data-revealed")).toBe(true);
    expect(card.hasAttribute("data-revealed")).toBe(true);
    expect(pane("heating").querySelector('[data-page="quiz:heating"]')).not.toBeNull();
    expect(
      within(pane("heating"))
        .queryAllByRole("button", { hidden: true })
        .map((button) => button.textContent?.trim()),
    ).toEqual(["Resume quiz", "Show it now"]);
    fireEvent.pointerOut(card, { pointerType: "mouse", relatedTarget: document.body });
    expect(cardHost("heating").hasAttribute("data-revealed")).toBe(false);
    const link = within(screen.getByRole("region", { name: "Quiz physics" })).getByRole("link", { name: "Quiz physics" });
    const overlay = [...document.querySelectorAll<HTMLElement>(".quiz-home-grid a[href], .quiz-home-grid button")];
    overlay[overlay.indexOf(link) - 1]!.focus();
    await user.tab();
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
    const { unmount } = render(<Home session={session} />);
    await vi.advanceTimersByTimeAsync(0);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(1);
    expect(session.refreshCrowd.mock.calls.map(([quiz]) => quiz)).toEqual([...QUIZZES]);
    await vi.advanceTimersByTimeAsync(LEADERBOARD_POLL_MS);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(2);
    fireEvent.pointerOver(screen.getByRole("region", { name: "Quiz heating" }), { pointerType: "mouse" });
    await vi.advanceTimersByTimeAsync(LEADERBOARD_POLL_MS);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(3);
    fireEvent.click(within(screen.getByRole("region", { name: "Leaderboard" })).getByRole("button", { name: "Full leaderboard" }));
    await vi.advanceTimersByTimeAsync(LEADERBOARD_POLL_MS);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(4);
    expect(session.refreshCrowd).toHaveBeenCalledTimes(QUIZZES.length);
    unmount();
    await vi.advanceTimersByTimeAsync(3 * LEADERBOARD_POLL_MS);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(4);
    const alone = stubSession();
    render(<LeaderboardPage session={alone as unknown as QuizSession} state={STATE} text={quizText("en")} locale="en" view={{ opened: true, revealed: false }} />);
    await vi.advanceTimersByTimeAsync(3 * LEADERBOARD_POLL_MS);
    expect(alone.refreshLeaderboard).not.toHaveBeenCalled();
    expect(screen.queryByRole("status")).toBeNull();
  });

  it("asks for the crowd of every quiz again whenever the polled leaderboard says that a run was submitted, and only then", () => {
    const session = stubSession();
    const home = (state: Partial<QuizState>) => <HomeScreen session={session as unknown as QuizSession} state={{ ...STATE, ...state }} text={quizText("en")} locale="en" preferences={PREFERENCES} onPreferences={() => undefined} />;
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
    const root = document.querySelector<HTMLElement>("[data-layered-overview]")!;
    expect([root.dataset.mode, root.dataset.pan]).toEqual(["strip", "pointer"]);
    const strip = root.querySelector<HTMLElement>("[data-layered-strip]")!;
    expect([strip.style.width, strip.style.height, strip.style.transform]).toEqual(["300%", "300%", "translate3d(0%, 0%, 0)"]);
    expect(root.querySelector<HTMLElement>("[data-layered-veil]")!.dataset.veil).toBe("whole");
    const cells = homeCells(PAGES, "desktop");
    for (const page of PAGES) {
      expect(pane(page).querySelector("[data-page]"), page).not.toBeNull();
      expect([Number(pane(page).style.gridColumn) - 1, Number(pane(page).style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
      const cell = cardHost(page).querySelector<HTMLElement>(".quiz-home-cell")!;
      expect([Number(cell.style.gridColumn) - 1, Number(cell.style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
    }
    const desktop = fixture.layouts.find((layout) => layout.layout === "desktop")!;
    expect(HOME_GRID_TRACKS).toEqual({ columns: desktop.columns, rows: desktop.rows });
    const overlay = document.querySelector<HTMLElement>(".quiz-home-grid")!;
    expect(weights(overlay.style.getPropertyValue("--quiz-home-columns"))).toEqual(desktop.columns);
    expect(weights(overlay.style.getPropertyValue("--quiz-home-rows"))).toEqual(desktop.rows);
    expect(homeTrackTemplate([1, 1.5, 1], 3)).toBe("minmax(0, 1fr) minmax(0, 1.5fr) minmax(0, 1fr)");
    expect(homeTrackTemplate(undefined, 2)).toBe("minmax(0, 1fr) minmax(0, 1fr)");
  });

  it("pans the pages under the glass while the mouse is between the cards, glides to the page of a hovered card and shows it clear, as on play", () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval", "requestAnimationFrame", "cancelAnimationFrame", "performance", "Date"] });
    const session = stubSession();
    render(<Home session={session} />);
    const strip = document.querySelector<HTMLElement>("[data-layered-strip]")!;
    const veil = document.querySelector<HTMLElement>("[data-layered-veil]")!;
    const resting = cards().map((card) => card.outerHTML);
    moveMouseOver(1, 1);
    expect(strip.style.transform).toBe("translate3d(-66.6667%, -66.6667%, 0)");
    moveMouseOver(0.5, 0);
    expect(strip.style.transform).toBe("translate3d(-33.3333%, 0%, 0)");
    expect(veil.dataset.veil).toBe("whole");
    expect(cards().map((card) => card.outerHTML)).toEqual(resting);
    const card = screen.getByRole("region", { name: "Quiz heating" });
    fireEvent.pointerOver(card, { pointerType: "mouse" });
    expect(veil.dataset.veil).toBe("whole");
    for (let spent = 0; spent < 600; spent += 10) act(() => void vi.advanceTimersByTime(10));
    expect(strip.style.transform).toBe("translate3d(0%, -33.3333%, 0)");
    expect([veil.dataset.veil, veil.style.visibility]).toEqual(["clear", "hidden"]);
    moveMouseOver(1, 1);
    expect(strip.style.transform).toBe("translate3d(0%, -33.3333%, 0)");
    fireEvent.pointerOut(card, { pointerType: "mouse", relatedTarget: document.body });
    expect([veil.dataset.veil, veil.style.visibility]).toEqual(["whole", "visible"]);
    expect(strip.style.transform).toBe("translate3d(0%, -33.3333%, 0)");
    moveMouseOver(1, 1);
    expect(strip.style.transform).toBe("translate3d(-66.6667%, -66.6667%, 0)");
    for (const page of PAGES) expect(pane(page).hasAttribute("inert"), page).toBe(true);
    expect(session.startRun).not.toHaveBeenCalled();
    expect(session.resumeRun).not.toHaveBeenCalled();
  });

  it("pans and glides just the same on a device that reports reduced motion, as a Remote Desktop session does for everyone", () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval", "requestAnimationFrame", "cancelAnimationFrame", "performance", "Date"] });
    vi.stubGlobal("matchMedia", (query: string) => ({ matches: query === "(prefers-reduced-motion: reduce)", media: query, onchange: null, addEventListener: () => undefined, removeEventListener: () => undefined, addListener: () => undefined, removeListener: () => undefined, dispatchEvent: () => false }));
    render(<Home session={stubSession()} />);
    expect(document.querySelector<HTMLElement>("[data-layered-overview]")!.dataset.pan).toBe("pointer");
    const strip = document.querySelector<HTMLElement>("[data-layered-strip]")!;
    const veil = document.querySelector<HTMLElement>("[data-layered-veil]")!;
    moveMouseOver(1, 1);
    expect(strip.style.transform).toBe("translate3d(-66.6667%, -66.6667%, 0)");
    fireEvent.pointerOver(screen.getByRole("region", { name: "Leaderboard" }), { pointerType: "mouse" });
    expect(strip.style.transform, "the glide starts where the pan stood").toBe("translate3d(-66.6667%, -66.6667%, 0)");
    for (let spent = 0; spent < 250; spent += 10) act(() => void vi.advanceTimersByTime(10));
    expect(veil.dataset.veil, "under way the glass has a hole over the page").toBe("hole");
    for (let spent = 0; spent < 350; spent += 10) act(() => void vi.advanceTimersByTime(10));
    expect(strip.style.transform).toBe("translate3d(-33.3333%, -33.3333%, 0)");
    expect(veil.dataset.veil).toBe("clear");
  });

  it("shows the others inside the pages behind the cards on each page's own anchors, and none with cursors off", () => {
    const view: PresenceView = {
      ...EMPTY_PRESENCE_VIEW,
      rooms: new Map([
        [roomScope(CATALOG.id, { screen: "leaderboard" })!, [peer("leaderboard")]],
        [roomScope(CATALOG.id, { screen: "quiz", quiz: "heating" })!, [peer("quiz:heating")]],
        [roomScope(CATALOG.id, { screen: "badges" })!, []],
      ]),
    };
    const session = stubSession();
    const tree = (showCursors: boolean) => (
      <PresenceProvider view={view} showCursors={showCursors} setTask={() => undefined}>
        <Home session={session} />
      </PresenceProvider>
    );
    const { rerender } = render(tree(true));
    for (const [page, anchor] of [
      ["board", "leaderboard"],
      ["heating", "quiz:heating"],
    ] as const) {
      const mark = pane(page).querySelector<HTMLElement>('[data-pane-peers] [data-peer="cursor"]')!;
      expect(mark.textContent, page).toBe("Mira K.");
      expect(mark.dataset.anchor).toBe(anchor);
      expect(mark.hidden, page).toBe(false);
      expect(mark.closest('[aria-hidden="true"]')).not.toBeNull();
      expect(pane(page).querySelector(`[data-presence-anchor="${anchor}"]`)).not.toBeNull();
    }
    for (const page of PAGES.filter((candidate) => candidate !== "board" && candidate !== "heating")) expect(pane(page).querySelector("[data-pane-peers]"), page).toBeNull();
    rerender(tree(false));
    expect(document.querySelector("[data-pane-peers]")).toBeNull();
  });

  it("keeps every leaderboard heading and number on one line, letting only learner names wrap", () => {
    const declarations = new Map<string, Readonly<Record<string, unknown>>>();
    transform({
      filename: "🎨️.css",
      code: new TextEncoder().encode(stylesheet),
      visitor: {
        Rule: {
          style(rule) {
            for (const selector of rule.value.selectors) {
              const name = selector.map((part) => (part.type === "class" ? part.name : "")).join("");
              if (name === "quiz-nowrap" || name === "quiz-name") declarations.set(name, Object.fromEntries(rule.value.declarations.declarations.map((declaration) => [declaration.property, declaration.value])));
            }
          },
        },
      },
    });
    expect(declarations.get("quiz-nowrap")).toEqual({ "white-space": "nowrap", hyphens: "none", "overflow-wrap": "normal" });
    expect(declarations.get("quiz-name")).toEqual({ hyphens: "none", "overflow-wrap": "anywhere" });

    const session = stubSession() as unknown as QuizSession;
    render(
      <>
        <LeaderboardCard session={session} state={STATE} text={quizText("en")} locale="en" revealed={false} onOpen={() => undefined} />
        <LeaderboardPage session={session} state={STATE} text={quizText("en")} locale="en" view={{ opened: true, revealed: false }} />
      </>,
    );
    const tables = screen.getAllByRole("table");
    expect(tables).toHaveLength(2);
    for (const table of tables) {
      const heads = within(table).getAllByRole("columnheader");
      expect(heads.length).toBeGreaterThanOrEqual(4);
      for (const head of heads) expect(head.classList.contains("quiz-nowrap"), head.textContent ?? "").toBe(true);
      for (const name of within(table).getAllByRole("rowheader")) {
        expect(name.classList.contains("quiz-name")).toBe(true);
        expect(name.classList.contains("quiz-nowrap")).toBe(false);
      }
      for (const cell of within(table).getAllByRole("cell")) {
        if (!/^[\d.,\s:%–APM]+$/u.test(cell.textContent ?? "") || cell.getAttribute("colspan") !== null) continue;
        const holder = cell.classList.contains("quiz-nowrap") ? cell : cell.querySelector(".quiz-nowrap");
        expect(holder?.textContent, cell.textContent ?? "").toBe(cell.textContent);
      }
    }
  });

  it("shows emoji in text presentation, so every glyph comes from the monochrome emoji face", () => {
    expect(textPresentation(SNOW_EMOJI)).toBe(SNOW_TEXT);
    expect(textPresentation("\u{1F525}")).toBe("\u{1F525}");
    const { container } = render(<Glyph emoji={SNOW_EMOJI} />);
    const glyph = container.querySelector(".quiz-glyph")!;
    expect(glyph.getAttribute("aria-hidden")).toBe("true");
    expect([...(glyph.textContent ?? "")].map((char) => char.codePointAt(0))).toEqual([0x2744, 0xfe0e]);
    expect(stylesheet).toMatch(/\.quiz-glyph \{[^}]*font-variant-emoji: text;/u);
  });

  it("takes its breakpoints from the design system, shares the grid's tracks with no spacing between them and spaces the cards inside their cells", () => {
    expect(fixture.breakpoints).toEqual({ mobileMaxPx: UI_MOBILE_MAX_WIDTH_PX, tabletMaxPx: UI_TABLET_MAX_WIDTH_PX });
    const rules = layerRules(stylesheet);
    expect([...new Set(rules.filter((rule) => rule.selector === ".quiz-home-grid").map((rule) => rule.minWidth))].sort((a, b) => a - b)).toEqual([0, UI_MOBILE_MAX_WIDTH_PX + 1]);
    expect(rules.filter((rule) => rule.selector.startsWith(".quiz-home-grid") && SPACING.includes(rule.property))).toEqual([]);
    expect(rules.filter((rule) => rule.selector === ".quiz-home-cell").map((rule) => rule.property)).toEqual(expect.arrayContaining(["padding", "align-items", "justify-content", "min-width", "min-height"]));
  });

  for (const expected of fixture.layouts) {
    it(`lays the cards over the ${expected.layout} at ${expected.width} px`, () => {
      const { mobileMaxPx, tabletMaxPx } = fixture.breakpoints;
      expect(expected.width <= mobileMaxPx ? "list" : expected.width <= tabletMaxPx ? "tablet" : "desktop").toBe(expected.layout);
      atViewport(expected.width);
      render(<Home session={stubSession()} />);
      const layer = layerAt(layerRules(stylesheet), expected.width);
      if (expected.rows === null) {
        expect(layer).toEqual({ columns: expected.columns.length });
        expect(document.querySelectorAll(".quiz-home-cell")).toHaveLength(0);
        return;
      }
      expect(layer).toEqual({ columns: "var(--quiz-home-columns)", rows: "var(--quiz-home-rows)" });
      const overlay = document.querySelector<HTMLElement>(".quiz-home-grid")!;
      expect(weights(overlay.style.getPropertyValue("--quiz-home-columns"))).toEqual(expected.columns);
      expect(weights(overlay.style.getPropertyValue("--quiz-home-rows"))).toEqual(expected.rows);
      const cells = homeCells(PAGES, expected.layout as "desktop" | "tablet");
      for (const page of PAGES) {
        const cell = cardHost(page).querySelector<HTMLElement>(".quiz-home-cell")!;
        expect([Number(cell.style.gridColumn) - 1, Number(cell.style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
      }
    });
  }
});

describe("🏆️ the leaderboard of the new contract", () => {
  it("is sent with at most a hundred rows", () => {
    expect(fixture.board.sentRows).toBe(LEADERBOARD_TOP);
  });

  for (const vector of fixture.board.page) {
    it(`shows the page ${vector.id}: the sent rows, the own row with its real rank and the number of ranked learners`, () => {
      const board = boardOf(vector.ranks, vector.mine);
      render(<LeaderboardPage session={stubSession() as unknown as QuizSession} state={{ ...STATE, leaderboards: held(board, 1) }} text={quizText("en")} locale="en" view={{ opened: true, revealed: false }} />);
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
    const session = stubSession() as unknown as QuizSession;
    render(<HomeScreen session={session} state={{ ...STATE, leaderboards: held(boardOf(250, 180), 1) }} text={quizText("en")} locale="en" preferences={PREFERENCES} onPreferences={() => undefined} />);
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
    const first = evolveQuizState(STATE, { type: "leaderboard-loaded", leaderboard: boardOf(250, 180), at: 10 });
    expect(shownLeaderboard(first)?.at).toBe(10);
    const again = evolveQuizState(first, { type: "leaderboard-loaded", leaderboard: boardOf(250, 180), at: 20 });
    expect(again).toBe(first);
    const moved = evolveQuizState(again, { type: "leaderboard-loaded", leaderboard: boardOf(250, 179), at: 30 });
    expect(moved).not.toBe(again);
    expect(shownLeaderboard(moved)?.at).toBe(30);
    expect(shownLeaderboard(moved)?.board.own?.rank).toBe(179);
  });
});

describe("🗓️ the four leaderboards and their categories", () => {
  const TODAY = { from: Date.UTC(2026, 9, 2), until: Date.UTC(2026, 9, 3) };
  const heating: Leaderboard = { ...boardOf(3, 2), period: "daily", quiz: "heating", window: TODAY, submissions: 8 };
  const page = (state: Partial<QuizState>, session = stubSession()) => {
    render(<LeaderboardPage session={session as unknown as QuizSession} state={{ ...STATE, ...state }} text={quizText("en")} locale="en" view={{ opened: true, revealed: false }} />);
    return session;
  };
  const choices = (group: string): string[] =>
    within(screen.getByRole("group", { name: group }))
      .getAllByRole("button")
      .map((button) => `${button.textContent}${button.getAttribute("aria-pressed") === "true" ? " ✓" : ""}`);
  const headings = (table: HTMLElement): (string | undefined)[] => within(table).getAllByRole("columnheader").map((heading) => heading.textContent?.trim());
  const ranks = (table: HTMLElement): (string | null | undefined)[] => [...table.querySelectorAll("tbody:not([data-board-own]) > tr:not([aria-hidden])")].map((line) => line.querySelector("td")?.textContent);

  it("offers the four periods and every quiz as a category on the page, the chosen ones pressed", async () => {
    const session = page({});
    expect(choices("Period")).toEqual(["Today", "This week", "This month", "All time ✓"]);
    expect(choices("Category")).toEqual(["All ✓", ...QUIZZES.map((quiz) => `${textPresentation(CATALOG.quizzes.find((entry) => entry.id === quiz)!.emoji)} Quiz ${quiz}`)]);
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
    expect(leaderboardColumns(CATALOG, quizText("en"), "en").map((column) => column.key)).toEqual(["rank", "learner", "total", ...QUIZZES.map((quiz) => `best:${quiz}`), "badges", "runs", "last-activity"]);
    expect(leaderboardColumns(CATALOG, quizText("de"), "de", "heating").map((column) => column.label)).toEqual(["Rang", "Lernende", "Punkte", "Abzeichen", "Durchgänge", "Letzte Abgabe"]);
  });

  it("keeps the choice on the page while a leaderboard it does not hold yet is fetched, and says when one is empty", () => {
    page({ board: { period: "weekly" } });
    expect(choices("Period")).toEqual(["Today", "This week ✓", "This month", "All time"]);
    expect(screen.getByRole("status").textContent).toBe("Loading from the quiz server…");
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
    const session = stubSession() as unknown as QuizSession;
    const view = (state: Partial<QuizState>) => <LeaderboardPage session={session} state={{ ...STATE, ...state }} text={quizText("en")} locale="en" view={{ opened: true, revealed: false }} />;
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
    const card = (state: Partial<QuizState>) => <LeaderboardCard session={session as unknown as QuizSession} state={{ ...STATE, ...state }} text={quizText("en")} locale="en" revealed={false} onOpen={session.open} />;
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
    expect(within(region).getByText("Loading from the quiz server…")).toBeTruthy();
    rerender(card({ board: { period: "monthly" }, leaderboards: held({ period: "monthly", window: TODAY, rows: [], learners: 0, submissions: 8 }, 1) }));
    expect(within(region).getByText("No quiz submitted in this period and category yet.")).toBeTruthy();
  });

  it("holds the last answer of every leaderboard looked at, each under its own period and quiz", () => {
    const chosen = evolveQuizState(STATE, { type: "board-chosen", board: { period: "daily", quiz: "heating" } });
    expect(chosen.board).toEqual({ period: "daily", quiz: "heating" });
    expect(shownLeaderboard(chosen)).toBeUndefined();
    expect(evolveQuizState(chosen, { type: "board-chosen", board: { period: "daily", quiz: "heating" } })).toBe(chosen);
    const answered = evolveQuizState(chosen, { type: "leaderboard-loaded", leaderboard: heating, at: 40 });
    expect(shownLeaderboard(answered)).toEqual({ board: heating, at: 40 });
    expect(Object.keys(answered.leaderboards).sort()).toEqual(["all-time", "daily/heating"]);
    expect(answered.leaderboards["all-time"]).toBe(STATE.leaderboards["all-time"]);
    const back = evolveQuizState(answered, { type: "board-chosen", board: { period: "all-time" } });
    expect(shownLeaderboard(back)).toBe(STATE.leaderboards["all-time"]);
    expect(boardKey({ period: "weekly" })).toBe("weekly");
    expect(boardKey({ period: "monthly", quiz: "cooling" })).toBe("monthly/cooling");
  });

  it("remembers how many runs the last answer counted, whatever leaderboard it was", () => {
    expect(STATE.submissions).toBe(8);
    const same = evolveQuizState(STATE, { type: "leaderboard-loaded", leaderboard: heating, at: 40 });
    expect(same.submissions).toBe(8);
    const more = evolveQuizState(same, { type: "leaderboard-loaded", leaderboard: { ...heating, submissions: 9 }, at: 50 });
    expect([more.submissions, shownLeaderboard(more)?.board.submissions]).toEqual([9, 8]);
    const overall = evolveQuizState(more, { type: "leaderboard-loaded", leaderboard: boardOf(8, 7), at: 60 });
    expect(overall.submissions).toBe(8);
    expect(overall.leaderboards).toBe(more.leaderboards);
    expect(evolveQuizState(overall, { type: "leaderboard-loaded", leaderboard: boardOf(8, 7), at: 70 })).toBe(overall);
  });

  it("takes no quiz the catalog does not list as a category, and forgets every leaderboard with the learner", () => {
    expect(evolveQuizState(STATE, { type: "board-chosen", board: { period: "weekly", quiz: "plumbing" } }).board).toEqual({ period: "weekly" });
    const chosen = evolveQuizState(STATE, { type: "board-chosen", board: { period: "weekly", quiz: "demand" } });
    const shrunk = evolveQuizState(chosen, { type: "catalog-loaded", catalog: { ...CATALOG, quizzes: CATALOG.quizzes.filter((quiz) => quiz.id !== "demand") } });
    expect(shrunk.board).toEqual({ period: "weekly" });
    expect(evolveQuizState(chosen, { type: "catalog-loaded", catalog: CATALOG }).board).toBe(chosen.board);
    expect(evolveQuizState(chosen, { type: "learner-identified", learner: LEARNER }).leaderboards).toBe(chosen.leaderboards);
    expect(evolveQuizState(chosen, { type: "learner-identified", learner: "f".repeat(32) }).leaderboards).toEqual({});
    expect(evolveQuizState(chosen, { type: "learner-forgotten" }).leaderboards).toEqual({});
    expect(evolveQuizState(chosen, { type: "learner-forgotten" }).board).toEqual({ period: "weekly", quiz: "demand" });
  });
});

describe("🏠️ the layout in the learner's text size", () => {
  it("needs a row high enough for a card in every row of a grid, below the navigation bar", () => {
    const { rowHeightPx, chromeHeightPx, minimum } = fixture.heights;
    expect([HOME_GRID_ROW_HEIGHT_PX, HOME_CHROME_HEIGHT_PX]).toEqual([rowHeightPx, chromeHeightPx]);
    expect(homeGridMinHeight("desktop", PAGES)).toBe(minimum.desktop);
    expect(homeGridMinHeight("tablet", PAGES)).toBe(minimum.tablet);
    const weight = HOME_GRID_TRACKS.rows.reduce((total, row) => total + row, 0) / Math.min(...HOME_GRID_TRACKS.rows);
    expect(minimum.desktop).toBe(Math.ceil(chromeHeightPx + rowHeightPx * weight));
    expect(minimum.tablet).toBe(chromeHeightPx + rowHeightPx * 5);
    expect(homeGridMinHeight("desktop", [...PAGES, "sixth", "seventh"])).toBe(Math.ceil(chromeHeightPx + rowHeightPx * (weight + 1)));
  });

  it("lets a card that is taller than its cell after all scroll inside the cell, never over the card below", () => {
    const cell = (property: string): string => new RegExp(`\\.quiz-home-cell \\{[^}]*\\b${property}: ([^;]+);`, "u").exec(stylesheet)?.[1] ?? "";
    expect(cell("overflow-y")).toBe("auto");
    expect(cell("align-items")).toBe("safe center");
    expect(cell("min-height")).toBe("0");
  });

  it("measures the breakpoints and the least grid heights in the text size", () => {
    expect(homeLayoutQueries(1, PAGES)).toEqual({
      narrow: `(max-width: ${UI_MOBILE_MAX_WIDTH_PX}px)`,
      medium: `(min-width: ${UI_MOBILE_MAX_WIDTH_PX + 1}px) and (max-width: ${UI_TABLET_MAX_WIDTH_PX}px)`,
      short: { desktop: `(max-height: ${fixture.heights.minimum.desktop - 1}px)`, tablet: `(max-height: ${fixture.heights.minimum.tablet - 1}px)` },
    });
    expect(homeLayoutQueries(1.5, PAGES)).toEqual({ narrow: "(max-width: 1150px)", medium: "(min-width: 1151px) and (max-width: 1534px)", short: { desktop: "(max-height: 899px)", tablet: "(max-height: 1283px)" } });
  });

  for (const { width, height, scale, layout } of fixture.heights.vectors) {
    const textSize = TEXT_SIZES.find((size) => size.scale === scale)!.value;
    it(`lays ${width} × ${height} px at the text size "${textSize}" out as ${layout}`, () => {
      atViewport(width, height);
      render(<HomeScreen session={stubSession() as unknown as QuizSession} state={STATE} text={quizText("en")} locale="en" preferences={{ ...PREFERENCES, textSize }} onPreferences={() => undefined} />);
      const overview = document.querySelector("[data-layered-overview]")!;
      expect(overview.getAttribute("data-mode")).toBe(layout === "list" ? "list" : "strip");
      if (layout === "list") return;
      const cells = homeCells(PAGES, layout as "desktop" | "tablet");
      for (const page of PAGES) {
        const cell = cardHost(page).querySelector<HTMLElement>(".quiz-home-cell")!;
        expect([Number(cell.style.gridColumn) - 1, Number(cell.style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
      }
    });
  }
});

describe("🥞️ the opened page and the short viewport", () => {
  it("leaves the way back to the overview to the navbar: an opened page carries no button of the overview's own", async () => {
    render(<Home session={stubSession()} initial={{ screen: "home", page: "board" }} />);
    await act(async () => undefined);
    const overview = document.querySelector<HTMLElement>("[data-layered-overview]")!;
    expect(pane("board").hasAttribute("data-opened")).toBe(true);
    expect(overview.querySelector("[data-layered-overview-button]")).toBeNull();
    expect(within(overview).queryByRole("button", { name: "Overview" })).toBeNull();
    const stops = [...overview.querySelectorAll<HTMLElement>("button, a[href], [tabindex]:not([tabindex='-1'])")].filter((element) => element.closest("[inert]") === null);
    expect(stops.length).toBeGreaterThan(1);
    for (const stop of stops) expect(pane("board").contains(stop)).toBe(true);
  });

  it("lets the card of a list section scroll instead of clipping it when the viewport is short", () => {
    atViewport(700, 360);
    render(<Home session={stubSession()} />);
    const layers = [...document.querySelectorAll<HTMLElement>("[data-layered-section] > div.pointer-events-none")].filter((layer) => layer.querySelector("[data-layered-card]") !== null);
    expect(layers.length).toBeGreaterThan(0);
    for (const layer of layers) {
      const classes = layer.className.split(" ");
      expect(classes).toContain("overflow-y-auto");
      expect(classes).toContain("items-center-safe");
      expect(classes.some((name) => /^pb-\[\d+px\]$/u.test(name))).toBe(true);
      expect(classes.some((name) => name.endsWith("rem]"))).toBe(false);
    }
  });
});
