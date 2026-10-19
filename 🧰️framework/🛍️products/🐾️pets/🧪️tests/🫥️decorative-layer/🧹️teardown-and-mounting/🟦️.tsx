/** 🧹️ The teardown, strict mode, seeded mount and failure containment tests. */
import { render } from "@testing-library/react";
import { StrictMode } from "react";
import { expect, it, vi } from "vitest";
import { PetLayer } from "@semio-tech/pets-react";
import { MENAGERIE, layerOf, ledger, looks, observers, page, pointer, standing } from "../🧰️decorative-layer-fixtures/🟦️.tsx";

/** 🔚 Registers the clean teardown, the strict mode double mount, the seeded stage, the silent failure end and the look hand over tests. */
export function registerTeardownAndMountingTests(): void {
  it("leaves no listener, observer, frame, timer or element behind when it goes", () => {
    page();
    const before = document.body.innerHTML;
    const { container, rerender, unmount } = render(<i />);
    const live = observers();
    const listening = [ledger(window), ledger(document), ledger(document.documentElement)];
    rerender(<PetLayer menagerie={MENAGERIE} scene="home" mode="lively" seed={7} />);
    vi.advanceTimersByTime(5000);
    const layer = layerOf(container);
    expect(layer.childElementCount).toBe(3);
    const hand = ["blur", "click", "contextmenu", "dragstart", "input", "keydown", "lostpointercapture", "pointercancel", "pointerdown", "pointermove", "pointerup", "selectstart", "wheel"];
    expect(listening.map((open) => open())).toEqual([
      ["pagehide", "pageshow", "pointercancel", "pointerdown", "pointermove", "pointerup", "resize", ...hand].sort(),
      ["focusin", "focusout", "scroll", "scroll", "transitionend", "visibilitychange"],
      ["pointerleave"],
    ]);
    expect(live).toEqual({ resize: 1, mutation: 1 });
    expect(vi.getTimerCount()).toBe(1);
    rerender(<i />);
    expect(listening.map((open) => open())).toEqual([[], [], []]);
    expect(live).toEqual({ resize: 0, mutation: 0 });
    expect(vi.getTimerCount()).toBe(0);
    expect(layer.childElementCount).toBe(0);
    expect(layer.style.zIndex).toBe("");
    expect(layer.isConnected).toBe(false);
    unmount();
    container.remove();
    expect(document.body.innerHTML).toBe(before);
    window.dispatchEvent(pointer("pointermove", 10, 10));
    window.dispatchEvent(new Event("resize"));
    vi.advanceTimersByTime(5000);
    expect(vi.getTimerCount()).toBe(0);
  });
  it("sets up once under StrictMode's double mount and tears down as cleanly", () => {
    page();
    const { container, rerender } = render(
      <StrictMode>
        <i />
      </StrictMode>,
    );
    const live = observers();
    const listening = [ledger(window), ledger(document), ledger(document.documentElement)];
    const frames = vi.spyOn(window, "requestAnimationFrame");
    rerender(
      <StrictMode>
        <PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />
      </StrictMode>,
    );
    expect(frames).toHaveBeenCalledTimes(2);
    expect(vi.getTimerCount()).toBe(1);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    expect(standing(layer).map((pet) => pet.species).sort()).toEqual(["alba", "bruno", "carla"]);
    expect(layer.childElementCount).toBe(3);
    expect(listening.map((open) => open().length)).toEqual([20, 6, 1]);
    expect(live).toEqual({ resize: 1, mutation: 1 });
    expect(vi.getTimerCount()).toBe(1);
    rerender(
      <StrictMode>
        <i />
      </StrictMode>,
    );
    expect(listening.map((open) => open())).toEqual([[], [], []]);
    expect(live).toEqual({ resize: 0, mutation: 0 });
    expect(vi.getTimerCount()).toBe(0);
  });
  it("gives every mount a stage of its own unless it is seeded, and the same stage for the same seed", () => {
    page();
    const play = (seed: number | undefined): string => {
      const { container, unmount } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={seed} />);
      vi.advanceTimersByTime(30_000);
      const painted = standing(layerOf(container))
        .map((pet) => `${pet.species}@${pet.x},${pet.y}`)
        .sort()
        .join(" ");
      unmount();
      return painted;
    };
    expect(play(7)).toBe(play(7));
    const random = vi.spyOn(Math, "random");
    play(undefined);
    expect(random).toHaveBeenCalled();
  });
  it("ends the show silently when something inside it fails", () => {
    page();
    const live = observers();
    const { container } = render(
      <PetLayer
        menagerie={MENAGERIE}
        scene="home"
        mode="calm"
        seed={7}
        glances={() => {
          throw new Error("a host function failed");
        }}
      />,
    );
    vi.advanceTimersByTime(5000);
    expect(layerOf(container).childElementCount).toBe(0);
    expect(live).toEqual({ resize: 0, mutation: 0 });
    expect(vi.getTimerCount()).toBe(0);
  });
  it("hands other things worth a look to the stage in its own pixels", () => {
    page();
    const glances = vi.fn(() => [{ x: 5000, y: 300 }]);
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} glances={glances} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    expect(glances).toHaveBeenCalled();
    expect(looks(layer)).toHaveLength(3);
    expect(looks(layer).every((look) => look > 0.5)).toBe(true);
  });
}
