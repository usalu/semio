/** 🖱️ A press reads no geometry it can avoid: a control press is never canvas, and any other press measures the cutouts once, however many listeners judge it — the measuring forces a layout while an animation runs, which made every pointerdown a long task. */
import { afterEach, describe, expect, mock, test } from "bun:test";
import { isSurfaceActiveBackgroundPointer } from "./../🟦️.tsx";

class FakeElement {
  constructor(private readonly control: boolean) {}
  closest(selector: string): FakeElement | null {
    return this.control && selector.includes("button") ? this : null;
  }
}

const measured = mock(() => ({ width: 10, height: 10, left: 0, right: 10, top: 0, bottom: 10 }));
const gap = { closest: () => null, getBoundingClientRect: measured };
const original = { document: globalThis.document, Element: globalThis.Element };

function stage(): void {
  Object.assign(globalThis, { Element: FakeElement, document: { querySelectorAll: () => [gap] } });
  measured.mockClear();
}

afterEach(() => Object.assign(globalThis, original));

describe("press geometry", () => {
  test("never measures a cutout for a press on a control", () => {
    stage();
    expect(isSurfaceActiveBackgroundPointer({ target: new FakeElement(true) as unknown as Element, clientX: 5, clientY: 5 })).toBe(false);
    expect(measured).not.toHaveBeenCalled();
  });

  test("measures once for another press however many listeners judge it, and finds the pointer in the cutout", () => {
    stage();
    const event = { target: new FakeElement(false) as unknown as Element, clientX: 5, clientY: 5 };
    expect(isSurfaceActiveBackgroundPointer(event)).toBe(true);
    expect(isSurfaceActiveBackgroundPointer({ ...event, nativeEvent: event })).toBe(true);
    expect(isSurfaceActiveBackgroundPointer(event)).toBe(true);
    expect(measured).toHaveBeenCalledTimes(1);
  });
});
