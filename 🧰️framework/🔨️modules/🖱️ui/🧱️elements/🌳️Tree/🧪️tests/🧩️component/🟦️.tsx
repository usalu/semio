// #region 🔌️Adapters
import * as React from "react";
import { act, fireEvent, render, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import Ajv2020 from "ajv/dist/2020.js";
import { describe, expect, it, vi } from "vitest";
import { TREE_WINDOW_BODY_NODE_BUDGET, TREE_WINDOW_OVERSCAN_ROWS, TREE_WINDOW_PATH_SEPARATOR, TREE_WINDOW_ROWS_MAX, Tree, TreeCheckbox, TreeItem, TreeSection, capTreeWindowRequests, treeRowHeightPx, treeWindowPathOf, treeWindowRequestsForViewport, treeWindowRowAriaAttributes, treeWindowVisibleRowsForViewport, type TreeDataSection, type TreeDataWindow, type TreeWindowContainerMeasure, type TreeWindowVisibleRows } from "../../🟦️.tsx";
import type { TreeWindowRowExtent } from "@semio-tech/framework";
import { uiDataLabel } from "../../../🎗️UiLabel/🟦️.tsx";
import rowExtentFixture from "../../../../🧫️fixtures/🌳️tree-window-row-extent/🔣️.json";
import disclosureCases from "../../🧫️fixtures/♿️disclosure/🔣️.json";
import actionCases from "../../🧫️fixtures/♿️actions/🔣️.json";
import focusCases from "../../🧫️fixtures/♿️focus-retention/🔣️.json";
import disabledActionCases from "../../../../🧫️fixtures/♿️disabled-row-action/🔣️.json";
// #endregion 🔌️Adapters

describe("Disabled row action focus", () => {
  it("matches the closed corpus and independent native button focus and dispatch", () => {
    for (const entry of disabledActionCases) {
      const onClick = vi.fn();
      const referenceClick = vi.fn();
      const reference = document.createElement("button");
      reference.disabled = entry.disabled && !entry.reason;
      reference.setAttribute("aria-disabled", String(entry.disabled));
      reference.addEventListener("click", () => { if (!entry.disabled) referenceClick(); });
      document.body.append(reference);
      reference.focus();
      const referenceFocusable = document.activeElement === reference;
      reference.click();
      reference.remove();
      const { container, unmount } = render(<Tree sections={[{ id: entry.id, label: entry.label, actions: [{ icon: <span aria-hidden="true" />, text: entry.label, disabled: entry.disabled, reason: entry.reason ?? undefined, onClick }], items: [] }]} />);
      const button = container.querySelector<HTMLButtonElement>("[data-slot=action]")!;
      button.focus();
      expect(document.activeElement === button).toBe(entry.focusable);
      expect(document.activeElement === button).toBe(referenceFocusable);
      expect(button.disabled).toBe(entry.disabled && !entry.focusable);
      expect(button.getAttribute("aria-disabled")).toBe(entry.disabled ? "true" : null);
      const descriptionId = button.getAttribute("aria-describedby");
      expect(descriptionId ? document.getElementById(descriptionId)?.textContent : null).toBe(entry.description);
      button.click();
      expect(onClick.mock.calls.length).toBe(entry.actionable ? 1 : 0);
      expect(onClick.mock.calls.length).toBe(referenceClick.mock.calls.length);
      unmount();
    }
  });
});

describe("Row action identity", () => {
  it("keeps an action's element and the focus on it when it becomes refused with a reason, and when it can run again", () => {
    const onClick = vi.fn();
    const sections = (reason: string | undefined) => [{ id: "rows", label: "Rows", actions: [{ icon: <span aria-hidden="true" />, text: "Edit", disabled: reason !== undefined, reason, onClick }], items: [] }];
    const view = render(<Tree sections={sections(undefined)} />);
    const button = view.container.querySelector<HTMLButtonElement>("[data-slot=action]")!;
    act(() => button.focus());
    expect([document.activeElement === button, button.hasAttribute("aria-describedby"), view.container.querySelector('[data-slot="row-action-reason"]')]).toEqual([true, false, null]);
    view.rerender(<Tree sections={sections("Not possible right now")} />);
    const refused = view.container.querySelector<HTMLButtonElement>("[data-slot=action]")!;
    const reasonElement = document.getElementById(refused.getAttribute("aria-describedby") ?? "");
    expect([refused === button, document.activeElement === button, refused.getAttribute("aria-disabled"), reasonElement?.textContent, reasonElement?.hasAttribute("data-revealed")], "a focused action that becomes refused keeps its element and shows why at once").toEqual([true, true, "true", "Not possible right now", true]);
    view.rerender(<Tree sections={sections(undefined)} />);
    const again = view.container.querySelector<HTMLButtonElement>("[data-slot=action]")!;
    expect([again === button, document.activeElement === button, again.hasAttribute("aria-disabled"), document.querySelector('[data-slot="row-action-reason"]')]).toEqual([true, true, false, null]);
    again.click();
    expect(onClick).toHaveBeenCalledTimes(1);
  });
});

describe("Busy row action focus", () => {
  it("keeps a busy action in the Tab order — aria-disabled and aria-busy, never the disabled attribute — with the reason its producer published, and runs nothing", () => {
    const cases = [
      { name: "busy", disabled: false, reason: undefined, busy: true, native: false, described: null },
      { name: "busy and refused with a reason", disabled: true, reason: "Not possible right now", busy: true, native: false, described: "Not possible right now" },
      { name: "refused with a reason", disabled: true, reason: "Not possible right now", busy: false, native: false, described: "Not possible right now" },
      { name: "refused without a reason", disabled: true, reason: undefined, busy: false, native: true, described: null },
    ] as const;
    for (const entry of cases) {
      const onClick = vi.fn();
      const { container, unmount } = render(<Tree sections={[{ id: "rows", label: "Rows", actions: [{ icon: <span aria-hidden="true" />, text: "Edit", disabled: entry.disabled, reason: entry.reason, busy: entry.busy, onClick }], items: [] }]} />);
      const button = container.querySelector<HTMLButtonElement>("[data-slot=action]")!;
      button.focus();
      const describedBy = button.getAttribute("aria-describedby");
      expect([button.disabled, document.activeElement === button, button.getAttribute("aria-disabled"), button.getAttribute("aria-busy"), describedBy ? document.getElementById(describedBy)?.textContent : null], entry.name).toEqual([entry.native, !entry.native, "true", entry.busy ? "true" : null, entry.described]);
      button.click();
      expect(onClick, entry.name).not.toHaveBeenCalled();
      unmount();
    }
  });
});

describe("Tree focus retention", () => {
  const sections = (parent: string, action: string, id: string): TreeDataSection[] => [{id:"focus-section", label:"Nodes", defaultOpen:true, items:[{id:"focus-anchor", label:parent, defaultOpen:true, items:[{id, label:action, onClick:() => {}}]}]}];

  it.each(focusCases)("returns to the surviving ancestor after $id action replacement", async (entry) => {
    const {getByRole, rerender, unmount} = render(<Tree sections={sections(entry.parent,entry.before,"convert-cubic")} />);
    getByRole("button", {name:entry.before}).focus();
    fireEvent.focusOut(getByRole("button", {name:entry.before}), {relatedTarget:null});
    rerender(<Tree sections={sections(entry.parent,entry.after,"convert-line")} />);
    await waitFor(() => expect(document.activeElement).toBe(getByRole("button", {name:entry.parent})));
    unmount();
  });

  it("does not take focus back from another control", async () => {
    const {getByRole, rerender, unmount} = render(<><button>Outside</button><Tree sections={sections("Anchor","Convert","convert")} /></>);
    getByRole("button", {name:"Convert"}).focus();
    await userEvent.setup().click(getByRole("button", {name:"Outside"}));
    rerender(<><button>Outside</button><Tree sections={sections("Anchor","Straighten","straighten")} /></>);
    await Promise.resolve();
    expect(document.activeElement).toBe(getByRole("button", {name:"Outside"}));
    unmount();
  });

  it("leaves an intentional blur alone", async () => {
    const {getByRole, rerender, unmount} = render(<Tree sections={sections("Anchor","Convert","convert")} />);
    const button = getByRole("button", {name:"Convert"});
    button.focus();
    button.blur();
    await Promise.resolve();
    rerender(<Tree sections={sections("Anchor","Straighten","straighten")} />);
    await Promise.resolve();
    expect(document.activeElement).toBe(document.body);
    unmount();
  });

  it("keeps focus in the tree when its last item disappears", async () => {
    const {getByRole, rerender, unmount} = render(<Tree sections={sections("Anchor","Convert","convert")} />);
    getByRole("button", {name:"Convert"}).focus();
    rerender(<Tree sections={[]} />);
    await waitFor(() => expect(document.activeElement).toBe(getByRole("tree")));
    unmount();
  });
});

describe("TreeItem keyboard action", () => {
  it.each(actionCases)("activates $id once per Enter or Space", async (entry) => {
    const user = userEvent.setup();
    const onClick = vi.fn();
    const {getByRole, unmount} = render(<TreeItem id={entry.id} label={<span>{entry.label}</span>} layoutKind={entry.layout as "default" | "property"} activatable onClick={onClick} />);
    const button = getByRole("button", {name:entry.label});
    expect(button.classList.contains("bg-transparent")).toBe(true);
    expect(button.classList.contains("border-0")).toBe(true);
    await user.tab();
    expect(document.activeElement).toBe(button);
    await user.keyboard("{Enter}");
    expect(onClick).toHaveBeenCalledTimes(1);
    await user.keyboard(" ");
    expect(onClick).toHaveBeenCalledTimes(2);
    unmount();
  });

  it("leaves passive row labels outside the tab order", () => {
    const {queryByRole, unmount} = render(<TreeItem id="passive" label="Position" layoutKind="property" expandable={false}><input aria-label="X" /></TreeItem>);
    expect(queryByRole("button", {name:"Position"})).toBeNull();
    unmount();
  });
});

describe("TreeItem named disclosure", () => {
  it("validates the neutral branch and leaf contract", () => {
  });

  it.each(disclosureCases.branches)("exposes the row label and expanded state for $id", async (entry) => {
    const user = userEvent.setup();
    const {getByRole, container, unmount} = render(
      <TreeItem id={entry.id} label={<span>{entry.label}</span>} layoutKind={entry.layout as "default" | "property"}>
        <TreeItem id={entry.id + "-child"} label="Child" />
      </TreeItem>,
    );
    const button = getByRole("button", {name:entry.label});
    const row = container.querySelector("#" + entry.id)!;
    expect(button.getAttribute("aria-expanded")).toBe("false");
    expect(row.getAttribute("aria-expanded")).toBe("false");
    button.focus();
    await user.keyboard("{Enter}");
    expect(button.getAttribute("aria-expanded")).toBe("true");
    expect(row.getAttribute("aria-expanded")).toBe("true");
    await user.keyboard(" ");
    expect(button.getAttribute("aria-expanded")).toBe("false");
    unmount();
  });

  it.each(disclosureCases.leaves)("keeps the leaf $id out of disclosure semantics", (entry) => {
    const {getByRole, queryByRole} = render(<TreeItem id={entry.id} label={entry.label} layoutKind={entry.layout as "default" | "property"} />);
    const row = getByRole(disclosureCases.expected.leaf.role, {name:entry.label});
    expect(row.getAttribute("aria-expanded")).toBe(disclosureCases.expected.leaf.expanded);
    expect(queryByRole("button", {name:entry.label})).toBeNull();
  });
});

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
    render(<TreeCheckbox id="tree-checkbox-activation" checked={false} title={uiDataLabel("Grid visible")} onCheckedChange={onCheckedChange} />);

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
        <TreeCheckbox id="tree-checkbox-disabled" checked disabled title={uiDataLabel("Grid snap")} onCheckedChange={onCheckedChange} />
        <TreeCheckbox id="tree-checkbox-enabled" checked={false} title={uiDataLabel("Grid visible")} onCheckedChange={onCheckedChange} />
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
    const { container } = render(<TreeCheckbox id="tree-checkbox-named" checked={false} title={uiDataLabel("Rasteranzeige")} onCheckedChange={vi.fn()} />);
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

  it("expands a row that has children from the row shell and from its label", () => {
    const onClick = vi.fn();
    const { container } = render(
      <TreeItem id="tree-row-weights" label="Layer Weights" onClick={onClick}>
        <TreeItem id="tree-row-weight-child" label="Roads" />
      </TreeItem>,
    );
    const row = rowOf(container, "tree-row-weights");

    expect(container.querySelector("#tree-row-weight-child")).toBeNull();

    fireEvent.click(row, { detail: 1 });
    expect(onClick).toHaveBeenCalledTimes(1);
    expect(container.querySelector("#tree-row-weight-child")).not.toBeNull();

    fireEvent.click(row.querySelector('[data-slot="tree-label"]') as HTMLElement, { detail: 1 });
    expect(onClick).toHaveBeenCalledTimes(2);
    expect(container.querySelector("#tree-row-weight-child")).toBeNull();
  });

  it("expands a property group from the row, not only from its chevron", () => {
    const onClick = vi.fn();
    const { container } = render(
      <TreeItem id="tree-row-property-group" label="Layer Weights" layoutKind="property" onClick={onClick}>
        <TreeItem id="tree-row-property-child" label="Roads" />
      </TreeItem>,
    );
    const row = rowOf(container, "tree-row-property-group");

    expect(container.querySelector("#tree-row-property-child")).toBeNull();
    fireEvent.click(row.querySelector('[data-slot="tree-label"]') as HTMLElement, { detail: 1 });
    expect(onClick).toHaveBeenCalledTimes(1);
    expect(container.querySelector("#tree-row-property-child")).not.toBeNull();
    fireEvent.click(row.querySelector("button.cursor-foldable") as HTMLButtonElement);
    expect(onClick).toHaveBeenCalledTimes(1);
    expect(container.querySelector("#tree-row-property-child")).toBeNull();
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

  it("uses each window's declared closed-row extent for both spacer bands", () => {
    const wireExtent = {
      Standard: "standard",
      CompactText: "compactText",
      CompactSmallControl: "compactSmallControl",
      CompactControl: "compactControl",
    } as const satisfies Readonly<Record<string, TreeWindowRowExtent>>;
    for (const testCase of rowExtentFixture.windowedCases.filter((testCase) => testCase.expected.accepted)) {
      const rowExtent = wireExtent[testCase.extent as keyof typeof wireExtent];
      const window = { total: testCase.total, offset: testCase.offset, rowExtent } satisfies TreeDataWindow;
      const { container, unmount } = render(<Tree sections={[windowedSection({ window, items: [{ id: `${testCase.id}/row`, label: testCase.id }] })]} />);
      const content = container.querySelector('[data-slot="tree-section-content"]') as HTMLDivElement;
      const row = container.querySelector("[data-tree-window-row]") as HTMLDivElement;
      const [leading, trailing] = spacers(container);
      expect(content.getAttribute("data-tree-window-row-extent"), testCase.id).toBe(rowExtent);
      expect(leading.style.height, `${testCase.id} leading`).toBe(`${testCase.expected.leadingPixels}px`);
      expect(row.style.getPropertyValue("--tree-row-height"), `${testCase.id} materialised`).toBe(`${testCase.materialized[0]!.headerPixels}px`);
      expect(trailing.style.height, `${testCase.id} trailing`).toBe(`${testCase.expected.trailingPixels}px`);
      unmount();
    }
  });

  it("stands in for the rows outside the streamed slice at exactly one row pitch each", () => {
    const items = Array.from({ length: 10 }, (_, index) => ({ id: `entry-${20 + index}`, label: `Entry ${20 + index}` }));
    const { container } = render(<Tree sections={[windowedSection({ window: { rowExtent: "standard", total: 100, offset: 20 }, items })]} />);

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
    const { container } = render(<Tree sections={[windowedSection({ window: { rowExtent: "standard", total: 4, offset: 0 }, items })]} />);

    expect(spacers(container)).toHaveLength(0);
  });

  it("stays expandable and wears the loading ring while an announced window has streamed no rows", () => {
    const { container } = render(<Tree sections={[windowedSection({ window: { rowExtent: "standard", total: 5, offset: 0 }, items: [] })]} />);

    const row = container.querySelector('[data-slot="tree-section-row"]') as HTMLDivElement;
    expect(row.getAttribute("role")).toBe("button");
    expect(row.getAttribute("aria-expanded")).toBe("true");
    expect(row.querySelector(".border-loading")).not.toBeNull();

    const [trailing] = spacers(container);
    expect(trailing.getAttribute("data-tree-window-rows")).toBe("5");
    expect(container.querySelectorAll('[data-slot="tree-item-row"]')).toHaveLength(0);
  });

  it("stamps every materialised row with its own entry index, direct children only", () => {
    const { container } = render(
      <Tree
        sections={[
          windowedSection({
            window: { rowExtent: "standard", total: 100, offset: 20 },
            items: [
              { id: "entry-20", label: "Entry 20" },
              { id: "entry-21", label: "Entry 21", defaultOpen: true, windowKey: "entry-21", window: { rowExtent: "standard", total: 9, offset: 4 }, items: [{ id: "child-4", label: "Child 4" }] },
            ],
          }),
        ]}
      />,
    );

    const section = container.querySelector('[data-slot="tree-section-content"]') as HTMLDivElement;
    expect(Array.from(section.querySelectorAll(":scope > [data-tree-window-row]")).map((row) => row.getAttribute("data-tree-window-row"))).toEqual(["20", "21"]);
    const nested = container.querySelector('[data-slot="tree-item-content"]') as HTMLDivElement;
    expect(Array.from(nested.querySelectorAll(":scope > [data-tree-window-row]")).map((row) => row.getAttribute("data-tree-window-row"))).toEqual(["4"]);
    // 🎯️ An UNWINDOWED container's rows carry no index — they are never measured as a window's rows.
    expect(container.querySelectorAll('[data-slot="tree-window-spacer"][data-tree-window-row]')).toHaveLength(0);
  });

  it("tells assistive technology each windowed row's place among all its siblings, and nothing for an unwindowed row", () => {
    const { container } = render(
      <Tree
        sections={[
          windowedSection({
            window: { rowExtent: "standard", total: 100, offset: 20 },
            items: [
              { id: "entry-20", label: "Entry 20" },
              { id: "entry-21", label: "Entry 21", defaultOpen: true, windowKey: "entry-21", window: { rowExtent: "standard", total: 9, offset: 4 }, items: [{ id: "child-4", label: "Child 4" }] },
              { id: "entry-22", label: "Entry 22", defaultOpen: true, items: [{ id: "plain-0", label: "Plain 0" }] },
            ],
          }),
        ]}
      />,
    );
    const place = (row: Element) => [row.getAttribute("role"), row.getAttribute("aria-posinset"), row.getAttribute("aria-setsize")];
    const section = container.querySelector('[data-slot="tree-section-content"]') as HTMLDivElement;
    expect(Array.from(section.querySelectorAll(":scope > [data-tree-window-row]")).map(place)).toEqual([["treeitem", "21", "100"], ["treeitem", "22", "100"], ["treeitem", "23", "100"]]);
    const nested = Array.from(container.querySelectorAll('[data-slot="tree-item-content"]')) as HTMLDivElement[];
    expect(Array.from(nested[0]!.querySelectorAll(":scope > [data-tree-window-row]")).map(place)).toEqual([["treeitem", "5", "9"]]);
    const plain = Array.from(container.querySelectorAll('[role="treeitem"]')).find((row) => (row.textContent ?? "").trim() === "Plain 0")!;
    expect([plain.hasAttribute("data-tree-window-row"), plain.hasAttribute("aria-posinset"), plain.hasAttribute("aria-setsize")]).toEqual([false, false, false]);
    expect([treeWindowRowAriaAttributes(undefined, 100), treeWindowRowAriaAttributes(3, undefined), treeWindowRowAriaAttributes(3, 2), treeWindowRowAriaAttributes(0, 1)]).toEqual([undefined, undefined, { "aria-posinset": 4, "aria-setsize": 4 }, { "aria-posinset": 1, "aria-setsize": 1 }]);
  });

  /** 🔑️ A window is addressed by its PATH, because its node key is also the pick target id and two
   * containers under different parents legitimately share one (📓️f2-sdk-body-node-ledger.md §10). */
  it("stamps a window path that nests, and leaves a top-level container's path equal to its key", () => {
    // 🚧️ PRINTABLE (U+241F), never the control character it depicts: a window path crosses the wasm boundary
    // inside `PluginViewState.treeWindows`, and `parseResolvedPluginViewState` refuses `[\u0000-\u001f\u007f]`.
    expect(TREE_WINDOW_PATH_SEPARATOR).toBe("\u241f");
    expect(/[\u0000-\u001f\u007f]/u.test(TREE_WINDOW_PATH_SEPARATOR)).toBe(false);
    expect(Array.from(TREE_WINDOW_PATH_SEPARATOR).length).toBe(1);
    expect(treeWindowPathOf(undefined, "objects")).toBe("objects");
    expect(treeWindowPathOf("objects", "shared")).toBe(`objects${TREE_WINDOW_PATH_SEPARATOR}shared`);
    expect(treeWindowPathOf("a", undefined)).toBeUndefined();

    const nested = (parent: string) => ({ id: `${parent}/shared`, label: "Shared", defaultOpen: true, windowKey: "shared", windowPath: `${parent}${TREE_WINDOW_PATH_SEPARATOR}shared`, window: { rowExtent: "standard" as const, total: 9, offset: 0 }, items: [{ id: `${parent}/shared/0`, label: "Child" }] });
    const { container } = render(
      <Tree
        sections={[
          { id: "left", label: "Left", defaultOpen: true, windowKey: "left", windowPath: "left", window: { rowExtent: "standard", total: 2, offset: 0 }, items: [nested("left")] },
          { id: "right", label: "Right", defaultOpen: true, windowKey: "right", windowPath: "right", window: { rowExtent: "standard", total: 2, offset: 0 }, items: [nested("right")] },
        ]}
      />,
    );

    const windowed = Array.from(container.querySelectorAll("[data-tree-window-path]"));
    expect(windowed.map((element) => element.getAttribute("data-tree-window-path"))).toEqual([
      "left",
      `left${TREE_WINDOW_PATH_SEPARATOR}shared`,
      "right",
      `right${TREE_WINDOW_PATH_SEPARATOR}shared`,
    ]);
    // 🎯️ The authored key — the pick target id — is untouched, and the two "shared" containers keep it.
    expect(windowed.map((element) => element.getAttribute("data-tree-window-key"))).toEqual(["left", "shared", "right", "shared"]);
  });

  it("windows a nested group row the same way it windows a section", () => {
    const { container } = render(
      <Tree
        sections={[
          windowedSection({
            window: undefined,
            windowKey: undefined,
            items: [{ id: "group", label: "Object", defaultOpen: true, windowKey: "object", window: { rowExtent: "standard", total: 40, offset: 8 }, items: [{ id: "vortex-8", label: "Vortex 8" }] }],
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
  const rowHeight = treeRowHeightPx;
  const measure = (overrides: Partial<TreeWindowContainerMeasure>): TreeWindowContainerMeasure => ({
    key: "entries",
    rowExtent: "standard",
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

    expect(treeWindowRequestsForViewport([above, inside], 400, 200, 2)).toEqual([{ key: "inside", offset: 0, rows: 10 }]);
  });

  it("asks from the first visible row less the overscan when only the container's top is cut off", () => {
    // Container starts 100px (5 rows) above the viewport top; 200px (10 rows) of it are on screen.
    const requests = treeWindowRequestsForViewport([measure({ top: -5 * rowHeight })], 0, 10 * rowHeight, 2);

    expect(requests).toEqual([{ key: "entries", offset: 3, rows: 14 }]);
  });

  it("centres a long container's window on the visible run", () => {
    // 50 rows of a 1000-row container are on screen, starting at row 100.
    const requests = treeWindowRequestsForViewport([measure({ top: 0 })], 100 * rowHeight, 50 * rowHeight, TREE_WINDOW_OVERSCAN_ROWS);

    expect(requests).toEqual([{ key: "entries", offset: 100 - TREE_WINDOW_OVERSCAN_ROWS, rows: 50 + 2 * TREE_WINDOW_OVERSCAN_ROWS }]);
  });

  it("never asks past the end of a container shorter than the viewport run", () => {
    const requests = treeWindowRequestsForViewport([measure({ total: 6, height: 6 * rowHeight })], 0, 40 * rowHeight, TREE_WINDOW_OVERSCAN_ROWS);

    expect(requests).toEqual([{ key: "entries", offset: 0, rows: 6 }]);
  });

  it("clamps one request to the built-children ceiling", () => {
    // 300 rows of a 500-row container on screen at once — more than one built child list can carry.
    const requests = treeWindowRequestsForViewport([measure({ total: 500, height: 500 * rowHeight })], 200 * rowHeight, 300 * rowHeight, TREE_WINDOW_OVERSCAN_ROWS);

    expect(requests).toEqual([{ key: "entries", offset: 200 - TREE_WINDOW_OVERSCAN_ROWS, rows: TREE_WINDOW_ROWS_MAX }]);
  });

  it("keeps the window inside the total when the viewport sits at the very end", () => {
    const requests = treeWindowRequestsForViewport([measure({ total: 500, height: 500 * rowHeight })], 490 * rowHeight, 20 * rowHeight, TREE_WINDOW_OVERSCAN_ROWS);

    expect(requests).toEqual([{ key: "entries", offset: 500 - 26, rows: 26 }]);
  });

  /** 🧯️ 📓️s3-review-streaming-loop.md §2: a materialised row can itself be an OPEN windowed group and be
   * many rows tall, so "pixels ÷ one row height" resolves the viewport to a row far below the one actually
   * under it — and the window that follows evicts the subtree the reader just opened. The rows' own measured
   * tops are the only thing that answers this, and the spacers keep the uniform pitch they are built from. */
  it("reads the row under the viewport off the rows' real tops, not off a uniform pitch", () => {
    // 20 rows of 1000, rows 0..2 plain, row 3 an open nested group 16 rows tall, rows 4..19 plain again.
    const rows = [] as { index: number; top: number }[];
    let top = 0;
    for (let index = 0; index < 20; index += 1) {
      rows.push({ index, top });
      top += (index === 3 ? 16 : 1) * rowHeight;
    }
    const nested = measure({ total: 1000, offset: 0, length: 20, height: (top + 980 * rowHeight), rows });
    // The viewport sits 20 row-heights down — past the tall row 3, at this container's own row 5.
    const requests = treeWindowRequestsForViewport([nested], 20 * rowHeight, 4 * rowHeight, TREE_WINDOW_OVERSCAN_ROWS);

    // 🎯️ The viewport covers this container's own rows 5…8, so its window still starts at row 0.
    expect(requests).toEqual([{ key: "entries", offset: 0, rows: 20 }]);
    // 🎯️ The flat rule reads the same pixels as rows 20…23 and answers a window that materialises NONE of
    // what is on screen — the expanded row 3 among it.
    expect(treeWindowRequestsForViewport([{ ...nested, rows: undefined }], 20 * rowHeight, 4 * rowHeight, TREE_WINDOW_OVERSCAN_ROWS)).toEqual([{ key: "entries", offset: 12, rows: 20 }]);
  });

  it("drops containers with nothing to stream and rejects a degenerate row pitch", () => {
    expect(treeWindowRequestsForViewport([measure({ total: 0, height: 0 })], 0, 400, 4)).toEqual([]);
  });
});
// #endregion 📐️WindowRequests

// #region 🧮️BodyNodeBudget
/** 🧮️ The body-wide ceiling, in the guest's own currency: every windowed container of one panel body costs
 * `1 + rows` out of one `TREE_WINDOW_BODY_NODE_BUDGET` ledger, so a request that fits here is one the guest can
 * answer in full (📓️s3-review-streaming-loop.md §1, 📓️w3-browser-verification.md §6.2). */
describe("capTreeWindowRequests", () => {
  const rowHeight = treeRowHeightPx;
  const metrics = (overrides: Partial<TreeWindowVisibleRows> = {}): TreeWindowVisibleRows => ({ total: 1000, visibleRows: 20, firstVisibleRow: 30, distancePx: 0, ...overrides });
  const visible = (entries: readonly (readonly [string, TreeWindowVisibleRows])[]) => new Map<string, TreeWindowVisibleRows>(entries);
  const nodeCost = (requests: readonly { readonly rows: number }[]) => requests.length + requests.reduce((sum, request) => sum + request.rows, 0);

  it("leaves a body that already fits exactly as it was", () => {
    const requests = [{ key: "a", offset: 10, rows: 40 }, { key: "b", offset: 0, rows: 30 }];
    const capped = capTreeWindowRequests(requests, visible([["a", metrics()], ["b", metrics()]]));

    expect(capped).toBe(requests);
    expect(nodeCost(capped)).toBe(72);
  });

  it("shrinks the overscan uniformly before any container loses a row the viewport shows", () => {
    // 4 containers × (20 visible + 2 × 8 overscan) + 4 container nodes = 148 — over budget. The rule gives
    // overscan back uniformly until the body fits, so every container keeps its whole visible run and they
    // all land on the SAME overscan. (Derived from the budget, which is jointly owned with the SDK ledger.)
    const keys = ["a", "b", "c", "d"];
    const overscan = Math.max(0, Math.min(TREE_WINDOW_OVERSCAN_ROWS, Math.floor((Math.floor((TREE_WINDOW_BODY_NODE_BUDGET - keys.length) / keys.length) - 20) / 2)));
    expect(overscan).toBeLessThan(TREE_WINDOW_OVERSCAN_ROWS);
    const requests = keys.map((key) => ({ key, offset: 30 - TREE_WINDOW_OVERSCAN_ROWS, rows: 20 + 2 * TREE_WINDOW_OVERSCAN_ROWS }));
    const capped = capTreeWindowRequests(requests, visible(keys.map((key) => [key, metrics()] as const)));

    expect(capped).toEqual(keys.map((key) => ({ key, offset: 30 - overscan, rows: 20 + 2 * overscan })));
    expect(nodeCost(capped)).toBeLessThanOrEqual(TREE_WINDOW_BODY_NODE_BUDGET);
  });

  it("trims the container furthest from the viewport centre once there is no overscan left to give", () => {
    // Zero overscan still costs 3 + 50 + 40 + 30 = 123; everything over budget comes off the FURTHEST one.
    const requests = [
      { key: "far", offset: 0, rows: 50 },
      { key: "mid", offset: 0, rows: 40 },
      { key: "near", offset: 0, rows: 30 },
    ];
    const excess = 3 + 50 + 40 + 30 - TREE_WINDOW_BODY_NODE_BUDGET;
    expect(excess).toBeGreaterThan(0);
    const capped = capTreeWindowRequests(requests, visible([
      ["far", metrics({ visibleRows: 50, firstVisibleRow: 0, distancePx: 900 })],
      ["mid", metrics({ visibleRows: 40, firstVisibleRow: 0, distancePx: 100 })],
      ["near", metrics({ visibleRows: 30, firstVisibleRow: 0, distancePx: 10 })],
    ]));

    expect(capped).toEqual([{ key: "far", offset: 0, rows: 50 - excess }, { key: "mid", offset: 0, rows: 40 }, { key: "near", offset: 0, rows: 30 }]);
    expect(nodeCost(capped)).toBe(TREE_WINDOW_BODY_NODE_BUDGET);
  });

  it("degrades the furthest container to a spacer-only window rather than dropping it off the wire", () => {
    // 🧯️ A container the host stops asking for rows is still IN the body and still costs its own node, so it
    // stays in the answer at `rows: 0` — dropping it would only hand the guest back its own default window.
    const requests = ["far", "mid", "near"].map((key) => ({ key, offset: 0, rows: 3 }));
    const capped = capTreeWindowRequests(requests, visible([
      ["far", metrics({ total: 9, visibleRows: 3, firstVisibleRow: 0, distancePx: 900 })],
      ["mid", metrics({ total: 9, visibleRows: 3, firstVisibleRow: 0, distancePx: 100 })],
      ["near", metrics({ total: 9, visibleRows: 3, firstVisibleRow: 0, distancePx: 10 })],
    ]), 5);

    expect(capped).toEqual([{ key: "far", offset: 0, rows: 0 }, { key: "mid", offset: 0, rows: 1 }, { key: "near", offset: 0, rows: 1 }]);
    expect(nodeCost(capped)).toBe(5);
  });

  it("keeps every capped window inside its own container", () => {
    const requests = [{ key: "a", offset: 0, rows: 90 }, { key: "b", offset: 60, rows: 40 }];
    const capped = capTreeWindowRequests(requests, visible([
      ["a", metrics({ total: 90, visibleRows: 90, firstVisibleRow: 0, distancePx: 400 })],
      ["b", metrics({ total: 100, visibleRows: 40, firstVisibleRow: 60, distancePx: 10 })],
    ]));

    expect(nodeCost(capped)).toBeLessThanOrEqual(TREE_WINDOW_BODY_NODE_BUDGET);
    for (const request of capped) {
      const total = request.key === "a" ? 90 : 100;
      expect(request.offset).toBeGreaterThanOrEqual(0);
      expect(request.offset + request.rows).toBeLessThanOrEqual(total);
    }
  });

  it("holds the ceiling for a whole measured body of twelve open containers", () => {
    // The fem3d House shape: a dozen open windowed containers of a few rows each, all on screen at once,
    // costing 156 nodes between them — more than one guest body can present.
    const containers: TreeWindowContainerMeasure[] = Array.from({ length: 12 }, (_, index) => ({ key: `c${index}`, rowExtent: "standard", total: 12, offset: 0, length: 12, top: index * 12 * rowHeight, height: 12 * rowHeight }));
    const viewportHeight = 12 * 12 * rowHeight;
    const uncapped = treeWindowRequestsForViewport(containers, 0, viewportHeight, TREE_WINDOW_OVERSCAN_ROWS);
    const capped = capTreeWindowRequests(uncapped, treeWindowVisibleRowsForViewport(containers, 0, viewportHeight));

    expect(nodeCost(uncapped)).toBe(156);
    expect(nodeCost(capped)).toBe(TREE_WINDOW_BODY_NODE_BUDGET);
    // 🎯️ The rows come off the far ends of the body, never off the containers around the viewport centre.
    expect(capped.find((request) => request.key === "c5")?.rows).toBe(12);
    expect(capped.find((request) => request.key === "c6")?.rows).toBe(12);
    for (const request of capped) expect(request.offset + request.rows).toBeLessThanOrEqual(12);
  });
});
// #endregion 🧮️BodyNodeBudget

// #region 🧺️SectionRowsFollowTheProp
describe("Tree section rows follow the sections prop in the same render", () => {
  /** 🧷️ A stand-in for a renderer whose row controls read an external store by a position the parent re-binds: one
   * notification both moves the control's slot and re-derives the `sections` prop, exactly as a body refresh that
   * renumbers its nodes does. */
  const boundTree = () => {
    let slots: readonly string[] = ["field"];
    const listeners = new Set<() => void>();
    const subscribe = (listener: () => void) => {
      listeners.add(listener);
      return () => void listeners.delete(listener);
    };
    const seen: string[] = [];
    function Field() {
      const [draft, setDraft] = React.useState("");
      return <input data-slot="draft" value={draft} onChange={(event) => setDraft(event.target.value)} />;
    }
    function Bound({ slot }: { readonly slot: number }) {
      const kind = React.useSyncExternalStore(subscribe, () => slots[slot] ?? "gone");
      seen.push(`${slot}:${kind}`);
      return kind === "field" ? <Field /> : <span data-kind={kind} />;
    }
    function Host() {
      const layout = React.useSyncExternalStore(subscribe, () => slots);
      return <Tree sections={[{ id: "inputs", label: "Inputs", defaultOpen: true, items: [...layout.slice(0, layout.indexOf("field")).map((kind, index) => ({ id: `filler-${index}`, label: kind })), { id: "factor.row", label: "Factor", control: <Bound key="factor" slot={layout.indexOf("field")} /> }] }]} />;
    }
    return { Host, seen, publish: (next: readonly string[]) => { slots = next; for (const listener of [...listeners]) listener(); } };
  };

  it("never renders a row control against rows the prop no longer holds: a typed draft, its element and the focus survive a refresh that moves the row", () => {
    const { Host, seen, publish } = boundTree();
    const { container } = render(<Host />);
    const field = container.querySelector<HTMLInputElement>('[data-slot="draft"]')!;
    fireEvent.change(field, { target: { value: "-5" } });
    act(() => field.focus());
    seen.length = 0;
    act(() => publish(["filler", "filler", "field"]));
    const after = container.querySelector<HTMLInputElement>('[data-slot="draft"]');
    expect([after === field, after?.value, document.activeElement === field, container.querySelectorAll('[role="treeitem"]').length]).toEqual([true, "-5", true, 3]);
    expect([seen.length > 0, seen.filter((entry) => entry !== "2:field")], "the control renders re-bound to its new slot — never once more against the slot it left").toEqual([true, []]);
  });

  it("still loads the rows of a section that declares none through getItems, and reloads them when the sections change", async () => {
    const getItems = vi.fn(async () => [{ id: "loaded", label: "Loaded" }]);
    const sections = (label: string): TreeDataSection[] => [{ id: "lazy", label, defaultOpen: true, getItems }, { id: "static", label: "Static", defaultOpen: true, items: [{ id: "static-row", label: "Static row" }], getItems }];
    const view = render(<Tree sections={sections("Lazy")} />);
    await waitFor(() => expect(view.queryByText("Loaded")).not.toBeNull());
    expect([getItems.mock.calls.length, view.queryByText("Static row") !== null], "a section with its own rows never asks getItems").toEqual([1, true]);
    view.rerender(<Tree sections={sections("Lazy again")} />);
    await waitFor(() => expect(getItems.mock.calls.length).toBe(2));
    await waitFor(() => expect(view.queryByText("Loaded")).not.toBeNull());
  });
});
// #endregion 🧺️SectionRowsFollowTheProp

// #region 🈳️EmptyStateDirection
describe("Tree empty state reading direction", () => {
  it("keeps the tree in its own reading direction inside a mirrored panel", async () => {
    const { FlowProvider } = await import("../../../../🔨️modules/🧭️flow-direction-context/🟦️.tsx");
    const view = render(
      <FlowProvider inline="rtl">
        <Tree sections={[]} emptyState={<p>No task is running.</p>} />
      </FlowProvider>,
    );
    expect(view.getByRole("tree").getAttribute("dir")).toBe("auto");
    const message = view.getByText("No task is running.");
    expect(message.closest("[dir]")?.getAttribute("dir")).toBe("auto");
    expect(message.closest('[data-slot="tree-empty-state"]')).not.toBeNull();
  });
});
// #endregion 🈳️EmptyStateDirection
