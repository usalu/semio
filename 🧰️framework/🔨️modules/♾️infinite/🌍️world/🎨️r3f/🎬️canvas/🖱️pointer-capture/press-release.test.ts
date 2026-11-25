import { describe, expect, test } from "bun:test";
import { releaseWorldOrbitMouseCapture } from "./🟦️.ts";

const target = (captured: Set<number>) => ({ hasPointerCapture: (id: number) => captured.has(id), releasePointerCapture: (id: number) => void captured.delete(id) });

describe("orbit pointer capture", () => {
  test("a mouse press is released back to the element under it, touch and pen keep their capture", () => {
    const captured = new Set([1, 2, 3]);
    expect(releaseWorldOrbitMouseCapture(target(captured), { pointerId: 1, pointerType: "mouse" })).toBe(true);
    expect(releaseWorldOrbitMouseCapture(target(captured), { pointerId: 2, pointerType: "touch" })).toBe(false);
    expect(releaseWorldOrbitMouseCapture(target(captured), { pointerId: 3, pointerType: "pen" })).toBe(false);
    expect([...captured]).toEqual([2, 3]);
    expect(releaseWorldOrbitMouseCapture(target(captured), { pointerId: 9, pointerType: "mouse" })).toBe(false);
  });
});
