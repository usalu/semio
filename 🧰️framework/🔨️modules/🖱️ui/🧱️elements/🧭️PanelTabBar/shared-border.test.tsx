import * as React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { PanelTabBar, singleTreeLeaf, type PanelTabNode } from "./🟦️.tsx";
import { panelTabBarEdge } from "../../🔨️modules/🎨️chrome-class-names/🟦️.ts";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

const Stub = (): null => null;
const tabs: PanelTabNode[] = ["a", "b", "c"].map((id) => singleTreeLeaf({ id: `tab-${id}`, icon: Stub, name: `Tab ${id}`, tree: { sections: [] } }));
const WIDTH = 200;
const STRIP = 30;
const BODY = 100;
const NATIVE_EDGE = /(^|\s)border(-[tblrxyse])?(\s|$)/;
const SCROLL_WIDTH = 600;

type Point = { x: number; y: number };

function points(path: SVGPathElement, origin: Point): Point[] {
  return [...(path.getAttribute("d") ?? "").matchAll(/[ML]\s*([\d.e+-]+)\s+([\d.e+-]+)/gi)].map((match) => ({ x: Number(match[1]) + origin.x, y: Number(match[2]) + origin.y }));
}

function segments(frame: HTMLElement, origin: Point) {
  return [...frame.querySelectorAll<SVGPathElement>(":scope > svg[data-shared-border-overlay] [data-shared-border-segment]")].flatMap((path) => {
    const list = points(path, origin);
    return list.slice(1).map((point, index) => ({ from: list[index]!, to: point }));
  });
}

function rectOf(element: HTMLElement, strip: Point): DOMRect {
  const slot = element.dataset.slot;
  if (slot === "panel-tab-frame" || slot === "mobile-panel-tab-frame" || slot === "panel-tabs" || slot === "mobile-panel-tabs") return new DOMRect(strip.x, strip.y, WIDTH, STRIP);
  if (slot === "panel-body") return new DOMRect(0, STRIP, WIDTH, BODY);
  if (slot === "panel-host") return new DOMRect(0, 0, WIDTH, STRIP + BODY);
  return new DOMRect();
}

function mockRects(strip: Point) {
  vi.spyOn(HTMLElement.prototype, "offsetWidth", "get").mockImplementation(function (this: HTMLElement) { return rectOf(this, strip).width; });
  vi.spyOn(HTMLElement.prototype, "offsetHeight", "get").mockImplementation(function (this: HTMLElement) { return rectOf(this, strip).height; });
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) { return rectOf(this, strip); });
}

function Host({ children, host = false }: { children: React.ReactNode; host?: boolean }) {
  return (
    <div className="semio-scope" style={{ position: "relative" }}>
      {host ? (
        <div data-slot="panel-host" data-shared-border-frame="">
          {children}
          <div data-slot="panel-body" data-shared-border-frame="" data-shared-border-edges="left right bottom" />
        </div>
      ) : (
        children
      )}
    </div>
  );
}

