import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { applyRovingTabindexV1, handleTreeKeyDownV1, treeNavigateV1 } from "../⌨️roving/🟦️.ts";
import rovingVectors from "./🧫️fixtures/⌨️roving/🔣️.json";

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

type RovingVectorItem = { readonly id: string; readonly expanded?: boolean; readonly items?: readonly RovingVectorItem[] };
const REACT_VECTOR_KEYS: ReadonlySet<string> = new Set(["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight", "Home", "End", "Enter"]);

describe("treeNavigateV1 against the neutral vectors the Rust twin also answers", () => {
  const visibleRows = (items: readonly RovingVectorItem[], expanded: ReadonlySet<string>, level = 1): { id: string; level: number; expanded: boolean | undefined }[] =>
    items.flatMap((item) => [{ id: item.id, level, expanded: item.items === undefined ? undefined : expanded.has(item.id) }, ...(item.items !== undefined && expanded.has(item.id) ? visibleRows(item.items, expanded, level + 1) : [])]);
  const initiallyExpanded = (items: readonly RovingVectorItem[]): string[] => items.flatMap((item) => [...(item.expanded === true ? [item.id] : []), ...initiallyExpanded(item.items ?? [])]);

  it("moves focus, folds and unfolds exactly as the vectors expect for every plain key React answers", () => {
    const tree = rovingVectors.tree as readonly RovingVectorItem[];
    for (const vector of rovingVectors.cases) {
      const steps = vector.steps as readonly { key?: string; modifiers?: readonly string[]; expect: { focus?: string; expanded?: readonly string[] } }[];
      if (vector.multiple || steps.some((step) => step.key === undefined || !REACT_VECTOR_KEYS.has(step.key) || step.modifiers !== undefined)) continue;
      const expanded = new Set(initiallyExpanded(tree));
      let focus = vector.focus as string;
      steps.forEach((step, index) => {
        const rows = visibleRows(tree, expanded);
        const intent = treeNavigateV1(step.key!, rows, rows.findIndex((row) => row.id === focus));
        if (intent?.kind === "focus") focus = rows[intent.index]!.id;
        if (intent?.kind === "click") {
          const id = rows[intent.index]!.id;
          focus = id;
          if (rows[intent.index]!.expanded !== undefined && !expanded.delete(id)) expanded.add(id);
        }
        const at = `${vector.name} / step ${index}`;
        if (step.expect.focus !== undefined) expect(focus, `${at}: focus`).toBe(step.expect.focus);
        if (step.expect.expanded !== undefined) expect([...expanded].sort(), `${at}: expanded`).toEqual([...step.expect.expanded]);
      });
    }
  });
});
