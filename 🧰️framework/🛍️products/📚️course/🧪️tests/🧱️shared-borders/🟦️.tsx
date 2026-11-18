/** 🧱️ Shared borders in the course: wherever a bordered thing lies against its parent's edge or against a sibling, one
 * hairline is drawn and one owner draws it. A flush card has no body padding and its content is a vertical shared-border
 * group of cells; a row list is a group (flush) or a frame (framed) of cells that carry no outline of their own; a
 * segmented choice is a frame whose buttons are cells, or a flush row; a row button and a problem note of a flush card
 * have no border utility. The stylesheet draws no border on a row of a shared list and no dashed guess; cells laid side
 * by side only set the design system's `--shared-border-flow`, which `lightningcss` (the CSS engine Tailwind runs on)
 * reads as the third-party oracle.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx
 * @see ../../../../🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css — the shared-border rules
 */

import { cleanup, render, screen, within } from "@testing-library/react";
import { transform } from "lightningcss";
import { afterEach, describe, expect, it } from "vitest";
import type { SheetClassificationTask } from "@semio-tech/course";
import { BodyButton, CELL, CardSection, ClassificationTaskView, CourseCard, ProblemNote, RowList, Segments, courseText } from "@semio-tech/course-react";
import { quizStylesheet as stylesheet } from "@semio-tech/course-react/stylesheet";
import { LoadFallback } from "../../🎯️targets/⚛️react/🔨️modules/📚️app/🛟️load-fallback/🟦️.tsx";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

afterEach(cleanup);

const text = courseText("en");
const BORDER = /(?:^|\s)border(?:\s|$)/u;
const BORDERISH = /(?:^|\s)border(?:-[a-z]+)?(?:\s|$)/u;
const cells = (parent: Element): Element[] => [...parent.children].filter((child) => child.hasAttribute("data-shared-border-cell"));

describe("🧱️ shared borders", () => {
  it("makes the content of a flush card a vertical group of cells with no padding of its own", () => {
    render(
      <CourseCard flush id="flush" card="flush" icon={null} title="Flush">
        <CardSection>one</CardSection>
        <RowList>
          <li {...CELL}>row</li>
        </RowList>
      </CourseCard>,
    );
    const content = document.querySelector('[data-slot="course-card-content"]')!;
    expect(content.getAttribute("data-shared-border-group")).toBe("");
    expect(content.getAttribute("data-orientation")).toBe("vertical");
    expect(content.className).not.toMatch(/gap-double/u);
    expect(cells(content)).toHaveLength(2);
    expect(document.querySelector('[data-slot="window-chrome-body"]')?.className ?? "").not.toMatch(/p-double/u);
  });

  it("keeps an ordinary card's body one padded cell of the card's group", () => {
    render(
      <CourseCard id="plain" card="plain" icon={null} title="Plain">
        <p>text</p>
      </CourseCard>,
    );
    const content = document.querySelector('[data-slot="course-card-content"]')!;
    expect(content.hasAttribute("data-shared-border-group")).toBe(true);
    expect(content.children).toHaveLength(1);
    expect(content.firstElementChild!.hasAttribute("data-shared-border-cell")).toBe(true);
    expect(content.firstElementChild!.className).toMatch(/p-double/u);
  });

  it("draws a list of rows as one group whose rows have no border of their own", () => {
    render(
      <RowList as="ol" aria-label="rows">
        <li {...CELL}>a</li>
        <li {...CELL}>b</li>
      </RowList>,
    );
    const list = screen.getByRole("list", { name: "rows" });
    expect(list.hasAttribute("data-shared-border-group")).toBe(true);
    expect(list.hasAttribute("data-shared-border-cell")).toBe(true);
    expect(list.className).not.toMatch(BORDER);
    for (const row of within(list).getAllByRole("listitem")) expect(row.className).not.toMatch(BORDERISH);
  });

  it("frames a list that lies in padding with one outline shared with its neighbours", () => {
    render(
      <RowList framed aria-label="framed">
        <li {...CELL}>a</li>
        <li {...CELL}>b</li>
      </RowList>,
    );
    const list = screen.getByRole("list", { name: "framed" });
    expect(list.hasAttribute("data-shared-border-frame")).toBe(true);
    expect(list.getAttribute("data-orientation")).toBe("vertical");
    expect(list.getAttribute("data-shared-border-edges")).toBe("top right bottom left");
    expect(list.hasAttribute("data-shared-border-group")).toBe(false);
    expect(list.className).not.toMatch(BORDER);
  });

  it("makes a segmented choice a frame of cells, or a flush row of cells without an outline", () => {
    const options = [
      { value: "a", label: "A" },
      { value: "b", label: "B" },
    ];
    const { rerender } = render(<Segments label="framed" options={options} value="a" onChange={() => undefined} />);
    const framed = screen.getByRole("group", { name: "framed" });
    expect(framed.hasAttribute("data-shared-border-frame")).toBe(true);
    expect(framed.className).not.toMatch(BORDER);
    expect(cells(framed)).toHaveLength(2);
    rerender(<Segments flush label="flush" options={options} value="a" onChange={() => undefined} />);
    const flush = screen.getByRole("group", { name: "flush" });
    expect(flush.hasAttribute("data-shared-border-group")).toBe(true);
    expect(flush.hasAttribute("data-shared-border-cell")).toBe(true);
    expect(flush.className).not.toMatch(BORDER);
    expect(cells(flush)).toHaveLength(2);
    expect(flush.className.split(" ")).toContain("flex-wrap");
    expect(flush.hasAttribute("data-shared-border-wrap")).toBe(true);
    expect(framed.hasAttribute("data-shared-border-wrap")).toBe(true);
  });

  it("gives a row button and a cell note no outline of their own", () => {
    render(
      <>
        <BodyButton row onClick={() => undefined}>
          Show now
        </BodyButton>
        <ProblemNote cell problem={{ message: "Broken" }} text={text} />
      </>,
    );
    const button = screen.getByRole("button", { name: "Show now" });
    expect(button.hasAttribute("data-shared-border-cell")).toBe(true);
    expect(button.className).not.toMatch(BORDER);
    expect(button.className).toMatch(/\bw-full\b/u);
    const note = screen.getByRole("alert");
    expect(note.hasAttribute("data-shared-border-cell")).toBe(true);
    expect(note.className).not.toMatch(BORDER);
  });
});

