/** 🎯️ Shared vectors keep committed stack drags addressed by their visible destination. */
import { describe, expect, test } from "vitest";
import Ajv from "ajv";
import * as React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import fixture from "../🧫️fixtures/🎯️stack-drop-destination/🔣️.json";
import { type WindowLayoutNode, windowIdOf } from "@semio-tech/ui-layout";
import { applyModeDrop, modeDockOutLayout } from "../../🎯️drop/🟦️.ts";

type Layout = WindowLayoutNode;
type StackProjection = { path: number[]; windows: string[] };

function stacks(layout: Layout, path: number[] = []): StackProjection[] {
  if (layout.kind === "window") return [];
  if (layout.kind === "stack") return [{ path, windows: layout.children.map(windowIdOf) }];
  return layout.children.flatMap((child, index) => stacks(child, [...path, index]));
}

function projectedMarkup(projection: StackProjection[]): string {
  return renderToStaticMarkup(React.createElement("main", null, projection.map(({ path, windows }) => React.createElement("section", { key: path.join("."), "data-path": path.join(".") }, windows.map((id) => React.createElement("span", { key: id }, id))))));
}

describe("🎯️ stack drop destination", () => {
  test("the language-neutral vectors satisfy their independent Ajv schema", () => {
  });

  test.each(fixture.cases)("$id uses the visible destination after extraction", (law) => {
    const layout = { kind: "split", axis: law.axis === "row" ? "horizontal" : "vertical", children: law.initialStacks.map((stack) => ({ kind: "stack", activeWindowId: stack.active, children: stack.windows.map((id) => ({ kind: "window", windowKindId: id })) })) } as Layout;
    const source = law.initialStacks[law.sourceIndex]!;
    const drag = { dragKind: law.dragKind, windowId: source.active, stackPath: [law.sourceIndex], tabIndex: source.windows.indexOf(source.active), pointerId: 1, ghostLabel: source.active, x: 0, y: 0 } as Parameters<typeof applyModeDrop>[1];
    const { path, ...rest } = law.zone;
    const zone = { ...rest, stackPath: path } as Parameters<typeof applyModeDrop>[2];
    const before = JSON.stringify(layout);
    const visible = modeDockOutLayout(layout, drag);
    expect(stacks(visible).flatMap((stack) => stack.windows)).not.toContain(source.active);
    const result = applyModeDrop(layout, drag, zone);
    expect(stacks(result)).toEqual(law.expectedStacks);
    expect(projectedMarkup(stacks(result))).toBe(projectedMarkup(law.expectedStacks));
    expect(JSON.stringify(layout)).toBe(before);
    if (!law.accepted) expect(result).toBe(layout);
  });
});
