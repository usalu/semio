// @vitest-environment jsdom

import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import React, { act, useState } from "react";
import { createRoot, type Root } from "react-dom/client";
import Ajv2020 from "ajv/dist/2020.js";
import { afterEach, describe, expect, it, vi } from "vitest";
import type * as AccessibilityOracle from "dom-accessibility-api" with { "resolution-mode": "require" };
import { createAccessibilityMirror, type AccessibilityProjectionWindow } from "../../🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts";
import { BrowserFrameTransport, type BrowserFrameUiMessage, type BrowserFrameWorkerMessage, type BrowserFrameWorkerPort } from "../../🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts";
import { resolveWgpuBootDescriptor } from "../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts";
import { PanelTabBar, type PanelTabNode } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx";
import accessibilityVisibilityFixture from "../../🧫️fixtures/♿️wgpu-accessibility-visibility/🔣️.json";
import accessibilityVisibilitySchema from "../../🧬️schema/♿️wgpu-accessibility-visibility/🔣️.json";
import accessibilityInteractionSchema from "../../🧬️schema/♿️wgpu-accessibility-interaction/🔣️.json";

type ProjectionNode = {
  readonly nodeId: number;
  readonly nodeKey: string;
  readonly depth: number;
  readonly role: string;
  readonly label: string;
  readonly focusable: boolean;
  readonly actionable: boolean;
  readonly focused: boolean;
  readonly disabled?: boolean;
  readonly value?: string;
  readonly checked?: boolean;
  readonly selected?: boolean;
  readonly expanded?: boolean;
  readonly level?: number;
};

type Address = { readonly windowId: string; readonly windowGeneration: number; readonly nodeId: number; readonly nodeKey: string };
type WireEvent = Address & { readonly kind: "accessibility-focus" | "accessibility-blur" | "accessibility-activate" | "accessibility-value"; readonly value?: string };
type Fixture = {
  readonly schema: string;
  readonly limits: { readonly windowIdBytes: number; readonly nodeKeyBytes: number; readonly valueCodeUnits: number; readonly losslessItems: number };
  readonly projection: { readonly windowId: string; readonly windowGeneration: number; readonly nodes: readonly ProjectionNode[] };
  readonly mirrorProjection: AccessibilityProjectionWindow;
  readonly nestedPanelSelection: {
    readonly rootId: string;
    readonly appFirstLeafId: string;
    readonly targetLeafId: string;
    readonly expectedPath: readonly string[];
    readonly requiredControlId: string;
    readonly pointerRects: readonly { readonly id: string; readonly x: number; readonly y: number; readonly width: number; readonly height: number }[];
  };
  readonly presentedChrome: { readonly firstEpoch: number; readonly successorEpoch: number; readonly controls: readonly { readonly id: string; readonly label: string }[]; readonly generationRule: { readonly controlId: string; readonly firstRect: readonly number[]; readonly sameChromeRect: readonly number[]; readonly changedChromeRect: readonly number[] } };
  readonly settingsTabStrip: { readonly availableWidth: number; readonly activeId: string; readonly tabs: readonly { readonly id: string; readonly label: string }[]; readonly tailIds: readonly string[] };
  readonly events: readonly { readonly id: string; readonly dom: { readonly type: string; readonly value?: string }; readonly wire: WireEvent; readonly expected: { readonly accepted: boolean; readonly focusedNodeId: number; readonly actions: number; readonly value?: string } }[];
  readonly rejections: readonly { readonly id: string; readonly wire: WireEvent; readonly current: (Pick<Address, "windowGeneration" | "nodeId" | "nodeKey"> & Partial<Pick<Address, "windowId">>) | null; readonly reason: string }[];
  readonly hierarchy: readonly { readonly nodeKey: string; readonly children: readonly string[] }[];
  readonly exactOnce: { readonly mirrorActivateEvents: number; readonly retainedActivateActions: number; readonly canvasKeyEvents: number; readonly staleActivateActions: number };
};

type OracleTreeNode = { readonly node: ProjectionNode; readonly children: OracleTreeNode[] };

