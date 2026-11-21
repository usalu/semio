import * as React from "react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { cleanup, render } from "@testing-library/react";
import { JSDOM } from "jsdom";
import postcss from "postcss";
import { afterEach, describe, expect, it } from "vitest";
import { Checkbox } from "../☑️Checkbox/🟦️.tsx";
import { Tabs, TabsContent } from "../📑️Tabs/🟦️.tsx";
import { BorderCompositor } from "./shared-border.ts";

const css = () => readFileSync(resolve(import.meta.dirname, "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
const source = (path: string) => readFileSync(resolve(import.meta.dirname, path), "utf8");
const rect = (left: number, top: number, width: number, height: number) => ({ left, top, right: left + width, bottom: top + height, width, height }) as DOMRect;

describe("keyboard focus belongs to the compositor", () => {
  afterEach(cleanup);

  it("suppresses the browser outline for compositor boxes while retaining it for unregistered controls", () => {
    const sheet = postcss.parse(css());
    const layer = sheet.nodes.find(node => node.type === "atrule" && node.name === "layer" && node.params === "shared-border-outline") as import("postcss").AtRule;
    const rule = layer.nodes!.find(node => node.type === "rule" && node.selector.includes("[data-shared-border-root]")) as import("postcss").Rule;
    expect(rule.nodes.some(node => node.type === "decl" && node.prop === "outline" && node.value === "none" && !node.important)).toBe(true);
    const root = document.createElement("div");
    root.setAttribute("data-shared-border-root", "");
    root.innerHTML = "<button id='owned' data-shared-border-frame></button><button id='native'></button>";
    document.body.append(root);
    try {
      for (const button of root.querySelectorAll("button")) {
        button.focus();
        expect(button.matches(rule.selector)).toBe(button.id === "owned");
      }
    } finally { root.remove(); }
    expect(sheet.nodes.indexOf(layer)).toBeGreaterThan(sheet.nodes.findIndex(node => node.type === "atrule" && node.name === "import" && node.params.includes("tailwindcss")));
    expect(css()).not.toMatch(/\[role="tabpanel"\]:focus-visible/);
  });

  it("class 1: tabpanel, selectable history rows, checkbox, icon gallery and the layout panel toggle are boxes", () => {
    const { container } = render(<Tabs defaultValue="a"><TabsContent value="a" /><Checkbox id="box" /></Tabs>);
    for (const element of [container.querySelector('[role="tabpanel"]'), container.querySelector("#box")]) {
      expect(element?.getAttribute("data-shared-border-frame")).toBe("");
      expect(element?.getAttribute("data-shared-border-edges")).toBe("");
    }
    expect(source("../🕰️HistoryTable/🟦️.tsx")).toContain("sharedBorderlessBoxProps : undefined");
    expect(source("../🔣️Icons/📖️stories/🧪️.story.tsx").match(/sharedBorderlessBoxProps/g)!.length).toBeGreaterThanOrEqual(3);
    expect(source("../📐️Layout/📖️stories/🧪️.story.tsx")).toContain("<button {...sharedBorderlessBoxProps}");
  });

  it("class 1: a diagram node that takes keyboard focus becomes the focused box", () => {
    const inner = source("../🕸️Diagram/🧩️inner/🟦️.tsx");
    expect(inner).toContain('.react-flow__node[data-id]');
    expect(inner).toContain('node.matches(":focus-visible")');
    expect(inner).toContain("setKeyboardFocusedNodeId(node.dataset.id");
  });

  it("class 2: focus of a cell reaches neither its ancestors nor a nested compositor's content, exactly like hover", () => {
    const dom = new JSDOM('<div class="semio-scope" id="outer"><div id="card" data-shared-border-frame tabindex="0"><div class="semio-scope" id="inner"><button id="b" data-shared-border-frame data-shared-border-edges=""></button></div></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const document = dom.window.document;
    const outer = document.querySelector<HTMLElement>("#outer")!, inner = document.querySelector<HTMLElement>("#inner")!, card = document.querySelector<HTMLElement>("#card")!, b = document.querySelector<HTMLElement>("#b")!;
    card.getBoundingClientRect = () => rect(0, 0, 100, 40); b.getBoundingClientRect = () => rect(10, 10, 40, 20); inner.getBoundingClientRect = () => rect(0, 0, 100, 40);
    const compositors = [new BorderCompositor(outer), new BorderCompositor(inner)];
    b.matches = (selector: string) => selector === ":focus-visible" || Element.prototype.matches.call(b, selector);
    try {
      for (const compositor of compositors) compositor.schedule();
      b.focus();
      for (const compositor of compositors) compositor.scheduleState();
      expect([...card.querySelectorAll(':scope > svg [data-shared-border-state="focus"]')]).toHaveLength(0);
      expect(b.querySelector('svg [data-shared-border-state="focus"]') ?? card.querySelector('#inner svg')).not.toBeNull();
    } finally {
      for (const compositor of compositors) compositor.dispose();
      Object.assign(globalThis, previous); dom.window.close();
    }
  });

  const harness = (html: string, rects: Record<string, DOMRect>) => {
    const dom = new JSDOM(html);
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    for (const [id, box] of Object.entries(rects)) root.querySelector<HTMLElement>(`#${id}`)!.getBoundingClientRect = () => box;
    const compositor = new BorderCompositor(root);
    compositor.schedule();
    const focus = (id: string) => {
      const element = root.querySelector<HTMLElement>(`#${id}`)!;
      element.matches = (selector: string) => selector === ":focus-visible" || Element.prototype.matches.call(element, selector);
      element.focus(); compositor.scheduleState();
    };
    const painted = (state: string) => [...root.querySelectorAll(`svg [data-shared-border-state="${state}"]`)].map(path => path.getAttribute("d")).sort();
    return { root, compositor, focus, painted, done: () => { compositor.dispose(); Object.assign(globalThis, previous); dom.window.close(); } };
  };
  const closed = (left: number, right: number, top: number, bottom: number) => [`M ${left} ${top} L ${left} ${bottom}`, `M ${right} ${top} L ${right} ${bottom}`, `M ${left} ${top} L ${right} ${top}`, `M ${left} ${bottom} L ${right} ${bottom}`].sort();

  it("class 3: a focused table row closes all four sides, also when a scroller narrower than the row clips it", () => {
    for (const [clipWidth, right] of [[100, 99], [85, 84]] as const) {
      const h = harness(`<div class="semio-scope"><div id="scroller" style="overflow:auto"><table><tbody><tr id="row" tabindex="0" data-shared-border-frame data-shared-border-edges=""><td>a</td></tr></tbody></table></div></div>`, { scroller: rect(0, 0, clipWidth, 40), row: rect(0, 10, 100, 20) });
      try {
        h.focus("row");
        expect(h.painted("focus"), `clip ${clipWidth}`).toEqual(closed(1, right, 1, 19));
      } finally { h.done(); }
    }
  });

  it("class 4: a focused control at the right edge of a clipping navbar keeps its right side over the full height", () => {
    const h = harness('<div class="semio-scope"><div id="bar" style="overflow:hidden" data-shared-border-frame data-shared-border-edges="bottom"><div id="group" data-shared-border-frame><button id="fullscreen"></button></div></div></div>', { bar: rect(0, 0, 200, 20), group: rect(150, 0, 50, 20), fullscreen: rect(150, 0, 50, 20) });
    try {
      h.focus("fullscreen");
      expect(h.painted("focus")).toEqual(closed(1, 49, 1, 19));
    } finally { h.done(); }
  });

  it("class 5: a hovered cell three owners deep states only itself, and a role-less interactive cell is closed", () => {
    const h = harness('<div class="semio-scope"><div id="selector" data-shared-border-frame data-shared-border-cells><div id="row" data-shared-border-frame data-shared-border-cells><div id="group" data-shared-border-frame><button id="trigger" role="combobox" data-shared-border-cell></button><button id="import" data-shared-border-cell></button></div></div></div></div>', { selector: rect(0, 0, 100, 20), row: rect(0, 0, 100, 20), group: rect(0, 0, 100, 20), trigger: rect(0, 0, 50, 20), import: rect(50, 0, 50, 20) });
    try {
      for (const id of ["trigger", "import"]) {
        h.root.querySelector(`#${id}`)!.dispatchEvent(new (h.root.ownerDocument.defaultView as unknown as typeof globalThis).MouseEvent("pointerover", { bubbles: true }));
        const [left, right] = id === "trigger" ? [1, 50] : [50, 99];
        expect(h.painted("hover"), id).toEqual(closed(left, right, 1, 19));
        expect(h.painted("normal").length).toBeGreaterThan(0);
      }
    } finally { h.done(); }
  });

  it("class 6: a native checkbox that sits beside another cell of a group row is a box and is hovered closed", () => {
    const h = harness('<div class="semio-scope"><div id="row" data-shared-border-group data-shared-border-cells data-shared-border-cell><input id="check" type="checkbox" data-shared-border-frame data-shared-border-edges="" /><div id="text" data-shared-border-cell>Check</div></div></div>', { row: rect(0, 0, 100, 20), check: rect(0, 2, 16, 16), text: rect(16, 0, 84, 20) });
    try {
      h.root.querySelector("#check")!.dispatchEvent(new (h.root.ownerDocument.defaultView as unknown as typeof globalThis).MouseEvent("pointerover", { bubbles: true }));
      expect(h.painted("hover")).toEqual(closed(1, 16, 3, 17));
    } finally { h.done(); }
  });

  it("round 7: cell edges lying on a bordered owner's edge resolve to the owner's own coordinates, so owner and cells paint one line", () => {
    const h = harness('<div class="semio-scope"><div id="group" data-shared-border-frame style="border:1px solid"><button id="a" data-shared-border-cell></button><button id="b" data-shared-border-cell></button><button id="c" data-shared-border-cell></button></div></div>', { group: rect(0, 0, 60, 20), a: rect(1, 1, 19, 18), b: rect(20, 1, 20, 18), c: rect(40, 1, 19, 18) });
    try {
      const all = [...h.root.querySelectorAll("svg path")].map(path => path.getAttribute("d")!);
      const numbers = all.flatMap(d => d.match(/-?\d+(?:\.\d+)?/g)!.map(Number));
      expect(numbers.every(value => [1, 19, 20, 40, 59].includes(value)), all.join("|")).toBe(true);
      expect(all.every(d => !/ (2|18) /.test(d))).toBe(true);
      h.root.querySelector("#b")!.dispatchEvent(new (h.root.ownerDocument.defaultView as unknown as typeof globalThis).MouseEvent("pointerover", { bubbles: true }));
      expect(h.painted("hover")).toEqual(closed(20, 40, 1, 19));
      h.root.querySelector("#b")!.dispatchEvent(new (h.root.ownerDocument.defaultView as unknown as typeof globalThis).MouseEvent("pointerout", { bubbles: true, relatedTarget: h.root }));
      h.focus("a");
      expect(h.painted("focus")).toEqual(closed(1, 20, 1, 19));
    } finally { h.done(); }
  });

  it("round 7: a cell nested in a group shell that is itself a cell of a bordered group paints on the group's coordinates (the shell overlay's origin is the group's inner corner, local 0 is screen 1)", () => {
    const h = harness('<div class="semio-scope"><div id="group" data-shared-border-frame style="border:1px solid"><div id="shell" data-shared-border-cell data-shared-border-group data-shared-border-cells><button id="item" data-shared-border-cell></button><div id="action" data-shared-border-cell></div></div><button id="other" data-shared-border-cell></button></div></div>', { group: rect(0, 0, 90, 20), shell: rect(1, 1, 58, 18), item: rect(1, 1, 40, 18), action: rect(41, 1, 18, 18), other: rect(59, 1, 30, 18) });
    try {
      h.focus("item");
      expect(h.painted("focus")).toEqual(closed(0, 40, 0, 18));
      const shell = h.root.querySelector<SVGElement>("#shell > svg")!;
      expect(shell).not.toBeNull();
    } finally { h.done(); }
  });

  it("round 7: focus outranks a neighbour's selection, so a focused cell is never opened by it", () => {
    const h = harness('<div class="semio-scope"><div id="group" data-shared-border-frame><button id="a" data-shared-border-cell aria-pressed="true"></button><button id="b" data-shared-border-cell></button></div></div>', { group: rect(0, 0, 100, 20), a: rect(0, 0, 50, 20), b: rect(50, 0, 50, 20) });
    try {
      h.focus("b");
      expect(h.painted("focus")).toEqual(closed(50, 99, 1, 19));
      expect(h.painted("active").every(d => !d.startsWith("M 50 1 L 50"))).toBe(true);
    } finally { h.done(); }
  });

  it("round 7: a diagram node never shows the browser outline, even before any compositor root exists", () => {
    expect(css()).toMatch(/@layer shared-border-outline \{[^}]*\.react-flow__node:focus-visible/s);
  });

  it("round 7: skeleton diagram nodes are borderless boxes", () => {
    expect(source("../🕸️Diagram/🟦️.tsx")).toContain("domAttributes: { ...sharedBorderlessBoxProps }");
  });
});
