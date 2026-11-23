import { describe, expect, test } from "vitest";
import Ajv from "ajv/dist/2020.js";
import cases from "./chip-abut.cases.json";
import schema from "./chip-abut.schema.json";
import { chipAbutSize, resolveChipRowAbuts, settleChipRowAbuts, settlePaneChipAbuts, type ChipRowChip } from "./chip-abut.ts";

describe("window chip rows abut instead of leaving a small gap", () => {
  test("neutral chip-abut schema", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const fixture of cases.cases) test(fixture.name, () => expect(resolveChipRowAbuts(fixture.chips as ChipRowChip[])).toEqual(fixture.expected));
});

describe("the width a stretched chip takes", () => {
  test("a right-anchored chip grows over the sliver to the middle chip", () => expect(chipAbutSize([{ left: 156, right: 228, natural: 72, reach: "none" }, { left: 266, right: 384, natural: 118, reach: "prev" }], 1)).toBeCloseTo(156, 5));
  test("a left-anchored chip grows over the sliver to the next chip", () => expect(chipAbutSize([{ left: 0, right: 75, natural: 75, reach: "next" }, { left: 100, right: 172, natural: 72, reach: "none" }], 0)).toBe(100));
});

describe("settling a painted row", () => {
  const box = (left: number, right: number) => ({ left, right, width: right - left, top: 0, bottom: 30, height: 30, x: left, y: 0, toJSON: () => ({}) }) as DOMRect;
  const place = (element: Element, left: number, right: number) => Object.defineProperty(element, "getBoundingClientRect", { value: () => box(left, right) });
  const row = (gapLeft: number, controlsLeft: number) => {
    document.body.innerHTML = '<div data-window-frame><div id="row"><div id="title" data-window-frame-chip data-chip-reach="next"><span></span></div><div id="controls" data-window-frame-chip></div></div></div>';
    const element = document.getElementById("row")!;
    place(element, 0, 420);
    place(document.getElementById("title")!, 0, gapLeft);
    place(document.querySelector("#title > span")!, 0, 200);
    place(document.getElementById("controls")!, controlsLeft, 420);
    return element;
  };
  test("marks the title chip when the gap is a sliver", () => {
    expect(settleChipRowAbuts(row(200, 239.79))).toBe(true);
    expect(document.getElementById("title")!.hasAttribute("data-chip-abut")).toBe(true);
    expect(document.getElementById("controls")!.hasAttribute("data-chip-abut")).toBe(false);
  });
  test("leaves a gap of at least the minimum open", () => {
    expect(settleChipRowAbuts(row(200, 240))).toBe(false);
    expect(document.getElementById("title")!.hasAttribute("data-chip-abut")).toBe(false);
  });
});

describe("settling the anchored panes of a window body", () => {
  const box = (left: number, right: number) => ({ left, right, width: right - left, top: 0, bottom: 30, height: 30, x: left, y: 0, toJSON: () => ({}) }) as DOMRect;
  const place = (element: Element, left: number, right: number) => Object.defineProperty(element, "getBoundingClientRect", { value: () => box(left, right) });
  const pane = (anchor: string, left: number, right: number, natural: number) => `<div data-anchor="${anchor}"><div data-window-frame><div data-window-frame-chip data-slot="${anchor}"><span data-w="${natural}" data-l="${left}" data-r="${right}"></span></div></div></div>`;
  const body = (rightLeft: number, natural = 118) => {
    document.body.innerHTML = `<div id="body" data-slot="window-body">${pane("top-left", 0, 75, 75)}${pane("top-middle", 156, 228, 72)}${pane("top-right", rightLeft, 384, natural)}</div>`;
    const element = document.getElementById("body")!;
    place(element, 0, 384);
    Object.defineProperty(element, "offsetWidth", { value: 384 });
    for (const chip of document.querySelectorAll<HTMLElement>("[data-window-frame-chip]")) {
      const span = chip.firstElementChild as HTMLElement;
      place(chip, +span.dataset.l!, +span.dataset.r!);
      place(span, 0, +span.dataset.w!);
    }
    return element;
  };
  test("a right-anchored chip within a sliver of the middle chip stretches over it", () => {
    expect(settlePaneChipAbuts(body(266))).toBe(true);
    const chip = document.querySelector<HTMLElement>('[data-slot="top-right"]')!;
    expect(chip.hasAttribute("data-chip-abut")).toBe(true);
    expect(chip.parentElement!.style.getPropertyValue("--chip-abut-size")).toBe("156px");
    expect(document.querySelector('[data-slot="top-middle"]')!.hasAttribute("data-chip-abut")).toBe(false);
  });
  test("a gap of at least 40 px stays open", () => {
    expect(settlePaneChipAbuts(body(268, 116))).toBe(false);
    expect(document.querySelector('[data-slot="top-right"]')!.hasAttribute("data-chip-abut")).toBe(false);
  });
});