describe("panel tab strip shares one seam with its panel body", () => {
  it("declares only the content-facing edge, and only while expanded", () => {
    expect(panelTabBarEdge("panel", "down", true)).toBe("bottom");
    expect(panelTabBarEdge("panel", "up", true)).toBe("top");
    expect(panelTabBarEdge("panel", "down", false)).toBeUndefined();
    expect(panelTabBarEdge("chrome", "up", false)).toBeUndefined();
    expect(panelTabBarEdge("mobile", "down", true)).toBe("bottom");
    expect(panelTabBarEdge("mobile", "up", false)).toBe("bottom");
  });

  it("hosts the edge on a non-scrolling frame and leaves the scroller border-free with one divider per adjacent tab pair", () => {
    const { container } = render(<Host><PanelTabBar variant="panel" tabs={tabs} activePath={["tab-a"]} onActivePathChange={() => {}} /></Host>);
    const frame = container.querySelector<HTMLElement>('[data-slot="panel-tab-frame"]')!;
    const scroller = container.querySelector<HTMLElement>('[data-slot="panel-tabs"]')!;
    expect(frame.getAttribute("data-shared-border-edges")).toBe("bottom");
    expect(frame.hasAttribute("data-shared-border-frame")).toBe(true);
    expect(frame.className).not.toMatch(/overflow/);
    expect(frame.className).not.toMatch(NATIVE_EDGE);
    expect(scroller.parentElement).toBe(frame);
    expect(scroller.className).toContain("overflow-x-auto");
    expect(scroller.className).not.toMatch(NATIVE_EDGE);
    expect(scroller.hasAttribute("data-shared-border-frame")).toBe(false);
    const cells = scroller.querySelectorAll("[data-shared-border-cell]");
    expect(cells).toHaveLength(tabs.length);
    expect([...cells].every((cell) => !NATIVE_EDGE.test(cell.className))).toBe(true);
  });

  it("registers a collapsed strip without any edge so the chrome frame owns every side", () => {
    const { container } = render(<Host><PanelTabBar variant="panel" tabs={tabs} activePath={["tab-a"]} onActivePathChange={() => {}} showActiveColor={false} /></Host>);
    const frame = container.querySelector<HTMLElement>('[data-slot="panel-tab-frame"]')!;
    expect(frame.hasAttribute("data-shared-border-frame")).toBe(false);
    expect(frame.hasAttribute("data-shared-border-edges")).toBe(false);
  });

  it("mirrors the edge to the top for bottom-docked panels", () => {
    const { container } = render(<Host><PanelTabBar variant="panel" direction="up" tabs={tabs} activePath={["tab-a"]} onActivePathChange={() => {}} /></Host>);
    expect(container.querySelector('[data-slot="panel-tab-frame"]')!.getAttribute("data-shared-border-edges")).toBe("top");
  });

  it("paints exactly one bottom hairline across the full strip, and the panel body adds no second owner at the seam", async () => {
    mockRects({ x: 0, y: 0 });
    const { container } = render(<Host host><PanelTabBar variant="panel" tabs={tabs} activePath={["tab-a"]} onActivePathChange={() => {}} /></Host>);
    const frame = container.querySelector<HTMLElement>('[data-slot="panel-tab-frame"]')!;
    const body = container.querySelector<HTMLElement>('[data-slot="panel-body"]')!;
    const host = container.querySelector<HTMLElement>('[data-slot="panel-host"]')!;
    await waitFor(() => expect(segments(frame, { x: 0, y: 0 }).length).toBeGreaterThan(0));
    const seamOwners = [frame, body, host].filter((element) => segments(element, element === body ? { x: 0, y: STRIP } : { x: 0, y: 0 }).some(({ from, to }) => from.y === to.y && Math.abs(from.y - STRIP) <= 2 && Math.abs(to.x - from.x) > WIDTH / 2));
    expect(seamOwners).toHaveLength(1);
    const own = segments(frame, { x: 0, y: 0 });
    expect(own.every(({ from, to }) => from.y === to.y)).toBe(true);
    const covered = own.reduce((sum, { from, to }) => sum + Math.abs(to.x - from.x), 0);
    expect(covered).toBeGreaterThanOrEqual(WIDTH - 2);
    expect(Math.min(...own.flatMap(({ from, to }) => [from.x, to.x]))).toBeLessThanOrEqual(1);
    expect(Math.max(...own.flatMap(({ from, to }) => [from.x, to.x]))).toBeGreaterThanOrEqual(WIDTH - 1);
  });

  it("keeps outer tab-bar sides undrawn so the hosting panel frame owns them", async () => {
    mockRects({ x: 0, y: 0 });
    const { container } = render(<Host host><PanelTabBar variant="panel" tabs={tabs} activePath={["tab-a"]} onActivePathChange={() => {}} /></Host>);
    const frame = container.querySelector<HTMLElement>('[data-slot="panel-tab-frame"]')!;
    const host = container.querySelector<HTMLElement>('[data-slot="panel-host"]')!;
    await waitFor(() => expect(segments(host, { x: 0, y: 0 }).length).toBeGreaterThan(0));
    expect(segments(frame, { x: 0, y: 0 }).some(({ from, to }) => from.x === to.x && from.y !== to.y)).toBe(false);
    expect(segments(frame, { x: 0, y: 0 }).some(({ from, to }) => from.y === to.y && from.y <= 1)).toBe(false);
    const left = segments(host, { x: 0, y: 0 }).filter(({ from, to }) => from.x === to.x && Math.min(from.x, to.x) <= 1 && Math.abs(to.y - from.y) > 0);
    expect(Math.min(...left.flatMap(({ from, to }) => [from.y, to.y]))).toBeLessThanOrEqual(1);
  });

  it("keeps the overlay anchored to the frame while overflowing tabs scroll", async () => {
    mockRects({ x: 0, y: 0 });
    const { container } = render(<Host><PanelTabBar variant="panel" tabs={tabs} activePath={["tab-a"]} onActivePathChange={() => {}} /></Host>);
    const frame = container.querySelector<HTMLElement>('[data-slot="panel-tab-frame"]')!;
    const scroller = container.querySelector<HTMLElement>('[data-slot="panel-tabs"]')!;
    Object.defineProperty(scroller, "scrollWidth", { configurable: true, value: SCROLL_WIDTH });
    Object.defineProperty(scroller, "clientWidth", { configurable: true, value: WIDTH });
    await waitFor(() => expect(frame.querySelector(":scope > svg[data-shared-border-overlay]")).not.toBeNull());
    const before = frame.querySelector("svg")!.innerHTML;
    scroller.scrollLeft = SCROLL_WIDTH - WIDTH;
    fireEvent.scroll(scroller);
    await new Promise((resolve) => setTimeout(resolve, 50));
    expect(scroller.querySelector("svg")).toBeNull();
    expect(frame.querySelector(":scope > svg[data-shared-border-overlay]")).not.toBeNull();
    expect(frame.querySelector("svg")!.innerHTML).toBe(before);
    const own = segments(frame, { x: 0, y: 0 });
    expect(Math.max(...own.flatMap(({ from, to }) => [from.x, to.x]))).toBeGreaterThanOrEqual(WIDTH - 1);
    expect(Math.min(...own.flatMap(({ from, to }) => [from.x, to.x]))).toBeLessThanOrEqual(1);
  });

  it("gives the mobile strip its bottom edge in the same single-owner way", () => {
    const { container } = render(<Host><PanelTabBar variant="mobile" tabs={tabs} activePath={["tab-a"]} onActivePathChange={() => {}} showActiveColor={false} /></Host>);
    const frame = container.querySelector<HTMLElement>('[data-slot="mobile-panel-tab-frame"]')!;
    expect(frame.getAttribute("data-shared-border-edges")).toBe("bottom");
    expect(frame.className).toContain("w-full");
    expect(container.querySelector('[data-slot="mobile-panel-tabs"]')!.className).not.toMatch(NATIVE_EDGE);
  });
});