const label = (en: string, de: string) => ({ en, de });
const PLACES: SheetClassificationTask = {
  kind: "classification",
  id: "places",
  title: label("Places", "Orte"),
  prompt: label("Assign each place.", "Ordne jeden Ort zu."),
  categories: [
    { id: "dry", label: label("Dry", "Trocken") },
    { id: "wet", label: label("Wet", "Nass") },
  ],
  items: [
    { id: "desert", label: label("Desert", "Wüste") },
    { id: "fjord", label: label("Fjord", "Fjord") },
  ],
};

describe("🗂️ the classification as shared cells", () => {
  it("makes the pool and the categories the two cells of one group, and an item a row whose select is its trailing cell without a border of its own", () => {
    render(<ClassificationTaskView task={PLACES} answer={{ kind: "classification", assignments: { fjord: "wet" } }} onAnswer={() => undefined} text={text} locale="en" />);
    const root = document.querySelector(".course-classify")!;
    expect(root.hasAttribute("data-shared-border-group")).toBe(true);
    expect(cells(root).map((cell) => cell.className.includes("course-pool") || cell.className.includes("course-categories"))).toEqual([true, true]);
    for (const row of document.querySelectorAll(".course-chip")) {
      expect(row.hasAttribute("data-shared-border-cell")).toBe(true);
      expect(row.className).not.toMatch(BORDERISH);
      const select = row.querySelector("select")!;
      expect(select.className).toMatch(/\bcourse-cell\b/u);
      expect(select.className).not.toMatch(BORDER);
    }
    expect(document.querySelector(".course-pool")?.className).not.toMatch(/dashed/u);
  });

  it("makes the categories one column of group cells without a frame, a gap or an inset box", () => {
    render(<ClassificationTaskView task={PLACES} answer={{ kind: "classification", assignments: { fjord: "wet" } }} onAnswer={() => undefined} text={text} locale="en" />);
    const categories = document.querySelector(".course-categories")!;
    expect(categories.hasAttribute("data-shared-border-group")).toBe(true);
    expect(categories.className).not.toMatch(/\bgap-|\bp-double\b/u);
    const bins = document.querySelector(".course-bins")!;
    expect(bins.hasAttribute("data-shared-border-group")).toBe(true);
    expect([...bins.children].map((bin) => bin.hasAttribute("data-shared-border-cell") && bin.hasAttribute("data-shared-border-group"))).toEqual([true, true]);
    for (const bin of bins.children) expect(bin.className).not.toMatch(BORDERISH);
    expect(stylesheet).not.toMatch(/\.course-bins \{[^}]*gap/u);
  });
});

interface Selector {
  readonly type: string;
  readonly name?: string;
}
interface Style {
  readonly value: { readonly selectors?: readonly (readonly Selector[])[]; readonly declarations?: { readonly declarations: readonly { readonly property: string }[] } };
}

