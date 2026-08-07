// #region 🖥️Platform
/// <reference types="vitest/importMeta" />
/** @emoji 🖥️ `@semio-tech/framework` — element ids, presence, dock/pane persistence, and inspector helpers. */
import type { IconName } from "@semio-tech/assets";
import {
  type ActionDescriptor,
  type UiStackNode,
  type UiTreeNode,
  type UiControlNode,
  type UiFieldNode,
  type UiGroupNode,
  type UiInspectorFieldGroup,
  type UiNode,
  type UiSectionNode,
  type UiTreeItemNode,
  type UiTreeSectionNode,
  type CanvasPickTarget,
  type CanvasHoverFocus,
  type WindowLayout,
  UI_INSPECTOR_MIXED_PLACEHOLDER,
} from "../🛂️manifest/🟦️component.ts";


//#region 🆔️ElementId
/** 🆔️ Element id of the app shell's navbar/footer — singular, shell-owned chrome. */
export const UI_NAVBAR_ELEMENT_ID = "ui.navbar";
export const UI_FOOTER_ELEMENT_ID = "ui.footer";

/** 🆔️ Normalizes arbitrary input into a single camelCase element-id segment — byte-for-byte mirror of
 * `element_id_segment` in `framework/core/rs/lib.rs` (core/js stays DOM-free, so the DOM-facing
 * `elementIdSelector`/alias helpers live in `framework/ui/js/react` instead). */
function elementIdSegment(raw: string): string {
  let segment = "";
  let capitalizeNext = false;
  for (const ch of raw) {
    if (ch === "-" || ch === "_" || ch === " " || ch === ".") {
      capitalizeNext = true;
      continue;
    }
    if (!/[a-zA-Z0-9]/.test(ch)) continue;
    if (segment.length === 0) {
      segment += ch.toLowerCase();
    } else if (capitalizeNext) {
      segment += ch.toUpperCase();
      capitalizeNext = false;
    } else {
      segment += ch;
    }
  }
  return segment;
}

/** 🆔️ Element id of a window kind's body — `framework.window.{camelCased kind id}`. */
export function windowElementId(kindId: string): string {
  return `framework.window.${elementIdSegment(kindId)}`;
}

/** 🆔️ Element id of a panel tab's uncollapsed panel body; `tabId` is already dotted, appended verbatim. */
export function panelTabElementId(tabId: string): string {
  return `framework.panelTab.${tabId}`;
}

/** 🆔️ Alias id of the first draggable tree row inside a panel tab, stamped via `data-element-alias`. */
export function panelTabFirstDraggableElementId(tabId: string): string {
  return `framework.panelTab.${tabId}.firstDraggable`;
}
//#endregion 🆔️ElementId

//#region 🧭️UiPresence
const DEFAULT_UI_PRESENCE: UiPresence = { state: "normal", status: "idle", hover: false, selected: false };

/** @emoji 🧭️ Resolves optional wire-format `presence` to the shared default inert model. */
export function resolveUiPresence(presence?: UiPresence): UiPresence {
  return presence ?? DEFAULT_UI_PRESENCE;
}

/** @emoji 🧭️ True when the element should show a skeleton instead of its content. */
export function uiPresenceShowsSkeleton(presence?: UiPresence): boolean {
  const status = resolveUiPresence(presence).status;
  return status === "loading" || status === "waiting";
}

/** @emoji 🧭️ Maps measure chrome booleans to the shared status axis until generated `WindowMeasure` gains `presence`. */
export function windowMeasureChromeStatus(measure: { readonly loading?: boolean; readonly waiting?: boolean }): UiStatus {
  if (measure.loading) return "loading";
  if (measure.waiting) return "waiting";
  return "idle";
}

/** @emoji 🧭️ Shared presence stamp for shell surfaces waiting on `refreshUi`. */
export const UI_PENDING_PRESENCE: UiPresence = { state: "normal", status: "loading", hover: false, selected: false };

