import { describe, expect, test } from "vitest";
import cases from "./shared-border.cases.json";
import { Vector2 } from "three";
import Ajv from "ajv/dist/2020.js";
import schema from "./shared-border.schema.json";
import { projectSharedBorderPoint, sharedOutlineSegments, sharedBorderJunction, clipSharedOutlineSegments, visibleSharedOutlineSegments } from "./shared-border.ts";
import junctions from "./junctions.cases.json";
import junctionSchema from "./junctions.schema.json";
import { createWindowSilhouetteGeometry, windowSilhouetteOutline } from "./🟦️.tsx";

describe("shared silhouette projection", () => {
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
  test("retains the measured visible fusion through geometry normalization", () => {
    const contextMenuFusion = { primary: { left: 0, top: 20, right: 80, bottom: 100 }, wings: [{ left: 80, top: 40, right: 140, bottom: 90 }] };
    const geometry = createWindowSilhouetteGeometry({ width: 140, height: 100, top: { depth: 20, chips: [{ left: 0, right: 40 }] }, bottom: { depth: 0, chips: [] }, contextMenuFusion });
    expect(geometry.metrics.contextMenuFusion).toEqual(contextMenuFusion);
  });
  test("keeps the exact stepped chip outline rather than a rectangle", () => {
    const geometry = createWindowSilhouetteGeometry({ width: 100, height: 100, top: { depth: 20, chips: [{ left: 0, right: 30 }, { left: 80, right: 100 }] }, bottom: { depth: 0, chips: [] } });
    const logical = windowSilhouetteOutline(geometry.metrics, 0);
    const segments = sharedOutlineSegments("pane", logical, windowSilhouetteOutline(geometry.metrics), "active", { left: 50, top: 60, scaleX: 2, scaleY: 2 });
    expect(segments.length).toBe(logical.length);
    expect(segments.some(segment => segment.from.y === 100 && segment.to.y === 100)).toBe(true);
    expect(segments.every(segment => segment.from.x === segment.to.x || segment.from.y === segment.to.y)).toBe(true);
    expect(geometry.borderPath).toBe("M1,1 H30 V21 H80 V1 H99 V99 H1 Z");
  });
});
