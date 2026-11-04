import { describe, expect, test } from "vitest";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import lodash from "lodash";
import Ajv from "ajv/dist/2020";
import postcss from "postcss";
import schema from "./segments.schema.json";
import cases from "./cases.json";
import { resolveBorderSegments, type BorderSegment, type BorderState, type ResolvedBorderSegment } from "./🟦️.ts";

/** 🔬 Independent brute-force interval oracle using third-party collection operations. */
function referenceSegments(input: readonly BorderSegment[]): ResolvedBorderSegment[] {
  return lodash.flatMap([true, false], horizontal => {
    const parallel = input.filter(s => (s.from.y === s.to.y) === horizontal);
    const groups = lodash.groupBy(parallel, s => horizontal ? s.from.y : s.from.x);
    return lodash.flatMap(Object.keys(groups).map(Number).sort((a, b) => a - b), coordinate => {
      const line = groups[coordinate];
      const start = (s: BorderSegment) => Math.min(horizontal ? s.from.x : s.from.y, horizontal ? s.to.x : s.to.y);
      const end = (s: BorderSegment) => Math.max(horizontal ? s.from.x : s.from.y, horizontal ? s.to.x : s.to.y);
      const cuts = lodash.sortBy(lodash.uniq(line.flatMap(s => [start(s), end(s)])));
      return cuts.slice(1).flatMap((to, index) => {
        const from = cuts[index];
        const covering = line.filter(s => start(s) <= from && end(s) >= to);
        if (!covering.length) return [];
        const winner = lodash.orderBy(covering, [s => states.indexOf(s.state), "owner", "id"], ["desc", "asc", "asc"])[0];
        return [{ id: winner.id, owner: winner.owner, state: winner.state,
          from: horizontal ? { x: from, y: coordinate } : { x: coordinate, y: from },
          to: horizontal ? { x: to, y: coordinate } : { x: coordinate, y: to },
          contributors: lodash.sortBy(covering.map(s => s.id)),
        }];
      });
    });
  });
}

const states: BorderState[] = ["normal", "focus", "active", "waiting", "loading", "introduced", "celebrated", "hover"];

