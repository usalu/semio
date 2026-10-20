/** 🟣️ The shared declarations of the wgpu accessibility interaction contract suite tests. */
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import React, { act, useState } from "react";
import { createRoot, type Root } from "react-dom/client";
import { type AccessibilityProjectionWindow } from "@semio-tech/framework-renderer-wgpu/accessibility-mirror";
import { type BrowserFrameUiMessage, type BrowserFrameWorkerMessage, type BrowserFrameWorkerPort } from "@semio-tech/framework-renderer-wgpu/browser-frame-transport/contract";
import type * as AccessibilityOracle from "dom-accessibility-api" with { "resolution-mode": "require" };
import { PanelTabBar, type PanelTabNode } from "@semio-tech/ui-react";
import { Toggle } from "@semio-tech/ui-react";
import { Mode, uiDataLabel, type WindowLayoutNode } from "@semio-tech/ui-react";

export type ProjectionNode = {
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

export type Address = { readonly windowId: string; readonly windowGeneration: number; readonly nodeId: number; readonly nodeKey: string };

export type WireEvent = Address & { readonly kind: "accessibility-focus" | "accessibility-blur" | "accessibility-activate" | "accessibility-value"; readonly value?: string };

export type Fixture = {
  readonly passiveChrome: readonly { readonly kind: string; readonly id: string; readonly element: "div" | "button"; readonly label: string | null; readonly action: boolean; readonly announced: boolean }[];
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
  readonly presentedChrome: { readonly firstEpoch: number; readonly successorEpoch: number; readonly controls: readonly { readonly id: string; readonly label: string }[]; readonly generationRule: { readonly controlId: string; readonly focusOnlyKeepsGeneration: boolean; readonly firstRect: readonly number[]; readonly sameChromeRect: readonly number[]; readonly changedChromeRect: readonly number[] } };
  readonly settingsTabStrip: { readonly role: string; readonly stateAttribute: string; readonly availableWidth: number; readonly activeId: string; readonly tabs: readonly { readonly id: string; readonly label: string }[]; readonly tailIds: readonly string[] };
  readonly chromePressedSemantics: {
    readonly stateAttribute: string;
    readonly fullscreen: { readonly id: string; readonly label: string; readonly pressed: boolean };
    readonly utility: {
      readonly button: { readonly id: string; readonly label: string };
      readonly toggle: { readonly id: string; readonly label: string; readonly pressed: boolean };
      readonly collection: { readonly id: string; readonly label: string };
    };
  };
  readonly textEditor: { readonly role: "textbox"; readonly label: string; readonly value: string; readonly multiline: true; readonly editable: true; readonly readOnly: false };
  readonly events: readonly { readonly id: string; readonly dom: { readonly type: string; readonly value?: string }; readonly wire: WireEvent; readonly expected: { readonly accepted: boolean; readonly focusedNodeId: number; readonly actions: number; readonly value?: string } }[];
  readonly rejections: readonly { readonly id: string; readonly wire: WireEvent; readonly current: (Pick<Address, "windowGeneration" | "nodeId" | "nodeKey"> & Partial<Pick<Address, "windowId">>) | null; readonly reason: string }[];
  readonly hierarchy: readonly { readonly nodeKey: string; readonly children: readonly string[] }[];
  readonly exactOnce: { readonly mirrorActivateEvents: number; readonly retainedActivateActions: number; readonly canvasKeyEvents: number; readonly staleActivateActions: number };
};

export type OracleTreeNode = { readonly node: ProjectionNode; readonly children: OracleTreeNode[] };

export const fixture = JSON.parse(readFileSync(join(dirname(dirname(fileURLToPath(import.meta.url))), "../🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json"), "utf8")) as Fixture;

export type DockTabKeyboardFixture = {
  readonly projection: AccessibilityProjectionWindow & { readonly nodes: readonly (AccessibilityProjectionWindow["nodes"][number] & { readonly tabbable: boolean; readonly controls?: string; readonly selected?: boolean })[] };
  readonly stacks: readonly { readonly id: string; readonly panel: string; readonly active: string; readonly tabs: readonly string[] }[];
  readonly cases: readonly { readonly id: string; readonly stack: string; readonly from: string; readonly key: "ArrowLeft" | "ArrowRight" | "Home" | "End" | "Enter" | " "; readonly focus: string; readonly activate: string | null }[];
  readonly stale: { readonly node: string; readonly generation: number };
};

export const dockTabKeyboardFixture = JSON.parse(readFileSync(join(dirname(dirname(fileURLToPath(import.meta.url))), "../🧫️fixtures/⌨️dock-tab-keyboard/🔣️.json"), "utf8")) as DockTabKeyboardFixture;

export const mounted: Root[] = [];

export class AccessibilityWorker implements BrowserFrameWorkerPort {
  onmessage: ((event: MessageEvent<BrowserFrameWorkerMessage>) => void) | null = null;
  onmessageerror: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  readonly messages: BrowserFrameUiMessage[] = [];
  postMessage(message: BrowserFrameUiMessage): void { this.messages.push(message); }
  terminate(): void {}
  reply(message: BrowserFrameWorkerMessage): void { this.onmessage?.({ data: message } as MessageEvent<BrowserFrameWorkerMessage>); }
}

export const FixtureIcon: React.FC<{ readonly size?: number }> = () => <span aria-hidden />;

export function SettingsStripOracle({ showActiveColor = true }: { readonly showActiveColor?: boolean }): React.JSX.Element {
  const [activePath, setActivePath] = useState<readonly string[]>([fixture.settingsTabStrip.activeId]);
  const tabs: readonly PanelTabNode[] = fixture.settingsTabStrip.tabs.map((tab, order) => ({ kind: "leaf", id: tab.id, name: tab.label, icon: FixtureIcon, order, trees: [] }));
  return <div style={{ width: fixture.settingsTabStrip.availableWidth }}><PanelTabBar variant="chrome" tabs={tabs} activePath={activePath} onActivePathChange={setActivePath} maxRows={1} showActiveColor={showActiveColor} /></div>;
}

export function PressedChromeOracle(): React.JSX.Element {
  const semantics = fixture.chromePressedSemantics;
  return <>
    <Toggle id={semantics.fullscreen.id} text={semantics.fullscreen.label} pressed={semantics.fullscreen.pressed} icon={<FixtureIcon />} />
    <Toggle id={semantics.utility.toggle.id} text={semantics.utility.toggle.label} pressed={semantics.utility.toggle.pressed} icon={<FixtureIcon />} />
  </>;
}

export function DockTabKeyboardOracle(): React.JSX.Element {
  const [active, setActive] = useState(dockTabKeyboardFixture.stacks[0]!.active);
  const windows = dockTabKeyboardFixture.stacks.flatMap((stack) => stack.tabs.map((id) => ({ id, title: uiDataLabel(id.toUpperCase()), iconId: "app-window" as const, children: <div>{id}</div> })));
  const layout: WindowLayoutNode = {
    kind: "row",
    children: dockTabKeyboardFixture.stacks.map((stack) => ({ kind: "stack" as const, activeId: stack.active, children: stack.tabs.map((id) => ({ kind: "window" as const, id })) })),
  };
  return <Mode windows={windows} activeWindowId={active} onActiveWindowChange={(id) => setActive(id ?? active)} layout={layout} />;
}

export function projectionTree(nodes: readonly ProjectionNode[]): OracleTreeNode[] {
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

export function address(node: ProjectionNode): Address {
  return { windowId: fixture.projection.windowId, windowGeneration: fixture.projection.windowGeneration, nodeId: node.nodeId, nodeKey: node.nodeKey };
}

export function OracleNode({ entry, focused, send }: { readonly entry: OracleTreeNode; readonly focused: number | null; readonly send: (event: WireEvent) => void }): React.JSX.Element {
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

export function OracleMirror({ sent }: { readonly sent: WireEvent[] }): React.JSX.Element {
  const [focused, setFocused] = useState<number | null>(null);
  const send = (event: WireEvent): void => {
    sent.push(event);
    if (event.kind === "accessibility-focus") setFocused(event.nodeId);
  };
  return <section aria-label="Semio controls">{projectionTree(fixture.projection.nodes).map((entry) => <OracleNode key={entry.node.nodeKey} entry={entry} focused={focused} send={send} />)}</section>;
}

export function NestedPanelOracle(): React.JSX.Element {
  const selection = fixture.nestedPanelSelection;
  const [path, setPath] = useState<readonly string[]>([selection.rootId, selection.appFirstLeafId]);
  const active = path.at(-1);
  return <section data-path={path.join("/")}>
    <button type="button" role="tab" aria-selected={active === selection.targetLeafId} onClick={() => setPath(selection.expectedPath)}>{selection.targetLeafId}</button>
    {active === selection.targetLeafId ? <select aria-label="Appearance" data-node-key={selection.requiredControlId}><option>System</option></select> : null}
  </section>;
}

export function mountOracle(): { readonly container: HTMLDivElement; readonly canvas: HTMLCanvasElement; readonly sent: WireEvent[] } {
  const container = document.createElement("div");
  const canvas = document.createElement("canvas");
  document.body.append(container, canvas);
  const root = createRoot(container);
  mounted.push(root);
  const sent: WireEvent[] = [];
  act(() => root.render(<OracleMirror sent={sent} />));
  return { container, canvas, sent };
}

export function staleReason(event: WireEvent): string | null {
  if (event.windowId !== fixture.projection.windowId) return "window-not-live";
  if (event.windowGeneration !== fixture.projection.windowGeneration) return "stale-window-generation";
  const node = fixture.projection.nodes.find((candidate) => candidate.nodeId === event.nodeId);
  if (!node || node.nodeKey !== event.nodeKey) return "stale-node-identity";
  return null;
}

export const { computeAccessibleDescription, computeAccessibleName, getRole }: typeof AccessibilityOracle = createRequire(import.meta.url)("dom-accessibility-api");