const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json"), "utf8")) as Fixture;
const { computeAccessibleDescription, computeAccessibleName }: typeof AccessibilityOracle = createRequire(import.meta.url)("dom-accessibility-api");
const mounted: Root[] = [];

class AccessibilityWorker implements BrowserFrameWorkerPort {
  onmessage: ((event: MessageEvent<BrowserFrameWorkerMessage>) => void) | null = null;
  onmessageerror: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  readonly messages: BrowserFrameUiMessage[] = [];
  postMessage(message: BrowserFrameUiMessage): void { this.messages.push(message); }
  terminate(): void {}
  reply(message: BrowserFrameWorkerMessage): void { this.onmessage?.({ data: message } as MessageEvent<BrowserFrameWorkerMessage>); }
}

const FixtureIcon: React.FC<{ readonly size?: number }> = () => <span aria-hidden />;

function SettingsStripOracle(): React.JSX.Element {
  const [activePath, setActivePath] = useState<readonly string[]>([fixture.settingsTabStrip.activeId]);
  const tabs: readonly PanelTabNode[] = fixture.settingsTabStrip.tabs.map((tab, order) => ({ kind: "leaf", id: tab.id, name: tab.label, icon: FixtureIcon, order, trees: [] }));
  return <div style={{ width: fixture.settingsTabStrip.availableWidth }}><PanelTabBar variant="chrome" tabs={tabs} activePath={activePath} onActivePathChange={setActivePath} maxRows={1} /></div>;
}

function projectionTree(nodes: readonly ProjectionNode[]): OracleTreeNode[] {
  const roots: OracleTreeNode[] = [];
  const stack: OracleTreeNode[] = [];
  for (const node of nodes) {
    const entry: OracleTreeNode = { node, children: [] };
    while (stack.length > node.depth) stack.pop();
    const parent = stack.at(-1);
    if (parent) parent.children.push(entry);
    else roots.push(entry);
    stack.push(entry);
  }
  return roots;
}

function address(node: ProjectionNode): Address {
  return { windowId: fixture.projection.windowId, windowGeneration: fixture.projection.windowGeneration, nodeId: node.nodeId, nodeKey: node.nodeKey };
}

function OracleNode({ entry, focused, send }: { readonly entry: OracleTreeNode; readonly focused: number | null; readonly send: (event: WireEvent) => void }): React.JSX.Element {
  const node = entry.node;
  const children = entry.children.map((child) => <OracleNode key={child.node.nodeKey} entry={child} focused={focused} send={send} />);
  const common = {
    "aria-label": node.label,
    "data-window": fixture.projection.windowId,
    "data-window-generation": String(fixture.projection.windowGeneration),
    "data-node-id": String(node.nodeId),
    "data-node-key": node.nodeKey,
    "data-focused": focused === node.nodeId ? "true" : undefined,
    ...(node.focusable ? { onFocus: () => send({ kind: "accessibility-focus", ...address(node) }) } : {}),
  };
  if (node.role === "form") return <form {...common}>{children}</form>;
  if (node.role === "button") return <button {...common} type="button" disabled={node.disabled} onClick={() => send({ kind: "accessibility-activate", ...address(node) })}>{node.label}{children}</button>;
  if (node.role === "textbox") return <input {...common} type="text" defaultValue={node.value} disabled={node.disabled} onInput={(event) => send({ kind: "accessibility-value", ...address(node), value: event.currentTarget.value })} />;
  if (node.role === "switch") return <button {...common} type="button" role="switch" aria-checked={node.checked} disabled={node.disabled} onClick={() => send({ kind: "accessibility-activate", ...address(node) })}>{node.label}{children}</button>;
  if (node.role === "treeitem") return <div {...common} role="treeitem" tabIndex={0} aria-selected={node.selected} aria-expanded={node.expanded} aria-level={node.level}>{node.label}{children}</div>;
  return <div {...common} role={node.role}>{children}</div>;
}

