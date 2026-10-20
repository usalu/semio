/** 🟠️ The activates actionable tree row tests of the wgpu accessibility interaction contract suite. */
import { act } from "react";
import { expect, it, vi } from "vitest";
import { createAccessibilityMirror } from "@semio-tech/framework-renderer-wgpu/accessibility-mirror";
import { address, computeAccessibleName, fixture, mountOracle } from "../🟣️shared/🟦️.tsx";
import type { WireEvent } from "../🟣️shared/🟦️.tsx";

/** 🟡 Registers the activates actionable tree row tests of the wgpu accessibility interaction contract suite. */
export function registerActivatesActionableTreeRowTests(): void {
  it("activates an actionable tree row by click, Enter and Space, and leaves a row without an activation alone", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const sent: WireEvent[] = [];
    const nodes = [
      { nodeId: 3, key: "actions", role: "tree", depth: 0, label: "Actions", live: "off" },
      { nodeId: 6, key: "action.selectAll", role: "treeitem", depth: 1, label: "Select All", live: "off", focusable: true, tabbable: true, actionable: true, selected: false },
      { nodeId: 7, key: "action.caption", role: "treeitem", depth: 1, label: "180 nodes", live: "off", focusable: true, tabbable: true, actionable: false, selected: false },
    ];
    const mirror = createAccessibilityMirror(root, {
      introspect: async () => JSON.stringify({ windows: [{ windowId: "pane", windowGeneration: 4, nodes }] }),
      enqueueLossless: (event) => { sent.push(event as WireEvent); return true; },
    }, "en");
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    const row = root.querySelector<HTMLElement>('[data-node-key="action.selectAll"]')!;
    const caption = root.querySelector<HTMLElement>('[data-node-key="action.caption"]')!;
    const activations = () => sent.filter((event) => event.kind === "accessibility-activate");
    const address = { kind: "accessibility-activate", windowId: "pane", windowGeneration: 4, nodeId: 6, nodeKey: "action.selectAll" };
    expect([row.dataset.actionable, caption.dataset.actionable]).toEqual(["true", undefined]);
    row.click();
    for (const key of ["Enter", " "]) {
      const event = new KeyboardEvent("keydown", { key, bubbles: true, cancelable: true });
      row.dispatchEvent(event);
      expect(event.defaultPrevented, `"${key}" is the row's own key`).toBe(true);
    }
    const chord = new KeyboardEvent("keydown", { key: "Enter", ctrlKey: true, bubbles: true, cancelable: true });
    row.dispatchEvent(chord);
    row.dispatchEvent(new KeyboardEvent("keydown", { key: "a", bubbles: true, cancelable: true }));
    expect(chord.defaultPrevented, "a chord stays the shell's").toBe(false);
    expect(activations(), "one activation per click, Enter and Space").toEqual([address, address, address]);
    caption.click();
    caption.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true, cancelable: true }));
    expect(activations(), "a row without an activation activates nothing").toHaveLength(3);
    mirror.dispose();
    root.remove();
  });
  it("spends one ownership walk and one projection pull on a storm of refresh calls", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const nodes = Array.from({ length: 40 }, (_, index) => ({ nodeId: index + 1, key: `row.${index}`, role: "treeitem", depth: 0, label: `Row ${index}`, live: "off", focusable: true, tabbable: true, actionable: false }));
    const introspect = vi.fn(async () => JSON.stringify({ windows: [{ windowId: "history", windowGeneration: 2, nodes }] }));
    const domOwns = vi.fn(() => false);
    const mirror = createAccessibilityMirror(root, { introspect, enqueueLossless: () => true }, "en", undefined, domOwns);
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    expect(root.querySelectorAll("[data-node-key]"), "the projection is mirrored").toHaveLength(nodes.length);
    for (let second = 0; second < 3; second += 1) {
      introspect.mockClear();
      domOwns.mockClear();
      for (let call = 0; call < 12_000; call += 1) mirror.refresh();
      expect(domOwns.mock.calls.length, "a burst of frame-step refreshes walks the mirror at most once").toBeLessThanOrEqual(nodes.length);
      expect(introspect, "and pulls nothing before the refresh floor").not.toHaveBeenCalled();
      await vi.advanceTimersByTimeAsync(400);
      expect(introspect, "one pull per refresh floor").toHaveBeenCalledTimes(1);
      expect(domOwns.mock.calls.length, "one walk when the burst starts and one when its pull starts").toBeLessThanOrEqual(2 * nodes.length);
    }
    mirror.dispose();
    root.remove();
  });
  it("keeps the real focus then activation address across a focus-only mirror refresh", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const sent: WireEvent[] = [];
    const control = fixture.presentedChrome.controls[1]!;
    const node = { nodeId: 1, key: control.id, role: "button", depth: 0, label: control.label, live: "off", focusable: true, tabbable: true, actionable: true, focused: false };
    const address = { windowId: "shell.chrome", windowGeneration: fixture.presentedChrome.firstEpoch, nodeId: node.nodeId, nodeKey: node.key };
    const mirror = createAccessibilityMirror(root, {
      introspect: async () => JSON.stringify({ windows: [{ ...address, nodes: [node] }] }),
      enqueueLossless: event => { sent.push(event as WireEvent); return true; },
    }, "en");
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    root.querySelector<HTMLButtonElement>("button")!.focus();
    node.focused = true;
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    const button = root.querySelector<HTMLButtonElement>("button")!;
    expect(computeAccessibleName(button)).toBe(control.label);
    expect(document.activeElement).toBe(button);
    button.click();
    expect(fixture.presentedChrome.generationRule.focusOnlyKeepsGeneration).toBe(true);
    expect(sent).toEqual([{ kind: "accessibility-focus", ...address }, { kind: "accessibility-activate", ...address }]);
    mirror.dispose();
  });
  it("transports a real editable blur after the final staged accessibility value", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const sent: WireEvent[] = [];
    const nodes = [
      { nodeId: 21, key: "driver.saveLabel", role: "textbox", depth: 0, label: "Save label", live: "off", focusable: true, tabbable: true, valueText: "" },
      { nodeId: 22, key: "driver.save", role: "button", depth: 0, label: "Save", live: "off", focusable: true, tabbable: true, actionable: true },
    ];
    const mirror = createAccessibilityMirror(root, {
      introspect: async () => JSON.stringify({ windows: [{ windowId: "settings", windowGeneration: 9, nodes }] }),
      enqueueLossless: (event) => { sent.push(event as WireEvent); return true; },
    }, "en");
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    const input = root.querySelector<HTMLInputElement>('[data-node-key="driver.saveLabel"]')!;
    const save = root.querySelector<HTMLButtonElement>('[data-node-key="driver.save"]')!;
    input.focus();
    input.value = "Focus Flow";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    save.focus();
    expect(sent.map((event) => event.kind)).toEqual(["accessibility-focus", "accessibility-value", "accessibility-blur", "accessibility-focus"]);
    expect(sent[1]).toMatchObject({ kind: "accessibility-value", value: "Focus Flow" });
    mirror.dispose();
  });
  it("pins bounded node-addressed events with no pointer-coordinate fallback", () => {
    expect(fixture.schema).toBe("semio.renderer.wgpu.accessibility-interaction.v1");
    expect(fixture.limits).toEqual({ windowIdBytes: 512, nodeKeyBytes: 512, valueCodeUnits: 1024, losslessItems: 64 });
    expect(fixture.events.map((row) => row.wire.kind)).toEqual(["accessibility-focus", "accessibility-activate", "accessibility-focus", "accessibility-value"]);
    for (const row of [...fixture.events, ...fixture.rejections]) {
      expect(row.wire).toMatchObject({ windowId: expect.any(String), windowGeneration: expect.any(Number), nodeId: expect.any(Number), nodeKey: expect.any(String) });
      expect(row.wire).not.toHaveProperty("x");
      expect(row.wire).not.toHaveProperty("y");
    }
  });
  it("mounts the neutral hierarchy as native roles and states", () => {
    const { container } = mountOracle();
    for (const row of fixture.hierarchy) {
      const parent = container.querySelector<HTMLElement>(`[data-node-key="${row.nodeKey}"]`);
      expect(parent, row.nodeKey).not.toBeNull();
      const direct = Array.from(parent!.children).filter((child): child is HTMLElement => child instanceof HTMLElement).map((child) => child.dataset.nodeKey);
      expect(direct).toEqual(row.children);
    }
    expect(container.querySelector<HTMLElement>('[data-node-key="#apply"]')?.tagName).toBe("BUTTON");
    expect(container.querySelector<HTMLElement>('[data-node-key="#apply"]')?.tabIndex).toBe(0);
    expect(container.querySelector<HTMLElement>('[data-node-key="#width"]')?.tagName).toBe("INPUT");
    expect(container.querySelector<HTMLElement>('[data-node-key="#width"]')?.getAttribute("role")).toBeNull();
    expect(container.querySelector<HTMLElement>('[data-node-key="#enabled"]')?.getAttribute("aria-checked")).toBe("true");
    expect(container.querySelector<HTMLElement>('[data-node-key="#feature-a"]')?.getAttribute("aria-selected")).toBe("true");
    expect(container.querySelector<HTMLElement>('[data-node-key="#feature-a"]')?.getAttribute("aria-level")).toBe("1");
  });
  it("emits focus activate and text value exactly once and reflects focus", () => {
    const { container, sent } = mountOracle();
    const button = container.querySelector<HTMLButtonElement>('[data-node-key="#apply"]')!;
    const input = container.querySelector<HTMLInputElement>('[data-node-key="#width"]')!;
    act(() => button.focus());
    act(() => button.click());
    act(() => input.focus());
    act(() => {
      input.value = "42";
      input.dispatchEvent(new Event("input", { bubbles: true }));
    });
    expect(sent).toEqual(fixture.events.map((row) => row.wire));
    expect(sent.filter((event) => event.kind === "accessibility-activate")).toHaveLength(fixture.exactOnce.mirrorActivateEvents);
    expect(container.querySelector<HTMLElement>('[data-node-key="#width"]')?.dataset.focused).toBe("true");
    expect(container.querySelector<HTMLElement>('[data-node-key="#apply"]')?.dataset.focused).toBeUndefined();
  });
}