describe("shared-border language-neutral cases", () => {
  for (const fixture of cases) {
    test(fixture.name, () => {
      const input = fixture.input as BorderSegment[];
      const expected = fixture.expected as ResolvedBorderSegment[];
      const snapshot = JSON.stringify(input);
      expect(new Ajv().compile(schema)(input)).toBe(true);
      expect(referenceSegments(input)).toEqual(expected);
      expect(resolveBorderSegments(input)).toEqual(expected);
      expect(resolveBorderSegments([...input].reverse())).toEqual(expected);
      expect(JSON.stringify(input)).toBe(snapshot);
    });
  }
  test("sweep matches independent lodash interval oracle across mixed fractional overlaps", () => {
    for (let seed = 0; seed < 40; seed++) {
      const input: BorderSegment[] = Array.from({ length: 24 }, (_, i) => {
        const a = ((i * 7 + seed) % 19) / 2 - 4;
        const b = a + ((i + seed) % 7 + 1) / 2;
        const fixed = i % 3;
        const horizontal = i % 2 === 0;
        return { id: `s${i}`, owner: `o${i % 4}`, state: states[(seed + i) % states.length],
          from: horizontal ? { x: a, y: fixed } : { x: fixed, y: a },
          to: horizontal ? { x: b, y: fixed } : { x: fixed, y: b } };
      });
      expect(resolveBorderSegments(input)).toEqual(referenceSegments(input));
      expect(resolveBorderSegments([...input].reverse())).toEqual(referenceSegments(input));
    }
  });
  test("canonical CSS uses real seams, explicit attachments and order-independent hover", () => {
    const css = postcss.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🎨️styling/🖌️ui/🎨️.css"), "utf8"));
    expect(css.nodes[1].type).toBe("atrule");
    expect((css.nodes[1] as import("postcss").AtRule).params).toBe("shared-border-ownership, shared-border-priority");
    const rules: import("postcss").Rule[] = [];
    css.walkRules(rule => { rules.push(rule); });
    const hover = rules.find(rule => rule.selector.includes(":has(+ :is(.shared-border-cell, [data-shared-border-cell]):hover)"));
    expect(hover?.nodes.some(n => n.type === "decl" && n.prop === "border-color" && n.important)).toBe(true);
    expect((hover?.parent as import("postcss").AtRule).params).toBe("shared-border-priority");
    expect(rules.some(rule => rule.selector.includes("shared-border-cell]):last-child"))).toBe(false);
    expect(rules.some(rule => rule.selector.includes(":not(:has(~ :is(.shared-border-cell, [data-shared-border-cell])))"))).toBe(true);
    expect(rules.some(rule => rule.selector.includes(":has(+ :is(.shared-border-cell, [data-shared-border-cell]):hover)") && (rule.parent as import("postcss").AtRule).params === "(forced-colors: active)" && rule.nodes.some(n => n.type === "decl" && n.prop === "border-color" && n.value === "Highlight" && n.important))).toBe(true);
    expect(rules.some(rule => rule.selector.includes('[data-window-silhouette-chip] > *'))).toBe(false);
    expect(rules.some(rule => rule.selector.includes(':is(.border-loading, .border-waiting, [data-celebrated="true"])::after') && rule.nodes.some(n => n.type === "decl" && n.prop === "content" && n.value === "none" && n.important))).toBe(true);
    expect(rules.some(rule => rule.selector.includes('[data-shared-border-attached~="left"][data-introduced="true"]') && rule.nodes.some(n => n.type === "decl" && n.prop === "box-shadow" && n.value === "none" && n.important))).toBe(true);
    for (const side of ["top", "right", "bottom", "left"]) {
      expect(rules.some(rule => rule.selector === `[data-shared-border-attached~="${side}"]` && rule.nodes.some(n => n.type === "decl" && n.prop === `border-${side}-color` && n.value === "transparent"))).toBe(true);
    }
    for (const rule of rules.filter(rule => rule.selector.includes("shared-border-cell"))) {
      expect(rule.nodes.some(n => n.type === "decl" && n.prop === "box-shadow")).toBe(false);
    }
  });
  test("hover wins every effect and either sibling position", () => {
    for (const state of states) {
      const first: BorderSegment = { id: "a", owner: "a", from: { x: 0, y: 0 }, to: { x: 4, y: 0 }, state };
      const hovered: BorderSegment = { ...first, id: "z", owner: "z", state: "hover" };
      if (state === "hover") continue;
      expect(resolveBorderSegments([first, hovered])[0].owner).toBe("z");
      expect(resolveBorderSegments([hovered, first])[0].owner).toBe("z");
    }
  });
  test("perpendicular crossing is not a coincident edge", () => {
    const input: BorderSegment[] = [
      { id: "h", owner: "h", from: { x: 0, y: 2 }, to: { x: 4, y: 2 }, state: "normal" },
      { id: "v", owner: "v", from: { x: 2, y: 0 }, to: { x: 2, y: 4 }, state: "hover" },
    ];
    expect(resolveBorderSegments(input)).toHaveLength(2);
  });
  test("rejects invalid segments and ignores points", () => {
    const segment: BorderSegment = { id: "a", owner: "a", from: { x: 0, y: 0 }, to: { x: 0, y: 0 }, state: "normal" };
    expect(resolveBorderSegments([segment])).toEqual([]);
    expect(() => resolveBorderSegments([segment, segment])).toThrow();
    expect(() => resolveBorderSegments([{ ...segment, to: { x: 1, y: 1 } }])).toThrow();
    expect(() => resolveBorderSegments([{ ...segment, to: { x: Infinity, y: 0 } }])).toThrow();
    expect(() => resolveBorderSegments([{ ...segment, state: "invalid" as BorderState }])).toThrow();
  });
});
