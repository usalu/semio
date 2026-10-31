/** 📐️ The adaptive layout: every part of the client lays itself out for the room it has — its card, its task, its
 * list — and never for the window. The tiers of the shared vectors (which part measures which room, and from how many
 * rem its roomier layout holds) must be exactly the container rules of the stylesheet, read by `lightningcss` (the CSS
 * engine Tailwind itself runs on) as the third-party oracle; every room must be a named inline-size container; a
 * folding table must fold by the one query of its frame, whose font size is its yardstick; a figure must fold below
 * the width its items and columns need; and the parts of every row must stand in the source in their reading order,
 * whatever line the styles put them on. jsdom lays nothing out, so what a learner sees at each width is held by the
 * walk of the site in a real browser.
 *
 * @see ../../🧫️fixtures/📐️adaptive-layout/🔣️.json
 * @see ../../🎯️targets/⚛️react/🎨️.css — the adaptive layouts
 * @see ../../../../../🎓️teaching/🏛️architecture/📚️course/🧪️tests/📐️adaptive-layout/🟦️.ts — the walk in a real browser
 */

import { cleanup, render, screen, within } from "@testing-library/react";
import { transform } from "lightningcss";
import { afterEach, describe, expect, it, vi } from "vitest";
import { act, waitFor } from "@testing-library/react";
import Ajv2020 from "ajv/dist/2020.js";
import { DatabaseSync } from "node:sqlite";
import fitting from "../🧫️fixtures/🗃️record-fitting/🔣️.json";
import fittingSchema from "../🧫️fixtures/🗃️record-fitting/📋️schema.json";
import type { SheetClassificationTask, SheetMatchingTask, SheetSortingTask } from "@semio-tech/course";
import { ClassificationTaskView, MatchingTaskView, Records, SortingTaskView, TABLE, plotFold, courseText } from "@semio-tech/course-react";
import { quizStylesheet as stylesheet } from "@semio-tech/course-react/stylesheet";
import vectors from "../🧫️fixtures/📐️adaptive-layout/🔣️.json";

interface Fixture {
  readonly rooms: Readonly<Record<string, string>>;
  readonly tiers: readonly { readonly part: string; readonly room: string; readonly from: number }[];
  readonly fold: { readonly room: string; readonly query: number; readonly unit: string; readonly yardstick: number };
  readonly plot: { readonly item: number; readonly column: number; readonly size: number; readonly vectors: readonly { readonly tracks: number; readonly fold: number }[] };
  readonly order: Readonly<Record<string, readonly string[]>>;
}

const fixture: Fixture = vectors;
const text = (en: string, de: string) => ({ en, de });

/** 📦️ One `@container` rule as lightningcss reads it: the room it asks, how it compares its width with which length,
 * and the class every style rule inside starts with. */
interface RoomRule {
  readonly room: string;
  readonly operator: string;
  readonly unit: string;
  readonly value: number;
  readonly parts: readonly string[];
}

interface ParsedStyle {
  readonly type: string;
  readonly value: { readonly selectors?: readonly (readonly { readonly type: string; readonly name?: string; readonly operation?: { readonly value: string } }[])[]; readonly declarations?: { readonly declarations: readonly { readonly property: string; readonly value: unknown }[] } };
}

interface ParsedContainer {
  readonly name: string;
  readonly condition: { readonly type: string; readonly value: { readonly type: string; readonly name: string; readonly operator: string; readonly value: { readonly value: { readonly value: { readonly unit: string; readonly value: number } } } } };
  readonly rules: readonly ParsedStyle[];
}

