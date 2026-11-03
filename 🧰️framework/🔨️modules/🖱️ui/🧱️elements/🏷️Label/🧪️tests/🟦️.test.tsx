// @vitest-environment jsdom
import * as React from "react";
import { act, cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { calc } from "@csstools/css-calc";
import vectors from "./🔣️.json";
import { resolvePropertyRowStacked, type PropertyLayout } from "../📐️layout/🟦️.ts";
import { Label } from "../🟦️.tsx";
import { TreeContext } from "../../🌳️Tree/📏️row-alignment/🟦️.tsx";

let width = 280;
let labelWidth = 29;
let resize: () => void;
vi.stubGlobal("ResizeObserver", class {
  constructor(callback: () => void) { resize = callback; }
  observe() {}
  disconnect() {}
});
vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => { callback(0); return 1; });
vi.stubGlobal("cancelAnimationFrame", () => {});
vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
  const slot = this.dataset.slot;
  const measured = slot === "property-row" ? width : slot === "property-label" ? labelWidth : 96;
  return { width: measured, height: 22.4, x: 0, y: 0, top: 0, bottom: 22.4, left: 0, right: measured, toJSON() {} };
});
Object.defineProperty(HTMLElement.prototype, "scrollWidth", { configurable: true, get() { return this.dataset.slot === "property-label" ? labelWidth : 1000; } });
afterEach(() => { cleanup(); width = 280; labelWidth = 29; });

describe("property label layout", () => {
  it("matches neutral vectors and an independent CSS arithmetic oracle", () => {
    for (const vector of vectors.cases) {
      const input = { ...vector, mode: vector.mode as PropertyLayout };
      const required = Number.parseFloat(calc(`calc(max(${vector.label}px, ${vector.track}px) + ${vector.control}px + ${vector.gap}px + ${vector.wasStacked ? vector.hysteresis : 0}px)`));
      const oracle = vector.mode === "stacked" || (vector.mode === "auto" && required > vector.width);
      expect(oracle, vector.name).toBe(vector.stacked);
      expect(resolvePropertyRowStacked(input), vector.name).toBe(oracle);
    }
  });

  it("ignores allocated wrapper widths, centers inline and removes stacked indentation", () => {
    const { container, getByLabelText } = render(<Label id="width" label="Width"><input id="width" defaultValue="240" /></Label>);
    expect(getByLabelText("Width")).toBe(container.querySelector("input"));
    const row = container.querySelector<HTMLElement>('[data-slot="property-row"]')!;
    const control = container.querySelector<HTMLElement>('[data-slot="property-control"]')!;
    expect(row.dataset.propertyLayout).toBe("inline");
    expect(row.style.alignItems).toBe("center");
    width = 180;
    act(() => resize());
    expect(row.dataset.propertyLayout).toBe("stacked");
    expect(control.style.paddingInlineStart).toBe("");
    width = 198;
    act(() => resize());
    expect(row.dataset.propertyLayout).toBe("stacked");
    width = 280;
    act(() => resize());
    expect(row.dataset.propertyLayout).toBe("inline");
    fireEvent.click(container.querySelector('[data-slot="property-label"]')!);
    expect(document.activeElement).toBe(container.querySelector("input"));
  });

  it("keeps nested tree labels and stacked controls free of value-column indentation", () => {
    width = 180;
    const { container, getByLabelText } = render(
      <TreeContext.Provider value={{ level: 2, isLastAtLevel: [], showLines: false, isTree: true, indentMultiplier: 1 }}>
        <Label id="tree-width" label="Breite"><input id="tree-width" /></Label>
      </TreeContext.Provider>,
    );
    expect(getByLabelText("Breite")).toBe(container.querySelector("input"));
    expect(container.querySelector<HTMLElement>('[data-slot="property-row"]')!.dataset.propertyLayout).toBe("stacked");
    expect(container.querySelector<HTMLElement>('[data-slot="property-control"]')!.style.paddingInlineStart).toBe("");
    expect(container.querySelector<HTMLElement>('[data-slot="property-label-tree"]')!.className).toContain("flex");
  });

  it("remeasures translated content, preserves customization and supports forced layouts", () => {
    const { container, rerender } = render(<Label label="Width" className="custom-row"><input /></Label>);
    const row = container.querySelector<HTMLElement>('[data-slot="property-row"]')!;
    labelWidth = 200;
    rerender(<Label label="Ausführliche Breitenbezeichnung" className="custom-row"><input /></Label>);
    expect(row.dataset.propertyLayout).toBe("stacked");
    expect(row.className).toContain("custom-row");
    rerender(<Label label="Breite" propertyLayout="inline"><input /></Label>);
    expect(row.dataset.propertyLayout).toBe("inline");
    rerender(<Label label="Breite" propertyLayout="stacked"><input /></Label>);
    expect(row.dataset.propertyLayout).toBe("stacked");
  });
});
