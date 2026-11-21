/** 🟢️ The home overview tests of the 🏠️ home overview suite. */
import { render } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { FollowProvider, homeCells, homeLayout } from "@semio-tech/course-react";
import { quizStylesheet as stylesheet } from "@semio-tech/course-react/stylesheet";
import { Home, PAGES, atViewport, cardHost, fixture, stubSession } from "../../🛠️home-grid-kit/🟦️.tsx";
import { layerAt, layerRules, weights } from "../🟣️harness/🟦️.tsx";

/** 🔵 Registers the home overview tests of the 🏠️ home overview suite. */
export function registerHomeOverviewTests(): void {
  it("does not announce the settled swipe page again when the overview rerenders", () => {
    atViewport(375);
    const session = stubSession();
    const onSwipe = vi.fn();
    const ignore = () => undefined;
    const tree = () => (
      <FollowProvider onCamera={ignore} onRevealed={ignore} onSubmission={ignore} onSort={ignore} onTaskPage={ignore} onSwipe={onSwipe}>
        <Home session={session} />
      </FollowProvider>
    );
    const { rerender } = render(tree());
    expect(onSwipe.mock.calls).toEqual([[PAGES[0]]]);
    rerender(tree());
    expect(onSwipe.mock.calls).toEqual([[PAGES[0]]]);
  });
  it("offers a direct page picker in a short touch landscape viewport and mounts only the active grid", () => {
    atViewport(812, 375, true);
    render(<Home session={stubSession()} />);
    const grids = document.querySelectorAll("[data-course-grid]");
    expect(grids).toHaveLength(1);
    expect(grids[0]?.querySelector("[data-layered-overview]")?.getAttribute("data-mode")).toBe("swipe");
    const picker = document.querySelector("form.course-page-picker");
    expect(picker?.getAttribute("data-shared-border-edges")).toBe("bottom");
    expect(picker?.querySelectorAll(":scope > :is(select, button)[data-shared-border-cell]")).toHaveLength(2);
  });
  for (const expected of fixture.layouts) {
    it(`lays the cards over the ${expected.grid} grid at ${expected.width} px, ${expected.mode === "swipe" ? "swiped one page at a time" : "fixed over the pages"}`, () => {
      const { mobileMaxPx, tabletMaxPx } = fixture.breakpoints;
      expect(homeLayout({ narrow: expected.width <= mobileMaxPx, medium: expected.width > mobileMaxPx && expected.width <= tabletMaxPx })).toEqual({ grid: expected.grid, mode: expected.mode });
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
}
