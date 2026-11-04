import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { applyRovingTabindexV1, handleTreeKeyDownV1, treeNavigateV1 } from "../⌨️roving/🟦️.ts";

afterEach(cleanup);

describe("treeNavigateV1", () => {
  const rows = [{ level: 1, expanded: true }, { level: 2, expanded: undefined }, { level: 1, expanded: false }] as const;
  it("moves with arrows, Home and End", () => {
    expect(treeNavigateV1("ArrowDown", rows, 0)).toEqual({ kind: "focus", index: 1 });
    expect(treeNavigateV1("ArrowUp", rows, 0)).toBeNull();
    expect(treeNavigateV1("Home", rows, 2)).toEqual({ kind: "focus", index: 0 });
    expect(treeNavigateV1("End", rows, 0)).toEqual({ kind: "focus", index: 2 });
  });
  it("Right unfolds a collapsed row or enters an expanded one; Left folds or climbs to the parent", () => {
    expect(treeNavigateV1("ArrowRight", rows, 2)).toEqual({ kind: "click", index: 2 });
    expect(treeNavigateV1("ArrowRight", rows, 0)).toEqual({ kind: "focus", index: 1 });
    expect(treeNavigateV1("ArrowLeft", rows, 0)).toEqual({ kind: "click", index: 0 });
    expect(treeNavigateV1("ArrowLeft", rows, 1)).toEqual({ kind: "focus", index: 0 });
    expect(treeNavigateV1("Enter", rows, 1)).toEqual({ kind: "click", index: 1 });
    expect(treeNavigateV1("x", rows, 1)).toBeNull();
  });
});

describe("roving tabindex on a real tree", () => {
  const Tree = () => (
    <div role="tree" onKeyDown={(event) => { handleTreeKeyDownV1(event.currentTarget, event.nativeEvent); }}>
      <div role="treeitem" aria-level={1} aria-expanded="true" aria-selected="false" data-id="a">A</div>
      <div role="treeitem" aria-level={2} aria-selected="true" data-id="b">B</div>
      <div role="treeitem" aria-level={1} aria-selected="false" data-id="c">C</div>
    </div>
  );

  it("exposes exactly one tab stop, the selected row, and moves it with the keyboard", () => {
    const view = render(<Tree />);
    const root = view.container.querySelector('[role="tree"]')!;
    applyRovingTabindexV1(root, null);
    const rows = screen.getAllByRole("treeitem");
    expect(rows.map((row) => row.getAttribute("tabindex"))).toEqual(["-1", "0", "-1"]);
    rows[1]!.focus();
    fireEvent.keyDown(rows[1]!, { key: "ArrowDown" });
    expect(document.activeElement).toBe(rows[2]);
    expect(rows.map((row) => row.getAttribute("tabindex"))).toEqual(["-1", "-1", "0"]);
    fireEvent.keyDown(rows[2]!, { key: "Home" });
    expect(document.activeElement).toBe(rows[0]);
  });

  it("ignores keys that land inside a row's own controls", () => {
    const view = render(<div role="tree" onKeyDown={(event) => { handleTreeKeyDownV1(event.currentTarget, event.nativeEvent); }}><div role="treeitem" aria-level={1}><input aria-label="name" /></div><div role="treeitem" aria-level={1} /></div>);
    const input = view.container.querySelector("input")!;
    input.focus();
    fireEvent.keyDown(input, { key: "ArrowDown" });
    expect(document.activeElement).toBe(input);
  });
});
