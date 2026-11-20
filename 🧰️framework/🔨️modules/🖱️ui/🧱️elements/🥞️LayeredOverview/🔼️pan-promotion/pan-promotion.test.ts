/** 🔼️ A panning strip is promoted to a layer of its own only while it moves: each write of the transform promotes it, and a moment after the last one it is flattened to the same 2D translation and given up, so a resting strip of many megapixels holds no layer.  */
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { PAN_REST_MS, flatten, writeTransform } from "./🟦️.ts";

beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
});

describe("flatten", () => {
  it("turns the strip's 3D translation into the same 2D one, and leaves anything else as it is", () => {
    expect(flatten("translate3d(-33.3333%, -11.1111%, 0)")).toBe("translate(-33.3333%, -11.1111%)");
    expect(flatten("translate3d(0%, 0%, 0)")).toBe("translate(0%, 0%)");
    expect(flatten("translate3d(1px, 2px, 3px)")).toBe("translate3d(1px, 2px, 3px)");
    expect(flatten("")).toBe("");
  });
});

describe("writeTransform", () => {
  it("promotes the element while it is written to, and flattens and releases it once it rests", () => {
    const strip = document.createElement("div");
    writeTransform(strip, "translate3d(-10%, 0%, 0)");
    expect(strip.style.transform).toBe("translate3d(-10%, 0%, 0)");
    expect(strip.style.willChange).toBe("transform");
    vi.advanceTimersByTime(PAN_REST_MS - 1);
    expect(strip.style.willChange).toBe("transform");
    vi.advanceTimersByTime(1);
    expect(strip.style.willChange).toBe("");
    expect(strip.style.transform).toBe("translate(-10%, 0%)");
  });

  it("waits for the last write of a pan, not the first", () => {
    const strip = document.createElement("div");
    for (let frame = 0; frame < 10; frame++) {
      writeTransform(strip, `translate3d(-${frame}%, 0%, 0)`);
      vi.advanceTimersByTime(PAN_REST_MS - 1);
    }
    expect(strip.style.willChange).toBe("transform");
    expect(strip.style.transform).toBe("translate3d(-9%, 0%, 0)");
    vi.advanceTimersByTime(1);
    expect(strip.style.willChange).toBe("");
    expect(strip.style.transform).toBe("translate(-9%, 0%)");
  });

  it("promotes again when the strip moves again, and keeps every element on its own timer", () => {
    const one = document.createElement("div");
    const two = document.createElement("div");
    writeTransform(one, "translate3d(-1%, 0%, 0)");
    vi.advanceTimersByTime(PAN_REST_MS);
    expect(one.style.willChange).toBe("");
    writeTransform(one, "translate3d(-2%, 0%, 0)");
    writeTransform(two, "translate3d(-5%, 0%, 0)");
    expect(one.style.willChange).toBe("transform");
    vi.advanceTimersByTime(PAN_REST_MS);
    expect([one.style.willChange, two.style.willChange]).toEqual(["", ""]);
  });

  it("leaves no timer behind when the element is let go of", () => {
    const strip = document.createElement("div");
    writeTransform(strip, "translate3d(-1%, 0%, 0)");
    writeTransform(strip, null);
    expect(vi.getTimerCount()).toBe(0);
  });
});
