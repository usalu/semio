/** 🟠️ The zooms hovered topic pans tests of the 🏠️ home overview suite. */
import { placePeers } from "@semio-tech/course-react";
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { transform } from "lightningcss";
import { expect, it, vi } from "vitest";
import { UI_MOBILE_MAX_WIDTH_PX, UI_TABLET_MAX_WIDTH_PX } from "@semio-tech/ui-react/chrome";
import { roomScope } from "@semio-tech/course";
import { EMPTY_PRESENCE_VIEW, Glyph, LeaderboardCard, LeaderboardPage, PresenceProvider, courseText, textPresentation, type PresenceView, type CourseSession } from "@semio-tech/course-react";
import { quizStylesheet as stylesheet } from "@semio-tech/course-react/stylesheet";
import { COURSE, Home, PAGES, SNOW_EMOJI, STATE, fixture, pane, stubSession } from "../../🛠️home-grid-kit/🟦️.tsx";
import { SNOW_TEXT, SPACING, gridStrip, gridVeil, layerRules, moveMouseOver, peer } from "../🟣️harness/🟦️.tsx";

/** 🟡 Registers the zooms hovered topic pans tests of the 🏠️ home overview suite. */
export function registerZoomsHoveredTopicPansTests(): void {
  it("zooms a topic whose explore control is hovered and pans the quizzes behind its cards without zooming those quizzes", () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval", "requestAnimationFrame", "cancelAnimationFrame", "performance", "Date"] });
    const topics = fixture.hierarchy.topics!.map((topic) => ({ ...topic, quizzes: topic.id === "energy" ? ["physics", "heating", "cooling", "demand", "storage", "network"] : [] }));
    render(<Home session={stubSession()} course={{ ...COURSE, topics }} initial={{ screen: "home", page: "quizzes" }} />);
    moveMouseOver(1, 1);
    expect(gridStrip("quizzes").style.transform).toBe("translate3d(-50%, 0%, 0)");
    expect(gridStrip("energy").style.transform).toBe("translate3d(-66.6667%, -50%, 0)");
    expect(gridVeil("energy").dataset.veil).toBe("whole");
    fireEvent.pointerOver(within(screen.getByRole("region", { name: "Energy" })).getByRole("button", { name: "Explore" }), { pointerType: "mouse" });
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
    fireEvent.pointerOver(within(screen.getByRole("region", { name: "Quizzes" })).getByRole("button", { name: "Explore" }), { pointerType: "mouse" });
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
    fireEvent.pointerOver(within(screen.getByRole("region", { name: "Badges" })).getByRole("button", { name: "All badges" }), { pointerType: "mouse" });
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
}