/** 🔎️ Every container rule of `css` and, per selector of a plain rule, the properties it declares with their values. */
function read(css: string): { readonly rooms: readonly RoomRule[]; readonly declared: ReadonlyMap<string, Readonly<Record<string, unknown>>> } {
  const rooms: RoomRule[] = [];
  const declared = new Map<string, Record<string, unknown>>();
  const named = (selector: readonly { readonly type: string; readonly name?: string; readonly operation?: { readonly value: string } }[]): string => selector.map((part) => (part.type === "class" ? `.${part.name}` : part.type === "attribute" ? `[${part.name}="${part.operation?.value}"]` : part.type === "combinator" ? " " : "?")).join("");
  transform({
    filename: "🎨️.css",
    code: new TextEncoder().encode(css),
    visitor: {
      Rule: {
        container(rule) {
          const { name, condition, rules } = rule.value as unknown as ParsedContainer;
          if (condition.type !== "feature" || condition.value.type !== "range" || condition.value.name !== "width") throw new Error(`a container rule asks something other than a width: ${JSON.stringify(condition)}`);
          const length = condition.value.value.value.value;
          const parts = rules.filter((inner) => inner.type === "style").map((inner) => inner.value.selectors?.[0]?.find((part) => part.type === "class")?.name ?? "");
          rooms.push({ room: name, operator: condition.value.operator, unit: length.unit, value: length.value, parts });
        },
        style(rule) {
          const style = rule as unknown as ParsedStyle;
          for (const selector of style.value.selectors ?? []) declared.set(named(selector), { ...declared.get(named(selector)), ...Object.fromEntries((style.value.declarations?.declarations ?? []).map((declaration) => [declaration.property, declaration.value])) });
        },
      },
    },
  });
  return { rooms, declared };
}

const sheet = read(stylesheet);

const PLACES: SheetClassificationTask = {
  kind: "classification",
  id: "places",
  title: text("Places", "Orte"),
  prompt: text("Assign each place.", "Ordne jeden Ort zu."),
  categories: [
    { id: "dry", label: text("Dry", "Trocken") },
    { id: "wet", label: text("Wet", "Nass") },
  ],
  items: [
    { id: "desert", label: text("Desert", "Wüste") },
    { id: "fjord", label: text("Fjord", "Fjord") },
  ],
};

const MASSES: SheetSortingTask = {
  kind: "sorting",
  id: "masses",
  title: text("Masses", "Massen"),
  prompt: text("Sort by mass.", "Nach Masse sortieren."),
  quantity: { label: text("Mass", "Masse"), unit: "g", scale: "logarithmic", prefixed: true, additive: true },
  keys: [20, 500_000],
  items: [
    { id: "horse", label: text("Horse", "Pferd") },
    { id: "mouse", label: text("Mouse", "Maus") },
  ],
};

const LAMPS: SheetMatchingTask = {
  kind: "matching",
  id: "lamps",
  title: text("Lamps", "Lampen"),
  prompt: text("Match each lamp its power.", "Ordne jeder Lampe ihre Leistung zu."),
  dimensions: [{ id: "power", quantity: { label: text("Power", "Leistung"), unit: "W", scale: "logarithmic", prefixed: true, additive: true }, cards: [50, 8] }],
  items: [
    { id: "led", label: text("LED bulb", "LED-Lampe") },
    { id: "halogen", label: text("Halogen spot", "Halogenstrahler") },
  ],
};

/** 🏷️ What a part of a row is, by the hook it carries. */
function partOf(element: Element): string {
  if (element.hasAttribute("data-course-grip")) return "grip";
  if (element.classList.contains("course-sort-place")) return "place";
  if (element.classList.contains("course-row-label")) return "label";
  if (element.classList.contains("course-guess")) return "guess";
  if (element.classList.contains("course-sort-key")) return "key";
  if (element.classList.contains("course-sort-up")) return "up";
  if (element.classList.contains("course-sort-down")) return "down";
  if (element.tagName === "SELECT") return "select";
  if (element.tagName === "BUTTON") return "remove";
  return element.tagName.toLowerCase();
}

function rowsOf(part: string): readonly (readonly string[])[] {
  return [...document.querySelectorAll(`.${part}`)].map((row) => [...row.children].map(partOf));
}

