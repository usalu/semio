import * as React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { act, cleanup, render } from "@testing-library/react";
import { ModeDropPlaces } from "../🧲️drop-places/🟦️.tsx";
import { ModeDockStack, ModeDockContext, type ModeDockContextValue } from "../⚓️dock/🟦️.tsx";
import { DragGhost } from "../../../🔨️modules/🫳️pointer-drag/🟦️.tsx";
import { modeDockTabInsertPreviewClass } from "../🧲️drag-preview/🟦️.tsx";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

const rect = (left: number, top: number, width: number, height: number) => new DOMRect(left, top, width, height);
const element = (bounds: DOMRect): HTMLElement => {
  const node = document.createElement("div");
  node.getBoundingClientRect = () => bounds;
  return node;
};
const modeRect = rect(100, 50, 400, 300);
const stacks = () => ({
  current: new Map([
    ["0", { path: [0], corners: {}, body: element(rect(100, 50, 200, 300)) }],
    ["1", { path: [1], corners: {}, body: element(rect(300, 50, 200, 300)) }],
  ]),
});
const host = () => ({ current: element(modeRect) });

const places = (container: HTMLElement) => [...container.querySelectorAll<HTMLElement>("[data-drop-place]")];
const box = (place: HTMLElement) => [place.style.left, place.style.top, place.style.width, place.style.height];

describe("🫳️ a window drag on the canvas shows every place it can be dropped", () => {
  it("tints every stack as a place, inside the canvas and above its content", () => {
    const view = render(<ModeDropPlaces stacks={stacks()} host={host()} zone={null} />);
    const layer = view.container.querySelector<HTMLElement>('[data-slot="mode-dock-drop-places"]')!;
    expect(layer.className).toMatch(/pointer-events-none/);
    expect(layer.getAttribute("aria-hidden")).toBe("true");
    expect(places(view.container).map((place) => [box(place), place.hasAttribute("data-drop-active")])).toEqual([
      [["0px", "0px", "200px", "300px"], false],
      [["200px", "0px", "200px", "300px"], false],
    ]);
  });

  it("tints the half of the stack a split lands on more strongly, as a rectangle without border", () => {
    const view = render(<ModeDropPlaces stacks={stacks()} host={host()} zone={{ kind: "split", stackPath: [1], side: "left" }} />);
    const active = places(view.container).filter((place) => place.hasAttribute("data-drop-active"));
    expect(active.map(box)).toEqual([["200px", "0px", "100px", "300px"]]);
    expect(active[0]!.getAttribute("data-slot")).toBe("mode-dock-drop-indicator");
    expect(active[0]!.className).not.toMatch(/border|accent|rounded/);
  });

  it("tints the half of the canvas a root split lands on", () => {
    const view = render(<ModeDropPlaces stacks={stacks()} host={host()} zone={{ kind: "rootSplit", side: "bottom" }} />);
    const active = places(view.container).filter((place) => place.hasAttribute("data-drop-active"));
    expect(active.map(box)).toEqual([["0px", "150px", "400px", "150px"]]);
  });

  it("shows no extra rectangle for a drop among the tabs, whose slot is the tab row itself", () => {
    const view = render(<ModeDropPlaces stacks={stacks()} host={host()} zone={{ kind: "tab", stackPath: [0], corner: "topLeft", index: 0 }} />);
    expect(places(view.container).filter((place) => place.hasAttribute("data-drop-active"))).toHaveLength(0);
    expect(view.container.querySelector('[data-slot="mode-dock-drop-indicator"]')).toBeNull();
  });

  it("fills the slot among the tabs with the active tint and no colour of its own", () => {
    expect(modeDockTabInsertPreviewClass).not.toMatch(/bg-|border|rounded/);
  });

  it("travels as the glass copy of its title above the page", () => {
    render(<DragGhost x={5} y={6}>Terminal</DragGhost>);
    const ghost = document.body.querySelector<HTMLElement>("[data-drag-ghost]")!;
    expect([ghost.textContent, ghost.getAttribute("data-level"), ghost.hasAttribute("data-glass-chip")]).toEqual(["Terminal", "menu", true]);
  });
});

describe("🫳️ the tab rows of a stack while a window travels", () => {
  const dock = (dragState: ModeDockContextValue["dragState"]): ModeDockContextValue => ({ dragState, tabInsertPreview: null, draggedInsertTabs: [], registerStackDropTargets: () => undefined, startTabDrag: () => undefined, clearPendingDrag: () => undefined, closeWindow: () => undefined, activateWindow: () => undefined, deactivateActiveWindow: () => undefined, maximizedStackPath: null, canMaximize: false, toggleMaximize: () => undefined });
  const stack = { kind: "stack" as const, children: [{ kind: "window" as const, windowKindId: "a" }], activeWindowId: "a" };
  const mount = (value: ModeDockContextValue) =>
    render(
      <ModeDockContext.Provider value={value}>
        <ModeDockStack stackPath={[0]} node={stack} windowsById={new Map([["a", { iconId: "app-window", title: "A", children: null } as never]])} activeWindowId="a" />
      </ModeDockContext.Provider>,
    );

  it("are places while a window travels and are none at rest", () => {
    const at = (container: HTMLElement) => container.querySelector<HTMLElement>('[data-slot="mode-dock-tabs"]')!;
    expect(at(mount(dock(null)).container).hasAttribute("data-drop-place")).toBe(false);
    cleanup();
    const traveling = mount(dock({ dragKind: "tab", windowId: "x", stackPath: [1], tabIndex: 0, pointerId: 1, ghostLabel: "X", x: 0, y: 0 }));
    expect(at(traveling.container).hasAttribute("data-drop-place")).toBe(true);
  });
});