/** @emoji 🦴 Declarative placeholder node while a window body is still loading. */
export function pendingWindowUiNode(): UiStackNode {
  return { type: "stack", direction: "column", children: [], presence: UI_PENDING_PRESENCE };
}

/** @emoji 🦴 Declarative placeholder node while a panel tab body is still loading. */
export function pendingPanelUiNode(): UiTreeNode {
  return { type: "tree", sections: [], presence: UI_PENDING_PRESENCE };
}
//#endregion 🧭️UiPresence


export function canvasPickTargetKey(target: CanvasPickTarget): string {
  return `${target.domain}:${target.id}`;
}

/** @emoji 🪪️ Parses a pick target key into domain and id. */
export function parseCanvasPickTargetKey(key: string): { readonly domain: string; readonly id: string } | null {
  const colon = key.indexOf(":");
  if (colon < 0) return null;
  return { domain: key.slice(0, colon), id: key.slice(colon + 1) };
}

export function sortCanvasPickTargetsGeneralFirst(targets: readonly CanvasPickTarget[]): readonly CanvasPickTarget[] {
  return [...targets].sort((left, right) => left.generality - right.generality || left.label.localeCompare(right.label));
}

export function pickMostSpecificCanvasTarget(targets: readonly CanvasPickTarget[]): CanvasPickTarget | null {
  if (targets.length === 0) return null;
  return [...targets].sort((left, right) => right.generality - left.generality)[0] ?? null;
}

export function canvasHoverFocusFromTarget(sourceId: string, target: CanvasPickTarget | null): CanvasHoverFocus {
  return { sourceId, target };
}

export function createWindowLayout(windowKindId: string, title?: string, options?: { readonly instanceId?: string; readonly templateId?: string }): WindowLayoutWindowNode {
  return {
    kind: "window",
    windowKindId,
    ...(title ? { title } : {}),
    ...(options?.instanceId ? { instanceId: options.instanceId } : {}),
    ...(options?.templateId ? { templateId: options.templateId } : {}),
  };
}

export function createStackLayout(windowKindIds: readonly string[], titles?: readonly string[]): WindowLayout {
  return {
    root: {
      kind: "stack",
      children: windowKindIds.map((windowKindId, index) => createWindowLayout(windowKindId, titles?.[index])),
    },
  };
}

export function createDefaultLayout(windowIds: readonly string[], direction: "row" | "column" = "row", sizes?: readonly number[], titles?: readonly string[]): WindowLayout {
  return {
    root: {
      kind: direction,
      children: windowIds.map((id, index) => ({
        kind: "stack" as const,
        ...(sizes?.[index] !== undefined ? { size: sizes[index] } : {}),
        children: [createWindowLayout(id, titles?.[index] ?? id)],
      })),
    },
  };
}

export function createTabStackLayout(windowIds: readonly string[], titles?: readonly string[]): WindowLayout {
  return createStackLayout(windowIds, titles);
}

export function createNamedLayout(id: string, label: string, layout: WindowLayout, origin: NamedLayout["origin"] = "builtin", iconId?: IconName, groupPath?: readonly string[]): NamedLayout {
  return {
    id,
    label,
    layout,
    origin,
    ...(iconId ? { iconId } : {}),
    ...(groupPath?.length ? { groupPath } : {}),
  };
}

export function mergeById<T extends { id: string }>(base: readonly T[] | undefined, extension: readonly T[] | undefined): T[] | undefined {
  if (!base?.length && !extension?.length) return undefined;
  const merged = new Map<string, T>();
  base?.forEach((entry) => merged.set(entry.id, entry));
  extension?.forEach((entry) => merged.set(entry.id, entry));
  return [...merged.values()];
}

export function mergeNamedLayouts(base: readonly NamedLayout[] | undefined, extension: readonly NamedLayout[] | undefined): NamedLayout[] {
  return mergeById(base, extension) ?? [];
}

export type PlatformSubscriber = () => void;