afterEach(cleanup);

describe("📐️ the rooms the parts measure", () => {
  it("makes every room a named inline-size container, so a part asks the width of its own card, task, list or table frame", () => {
    for (const [name, selector] of Object.entries(fixture.rooms)) {
      const rule = [...sheet.declared.entries()].find(([declared]) => declared === selector || declared.endsWith(` ${selector}`))?.[1];
      const container = rule?.container as { readonly name?: { readonly type: string; readonly value: readonly string[] }; readonly containerType?: string } | undefined;
      expect(container?.name, selector).toEqual({ type: "names", value: [name] });
      expect(container?.containerType, selector).toBe("inline-size");
    }
  });

  it("uses container queries for every part's width", () => {
    const widths = [...stylesheet.matchAll(/@media \((?:min|max)-width: [^)]*\)\s*\{\s*([^\s{]+)/gu)].map((found) => found[1]);
    expect(widths).toEqual([]);
  });
});

describe("📐️ the tiers of the layout", () => {
  it("holds every tier of the shared vectors as a container rule of its room, from its width in rem, around its part", () => {
    for (const tier of fixture.tiers) {
      const rules = sheet.rooms.filter((rule) => rule.room === tier.room && rule.operator === "greater-than-equal" && rule.unit === "rem" && rule.value === tier.from);
      expect(rules.flatMap((rule) => rule.parts), `${tier.part} from ${tier.from}rem of ${tier.room}`).toContain(tier.part);
    }
  });

  it("has no tier the shared vectors do not name", () => {
    const tiers = (rules: readonly { readonly room: string; readonly from: number }[]): readonly string[] => [...new Set(rules.map((rule) => `${rule.room} ${rule.from}`))].sort();
    expect(tiers(sheet.rooms.filter((rule) => rule.operator === "greater-than-equal").map((rule) => ({ room: rule.room, from: rule.value })))).toEqual(tiers(fixture.tiers));
    expect(sheet.rooms.filter((rule) => rule.operator === "greater-than-equal").every((rule) => rule.unit === "rem")).toBe(true);
  });

  it("folds every table by the one query of its frame, whose font size is the yardstick of its fold", () => {
    const folds = sheet.rooms.filter((rule) => rule.operator !== "greater-than-equal");
    expect(folds.length).toBeGreaterThan(0);
    for (const rule of folds) expect([rule.room, rule.operator, rule.value, rule.unit]).toEqual([fixture.fold.room, "less-than-equal", fixture.fold.query, fixture.fold.unit]);
    expect(folds.flatMap((rule) => rule.parts)).toEqual(expect.arrayContaining(["course-fold", "course-plot"]));
    expect(/\.course-records \{[^}]*\}/u.exec(stylesheet)?.[0]).toContain(`font-size: calc(var(--course-fold, ${fixture.fold.yardstick}) / ${fixture.fold.yardstick} * 1rem);`);
  });

  it("says in the frame of a folding table from how many rem it folds", () => {
    render(
      <Records fold={44}>
        <table {...TABLE.table} className="course-fold">
          <tbody {...TABLE.group}>
            <tr {...TABLE.row}>
              <th {...TABLE.name}>Desert</th>
              <td {...TABLE.cell} data-label="Solution">
                Dry
              </td>
            </tr>
          </tbody>
        </table>
      </Records>,
    );
    const table = screen.getByRole("table");
    expect(table.parentElement?.className).toBe("course-records");
    expect(table.parentElement?.style.getPropertyValue("--course-fold")).toBe("44");
    expect(within(table).getByRole("rowheader").textContent).toBe("Desert");
    expect(within(table).getByRole("cell").getAttribute("data-label")).toBe("Solution");
    expect(within(table).getByRole("row").parentElement?.getAttribute("role")).toBe("rowgroup");
  });

  it("folds a figure below the width its items and its columns need side by side", () => {
    for (const vector of fixture.plot.vectors) {
      expect(plotFold(vector.tracks), `${vector.tracks} columns`).toBe(vector.fold);
      expect((fixture.plot.item + fixture.plot.column * vector.tracks) * fixture.plot.size).toBe(vector.fold);
    }
    const plot = /\n\.course-plot \{[^}]*\}/u.exec(stylesheet)?.[0] ?? "";
    expect(plot).toContain(`min-inline-size: calc(${fixture.plot.item}em + var(--course-plot-columns, 1) * ${fixture.plot.column}em);`);
    expect(plot).toContain(`font-size: ${fixture.plot.size}rem;`);
  });
});

