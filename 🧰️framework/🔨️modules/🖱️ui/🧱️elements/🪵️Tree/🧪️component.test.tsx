// #region 🔌️Adapters
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { TreeSection } from "./🟦️component.tsx";
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