export abstract class Store<TSnapshot> {
  private readonly listeners = new Set<PlatformSubscriber>();
  private disposed = false;

  abstract getSnapshot(): TSnapshot;

  subscribe(listener: PlatformSubscriber): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  protected notify(): void {
    if (this.disposed) return;
    for (const listener of this.listeners) listener();
  }

  dispose(): void {
    this.disposed = true;
    this.listeners.clear();
  }
}

export interface StoragePort {
  get(key: string): string | null;
  set(key: string, value: string): void;
  remove(key: string): void;
}

function namedLayoutStorageKey(appId: string): string {
  return `compose.display.layouts.${appId}`;
}

export class NamedLayoutStore extends Store<readonly NamedLayout[]> {
  private layouts: NamedLayout[] = [];

  constructor(
    private readonly appId: string,
    private readonly storage: StoragePort,
  ) {
    super();
    this.layouts = this.readPersisted();
  }

  getSnapshot(): readonly NamedLayout[] {
    return this.layouts;
  }

  save(layout: NamedLayout): void {
    const next = mergeNamedLayouts(
      this.layouts.filter((entry) => entry.id !== layout.id),
      [layout],
    );
    this.layouts = next;
    this.persist();
    this.notify();
  }

  remove(layoutId: string): void {
    const next = this.layouts.filter((entry) => entry.id !== layoutId);
    if (next.length === this.layouts.length) return;
    this.layouts = next;
    this.persist();
    this.notify();
  }

  private readPersisted(): NamedLayout[] {
    const raw = this.storage.get(namedLayoutStorageKey(this.appId));
    if (!raw) return [];
    try {
      const parsed = JSON.parse(raw) as unknown;
      if (!Array.isArray(parsed)) return [];
      return parsed.filter(
        (entry): entry is NamedLayout =>
          Boolean(entry) && typeof entry === "object" && typeof (entry as NamedLayout).id === "string" && typeof (entry as NamedLayout).label === "string" && (entry as NamedLayout).origin === "user" && Boolean((entry as NamedLayout).layout),
      );
    } catch {
      return [];
    }
  }

  private persist(): void {
    this.storage.set(namedLayoutStorageKey(this.appId), JSON.stringify(this.layouts));
  }
}

/** 🧭️ The eight anchor ids, mirroring `Anchor` in `framework/ui/js/react/index.tsx` (kept inline/private here to stay dependency-free of that package) — shared by every persisted anchor-keyed shape below so they can't drift apart from one another. */
type PersistedAnchor = "top-left" | "top-middle" | "top-right" | "right-middle" | "bottom-right" | "bottom-middle" | "bottom-left" | "left-middle";

//#region DockLayoutStore
/** 🐳️ One tab (leaf or branch) in a persisted dock panel-arrangement tree; leaves carry `trees`, branches carry `children`. */
export interface DockTabSkeleton {
  id: string;
  children?: readonly DockTabSkeleton[];
  trees?: readonly string[];
}

/** 🐳️ The full persisted dock arrangement, one tab tree per anchor — anchor ids mirror `Anchor` in `framework/ui/js/react/index.tsx` (kept inline here to stay dependency-free of that package). */
export interface DockSkeleton {
  version: 3;
  anchors: Record<PersistedAnchor, readonly DockTabSkeleton[]>;
}

function dockOsStorageKey(): string {
  return "semio.os.dock";
}

function dockAppStorageKey(appId: string): string {
  return `semio.os.dock.${appId}`;
}

/** 🧪️ Defensive read: corrupt or foreign JSON at `key` resolves to `null` rather than throwing. */
function readDockSkeleton(storage: StoragePort, key: string): DockSkeleton | null {
  const raw = storage.get(key);
  if (!raw) return null;
  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!parsed || typeof parsed !== "object" || (parsed as DockSkeleton).version !== 3 || !(parsed as DockSkeleton).anchors || typeof (parsed as DockSkeleton).anchors !== "object") return null;
    return parsed as DockSkeleton;
  } catch {
    return null;
  }
}

