/** 🎯️ Shared vectors keep committed stack drags addressed by their visible destination. */
import { describe, expect, test } from "vitest";
import Ajv from "ajv";
import * as React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import fixture from "../../🧫️fixtures/🎯️stack-drop-destination/🔣️.json";
import { applyModeDrop, modeDockOutLayout } from "../../🟦️";

type Layout = Parameters<typeof applyModeDrop>[0];
type StackProjection = { path: number[]; windows: string[] };

function stacks(layout: Layout, path: number[] = []): StackProjection[] {
  if (layout.kind === "window") return [];
  if (layout.kind === "stack") return [{ path, windows: layout.children.map((child) => child.id) }];
  return layout.children.flatMap((child, index) => stacks(child, [...path, index]));
}

function projectedMarkup(projection: StackProjection[]): string {
  return renderToStaticMarkup(React.createElement("main", null, projection.map(({ path, windows }) => React.createElement("section", { key: path.join("."), "data-path": path.join(".") }, windows.map((id) => React.createElement("span", { key: id }, id))))));
}

describe("🎯️ stack drop destination", () => {
  test("the language-neutral vectors satisfy their independent Ajv schema", () => {
  });

  test.each(fixture.cases)("$id uses the visible destination after extraction", (law) => {
    const layout = { kind: law.axis, children: law.initialStacks.map((stack) => ({ kind: "stack", activeId: stack.active, children: stack.windows.map((id) => ({ kind: "window", id })) })) } as Layout;
    const source = law.initialStacks[law.sourceIndex]!;
    const drag = { dragKind: law.dragKind, windowId: source.active, stackPath: String(law.sourceIndex), tabIndex: source.windows.indexOf(source.active), pointerId: 1, ghostLabel: source.active, x: 0, y: 0 } as Parameters<typeof applyModeDrop>[1];
    const zone = { ...law.zone, stackPath: law.zone.path.join(".") } as Parameters<typeof applyModeDrop>[2];
    const before = JSON.stringify(layout);
    const visible = modeDockOutLayout(layout, drag);
    expect(stacks(visible).flatMap((stack) => stack.windows)).not.toContain(source.active);
    const result = applyModeDrop(layout, drag, zone);
    expect(stacks(result)).toEqual(law.expectedStacks);
    expect(projectedMarkup(stacks(result))).toBe(projectedMarkup(law.expectedStacks));
    expect(JSON.stringify(layout)).toBe(before);
    if (!law.accepted) expect(result).toBe(layout);
    console.log(`[DEBUG] React stack drop ${law.id}: ${JSON.stringify(stacks(result))}`);
  });
});
