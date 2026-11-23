import { describe, expect, test } from "vitest";
import cases from "./shared-border.cases.json";
import { Vector2 } from "three";
import Ajv from "ajv/dist/2020.js";
import schema from "./shared-border.schema.json";
import { projectSharedBorderPoint, sharedOutlineSegments, sharedBorderJunction, clipSharedOutlineSegments, sharedBorderPaintPoints, visibleSharedOutlineSegments } from "./shared-border.ts";
import junctions from "./junctions.cases.json";
import junctionSchema from "./junctions.schema.json";
import { createWindowFrameGeometry } from "./🟦️.tsx";

describe("shared frame projection", () => {
  test("neutral junction schema", () => expect(new Ajv().validate(junctionSchema, junctions)).toBe(true));
  for (const entry of junctions.cases) test(entry.name, () => {
    const sources = entry.rectangles.flatMap(([left, top, width, height, scaleX, scaleY], index) => sharedOutlineSegments(String(index), [{ x: 0, y: 0 }, { x: width!, y: 0 }, { x: width!, y: height! }, { x: 0, y: height! }], [{ x: 1, y: 1 }, { x: width! - 1, y: 1 }, { x: width! - 1, y: height! - 1 }, { x: 1, y: height! - 1 }], "normal", { left: left!, top: top!, scaleX: scaleX!, scaleY: scaleY! }));
    const point = { x: entry.point[0]!, y: entry.point[1]! };
    const actual = sharedBorderJunction(point, sources);
    expect(actual).toEqual({ x: entry.expected[0]!, y: entry.expected[1]! });
    expect(sharedBorderJunction(point, [...sources].reverse())).toEqual(actual);
    const reference = new Vector2(...entry.expected as [number, number]);
    expect(new Vector2(actual.x, actual.y).distanceTo(reference)).toBe(0);
  });
  for (const entry of junctions.paintSpans) test(entry.name, () => {
    const point = ([x, y]: readonly number[]) => ({ x: x!, y: y! });
    const actual = sharedBorderPaintPoints(point(entry.from), point(entry.to), entry.horizontal ? "y" : "x", entry.centerline);
    expect(actual.map(({ x, y }) => [x, y])).toEqual(entry.expected);
  });
  test("invisible inset cannot win at an overflow boundary", () => {
    const sources = sharedOutlineSegments("hidden", [{ x: 0, y: 0 }, { x: 100, y: 0 }, { x: 100, y: 100 }, { x: 0, y: 100 }], [{ x: 1, y: 1 }, { x: 99, y: 1 }, { x: 99, y: 99 }, { x: 1, y: 99 }], "active", { left: 100, top: 0, scaleX: 1, scaleY: 1 });
    expect(clipSharedOutlineSegments(sources, { left: 0, top: 0, right: 100, bottom: 100 })).toEqual([]);
  });
  test("canonical movement cannot let a viewport-clipped active owner steal paint", () => {
    const logical = [{ x: 0, y: 0 }, { x: 100, y: 0 }, { x: 100, y: 100 }, { x: 0, y: 100 }];
    const paint = [{ x: 1, y: 1 }, { x: 99, y: 1 }, { x: 99, y: 99 }, { x: 1, y: 99 }];
    const left = sharedOutlineSegments("left", logical, paint, "active", { left: 0, top: 0, scaleX: 1, scaleY: 1 }).map(source => ({ ...source, clip: { left: 0, top: 0, right: 99.5, bottom: 100 } }));
    const right = sharedOutlineSegments("right", logical, paint, "normal", { left: 100, top: 0, scaleX: 1, scaleY: 1 });
    const eligible = visibleSharedOutlineSegments([...left, ...right]);
    expect(eligible.some(source => source.owner === "left" && source.from.x === 100 && source.to.x === 100)).toBe(false);
    expect(eligible.some(source => source.owner === "right" && source.from.x === 100 && source.to.x === 100)).toBe(true);
  });
  test("neutral fixture conforms to its schema", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const entry of cases.cases) test(entry.name, () => {
    const actual = [entry.from, entry.to].map(point => projectSharedBorderPoint(point, entry.logical[0]!, entry.logical[1]!, entry.paint[0]!, entry.paint[1]!));
    actual.forEach((point, index) => {
      expect(point.x).toBeCloseTo(entry.expected[index]!.x);
      expect(point.y).toBeCloseTo(entry.expected[index]!.y);
      const input = [entry.from, entry.to][index]!;
      const start = entry.logical[0]!, end = entry.logical[1]!;
      const ratio = start.x === end.x ? (input.y - start.y) / (end.y - start.y) : (input.x - start.x) / (end.x - start.x);
      const reference = new Vector2(entry.paint[0]!.x, entry.paint[0]!.y).lerp(new Vector2(entry.paint[1]!.x, entry.paint[1]!.y), ratio);
      expect(point.x).toBeCloseTo(reference.x);
      expect(point.y).toBeCloseTo(reference.y);
    });
  });
  test("registers the four frame edges and only the inner edges of each chip", () => {
    const geometry = createWindowFrameGeometry({ width: 100, height: 100, top: { depth: 20, chips: [{ left: 0, right: 30 }, { left: 50, right: 80 }] }, bottom: { depth: 10, chips: [{ left: 70, right: 100 }] } });
    const ring = [{ x: 0, y: 0 }, { x: 100, y: 0 }, { x: 100, y: 100 }, { x: 0, y: 100 }];
    const paint = [{ x: 1, y: 1 }, { x: 99, y: 1 }, { x: 99, y: 99 }, { x: 1, y: 99 }];
    const segments = sharedOutlineSegments("pane", ring, paint, "active", { left: 50, top: 60, scaleX: 2, scaleY: 2 }, undefined, "pane", geometry.chipRegions);
    const frame = segments.slice(0, 4), inner = segments.slice(4);
    expect(frame.map(segment => [segment.from, segment.to])).toEqual([[{ x: 50, y: 60 }, { x: 250, y: 60 }], [{ x: 250, y: 60 }, { x: 250, y: 260 }], [{ x: 250, y: 260 }, { x: 50, y: 260 }], [{ x: 50, y: 260 }, { x: 50, y: 60 }]]);
    expect(inner.map(segment => [segment.from.x, segment.from.y, segment.to.x, segment.to.y])).toEqual([
      [110, 60, 110, 100], [110, 100, 50, 100],
      [210, 60, 210, 100], [210, 100, 150, 100], [150, 100, 150, 60],
      [190, 240, 250, 240], [190, 260, 190, 240],
    ]);
    expect(inner[0]!.paintFrom).toEqual({ x: 29, y: 1 });
    expect(inner[1]!.paintTo).toEqual({ x: 1, y: 19 });
    expect(inner.every(segment => segment.from.x === segment.to.x || segment.from.y === segment.to.y)).toBe(true);
    expect(new Set(segments.map(segment => segment.id)).size).toBe(segments.length);
  });
});
