/** 🪞️ Lifting shows a copy of a host element in the pet layer while the element itself stays exactly where and what it
 * is, only transparent, and gives it back in the same task the moment the learner reaches for it. The copy wears the
 * computed look of every node of its original and carries nothing a script or a person could find or reach it by —
 * judged by `@testing-library`'s accessibility tree —; its place on screen is held to gl-matrix's `mat2d`, which
 * composes the turn about its middle the way CSS does. The original is compared by its `outerHTML`: while lifted it
 * differs in its inline style alone, afterwards not at all. Nothing is written when nothing changes, nothing in a way a
 * Content-Security-Policy without inline styles would block, and nothing to the console.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/🪞️lifting/🟦️.ts
 * @see ../../🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts
 * @see https://glmatrix.net/docs/module-mat2d.html — the third-party oracle of the placement
 * @see https://testing-library.com/docs/dom-testing-library/api-accessibility — the third-party oracle of the accessibility tree
 */

import { getRoles, isInaccessible } from "@testing-library/react";
import { glMatrix, mat2d, vec2 } from "gl-matrix";
import { afterEach, beforeEach, describe, expect, it, vi, type MockInstance } from "vitest";
import type { LiftFrame } from "@semio-tech/pets";
import { FIXTURE_ELEMENTS, LIFT_LOOK, LIFT_REACH, PET_PROPS, RECLAIMS, copyOf, copyable, fixtureId, liftFixtures, type Lifting } from "@semio-tech/pets-react";

glMatrix.setMatrixArrayType(Array);

interface Box {
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
}

const ROW: Box = { left: 120, top: 200, width: 400, height: 48 };
const ORIGIN = { x: 0, y: 0 };
const SHEET = `
  .row { display: flex; gap: 6px; padding: 4px 8px; border: 1px solid rgb(10, 20, 30); color: rgb(0, 17, 23); background-color: rgb(247, 243, 227); font-size: 14px; font-weight: 600; line-height: 20px; border-radius: 3px; box-shadow: 0 1px 2px rgb(0, 0, 0); }
  .label { font-style: italic; letter-spacing: 1px; text-decoration-line: underline; }
  .busy { animation-name: spin; pointer-events: auto; }
  input, select { display: inline-block; }
`;

function place<T extends Element>(element: T, box: Box): T {
  element.getBoundingClientRect = () => ({ ...box, x: box.left, y: box.top, right: box.left + box.width, bottom: box.top + box.height, toJSON: () => box }) as DOMRect;
  return element;
}

/** 🏗️ A layer and one row of a list a pet may play with, full of what a copy must leave behind: ids, names, hooks, handlers, a link, a label, form controls with what a person entered, an inline style. */
function scene(): { readonly host: HTMLElement; readonly row: HTMLLIElement; readonly field: HTMLInputElement; readonly choice: HTMLSelectElement; readonly deep: HTMLElement; readonly id: string } {
  const sheet = document.createElement("style");
  sheet.textContent = SHEET;
  document.head.append(sheet);
  const list = document.createElement("ul");
  const row = place(document.createElement("li"), ROW);
  row.className = "row";
  row.id = "row-1";
  row.setAttribute("data-pet-prop", "heating/heating-load-and-demand");
  row.setAttribute("data-quiz-item", "wall-geg");
  row.setAttribute("data-presence-anchor", "item:wall-geg");
  row.setAttribute("style", "color: rgb(200, 0, 0); opacity: 0.9");
  row.setAttribute("title", "a row");
  row.setAttribute("onclick", "void 0");
  const label = place(document.createElement("label"), { left: 128, top: 205, width: 120, height: 20 });
  label.className = "label";
  label.htmlFor = "field-1";
  label.textContent = "Heating load";
  const field = place(document.createElement("input"), { left: 256, top: 205, width: 100, height: 20 });
  field.id = "field-1";
  field.name = "guess";
  field.setAttribute("form", "answers");
  field.setAttribute("autofocus", "");
  field.setAttribute("aria-label", "Guess");
  field.value = "12.5";
  const choice = place(document.createElement("select"), { left: 360, top: 205, width: 80, height: 20 });
  choice.name = "category";
  for (const text of ["walls", "roofs", "windows"]) choice.append(Object.assign(document.createElement("option"), { textContent: text }));
  choice.selectedIndex = 2;
  const link = place(document.createElement("a"), { left: 444, top: 205, width: 40, height: 20 });
  link.href = "#more";
  link.setAttribute("role", "button");
  link.tabIndex = 0;
  link.textContent = "more";
  link.className = "busy";
  let deep: HTMLElement = row;
  for (let level = 0; level < LIFT_REACH; level++) {
    const span = document.createElement("span");
    span.setAttribute("data-depth", String(level + 1));
    deep.append(span);
    deep = span;
  }
  deep.textContent = "deep";
  row.prepend(label, field, choice, link);
  list.append(row);
  const host = document.createElement("div");
  host.className = "pet-layer";
  host.setAttribute("aria-hidden", "true");
  document.body.append(list, host);
  return { host, row, field, choice, deep, id: fixtureId(row) };
}

