import { render } from "@testing-library/react";
import * as React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { OverviewCard, OverviewCardAction } from "../🃏️OverviewCard/🟦️.tsx";
import { CardGrid, CardGridCell, CardStack } from "./🟦️.tsx";

const rect = (top: number, height: number) => ({ left: 0, right: 400, width: 400, height, top, bottom: top + height, x: 0, y: top, toJSON: () => ({}) }) as DOMRect;

function measure(heights: Record<string, number>) {
  return vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
    const id = this.getAttribute("data-card") ?? this.closest("[data-card]")?.getAttribute("data-card") ?? "";
    return rect(0, heights[id] ?? 0);
  });
}

const card = (id: string, props: { footerLeft?: React.ReactNode; footerRight?: React.ReactNode } = {}) => (
  <OverviewCard key={id} as="section" slot="c" headingId={id} icon={null} title={id} data={{ "data-card": id }} {...props}>
    <p>{id}</p>
  </OverviewCard>
);

describe("CardStack", () => {
  afterEach(() => vi.restoreAllMocks());

  it("stacks cards as abutting rectangles: chips are inside each card, no lift, band or margin exists", () => {
    measure({ a: 100, b: 100 });
    const { container } = render(<CardStack>{card("a", { footerRight: <OverviewCardAction primary>Next</OverviewCardAction> })}{card("b")}</CardStack>);
    for (const element of container.querySelectorAll<HTMLElement>("[data-overview-card], [data-overview-card-stack]")) {
      expect(element.getAttribute("style") ?? "").not.toMatch(/--card-stack|margin/u);
    }
    const first = container.querySelector("[data-card='a']")!;
    expect(first.querySelector("[data-window-frame]")).not.toBeNull();
    expect(first.querySelector("[data-window-frame-chip]")).not.toBeNull();
    expect(first.querySelector("[data-window-frame-chip][data-dock='bottom']")).not.toBeNull();
  });

  it("snaps a card to whole device pixels so two abutting lines fall on one pixel", () => {
    measure({ a: 69.59, b: 50 });
    const { container } = render(<CardStack>{card("a")}{card("b")}</CardStack>);
    const [a, b] = [...container.querySelectorAll<HTMLElement>("[data-overview-card]")];
    expect(a!.style.getPropertyValue("min-block-size")).toBe("70px");
    expect(b!.style.getPropertyValue("min-block-size")).toBe("");
  });

  it("settles: further measure passes of an unchanged stack write no style at all", async () => {
    const reads = measure({ a: 69.59, b: 50 });
    const { container } = render(<CardStack>{card("a")}{card("b")}</CardStack>);
    const before = [...container.querySelectorAll("[data-overview-card]")].map(element => element.getAttribute("style"));
    const records: MutationRecord[] = [];
    const observer = new MutationObserver(list => records.push(...list));
    observer.observe(container, { attributes: true, subtree: true });
    reads.mockClear();
    for (let pass = 0; pass < 3; pass++) { window.dispatchEvent(new Event("resize")); await new Promise(resolve => setTimeout(resolve, 40)); }
    records.push(...observer.takeRecords());
    observer.disconnect();
    expect(reads).toHaveBeenCalled();
    expect(records.filter(record => (record.target as Element).matches("[data-overview-card], [data-overview-card-stack]") && record.attributeName === "style")).toEqual([]);
    expect([...container.querySelectorAll("[data-overview-card]")].map(element => element.getAttribute("style"))).toEqual(before);
  });

  it("is a gapless, paddingless column", () => {
    const { container } = render(<CardStack maxWidth="56rem">{card("a")}</CardStack>);
    const stack = container.querySelector("[data-card-stack]") as HTMLElement;
    expect(stack.className).not.toMatch(/\b(gap|p|px|py|pt|pb)-/);
    expect(stack.style.maxWidth).toBe("56rem");
  });
});

describe("CardGrid", () => {
  it("tiles the cards of consecutive rows edge to edge through their cells, without lifting them", () => {
    measure({ top: 200, bottom: 200 });
    const cell = (id: string, row: number) => <CardGridCell key={id} row={row} column={0}>{card(id)}</CardGridCell>;
    const { container } = render(<CardGrid columns="1fr" rows="auto auto">{cell("top", 0)}{cell("bottom", 1)}</CardGrid>);
    for (const element of container.querySelectorAll<HTMLElement>("[data-card-grid-cell]")) expect(element.style.marginBlockStart).toBe("");
  });

  it("places cells explicitly and flows the rest, with tracks as custom properties", () => {
    const { container } = render(<CardGrid columns="1fr 2fr" rows="auto"><CardGridCell column={1} row={0}>x</CardGridCell><CardGridCell>y</CardGridCell></CardGrid>);
    const grid = container.querySelector("[data-card-grid]") as HTMLElement;
    expect(grid.style.getPropertyValue("--card-grid-columns")).toBe("1fr 2fr");
    const [first, second] = [...container.querySelectorAll<HTMLElement>("[data-card-grid-cell]")];
    expect([first!.getAttribute("style"), second!.getAttribute("style")]).toEqual(["grid-column: 2; grid-row: 1;", null]);
  });
});