/** 🐳️ Persists the dock panel arrangement across an "os" layer (global default across all apps) and an optional per-app layer that wins when present — `save(null)`/`saveOs(null)` remove rather than persist a JSON `"null"`. */
export class DockLayoutStore extends Store<DockSkeleton | null> {
  constructor(
    private readonly storage: StoragePort,
    private readonly appId?: string,
  ) {
    super();
  }

  getSnapshot(): DockSkeleton | null {
    if (this.appId) {
      const app = readDockSkeleton(this.storage, dockAppStorageKey(this.appId));
      if (app) return app;
    }
    return readDockSkeleton(this.storage, dockOsStorageKey());
  }

  save(skeleton: DockSkeleton | null): void {
    this.writeOrRemove(this.appId ? dockAppStorageKey(this.appId) : dockOsStorageKey(), skeleton);
    this.notify();
  }

  saveOs(skeleton: DockSkeleton | null): void {
    this.writeOrRemove(dockOsStorageKey(), skeleton);
    this.notify();
  }

  reset(): void {
    this.storage.remove(dockOsStorageKey());
    if (this.appId) this.storage.remove(dockAppStorageKey(this.appId));
    this.notify();
  }

  private writeOrRemove(key: string, skeleton: DockSkeleton | null): void {
    if (skeleton === null) this.storage.remove(key);
    else this.storage.set(key, JSON.stringify(skeleton));
  }
}
//#endregion DockLayoutStore

//#region DockUiStateStore
/** 🌱️ Persisted per-anchor panel chrome — only the fields that differ from the shell's computed defaults are ever stored. */
export interface DockUiPanelState {
  visible?: boolean;
  size?: number;
  path?: readonly string[];
}

/** 🌱️ The full persisted dock UI state: per-anchor visibility/size/active-path, per-branch drill-down memory, and tree section/group expansion. Anchor ids mirror `Anchor` (kept inline here to stay dependency-free of the `ui` package, same convention as {@link DockSkeleton}). */
export interface DockUiState {
  version: 3;
  anchors: Partial<Record<PersistedAnchor, DockUiPanelState>>;
  pathMemory?: Readonly<Record<string, string>>;
  treeOpen?: Readonly<Record<string, boolean>>;
}

function dockUiOsStorageKey(): string {
  return "semio.os.dockUi";
}

function dockUiAppStorageKey(appId: string): string {
  return `semio.os.dockUi.${appId}`;
}

/** 🧪️ Defensive read: corrupt or foreign JSON at `key` resolves to `null` rather than throwing. */
function readDockUiState(storage: StoragePort, key: string): DockUiState | null {
  const raw = storage.get(key);
  if (!raw) return null;
  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!parsed || typeof parsed !== "object" || (parsed as DockUiState).version !== 3 || !(parsed as DockUiState).anchors || typeof (parsed as DockUiState).anchors !== "object") return null;
    return parsed as DockUiState;
  } catch {
    return null;
  }
}

/** 🌱️ Persists panel visibility/size/path, drill-down memory, and tree expansion across an "os" layer (global default) and an optional per-app layer that wins when present — `save(null)`/`saveOs(null)` remove rather than persist a JSON `"null"`. */
export class DockUiStateStore extends Store<DockUiState | null> {
  constructor(
    private readonly storage: StoragePort,
    private readonly appId?: string,
  ) {
    super();
  }

  getSnapshot(): DockUiState | null {
    if (this.appId) {
      const app = readDockUiState(this.storage, dockUiAppStorageKey(this.appId));
      if (app) return app;
    }
    return readDockUiState(this.storage, dockUiOsStorageKey());
  }

  save(state: DockUiState | null): void {
    this.writeOrRemove(this.appId ? dockUiAppStorageKey(this.appId) : dockUiOsStorageKey(), state);
    this.notify();
  }

