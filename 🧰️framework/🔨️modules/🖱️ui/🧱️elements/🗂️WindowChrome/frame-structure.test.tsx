import * as React from "react";
import { cleanup, render } from "@testing-library/react";
import { JSDOM } from "jsdom";
import { afterEach, describe, expect, it } from "vitest";
import { WindowChrome, measureWindowFrameMetrics, windowFrameChipLevel } from "./🟦️.tsx";
import { LEVELS } from "../🌈️Surface/🟦️.tsx";
import { windowFrameLines, windowFrameOutline } from "./shared-border.ts";

const rect = (left: number, top: number, width: number, height: number) => ({ left, top, right: left + width, bottom: top + height, width, height, x: left, y: top, toJSON: () => ({}) }) as DOMRect;

describe("🔲️ a window chrome stack is one rectangle with chip overlays inside it", () => {
  afterEach(cleanup);

  const stack = () => render(<WindowChrome level="window" titleChips={<span>title</span>} capRightChips={<span>controls</span>} body={<p>body</p>} footerLeftChips={<span>left</span>} footerRightChips={<span>right</span>} />).container.querySelector<HTMLElement>("[data-window-frame]")!;

  it("clips to the frame rectangle and fills it with one content plane", () => {
    const root = stack();
    expect(root.className).toMatch(/(?:^|\s)overflow-clip(?:\s|$)/);
    expect(root.querySelectorAll("[data-window-frame-content]")).toHaveLength(1);
    expect(root.querySelector("[data-window-frame-content]")!.className).toContain("window-frame-content-plane");
    expect(root.querySelector("[data-window-frame-content]")!.textContent).toBe("body");
    expect(root.style.getPropertyValue("--window-frame-top-clearance")).toBe("0px");
    expect(root.style.getPropertyValue("--window-frame-bottom-clearance")).toBe("0px");
  });

  it("anchors the chip rows as pointer-transparent overlays and the chips as pointer-active glass one level above the content", () => {
    const root = stack();
    for (const slot of ["window-chrome-cap", "window-chrome-footer"]) {
      const row = root.querySelector<HTMLElement>(`:scope > [data-slot="${slot}"]`)!;
      expect(row.className).toMatch(/(?:^|\s)pointer-events-none(?:\s|$)/);
      expect(row.className).toMatch(/(?:^|\s)absolute(?:\s|$)/);
      expect(row.parentElement).toBe(root);
    }
    const chips = [...root.querySelectorAll<HTMLElement>("[data-window-frame-chip]")];
    expect(chips.map(chip => chip.dataset.dock)).toEqual(["top", "top", "bottom", "bottom"]);
    for (const chip of chips) {
      expect(chip.className).toContain("pointer-events-auto");
      expect(chip.className).toContain("ui-glass");
      expect(chip.dataset.level).toBe("pane");
    }
    expect(root.querySelector("[data-window-frame-content]")!.getAttribute("data-level")).toBe("window");
  });

  it("has no gap element, cutout fill, polygon clip or negative-margin plane", () => {
    const root = stack();
    expect(root.querySelector('[data-window-frame-gap], [data-slot$="-gap"], [data-slot*="-gap-"]')).toBeNull();
    expect(root.outerHTML).not.toMatch(/clip-path|silhouette|margin-block/i);
    const surface = root.querySelector<HTMLElement>('[data-slot="window-chrome-body-surface"]')!;
    expect(surface.className).toMatch(/(?:^|\s)inset-0(?:\s|$)/);
  });

  it("docks the cap row to the bottom edge for bottom-anchored panels", () => {
    const { container } = render(<WindowChrome capDock="bottom" titleChips={<span>t</span>} body={<p>b</p>} />);
    const row = container.querySelector<HTMLElement>('[data-slot="window-chrome-cap"]')!;
    expect(row.className).toMatch(/(?:^|\s)bottom-0(?:\s|$)/);
    expect(container.querySelector<HTMLElement>("[data-window-frame-chip]")!.dataset.dock).toBe("bottom");
  });

  it("renders a chip-only stack as the chip alone, still a frame", () => {
    const { container } = render(<WindowChrome chipOnly titleChips={<span>t</span>} />);
    const root = container.querySelector<HTMLElement>("[data-window-frame]")!;
    expect(root.querySelectorAll("[data-window-frame-chip]")).toHaveLength(1);
    expect(root.querySelector("[data-window-frame-content]")).toBeNull();
  });
});

describe("🪜️ the chip level is one elevation above the content surface", () => {
  it("steps every level up and stays on the topmost one", () => {
    expect(LEVELS.map(windowFrameChipLevel)).toEqual(["window", "pane", "panel", "dialog", "menu", "menu"]);
  });
});

describe("📊️ the frame reports the chip rows it measures", () => {
  it("reads chip spans and the row depth per dock and ignores the space between chips", () => {
    const { document } = new JSDOM('<div id="stack" data-window-frame><div id="cap" data-slot="window-chrome-cap"><div id="a" data-window-frame-chip data-dock="top"></div><div id="b" data-window-frame-chip data-dock="top"></div></div><div id="foot" data-slot="window-chrome-footer"><div id="c" data-window-frame-chip data-dock="bottom"></div></div></div>').window;
    const place = (id: string, box: DOMRect) => Object.defineProperty(document.getElementById(id)!, "getBoundingClientRect", { value: () => box });
    place("stack", rect(0, 0, 200, 100));
    place("cap", rect(0, 0, 200, 24));
    place("a", rect(0, 0, 60, 24));
    place("b", rect(150, 0, 50, 24));
    place("foot", rect(0, 80, 200, 20));
    place("c", rect(40, 80, 80, 20));
    expect(measureWindowFrameMetrics(document.getElementById("stack")!)).toEqual({
      width: 200,
      height: 100,
      top: { depth: 24, chips: [{ left: 0, right: 60 }, { left: 150, right: 200 }] },
      bottom: { depth: 20, chips: [{ left: 40, right: 120 }] },
    });
  });

  it("keeps the chips of a nested frame out of the enclosing frame", () => {
    const { document } = new JSDOM('<div id="stack" data-window-frame><div data-window-frame><div id="inner" data-window-frame-chip data-dock="top"></div></div></div>').window;
    Object.defineProperty(document.getElementById("stack")!, "getBoundingClientRect", { value: () => rect(0, 0, 100, 100) });
    Object.defineProperty(document.getElementById("inner")!, "getBoundingClientRect", { value: () => rect(0, 0, 40, 20) });
    expect(measureWindowFrameMetrics(document.getElementById("stack")!)).toEqual({ width: 100, height: 100, top: { depth: 0, chips: [] }, bottom: { depth: 0, chips: [] } });
  });
});

describe("📏️ the compositor lines of a frame", () => {
  it("are the four frame edges plus the inner edges of each chip region", () => {
    const { paint } = windowFrameOutline({ width: 100, height: 100 });
    expect(paint).toEqual([{ x: 1, y: 1 }, { x: 99, y: 1 }, { x: 99, y: 99 }, { x: 1, y: 99 }]);
    expect(windowFrameLines(paint, [{ x: 0, y: 0, width: 40, height: 20, dock: "top" }])).toEqual({
      vertical: [{ x: 1, top: 1, bottom: 99 }, { x: 99, top: 1, bottom: 99 }, { x: 1, top: 1, bottom: 19 }, { x: 39, top: 1, bottom: 19 }],
      ys: [1, 99, 19],
    });
  });
});
