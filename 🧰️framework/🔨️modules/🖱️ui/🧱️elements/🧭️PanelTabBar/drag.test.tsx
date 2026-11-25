import * as React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { act, cleanup, fireEvent, render } from "@testing-library/react";
import { PanelTabBar, singleTreeLeaf, type PanelTabNode } from "./🟦️.tsx";
import { PanelDockContext, type PanelDockContextValue } from "../../🔨️modules/🧲️panel-dock/🟦️.tsx";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  document.documentElement.removeAttribute("data-drag-active");
});

const Stub = (): null => null;
const leaf = (id: string) => singleTreeLeaf({ id: `tab-${id}`, icon: Stub, name: `Tab ${id}`, tree: { sections: [] } });
const branch = (id: string): PanelTabNode => ({ kind: "branch", id: `branch-${id}`, icon: Stub, name: `Branch ${id}`, children: [leaf(`${id}1`)] });
const flat = [leaf("a"), leaf("b"), leaf("c")];

const dock = (over: Partial<PanelDockContextValue>): PanelDockContextValue => ({ dragTabId: null, draggedSubtreeIds: null, dropTarget: null, startTabDrag: () => undefined, registerTabRowDropTarget: () => undefined, onTreeUnitDockDrop: () => undefined, ...over });

function mockRects() {
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
    const id = this.dataset.tabId;
    if (id !== undefined) {
      const index = ["tab-a", "tab-b", "tab-c", "branch-x", "branch-y"].indexOf(id);
      return new DOMRect(index * 50, 0, 50, 24);
    }
    if (this.dataset.slot === "panel-tabs") return new DOMRect(0, 0, 250, 24);
    return new DOMRect();
  });
}

const mount = (tabs: PanelTabNode[], value: PanelDockContextValue) =>
  render(
    <PanelDockContext.Provider value={value}>
      <PanelTabBar variant="panel" anchor="top-left" tabs={tabs} activePath={[]} onActivePathChange={() => undefined} />
    </PanelDockContext.Provider>,
  );

describe("🫳️ a tab drag in a panel tab bar", () => {
  it("shows nothing of a drag at rest", () => {
    const view = mount(flat, dock({}));
    expect(view.container.querySelector("[data-drop-place]")).toBeNull();
    expect(view.container.querySelector("[data-drag-hidden]")).toBeNull();
    expect(view.container.querySelector('[data-slot="panel-tab-insert-preview"]')).toBeNull();
  });

  it("hides the dragged tab in place without dimming it and tints the row as a place", () => {
    const view = mount(flat, dock({ dragTabId: "tab-b", draggedSubtreeIds: new Set(["tab-b"]) }));
    const source = view.container.querySelector<HTMLElement>('[data-tab-id="tab-b"]')!;
    expect(source.hasAttribute("data-drag-hidden")).toBe(true);
    expect(source.className).not.toMatch(/opacity-40/);
    expect(view.container.querySelector<HTMLElement>('[data-slot="panel-tabs"]')!.hasAttribute("data-drop-place")).toBe(true);
    expect(view.container.querySelector('[data-tab-id="tab-a"]')!.hasAttribute("data-drag-hidden")).toBe(false);
  });

  it("shows the insert position as a rectangle over half of the next tab, inside the row", () => {
    mockRects();
    const view = mount(flat, dock({ dragTabId: "tab-c", draggedSubtreeIds: new Set(["tab-c"]), dropTarget: { kind: "insert", anchor: "top-left", parentPath: [], index: 1 } }));
    const slot = view.container.querySelector<HTMLElement>('[data-slot="panel-tab-insert-preview"]')!;
    const row = view.container.querySelector<HTMLElement>('[data-slot="panel-tabs"]')!;
    expect(slot.parentElement).toBe(row);
    expect(slot.hasAttribute("data-drop-place")).toBe(true);
    expect(slot.hasAttribute("data-drop-active")).toBe(true);
    expect(slot.className).toMatch(/absolute/);
    expect(slot.className).not.toMatch(/rounded|w-0\.5/);
    expect([slot.style.left, slot.style.width, slot.style.top, slot.style.height]).toEqual(["50px", "25px", "0px", "24px"]);
  });

  it("shows the insert position at the end as the second half of the last tab", () => {
    mockRects();
    const view = mount(flat, dock({ dragTabId: "tab-a", draggedSubtreeIds: new Set(["tab-a"]), dropTarget: { kind: "insert", anchor: "top-left", parentPath: [], index: 2 } }));
    const slot = view.container.querySelector<HTMLElement>('[data-slot="panel-tab-insert-preview"]')!;
    expect([slot.style.left, slot.style.width]).toEqual(["125px", "25px"]);
  });

  it("offers a branch as a place to nest into and tints the one under the pointer more strongly", () => {
    const tabs = [leaf("a"), branch("x"), branch("y")];
    const view = mount(tabs, dock({ dragTabId: "tab-a", draggedSubtreeIds: new Set(["tab-a"]), dropTarget: { kind: "child", anchor: "top-left", parentId: "branch-y" } }));
    const nest = (id: string) => view.container.querySelector<HTMLElement>(`[data-tab-id="${id}"]`)!;
    expect([nest("branch-x").hasAttribute("data-drop-place"), nest("branch-x").hasAttribute("data-drop-active")]).toEqual([true, false]);
    expect([nest("branch-y").hasAttribute("data-drop-place"), nest("branch-y").hasAttribute("data-drop-active")]).toEqual([true, true]);
    expect(nest("tab-a").hasAttribute("data-drop-place")).toBe(false);
    expect(nest("branch-y").className).not.toMatch(/bg-accent/);
  });

  it("does not offer the dragged branch or its row as a place", () => {
    const tabs = [leaf("a"), branch("x")];
    const view = mount(tabs, dock({ dragTabId: "branch-x", draggedSubtreeIds: new Set(["branch-x", "tab-x1"]) }));
    expect(view.container.querySelector('[data-tab-id="branch-x"]')!.hasAttribute("data-drop-place")).toBe(false);
  });
});

describe("🫳️ a tree unit drag over the tabs of a panel", () => {
  it("offers a leaf tab as a place and tints it more strongly while the pointer is over it", async () => {
    const { beginPanelTreeUnitDrag, endPanelTreeUnitDrag } = await import("../../🔨️modules/🧲️panel-dock/🟦️.tsx");
    const view = mount(flat, dock({}));
    act(() => beginPanelTreeUnitDrag({ tabId: "tab-a", unitId: "tab-a.tree", label: "Unit" }));
    const tab = view.container.querySelector<HTMLElement>('[data-tab-id="tab-b"]')!;
    expect([tab.hasAttribute("data-drop-place"), tab.hasAttribute("data-drop-active")]).toEqual([true, false]);
    act(() => void fireEvent.dragEnter(tab));
    expect(tab.hasAttribute("data-drop-active")).toBe(true);
    act(() => void fireEvent.dragLeave(tab));
    expect(tab.hasAttribute("data-drop-active")).toBe(false);
    act(() => endPanelTreeUnitDrag());
    expect(tab.hasAttribute("data-drop-place")).toBe(false);
  });
});
