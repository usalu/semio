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
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { transform } from "lightningcss";
import { describe, expect, it, vi } from "vitest";
import { UI_MOBILE_MAX_WIDTH_PX, UI_TABLET_MAX_WIDTH_PX } from "@semio-tech/ui-react/chrome";
import { learnerTag, roomScope } from "@semio-tech/quiz";
import { EMPTY_PRESENCE_VIEW, Glyph, HOME_GRID_TRACKS, HomeScreen, LEADERBOARD_POLL_MS, LeaderboardCard, LeaderboardPage, PresenceProvider, boardExcerpt, homeCells, homeLayout, homePages, homeTrackTemplate, quizText, textPresentation, type PeerCursor, type PresenceView, type QuizSession, type QuizState } from "@semio-tech/quiz-react";
import { quizStylesheet as stylesheet } from "@semio-tech/quiz-react/stylesheet";
import { CATALOG, Home, LEARNER, PAGES, PREFERENCES, QUIZZES, SNOW_EMOJI, STATE, atViewport, boardOf, cardHost, fixture, held, pane, stubSession } from "../🛠️home-grid-kit/🟦️.tsx";

const SNOW_TEXT = String.fromCodePoint(0x2744, 0xfe0e);

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
//#endregion 🎨️Tracks

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

describe("🏠️ home overview", () => {
  for (const order of fixture.orders) {
    it(`orders the pages of a catalog with ${order.quizzes.length} quizzes`, () => {
      expect(homePages(order.quizzes)).toEqual(order.pages);
    });
  }

  for (const vector of fixture.cells) {
    it(`lays the pages of ${vector.quizzes} quizzes on the ${vector.grid} strip where their cards sit`, () => {
      const order = fixture.orders.find((candidate) => candidate.quizzes.length === vector.quizzes)!;
      expect(homeCells(order.pages, vector.grid)).toEqual(vector.cells);
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
    expect(actions("Quiz physics")).toEqual(["Start (Medium)"]);
    expect(actions("Quiz heating")).toEqual(["Resume (Medium)"]);
    expect(actions("Quiz cooling")).toEqual(["View last result", "Again (Medium)"]);
    expect(actions("Quiz demand")).toEqual(["View last result", "Resume (Expert)"]);
    expect(within(screen.getByRole("region", { name: "Quiz cooling" })).getByText("Best: 299.1 of 300 (Hard)")).toBeTruthy();
    expect(within(screen.getByRole("region", { name: "Quiz demand" })).getByText("Best: 50 of 100 (Easy)")).toBeTruthy();
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
    ).toEqual(["Resume (Medium)", "Show it now"]);
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
    const desktop = fixture.layouts.find((layout) => layout.grid === "desktop" && layout.mode === "strip")!;
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
    expect([...new Set(rules.filter((rule) => rule.selector === ".quiz-home-grid").map((rule) => rule.minWidth))]).toEqual([0]);
    expect(rules.filter((rule) => rule.selector.startsWith(".quiz-home-grid") && SPACING.includes(rule.property))).toEqual([]);
    expect(rules.filter((rule) => rule.selector === ".quiz-home-cell").map((rule) => rule.property)).toEqual(expect.arrayContaining(["padding", "align-items", "justify-content", "min-width", "min-height"]));
  });

  for (const expected of fixture.layouts) {
    it(`lays the cards over the ${expected.grid} grid at ${expected.width} px, ${expected.mode === "swipe" ? "swiped one page at a time" : "fixed over the pages"}`, () => {
      const { mobileMaxPx, tabletMaxPx } = fixture.breakpoints;
      expect(homeLayout({ narrow: expected.width <= mobileMaxPx, medium: expected.width > mobileMaxPx && expected.width <= tabletMaxPx, short: { desktop: false, tablet: false } })).toEqual({ grid: expected.grid, mode: expected.mode });
      atViewport(expected.width);
      render(<Home session={stubSession()} />);
      expect(document.querySelector("[data-layered-overview]")!.getAttribute("data-mode")).toBe(expected.mode);
      const cells = homeCells(PAGES, expected.grid);
      if (expected.mode === "swipe") {
        expect(document.querySelector(".quiz-home-grid")).toBeNull();
        expect(document.querySelectorAll(".quiz-home-cell")).toHaveLength(0);
        expect([...expected.columns, ...expected.rows].every((weight) => weight === 1)).toBe(true);
        const overlay = document.querySelector<HTMLElement>("[data-layered-overlay]")!;
        expect([overlay.style.gridTemplateColumns, overlay.style.gridTemplateRows]).toEqual([`repeat(${expected.columns.length}, minmax(0, 1fr))`, `repeat(${expected.rows.length}, minmax(0, 1fr))`]);
        for (const page of PAGES) {
          const cell = document.querySelector<HTMLElement>(`[data-layered-cell="${page}"]`)!;
          expect([Number(cell.style.gridColumn) - 1, Number(cell.style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
          expect(cell.querySelector("[data-layered-card] .quiz-home-entry"), page).not.toBeNull();
        }
        return;
      }
      expect(layerAt(layerRules(stylesheet), expected.width)).toEqual({ columns: "var(--quiz-home-columns)", rows: "var(--quiz-home-rows)" });
      const overlay = document.querySelector<HTMLElement>(".quiz-home-grid")!;
      expect(weights(overlay.style.getPropertyValue("--quiz-home-columns"))).toEqual(expected.columns);
      expect(weights(overlay.style.getPropertyValue("--quiz-home-rows"))).toEqual(expected.rows);
      for (const page of PAGES) {
        const cell = cardHost(page).querySelector<HTMLElement>(".quiz-home-cell")!;
        expect([Number(cell.style.gridColumn) - 1, Number(cell.style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
      }
    });
  }
});