describe("📐️ the reading order of a row", () => {
  it("keeps the parts of a classification item in their reading order inside a list that is their room", () => {
    render(<ClassificationTaskView task={PLACES} answer={{ kind: "classification", assignments: { fjord: "wet" } }} onAnswer={() => undefined} text={courseText("en")} locale="en" />);
    expect(rowsOf("course-chip")).toEqual([fixture.order["course-chip"], fixture.order["course-chip"]]);
    expect([...document.querySelectorAll(".course-chip")].every((row) => row.parentElement?.classList.contains("course-rows"))).toBe(true);
    expect(document.querySelector(".course-classify > .course-pool")).not.toBeNull();
    expect(document.querySelectorAll(".course-classify .course-bins > section")).toHaveLength(2);
  });

  it("keeps the parts of a sorting item in their reading order: the key of its place before the buttons that move it, or its guess where the keys are hidden", () => {
    render(<SortingTaskView task={MASSES} answer={undefined} onAnswer={() => undefined} text={courseText("en")} locale="en" />);
    expect(rowsOf("course-sort")).toEqual([fixture.order["course-sort"], fixture.order["course-sort"]]);
    expect(screen.getByRole("list", { name: "Order by Mass" }).classList.contains("course-rows")).toBe(true);
    cleanup();
    const { keys: _keys, ...guessed } = MASSES;
    render(<SortingTaskView task={guessed} answer={undefined} onAnswer={() => undefined} text={courseText("en")} locale="en" />);
    expect(rowsOf("course-sort")).toEqual([fixture.order["course-sort-guessed"], fixture.order["course-sort-guessed"]]);
  });

  it("keeps the parts of a matching row in their reading order, the value cards before the rows", () => {
    render(<MatchingTaskView task={LAMPS} answer={undefined} onAnswer={() => undefined} text={courseText("en")} locale="en" />);
    expect(rowsOf("course-slot")).toEqual([fixture.order["course-slot"], fixture.order["course-slot"]]);
    expect([...document.querySelector(".course-match")!.children].map((part) => part.tagName + (part.classList.contains("course-cards") ? ".cards" : part.classList.contains("course-rows") ? ".rows" : ""))).toEqual(["H3", "UL.cards", "UL.rows"]);
  });
});