function lift(fixture: string, change: Partial<LiftFrame> = {}): LiftFrame {
  return { fixture, dx: 24, dy: -1.5, tilt: 0.002, opacity: 1, ...change };
}

/** 🧾️ The `outerHTML` of an element with its own inline style left out, and that inline style alone as its declarations (a style written through the CSSOM is serialised anew, so its text may differ where its declarations do not). */
function parts(element: Element & ElementCSSInlineStyle): { readonly markup: string; readonly style: readonly string[] } {
  const clone = element.cloneNode(true) as Element;
  clone.removeAttribute("style");
  const style = element.style;
  return { markup: clone.outerHTML, style: Array.from({ length: style.length }, (_, index) => style.item(index)).map((name) => `${name}: ${style.getPropertyValue(name)} ${style.getPropertyPriority(name)}`).sort() };
}

/** 🔮️ gl-matrix: the corners of a box of `width` × `height` at the layer's corner, moved by a CSS `transform` about its middle (`transform-origin: 50% 50%`). */
function corners(transform: string, width: number, height: number): vec2[] {
  const matrix = mat2d.fromTranslation(mat2d.create(), [width / 2, height / 2]);
  for (const [, name, body] of transform.matchAll(/(\w+)\(([^)]*)\)/gu)) {
    const numbers = body!.split(/[\s,]+/u).map((entry) => Number.parseFloat(entry));
    if (name === "translate") mat2d.translate(matrix, matrix, [numbers[0]!, numbers[1]!]);
    else if (name === "rotate") mat2d.rotate(matrix, matrix, glMatrix.toRadian(numbers[0]!));
    else throw new Error(`unexpected ${name}`);
  }
  mat2d.translate(matrix, matrix, [-width / 2, -height / 2]);
  return [
    [0, 0],
    [width, 0],
    [width, height],
    [0, height],
  ].map((corner) => vec2.transformMat2d(vec2.create(), corner as vec2, matrix));
}

/** 📒️ Everything listening on a target, added and not yet removed, with the options it was added with. */
function ledger(target: EventTarget): { readonly open: () => string[]; readonly options: () => unknown[] } {
  const open: { readonly type: string; readonly listener: unknown; readonly capture: boolean }[] = [];
  const seen: unknown[] = [];
  const capture = (options: unknown): boolean => (typeof options === "boolean" ? options : ((options as AddEventListenerOptions | undefined)?.capture ?? false));
  const add = target.addEventListener.bind(target);
  const remove = target.removeEventListener.bind(target);
  vi.spyOn(target, "addEventListener").mockImplementation((type: string, listener: EventListenerOrEventListenerObject | null, options?: boolean | AddEventListenerOptions) => {
    open.push({ type, listener, capture: capture(options) });
    seen.push(options);
    add(type, listener, options);
  });
  vi.spyOn(target, "removeEventListener").mockImplementation((type: string, listener: EventListenerOrEventListenerObject | null, options?: boolean | EventListenerOptions) => {
    const index = open.findIndex((entry) => entry.type === type && entry.listener === listener && entry.capture === capture(options));
    if (index >= 0) open.splice(index, 1);
    remove(type, listener, options);
  });
  return { open: () => open.map((entry) => entry.type).sort(), options: () => seen };
}

function event(type: string): Event {
  return type.startsWith("pointer") || type === "click" ? new MouseEvent(type, { bubbles: true, cancelable: true }) : type === "keydown" ? new KeyboardEvent(type, { bubbles: true, key: "a" }) : type === "focusin" ? new FocusEvent(type, { bubbles: true }) : new Event(type, { bubbles: true, cancelable: true });
}