  saveOs(state: DockUiState | null): void {
    this.writeOrRemove(dockUiOsStorageKey(), state);
    this.notify();
  }

  reset(): void {
    this.storage.remove(dockUiOsStorageKey());
    if (this.appId) this.storage.remove(dockUiAppStorageKey(this.appId));
    this.notify();
  }

  private writeOrRemove(key: string, state: DockUiState | null): void {
    if (state === null) this.storage.remove(key);
    else this.storage.set(key, JSON.stringify(state));
  }
}
//#endregion DockUiStateStore

//#region WindowPaneStateStore
/** 🪟️ Persisted state for one window-level pane (a {@link DockUiPanelState} sibling, but keyed per window INSTANCE id rather than globally) — only the fields that differ from the shell's computed defaults are ever stored. */
export interface WindowPaneState {
  anchor?: PersistedAnchor;
  folded?: boolean;
  size?: number;
}

/** 🪟️ The full persisted window-pane arrangement: per-window-instance, per-pane anchor/fold/size — the pane-level analog of {@link DockUiState}, since panes float inside a window rather than docking to the shell's own edges. */
export interface WindowPaneUiState {
  version: 1;
  windows: Record<string, Record<string, WindowPaneState>>;
}

function windowPaneUiOsStorageKey(): string {
  return "semio.os.paneUi";
}

function windowPaneUiAppStorageKey(appId: string): string {
  return `semio.os.paneUi.${appId}`;
}

/** 🧪️ Defensive read: corrupt or foreign JSON at `key` resolves to `null` rather than throwing. */
function readWindowPaneUiState(storage: StoragePort, key: string): WindowPaneUiState | null {
  const raw = storage.get(key);
  if (!raw) return null;
  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!parsed || typeof parsed !== "object" || (parsed as WindowPaneUiState).version !== 1 || !(parsed as WindowPaneUiState).windows || typeof (parsed as WindowPaneUiState).windows !== "object") return null;
    return parsed as WindowPaneUiState;
  } catch {
    return null;
  }
}

/** 🪟️ Persists window-pane anchor/fold/size across an "os" layer (global default across all apps) and an optional per-app layer that wins when present — `save(null)`/`saveOs(null)` remove rather than persist a JSON `"null"`. */
export class WindowPaneStateStore extends Store<WindowPaneUiState | null> {
  constructor(
    private readonly storage: StoragePort,
    private readonly appId?: string,
  ) {
    super();
  }

  getSnapshot(): WindowPaneUiState | null {
    if (this.appId) {
      const app = readWindowPaneUiState(this.storage, windowPaneUiAppStorageKey(this.appId));
      if (app) return app;
    }
    return readWindowPaneUiState(this.storage, windowPaneUiOsStorageKey());
  }

  save(state: WindowPaneUiState | null): void {
    this.writeOrRemove(this.appId ? windowPaneUiAppStorageKey(this.appId) : windowPaneUiOsStorageKey(), state);
    this.notify();
  }

  saveOs(state: WindowPaneUiState | null): void {
    this.writeOrRemove(windowPaneUiOsStorageKey(), state);
    this.notify();
  }

  reset(): void {
    this.storage.remove(windowPaneUiOsStorageKey());
    if (this.appId) this.storage.remove(windowPaneUiAppStorageKey(this.appId));
    this.notify();
  }

  private writeOrRemove(key: string, state: WindowPaneUiState | null): void {
    if (state === null) this.storage.remove(key);
    else this.storage.set(key, JSON.stringify(state));
  }
}
//#endregion WindowPaneStateStore

export function createBrowserStoragePort(): StoragePort {
  return {
    get: (key) => {
      try {
        return typeof localStorage !== "undefined" ? localStorage.getItem(key) : null;
      } catch {
        return null;
      }
    },
    set: (key, value) => {
      try {
        if (typeof localStorage !== "undefined") localStorage.setItem(key, value);
      } catch {
        /* ignore */
      }
    },
    remove: (key) => {
      try {
        if (typeof localStorage !== "undefined") localStorage.removeItem(key);
      } catch {
        /* ignore */
      }
    },
  };
}

