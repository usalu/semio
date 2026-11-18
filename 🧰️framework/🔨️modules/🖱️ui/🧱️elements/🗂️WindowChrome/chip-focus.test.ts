import { afterEach, describe, expect, test } from "vitest";
import { focusedWindowChipPath } from "./chip-focus.ts";

const box = (left: number, top: number, right: number, bottom: number) => ({ left, top, right, bottom, width: right - left, height: bottom - top, x: left, y: top, toJSON: () => ({}) }) as DOMRect;
const place = (element: Element, rect: DOMRect) => Object.defineProperty(element, "getBoundingClientRect", { value: () => rect });

describe("keyboard focus on a window chip recolours the chip's own lines", () => {
  afterEach(() => { document.body.innerHTML = ""; });
  const build = () => {
    document.body.innerHTML = '<div id="stack" data-window-silhouette><div id="chip" data-window-silhouette-chip><button id="control">x</button></div><button id="outside">y</button></div>';
    place(document.getElementById("stack")!, box(0, 0, 400, 300));
    place(document.getElementById("chip")!, box(0, 0, 120, 30));
    return document.getElementById("stack")!;
  };
  test("no path while nothing in a chip is focused", () => expect(focusedWindowChipPath(build())).toBeNull());
  test("outlines the focused chip on the silhouette centreline when the focus is visible", () => {
    const stack = build();
    const control = document.getElementById("control")!;
    control.matches = (selector: string) => selector === ":focus-visible" || Element.prototype.matches.call(control, selector);
    control.focus();
    expect(focusedWindowChipPath(stack)).toBe("M 1 1 L 119 1 L 119 29 L 1 29 Z");
  });
  test("snaps an interior chip edge onto the seam of the silhouette's own border", () => {
    const stack = build();
    stack.insertAdjacentHTML("afterbegin", '<svg data-window-silhouette-border><path d="M1,1 H121 V31 H399 V299 H1 Z" /></svg>');
    const control = document.getElementById("control")!;
    control.matches = (selector: string) => selector === ":focus-visible" || Element.prototype.matches.call(control, selector);
    control.focus();
    expect(focusedWindowChipPath(stack)).toBe("M 1 1 L 121 1 L 121 31 L 1 31 Z");
  });
  test("ignores pointer focus and controls outside the chips", () => {
    const stack = build();
    const control = document.getElementById("control")!;
    control.matches = () => false;
    control.focus();
    expect(focusedWindowChipPath(stack)).toBeNull();
    document.getElementById("outside")!.focus();
    expect(focusedWindowChipPath(stack)).toBeNull();
  });
});
