/** 📐️ Language-neutral dock axis geometry oracle. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/📐️dock-axis-geometry/🔣️.json";
import schema from "../../🧬️schema/📐️dock-axis-geometry/🔣️.json";
import { applyAxisResizeDelta } from "../../🧱️elements/🎨️Canvas/🟦️";

type Rect = { x: number; y: number; width: number; height: number };
type Node = { kind: "stack"; id: string } | { kind: "row" | "column"; children: Array<{ weight: number; node: Node }> };

function solve(node: Node, rect: Rect, separator: number, stacks: Record<string, Rect>): void {
  if (node.kind === "stack") {
    stacks[node.id] = rect;
    return;
  }
  const horizontal = node.kind === "row";
  const extent = horizontal ? rect.width : rect.height;
  const distributable = Math.max(0, extent - separator * (node.children.length - 1));
  const totalWeight = node.children.reduce((total, child) => total + child.weight, 0);
  let cursor = horizontal ? rect.x : rect.y;
  node.children.forEach((child, index) => {
    const childExtent = index + 1 === node.children.length ? (horizontal ? rect.x + rect.width : rect.y + rect.height) - cursor : (distributable * child.weight) / totalWeight;
    solve(
      child.node,
      horizontal ? { x: cursor, y: rect.y, width: childExtent, height: rect.height } : { x: rect.x, y: cursor, width: rect.width, height: childExtent },
      separator,
      stacks,
    );
    cursor += childExtent + separator;
  });
}

function solveFixture(tokenPixels: number): Record<string, Rect> {
  const viewport = fixture.viewport;
  const stacks: Record<string, Rect> = {};
  solve(fixture.layout as Node, { x: tokenPixels, y: tokenPixels, width: viewport.width - tokenPixels * 2, height: viewport.height - tokenPixels * 2 }, tokenPixels, stacks);
  return stacks;
}

describe("📐️ dock axis geometry", () => {
  test("the neutral vectors satisfy the Ajv schema oracle", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("each axis reserves exactly one themed separator before weighted distribution", () => {
    const actual = solveFixture(fixture.oracleSample.tokenPixels);
    for (const [id, expected] of Object.entries(fixture.oracleSample.stacks)) {
      for (const field of ["x", "y", "width", "height"] as const) expect(actual[id]![field]).toBeCloseTo(expected[field], 6);
    }
    const left = actual.left!;
    const top = actual["right-top"]!;
    const bottom = actual["right-bottom"]!;
    expect(top.x - (left.x + left.width)).toBeCloseTo(fixture.oracleSample.tokenPixels, 6);
    expect(bottom.y - (top.y + top.height)).toBeCloseTo(fixture.oracleSample.tokenPixels, 6);
  });

  test("React moves a percentage-weighted split by the physical pointer delta", () => {
    const oracle = fixture.resizeOracle;
    const children = oracle.beforeWeights.map((size, index) => ({ kind: "stack" as const, children: [{ kind: "window" as const, id: `window-${index}` }], size }));
    const layout = { kind: "row" as const, children };
    const deltaPercent = (oracle.deltaPixels / oracle.axisExtentPixels) * 100;
    const resized = applyAxisResizeDelta(layout, "", oracle.separatorIndex, deltaPercent, oracle.minimumPercent);
    expect(resized.kind).toBe("row");
    if (resized.kind !== "row") throw new Error("resize oracle must remain a row");
    const actualWeights = resized.children.map((child) => child.size ?? Number.NaN);
    expect(actualWeights).toEqual(oracle.afterWeights);
    const beforeSeparator = (oracle.beforeWeights[0]! / oracle.beforeWeights.reduce((sum, weight) => sum + weight, 0)) * oracle.axisExtentPixels;
    const afterSeparator = (actualWeights[0]! / actualWeights.reduce((sum, weight) => sum + weight, 0)) * oracle.axisExtentPixels;
    expect(afterSeparator - beforeSeparator).toBeCloseTo(oracle.expectedSeparatorDeltaPixels, 6);
  });
});
