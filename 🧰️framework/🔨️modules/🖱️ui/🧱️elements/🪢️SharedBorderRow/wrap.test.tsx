import * as React from "react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, render, waitFor } from "@testing-library/react";
import { SharedBorderRow, SharedBorderRowCell, sharedBorderWrapPlacements } from "./🟦️.tsx";
import { ToggleGroup } from "../🎛️ToggleGroup/🟦️.tsx";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

const cases = JSON.parse(readFileSync(resolve(import.meta.dirname, "wrap.cases.json"), "utf8")) as { placements: { name: string; tops: number[]; tolerance: number; expected: { end: boolean; last: boolean }[] }[] };
const stylesheet = () => readFileSync(resolve(import.meta.dirname, "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");

function lineTops(tops: readonly number[]) {
  vi.spyOn(HTMLElement.prototype, "offsetTop", "get").mockImplementation(function (this: HTMLElement) {
    const index = this.parentElement ? [...this.parentElement.children].filter(child => child.hasAttribute("data-shared-border-cell") || child.hasAttribute("data-shared-border-frame")).indexOf(this) : -1;
    return tops[index] ?? 0;
  });
}

describe("wrapped shared-border rows", () => {
  for (const entry of cases.placements) it(`classifies lines: ${entry.name}`, () => {
    expect(sharedBorderWrapPlacements(entry.tops, entry.tolerance)).toEqual(entry.expected);
  });

  it("marks a wrapping row and tags its cells with the line they end and the last line", async () => {
    lineTops([0, 0, 28, 28, 56]);
    const { container } = render(<div className="semio-scope"><SharedBorderRow wrap>{["a", "b", "c", "d", "e"].map(key => <SharedBorderRowCell key={key}>{key}</SharedBorderRowCell>)}</SharedBorderRow></div>);
    const row = container.querySelector<HTMLElement>('[data-slot="shared-border-row"]')!;
    expect(row.hasAttribute("data-shared-border-wrap")).toBe(true);
    expect(row.className).toContain("flex-wrap");
    const cells = [...row.querySelectorAll<HTMLElement>(":scope > [data-shared-border-cell]")];
    await waitFor(() => expect(cells.map(cell => [cell.hasAttribute("data-wrap-end"), cell.hasAttribute("data-wrap-last")])).toEqual([[false, false], [true, false], [false, false], [true, false], [true, true]]));
  });

  it("retags the cells when one is added and removes every tag when wrapping is switched off", async () => {
    lineTops([0, 0, 28]);
    const rows = (wrap: boolean, count: number) => <div className="semio-scope"><SharedBorderRow wrap={wrap}>{Array.from({ length: count }, (_, index) => <SharedBorderRowCell key={index}>{index}</SharedBorderRowCell>)}</SharedBorderRow></div>;
    const { container, rerender } = render(rows(true, 2));
    const row = container.querySelector<HTMLElement>('[data-slot="shared-border-row"]')!;
    await waitFor(() => expect(row.querySelectorAll("[data-wrap-last]").length).toBe(2));
    rerender(rows(true, 3));
    await waitFor(() => expect(row.querySelectorAll("[data-wrap-last]").length).toBe(1));
    expect(row.querySelectorAll("[data-wrap-end]").length).toBe(2);
    rerender(rows(false, 3));
    await waitFor(() => expect(row.querySelectorAll("[data-wrap-end], [data-wrap-last]").length).toBe(0));
    expect(row.hasAttribute("data-shared-border-wrap")).toBe(false);
  });

  it("wraps the items of a ToggleGroup the same way and keeps the wrap flag out of the DOM", async () => {
    lineTops([0, 0, 28]);
    const items = ["a", "b", "c"].map(value => ({ value, icon: "check" as const, text: value }));
    const { container } = render(<div className="semio-scope"><SharedBorderRow orientation="vertical"><ToggleGroup kind="multiple" wrap items={items} /></SharedBorderRow></div>);
    const group = container.querySelector<HTMLElement>('[data-slot="toggle-group"]')!;
    expect(group.hasAttribute("data-shared-border-wrap")).toBe(true);
    expect(group.hasAttribute("wrap")).toBe(false);
    expect(group.className).toContain("flex-wrap");
    const shells = [...group.querySelectorAll<HTMLElement>('[data-slot="toggle-group-item-shell"]')];
    await waitFor(() => expect(shells.map(shell => [shell.hasAttribute("data-wrap-end"), shell.hasAttribute("data-wrap-last")])).toEqual([[false, false], [true, false], [true, true]]));
  });

  it("styles wrapped cells in the stylesheet: stretched lines, a divider below, dropped dividers at the owner's own edge with a same-size margin", () => {
    const css = stylesheet();
    expect(css).toMatch(/\[data-shared-border-wrap\] \{\s*flex-wrap: wrap;/);
    expect(css).toMatch(/\[data-shared-border-wrap\] > :is\([^{]*\) \{\s*flex: 1 0 auto;\s*border-block-end: var\(--stroke-hairline\) solid var\(--border-normal-color\);/);
    expect(css).toMatch(/\[data-wrap-end\]:is\([^{]*\) \{\s*border-inline-end-width: 0;/);
    expect(css).toMatch(/\[data-wrap-last\]:is\([^{]*\) \{\s*border-block-end-width: 0;/);
    expect(css).toMatch(/> \[data-wrap-end\] \{\s*margin-inline-end: var\(--stroke-hairline\);/);
    expect(css).toMatch(/> \[data-wrap-last\] \{\s*margin-block-end: var\(--stroke-hairline\);/);
    expect(css.indexOf("[data-shared-border-wrap] {")).toBeLessThan(css.indexOf("@layer shared-border-priority {"));
    expect(css.indexOf("[data-shared-border-wrap] {")).toBeGreaterThan(css.indexOf("Shared cells use one real divider"));
  });
});
