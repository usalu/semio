// @vitest-environment jsdom

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import { wireBrowserKeyboard, wireBrowserFullscreen, type BrowserKeyboardEvent } from "../../🎯️targets/🧊️wgpu/🎮️input-wire/🟦️.ts";
import { createAccessibilityMirror, WGPU_ACCESSIBILITY_MIRROR_ID } from "../../🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts";
import fixture from "../../🧫️fixtures/⌨️browser-keyboard-scope/🔣️.json";

const cleanups: (() => void)[] = [];
Object.defineProperty(document, "fullscreenElement", { configurable: true, get: () => null });

function mount() {
  const root = document.createElement("main");
  const canvas = document.createElement("canvas");
  canvas.tabIndex = 0;
  const mirror = document.createElement("section");
  mirror.id = WGPU_ACCESSIBILITY_MIRROR_ID;
  const tree = document.createElement("div");
  tree.setAttribute("role", "treeitem");
  tree.tabIndex = 0;
  const button = document.createElement("button");
  button.type = "button";
  const input = document.createElement("input");
  const combobox = document.createElement("button");
  combobox.type = "button";
  combobox.setAttribute("role", "combobox");
  const editableCombobox = document.createElement("input");
  editableCombobox.setAttribute("role", "combobox");
  const number = document.createElement("input");
  number.type = "number";
  number.setAttribute("data-engine-keys", "ArrowUp ArrowDown PageUp PageDown Home Enter Escape");
  const outside = document.createElement("button");
  root.append(canvas, mirror);
  mirror.append(tree, button, input, combobox, editableCombobox, number);
  document.body.append(root, outside);
  const events: BrowserKeyboardEvent[] = [];
  cleanups.push(wireBrowserKeyboard(root, canvas, event => events.push(event)));
  return { root, canvas, mirror, events, targets: { canvas, "mirror-tree": tree, "mirror-button": button, "mirror-input": input, "mirror-combobox": combobox, "mirror-editable-combobox": editableCombobox, "mirror-number": number, outside } };
}

afterEach(() => {
  for (const cleanup of cleanups.splice(0)) cleanup();
  vi.useRealTimers();
  vi.restoreAllMocks();
  document.body.replaceChildren();
});

