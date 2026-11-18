import { render } from "@testing-library/react";
import * as React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { OverviewCard, OverviewCardAction } from "../🃏️OverviewCard/🟦️.tsx";
import { CardGrid, CardGridCell, CardStack } from "./🟦️.tsx";

const rect = (left: number, width: number, height: number) => ({ left, right: left + width, width, height, top: 0, bottom: height, x: left, y: 0, toJSON: () => ({}) }) as DOMRect;

function measure(rows: Record<string, number>, chips: Record<string, readonly [number, number]>) {
  return vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
    const slot = this.getAttribute("data-slot") ?? "";
    if (this.hasAttribute("data-window-silhouette-chip")) { const [left, right] = chips[slot] ?? [0, 0]; return rect(left, right - left, 28); }
    return rect(0, 400, rows[slot] ?? 0);
  });
}

const card = (id: string, props: { footerLeft?: React.ReactNode; footerRight?: React.ReactNode } = {}) => (
  <OverviewCard key={id} as="section" slot="c" headingId={id} icon={null} title={id} {...props}>
    <p>{id}</p>
  </OverviewCard>
);
const vars = (element: Element) => ["--card-stack-lift", "--card-stack-cap", "--card-stack-footer"].map(name => (element as HTMLElement).style.getPropertyValue(name));

describe("CardStack", () => {
  afterEach(() => vi.restoreAllMocks());

  it("lifts a card onto the footer band of the card above and sizes both rows to the band", () => {
    measure({ "window-chrome-cap": 28, "window-chrome-footer": 28 }, { "window-chrome-footer-right": [300, 400], "window-chrome-chip-cap": [0, 100] });
    const { container } = render(<CardStack>{card("a", { footerRight: <OverviewCardAction primary>Next</OverviewCardAction> })}{card("b")}</CardStack>);
    const [a, b] = [...container.querySelectorAll("[data-overview-card]")];
    expect(vars(b!)).toEqual(["28px", "28px", "0px"]);
    expect(vars(a!)).toEqual(["0px", "0px", "28px"]);
  });

  it("stacks the rows when the chips would collide and does not lift when there is no footer", () => {
    measure({ "window-chrome-cap": 28, "window-chrome-footer": 28 }, { "window-chrome-footer-right": [50, 400], "window-chrome-chip-cap": [0, 100] });
    const { container } = render(<CardStack>{card("a", { footerRight: <OverviewCardAction primary>Next</OverviewCardAction> })}{card("b")}{card("c")}</CardStack>);
    const [a, b, c] = [...container.querySelectorAll("[data-overview-card]")];
    expect([vars(a!), vars(b!), vars(c!)]).toEqual([["0px", "0px", "0px"], ["0px", "0px", "0px"], ["0px", "0px", "0px"]]);
  });

  it("is a gapless, paddingless column", () => {
    const { container } = render(<CardStack maxWidth="56rem">{card("a")}</CardStack>);
    const stack = container.querySelector("[data-card-stack]") as HTMLElement;
    expect(stack.className).not.toMatch(/\b(gap|p|px|py|pt|pb)-/);
    expect(stack.style.maxWidth).toBe("56rem");
  });
});

describe("CardGrid", () => {
  it("interlocks the cards of consecutive rows through their cells", () => {
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      const slot = this.getAttribute("data-slot") ?? "";
      if (this.hasAttribute("data-window-silhouette-chip")) return slot === "window-chrome-footer-right" ? rect(300, 100, 28) : rect(0, 100, 28);
      if (this.hasAttribute("data-overview-card")) return this.getAttribute("data-card") === "top" ? rect(0, 400, 200) : { ...rect(0, 400, 200), top: 200, bottom: 400 } as DOMRect;
      return rect(0, 400, slot.startsWith("window-chrome-c") || slot === "window-chrome-footer" ? 28 : 0);
    });
    const cell = (id: string, row: number, footer?: boolean) => (
      <CardGridCell key={id} row={row} column={0}>
        <OverviewCard as="section" slot="c" headingId={id} icon={null} title={id} data={{ "data-card": id }} footerRight={footer ? <OverviewCardAction primary>Go</OverviewCardAction> : undefined}><p>{id}</p></OverviewCard>
      </CardGridCell>
    );
    const { container } = render(<CardGrid columns="1fr" rows="auto auto">{cell("top", 0, true)}{cell("bottom", 1)}</CardGrid>);
    const [top, bottom] = [...container.querySelectorAll<HTMLElement>("[data-card-grid-cell]")];
    expect([top!.style.getPropertyValue("--card-stack-lift"), bottom!.style.getPropertyValue("--card-stack-lift")]).toEqual(["0px", "28px"]);
  });

  it("places cells explicitly and flows the rest, with tracks as custom properties", () => {
    const { container } = render(<CardGrid columns="1fr 2fr" rows="auto"><CardGridCell column={1} row={0}>x</CardGridCell><CardGridCell>y</CardGridCell></CardGrid>);
    const grid = container.querySelector("[data-card-grid]") as HTMLElement;
    expect(grid.style.getPropertyValue("--card-grid-columns")).toBe("1fr 2fr");
    const [first, second] = [...container.querySelectorAll<HTMLElement>("[data-card-grid-cell]")];
    expect([first!.getAttribute("style"), second!.getAttribute("style")]).toEqual(["grid-column: 2; grid-row: 1;", null]);
  });
});
