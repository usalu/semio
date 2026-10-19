/** 🎞️ The still and living stage painting, pointer rest and surface follow tests. */
import { render } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { PetLayer } from "@semio-tech/pets-react";
import { CARDS, MENAGERIE, VIEWPORT, layerOf, looks, page, place, pointer, standing } from "../🧰️decorative-layer-fixtures/🟦️.tsx";

/** 🐾 Registers the still stage paint, the pointer rest on still and living stages, the page move follow and the calm stage tests. */
export function registerStillAndLivingStageTests(): void {
  it("paints a still stage once and schedules nothing while the pointer keeps away from the pets", () => {
    page();
    const frames = vi.spyOn(window, "requestAnimationFrame");
    const timers = vi.spyOn(window, "setTimeout");
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={11} />);
    expect(frames).toHaveBeenCalledTimes(1);
    vi.advanceTimersByTime(100);
    const layer = layerOf(container);
    const painted = layer.innerHTML;
    expect(standing(layer)).toHaveLength(3);
    expect(vi.getTimerCount()).toBe(0);
    const writes = vi.spyOn(Element.prototype, "setAttribute");
    window.dispatchEvent(pointer("pointermove", 900, 100));
    window.dispatchEvent(pointer("pointerdown", 900, 120));
    window.dispatchEvent(pointer("pointerup", 900, 120));
    document.documentElement.dispatchEvent(pointer("pointerleave", 0, 0));
    expect(vi.getTimerCount()).toBe(0);
    vi.advanceTimersByTime(120_000);
    expect(layer.innerHTML).toBe(painted);
    expect(writes).not.toHaveBeenCalled();
    expect(frames).toHaveBeenCalledTimes(1);
    expect(timers).not.toHaveBeenCalled();
    expect(vi.getTimerCount()).toBe(0);
  });
  it("keeps a pet of a still stage whole while the pointer rests on it, and wakes nothing for the pointer: no frame, no write, no timer", () => {
    page();
    const frames = vi.spyOn(window, "requestAnimationFrame");
    const timers = vi.spyOn(window, "setTimeout");
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={11} />);
    vi.advanceTimersByTime(100);
    const layer = layerOf(container);
    const painted = layer.innerHTML;
    const before = standing(layer);
    const writes = vi.spyOn(Element.prototype, "setAttribute");
    window.dispatchEvent(pointer("pointermove", before[0]!.x, before[0]!.y - 10));
    vi.advanceTimersByTime(60_000);
    expect(standing(layer)).toEqual(before);
    expect(layer.innerHTML).toBe(painted);
    expect(frames).toHaveBeenCalledTimes(1);
    expect(writes).not.toHaveBeenCalled();
    expect(timers).not.toHaveBeenCalled();
    expect(vi.getTimerCount()).toBe(0);
  });
  it("keeps a pet of a living stage whole while the pointer rests on it: a pet on its perch is never see-through", () => {
    page();
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(1000);
    const layer = layerOf(container);
    const [first] = standing(layer);
    expect(first!.opacity).toBe(1);
    window.dispatchEvent(pointer("pointermove", first!.x, first!.y - 20));
    for (let wait = 0; wait < 7; wait++) {
      vi.advanceTimersByTime(100);
      expect(standing(layer).map((pet) => pet.opacity)).toEqual([1, 1, 1]);
    }
    document.documentElement.dispatchEvent(pointer("pointerleave", 0, 0));
    vi.advanceTimersByTime(600);
    expect(standing(layer).map((pet) => pet.opacity)).toEqual([1, 1, 1]);
  });
  it("follows a still stage's surfaces when the page moves, in one frame, and rests again", () => {
    const { first } = page();
    const frames = vi.spyOn(window, "requestAnimationFrame");
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={11} />);
    vi.advanceTimersByTime(100);
    const layer = layerOf(container);
    const before = standing(layer);
    const riders = before.filter((pet) => pet.y === CARDS.first.top);
    place(first, { ...CARDS.first, left: CARDS.first.left + 30, top: CARDS.first.top + 50 });
    window.dispatchEvent(new Event("resize"));
    expect(frames).toHaveBeenCalledTimes(2);
    vi.advanceTimersByTime(100);
    const after = standing(layer);
    for (const rider of riders) {
      const moved = after.find((pet) => pet.species === rider.species)!;
      expect([moved.x, moved.y], rider.species).toEqual([rider.x + 30, rider.y + 50]);
    }
    for (const pet of before.filter((each) => each.y !== CARDS.first.top)) expect(after.find((each) => each.species === pet.species), pet.species).toEqual(pet);
    expect(frames).toHaveBeenCalledTimes(2);
    expect(vi.getTimerCount()).toBe(0);
  });
  it("keeps a calm stage alive outside React and lets its pets look at the pointer", () => {
    page();
    let renders = 0;
    function Host(): ReturnType<typeof PetLayer> {
      renders += 1;
      return <PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />;
    }
    const { container } = render(<Host />);
    vi.advanceTimersByTime(1000);
    const layer = layerOf(container);
    expect(standing(layer).map((pet) => pet.opacity)).toEqual([1, 1, 1]);
    const event = pointer("pointermove", VIEWPORT.width + 2000, 300);
    window.dispatchEvent(event);
    vi.advanceTimersByTime(1000);
    expect(looks(layer).every((look) => look > 1)).toBe(true);
    window.dispatchEvent(pointer("pointermove", -2000, 300));
    vi.advanceTimersByTime(1000);
    expect(looks(layer).every((look) => look < -1)).toBe(true);
    document.documentElement.dispatchEvent(pointer("pointerleave", 0, 0));
    vi.advanceTimersByTime(1000);
    expect(looks(layer).every((look) => Math.abs(look) < 0.7)).toBe(true);
    const [first] = standing(layer);
    const button = document.querySelector("button")!;
    const heard = vi.fn();
    document.addEventListener("pointerdown", heard);
    const missed = pointer("pointerdown", first!.x, first!.y - 10);
    button.dispatchEvent(missed);
    window.dispatchEvent(pointer("pointerup", first!.x, first!.y - 10));
    const press = pointer("pointerdown", first!.x, first!.y - 10);
    document.body.dispatchEvent(press);
    window.dispatchEvent(pointer("pointerup", first!.x, first!.y - 10));
    vi.advanceTimersByTime(300);
    document.removeEventListener("pointerdown", heard);
    expect([missed.defaultPrevented, press.defaultPrevented], "a control under a pet keeps its press; a press on the pet is the pets'").toEqual([false, true]);
    expect(heard.mock.calls.map(([seen]) => seen)).toEqual([missed]);
    const seen = new Set<string>();
    for (let second = 0; second < 30; second++) {
      vi.advanceTimersByTime(1000);
      seen.add(layer.innerHTML);
    }
    expect(seen.size).toBeGreaterThan(10);
    expect(standing(layer)).toHaveLength(3);
    expect(vi.getTimerCount()).toBe(1);
    expect(event.defaultPrevented).toBe(false);
    expect(renders).toBe(1);
  });
}