/** 🧠️ In-memory {@link StoragePort} — used by ephemeral branded shells so nothing survives a window refresh. */
export function createMemoryStoragePort(): StoragePort {
  const map = new Map<string, string>();
  return {
    get: (key) => map.get(key) ?? null,
    set: (key, value) => {
      map.set(key, value);
    },
    remove: (key) => {
      map.delete(key);
    },
  };
}



/** 🐚️ Namespaces a {@link StoragePort} under `semio.shell.<namespace>.` so several {@link FrameworkOsShell}
 * instances sharing one browser storage origin (e.g. several demonstrator panes) don't read/write each
 * other's `semio.os.dock`/`ui.chrome.*` keys. Not needed for a single page-owning shell — that shell's
 * default (unprefixed) storage is the intended shared surface. */
export function createScopedStoragePort(base: StoragePort, namespace: string): StoragePort {
  const prefix = `semio.shell.${namespace}.`;
  return {
    get: (key) => base.get(`${prefix}${key}`),
    set: (key, value) => base.set(`${prefix}${key}`, value),
    remove: (key) => base.remove(`${prefix}${key}`),
  };
}

export function uiInspectorAllEqual<T>(values: readonly T[]): boolean {
  if (values.length <= 1) return true;
  const first = values[0];
  for (let index = 1; index < values.length; index += 1) {
    if (values[index] !== first) return false;
  }
  return true;
}

export function uiInspectorMixedText(values: readonly string[]): { readonly value: string; readonly placeholder?: string } {
  const uniform = uiInspectorAllEqual(values);
  return { value: uniform ? (values[0] ?? "") : "", placeholder: uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER };
}

export function uiInspectorMixedNumber(values: readonly number[]): { readonly value: number; readonly uniform: boolean } {
  const uniform = uiInspectorAllEqual(values);
  return { value: uniform ? (values[0] ?? 0) : Number.NaN, uniform };
}

export function uiInspectorMixedSelect(values: readonly string[]): { readonly value: string; readonly placeholder?: string } {
  return uiInspectorMixedText(values);
}

export function uiInspectorMixedToggle(values: readonly boolean[]): { readonly pressed: boolean; readonly uniform: boolean } {
  const uniform = uiInspectorAllEqual(values);
  return { pressed: uniform ? (values[0] ?? false) : false, uniform };
}

export function uiInspectorMixedSlider(values: readonly number[]): { readonly value: number; readonly uniform: boolean } {
  return uiInspectorMixedNumber(values);
}

/** @emoji 🔢️ Builds an editable number-stepper field row, computing the mixed/uniform display from
 * `values` via {@link uiInspectorMixedNumber}. `action` is merged into both `onAbsolute` (typed
 * entry, dispatched with `{value}`) and `onDelta` (nudge buttons, `{delta}`) — the patch handler
 * branches on whichever key the dispatched action actually carries. */
export function uiInspectorStepperField(id: string, label: string, values: readonly number[], step: number, action: ActionDescriptor): UiFieldNode {
  const mixed = uiInspectorMixedNumber(values);
  return {
    type: "field",
    id,
    label,
    child: { type: "numberStepper", id, value: mixed.value, step, uniform: mixed.uniform, onAbsolute: action, onDelta: action },
  };
}

/** @emoji 🔘️ Builds an editable boolean toggle field row, computing the mixed/uniform display from
 * `values` via {@link uiInspectorMixedToggle}. */
export function uiInspectorToggleField(id: string, label: string, iconId: IconName, values: readonly boolean[], action: ActionDescriptor): UiFieldNode {
  const mixed = uiInspectorMixedToggle(values);
  return {
    type: "field",
    id,
    label,
    child: { type: "toggle", id, iconId, pressed: mixed.pressed, onChange: action },
  };
}

