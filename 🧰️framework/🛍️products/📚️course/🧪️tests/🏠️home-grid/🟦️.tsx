import { cleanup } from "@testing-library/react";
import { type LayeredMode } from "@semio-tech/ui-react/chrome";
import { type HomeGrid } from "@semio-tech/course-react";
import { produce } from "immer";
import { FollowProvider, addressPages, parentPage, pagesWithin, previewCamera, placePeers } from "@semio-tech/course-react";
/** 🏠️ The layered home (design §16–§17): the pages of the overview in reading order and the cell of each (shared
 * vectors), five general cards over the glass and nested topic and quiz grids, each heading linking its page, every
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
import { learnerTag, roomScope } from "@semio-tech/course";
import { EMPTY_PRESENCE_VIEW, Glyph, HOME_GRID_TRACKS, HomeScreen, LEADERBOARD_POLL_MS, LeaderboardCard, LeaderboardPage, PresenceProvider, boardExcerpt, homeCells, homeLayout, homePages, homeTrackTemplate, courseText, textPresentation, type PeerCursor, type PresenceView, type CourseSession, type CourseState } from "@semio-tech/course-react";
import { quizStylesheet as stylesheet } from "@semio-tech/course-react/stylesheet";
import { COURSE, Home, LEARNER, PAGES, PREFERENCES, QUIZZES, SNOW_EMOJI, STATE, atViewport, boardOf, cardHost, fixture, held, pane, stubSession } from "../🛠️home-grid-kit/🟦️.tsx";
import followedLayouts from "../../🧫️fixtures/👀️follow/🔣️.json";

const SNOW_TEXT = String.fromCodePoint(0x2744, 0xfe0e);

const DESKTOP_CELLS_OF_THE_REFERENCE = Object.fromEntries(PAGES.map((page, index) => [page, { column: index % 3, row: Math.floor(index / 3) }]));
const TABLET_CELLS_OF_THE_REFERENCE = Object.fromEntries(PAGES.map((page, index) => [page, { column: index % 2, row: Math.floor(index / 2) }]));

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

/** 🎨️ Every declaration of `.course-home-grid` and `.course-home-cell` with the minimum viewport width it applies from. */
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
        if (!name.startsWith(".course-home-grid") && !name.startsWith(".course-home-cell")) continue;
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
    if (width < rule.minWidth || rule.selector !== ".course-home-grid") continue;
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
  const root = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview]')!;
  vi.spyOn(root, "getBoundingClientRect").mockReturnValue({ left: 0, top: 0, width: 1400, height: 850, right: 1400, bottom: 850, x: 0, y: 0, toJSON: () => ({}) });
  act(() => void window.dispatchEvent(new PointerEvent("pointermove", { clientX: fx * 1400, clientY: fy * 850, pointerType: "mouse" })));
  for (let spent = 0; spent < 3000; spent += 10) act(() => void vi.advanceTimersByTime(10));
}

function peer(anchor: string): PeerCursor {
  return { session: `w-${anchor}`, tag: "0badc0de", label: "Mira K.", colour: 3, place: { screen: "home" }, home: undefined, cursor: { anchor, x: 0.5, y: 0.5 }, focus: undefined, drag: undefined };
}

function cards(): readonly HTMLElement[] {
  return [...document.querySelectorAll<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-overlay].course-home-grid section[data-card]')];
}

