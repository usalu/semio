/** 🔶️ The validates accepted chrome epoch tests of the wgpu accessibility interaction contract suite. */
import { act } from "react";
import { createRoot } from "react-dom/client";
import { expect, it, vi } from "vitest";
import { createAccessibilityMirror, type AccessibilityProjectionWindow } from "@semio-tech/framework-renderer-wgpu/accessibility-mirror";
import { DockTabKeyboardOracle, NestedPanelOracle, PressedChromeOracle, SettingsStripOracle, computeAccessibleName, dockTabKeyboardFixture, fixture, getRole, mounted } from "../🟣️shared/🟦️.tsx";
import type { WireEvent } from "../🟣️shared/🟦️.tsx";

/** 🔷 Registers the validates accepted chrome epoch tests of the wgpu accessibility interaction contract suite. */
export function registerValidatesAcceptedChromeEpochTests(): void {
  it("validates the accepted chrome epoch and bounded Settings strip grammar", () => {
    expect(fixture.presentedChrome.successorEpoch).toBeGreaterThan(fixture.presentedChrome.firstEpoch);
    const rule = fixture.presentedChrome.generationRule;
    expect(fixture.presentedChrome.controls.map((control) => control.id)).toContain(rule.controlId);
    expect(rule.sameChromeRect).toEqual(rule.firstRect);
    expect(rule.changedChromeRect).not.toEqual(rule.firstRect);
    expect(new Set(fixture.settingsTabStrip.tabs.map((tab) => tab.id)).size).toBe(fixture.settingsTabStrip.tabs.length);
  });
  it("matches fullscreen and utility pressed buttons against the production React Toggle", () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);
    mounted.push(root);
    act(() => root.render(<PressedChromeOracle />));
    for (const row of [fixture.chromePressedSemantics.fullscreen, fixture.chromePressedSemantics.utility.toggle]) {
      const button = container.querySelector<HTMLButtonElement>(`[id="${row.id}"]`)!;
      expect(getRole(button)).toBe("button");
      expect(computeAccessibleName(button)).toBe(row.label);
      expect(button.getAttribute(fixture.chromePressedSemantics.stateAttribute)).toBe(String(row.pressed));
      expect(button.hasAttribute("aria-checked")).toBe(false);
    }
  });
  it.each(dockTabKeyboardFixture.cases)("matches the production React Mode roving Dock tab keyboard contract: $id", (row) => {
    vi.stubGlobal("ResizeObserver", class { observe(): void {} unobserve(): void {} disconnect(): void {} });
    vi.stubGlobal("MutationObserver", class { observe(): void {} disconnect(): void {} takeRecords(): MutationRecord[] { return []; } });
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);
    try {
      act(() => root.render(<DockTabKeyboardOracle key={row.id} />));
      const tabs = Array.from(container.querySelectorAll<HTMLButtonElement>('[role="tab"]'));
      const activeIds = new Set(dockTabKeyboardFixture.stacks.map((stack) => stack.active));
      expect(tabs.filter((tab) => tab.tabIndex === 0).map((tab) => tab.closest<HTMLElement>("[data-window-id]")?.dataset.windowId)).toEqual([...activeIds]);
      const from = tabs.find((tab) => tab.closest<HTMLElement>("[data-window-id]")?.dataset.windowId === row.from)!;
      act(() => from.focus());
      act(() => from.dispatchEvent(new KeyboardEvent("keydown", { key: row.key, bubbles: true, cancelable: true })));
      const focused = document.activeElement as HTMLButtonElement;
      expect(focused.closest<HTMLElement>("[data-window-id]")?.dataset.windowId).toBe(row.focus);
      if (row.activate === null) {
        expect(tabs.filter((tab) => tab.getAttribute("aria-selected") === "true").map((tab) => tab.closest<HTMLElement>("[data-window-id]")?.dataset.windowId)).toEqual([...activeIds]);
      } else {
        expect(container.querySelector(`[data-window-id="${row.activate}"] [role="tab"]`)?.getAttribute("aria-selected")).toBe("true");
      }
    } finally {
      act(() => root.unmount());
      vi.unstubAllGlobals();
    }
  }, 15_000);
  it("mirrors Dock tabs with one tab stop per stack and exact addressed keyboard routing", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const sent: WireEvent[] = [];
    const bubbled = vi.fn();
    root.addEventListener("keydown", bubbled);
    const mirror = createAccessibilityMirror(root, {
      introspect: async () => JSON.stringify({ windows: [dockTabKeyboardFixture.projection] }),
      enqueueLossless: (event) => { sent.push(event as WireEvent); return true; },
    }, "en");
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    const tabs = Array.from(root.querySelectorAll<HTMLElement>('[role="tab"]'));
    expect(tabs.filter((tab) => tab.tabIndex === 0).map((tab) => tab.dataset.nodeKey)).toEqual(
      dockTabKeyboardFixture.stacks.map((stack) => `dock.tab.${stack.id === "left" ? "0" : "1"}.${stack.active}`),
    );
    expect(tabs.filter((tab) => tab.tabIndex === -1)).toHaveLength(3);
    for (const row of dockTabKeyboardFixture.cases) {
      const stack = dockTabKeyboardFixture.stacks.find((candidate) => candidate.id === row.stack)!;
      const from = tabs.find((tab) => tab.dataset.nodeKey?.endsWith(`.${row.from}`))!;
      from.focus();
      sent.length = 0;
      const event = new KeyboardEvent("keydown", { key: row.key, bubbles: true, cancelable: true });
      expect(from.dispatchEvent(event)).toBe(false);
      const focused = document.activeElement as HTMLElement;
      expect(focused.dataset.nodeKey?.endsWith(`.${row.focus}`)).toBe(true);
      expect(focused.getAttribute("aria-controls")).toBe(tabs.find((tab) => tab.dataset.nodeKey?.endsWith(`.${stack.active}`))?.getAttribute("aria-controls"));
      expect(sent.filter((candidate) => candidate.kind === "accessibility-activate").map((candidate) => candidate.nodeKey)).toEqual(row.activate === null ? [] : [from.dataset.nodeKey]);
    }
    expect(bubbled).not.toHaveBeenCalled();
    mirror.dispose();
  });
  it("mirrors the shared TextEditor projection as the same named multiline textarea React exposes", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const row = fixture.textEditor;
    const projection: AccessibilityProjectionWindow = {
      windowId: "text-editor-window",
      windowGeneration: 1,
      nodes: [{ nodeId: 1, key: "scene.editor", role: row.role, depth: 0, label: row.label, live: "off", focusable: true, tabbable: true, actionable: true, editable: row.editable, multiline: row.multiline, valueText: row.value }],
    };
    const mirror = createAccessibilityMirror(root, { enqueueLossless: vi.fn(), introspect: vi.fn(async () => JSON.stringify({ windows: [projection] })) }, "en");
    mirror.refresh();
    await vi.runAllTimersAsync();
    const area = root.querySelector<HTMLTextAreaElement>("textarea")!;
    expect(area).toBeInstanceOf(HTMLTextAreaElement);
    expect(getRole(area)).toBe(row.role);
    expect(computeAccessibleName(area)).toBe(row.label);
    expect(area.value).toBe(row.value);
    expect(area.getAttribute("aria-multiline")).toBe(String(row.multiline));
    expect(area.getAttribute("aria-readonly")).toBe(String(row.readOnly));
    mirror.dispose();
  });
  it("selects the same root-to-leaf Settings path in the independent React oracle", () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);
    mounted.push(root);
    act(() => root.render(<NestedPanelOracle />));
    const tab = container.querySelector<HTMLButtonElement>('[role="tab"]')!;
    expect(tab.getAttribute("aria-selected")).toBe("false");
    act(() => tab.click());
    expect(tab.getAttribute("aria-selected")).toBe("true");
    expect(container.querySelector("section")?.dataset.path).toBe(fixture.nestedPanelSelection.expectedPath.join("/"));
    expect(container.querySelector(`[data-node-key="${fixture.nestedPanelSelection.requiredControlId}"]`)).not.toBeNull();
  });
  it.each([true, false])("keeps every constrained Settings child mounted and reveals a selected tail in the production React strip, showActiveColor=%s", (showActiveColor) => {
    const scrollIntoView = Element.prototype.scrollIntoView;
    Element.prototype.scrollIntoView = () => undefined;
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);
    mounted.push(root);
    act(() => root.render(<SettingsStripOracle showActiveColor={showActiveColor} />));
    const buttons = Array.from(container.querySelectorAll<HTMLButtonElement>('[data-slot="panel-tab-button"]'));
    expect(buttons.map((button) => button.id)).toEqual(fixture.settingsTabStrip.tabs.map((tab) => tab.id));
    for (const button of buttons) {
      expect(getRole(button)).toBe(fixture.settingsTabStrip.role);
      expect(button.getAttribute(fixture.settingsTabStrip.stateAttribute)).toBe(String(button.id === fixture.settingsTabStrip.activeId));
      expect(button.hasAttribute("aria-selected")).toBe(false);
      expect(button.hasAttribute("aria-checked")).toBe(false);
    }
    const tail = buttons.find((button) => button.id === fixture.settingsTabStrip.tailIds[1])!;
    act(() => tail.click());
    expect(tail.getAttribute("aria-pressed")).toBe("true");
    expect(container.querySelectorAll('[data-slot="panel-tab-button"]')).toHaveLength(6);
    Element.prototype.scrollIntoView = scrollIntoView;
  });
}