const CONSOLE = ["log", "info", "warn", "error", "debug", "trace"] as const;
let spoken: MockInstance[] = [];
let running: Lifting[] = [];

function lifting(host: HTMLElement, quiet = false): { readonly lifting: Lifting; readonly heard: string[] } {
  const heard: string[] = [];
  const started = liftFixtures(host, { quiet: () => quiet, reclaimed: (fixture) => void heard.push(fixture) });
  running.push(started);
  return { lifting: started, heard };
}

beforeEach(() => {
  spoken = CONSOLE.map((method) => vi.spyOn(console, method).mockImplementation(() => {}));
});

afterEach(() => {
  for (const started of running) started.end();
  running = [];
  for (const [index, spy] of spoken.entries()) expect(spy.mock.calls, `console.${CONSOLE[index]}`).toEqual([]);
  document.body.replaceChildren();
  document.head.replaceChildren();
  vi.restoreAllMocks();
});

describe("🪞️ fixture lifting", () => {
  it("copies an element with the computed look of every node and nothing anyone could find or reach it by", () => {
    const { row, field, choice } = scene();
    const copy = copyOf(row, window);
    expect(copy.isConnected).toBe(false);
    expect(copy.localName).toBe("li");
    const sources = [row, ...row.querySelectorAll("*")];
    const made = [copy, ...copy.querySelectorAll("*")] as (Element & ElementCSSInlineStyle)[];
    expect(made.map((node) => node.localName)).toEqual(sources.map((node) => node.localName));
    expect(copy.textContent).toBe(row.textContent);
    for (const [index, node] of made.entries()) {
      for (const name of node.getAttributeNames()) expect(["class", "style", "type", "value", "href", "inert", "aria-hidden", "selected"].includes(name) && !(name === "href" && node.localName === "a"), `${node.localName} ${name}`).toBe(true);
      const look = getComputedStyle(sources[index]!);
      for (const name of LIFT_LOOK) {
        if (index === 0 && /^(?:position|left|top|right|bottom|margin-.*)$/u.test(name)) continue;
        if (look.getPropertyValue(name) !== "") expect(node.style.getPropertyValue(name), `${node.localName} ${name}`).toBe(look.getPropertyValue(name));
      }
      expect(node.style.getPropertyValue("pointer-events"), node.localName).toBe("none");
      expect(node.style.getPropertyValue("animation-name"), node.localName).toBe("none");
    }
    expect(copy.style.getPropertyValue("color")).toBe("rgb(200, 0, 0)");
    expect(copy.style.getPropertyValue("font-size")).toBe("14px");
    expect(copy.querySelector("label")!.style.getPropertyValue("font-style")).toBe("italic");
    expect([copy.style.width, copy.style.height, copy.style.boxSizing, copy.style.position, copy.style.left, copy.style.top, copy.style.margin, copy.style.transformOrigin]).toEqual(["400px", "48px", "border-box", "absolute", "0px", "0px", "0px", "50% 50%"]);
    expect([copy.querySelector("input")!.style.width, copy.querySelector("select")!.style.width]).toEqual(["100px", "80px"]);
    expect(copy.getAttribute("inert")).toBe("");
    expect(copy.getAttribute("aria-hidden")).toBe("true");
    expect((copy.querySelector("input") as HTMLInputElement).value).toBe(field.value);
    expect((copy.querySelector("select") as HTMLSelectElement).selectedIndex).toBe(choice.selectedIndex);
    expect(copy.querySelector("[id], [name], [for], [form], [onclick], [autofocus], [tabindex], [role], [title], [href], [aria-label], [data-pet-prop], [data-depth]")).toBeNull();
  });

  it("realises a lift without touching the original: the same place, content, attributes, focus and listeners, only transparent while the copy shows all of it", () => {
    const { host, row, field, id } = scene();
    const before = parts(row);
    const clicks: string[] = [];
    row.addEventListener("click", () => clicks.push("row"));
    const { lifting: lifted, heard } = lifting(host);
    lifted.realise([lift(id, { opacity: 0.5 })], ORIGIN, 1);
    expect(host.childElementCount).toBe(1);
    const copy = host.firstElementChild as HTMLElement;
    expect(lifted.copies.has(copy)).toBe(true);
    expect(parts(row).markup).toBe(before.markup);
    expect(row.style.getPropertyValue("opacity")).toBe("0.9");
    expect(row.style.getPropertyValue("transition-property")).toBe("none");
    expect(copy.style.opacity).toBe("0.45");
    lifted.realise([lift(id)], ORIGIN, 1);
    expect(parts(row).markup).toBe(before.markup);
    expect([row.style.getPropertyValue("opacity"), row.style.getPropertyPriority("opacity")]).toEqual(["0", "important"]);
    expect(copy.style.opacity).toBe("0.9");
    expect(field.tabIndex).toBe(0);
    expect(row.isConnected && row.parentElement?.localName).toBe("ul");
    lifted.realise([lift(id, { opacity: 0.2 })], ORIGIN, 1);
    expect(row.style.getPropertyValue("opacity")).toBe("0.9");
    lifted.realise([], ORIGIN, 1);
    expect(host.childElementCount).toBe(0);
    expect(lifted.copies.size).toBe(0);
    expect(parts(row)).toEqual(before);
    expect(heard).toEqual([]);
    row.click();
    expect(clicks).toEqual(["row"]);

    const bare = document.createElement("li");
    place(bare, ROW).setAttribute("data-pet-prop", "heating");
    document.body.append(bare);
    const outer = bare.outerHTML;
    lifted.realise([lift(fixtureId(bare))], ORIGIN, 1);
    expect(bare.getAttribute("style")).not.toBeNull();
    lifted.realise([], ORIGIN, 1);
    expect(bare.outerHTML).toBe(outer);
  });

  it("places the copy at the original's box plus the lift in the pets' pixels, turned about its middle as gl-matrix composes it", () => {
    const { host, id } = scene();
    const { lifting: lifted } = lifting(host);
    const origin = { x: 10, y: 4 };
    for (const [size, change] of [
      [1, { dx: 24, dy: -1.5, tilt: 0.002 }],
      [0.8, { dx: -30, dy: 2, tilt: -0.004 }],
      [1, { dx: 0, dy: 0, tilt: 0 }],
    ] as const) {
      lifted.realise([lift(id, change)], origin, size);
      const copy = host.firstElementChild as HTMLElement;
      const drawn = corners(copy.style.transform, ROW.width, ROW.height);
      const middle = vec2.fromValues(ROW.left - origin.x + change.dx * size + ROW.width / 2, ROW.top - origin.y + change.dy * size + ROW.height / 2);
      const turn = change.tilt * 2 * Math.PI;
      const meant = [
        [-1, -1],
        [1, -1],
        [1, 1],
        [-1, 1],
      ].map(([sx, sy]) => {
        const hx = (sx! * ROW.width) / 2;
        const hy = (sy! * ROW.height) / 2;
        return vec2.fromValues(middle[0]! + hx * Math.cos(turn) - hy * Math.sin(turn), middle[1]! + hx * Math.sin(turn) + hy * Math.cos(turn));
      });
      for (const [index, corner] of drawn.entries()) expect(vec2.distance(corner, meant[index]!), `${size} ${index}`).toBeLessThan(0.05);
    }
  });

  for (const type of RECLAIMS) {
    it(`gives the original back in the same task on ${type} on it or inside it, and tells the stage`, () => {
      const { host, row, deep, field, choice, id } = scene();
      const before = parts(row);
      const { lifting: lifted, heard } = lifting(host);
      lifted.realise([lift(id)], ORIGIN, 1);
      document.body.dispatchEvent(event(type));
      host.dispatchEvent(event(type));
      expect(host.childElementCount).toBe(1);
      expect(heard).toEqual([]);
      const target = type === "input" ? field : type === "change" ? choice : deep;
      target.dispatchEvent(event(type));
      expect(host.childElementCount).toBe(0);
      expect(parts(row)).toEqual(before);
      expect(heard).toEqual([id]);
    });
  }

  it("gives the original back when focus moves into it, when a script fills a field or picks an option, and looks no further than its reach", () => {
    const { host, row, deep, field, choice, id } = scene();
    const before = parts(row);
    const { lifting: lifted, heard } = lifting(host);
    lifted.realise([lift(id)], ORIGIN, 1);
    field.focus();
    expect([host.childElementCount, heard]).toEqual([0, [id]]);
    field.blur();
    lifted.realise([], ORIGIN, 1);
    lifted.realise([lift(id)], ORIGIN, 1);
    field.value = "13";
    field.dispatchEvent(new Event("input", { bubbles: true }));
    expect([host.childElementCount, heard]).toEqual([0, [id, id]]);
    lifted.realise([], ORIGIN, 1);
    lifted.realise([lift(id)], ORIGIN, 1);
    choice.selectedIndex = 1;
    choice.dispatchEvent(new Event("change", { bubbles: true }));
    expect([host.childElementCount, heard]).toEqual([0, [id, id, id]]);
    lifted.realise([], ORIGIN, 1);
    const deeper = deep.appendChild(document.createElement("b"));
    lifted.realise([lift(id)], ORIGIN, 1);
    deeper.dispatchEvent(event("pointerdown"));
    expect(host.childElementCount, `${LIFT_REACH + 1} levels below the original`).toBe(1);
    deep.dispatchEvent(event("pointerdown"));
    expect(host.childElementCount).toBe(0);
    deeper.remove();
    expect(parts(row)).toEqual(before);
  });

  it("lifts nothing the stage was told about until a frame comes that no longer holds it", () => {
    const { host, deep, id } = scene();
    const { lifting: lifted, heard } = lifting(host);
    lifted.realise([lift(id)], ORIGIN, 1);
    deep.dispatchEvent(event("pointerover"));
    lifted.realise([lift(id)], ORIGIN, 1);
    lifted.realise([lift(id, { dx: 30 })], ORIGIN, 1);
    expect(host.childElementCount).toBe(0);
    expect(heard).toEqual([id]);
    lifted.realise([], ORIGIN, 1);
    lifted.realise([lift(id)], ORIGIN, 1);
    expect(host.childElementCount).toBe(1);
  });

  it("takes every lift back on any input while the stage is quiet", () => {
    const { host, row, id } = scene();
    const other = place(document.createElement("p"), { left: 0, top: 400, width: 300, height: 40 });
    other.setAttribute("data-pet-prop", "cooling");
    document.body.append(other);
    const { lifting: lifted, heard } = lifting(host, true);
    lifted.realise([lift(id), lift(fixtureId(other))], ORIGIN, 1);
    expect(host.childElementCount).toBe(2);
    document.body.dispatchEvent(event("keydown"));
    expect(host.childElementCount).toBe(0);
    expect(heard.sort()).toEqual([id, fixtureId(other)].sort());
    expect(row.style.getPropertyValue("opacity")).toBe("0.9");
  });

  it("takes every lift back when the document is hidden or goes away, and when the original moves, leaves or hides", () => {
    const { host, row, id } = scene();
    const { lifting: lifted, heard } = lifting(host);
    let state: DocumentVisibilityState = "visible";
    vi.spyOn(document, "visibilityState", "get").mockImplementation(() => state);
    lifted.realise([lift(id)], ORIGIN, 1);
    document.dispatchEvent(new Event("visibilitychange"));
    expect(host.childElementCount).toBe(1);
    state = "hidden";
    document.dispatchEvent(new Event("visibilitychange"));
    expect([host.childElementCount, heard]).toEqual([0, [id]]);
    lifted.realise([], ORIGIN, 1);
    lifted.realise([lift(id)], ORIGIN, 1);
    window.dispatchEvent(new Event("pagehide"));
    expect([host.childElementCount, heard]).toEqual([0, [id, id]]);
    lifted.realise([], ORIGIN, 1);
    lifted.realise([lift(id)], ORIGIN, 1);
    lifted.measured();
    expect(host.childElementCount).toBe(1);
    place(row, { ...ROW, top: ROW.top + 0.4 });
    lifted.measured();
    expect(host.childElementCount).toBe(1);
    place(row, { ...ROW, top: ROW.top + 30 });
    lifted.measured();
    expect([host.childElementCount, heard]).toEqual([0, [id, id, id]]);
    lifted.realise([], ORIGIN, 1);
    lifted.realise([lift(id)], ORIGIN, 1);
    row.parentElement!.setAttribute("inert", "");
    lifted.measured();
    expect([host.childElementCount, heard.length]).toEqual([0, 4]);
    row.parentElement!.removeAttribute("inert");
    lifted.realise([], ORIGIN, 1);
    lifted.realise([lift(id)], ORIGIN, 1);
    lifted.release();
    expect([host.childElementCount, heard.length]).toEqual([0, 5]);
    expect(row.style.getPropertyValue("opacity")).toBe("0.9");
  });

  it("refuses what it cannot copy, what is in use and what is gone, and tells the stage at once", () => {
    const { host, field, id } = scene();
    const { lifting: lifted, heard } = lifting(host);
    field.focus();
    lifted.realise([lift(id)], ORIGIN, 1);
    expect([host.childElementCount, heard]).toEqual([0, [id]]);
    field.blur();
    const big = place(document.createElement("div"), ROW);
    big.setAttribute("data-pet-prop", "physics");
    for (let index = 1; index < FIXTURE_ELEMENTS; index++) big.append(document.createElement("i"));
    document.body.append(big);
    expect(copyable(big)).toBe(true);
    big.append(document.createElement("i"));
    expect(copyable(big)).toBe(false);
    const film = place(document.createElement("div"), ROW);
    film.setAttribute("data-pet-prop", "physics");
    film.append(document.createElement("span"));
    film.firstElementChild!.append(document.createElement("canvas"));
    const table = document.createElement("table");
    const line = place(table.insertRow(), ROW);
    line.setAttribute("data-pet-prop", "physics");
    const custom = place(document.createElement("div"), ROW);
    custom.setAttribute("data-pet-prop", "physics");
    custom.append(document.createElement("quiz-glyph"));
    const flat = document.createElement("div");
    flat.setAttribute("data-pet-prop", "physics");
    document.body.append(film, table, custom, flat);
    expect([film, line, custom, flat].map(copyable)).toEqual([false, false, false, true]);
    const strangers = [big, film, line, custom, flat].map(fixtureId);
    lifted.realise([...strangers, "f-unknown"].map((fixture) => lift(fixture)), ORIGIN, 1);
    expect(host.childElementCount).toBe(0);
    expect(heard).toEqual([id, ...strangers, "f-unknown"]);
    expect(document.querySelectorAll("[style]").length).toBe(1);
  });

  it("finds its fixtures by the host's selector, never in the layer, and names each one for good", () => {
    const { host, row } = scene();
    const named = place(document.createElement("section"), { left: 0, top: 0, width: 300, height: 100 });
    named.setAttribute("data-toy", "");
    document.body.append(named);
    const heard: string[] = [];
    const toys = liftFixtures(host, { props: () => "[data-toy]", reclaimed: (fixture) => void heard.push(fixture) });
    running.push(toys);
    toys.realise([lift(fixtureId(row))], ORIGIN, 1);
    expect([host.childElementCount, heard]).toEqual([0, [fixtureId(row)]]);
    toys.realise([lift(fixtureId(named))], ORIGIN, 1);
    expect(host.childElementCount).toBe(1);
    expect(fixtureId(named)).toBe(fixtureId(named));
    expect(fixtureId(named)).not.toBe(fixtureId(row));
    expect(PET_PROPS).toBe("[data-pet-prop]");
  });

  it("keeps the copy out of the accessibility tree, out of every hook of the host and out of the pointer's way", () => {
    const { host, id } = scene();
    const { lifting: lifted } = lifting(host);
    lifted.realise([lift(id)], ORIGIN, 1);
    const nodes = [...host.querySelectorAll("*")];
    expect(nodes.length).toBeGreaterThan(LIFT_REACH);
    expect(getRoles(host)).toEqual({});
    for (const node of nodes) {
      expect(isInaccessible(node), node.localName).toBe(true);
      expect(node.closest("[inert]"), node.localName).not.toBeNull();
      for (const name of ["id", "name", "tabindex", "role", "title", "href", "contenteditable", "draggable", "for", "form", "autofocus", "aria-label"]) expect(node.hasAttribute(name), `${node.localName} ${name}`).toBe(false);
      expect(getComputedStyle(node).pointerEvents, node.localName).toBe("none");
    }
    for (const selector of ["[data-quiz-item]", "[data-presence-anchor]", "[data-pet-prop]", "[id]", "[name]", "[onclick]"]) for (const found of document.querySelectorAll(selector)) expect(host.contains(found), selector).toBe(false);
  });

  it("listens only while something is lifted, in the capture phase and passively, and leaves nothing behind", () => {
    const { host, deep, id } = scene();
    const page = ledger(document);
    const view = ledger(window);
    const { lifting: lifted } = lifting(host);
    lifted.realise([], ORIGIN, 1);
    expect([page.open(), view.open()]).toEqual([[], []]);
    lifted.realise([lift(id)], ORIGIN, 1);
    expect(page.open()).toEqual([...RECLAIMS, "visibilitychange"].sort());
    expect(view.open()).toEqual(["pagehide"]);
    for (const options of [...page.options(), ...view.options()]) expect(options).toEqual({ capture: true, passive: true });
    const press = event("pointerdown");
    deep.dispatchEvent(press);
    expect(press.defaultPrevented).toBe(false);
    expect([page.open(), view.open()]).toEqual([[], []]);
    lifted.realise([], ORIGIN, 1);
    lifted.realise([lift(id)], ORIGIN, 1);
    lifted.end();
    expect([page.open(), view.open(), host.childElementCount]).toEqual([[], [], 0]);
    lifted.realise([], ORIGIN, 1);
    expect([page.open(), view.open()]).toEqual([[], []]);
  });

  it("writes nothing when a frame changes nothing, and only what a frame changes", () => {
    const { host, row, id } = scene();
    const { lifting: lifted } = lifting(host);
    lifted.realise([lift(id)], ORIGIN, 1);
    const records = (act: () => void): string[] => {
      const observer = new MutationObserver(() => {});
      observer.observe(document.body, { attributes: true, childList: true, subtree: true, characterData: true });
      act();
      const taken = observer.takeRecords();
      observer.disconnect();
      return taken.map((record) => `${record.type}:${(record.target as Element).localName}.${record.attributeName ?? ""}`);
    };
    expect(records(() => lifted.realise([lift(id, { dx: 24.001, tilt: 0.0020001 })], ORIGIN, 1))).toEqual([]);
    expect(records(() => lifted.realise([lift(id, { dx: 25 })], ORIGIN, 1))).toEqual(["attributes:li.style"]);
    expect(records(() => lifted.realise([lift(id, { dx: 25, opacity: 0.5 })], ORIGIN, 1))).toEqual(["attributes:li.style", "attributes:li.style"]);
    expect(row.style.getPropertyValue("opacity")).toBe("0.9");
    expect(records(() => lifted.realise([lift(id, { dx: 25, opacity: 0.5 })], ORIGIN, 1))).toEqual([]);
  });

  it("never writes in a way a policy without inline styles blocks: the copy loses the style attributes it was cloned with and gets its look through the CSSOM", () => {
    const setAttribute = vi.spyOn(Element.prototype, "setAttribute");
    const removeAttribute = vi.spyOn(Element.prototype, "removeAttribute");
    const innerHTML = vi.spyOn(Element.prototype, "innerHTML", "set");
    const outerHTML = vi.spyOn(Element.prototype, "outerHTML", "set");
    const cssText = vi.spyOn(CSSStyleDeclaration.prototype, "cssText", "set");
    const { host, row, deep, id } = scene();
    setAttribute.mockClear();
    cssText.mockClear();
    const { lifting: lifted } = lifting(host);
    lifted.realise([lift(id, { opacity: 0.4 })], ORIGIN, 1);
    lifted.realise([lift(id)], ORIGIN, 1);
    deep.dispatchEvent(event("pointerover"));
    expect(setAttribute.mock.calls.map(([name]) => name).sort()).toEqual(["aria-hidden", "inert"]);
    expect(innerHTML).not.toHaveBeenCalled();
    expect(outerHTML).not.toHaveBeenCalled();
    const unstyled = removeAttribute.mock.calls.flatMap(([name], index) => (name === "style" ? [removeAttribute.mock.contexts[index] as Element] : []));
    expect(unstyled.map((element) => [element.localName, element === row, row.contains(element)])).toEqual([["li", false, false]]);
    expect(cssText.mock.calls, "only the engine's own echo of the style attribute the clone was born with, and of its removal").toEqual([["color: rgb(200, 0, 0); opacity: 0.9"], [null]]);
  });
});
