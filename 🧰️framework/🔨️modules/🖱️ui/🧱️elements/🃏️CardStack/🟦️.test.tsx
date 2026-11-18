import { render } from "@testing-library/react";
import * as React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { OverviewCard, OverviewCardAction } from "../🃏️OverviewCard/🟦️.tsx";
import { CardGrid, CardGridCell, CardStack } from "./🟦️.tsx";

const rect = (left: number, width: number, height: number) => ({ left, right: left + width, width, height, top: 0, bottom: height, x: left, y: 0, toJSON: () => ({}) }) as DOMRect;

const shifted = (element: HTMLElement, box: DOMRect): DOMRect => {
  let by = 0;
  for (let node: Element | null = element.closest<HTMLElement>("[data-card-grid-cell]") ?? element; node; node = node.previousElementSibling) by += Number.parseFloat((node as HTMLElement).style.marginBlockStart || "0");
  return by ? { ...box, top: box.top + by, bottom: box.bottom + by } as DOMRect : box;
};

function measure(rows: Record<string, number>, chips: Record<string, readonly [number, number]>) {
  return vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
    const slot = this.getAttribute("data-slot") ?? "";
    if (this.hasAttribute("data-window-silhouette-chip")) { const [left, right] = chips[slot] ?? [0, 0]; return rect(left, right - left, 28); }
    if (!this.hasAttribute("data-overview-card")) return rect(0, 400, rows[slot] ?? 0);
    const top = 100 * "abc".indexOf(this.getAttribute("data-card") ?? "a");
    return shifted(this, { ...rect(0, 400, 100), top, bottom: top + 100 } as DOMRect);
  });
}

const card = (id: string, props: { footerLeft?: React.ReactNode; footerRight?: React.ReactNode } = {}) => (
  <OverviewCard key={id} as="section" slot="c" headingId={id} icon={null} title={id} data={{ "data-card": id }} {...props}>
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
    expect(vars(b!)).toEqual(["30px", "28px", "0px"]);
    expect(vars(a!)).toEqual(["0px", "0px", "28px"]);
  });

  it("stacks the rows when the chips would collide and meets on one line when there is no footer", () => {
    measure({ "window-chrome-cap": 28, "window-chrome-footer": 28 }, { "window-chrome-footer-right": [50, 400], "window-chrome-chip-cap": [0, 100] });
    const { container } = render(<CardStack>{card("a", { footerRight: <OverviewCardAction primary>Next</OverviewCardAction> })}{card("b")}{card("c")}</CardStack>);
    const [a, b, c] = [...container.querySelectorAll("[data-overview-card]")];
    expect([vars(a!), vars(b!), vars(c!)]).toEqual([["0px", "0px", "0px"], ["2px", "0px", "0px"], ["2px", "0px", "0px"]]);
  });

  it("snaps a card to whole device pixels and absorbs a sub-pixel drift between the card above and the lifted card", () => {
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      if (this.hasAttribute("data-window-silhouette-chip")) return rect(0, 100, 28);
      if (this.hasAttribute("data-overview-card")) return shifted(this, this.getAttribute("data-card") === "a" ? { ...rect(0, 400, 69.59), top: 0, bottom: 69.59 } as DOMRect : { ...rect(0, 400, 50), top: 69.1, bottom: 119.1 } as DOMRect);
      return rect(0, 400, 0);
    });
    const { container } = render(<CardStack>{[["a"], ["b"]].map(([id]) => <OverviewCard key={id} as="section" slot="c" headingId={id!} icon={null} title={id!} data={{ "data-card": id! }}><p>{id}</p></OverviewCard>)}</CardStack>);
    const [a, b] = [...container.querySelectorAll<HTMLElement>("[data-overview-card]")];
    expect(a!.style.getPropertyValue("min-block-size")).toBe("70px");
    expect(b!.style.getPropertyValue("margin-block-start")).toBe("-1.51px");
  });

  it("re-reads the seam after the lift moved the tracks and settles it to the band, however far the card above reaches into the lifted one", () => {
    const grow = 0.5;
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      if (this.hasAttribute("data-window-silhouette-chip")) return rect(0, 100, 28);
      if (!this.hasAttribute("data-overview-card")) return rect(0, 400, 0);
      const below = this.getAttribute("data-card") === "b";
      const by = -Number.parseFloat(this.style.marginBlockStart || "0");
      return below ? { ...rect(0, 400, 50), top: 100 - by, bottom: 150 - by } as DOMRect : { ...rect(0, 400, 100.8), top: 0, bottom: 100.8 + grow * Number.parseFloat(this.parentElement?.querySelector<HTMLElement>("[data-card='b']")?.style.marginBlockStart.replace("-", "") || "0") } as DOMRect;
    });
    const { container } = render(<CardStack>{[["a"], ["b"]].map(([id]) => <OverviewCard key={id} as="section" slot="c" headingId={id!} icon={null} title={id!} data={{ "data-card": id! }}><p>{id}</p></OverviewCard>)}</CardStack>);
    const [a, b] = [...container.querySelectorAll<HTMLElement>("[data-overview-card]")];
    const lifted = Number.parseFloat(b!.style.marginBlockStart.replace("-", ""));
    const reach = 100.8 + grow * lifted;
    expect(reach - (100 - lifted)).toBeCloseTo(2, 2);
    expect(a!.style.getPropertyValue("min-block-size")).toBe("101px");
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
      if (this.hasAttribute("data-overview-card")) return shifted(this, this.getAttribute("data-card") === "top" ? rect(0, 400, 200) : { ...rect(0, 400, 200), top: 200, bottom: 400 } as DOMRect);
      return rect(0, 400, slot.startsWith("window-chrome-c") || slot === "window-chrome-footer" ? 28 : 0);
    });
    const cell = (id: string, row: number, footer?: boolean) => (
      <CardGridCell key={id} row={row} column={0}>
        <OverviewCard as="section" slot="c" headingId={id} icon={null} title={id} data={{ "data-card": id }} footerRight={footer ? <OverviewCardAction primary>Go</OverviewCardAction> : undefined}><p>{id}</p></OverviewCard>
      </CardGridCell>
    );
    const { container } = render(<CardGrid columns="1fr" rows="auto auto">{cell("top", 0, true)}{cell("bottom", 1)}</CardGrid>);
    const [top, bottom] = [...container.querySelectorAll<HTMLElement>("[data-card-grid-cell]")];
    expect([top!.style.getPropertyValue("--card-stack-lift"), bottom!.style.getPropertyValue("--card-stack-lift")]).toEqual(["0px", "30px"]);
  });

  it("places cells explicitly and flows the rest, with tracks as custom properties", () => {
    const { container } = render(<CardGrid columns="1fr 2fr" rows="auto"><CardGridCell column={1} row={0}>x</CardGridCell><CardGridCell>y</CardGridCell></CardGrid>);
    const grid = container.querySelector("[data-card-grid]") as HTMLElement;
    expect(grid.style.getPropertyValue("--card-grid-columns")).toBe("1fr 2fr");
    const [first, second] = [...container.querySelectorAll<HTMLElement>("[data-card-grid-cell]")];
    expect([first!.getAttribute("style"), second!.getAttribute("style")]).toEqual(["grid-column: 2; grid-row: 1;", null]);
  });
});
