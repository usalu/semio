// #region 🔌️Adapters
import { fireEvent, render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { TREE_WINDOW_OVERSCAN_ROWS, TREE_WINDOW_ROWS_MAX, Tree, TreeCheckbox, TreeItem, TreeSection, treeRowHeightPx, treeWindowRequestsForViewport, type TreeDataSection, type TreeWindowContainerMeasure } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🌳️BranchDisclosure
describe("TreeSection branch disclosure", () => {
  it("preserves controlled branch state and its owned disclosure association", () => {
    const onOpenChange = vi.fn();
    const { container, rerender } = render(
      <TreeSection id="tree-test-branch" label="Branch" open={false} onOpenChange={onOpenChange}>
        <div data-testid="leaf">Leaf</div>
      </TreeSection>,
    );
    const row = container.querySelector('[data-slot="tree-section-row"]') as HTMLDivElement;
    const content = container.querySelector('[data-slot="collapsible-content"]') as HTMLDivElement;

    expect(row.getAttribute("role")).toBe("button");
    expect(row.getAttribute("aria-expanded")).toBe("false");
    expect(row.getAttribute("aria-controls")).toBe(content.id);
    expect(content.hidden).toBe(true);

    fireEvent.click(row);
    expect(onOpenChange).toHaveBeenCalledWith(true);
    expect(row.getAttribute("aria-expanded")).toBe("false");

    rerender(
      <TreeSection id="tree-test-branch" label="Branch" open onOpenChange={onOpenChange}>
        <div data-testid="leaf">Leaf</div>
      </TreeSection>,
    );
    expect(row.getAttribute("aria-expanded")).toBe("true");
    expect(content.hidden).toBe(false);

    fireEvent.keyDown(row, { key: "Enter" });
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
  });

  it("isolates real child-action and drag events from branch disclosure", () => {
    const onAction = vi.fn();
    const onDragStart = vi.fn();
    const onDragEnd = vi.fn();
    const onOpenChange = vi.fn();
    const { container, getByTestId } = render(
      <TreeSection
        id="tree-interaction-branch"
        label="Branch"
        open={false}
        onOpenChange={onOpenChange}
        actions={[{ id: "child-action", icon: <span data-testid="child-action-icon" />, onClick: onAction }]}
        draggable
        dragInitiation="surface"
        onDragStart={onDragStart}
        onDragEnd={onDragEnd}
      >
        <div>Leaf</div>
      </TreeSection>,
    );
    const row = container.querySelector('[data-slot="tree-section-row"]') as HTMLDivElement;
    const action = getByTestId("child-action-icon").closest("button") as HTMLButtonElement;

    fireEvent.click(action);
    fireEvent.dragStart(row);
    fireEvent.dragEnd(row);

    expect(onAction).toHaveBeenCalledTimes(1);
    expect(onDragStart).toHaveBeenCalledTimes(1);
    expect(onDragEnd).toHaveBeenCalledTimes(1);
    expect(onOpenChange).not.toHaveBeenCalled();
    expect(row.getAttribute("aria-expanded")).toBe("false");
  });

  it("distinguishes a delayed single click from a double-click action without spurious disclosure", () => {
    vi.useFakeTimers();
    try {
      const onDoubleClick = vi.fn();
      const onOpenChange = vi.fn();
      const { container } = render(
        <TreeSection id="tree-double-click-branch" label="Branch" open={false} onOpenChange={onOpenChange} onDoubleClick={onDoubleClick}>
          <div>Leaf</div>
        </TreeSection>,
      );
      const row = container.querySelector('[data-slot="tree-section-row"]') as HTMLDivElement;

      fireEvent.click(row, { detail: 1 });
      fireEvent.click(row, { detail: 2 });
      fireEvent.doubleClick(row, { detail: 2 });
      vi.advanceTimersByTime(400);

      expect(onDoubleClick).toHaveBeenCalledTimes(1);
      expect(onOpenChange).not.toHaveBeenCalled();
      expect(row.getAttribute("aria-expanded")).toBe("false");

      fireEvent.click(row, { detail: 1 });
      vi.advanceTimersByTime(400);
      expect(onOpenChange).toHaveBeenCalledOnce();
      expect(onOpenChange).toHaveBeenLastCalledWith(true);
      expect(row.getAttribute("aria-expanded")).toBe("false");
    } finally {
      vi.useRealTimers();
    }
  });
});
// #endregion 🌳️BranchDisclosure

// #region ☑️CheckboxActivation
describe("TreeCheckbox activation", () => {
  const renderControlled = (onCheckedChange: (checked: boolean) => void) =>
    render(<TreeCheckbox id="tree-checkbox-activation" checked={false} title="Grid visible" onCheckedChange={onCheckedChange} />);

  it("reports exactly one activation for a pointer click on the input", () => {
    const onCheckedChange = vi.fn();
    const { container } = renderControlled(onCheckedChange);
    const input = container.querySelector('[data-slot="tree-action-checkbox"]') as HTMLInputElement;

    fireEvent.click(input);

    expect(onCheckedChange).toHaveBeenCalledTimes(1);
    expect(onCheckedChange).toHaveBeenLastCalledWith(true);
  });

  it("reports exactly one activation for a pointer click on the wrapping label", () => {
    const onCheckedChange = vi.fn();
    const { container } = renderControlled(onCheckedChange);
    const wrapper = container.querySelector('[data-slot="tree-action-checkbox-wrapper"]') as HTMLLabelElement;

    fireEvent.click(wrapper);

    expect(onCheckedChange).toHaveBeenCalledTimes(1);
    expect(onCheckedChange).toHaveBeenLastCalledWith(true);
  });

  it("reports one activation for the keyboard space key and keeps the control focusable", async () => {
    const onCheckedChange = vi.fn();
    const { container } = renderControlled(onCheckedChange);
    const input = container.querySelector('[data-slot="tree-action-checkbox"]') as HTMLInputElement;

    input.focus();
    expect(document.activeElement).toBe(input);
    await userEvent.keyboard("[Space]");

    expect(onCheckedChange).toHaveBeenCalledTimes(1);
    expect(onCheckedChange).toHaveBeenLastCalledWith(true);
  });

  it("keeps a disabled checkbox inert and never reaches the enclosing row", () => {
    const onCheckedChange = vi.fn();
    const onRowClick = vi.fn();
    const { container } = render(
      <div onClick={onRowClick}>
        <TreeCheckbox id="tree-checkbox-disabled" checked disabled title="Grid snap" onCheckedChange={onCheckedChange} />
        <TreeCheckbox id="tree-checkbox-enabled" checked={false} title="Grid visible" onCheckedChange={onCheckedChange} />
      </div>,
    );
    const disabled = container.querySelector("#tree-checkbox-disabled") as HTMLInputElement;
    const enabled = container.querySelector("#tree-checkbox-enabled") as HTMLInputElement;

    fireEvent.click(disabled);
    expect(onCheckedChange).not.toHaveBeenCalled();

    fireEvent.click(enabled);
    expect(onCheckedChange).toHaveBeenCalledTimes(1);
    expect(onRowClick).not.toHaveBeenCalled();
  });

  it("names the control for assistive technology from its explicit label", () => {
    const { container } = render(<TreeCheckbox id="tree-checkbox-named" checked={false} title="Rasteranzeige" onCheckedChange={vi.fn()} />);
    const input = container.querySelector("#tree-checkbox-named") as HTMLInputElement;

    expect(input.getAttribute("aria-label")).toBe("Rasteranzeige");
    expect(input.type).toBe("checkbox");
  });
});
// #endregion ☑️CheckboxActivation

// #region 🌲️RowActivation
/** 🖱️ An EXPANDABLE row is a row: it activates from the `role="treeitem"` shell, not only from its label
 * text, and it announces its selection. Both used to hold for the leaf layout alone, which is why the
 * puzzle3d outliner's object rows — expandable, because they nest their vortices — selected nothing when
 * clicked anywhere but on the label glyphs. */
describe("TreeItem row activation", () => {
  const rowOf = (container: HTMLElement, id: string) => container.querySelector(`#${id}`) as HTMLDivElement;

  it("activates an expandable row from the row shell and from its label alike", () => {
    const onClick = vi.fn();
    const { container } = render(
      <TreeItem id="tree-row-object" label="Hexagonal Cut Concrete" isSelected onClick={onClick}>
        <TreeItem id="tree-row-vortex" label="Vortex" />
      </TreeItem>,
    );
    const row = rowOf(container, "tree-row-object");

    expect(row.getAttribute("data-tree-row-kind")).toBe("group");
    expect(row.getAttribute("aria-selected")).toBe("true");

    fireEvent.click(row, { detail: 1 });
    expect(onClick).toHaveBeenCalledTimes(1);

    fireEvent.click(row.querySelector('[data-slot="tree-label"]') as HTMLElement, { detail: 1 });
    expect(onClick).toHaveBeenCalledTimes(2);
  });

  it("keeps the fold chevron and the row actions out of row activation", () => {
    const onClick = vi.fn();
    const onAction = vi.fn();
    const { container, getByTestId } = render(
      <TreeItem id="tree-row-folded" label="Objects" onClick={onClick} actions={[{ id: "hide", icon: <span data-testid="hide-icon" />, onClick: onAction }]}>
        <TreeItem id="tree-row-child" label="Child" />
      </TreeItem>,
    );
    const row = rowOf(container, "tree-row-folded");

    fireEvent.click(row.querySelector("button.cursor-foldable") as HTMLButtonElement);
    expect(onClick).not.toHaveBeenCalled();

    fireEvent.click(getByTestId("hide-icon").closest("button") as HTMLButtonElement);
    expect(onAction).toHaveBeenCalledTimes(1);
    expect(onClick).not.toHaveBeenCalled();
  });

  it("yields a double click on an expandable row to onDoubleClick", () => {
    const onClick = vi.fn();
    const onDoubleClick = vi.fn();
    const { container } = render(
      <TreeItem id="tree-row-double" label="Objects" onClick={onClick} onDoubleClick={onDoubleClick}>
        <TreeItem id="tree-row-double-child" label="Child" />
      </TreeItem>,
    );
    const row = rowOf(container, "tree-row-double");

    fireEvent.click(row, { detail: 2 });
    fireEvent.doubleClick(row, { detail: 2 });

    expect(onClick).not.toHaveBeenCalled();
    expect(onDoubleClick).toHaveBeenCalledTimes(1);
  });

  it("announces selection on a leaf row and on an unselected row alike", () => {
    const onClick = vi.fn();
    const { container } = render(
      <>
        <TreeItem id="tree-row-leaf" label="Reference" isSelected onClick={onClick} />
        <TreeItem id="tree-row-leaf-idle" label="Other reference" onClick={onClick} />
      </>,
    );
    const selected = rowOf(container, "tree-row-leaf");
    const idle = rowOf(container, "tree-row-leaf-idle");

    expect(selected.getAttribute("data-tree-row-kind")).toBe("leaf");
    expect(selected.getAttribute("aria-selected")).toBe("true");
    expect(idle.getAttribute("aria-selected")).toBe("false");

    fireEvent.click(selected, { detail: 1 });
    expect(onClick).toHaveBeenCalledTimes(1);
  });
});
// #endregion 🌲️RowActivation

// #region 🪟️WindowedContainers
/** 🪟️ A windowed container renders only the slice the host streamed, and stands in for every row it did NOT
 * render with a spacer of exactly one row pitch each — so the scrollbar spans the whole `total` and the row a
 * viewport offset lands on is the row the guest will be asked for. See
 * `🎫️26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING/📓️design-virtualised-tree.md` §6.1. */
describe("Tree windowed containers", () => {
  const windowedSection = (overrides: Partial<TreeDataSection>): TreeDataSection => ({
    id: "tree.window.section",
    label: "Entries",
    defaultOpen: true,
    windowKey: "entries",
    ...overrides,
  });

  const spacers = (container: HTMLElement) => Array.from(container.querySelectorAll('[data-slot="tree-window-spacer"]')) as HTMLDivElement[];

  it("stands in for the rows outside the streamed slice at exactly one row pitch each", () => {
    const items = Array.from({ length: 10 }, (_, index) => ({ id: `entry-${20 + index}`, label: `Entry ${20 + index}` }));
    const { container } = render(<Tree sections={[windowedSection({ window: { total: 100, offset: 20 }, items })]} />);

    const [leading, trailing] = spacers(container);
    expect(leading.getAttribute("data-tree-window-spacer")).toBe("leading");
    expect(leading.getAttribute("data-tree-window-rows")).toBe("20");
    expect(leading.style.height).toBe(`${20 * treeRowHeightPx}px`);
    expect(trailing.getAttribute("data-tree-window-spacer")).toBe("trailing");
    expect(trailing.getAttribute("data-tree-window-rows")).toBe("70");
    expect(trailing.style.height).toBe(`${70 * treeRowHeightPx}px`);

    const content = container.querySelector('[data-slot="tree-section-content"]') as HTMLDivElement;
    expect(content.getAttribute("data-tree-window-key")).toBe("entries");
    expect(content.getAttribute("data-tree-window-total")).toBe("100");
    expect(content.getAttribute("data-tree-window-offset")).toBe("20");
    expect(content.getAttribute("data-tree-window-length")).toBe("10");
    expect(container.querySelectorAll('[data-slot="tree-item-row"]')).toHaveLength(10);
  });

  it("omits a zero-height spacer instead of rendering an empty block", () => {
    const items = Array.from({ length: 4 }, (_, index) => ({ id: `head-${index}`, label: `Head ${index}` }));
    const { container } = render(<Tree sections={[windowedSection({ window: { total: 4, offset: 0 }, items })]} />);

    expect(spacers(container)).toHaveLength(0);
  });

  it("stays expandable and wears the loading ring while an announced window has streamed no rows", () => {
    const { container } = render(<Tree sections={[windowedSection({ window: { total: 5, offset: 0 }, items: [] })]} />);

    const row = container.querySelector('[data-slot="tree-section-row"]') as HTMLDivElement;
    expect(row.getAttribute("role")).toBe("button");
    expect(row.getAttribute("aria-expanded")).toBe("true");
    expect(row.querySelector(".border-loading")).not.toBeNull();

    const [trailing] = spacers(container);
    expect(trailing.getAttribute("data-tree-window-rows")).toBe("5");
    expect(container.querySelectorAll('[data-slot="tree-item-row"]')).toHaveLength(0);
  });

  it("windows a nested group row the same way it windows a section", () => {
    const { container } = render(
      <Tree
        sections={[
          windowedSection({
            window: undefined,
            windowKey: undefined,
            items: [{ id: "group", label: "Object", defaultOpen: true, windowKey: "object", window: { total: 40, offset: 8 }, items: [{ id: "vortex-8", label: "Vortex 8" }] }],
          }),
        ]}
      />,
    );

    const content = container.querySelector('[data-slot="tree-item-content"]') as HTMLDivElement;
    expect(content.getAttribute("data-tree-window-key")).toBe("object");
    expect(content.getAttribute("data-tree-window-total")).toBe("40");
    expect(content.getAttribute("data-tree-window-offset")).toBe("8");
    expect(content.getAttribute("data-tree-window-length")).toBe("1");
    expect(spacers(container).map((spacer) => spacer.getAttribute("data-tree-window-rows"))).toEqual(["8", "31"]);
  });
});
// #endregion 🪟️WindowedContainers

// #region 📐️WindowRequests
/** 📐️ The one pure viewport rule: which rows each on-screen container must materialise next. */
describe("treeWindowRequestsForViewport", () => {
  const rowHeight = 20;
  const measure = (overrides: Partial<TreeWindowContainerMeasure>): TreeWindowContainerMeasure => ({
    key: "entries",
    total: 1000,
    offset: 0,
    length: 0,
    top: 0,
    height: 1000 * rowHeight,
    ...overrides,
  });

  it("skips a container that sits entirely above the viewport", () => {
    const above = measure({ key: "above", top: 0, height: 10 * rowHeight, total: 10 });
    const inside = measure({ key: "inside", top: 400, height: 10 * rowHeight, total: 10 });

    expect(treeWindowRequestsForViewport([above, inside], 400, 200, rowHeight, 2)).toEqual([{ key: "inside", offset: 0, rows: 10 }]);
  });

  it("asks from the first visible row less the overscan when only the container's top is cut off", () => {
    // Container starts 100px (5 rows) above the viewport top; 200px (10 rows) of it are on screen.
    const requests = treeWindowRequestsForViewport([measure({ top: -100 })], 0, 200, rowHeight, 2);

    expect(requests).toEqual([{ key: "entries", offset: 3, rows: 14 }]);
  });

  it("centres a long container's window on the visible run", () => {
    // 50 rows of a 1000-row container are on screen, starting at row 100.
    const requests = treeWindowRequestsForViewport([measure({ top: 0 })], 100 * rowHeight, 50 * rowHeight, rowHeight, TREE_WINDOW_OVERSCAN_ROWS);

    expect(requests).toEqual([{ key: "entries", offset: 100 - TREE_WINDOW_OVERSCAN_ROWS, rows: 50 + 2 * TREE_WINDOW_OVERSCAN_ROWS }]);
  });

  it("never asks past the end of a container shorter than the viewport run", () => {
    const requests = treeWindowRequestsForViewport([measure({ total: 6, height: 6 * rowHeight })], 0, 40 * rowHeight, rowHeight, TREE_WINDOW_OVERSCAN_ROWS);

    expect(requests).toEqual([{ key: "entries", offset: 0, rows: 6 }]);
  });

  it("clamps one request to the built-children ceiling", () => {
    // 300 rows of a 500-row container on screen at once — more than one built child list can carry.
    const requests = treeWindowRequestsForViewport([measure({ total: 500, height: 500 * rowHeight })], 200 * rowHeight, 300 * rowHeight, rowHeight, TREE_WINDOW_OVERSCAN_ROWS);

    expect(requests).toEqual([{ key: "entries", offset: 200 - TREE_WINDOW_OVERSCAN_ROWS, rows: TREE_WINDOW_ROWS_MAX }]);
  });

  it("keeps the window inside the total when the viewport sits at the very end", () => {
    const requests = treeWindowRequestsForViewport([measure({ total: 500, height: 500 * rowHeight })], 490 * rowHeight, 20 * rowHeight, rowHeight, TREE_WINDOW_OVERSCAN_ROWS);

    expect(requests).toEqual([{ key: "entries", offset: 500 - 26, rows: 26 }]);
  });

  it("drops containers with nothing to stream and rejects a degenerate row pitch", () => {
    expect(treeWindowRequestsForViewport([measure({ total: 0, height: 0 })], 0, 400, rowHeight, 4)).toEqual([]);
    expect(treeWindowRequestsForViewport([measure({})], 0, 400, 0, 4)).toEqual([]);
  });
});
// #endregion 📐️WindowRequests