describe("browser keyboard scope", () => {
  

  for (const row of fixture.cases) it(row.id, () => {
    const { events, targets } = mount();
    const target = targets[row.target as keyof typeof targets];
    target.focus();
    for (const type of ["keydown", "keyup"]) {
      const event = new KeyboardEvent(type, { key: row.key, ctrlKey: row.modifiers.ctrl, metaKey: row.modifiers.meta, altKey: row.modifiers.alt, shiftKey: row.modifiers.shift, bubbles: true, cancelable: true });
      if (row.prevented) event.preventDefault();
      target.dispatchEvent(event);
    }
    expect(events).toHaveLength(row.forwarded);
    if (row.forwarded) expect(events.map(event => event.type)).toEqual(["keydown", "keyup"]);
  });

  it("keeps native button activation and Tab focus singular through the installed DOM oracle", async () => {
    const { events, targets } = mount();
    const clicks = vi.fn();
    targets["mirror-button"].addEventListener("click", clicks);
    const user = userEvent.setup();
    targets["mirror-button"].focus();
    await user.keyboard("{Enter} ");
    expect(clicks).toHaveBeenCalledTimes(2);
    expect(events).toEqual([]);
    await user.tab();
    expect(document.activeElement).toBe(targets["mirror-input"]);
    expect(events).toEqual([]);
    await user.keyboard("hello");
    expect(targets["mirror-input"].value).toBe("hello");
    expect(events).toEqual([]);
  });

  it("retains raw keyboard ownership after an actual accessibility refresh moves canvas focus", async () => {
    vi.useFakeTimers();
    const root = document.createElement("main");
    const canvas = document.createElement("canvas");
    canvas.tabIndex = 0;
    root.append(canvas);
    document.body.append(root);
    const events: BrowserKeyboardEvent[] = [];
    const projection = [{ windowId: "reopened-world", windowGeneration: 8, nodes: [{ nodeId: 3, key: "world", role: "treeitem", depth: 0, live: "off", focusable: true, tabbable: true, focused: true }] }];
    const mirror = createAccessibilityMirror(root, { enqueueLossless: () => true, introspect: async () => JSON.stringify({ windows: projection }) }, "en", canvas);
    cleanups.push(mirror.dispose, wireBrowserKeyboard(root, canvas, event => events.push(event)));
    canvas.focus();
    mirror.refresh();
    await vi.runAllTimersAsync();
    expect(document.activeElement?.getAttribute("data-node-key")).toBe("world");
    const target = document.activeElement!;
    target.dispatchEvent(new KeyboardEvent("keydown", { key: "f", ctrlKey: true, metaKey: true, bubbles: true, cancelable: true }));
    target.dispatchEvent(new KeyboardEvent("keyup", { key: "f", ctrlKey: true, metaKey: true, bubbles: true, cancelable: true }));
    expect(events).toHaveLength(2);
    for (const cleanup of cleanups.splice(0).reverse()) cleanup();
    const disposedCount = events.length;
    target.dispatchEvent(new KeyboardEvent("keydown", { key: "f", ctrlKey: true, metaKey: true, bubbles: true }));
    expect(events).toHaveLength(disposedCount);
  });

  for (const row of fixture.retirementCases) it(row.id, async () => {
    vi.useFakeTimers();
    const root = document.createElement("main");
    const canvas = document.createElement("canvas");
    canvas.tabIndex = 0;
    root.append(canvas);
    document.body.append(root);
    const events: BrowserKeyboardEvent[] = [];
    let projection = [{ windowId: "shell.chrome", windowGeneration: 1, nodes: [
      { nodeId: 1, key: "ui.search.dialog", role: "dialog", depth: 0, live: "off" },
      { nodeId: 2, key: row.retiredNodeKey, role: "combobox", depth: 1, live: "off", focusable: true, tabbable: true, focused: true, editable: true },
    ] }];
    const mirror = createAccessibilityMirror(root, { enqueueLossless: () => true, introspect: async () => JSON.stringify({ windows: projection }) }, "en", canvas);
    cleanups.push(mirror.dispose, wireBrowserKeyboard(root, canvas, event => events.push(event)));
    canvas.focus();
    mirror.refresh();
    await vi.runAllTimersAsync();
    expect((document.activeElement as HTMLElement).dataset.nodeKey).toBe(row.retiredNodeKey);
    projection = [];
    mirror.refresh();
    await vi.runAllTimersAsync();
    expect(document.activeElement).toBe(canvas);
    canvas.dispatchEvent(new KeyboardEvent("keydown", { key: row.reopenKey, metaKey: row.modifiers.meta, bubbles: true, cancelable: true }));
    canvas.dispatchEvent(new KeyboardEvent("keyup", { key: row.reopenKey, metaKey: row.modifiers.meta, bubbles: true, cancelable: true }));
    expect(events.slice(-2).map(event => [event.type, event.key, event.meta])).toEqual([["keydown", row.reopenKey, true], ["keyup", row.reopenKey, true]]);
  });

  it("projects an editable combobox with a resolved listbox and active descendant while keeping Select a button", async () => {
    vi.useFakeTimers();
    const root = document.createElement("main");
    const canvas = document.createElement("canvas");
    root.append(canvas);
    document.body.append(root);
    const projection = [{ windowId: "shell.chrome", windowGeneration: 1, nodes: [
      { nodeId: 1, key: "ui.search.dialog", role: "dialog", depth: 0, live: "off" },
      { nodeId: 2, key: "ui.search.input", role: "combobox", depth: 1, live: "off", focusable: true, tabbable: true, editable: true, controls: "ui.search.list", activeDescendant: "studio.undo", expanded: true, valueText: "ruck" },
      { nodeId: 3, key: "ui.search.list", role: "listbox", depth: 1, live: "off" },
      { nodeId: 4, key: "studio.undo", role: "option", depth: 2, live: "off", label: "Undo", selected: true },
      { nodeId: 5, key: "settings.theme.select", role: "combobox", depth: 0, live: "off", focusable: true, tabbable: true, valueText: "dark" },
    ] }];
    const mirror = createAccessibilityMirror(root, { enqueueLossless: () => true, introspect: async () => JSON.stringify({ windows: projection }) }, "en", canvas);
    cleanups.push(mirror.dispose);
    mirror.refresh();
    await vi.runAllTimersAsync();
    const editable = root.querySelector<HTMLElement>('[data-node-key="ui.search.input"]')!;
    const list = root.querySelector<HTMLElement>('[data-node-key="ui.search.list"]')!;
    const option = root.querySelector<HTMLElement>('[data-node-key="studio.undo"]')!;
    const select = root.querySelector<HTMLElement>('[data-node-key="settings.theme.select"]')!;
    expect(editable).toBeInstanceOf(HTMLInputElement);
    expect(editable.getAttribute("aria-controls")).toBe(list.id);
    expect(editable.getAttribute("aria-activedescendant")).toBe(option.id);
    expect(editable.getAttribute("aria-autocomplete")).toBe("list");
    expect(select).toBeInstanceOf(HTMLButtonElement);
  });

  for (const row of fixture.fullscreenCases) it(row.id, async () => {
    const { root, canvas, events, targets } = mount();
    const target = targets[row.focus as keyof typeof targets];
    let fullscreenElement: Element | null = null;
    vi.spyOn(document, "fullscreenElement", "get").mockImplementation(() => fullscreenElement);
    const transition = (element: Element | null) => {
      (document.activeElement as HTMLElement)?.blur();
      fullscreenElement = element;
      document.dispatchEvent(new Event("fullscreenchange"));
      return Promise.resolve();
    };
    root.requestFullscreen = vi.fn(() => transition(root));
    canvas.requestFullscreen = vi.fn(() => transition(canvas));
    document.exitFullscreen = vi.fn(() => transition(null));
    const owner = wireBrowserFullscreen(root, canvas);
    cleanups.push(owner.dispose);
    target.focus();
    await owner.set(true);
    expect(document.fullscreenElement).toBe(root);
    expect(document.activeElement).toBe(targets[row.expectedFocus as keyof typeof targets]);
    await userEvent.setup().keyboard("{Control>}{Meta>}f{/Meta}{/Control}");
    expect(events.some(event => event.type === "keydown" && event.key === "f" && event.ctrl && event.meta)).toBe(true);
    await owner.set(false);
    expect(document.fullscreenElement).toBeNull();
    expect(document.activeElement).toBe(targets[row.expectedFocus as keyof typeof targets]);
    const before = events.length;
    await userEvent.setup().keyboard("{Control>}{Meta>}f{/Meta}{/Control}");
    expect(events.slice(before).some(event => event.type === "keydown" && event.key === "f")).toBe(true);
    owner.dispose();
    targets.outside.focus();
    transition(root);
    expect(document.activeElement).toBe(document.body);
  });

  it("wires the production browser entry through the tested root keyboard owner", () => {
    const source = (readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), "utf8") + "\n" + readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts"), "utf8"));
    expect(source).toContain("wireBrowserKeyboard(root, canvas,");
    expect(source).not.toContain('canvas.addEventListener("keydown"');
    expect(source.replace(/\s+/gu, " ")).toContain('if (typeof fullscreen === "boolean") void fullscreenOwner.set(fullscreen)');
    expect(source).not.toContain("canvas.requestFullscreen()");
  });

  for (const row of fixture.focusLossCases) it(row.id, async () => {
    const { canvas, events, targets } = mount();
    const user = userEvent.setup();
    canvas.focus();
    await user.keyboard("{Control>}{Meta>}{Alt>}{Shift>}");
    const before = events.length;
    if (row.loss === "outside-focus") targets.outside.focus();
    if (row.loss === "window-blur") window.dispatchEvent(new Event("blur"));
    if (row.loss === "hidden-page") {
      vi.spyOn(document, "visibilityState", "get").mockReturnValue("hidden");
      document.dispatchEvent(new Event("visibilitychange"));
    }
    await Promise.resolve();
    const reset = events.slice(before);
    expect(reset.map(event => event.key).sort()).toEqual([...row.releasedKeys].sort());
    for (const event of reset) expect(event).toEqual({ type: "keyup", key: event.key, ...row.expectedModifiers });
    const after = events.length;
    window.dispatchEvent(new Event("blur"));
    document.dispatchEvent(new Event("visibilitychange"));
    expect(events).toHaveLength(after);
    vi.restoreAllMocks();
  });

  it("keeps held modifiers across mirror focus moves and removes loss listeners on disposal", async () => {
    const { canvas, events, targets } = mount();
    canvas.focus();
    await userEvent.setup().keyboard("{Shift>}");
    targets["mirror-tree"].focus();
    await Promise.resolve();
    expect(events).toHaveLength(1);
    cleanups.pop()!();
    const count = events.length;
    targets.outside.focus();
    window.dispatchEvent(new Event("blur"));
    document.dispatchEvent(new Event("visibilitychange"));
    await Promise.resolve();
    expect(events).toHaveLength(count);
  });

  it("hands a mirrored number control's law keys to the renderer after re-asserting its focus and leaves its typing native", async () => {
    vi.useFakeTimers();
    const root = document.createElement("main");
    const canvas = document.createElement("canvas");
    canvas.tabIndex = 0;
    root.append(canvas);
    document.body.append(root);
    const events: BrowserKeyboardEvent[] = [];
    const sent: { readonly kind: string; readonly nodeKey?: string; readonly value?: string }[] = [];
    const projection = [{ windowId: "history", windowGeneration: 3, nodes: [
      { nodeId: 1, key: "editor.index", role: "spinbutton", depth: 0, live: "off", label: "Index", description: "Must be at least 0", focusable: true, tabbable: true, actionable: true, invalid: true, valueMin: 0, valueNow: -1, valueText: "-1", valueStep: 1 },
      { nodeId: 2, key: "editor.angle", role: "slider", depth: 0, live: "off", label: "Angle", focusable: true, tabbable: true, actionable: true, valueMin: -180, valueMax: 180, valueNow: 90, valueText: "90 °", valueStep: 1 },
      { nodeId: 2, key: "editor.angle::editor", role: "spinbutton", depth: 0, live: "off", label: "Angle", focusable: true, actionable: true, editable: true, valueMin: -360, valueMax: 360, valueNow: 90, valueText: "90", valueStep: 1 },
      { nodeId: 3, key: "editor.orb", role: "slider", depth: 0, live: "off", label: "Orb", focusable: true, tabbable: true, actionable: true, valueMin: 0, valueMax: 1, valueNow: 0.5, valueText: "0.5" },
      { nodeId: 4, key: "history.row", role: "treeitem", depth: 0, live: "off", label: "Row 41", focusable: true, tabbable: true, setSize: 120, posInSet: 41, tone: "warning" },
    ] }];
    const mirror = createAccessibilityMirror(root, { enqueueLossless: (event) => { sent.push(event as { readonly kind: string }); return true; }, introspect: async () => JSON.stringify({ windows: projection }) }, "en", canvas);
    cleanups.push(mirror.dispose, wireBrowserKeyboard(root, canvas, event => events.push(event)));
    mirror.refresh();
    await vi.runAllTimersAsync();
    const node = (key: string) => root.querySelector<HTMLInputElement>(`[data-node-key="${key}"]`)!;
    const press = (target: HTMLElement, key: string) => {
      for (const type of ["keydown", "keyup"]) target.dispatchEvent(new KeyboardEvent(type, { key, bubbles: true, cancelable: true }));
    };
    const stepper = node("editor.index");
    expect([stepper.type, stepper.step, stepper.min, stepper.max, stepper.value, stepper.getAttribute("aria-invalid")]).toEqual(["number", "1", "0", "", "-1", "true"]);
    expect(stepper.getAttribute("data-engine-keys")).toBe("ArrowUp ArrowDown PageUp PageDown Home Enter Escape");
    const slider = node("editor.angle");
    expect([slider.type, slider.step, slider.min, slider.max, slider.value]).toEqual(["range", "1", "-180", "180", "90"]);
    expect(slider.getAttribute("data-engine-keys")).toBe("ArrowLeft ArrowRight ArrowUp ArrowDown PageUp PageDown Home End");
    const readout = node("editor.angle::editor");
    expect([readout.type, readout.tabIndex, readout.getAttribute("data-engine-keys")]).toEqual(["number", -1, "Enter Escape"]);
    const orb = node("editor.orb");
    expect([orb.step, orb.getAttribute("data-engine-keys")]).toEqual(["any", null]);
    const row = node("history.row");
    expect([row.getAttribute("aria-setsize"), row.getAttribute("aria-posinset"), row.dataset.tone]).toEqual(["120", "41", "warning"]);

    stepper.focus();
    sent.length = 0;
    press(stepper, "Home");
    expect(sent).toEqual([expect.objectContaining({ kind: "accessibility-focus", nodeKey: "editor.index" })]);
    expect(events.map(event => [event.type, event.key])).toEqual([["keydown", "Home"], ["keyup", "Home"]]);
    events.length = 0;
    press(stepper, "End");
    press(stepper, "7");
    expect(events).toEqual([]);
    readout.focus();
    press(readout, "ArrowUp");
    expect(events).toEqual([]);
    press(readout, "Enter");
    expect(events.map(event => [event.type, event.key])).toEqual([["keydown", "Enter"], ["keyup", "Enter"]]);
    readout.value = "-5";
    readout.dispatchEvent(new Event("input", { bubbles: true }));
    expect(sent.at(-1)).toEqual(expect.objectContaining({ kind: "accessibility-value", nodeKey: "editor.angle::editor", value: "-5" }));
  });
});
