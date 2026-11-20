/** ⏱️ The sliced survey tests of the 🫥️ decorative layer suite: the layer reads the page a few elements per frame instead of all of it inside one. */
import { render } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { PetLayer } from "@semio-tech/pets-react";
import { MENAGERIE, layerOf, page, place, standing } from "../🧰️decorative-layer-fixtures/🟦️.tsx";

/** 🔷 Registers the sliced survey tests of the 🫥️ decorative layer suite. */
export function registerSlicedSurveyTests(): void {
  it("reads a page of many surfaces a few elements per frame and stages the pets once all of it is read", () => {
    page();
    let reads = 0;
    for (let index = 0; index < 40; index++) {
      const element = place(document.createElement("article"), { left: 20 + (index % 10) * 110, top: 60 + Math.floor(index / 10) * 120, width: 100, height: 60 });
      element.setAttribute("data-pet-surface", "");
      const read = element.getBoundingClientRect;
      element.getBoundingClientRect = () => {
        reads += 1;
        return read.call(element);
      };
      document.body.append(element);
    }
    const base = window.performance.now.bind(window.performance);
    vi.spyOn(window.performance, "now").mockImplementation(() => base() + reads);
    const perFrame: number[] = [];
    const frame = window.requestAnimationFrame.bind(window);
    vi.spyOn(window, "requestAnimationFrame").mockImplementation((callback) =>
      frame((time) => {
        const before = reads;
        callback(time);
        perFrame.push(reads - before);
      }),
    );
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(48);
    expect(standing(layerOf(container))).toEqual([]);
    vi.advanceTimersByTime(5000);
    expect(Math.max(...perFrame)).toBeLessThanOrEqual(8);
    expect(perFrame.filter((count) => count > 0).length).toBeGreaterThanOrEqual(6);
    expect(reads).toBeGreaterThanOrEqual(42);
    expect(standing(layerOf(container)).length).toBeGreaterThan(0);
  });
  it("checks a menagerie once however often a layer for it is mounted", () => {
    page();
    let reads = 0;
    const counted = Object.defineProperty({ ...MENAGERIE }, "schema", {
      enumerable: true,
      get: () => {
        reads += 1;
        return MENAGERIE.schema;
      },
    });
    render(<PetLayer menagerie={counted} scene="home" mode="calm" seed={7} />).unmount();
    const checked = reads;
    expect(checked).toBeGreaterThan(0);
    render(<PetLayer menagerie={counted} scene="home" mode="calm" seed={7} />).unmount();
    render(<PetLayer menagerie={counted} scene="home" mode="calm" seed={7} />).unmount();
    expect(reads).toBe(checked);
  });
}
