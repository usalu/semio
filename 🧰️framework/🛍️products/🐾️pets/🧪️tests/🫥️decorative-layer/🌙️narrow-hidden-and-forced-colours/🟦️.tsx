/** 🌙️ The narrow stage, hidden document, inert page and forced colours tests. */
import { render } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { PetLayer, petCapacity, petScale } from "@semio-tech/pets-react";
import { CARDS, HALF, MENAGERIE, VIEWPORT, layerOf, page, pointer, standing } from "../🧰️decorative-layer-fixtures/🟦️.tsx";

/** 🌗 Registers the narrow stage, hidden document, inert dialog, hidden start and forced colours tests. */
export function registerNarrowHiddenAndForcedColoursTests(): void {
  it("holds fewer and smaller pets on a narrow stage", () => {
    expect([0, 375, 767, 768, 1023, 1024, 1920].map(petCapacity)).toEqual([2, 2, 2, 4, 4, 6, 6]);
    expect([375, 767, 768, 1920].map(petScale)).toEqual([0.8, 0.8, 1, 1]);
    page();
    Object.defineProperty(document.documentElement, "clientWidth", { configurable: true, value: 600 });
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={7} />);
    vi.advanceTimersByTime(100);
    const pets = standing(layerOf(container));
    expect(pets).toHaveLength(2);
    expect(pets.map((pet) => pet.species).sort()[1]).toBe("carla");
    expect(pets.map((pet) => pet.scale)).toEqual([0.8, 0.8]);
    for (const pet of pets) {
      expect([CARDS.first.top, VIEWPORT.height], `${pet.species} stands on an edge of the page, in the pixels of the page`).toContain(pet.y);
      const [left, right] = pet.y === CARDS.first.top ? [CARDS.first.left, CARDS.first.left + CARDS.first.width] : [0, 600];
      expect(pet.x - HALF * 0.8, pet.species).toBeGreaterThanOrEqual(left - 0.01);
      expect(pet.x + HALF * 0.8, pet.species).toBeLessThanOrEqual(right + 0.01);
    }
    Object.defineProperty(document.documentElement, "clientWidth", { configurable: true, value: VIEWPORT.width });
    window.dispatchEvent(new Event("resize"));
    vi.advanceTimersByTime(100);
    const wide = standing(layerOf(container));
    expect(wide.map((pet) => pet.species).sort()).toEqual(["alba", "bruno", "carla"]);
    expect(wide.map((pet) => pet.scale)).toEqual([1, 1, 1]);
  });
  it("runs neither frames nor timers while the document is hidden and resumes without catching up", () => {
    page();
    let state: DocumentVisibilityState = "visible";
    vi.spyOn(document, "visibilityState", "get").mockImplementation(() => state);
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    expect(vi.getTimerCount()).toBe(1);
    state = "hidden";
    document.dispatchEvent(new Event("visibilitychange"));
    expect(vi.getTimerCount()).toBe(0);
    const hidden = layer.innerHTML;
    window.dispatchEvent(pointer("pointermove", 500, 100));
    window.dispatchEvent(new Event("resize"));
    vi.advanceTimersByTime(600_000);
    expect(vi.getTimerCount()).toBe(0);
    expect(layer.innerHTML).toBe(hidden);
    state = "visible";
    document.dispatchEvent(new Event("visibilitychange"));
    expect(vi.getTimerCount()).toBe(1);
    vi.advanceTimersByTime(3000);
    expect(standing(layer)).toHaveLength(3);
    expect(layer.innerHTML).not.toBe(hidden);
  });
  it("waits where it is while a dialog makes the whole page inert, instead of falling off surfaces that seem gone", async () => {
    const { first, second } = page();
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    const before = standing(layer);
    expect(before).toHaveLength(3);
    expect(before.some((pet) => pet.y === CARDS.first.top || pet.y === CARDS.second.top)).toBe(true);
    for (const element of [first, second, container]) element.setAttribute("inert", "");
    await vi.advanceTimersByTimeAsync(0);
    expect(vi.getTimerCount()).toBe(0);
    const waiting = layer.innerHTML;
    window.dispatchEvent(new Event("resize"));
    window.dispatchEvent(pointer("pointermove", 500, 100));
    await vi.advanceTimersByTimeAsync(120_000);
    expect(vi.getTimerCount()).toBe(0);
    expect(layer.innerHTML).toBe(waiting);
    expect(standing(layer)).toEqual(before);
    for (const element of [first, second, container]) element.removeAttribute("inert");
    await vi.advanceTimersByTimeAsync(0);
    expect(vi.getTimerCount()).toBe(1);
    await vi.advanceTimersByTimeAsync(48);
    expect(standing(layer).map(({ species, x, y }) => ({ species, x, y }))).toEqual(before.map(({ species, x, y }) => ({ species, x, y })));
    await vi.advanceTimersByTimeAsync(3000);
    expect(layer.innerHTML).not.toBe(waiting);
  });
  it("starts nothing in a hidden document until it is shown", () => {
    page();
    let state: DocumentVisibilityState = "hidden";
    vi.spyOn(document, "visibilityState", "get").mockImplementation(() => state);
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    expect(vi.getTimerCount()).toBe(0);
    vi.advanceTimersByTime(5000);
    expect(layerOf(container).childElementCount).toBe(0);
    state = "visible";
    document.dispatchEvent(new Event("visibilitychange"));
    vi.advanceTimersByTime(3000);
    expect(standing(layerOf(container))).toHaveLength(3);
  });
  it("stays empty and runs nothing under forced colours", () => {
    page();
    const queries: string[] = [];
    vi.stubGlobal("matchMedia", (query: string) => {
      queries.push(query);
      return { matches: query === "(forced-colors: active)", media: query, addEventListener: () => {}, removeEventListener: () => {} };
    });
    const frames = vi.spyOn(window, "requestAnimationFrame");
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="lively" seed={7} />);
    vi.advanceTimersByTime(5000);
    expect(queries).toContain("(forced-colors: active)");
    expect(layerOf(container).childElementCount).toBe(0);
    expect(frames).not.toHaveBeenCalled();
    expect(vi.getTimerCount()).toBe(0);
  });
}
