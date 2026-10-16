/** 📱️ The layered home in the learner's text size, the opened page and the short viewport. */
import { act, render, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { UI_MOBILE_MAX_WIDTH_PX, UI_TABLET_MAX_WIDTH_PX } from "@semio-tech/ui-react/chrome";
import { HOME_CHROME_HEIGHT_PX, HOME_GRID_ROW_HEIGHT_PX, HOME_GRID_TRACKS, HomeScreen, TEXT_SIZES, homeCells, homeGridMinHeight, homeLayoutQueries, quizText, type QuizSession } from "@semio-tech/quiz-react";
import stylesheet from "../../🎯️targets/⚛️react/🎨️.css?raw";
import { Home, PAGES, PREFERENCES, STATE, atViewport, cardHost, fixture, pane, stubSession } from "../🛠️home-grid-kit/🟦️.tsx";

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

  for (const { width, height, scale, grid, mode } of fixture.heights.vectors) {
    const textSize = TEXT_SIZES.find((size) => size.scale === scale)!.value;
    it(`lays ${width} × ${height} px at the text size "${textSize}" out on the ${grid} grid, ${mode === "swipe" ? "swiped" : "fixed"}`, () => {
      atViewport(width, height);
      render(<HomeScreen session={stubSession() as unknown as QuizSession} state={STATE} text={quizText("en")} locale="en" preferences={{ ...PREFERENCES, textSize }} onPreferences={() => undefined} />);
      const overview = document.querySelector("[data-layered-overview]")!;
      expect(overview.getAttribute("data-mode")).toBe(mode);
      const cells = homeCells(PAGES, grid);
      for (const page of PAGES) {
        const cell = mode === "swipe" ? document.querySelector<HTMLElement>(`[data-layered-cell="${page}"]`)! : cardHost(page).querySelector<HTMLElement>(".quiz-home-cell")!;
        expect([Number(cell.style.gridColumn) - 1, Number(cell.style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
        expect([Number(pane(page).style.gridColumn) - 1, Number(pane(page).style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
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

  it("names the pages one swipe away at the edges of a phone's view, in the learner's language", () => {
    atViewport(375, 812);
    for (const [locale, right, down] of [["en", "Go right to Quiz physics", "Go down to Quiz heating"], ["de", "Nach rechts zu Quiz physics", "Nach unten zu Quiz heating"]] as const) {
      const { unmount } = render(<HomeScreen session={stubSession() as unknown as QuizSession} state={STATE} text={quizText(locale)} locale={locale} preferences={PREFERENCES} onPreferences={() => undefined} />);
      const hints = [...document.querySelectorAll<HTMLElement>("[data-layered-neighbour]")].map((hint) => [hint.dataset.layeredNeighbour, hint.dataset.pane, hint.getAttribute("aria-label"), hint.textContent?.trim()]);
      expect(hints.map(([direction, page]) => [direction, page]), `${locale}: the ring wraps, so the badges lie above the learner and how it works on the left`).toEqual([
        ["up", "badges"],
        ["left", "intro"],
        ["right", "physics"],
        ["down", "heating"],
      ]);
      expect(hints.slice(2), locale).toEqual([
        ["right", "physics", right, "Quiz physics"],
        ["down", "heating", down, "Quiz heating"],
      ]);
      unmount();
    }
  });

  it("lets a card the learner swipes to scroll inside its page's cell instead of clipping it when the viewport is short", () => {
    atViewport(700, 360);
    render(<Home session={stubSession()} />);
    const layers = [...document.querySelectorAll<HTMLElement>("[data-layered-cell]")].filter((layer) => layer.querySelector("[data-layered-card]") !== null);
    expect(layers).toHaveLength(PAGES.length);
    for (const layer of layers) {
      const classes = layer.className.split(" ");
      expect(classes).toContain("overflow-y-auto");
      expect(classes).toContain("items-center-safe");
      expect(layer.style.paddingBottom, "room for the hint below, in px so large text never narrows it").toMatch(/^calc\((0px \+ )?\d+px\)$/u);
      expect(classes.some((name) => name.endsWith("rem]"))).toBe(false);
    }
  });
});
