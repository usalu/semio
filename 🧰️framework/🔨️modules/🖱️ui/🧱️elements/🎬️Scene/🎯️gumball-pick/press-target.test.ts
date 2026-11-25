import { afterAll, beforeAll, describe, expect, test } from "bun:test";
import { gumballPressTarget } from "./🟦️.ts";

class FakeElement {}

describe("gumball press target", () => {
  const original = (globalThis as { HTMLElement?: unknown }).HTMLElement;
  beforeAll(() => {
    (globalThis as { HTMLElement?: unknown }).HTMLElement = FakeElement;
  });
  afterAll(() => {
    (globalThis as { HTMLElement?: unknown }).HTMLElement = original;
  });

  test("a press is decided on the r3f event source when the canvas has one, else on the canvas", () => {
    const canvas = new FakeElement() as unknown as HTMLElement;
    const source = new FakeElement() as unknown as HTMLElement;
    expect(gumballPressTarget(source, canvas)).toBe(source);
    expect(gumballPressTarget(undefined, canvas)).toBe(canvas);
    expect(gumballPressTarget({}, canvas)).toBe(canvas);
  });
});
