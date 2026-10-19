import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import chromeSchema from "../../🧬️schema/🖼️chrome-emphasis/🔣️.json";
import chrome from "../../🧫️fixtures/🖼️chrome-emphasis/🔣️.json";
import treeSchema from "../../🧬️schema/🌳️tree-emphasis/🔣️.json";
import tree from "../../🧫️fixtures/🌳️tree-emphasis/🔣️.json";
import contourSchema from "../../🧬️schema/🖼️window-contour/🔣️.json";
import contour from "../../🧫️fixtures/🖼️window-contour/🔣️.json";
import dragSchema from "../../🧬️schema/🪟️drag-preview/🔣️.json";
import drag from "../../🧫️fixtures/🪟️drag-preview/🔣️.json";

test("language-agnostic terminal emphasis fixtures satisfy their schemas", () => {
  const oracle = new Ajv2020({ strict: false, allErrors: true });
  for (const [schema, fixture] of [[chromeSchema, chrome], [treeSchema, tree], [contourSchema, contour], [dragSchema, drag]]) {
    const validate = oracle.compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  }
});

test("destination preview fixtures cover placements, overflow and source lifecycle", () => {
  expect(new Set(drag.cases.map(fixture => fixture.name)).size).toBe(drag.cases.length);
  expect(new Set(drag.cases.map(fixture => fixture.placement))).toEqual(new Set(["top-left", "top-right", "bottom-left", "bottom-right", "left", "right", "top", "bottom", "reorder", "overflow-next"]));
  expect(drag.cases.some(fixture => fixture.sourcePeer)).toBe(true);
  expect(drag.cases.some(fixture => !fixture.sourcePeer && !fixture.sameStack)).toBe(true);
  expect(drag.cases.some(fixture => fixture.sameStack && fixture.placement === "reorder")).toBe(true);
  expect(drag.cases.some(fixture => fixture.compact)).toBe(true);
  expect(drag.cases.some(fixture => "cap" in fixture && fixture.cap)).toBe(true);
  expect(drag.handle.unicode).toHaveLength(1);
  expect(drag.handle.ascii).toHaveLength(1);
});

test("window contour fixtures distinguish body and tab color ownership", () => {
  for (const fixture of contour.cases) {
    expect(fixture.active).toBeLessThan(fixture.tabs.length);
    expect(fixture.bodyRole).toBe(fixture.emphasized ? "emphasized" : fixture.focused ? "active" : "normal");
    const [width, height] = fixture.size;
    const [x, y, bodyWidth, bodyHeight] = fixture.body;
    expect(x + bodyWidth).toBeLessThanOrEqual(width);
    expect(y + bodyHeight).toBeLessThanOrEqual(height);
    if (fixture.mode === "flat") continue;
    const shown = "rendered" in fixture ? fixture.rendered : fixture.tabs;
    for (const tab of shown) {
      expect(tab.x + tab.width).toBeLessThanOrEqual(width);
      if (fixture.mode === "full") {
        expect(tab.corner.startsWith("top") ? y : y + bodyHeight).toBe(tab.corner.startsWith("top") ? 2 : height - 2);
      }
    }
  }
});

test("only selected tab outlines receive active color", () => {
  for (const fixture of chrome.cases) {
    expect(fixture.active).toBeLessThan(chrome.labels.length);
    if (fixture.hover !== null) expect(fixture.hover).toBeLessThan(chrome.labels.length);
    expect(fixture.roles).toEqual(chrome.labels.map((_, index) => index === fixture.active ? "active" : index === fixture.hover ? "emphasized" : "normal"));
  }
});

test("tree emphasis matches independently reconstructed parent paths", () => {
  const parents: (number | undefined)[] = [];
  const stack: number[] = [];
  tree.items.forEach(([, depth], index) => {
    expect(Number(depth)).toBeLessThanOrEqual(stack.length);
    stack.length = Number(depth);
    parents[index] = stack.at(-1);
    stack.push(index);
  });
  for (const fixture of tree.cases) {
    for (const index of [fixture.selected, ...fixture.expanded, ...fixture.rows.map(row => row.item), ...(fixture.hover === null ? [] : [fixture.hover])]) expect(index).toBeLessThan(tree.items.length);
    const path = new Set<number>();
    let cursor: number | undefined = fixture.hover ?? undefined;
    while (cursor !== undefined) {
      path.add(cursor);
      cursor = parents[cursor];
    }
    for (const row of fixture.rows) {
      const ancestors: number[] = [];
      let parent = parents[row.item];
      while (parent !== undefined) {
        ancestors.unshift(parent);
        parent = parents[parent];
      }
      const label = [...ancestors, row.item].map(index => tree.items[index][0]).join(" ").toLowerCase();
      const matched = fixture.query.toLowerCase().split(/\s+/).filter(Boolean).every(token => label.includes(token));
      expect(row.label).toBe(row.item === fixture.selected ? "active" : path.has(row.item) ? "emphasized" : fixture.query.trim() && !matched ? "muted" : "foreground");
      expect(row.guides).toEqual(ancestors.map(ancestor => path.has(ancestor) && ancestor !== fixture.hover ? "emphasized" : "normal"));
    }
  }
});
