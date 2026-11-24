import * as React from "react";
import { Canvas, Layout } from "@semio-tech/ui-react";
import { cleanup, render } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { chromeHostedOpenPanelPositionStyle } from "../../../🔨️modules/⚓️panel-anchor/🟦️.ts";
import { chromePanelSafeArea, chromePanelSafeAreaStyle, publishShellChromePanelBox, useShellChromePanelBoxes } from "../../../🔨️modules/🛟️chrome-safe-area/🟦️.tsx";

afterEach(cleanup);

const chips = {
  start: <span data-testid="start">Start</span>,
  center: <span data-testid="center">Center</span>,
  end: <span data-testid="end">End</span>,
};

describe("canvas bottom edge chips", () => {
  it("renders no chip row without chips", () => {
    const { container } = render(<Canvas>content</Canvas>);
    expect(container.querySelector('[data-slot="canvas-footer"]')).toBeNull();
  });

  it("overlays start, center and end chips inside the canvas rectangle on its bottom edge", () => {
    const { container } = render(<Canvas bottomChips={chips}>content</Canvas>);
    const canvas = container.querySelector('[data-slot="canvas"]') as HTMLElement;
    const row = canvas.querySelector('[data-slot="canvas-footer"]') as HTMLElement;
    expect(row).toBeTruthy();
    expect(row.getAttribute("data-dock")).toBe("bottom");
    expect(row.className).toMatch(/\babsolute\b/u);
    expect(row.className).toMatch(/\bbottom-0\b/u);
    expect(row.className).toMatch(/\bpointer-events-none\b/u);
    expect(canvas.className).toMatch(/\brelative\b/u);
    for (const placement of ["start", "center", "end"] as const) {
      const slot = row.querySelector(`[data-slot="canvas-footer-${placement}"]`) as HTMLElement;
      expect(slot.hasAttribute("data-canvas-chip")).toBe(true);
      expect(slot.getAttribute("data-dock")).toBe("bottom");
      expect(slot.className).toMatch(/\bpointer-events-auto\b/u);
      expect(slot.querySelector(`[data-testid="${placement}"]`)).toBeTruthy();
    }
    expect(row.children).toHaveLength(3);
  });

  it("keeps the content full-bleed: the chip row is an overlay after the content, never a band", () => {
    const { container } = render(<Canvas bottomChips={{ end: chips.end }}>content</Canvas>);
    const canvas = container.querySelector('[data-slot="canvas"]') as HTMLElement;
    expect(canvas.className).toMatch(/\bp-0\b/u);
    const row = canvas.querySelector('[data-slot="canvas-footer"]') as HTMLElement;
    expect(canvas.firstChild?.textContent).toBe("content");
    expect(canvas.lastElementChild).toBe(row);
    expect(row.children).toHaveLength(1);
    expect(row.querySelector('[data-slot="canvas-footer-end"]')).toBeTruthy();
  });

  it("places the chips of a layout inside the canvas viewport, not in the footer band", () => {
    const { container } = render(<Layout footer={<div data-testid="footer" />} canvas={<div data-testid="canvas-body" />} canvasChips={chips} />);
    const row = container.querySelector('[data-slot="canvas-footer"]') as HTMLElement;
    expect(row).toBeTruthy();
    const column = container.querySelector('[data-slot="layout-canvas-column"]') as HTMLElement;
    expect(column.contains(row)).toBe(true);
    expect(row.parentElement?.contains(container.querySelector('[data-testid="canvas-body"]'))).toBe(true);
    const footer = container.querySelector('[data-testid="footer"]') as HTMLElement;
    expect(footer.contains(row)).toBe(false);
    expect(footer.parentElement?.contains(row)).toBe(false);
  });

  it("opens a chrome-hosted bottom panel cap in place over the chip row instead of overhanging a footer band", () => {
    const bottom = chromeHostedOpenPanelPositionStyle("bottom-right") as Record<string, unknown>;
    expect(bottom.bottom).toBe("0px");
    expect(bottom.maxHeight).toBe("calc(100% - (0px * 2))");
    expect(bottom["--panel-header-height"]).toBe("var(--footer-band-height)");
    const top = chromeHostedOpenPanelPositionStyle("top-left") as Record<string, unknown>;
    expect(top.top).toBe("calc(-1 * var(--navbar-band-height))");
  });

  it("publishes every chip slot as an occupied chrome box and retracts it on unmount", () => {
    const originalRect = Element.prototype.getBoundingClientRect;
    Element.prototype.getBoundingClientRect = function stubbedRect(this: Element): DOMRect {
      if (this.getAttribute("data-slot") !== "canvas-footer-start") return originalRect.call(this);
      return { x: 0, y: 947, width: 208, height: 24, top: 947, right: 208, bottom: 971, left: 0, toJSON: () => ({}) } as DOMRect;
    };
    let seen: readonly { readonly anchor: string; readonly box: { readonly left: number; readonly top: number; readonly right: number; readonly bottom: number } }[] = [];
    const Probe: React.FC = () => {
      seen = useShellChromePanelBoxes();
      return null;
    };
    try {
      const { unmount } = render(
        <>
          <Probe />
          <Canvas bottomChips={chips}>content</Canvas>
        </>,
      );
      expect(seen.map((entry) => entry.anchor).sort()).toEqual(["bottom-left", "bottom-middle", "bottom-right"]);
      expect(seen.find((entry) => entry.anchor === "bottom-left")?.box).toEqual({ left: 0, top: 947, right: 208, bottom: 971 });
      unmount();
      render(<Probe />);
      expect(seen).toHaveLength(0);
    } finally {
      Element.prototype.getBoundingClientRect = originalRect;
    }
  });

  it("lifts bottom-anchored window chrome above the chip row at any label width", () => {
    const host = { left: 0, top: 0, right: 1600, bottom: 971 };
    const chipRow = [{ left: 0, top: 947, right: 208, bottom: 971 }];
    for (const width of [74, 87, 140]) {
      const toggle = { left: 0, top: 947, right: width, bottom: 971 };
      const reserve = chromePanelSafeArea(toggle, host, "bottom-left", chipRow, "either", -0.5);
      expect(reserve).toEqual({ inlinePx: 0, blockPx: 24 });
      expect(chromePanelSafeAreaStyle("bottom-left", reserve, { block: "0px", inline: "0px" })).toEqual({ bottom: "calc(0px + 24px)", left: "0px" });
    }
    expect(chromePanelSafeArea({ left: 400, top: 947, right: 480, bottom: 971 }, host, "bottom-left", chipRow, "either", -0.5)).toEqual({ inlinePx: 0, blockPx: 0 });
  });
});