function OracleMirror({ sent }: { readonly sent: WireEvent[] }): React.JSX.Element {
  const [focused, setFocused] = useState<number | null>(null);
  const send = (event: WireEvent): void => {
    sent.push(event);
    if (event.kind === "accessibility-focus") setFocused(event.nodeId);
  };
  return <section aria-label="Semio controls">{projectionTree(fixture.projection.nodes).map((entry) => <OracleNode key={entry.node.nodeKey} entry={entry} focused={focused} send={send} />)}</section>;
}

function NestedPanelOracle(): React.JSX.Element {
  const selection = fixture.nestedPanelSelection;
  const [path, setPath] = useState<readonly string[]>([selection.rootId, selection.appFirstLeafId]);
  const active = path.at(-1);
  return <section data-path={path.join("/")}>
    <button type="button" role="tab" aria-selected={active === selection.targetLeafId} onClick={() => setPath(selection.expectedPath)}>{selection.targetLeafId}</button>
    {active === selection.targetLeafId ? <select aria-label="Appearance" data-node-key={selection.requiredControlId}><option>System</option></select> : null}
  </section>;
}

function mountOracle(): { readonly container: HTMLDivElement; readonly canvas: HTMLCanvasElement; readonly sent: WireEvent[] } {
  const container = document.createElement("div");
  const canvas = document.createElement("canvas");
  document.body.append(container, canvas);
  const root = createRoot(container);
  mounted.push(root);
  const sent: WireEvent[] = [];
  act(() => root.render(<OracleMirror sent={sent} />));
  return { container, canvas, sent };
}

function staleReason(event: WireEvent): string | null {
  if (event.windowId !== fixture.projection.windowId) return "window-not-live";
  if (event.windowGeneration !== fixture.projection.windowGeneration) return "stale-window-generation";
  const node = fixture.projection.nodes.find((candidate) => candidate.nodeId === event.nodeId);
  if (!node || node.nodeKey !== event.nodeKey) return "stale-node-identity";
  return null;
}

afterEach(() => {
  vi.useRealTimers();
  while (mounted.length) act(() => mounted.pop()?.unmount());
  document.body.replaceChildren();
});

