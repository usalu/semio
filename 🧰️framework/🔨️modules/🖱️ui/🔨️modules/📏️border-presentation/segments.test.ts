import { describe, expect, test } from "vitest";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import lodash from "lodash";
import Ajv from "ajv/dist/2020";
import postcss from "postcss";
import schema from "./segments.schema.json";
import cases from "./cases.json";
import partitionCases from "../../../🌱️value/📋️list/partition.cases.json";
import partitionSchema from "../../../🌱️value/📋️list/partition.schema.json";
import clipCases from "../../🎯️targets/🧊️wgpu/🖍️draw/🏷️types/📏️shared-border-clip/cases.json";
import appendCases from "../../🎯️targets/🧊️wgpu/🖍️draw/🏷️types/📥️retained-append/cases.json";
import appendSchema from "../../🎯️targets/🧊️wgpu/🖍️draw/🏷️types/📥️retained-append/cases.schema.json";
import clipSchema from "../../🎯️targets/🧊️wgpu/🖍️draw/🏷️types/📏️shared-border-clip/cases.schema.json";
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

const states: BorderState[] = ["normal", "active", "invalid", "waiting", "loading", "introduced", "celebrated", "focus", "hover"];

describe("shared-border language-neutral cases", () => {
  test("whole-frame preparation vectors match independent lodash edge ownership", () => {
    type Quad = [number, number, number, number];
    const folder = resolve(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/🧊️wgpu/🎟️prepared/🧪️tests/📏️shared-border");
    const fixture = JSON.parse(readFileSync(resolve(folder, "cases.json"), "utf8")) as { frames: { rect: Quad; color: Quad; state: BorderState }[]; visibleEdges: number; sharedPaint: Quad; sharedColor: Quad; suppressedPaint: Quad };
    expect(new Ajv().compile(JSON.parse(readFileSync(resolve(folder, "schema.json"), "utf8")))(fixture)).toBe(true);
    const edges = fixture.frames.flatMap((frame, owner) => {
      const [x, y, w, h] = frame.rect;
      return [
        { key: ["h", y, x, x + w], rect: [x, y, w, 1] },
        { key: ["v", x + w, y, y + h], rect: [x + w - 1, y, 1, h] },
        { key: ["h", y + h, x, x + w], rect: [x, y + h - 1, w, 1] },
        { key: ["v", x, y, y + h], rect: [x, y, 1, h] },
      ].map(edge => ({ ...edge, owner, state: frame.state, color: frame.color }));
    });
    const owned = Object.values(lodash.groupBy(edges, edge => JSON.stringify(edge.key))).map(group => lodash.orderBy(group, [edge => states.indexOf(edge.state), "owner"], ["desc", "asc"])[0]!);
    expect(owned).toHaveLength(fixture.visibleEdges);
    expect(owned.filter(edge => lodash.isEqual(edge.rect, fixture.sharedPaint)).map(edge => edge.color)).toEqual([fixture.sharedColor]);
    expect(owned.some(edge => lodash.isEqual(edge.rect, fixture.suppressedPaint))).toBe(false);
  });
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
  test("canonical CSS leaves every state line to the compositor overlay and maps its strokes in forced colours", () => {
    const css = postcss.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🎨️styling/🖌️ui/🎨️.css"), "utf8"));
    expect(css.nodes[1].type).toBe("atrule");
    expect((css.nodes[1] as import("postcss").AtRule).params).toBe("shared-border-ownership, shared-border-priority");
    const rules: import("postcss").Rule[] = [];
    css.walkRules(rule => { rules.push(rule); });
    const participant = /shared-border-cell|shared-border-cells|\[data-shared-border-frame\]|\.shared-border-frame|data-window-silhouette-chip/;
    for (const rule of rules.filter(rule => participant.test(rule.selector) && /:hover|:focus|:has\(\+|\[data-active|\[data-state|aria-(selected|pressed)|border-(loading|waiting)|data-(introduced|celebrated)/.test(rule.selector))) {
      for (const node of rule.nodes) if (node.type === "decl") {
        expect(/^(border(-[a-z-]+)?-color|outline-color|box-shadow)$/.test(node.prop) && !/^(none|transparent)$/.test(node.value) && !(node.prop === "box-shadow" && node.value === "none"), `${rule.selector} { ${node.prop}: ${node.value} }`).toBe(false);
      }
    }
    expect(rules.some(rule => rule.selector.includes(":has(+ :is(.shared-border-cell"))).toBe(false);
    expect(rules.some(rule => rule.selector.includes("[data-overview-card]:is(button):focus-visible"))).toBe(false);
    expect(rules.some(rule => rule.selector.startsWith("[data-window-silhouette]:hover"))).toBe(false);
    const hover = rules.find(rule => rule.selector === '[data-shared-border-segment][data-shared-border-state="hover"]');
    expect(hover?.nodes.some(n => n.type === "decl" && n.prop === "stroke" && !n.important)).toBe(true);
    const forced = rules.filter(rule => (rule.parent as import("postcss").AtRule).params === "(forced-colors: active)" && rule.selector.includes("[data-shared-border-segment]"));
    expect(forced.some(rule => rule.nodes.some(n => n.type === "decl" && n.prop === "stroke" && n.value === "CanvasText"))).toBe(true);
    expect(forced.some(rule => rule.selector.includes("[data-shared-border-disabled]") && rule.nodes.some(n => n.type === "decl" && n.prop === "stroke" && n.value === "GrayText"))).toBe(true);
    expect(forced.some(rule => ["hover", "focus", "active"].every(state => rule.selector.includes(`[data-shared-border-state="${state}"]`)) && rule.nodes.some(n => n.type === "decl" && n.prop === "stroke" && n.value === "Highlight"))).toBe(true);
    expect(rules.some(rule => rule.selector.includes('[data-window-silhouette-chip] > *'))).toBe(false);
    expect(rules.some(rule => /::after/.test(rule.selector) && /(border-loading|border-waiting|data-celebrated)/.test(rule.selector))).toBe(false);
    expect(rules.some(rule => /data-introduced/.test(rule.selector) && rule.nodes.some(n => n.type === "decl" && /box-shadow|animation/.test(n.prop)))).toBe(false);
    expect(rules.some(rule => rule.selector === ".semio-scope[data-contrast=\"more\"] :focus-visible")).toBe(false);
    expect(rules.some(rule => rule.selector.includes('[data-slot="window-measures-tree"]') && /select-trigger"\]\[data-state="open"\]|toggle-group-item"\]\[data-state="on"\]/.test(rule.selector))).toBe(false);
    expect(css.toString()).not.toMatch(/@utility border-(loading|waiting)\b/);
    for (const side of ["top", "right", "bottom", "left"]) {
      expect(rules.some(rule => rule.selector === `[data-shared-border-attached~="${side}"]` && rule.nodes.some(n => n.type === "decl" && n.prop === `border-${side}-color` && n.value === "transparent"))).toBe(true);
    }
    for (const rule of rules.filter(rule => rule.selector.includes("shared-border-cell"))) {
      expect(rule.nodes.some(n => n.type === "decl" && (n.prop === "box-shadow" && n.value !== "none" || /^border(-(inline|block|top|right|bottom|left)(-(start|end))?)?$/.test(n.prop) && /solid/.test(n.value)))).toBe(false);
    }
  });
  test("paged partition fixtures match lodash binary search", () => {
    expect(new Ajv().validate(partitionSchema, partitionCases)).toBe(true);
    for (const fixture of partitionCases) expect(lodash.sortedIndex(fixture.values, fixture.limit)).toBe(fixture.expected);
  });
  test("retained append clip fixture matches lodash rectangle intersection", () => {
    expect(new Ajv().validate(appendSchema, appendCases)).toBe(true);
    const [ax, ay, aw, ah] = appendCases.parent, [bx, by, bw, bh] = appendCases.child;
    const x = lodash.max([ax, bx])!, y = lodash.max([ay, by])!;
    const right = lodash.min([ax + aw, bx + bw])!, bottom = lodash.min([ay + ah, by + bh])!;
    expect([x, y, Math.max(0, right - x), Math.max(0, bottom - y)]).toEqual(appendCases.intersection);
  });
  test("clipped edge fixture matches independent lodash interval union", () => {
    expect(new Ajv().validate(clipSchema, clipCases)).toBe(true);
    const [left, , width] = clipCases.scissor;
    const intervals = lodash.sortBy(clipCases.pieces.map(([x, , w]) => [Math.max(x, left), Math.min(x + w, left + width)]), interval => interval[0]);
    const result: number[][] = [];
    for (const [lo, hi] of intervals) {
      if (lo >= hi) continue;
      const previous = result.at(-1);
      if (previous && lo <= previous[1]) previous[1] = Math.max(previous[1], hi);
      else result.push([lo, hi]);
    }
    expect(result).toEqual(clipCases.expected);
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
    expect(() => resolveBorderSegments([{ ...segment, state: "bogus" as BorderState }])).toThrow();
  });
});
