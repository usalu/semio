import * as React from "react";
import { act, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { ContextMenuController } from "../../🟦️.tsx";
import { uiDataLabel } from "../../../🎗️UiLabel/🟦️.tsx";
import { ShellScopeContext, type ShellScope } from "../../../🐚️ShellScope/🟦️.tsx";

afterEach(() => {
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe("fused context menu placement", () => {
  it("repositions successive wings and reclamps after fusion growth, resize and collapse", () => {
    let wingWidth = 90;
    const callbacks: ResizeObserverCallback[] = [];
    vi.stubGlobal("ResizeObserver", class { constructor(callback: ResizeObserverCallback) { callbacks.push(callback); } observe() {} disconnect() {} });
    const rect = (left: number, top: number, width: number, height: number) => ({ left, top, right: left + width, bottom: top + height, width, height }) as DOMRect;
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      const slot = this.getAttribute("data-slot");
      if (slot === "context-menu-content") {
        const body = this.querySelector<HTMLElement>('[data-slot="context-menu-fusion-body"]');
        return rect(parseFloat(this.style.left) || 0, 20, 100 + (parseFloat(body?.style.paddingRight ?? "") || 0), 120);
      }
      if (slot === "context-menu-fusion-body") return rect(0, 40, 100 + (parseFloat(this.style.paddingRight) || 0), 100);
      const scrollLeft = this.closest<HTMLElement>('[data-slot="context-menu-fusion-body"]')?.scrollLeft ?? 0;
      if (slot === "context-menu-fusion-primary") return rect(-scrollLeft, 40, 100, 100);
      if (slot === "context-menu-submenu") return rect((parseFloat(this.style.left) || 0) - scrollLeft, 40 + (parseFloat(this.style.top) || 0), wingWidth, 80);
      const wing = this.closest<HTMLElement>('[data-slot="context-menu-submenu"]');
      return rect((wing ? parseFloat(wing.style.left) || 0 : 0) - scrollLeft, wing ? 40 + (parseFloat(wing.style.top) || 0) : 60, wing ? wingWidth : 100, 20);
    });
    const items = [{ id: "parent", label: uiDataLabel("Parent"), children: [{ id: "nested", label: uiDataLabel("Nested"), children: [{ id: "leaf", label: uiDataLabel("Leaf") }] }] }];
    const { getByRole, unmount } = render(<ContextMenuController open position={{ x: window.innerWidth - 100, y: 40 }} items={items} onOpenChange={() => undefined} title={uiDataLabel("Actions")} />);
    fireEvent.click(getByRole("menuitem", { name: /Parent/u }));
    fireEvent.click(getByRole("menuitem", { name: /Nested/u }));
    const wings = document.querySelectorAll<HTMLElement>('[data-slot="context-menu-submenu"]');
    const chrome = document.querySelector<HTMLElement>('[data-slot="context-menu-content"]')!;
    expect(wings[0]!.style.left).toBe("100px");
    expect(wings[1]!.style.left).toBe("190px");
    expect(chrome.style.left).toBe(`${window.innerWidth - 280}px`);
    wingWidth = 120;
    act(() => callbacks.forEach(callback => callback([], {} as ResizeObserver)));
    expect(wings[1]!.style.left).toBe("220px");
    expect(chrome.style.left).toBe(`${window.innerWidth - 340}px`);
    const originalWidth = window.innerWidth;
    vi.stubGlobal("innerWidth", originalWidth - 50);
    fireEvent.resize(window);
    expect(chrome.style.left).toBe(`${window.innerWidth - 340}px`);
    fireEvent.click(getByRole("menuitem", { name: /Parent/u }));
    expect(chrome.style.left).toBe(`${window.innerWidth - 100}px`);
    unmount();
  });

  it("reveals keyboard-focused rows inside an oversized fusion scroller", () => {
    const rect = (top: number, height: number) => ({ left: 0, top, right: 100, bottom: top + height, width: 100, height }) as DOMRect;
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      return this.matches('[role="menuitem"]') ? rect(160, 20) : rect(0, 100);
    });
    const { getByRole, unmount } = render(<ContextMenuController open position={{ x: 10, y: 10 }} items={[{ id: "a", label: uiDataLabel("First") }]} title={uiDataLabel("Actions")} onOpenChange={() => undefined} />);
    const scroller = document.querySelector<HTMLElement>('[data-slot="context-menu-fusion-body"]')!;
    fireEvent.keyDown(getByRole("menu"), { key: "ArrowDown" });
    expect(document.activeElement).toBe(getByRole("menuitem", { name: "First" }));
    expect(scroller.scrollTop).toBe(80);
    unmount();
  });

  it("keeps shell portal ownership and leaves the silhouette host unclipped", () => {
    const layer = document.createElement("div");
    document.body.append(layer);
    const scope = { portalLayerRef: { current: layer } } as ShellScope;
    const { unmount } = render(<ShellScopeContext.Provider value={scope}><ContextMenuController open position={{ x: 10, y: 10 }} items={[{ id: "a", label: uiDataLabel("A") }]} title={uiDataLabel("Actions")} onOpenChange={() => undefined} /></ShellScopeContext.Provider>);
    const chrome = layer.querySelector<HTMLElement>('[data-slot="context-menu-content"]')!;
    expect(chrome.parentElement).toBe(layer);
    expect(chrome.className).not.toContain("overflow-y-auto");
    unmount();
    layer.remove();
  });
});