describe("wgpu accessibility interaction contract", () => {
  it("validates the accepted chrome epoch and bounded Settings strip grammar", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(accessibilityInteractionSchema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.presentedChrome.successorEpoch).toBeGreaterThan(fixture.presentedChrome.firstEpoch);
    const rule = fixture.presentedChrome.generationRule;
    expect(fixture.presentedChrome.controls.map((control) => control.id)).toContain(rule.controlId);
    expect(rule.sameChromeRect).toEqual(rule.firstRect);
    expect(rule.changedChromeRect).not.toEqual(rule.firstRect);
    expect(new Set(fixture.settingsTabStrip.tabs.map((tab) => tab.id)).size).toBe(fixture.settingsTabStrip.tabs.length);
  });

  it("validates the visible-document publication grammar", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(accessibilityVisibilitySchema);
    expect(validate(accessibilityVisibilityFixture), JSON.stringify(validate.errors)).toBe(true);
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

  it("keeps every constrained Settings child mounted and reveals a selected tail in the production React strip", () => {
    const scrollIntoView = Element.prototype.scrollIntoView;
    Element.prototype.scrollIntoView = () => undefined;
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);
    mounted.push(root);
    act(() => root.render(<SettingsStripOracle />));
    const buttons = Array.from(container.querySelectorAll<HTMLButtonElement>('[data-slot="panel-tab-button"]'));
    expect(buttons.map((button) => button.id)).toEqual(fixture.settingsTabStrip.tabs.map((tab) => tab.id));
    const tail = buttons.find((button) => button.id === fixture.settingsTabStrip.tailIds[1])!;
    act(() => tail.click());
    expect(tail.getAttribute("aria-pressed")).toBe("true");
    expect(container.querySelectorAll('[data-slot="panel-tab-button"]')).toHaveLength(6);
    Element.prototype.scrollIntoView = scrollIntoView;
  });

  it("carries one mirror activation through the real transport and replaces it with the next accepted epoch", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const worker = new AccessibilityWorker();
    const frameTransport = new BrowserFrameTransport({
      worker,
      boot: {
        bindingsModuleUrl: "renderer.js",
        bindingsWasmUrl: "renderer_bg.wasm",
        canvas: {} as OffscreenCanvas,
        width: 800,
        height: 600,
        dpr: 1,
        locale: "en",
        descriptor: resolveWgpuBootDescriptor({ defaultVariant: "s" }),
        appearance: { preference: "", systemDark: false },
        platform: "MacIntel",
        storage: {},
      },
      setTimer: () => 1,
      clearTimer: () => {},
    });
    worker.reply({ kind: "booted", lifecycle: 1 });
    const chrome = fixture.presentedChrome;
    let generation = chrome.firstEpoch;
    let displayChecked = false;
    const projection = (): AccessibilityProjectionWindow => ({
      windowId: "shell.chrome",
      windowGeneration: generation,
      nodes: chrome.controls.map((control, index) => ({ nodeId: index + 1, key: control.id, role: "switch", depth: 0, label: control.label, live: "off", focusable: true, actionable: true, checked: control.id === "ui.panelToggle.display" && displayChecked })),
    });
    const mirror = createAccessibilityMirror(root, {
      introspect: async () => JSON.stringify({ windows: [projection()] }),
      enqueueLossless: (event) => frameTransport.enqueueLossless(event),
    }, "en");
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    const staleDisplay = root.querySelector<HTMLButtonElement>('[data-node-key="ui.panelToggle.display"]')!;
    staleDisplay.click();
    expect(frameTransport.flush(1)).toBe(true);
    const batch = worker.messages.at(-1);
    if (batch?.kind !== "batch") throw new Error("accessibility activation batch");
    expect(batch.lossless).toEqual([expect.objectContaining({ kind: "accessibility-activate", windowGeneration: chrome.firstEpoch, nodeKey: "ui.panelToggle.display" })]);
    worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: batch.inputSequence, generation: batch.generation });
    displayChecked = true;
    generation = chrome.successorEpoch;
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    expect(root.querySelector('[data-node-key="ui.panelToggle.display"]')?.getAttribute("aria-checked")).toBe("true");
    root.querySelector<HTMLButtonElement>('[data-node-key="ui.panelToggle.settings"]')!.click();
    expect(frameTransport.flush(2)).toBe(true);
    const successor = worker.messages.at(-1);
    if (successor?.kind !== "batch") throw new Error("successor accessibility activation batch");
    expect(successor.lossless).toEqual([expect.objectContaining({ kind: "accessibility-activate", windowGeneration: chrome.successorEpoch, nodeKey: "ui.panelToggle.settings" })]);
    staleDisplay.click();
    worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: successor.inputSequence, generation: successor.generation });
    expect(frameTransport.flush(3)).toBe(true);
    const stale = worker.messages.at(-1);
    if (stale?.kind !== "batch") throw new Error("stale accessibility activation batch");
    expect(stale.lossless).toEqual([expect.objectContaining({ kind: "accessibility-activate", windowGeneration: chrome.firstEpoch })]);
    mirror.dispose();
    frameTransport.close();
  });

  it("mounts the production mirror with sibling descriptions, semantic states, and paragraph text", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const sent: WireEvent[] = [];
    let introspections = 0;
    const json = JSON.stringify({ windows: [fixture.mirrorProjection] });
    const mirror = createAccessibilityMirror(root, {
      introspect: async () => { introspections++; return json; },
      enqueueLossless: (event) => { sent.push(event as WireEvent); return true; },
    }, "en");
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    const input = root.querySelector<HTMLInputElement>('[data-node-key="settings.iterations"]')!;
    const description = document.getElementById(input.getAttribute("aria-describedby")!)!;
    expect(input.tagName).toBe("INPUT");
    expect(description.parentElement).toBe(input.parentElement);
    expect(input.contains(description)).toBe(false);
    expect(computeAccessibleName(input)).toBe("Iterations");
    expect(computeAccessibleDescription(input)).toBe("Whole-number iterations");
    expect(root.querySelector<HTMLElement>('[data-node-key="settings.status"]')?.textContent).toBe("Ready");
    expect(root.querySelector<HTMLElement>('[data-node-key="framework.settings"]')?.getAttribute("aria-checked")).toBe("true");
    expect(root.querySelector<HTMLElement>('[data-node-key="framework.settings.general"]')?.getAttribute("aria-selected")).toBe("true");
    expect(document.activeElement?.getAttribute("data-node-key")).toBe("framework.settings.general");
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    expect(introspections).toBe(2);
    expect(root.querySelectorAll("[data-node-key]")).toHaveLength(fixture.mirrorProjection.nodes.length);
    expect(sent).toEqual([]);
    mirror.dispose();
  });

  it("mounts an open retained Select as a read-only combobox with live listbox options", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const sent: WireEvent[] = [];
    const nodes = [
      { nodeId: 11, key: "appearance", role: "combobox", depth: 0, label: "Appearance", live: "off", focusable: true, actionable: true, expanded: true, valueText: "system" },
      { nodeId: 11, key: "appearance::listbox", role: "listbox", depth: 0, label: "Appearance", live: "off" },
      { nodeId: 11, key: "appearance::option::system", role: "option", depth: 1, label: "System", live: "off", actionable: true, selected: true },
      { nodeId: 11, key: "appearance::option::dark", role: "option", depth: 1, label: "Dark", live: "off", actionable: true, selected: false },
    ];
    const mirror = createAccessibilityMirror(root, {
      introspect: async () => JSON.stringify({ windows: [{ windowId: "settings", windowGeneration: 4, nodes }] }),
      enqueueLossless: (event) => { sent.push(event as WireEvent); return true; },
    }, "en");
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    const combobox = root.querySelector<HTMLElement>('[role="combobox"]')!;
    const listbox = root.querySelector<HTMLElement>('[role="listbox"]')!;
    const options = Array.from(root.querySelectorAll<HTMLElement>('[role="option"]'));
    expect(combobox.tagName).toBe("BUTTON");
    expect(combobox).not.toBeInstanceOf(HTMLInputElement);
    expect(computeAccessibleName(combobox)).toBe("Appearance");
    expect(listbox.parentElement).toBe(combobox.parentElement);
    expect(options.every((option) => option.parentElement === listbox)).toBe(true);
    options[1]!.click();
    expect(sent.filter((event) => event.kind === "accessibility-activate")).toEqual([{ kind: "accessibility-activate", windowId: "settings", windowGeneration: 4, nodeId: 11, nodeKey: "appearance::option::dark" }]);
    mirror.dispose();
  });

  it("transports a real editable blur after the final staged accessibility value", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const sent: WireEvent[] = [];
    const nodes = [
      { nodeId: 21, key: "driver.saveLabel", role: "textbox", depth: 0, label: "Save label", live: "off", focusable: true, valueText: "" },
      { nodeId: 22, key: "driver.save", role: "button", depth: 0, label: "Save", live: "off", focusable: true, actionable: true },
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

  it("keeps canvas keyboard delivery independent from mirror activation", () => {
    const { container, canvas, sent } = mountOracle();
    let canvasKeys = 0;
    canvas.addEventListener("keydown", () => canvasKeys++);
    const button = container.querySelector<HTMLButtonElement>('[data-node-key="#apply"]')!;
    act(() => button.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true })));
    expect(canvasKeys).toBe(fixture.exactOnce.canvasKeyEvents);
    expect(sent.filter((event) => event.kind === "accessibility-activate")).toHaveLength(0);
    act(() => button.click());
    expect(sent.filter((event) => event.kind === "accessibility-activate")).toHaveLength(fixture.exactOnce.retainedActivateActions);
    act(() => canvas.dispatchEvent(new KeyboardEvent("keydown", { key: "f", bubbles: true })));
    expect(canvasKeys).toBe(1);
    expect(sent.filter((event) => event.kind === "accessibility-activate")).toHaveLength(1);
  });

  it("rejects retired windows generations and reused node ids before dispatch", () => {
    for (const row of fixture.rejections) expect(staleReason(row.wire), row.id).toBe(row.reason);
    for (const row of fixture.events) expect(staleReason(row.wire), row.id).toBeNull();
    expect(fixture.exactOnce.staleActivateActions).toBe(0);
  });
});
