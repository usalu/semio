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
 * @see ../../../../../🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/📐️adaptive-layout/🟦️.ts — the walk in a real browser
 */

import { cleanup, render, screen, within } from "@testing-library/react";
import { transform } from "lightningcss";
import { afterEach, describe, expect, it } from "vitest";
import type { SheetClassificationTask, SheetMatchingTask, SheetSortingTask } from "@semio-tech/quiz";
import { ClassificationTaskView, MatchingTaskView, Records, SortingTaskView, TABLE, plotFold, quizText } from "@semio-tech/quiz-react";
import stylesheet from "../../🎯️targets/⚛️react/🎨️.css?raw";
import vectors from "../../🧫️fixtures/📐️adaptive-layout/🔣️.json";

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
  if (element.hasAttribute("data-quiz-grip")) return "grip";
  if (element.classList.contains("quiz-sort-place")) return "place";
  if (element.classList.contains("quiz-row-label")) return "label";
  if (element.classList.contains("quiz-guess")) return "guess";
  if (element.classList.contains("quiz-sort-key")) return "key";
  if (element.classList.contains("quiz-sort-up")) return "up";
  if (element.classList.contains("quiz-sort-down")) return "down";
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

  it("asks no room for the width of the window: the only width media queries are the home grid's", () => {
    const widths = [...stylesheet.matchAll(/@media \((?:min|max)-width: [^)]*\)\s*\{\s*([^\s{]+)/gu)].map((found) => found[1]);
    expect(widths).toEqual([".quiz-home-grid", ".quiz-pair"]);
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
    expect(folds.flatMap((rule) => rule.parts)).toEqual(expect.arrayContaining(["quiz-fold", "quiz-plot"]));
    expect(/\.quiz-records \{[^}]*\}/u.exec(stylesheet)?.[0]).toContain(`font-size: calc(var(--quiz-fold, ${fixture.fold.yardstick}) / ${fixture.fold.yardstick} * 1rem);`);
  });

  it("says in the frame of a folding table from how many rem it folds", () => {
    render(
      <Records fold={44}>
        <table {...TABLE.table} className="quiz-fold">
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
    expect(table.parentElement?.className).toBe("quiz-records");
    expect(table.parentElement?.style.getPropertyValue("--quiz-fold")).toBe("44");
    expect(within(table).getByRole("rowheader").textContent).toBe("Desert");
    expect(within(table).getByRole("cell").getAttribute("data-label")).toBe("Solution");
    expect(within(table).getByRole("row").parentElement?.getAttribute("role")).toBe("rowgroup");
  });

  it("folds a figure below the width its items and its columns need side by side", () => {
    for (const vector of fixture.plot.vectors) {
      expect(plotFold(vector.tracks), `${vector.tracks} columns`).toBe(vector.fold);
      expect((fixture.plot.item + fixture.plot.column * vector.tracks) * fixture.plot.size).toBe(vector.fold);
    }
    const plot = /\n\.quiz-plot \{[^}]*\}/u.exec(stylesheet)?.[0] ?? "";
    expect(plot).toContain(`min-inline-size: calc(${fixture.plot.item}em + var(--quiz-plot-columns, 1) * ${fixture.plot.column}em);`);
    expect(plot).toContain(`font-size: ${fixture.plot.size}rem;`);
  });
});

describe("📐️ the reading order of a row", () => {
  it("keeps the parts of a classification item in their reading order inside a list that is their room", () => {
    render(<ClassificationTaskView task={PLACES} answer={{ kind: "classification", assignments: { fjord: "wet" } }} onAnswer={() => undefined} text={quizText("en")} locale="en" />);
    expect(rowsOf("quiz-chip")).toEqual([fixture.order["quiz-chip"], fixture.order["quiz-chip"]]);
    expect([...document.querySelectorAll(".quiz-chip")].every((row) => row.parentElement?.classList.contains("quiz-rows"))).toBe(true);
    expect(document.querySelector(".quiz-classify > .quiz-pool")).not.toBeNull();
    expect(document.querySelectorAll(".quiz-classify .quiz-bins > section")).toHaveLength(2);
  });

  it("keeps the parts of a sorting item in their reading order: the key of its place before the buttons that move it, or its guess where the keys are hidden", () => {
    render(<SortingTaskView task={MASSES} answer={undefined} onAnswer={() => undefined} text={quizText("en")} locale="en" />);
    expect(rowsOf("quiz-sort")).toEqual([fixture.order["quiz-sort"], fixture.order["quiz-sort"]]);
    expect(screen.getByRole("list", { name: "Order by Mass" }).classList.contains("quiz-rows")).toBe(true);
    cleanup();
    const { keys: _keys, ...guessed } = MASSES;
    render(<SortingTaskView task={guessed} answer={undefined} onAnswer={() => undefined} text={quizText("en")} locale="en" />);
    expect(rowsOf("quiz-sort")).toEqual([fixture.order["quiz-sort-guessed"], fixture.order["quiz-sort-guessed"]]);
  });

  it("keeps the parts of a matching row in their reading order, the value cards before the rows", () => {
    render(<MatchingTaskView task={LAMPS} answer={undefined} onAnswer={() => undefined} text={quizText("en")} locale="en" />);
    expect(rowsOf("quiz-slot")).toEqual([fixture.order["quiz-slot"], fixture.order["quiz-slot"]]);
    expect([...document.querySelector(".quiz-match")!.children].map((part) => part.tagName + (part.classList.contains("quiz-cards") ? ".cards" : part.classList.contains("quiz-rows") ? ".rows" : ""))).toEqual(["H3", "UL.cards", "UL.rows"]);
  });
});