/** 🗃️ Actual table demand supplements the minimum reading tier without dropping any column. */
describe("🗃️ content-driven table folding", () => {
  it("matches the closed neutral vectors and independent SQL decision", async () => {
    expect(new Ajv2020({ strict: true }).compile(fittingSchema)(fitting)).toBe(true);
    const sql = new DatabaseSync(":memory:");
    const widths = { available: 0, content: 0 };
    let changed: (() => void) | undefined;
    const client = vi.spyOn(Element.prototype, "clientWidth", "get").mockImplementation(function (this: Element) { return this.classList.contains("course-records") ? widths.available : 0; });
    const scroll = vi.spyOn(Element.prototype, "scrollWidth", "get").mockImplementation(function (this: Element) { return this.classList.contains("course-fold") ? widths.content : 0; });
    vi.stubGlobal("ResizeObserver", class { constructor(callback: () => void) { changed = callback; } observe() {} disconnect() {} });
    try {
      const view = render(<Records fold={44}><table {...TABLE.table} className="course-fold"><tbody {...TABLE.group}><tr {...TABLE.row}><th {...TABLE.name}>Physikalisches Verständnis</th><td {...TABLE.cell}>100 · Mittel</td></tr></tbody></table></Records>);
      const table = view.getByRole("table");
      const frame = table.parentElement!;
      for (const row of fitting) {
        widths.available = row.available;
        widths.content = row.content;
        table.style.setProperty("--course-records-minimum-fold", row.minimum ? "1" : "0");
        const oracle = sql.prepare("SELECT (? OR ? > ?) AS folded").get(Number(row.minimum), row.content, row.available) as { folded: number };
        expect(Boolean(oracle.folded), row.name).toBe(row.folded);
        act(() => changed?.());
        await waitFor(() => expect(frame.hasAttribute("data-folded"), row.name).toBe(row.folded));
        expect(view.getByRole("rowheader").textContent).toBe("Physikalisches Verständnis");
        expect(view.getByRole("cell").textContent).toBe("100 · Mittel");
      }
      view.unmount();
    } finally { client.mockRestore(); scroll.mockRestore(); vi.unstubAllGlobals(); sql.close(); }
  });
});

import scrollportCases from "../🧫️fixtures/📏️scrollport/🔣️.json";
import scrollportSchema from "../../🎯️targets/⚛️react/🔨️modules/🪟️chrome/📏️scrollport/🧬️schema/🔣️.json";
import { bindIntrinsicScrollport, intrinsicScrollport, scrollportFocusReveal, scrollportFocusMargins, revealScrollportFocus } from "../../🎯️targets/⚛️react/🔨️modules/🪟️chrome/📏️scrollport/🟦️.ts";


