// #region 🔌️Adapters
import { fireEvent, render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { TreeCheckbox, TreeItem, TreeSection } from "../../🟦️.tsx";
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