/** @emoji 📐️ Builds a nested `Origin`-style group: a parent tree item labeled `label` containing
 * three {@link uiInspectorStepperField} children (`X`/`Y`/`Z`), each computing its own per-axis
 * mixed state independently — a multi-selection that agrees on X but not Y shows only Y as "Mixed".
 * `axisAction(axis)` builds the per-axis {@link ActionDescriptor}; callers typically merge
 * `{field: "<id>.x"}` etc. into its `args` so the patch handler can dot-path into the right
 * component with `value` (absolute) or `delta` (relative, offset-preserving across multi-select). */
export function uiInspectorVec3Group(
  id: string,
  label: string,
  values: readonly (readonly [number, number, number])[],
  step: number,
  axisAction: (axis: "x" | "y" | "z") => ActionDescriptor,
): UiGroupNode {
  const xs = values.map((v) => v[0]);
  const ys = values.map((v) => v[1]);
  const zs = values.map((v) => v[2]);
  return {
    type: "group",
    id,
    label,
    defaultOpen: true,
    children: [
      uiInspectorStepperField(`${id}.x`, "X", xs, step, axisAction("x")),
      uiInspectorStepperField(`${id}.y`, "Y", ys, step, axisAction("y")),
      uiInspectorStepperField(`${id}.z`, "Z", zs, step, axisAction("z")),
    ],
  };
}

export function uiInspectorGroupsToTree(groups: readonly UiInspectorFieldGroup[]): UiTreeNode {
  return uiDeclarativeSectionsToTree(
    groups
      .filter((group) => group.fields.length > 0)
      .map((group) => ({
        type: "section" as const,
        id: group.id,
        label: group.label,
        defaultOpen: group.defaultOpen ?? true,
        children: group.fields,
      })),
  );
}

const UI_CONTROL_NODE_TYPES = new Set(["input", "select", "toggle", "button", "keyValue", "slider", "numberStepper", "ring", "iconSelect"]);

function isUiControlNode(node: UiNode): node is UiControlNode {
  return UI_CONTROL_NODE_TYPES.has(node.type);
}

export function uiDeclarativeSectionsToTree(sections: readonly UiSectionNode[]): UiTreeNode {
  const treeSections: UiTreeSectionNode[] = sections.map((section) => ({
    id: section.id,
    label: section.label,
    defaultOpen: section.defaultOpen ?? true,
    items: section.children.map((child, index) => uiDeclarativeChildToTreeItem(child, `${section.id}.${index}`)),
  }));
  return {
    type: "tree",
    sections: treeSections.length ? treeSections : [{ id: "empty", items: [{ id: "empty", label: "—" }] }],
  };
}

function uiDeclarativeChildToTreeItem(node: UiNode, fallbackId: string): UiTreeItemNode {
  if (node.type === "text") return { id: `${fallbackId}.text`, label: node.value };
  if (node.type === "field") {
    if (node.child.type === "text") return { id: node.id, label: node.label, description: node.child.value };
    return { id: node.id, label: node.label, control: isUiControlNode(node.child) ? node.child : undefined };
  }
  if (node.type === "button") return { id: node.id ?? fallbackId, label: node.label, control: node };
  if (node.type === "group") {
    return {
      id: node.id,
      label: node.label,
      defaultOpen: node.defaultOpen,
      items: node.children.map((child, index) => uiDeclarativeChildToTreeItem(child, `${node.id}.${index}`)),
    };
  }
  if (node.type === "input" || node.type === "select" || node.type === "toggle" || node.type === "keyValue" || node.type === "slider" || node.type === "numberStepper" || node.type === "ring" || node.type === "iconSelect") {
    return { id: "id" in node ? String(node.id) : fallbackId, label: "", control: node };
  }
  if (node.type === "separator") return { id: `${fallbackId}.sep`, label: "—" };
  return { id: fallbackId, label: node.type };
}

// #endregion 🖥️Platform