describe("📏️ intrinsic vertical scrollports", () => {
  it("keeps a whole native control reachable when natural chrome consumes the viewport", () => {
    const ajv = new Ajv2020({ strict: true }).addSchema(scrollportSchema);
    const validate = ajv.getSchema(scrollportSchema.$id)!;
    const database = new DatabaseSync(":memory:");
    try {
      expect(new Set(scrollportCases.cases.map(row => row.id)).size).toBe(scrollportCases.cases.length);
      for (const { id, expected, ...input } of scrollportCases.cases) {
        expect(validate(input), id).toBe(true);
        const actual = intrinsicScrollport({ ...input, padding: [input.padding[0]!, input.padding[1]!] });
        const chrome = database.prepare("SELECT coalesce(sum(value),0) AS amount FROM json_each(?)").get(JSON.stringify(input.chrome)) as { amount: number };
        const minimum = database.prepare("SELECT coalesce(max(value),0)+?+? AS amount FROM json_each(?)").get(input.padding[0]!, input.padding[1]!, JSON.stringify(input.controls)) as { amount: number };
        const main = Math.max(minimum.amount, input.viewport - chrome.amount);
        const oracle = { minimum: minimum.amount, main, total: chrome.amount + main, overflow: Math.max(0, chrome.amount + main - input.viewport) };
        expect(actual, id).toEqual(expected);
        expect(actual, id).toEqual(oracle);
      }
      for (const invalid of [-1, Infinity, NaN]) expect(() => intrinsicScrollport({ viewport: invalid, chrome: [], controls: [24], padding: [0, 0] })).toThrow();
      expect(() => intrinsicScrollport({ viewport: 100, chrome: [-1], controls: [24], padding: [0, 0] })).toThrow();
      expect(() => intrinsicScrollport({ viewport: 100, chrome: [], controls: [Infinity], padding: [0, 0] })).toThrow();
    } finally {
      database.close();
    }
  });

  it("restores intrinsic probes before publication and cancels every observer without replacing focused controls", async () => {
    const app = document.createElement("div");
    app.innerHTML = '<header data-block="28"></header><main style="min-block-size:0px"><section style="display:flex;flex-direction:column"><aside data-block="46"></aside><div data-course-scrollport=""><div style="padding:8px"><button style="color:red">Sign in</button><span inert><button>Hidden</button></span></div></div></section></main><footer data-block="26"></footer>';
    document.body.append(app);
    const main = app.querySelector("main")!, port = app.querySelector<HTMLElement>("[data-course-scrollport]")!, button = port.querySelector("button")!;
    const pending = new Map<number, FrameRequestCallback>();
    let serial = 0, nativeHeight = 24;
    const disconnected = vi.fn();
    const fonts = Object.assign(new EventTarget(), { ready: Promise.resolve() });
    const priorFonts = Object.getOwnPropertyDescriptor(document, "fonts");
    Object.defineProperty(document, "fonts", { configurable: true, value: fonts });
    Object.defineProperty(app, "clientHeight", { configurable: true, value: 100 });
    vi.stubGlobal("ResizeObserver", class { observe() {} unobserve() {} disconnect() { disconnected(); } });
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => { pending.set(++serial, callback); return serial; });
    vi.stubGlobal("cancelAnimationFrame", (id: number) => pending.delete(id));
    vi.spyOn(HTMLElement.prototype, "getClientRects").mockImplementation(function(this: HTMLElement) { return [this.getBoundingClientRect()] as unknown as DOMRectList; });
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function(this: HTMLElement) {
      const height = this === button ? this.style.blockSize === "max-content" ? nativeHeight : 120 : Number(this.dataset.block ?? 0);
      return { x: 0, y: 0, top: 0, left: 0, width: 200, height, right: 200, bottom: height, toJSON: () => ({}) };
    });
    let close: (() => void) | undefined;
    try {
      button.focus();
      close = bindIntrinsicScrollport(main);
      expect(port.style.minBlockSize).toBe("40px");
      expect(main.style.minBlockSize).toBe("86px");
      expect(button.style.blockSize).toBe("");
      expect(button.style.flex).toBe("");
      expect(button.style.alignSelf).toBe("");
      expect(button.style.color).toBe("red");
      expect(document.activeElement).toBe(button);
      await Promise.resolve();
      const initial = [...pending.values()];
      pending.clear();
      for (const callback of initial) callback(0);
      nativeHeight = 48;
      fonts.dispatchEvent(new Event("loadingdone"));
      window.dispatchEvent(new Event("resize"));
      expect(pending.size).toBe(1);
      const callback = [...pending.values()][0]!;
      pending.clear();
      callback(1);
      expect(port.style.minBlockSize).toBe("64px");
      expect(main.style.minBlockSize).toBe("110px");
      expect(port.querySelector("button")).toBe(button);
      expect(document.activeElement).toBe(button);
      window.dispatchEvent(new Event("resize"));
      expect(pending.size).toBe(1);
      close();
      close = undefined;
      expect(pending.size).toBe(0);
      expect(disconnected).toHaveBeenCalledOnce();
      expect(main.style.minBlockSize).toBe("0px");
      expect(port.style.minBlockSize).toBe("");
      fonts.dispatchEvent(new Event("loadingdone"));
      window.dispatchEvent(new Event("resize"));
      expect(pending.size).toBe(0);
    } finally {
      close?.();
      app.remove();
      if (priorFonts === undefined) Reflect.deleteProperty(document, "fonts");
      else Object.defineProperty(document, "fonts", priorFonts);
      vi.restoreAllMocks();
      vi.unstubAllGlobals();
    }
  });

});

  it("reveals owned clipped focus only after a changed layout, independently of manual scrolling", () => {
    const validate = new Ajv2020({ strict: true }).compile({ ...scrollportSchema, $ref: "#/$defs/FocusVisibility" });
    const sql = new DatabaseSync(":memory:");
    try {
      for (const { id, expected, ...input } of scrollportCases.focusCases) {
        expect(validate(input), id).toBe(true);
        const oracle = sql.prepare("SELECT (? AND ? AND EXISTS(SELECT 1 FROM json_each(?) WHERE ? < json_extract(value, '$[0]') OR ? < json_extract(value, '$[1]') OR ? > json_extract(value, '$[2]') OR ? > json_extract(value, '$[3]'))) AS reveal").get(Number(input.changed), Number(input.eligible), JSON.stringify(input.clips), ...input.control) as { reveal: number };
        expect(Boolean(oracle.reveal), id).toBe(expected);
        expect(scrollportFocusReveal(input), id).toBe(expected);
      }
    } finally { sql.close(); }
  });