describe("🎨️ the stylesheet and the shared borders", () => {
  const declared = new Map<string, Set<string>>();
  transform({
    filename: "🎨️.css",
    code: new TextEncoder().encode(stylesheet),
    visitor: {
      Rule: {
        style(rule) {
          const { value } = rule as unknown as Style;
          for (const selector of value.selectors ?? []) {
            const key = selector.map((part) => (part.type === "class" ? `.${part.name}` : part.type === "combinator" ? " " : "?")).join("");
            const set = declared.get(key) ?? new Set<string>();
            for (const declaration of value.declarations?.declarations ?? []) set.add(declaration.property);
            declared.set(key, set);
          }
        },
      },
    },
  });

  it("gives a trailing control of a row only the divider to its left", () => {
    expect(/\.course-cell \{[^}]*align-self: stretch;[^}]*border: 0;[^}]*border-radius: 0;[^}]*border-inline-start: var\(--stroke-hairline\) solid/u.test(stylesheet)).toBe(true);
  });

  it("draws no border on a row of the matching slots or the steps", () => {
    for (const row of [".course-slot", ".course-step", ".course-steps"]) {
      const properties = [...(declared.get(row) ?? [])];
      expect(properties.filter((property) => property.startsWith("border")), row).toEqual([]);
    }
  });

  it("moves the dividers of side-by-side cells from rows to columns through the design system's flow variable, never a hand-rolled divider", () => {
    for (const owner of [".course-steps", ".course-classify", ".course-result\\[data-crowd\\]"]) expect(stylesheet).toMatch(new RegExp(`${owner} > \\* \\{\\s*--shared-border-flow: 1;`, "u"));
    expect(stylesheet).toMatch(/\.course-match > \.course-cards-cell \{\s*--shared-border-flow: 1;/u);
    expect(stylesheet).not.toMatch(/\[data-shared-border-cell\][^{]*\{[^}]*border-inline-(?:end|start): var\(--stroke-hairline\)/u);
    expect(stylesheet).not.toMatch(/\.course-guess[^{]*\{[^}]*dashed/u);
    expect(stylesheet).toMatch(/\.course-guess-entry > input\.course-input:read-only \{\s*background: transparent;/u);
  });

  it("draws no frame on a notice, a text field or a narrow guess, and lets a short page's last card end on the footer", () => {
    expect([...(declared.get(".course-alert") ?? [])].filter((property) => property.startsWith("border"))).toEqual([]);
    expect(stylesheet).toMatch(/\.course-field > input\.course-input \{[^}]*border: 0;/u);
    expect(/\.course-guess \{[^}]*border-inline-start/u.test(stylesheet.slice(0, stylesheet.indexOf("@container")))).toBe(false);
    expect(stylesheet).toMatch(/"grip guess guess up down"/u);
    expect(stylesheet).toMatch(/\[data-card-stack\] > \[data-overview-card\]:last-child \{\s*flex: 1 1 auto;/u);
  });

  it("uses the hairline for every divider of a table, never a heavier line", () => {
    expect(stylesheet).not.toMatch(/border-block-(?:end|start): var\(--stroke-default\)/u);
  });
});

describe("🛟️ the load fallback and the quiz actions", () => {
  it("joins the two fallback buttons in one outline with one divider and no gap or frame of their own", () => {
    render(<LoadFallback failed onRetry={() => undefined} onReload={() => undefined} />);
    for (const button of screen.getAllByRole("button")) expect(button.className).not.toMatch(BORDERISH);
    for (const group of document.querySelectorAll("section[lang] > span")) {
      expect(group.className).not.toMatch(/\bgap-/u);
      expect(group.className).toMatch(BORDER);
      expect(group.className).toContain("[&>button:not(:last-child)]:border-e");
    }
  });

  it("draws the divider between the footer actions of a quiz through the shared-border cell, never a border utility", () => {
    const source = readFileSync(resolve(import.meta.dirname, "../../🎯️targets/⚛️react/🔨️modules/📖️quiz-page/🟦️.tsx"), "utf8");
    expect(source).not.toMatch(/CardAction[^>]*border-e/u);
    expect(source.match(/<CardAction[^>]*\{\.\.\.CELL\}/gu)?.length).toBe(3);
  });

  it("stacks the result cards with no gap so they interlock", () => {
    const source = readFileSync(resolve(import.meta.dirname, "../../🎯️targets/⚛️react/🔨️modules/🏁️results/🟦️.tsx"), "utf8");
    expect(source).toMatch(/className="course-results flex flex-col"/u);
  });

  it("keeps the sr-only clock announcer inside the progress cell, never in a padded cell of its own", () => {
    const source = readFileSync(resolve(import.meta.dirname, "../../🎯️targets/⚛️react/🔨️modules/▶️run/🟦️.tsx"), "utf8");
    expect(source).not.toMatch(/<CardSection>\s*<ClockWatch/u);
    expect(source).toMatch(/<ClockWatch[^>]*\/> : null\}\s*<\/CardSection>/u);
  });
});