describe("🏠️ home overview", () => {
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

  it("shows a quiz's page clear behind its hovered or focused card and never starts or resumes a run for it", () => {
    const session = stubSession();
    render(<Home initial={{ screen: "home", page: "quizzes" }} session={session} />);
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
    fireEvent.pointerOver(screen.getByRole("region", { name: "Quiz heating" }), { pointerType: "mouse" });
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

  it("pans the pages under the glass while the mouse is between the cards, glides to the page of a hovered card and shows it clear, as on play", () => {
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
    fireEvent.pointerOver(card, { pointerType: "mouse" });
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

  /** 🧭 The strip directly under one overview. */
  function gridStrip(id: string): HTMLElement {
    return document.querySelector<HTMLElement>(`[data-course-grid="${id}"] > [data-layered-overview] > [data-layered-strip]`)!;
  }

  /** 🪟 The glass directly under one overview. */
  function gridVeil(id: string): HTMLElement {
    return document.querySelector<HTMLElement>(`[data-course-grid="${id}"] > [data-layered-overview] > [data-layered-veil]`)!;
  }

  it("zooms a hovered topic and pans the quizzes behind its cards without zooming those quizzes", () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval", "requestAnimationFrame", "cancelAnimationFrame", "performance", "Date"] });
    const topics = fixture.hierarchy.topics!.map((topic) => ({ ...topic, quizzes: topic.id === "energy" ? ["physics", "heating", "cooling", "demand", "storage", "network"] : [] }));
    render(<Home session={stubSession()} course={{ ...COURSE, topics }} initial={{ screen: "home", page: "quizzes" }} />);
    moveMouseOver(1, 1);
    expect(gridStrip("quizzes").style.transform).toBe("translate3d(-50%, 0%, 0)");
    expect(gridStrip("energy").style.transform).toBe("translate3d(-66.6667%, -50%, 0)");
    expect(gridVeil("energy").dataset.veil).toBe("whole");
    fireEvent.pointerOver(screen.getByRole("region", { name: "Energy" }), { pointerType: "mouse" });
    for (let spent = 0; spent < 600; spent += 10) act(() => void vi.advanceTimersByTime(10));
    expect(gridStrip("quizzes").style.transform).toBe("translate3d(0%, 0%, 0)");
    expect(gridVeil("quizzes").dataset.veil).toBe("clear");
    expect(gridStrip("energy").style.transform).toBe("translate3d(-66.6667%, -50%, 0)");
    moveMouseOver(0, 0);
    expect(gridStrip("quizzes").style.transform).toBe("translate3d(0%, 0%, 0)");
    expect(gridVeil("quizzes").dataset.veil).toBe("clear");
    expect(gridStrip("energy").style.transform).toBe("translate3d(0%, 0%, 0)");
    expect(gridVeil("energy").dataset.veil).toBe("whole");
    for (const id of ["physics", "heating", "cooling", "demand", "storage", "network"]) expect(document.querySelector(`[data-course-grid="energy"] [data-layered-card="${id}"]`)!.hasAttribute("data-revealed"), id).toBe(false);
  });

  it("pans every grid behind a zoomed overview without zooming those deeper cards", () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval", "requestAnimationFrame", "cancelAnimationFrame", "performance", "Date"] });
    const topics = fixture.hierarchy.topics!.map((topic) => ({ ...topic, quizzes: topic.id === "energy" ? ["physics", "heating", "cooling", "demand", "storage", "network"] : [] }));
    render(<Home session={stubSession()} course={{ ...COURSE, topics }} />);
    fireEvent.pointerOver(screen.getByRole("region", { name: "Quizzes" }), { pointerType: "mouse" });
    for (let spent = 0; spent < 600; spent += 10) act(() => void vi.advanceTimersByTime(10));
    expect(gridStrip("root").style.transform).toBe("translate3d(-33.3333%, 0%, 0)");
    expect(gridVeil("root").dataset.veil).toBe("clear");
    moveMouseOver(1, 1);
    expect(gridStrip("root").style.transform).toBe("translate3d(-33.3333%, 0%, 0)");
    expect(gridVeil("root").dataset.veil).toBe("clear");
    expect(gridStrip("quizzes").style.transform).toBe("translate3d(-50%, 0%, 0)");
    expect(gridVeil("quizzes").dataset.veil).toBe("whole");
    expect(gridStrip("energy").style.transform).toBe("translate3d(-66.6667%, -50%, 0)");
    expect(gridVeil("energy").dataset.veil).toBe("whole");
    expect(document.querySelector('[data-layered-card="energy"]')!.hasAttribute("data-revealed")).toBe(false);
    for (const id of ["physics", "heating", "cooling", "demand", "storage", "network"]) expect(document.querySelector(`[data-layered-card="${id}"]`)!.hasAttribute("data-revealed"), id).toBe(false);
  });

  it("pans and glides just the same on a device that reports reduced motion, as a Remote Desktop session does for everyone", () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval", "requestAnimationFrame", "cancelAnimationFrame", "performance", "Date"] });
    vi.stubGlobal("matchMedia", (query: string) => ({ matches: query === "(prefers-reduced-motion: reduce)", media: query, onchange: null, addEventListener: () => undefined, removeEventListener: () => undefined, addListener: () => undefined, removeListener: () => undefined, dispatchEvent: () => false }));
    render(<Home session={stubSession()} />);
    expect(document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview]')!.dataset.pan).toBe("pointer");
    const strip = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-strip]')!;
    const veil = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-veil]')!;
    moveMouseOver(1, 1);
    expect(strip.style.transform).toBe("translate3d(-33.3333%, -50%, 0)");
    fireEvent.pointerOver(screen.getByRole("region", { name: "Badges" }), { pointerType: "mouse" });
    expect(strip.style.transform, "the glide starts where the pan stood").toBe("translate3d(-33.3333%, -50%, 0)");
    for (let spent = 0; spent < 250; spent += 10) act(() => void vi.advanceTimersByTime(10));
    expect(veil.dataset.veil, "under way the glass has a hole over the page").toBe("hole");
    for (let spent = 0; spent < 350; spent += 10) act(() => void vi.advanceTimersByTime(10));
    expect(strip.style.transform).toBe("translate3d(0%, -50%, 0)");
    expect(veil.dataset.veil).toBe("clear");
  });

  it("shows the others inside the pages behind the cards on each page's own anchors, and none with cursors off", () => {
    const view: PresenceView = {
      ...EMPTY_PRESENCE_VIEW,
      rooms: new Map([
        [roomScope(COURSE.id, { screen: "leaderboard" })!, [peer("leaderboard")]],
        [roomScope(COURSE.id, { screen: "quiz", quiz: "heating" })!, [peer("quiz:heating")]],
        [roomScope(COURSE.id, { screen: "badges" })!, []],
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
      const anchorElement = pane(page).querySelector<HTMLElement>(`[data-presence-anchor="${anchor}"]`)!;
      vi.spyOn(anchorElement, "getBoundingClientRect").mockReturnValue({ left: 0, top: 0, right: 200, bottom: 100, width: 200, height: 100, x: 0, y: 0, toJSON: () => ({}) });
      const layer = mark.closest<HTMLElement>("[data-pane-peers]")!;
      placePeers(layer, layer.parentElement!);
      expect(mark.textContent, page).toBe("Mira K.");
      expect(mark.dataset.anchor).toBe(anchor);
      expect(mark.hidden, page).toBe(false);
      expect(mark.style.transform, page).toBe("translate(100px, 50px)");
      expect(mark.closest('[aria-hidden="true"]')).not.toBeNull();
      expect(pane(page).querySelector(`[data-presence-anchor="${anchor}"]`)).not.toBeNull();
    }
    for (const page of PAGES.filter((candidate) => candidate !== "board" && candidate !== "quizzes")) expect(pane(page).querySelector("[data-pane-peers]"), page).toBeNull();
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
              if (name === "course-nowrap" || name === "course-name") declarations.set(name, Object.fromEntries(rule.value.declarations.declarations.map((declaration) => [declaration.property, declaration.value])));
            }
          },
        },
      },
    });
    expect(declarations.get("course-nowrap")).toEqual({ "white-space": "nowrap", hyphens: "none", "overflow-wrap": "normal" });
    expect(declarations.get("course-name")).toEqual({ hyphens: "none", "overflow-wrap": "anywhere" });

    const session = stubSession() as unknown as CourseSession;
    render(
      <>
        <LeaderboardCard session={session} state={STATE} text={courseText("en")} locale="en" revealed={false} onOpen={() => undefined} />
        <LeaderboardPage session={session} state={STATE} text={courseText("en")} locale="en" view={{ opened: true, revealed: false }} />
      </>,
    );
    const tables = screen.getAllByRole("table");
    expect(tables).toHaveLength(2);
    for (const table of tables) {
      const heads = within(table).getAllByRole("columnheader");
      expect(heads.length).toBeGreaterThanOrEqual(4);
      for (const head of heads) expect(head.classList.contains("course-nowrap"), head.textContent ?? "").toBe(true);
      for (const name of within(table).getAllByRole("rowheader")) {
        expect(name.classList.contains("course-name")).toBe(true);
        expect(name.classList.contains("course-nowrap")).toBe(false);
      }
      for (const cell of within(table).getAllByRole("cell")) {
        if (!/^[\d.,\s:%–APM]+$/u.test(cell.textContent ?? "") || cell.getAttribute("colspan") !== null) continue;
        const holder = cell.classList.contains("course-nowrap") ? cell : cell.querySelector(".course-nowrap");
        expect(holder?.textContent, cell.textContent ?? "").toBe(cell.textContent);
      }
    }
  });

  it("shows emoji in text presentation, so every glyph comes from the monochrome emoji face", () => {
    expect(textPresentation(SNOW_EMOJI)).toBe(SNOW_TEXT);
    expect(textPresentation("\u{1F525}")).toBe("\u{1F525}");
    const { container } = render(<Glyph emoji={SNOW_EMOJI} />);
    const glyph = container.querySelector(".course-glyph")!;
    expect(glyph.getAttribute("aria-hidden")).toBe("true");
    expect([...(glyph.textContent ?? "")].map((char) => char.codePointAt(0))).toEqual([0x2744, 0xfe0e]);
    expect(stylesheet).toMatch(/\.course-glyph \{[^}]*font-variant-emoji: text;/u);
  });

  it("takes its breakpoints from the design system, shares the grid's tracks with no spacing between them and spaces the cards inside their cells", () => {
    expect(fixture.breakpoints).toEqual({ mobileMaxPx: UI_MOBILE_MAX_WIDTH_PX, tabletMaxPx: UI_TABLET_MAX_WIDTH_PX });
    const rules = layerRules(stylesheet);
    expect([...new Set(rules.filter((rule) => rule.selector === ".course-home-grid").map((rule) => rule.minWidth))]).toEqual([0]);
    expect(rules.filter((rule) => rule.selector.startsWith(".course-home-grid") && SPACING.includes(rule.property))).toEqual([]);
    expect(rules.filter((rule) => rule.selector === ".course-home-cell").map((rule) => rule.property)).toEqual(expect.arrayContaining(["padding", "align-items", "justify-content", "min-width", "min-height"]));
  });

  for (const expected of fixture.layouts) {
    it(`lays the cards over the ${expected.grid} grid at ${expected.width} px, ${expected.mode === "swipe" ? "swiped one page at a time" : "fixed over the pages"}`, () => {
      const { mobileMaxPx, tabletMaxPx } = fixture.breakpoints;
      expect(homeLayout({ narrow: expected.width <= mobileMaxPx, medium: expected.width > mobileMaxPx && expected.width <= tabletMaxPx, short: { desktop: false, tablet: false } })).toEqual({ grid: expected.grid, mode: expected.mode });
      atViewport(expected.width);
      render(<Home session={stubSession()} />);
      expect(document.querySelector('[data-grid-active] > [data-layered-overview]')!.getAttribute("data-mode")).toBe(expected.mode);
      const cells = homeCells(PAGES, expected.grid);
      if (expected.mode === "swipe") {
        expect(document.querySelector('[data-grid-active] > [data-layered-overview] > [data-layered-overlay].course-home-grid')).toBeNull();
        expect(document.querySelectorAll('[data-grid-active] > [data-layered-overview] > [data-layered-overlay] .course-home-cell')).toHaveLength(0);
        expect([...expected.columns, ...expected.rows].every((weight) => weight === 1)).toBe(true);
        const overlay = document.querySelector<HTMLElement>("[data-layered-overlay]")!;
        expect([overlay.style.gridTemplateColumns, overlay.style.gridTemplateRows]).toEqual([`repeat(${expected.columns.length}, minmax(0, 1fr))`, `repeat(${expected.rows.length}, minmax(0, 1fr))`]);
        for (const page of PAGES) {
          const cell = document.querySelector<HTMLElement>(`[data-layered-cell="${page}"]`)!;
          expect([Number(cell.style.gridColumn) - 1, Number(cell.style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
          expect(cell.querySelector("[data-layered-card] .course-home-entry"), page).not.toBeNull();
        }
        return;
      }
      expect(layerAt(layerRules(stylesheet), expected.width)).toEqual({ columns: "var(--course-home-columns)", rows: "var(--course-home-rows)" });
      const overlay = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-overlay].course-home-grid')!;
      expect(weights(overlay.style.getPropertyValue("--course-home-columns"))).toEqual(expected.columns);
      expect(weights(overlay.style.getPropertyValue("--course-home-rows"))).toEqual(expected.rows);
      for (const page of PAGES) {
        const cell = cardHost(page).querySelector<HTMLElement>(".course-home-cell")!;
        expect([Number(cell.style.gridColumn) - 1, Number(cell.style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
      }
    });
  }
});