it("uses native nearest scrolling after changed geometry without reclaiming manual scroll or focus", async () => {
  const app = document.createElement("div");
  app.style.overflowY = "auto";
  app.innerHTML = '<main><div data-course-scrollport=""><button>Continue</button></div></main><button>Outside main</button>';
  document.body.append(app);
  const main = app.querySelector("main")!, port = main.querySelector<HTMLElement>("[data-course-scrollport]")!, button = port.querySelector("button")!, outside = app.lastElementChild as HTMLButtonElement;
  let height = 100, top = 10, sequence = 0;
  const pending = new Map<number, FrameRequestCallback>();
  const flush = () => { const callbacks = [...pending.values()]; pending.clear(); callbacks.forEach(callback => callback(0)); };
  Object.defineProperties(app, { clientHeight: { get: () => height }, clientWidth: { value: 200 } });
  Object.defineProperties(main, { clientHeight: { value: 300 }, clientWidth: { value: 200 } });
  Object.defineProperties(port, { clientHeight: { value: 100 }, clientWidth: { value: 200 } });
  vi.stubGlobal("ResizeObserver", class { observe() {} unobserve() {} disconnect() {} });
  vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => { pending.set(++sequence, callback); return sequence; });
  vi.stubGlobal("cancelAnimationFrame", (id: number) => pending.delete(id));
  vi.spyOn(HTMLElement.prototype, "getClientRects").mockImplementation(function(this: HTMLElement) { return [this.getBoundingClientRect()] as unknown as DOMRectList; });
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function(this: HTMLElement) {
    const y = this === button ? top : 0, h = this === app ? height : this === button ? 24 : 300;
    return { x: 0, y, top: y, left: 0, width: 200, height: h, right: 200, bottom: y + h, toJSON: () => ({}) };
  });
  const reveal = vi.fn(() => { for (const edge of ["top", "right", "bottom", "left"]) expect(parseFloat(button.style.getPropertyValue("scroll-margin-" + edge))).toBe(1 / window.devicePixelRatio); top = 10; });
  Object.defineProperty(button, "scrollIntoView", { configurable: true, value: reveal });
  button.focus();
  const refocus = vi.spyOn(button, "focus");
  let close: (() => void) | undefined;
  try {
    close = bindIntrinsicScrollport(main);
    await Promise.resolve();
    flush();
    expect(reveal).not.toHaveBeenCalled();
    top = 120;
    app.scrollTop = 90;
    window.dispatchEvent(new Event("resize"));
    flush();
    expect(reveal).not.toHaveBeenCalled();
    expect(app.scrollTop).toBe(90);
    height = 80;
    window.dispatchEvent(new Event("resize"));
    flush();
    expect(reveal).toHaveBeenCalledExactlyOnceWith({ block: "nearest", inline: "nearest" });
    expect(main.querySelector("button")).toBe(button);
    expect(document.activeElement).toBe(button);
    expect(refocus).not.toHaveBeenCalled();
    top = 120;
    window.dispatchEvent(new Event("resize"));
    flush();
    expect(reveal).toHaveBeenCalledTimes(1);
    outside.focus();
    height = 70;
    window.dispatchEvent(new Event("resize"));
    flush();
    expect(reveal).toHaveBeenCalledTimes(1);
    expect(document.activeElement).toBe(outside);
    close();
    close = undefined;
    height = 60;
    window.dispatchEvent(new Event("resize"));
    expect(pending.size).toBe(0);
  } finally {
    close?.();
    app.remove();
    vi.restoreAllMocks();
    vi.unstubAllGlobals();
  }
});

it("adds exactly one physical pixel to every authored scroll margin without changing clipping decisions", () => {
  const validate = new Ajv2020({ strict: true }).compile({ ...scrollportSchema, $ref: "#/$defs/FocusAlignment" });
  const sql = new DatabaseSync(":memory:");
  try {
    for (const { id, expected, ...input } of scrollportCases.marginCases) {
      expect(validate(input), id).toBe(true);
      const oracle = sql.prepare("SELECT value + 1.0 / ? AS margin FROM json_each(?) ORDER BY CAST(key AS INTEGER)").all(input.pixelRatio, JSON.stringify(input.margins)) as { margin: number }[];
      expect(scrollportFocusMargins(input), id).toEqual(expected);
      expect(scrollportFocusMargins(input), id).toEqual(oracle.map(row => row.margin));
    }
    for (const pixelRatio of [0, -1, Infinity, NaN, Number.MIN_VALUE]) expect(() => scrollportFocusMargins({ margins: [0, 0, 0, 0], pixelRatio })).toThrow(RangeError);
    expect(() => scrollportFocusMargins({ margins: [0, 0, 0], pixelRatio: 1 })).toThrow(RangeError);
    expect(() => scrollportFocusMargins({ margins: [0, NaN, 0, 0], pixelRatio: 1 })).toThrow(RangeError);
    expect(scrollportFocusReveal({ changed: true, eligible: true, control: [0, 175.46875, 100, 200.46875], clips: [[0, 0, 375, 200]] })).toBe(true);
  } finally { sql.close(); }
});

it("scopes all four native alignment margins and restores their values and priorities on success or error", () => {
  const edges = ["top", "right", "bottom", "left"] as const;
  for (const pixelRatio of [1, 2, 0.8]) for (const failed of [false, true]) {
    const button = document.createElement("button");
    button.textContent = "Continue";
    button.style.color = "blue";
    const authored = [3, 5, -2, 11], priorities = ["important", "", "", "important"];
    edges.forEach((edge, index) => button.style.setProperty("scroll-margin-" + edge, authored[index] + "px", priorities[index]));
    document.body.append(button);
    button.focus();
    const focus = vi.spyOn(button, "focus"), primary = Error("native alignment failed");
    const scroll = vi.fn((options: ScrollIntoViewOptions) => {
      expect(options).toEqual({ block: "nearest", inline: "nearest" });
      edges.forEach((edge, index) => {
        expect(parseFloat(button.style.getPropertyValue("scroll-margin-" + edge))).toBeCloseTo(authored[index]! + 1 / pixelRatio);
        expect(button.style.getPropertyPriority("scroll-margin-" + edge)).toBe("important");
      });
      button.style.color = "purple";
      if (failed) throw primary;
    });
    Object.defineProperty(button, "scrollIntoView", { value: scroll });
    try {
      let error: unknown;
      try { revealScrollportFocus(button, pixelRatio); } catch (caught) { error = caught; }
      expect(error).toBe(failed ? primary : undefined);
      expect(scroll).toHaveBeenCalledOnce();
      edges.forEach((edge, index) => {
        expect(button.style.getPropertyValue("scroll-margin-" + edge)).toBe(authored[index] + "px");
        expect(button.style.getPropertyPriority("scroll-margin-" + edge)).toBe(priorities[index]);
      });
      expect(button.style.color).toBe("purple");
      expect(document.activeElement).toBe(button);
      expect(focus).not.toHaveBeenCalled();
    } finally { focus.mockRestore(); button.remove(); }
  }
});
