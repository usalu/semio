// #region 🖥️Platform
/// <reference types="vitest/importMeta" />
/** @emoji 🖥️ `@semio-tech/framework` — element ids, presence, dock/pane persistence, and inspector helpers. */
import type { IconName } from "@semio-tech/assets";
import {
  type ActionDescriptor,
  type CanvasPickTarget,
  type CanvasHoverFocus,
  type NamedLayout,
  type WindowLayout,
  type WindowLayoutWindowNode,
  type UiPresence,
  type UiStatus,
  type Component,
  type LayoutSpec,
  type StyleSpec,
  type AccessibilitySpec,
  type BuiltNode,
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
const DEFAULT_UI_PRESENCE: UiPresence = { state: "normal", status: "idle", hover: false, selected: false, color: null, peers: [] };

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
export const UI_PENDING_PRESENCE: UiPresence = { state: "normal", status: "loading", hover: false, selected: false, color: null, peers: [] };

/** 🧬️ Contract-neutral defaults for a freshly authored {@link BuiltNode} — every field a record
 * carries besides `key`/`component`/`children`, at the value that serializes away for free
 * (`BuiltNode`'s own docstring: "every field ... serializes away at its default"). */
const DEFAULT_BUILT_LAYOUT: LayoutSpec = { kind: "leaf", width: "hug", height: "hug" };
const DEFAULT_BUILT_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const DEFAULT_BUILT_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };

/** 🧱️ Stamps a {@link BuiltNode} from a `component` + optional `children`, filling every other field
 * with the shared defaults above — the small hand-rolled twin of the (not-yet-ported-to-TS) Rust
 * builder DSL `BuiltNode`'s own docstring describes. */
function builtNode(key: string, component: Component, children: readonly BuiltNode[] = []): BuiltNode {
  return {
    key,
    component,
    layout: DEFAULT_BUILT_LAYOUT,
    style: DEFAULT_BUILT_STYLE,
    activity: "idle",
    disabled: false,
    accessibility: DEFAULT_BUILT_ACCESSIBILITY,
    bindings: [],
    menu: null,
    children: [...children],
  };
}

/** @emoji 🦴 Declarative placeholder node while a window body is still loading. `activity: "loading"`
 * is the contract's own mechanism for this (see `Activity`'s docstring: "was `UiStatus` on the old
 * wgpu target's `UiPresence`") — never a `presence` field baked into the node, which the new contract
 * deliberately keeps as a separate, document-external channel (`UiPresenceOverlayContext`). */
export function pendingWindowUiNode(): BuiltNode {
  return { ...builtNode("pending", { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }), activity: "loading" };
}

/** @emoji 🦴 Declarative placeholder node while a panel tab body is still loading. */
export function pendingPanelUiNode(): BuiltNode {
  return { ...builtNode("pending", { type: "tree", interactionDomain: null }), activity: "loading" };
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

/** @emoji 🎚️ The single persisted local-only OS shell configuration document. The four shell
 * projections share this schema and storage key, so persistence has one authority rather than
 * independent key-value stores that can drift across shell instances. */
export interface OsShellConfigSnapshot {
  readonly version: 1;
  readonly preferences: Readonly<Record<string, string>>;
  readonly namedLayouts: Readonly<Record<string, readonly NamedLayout[]>>;
  readonly dockLayouts: {
    readonly os?: DockSkeleton;
    readonly apps: Readonly<Record<string, DockSkeleton>>;
  };
  readonly dockUi: {
    readonly os?: DockUiState;
    readonly apps: Readonly<Record<string, DockUiState>>;
  };
  readonly windowPanes: {
    readonly os?: WindowPaneUiState;
    readonly apps: Readonly<Record<string, WindowPaneUiState>>;
  };
}

const OS_SHELL_CONFIG_STORAGE_KEY = "semio.os.config";

function emptyOsShellConfig(): OsShellConfigSnapshot {
  return { version: 1, preferences: {}, namedLayouts: {}, dockLayouts: { apps: {} }, dockUi: { apps: {} }, windowPanes: { apps: {} } };
}

/** @emoji 🎚️ Typed config-lane adapter over the host's storage port. Writes always re-read the
 * latest complete document before applying a projection update, preserving sibling projections
 * when several store views share a browser origin. */
export class OsShellConfig extends Store<OsShellConfigSnapshot> {
  private readonly storage: StoragePort;
  constructor(storage: StoragePort) {
    super();
    this.storage = storage;
  }

  getSnapshot(): OsShellConfigSnapshot {
    const raw = this.storage.get(OS_SHELL_CONFIG_STORAGE_KEY);
    if (!raw) return emptyOsShellConfig();
    try {
      const parsed = JSON.parse(raw) as Partial<OsShellConfigSnapshot>;
      if (parsed.version !== 1 || !parsed.preferences || !parsed.namedLayouts || !parsed.dockLayouts?.apps || !parsed.dockUi?.apps || !parsed.windowPanes?.apps) return emptyOsShellConfig();
      return parsed as OsShellConfigSnapshot;
    } catch {
      return emptyOsShellConfig();
    }
  }

  update(update: (current: OsShellConfigSnapshot) => OsShellConfigSnapshot): void {
    const next = update(this.getSnapshot());
    this.storage.set(OS_SHELL_CONFIG_STORAGE_KEY, JSON.stringify(next));
    this.notify();
  }

  getPreference(key: string): string | undefined {
    return this.getSnapshot().preferences[key];
  }

  setPreference(key: string, value: string): void {
    this.update((current) => ({ ...current, preferences: { ...current.preferences, [key]: value } }));
  }

  reset(): void {
    this.storage.remove(OS_SHELL_CONFIG_STORAGE_KEY);
    this.notify();
  }
}

export class NamedLayoutStore extends Store<readonly NamedLayout[]> {
  private layouts: NamedLayout[] = [];
  private readonly config: OsShellConfig;
  private readonly appId: string;

  constructor(
    appId: string,
    storage: StoragePort,
  ) {
    super();
    this.appId = appId;
    this.config = new OsShellConfig(storage);
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
    const parsed = this.config.getSnapshot().namedLayouts[this.appId];
    if (!Array.isArray(parsed)) return [];
    return parsed.filter(
      (entry): entry is NamedLayout =>
        Boolean(entry) && typeof entry === "object" && typeof entry.id === "string" && typeof entry.label === "string" && entry.origin === "user" && Boolean(entry.layout),
    );
  }

  private persist(): void {
    this.config.update((current) => ({ ...current, namedLayouts: { ...current.namedLayouts, [this.appId]: this.layouts } }));
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

function validDockSkeleton(value: unknown): value is DockSkeleton {
  return Boolean(value) && typeof value === "object" && (value as DockSkeleton).version === 3 && Boolean((value as DockSkeleton).anchors) && typeof (value as DockSkeleton).anchors === "object";
}

/** 🐳️ Persists the dock panel arrangement across an "os" layer (global default across all apps) and an optional per-app layer that wins when present — `save(null)`/`saveOs(null)` remove rather than persist a JSON `"null"`. */
export class DockLayoutStore extends Store<DockSkeleton | null> {
  private readonly config: OsShellConfig;
  private readonly appId?: string;

  constructor(
    storage: StoragePort,
    appId?: string,
  ) {
    super();
    this.appId = appId;
    this.config = new OsShellConfig(storage);
  }

  getSnapshot(): DockSkeleton | null {
    const layouts = this.config.getSnapshot().dockLayouts;
    if (this.appId) {
      const app = layouts.apps[this.appId];
      if (validDockSkeleton(app)) return app;
    }
    return validDockSkeleton(layouts.os) ? layouts.os : null;
  }

  save(skeleton: DockSkeleton | null): void {
    this.updateLayer(this.appId, skeleton);
    this.notify();
  }

  saveOs(skeleton: DockSkeleton | null): void {
    this.updateLayer(undefined, skeleton);
    this.notify();
  }

  reset(): void {
    this.config.update((current) => {
      const apps = { ...current.dockLayouts.apps };
      if (this.appId) delete apps[this.appId];
      return { ...current, dockLayouts: { apps } };
    });
    this.notify();
  }

  private updateLayer(appId: string | undefined, skeleton: DockSkeleton | null): void {
    this.config.update((current) => {
      const apps = { ...current.dockLayouts.apps };
      if (appId) {
        if (skeleton) apps[appId] = skeleton;
        else delete apps[appId];
        return { ...current, dockLayouts: { ...current.dockLayouts, apps } };
      }
      return { ...current, dockLayouts: skeleton ? { ...current.dockLayouts, os: skeleton } : { apps } };
    });
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

function validDockUiState(value: unknown): value is DockUiState {
  return Boolean(value) && typeof value === "object" && (value as DockUiState).version === 3 && Boolean((value as DockUiState).anchors) && typeof (value as DockUiState).anchors === "object";
}

/** 🌱️ Persists panel visibility/size/path, drill-down memory, and tree expansion across an "os" layer (global default) and an optional per-app layer that wins when present — `save(null)`/`saveOs(null)` remove rather than persist a JSON `"null"`. */
export class DockUiStateStore extends Store<DockUiState | null> {
  private readonly config: OsShellConfig;
  private readonly appId?: string;

  constructor(
    storage: StoragePort,
    appId?: string,
  ) {
    super();
    this.appId = appId;
    this.config = new OsShellConfig(storage);
  }

  getSnapshot(): DockUiState | null {
    const dockUi = this.config.getSnapshot().dockUi;
    if (this.appId) {
      const app = dockUi.apps[this.appId];
      if (validDockUiState(app)) return app;
    }
    return validDockUiState(dockUi.os) ? dockUi.os : null;
  }

  save(state: DockUiState | null): void {
    this.updateLayer(this.appId, state);
    this.notify();
  }

  saveOs(state: DockUiState | null): void {
    this.updateLayer(undefined, state);
    this.notify();
  }

  reset(): void {
    this.config.update((current) => {
      const apps = { ...current.dockUi.apps };
      if (this.appId) delete apps[this.appId];
      return { ...current, dockUi: { apps } };
    });
    this.notify();
  }

  private updateLayer(appId: string | undefined, state: DockUiState | null): void {
    this.config.update((current) => {
      const apps = { ...current.dockUi.apps };
      if (appId) {
        if (state) apps[appId] = state;
        else delete apps[appId];
        return { ...current, dockUi: { ...current.dockUi, apps } };
      }
      return { ...current, dockUi: state ? { ...current.dockUi, os: state } : { apps } };
    });
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

function validWindowPaneUiState(value: unknown): value is WindowPaneUiState {
  return Boolean(value) && typeof value === "object" && (value as WindowPaneUiState).version === 1 && Boolean((value as WindowPaneUiState).windows) && typeof (value as WindowPaneUiState).windows === "object";
}

/** 🪟️ Persists window-pane anchor/fold/size across an "os" layer (global default across all apps) and an optional per-app layer that wins when present — `save(null)`/`saveOs(null)` remove rather than persist a JSON `"null"`. */
export class WindowPaneStateStore extends Store<WindowPaneUiState | null> {
  private readonly config: OsShellConfig;
  private readonly appId?: string;

  constructor(
    storage: StoragePort,
    appId?: string,
  ) {
    super();
    this.appId = appId;
    this.config = new OsShellConfig(storage);
  }

  getSnapshot(): WindowPaneUiState | null {
    const panes = this.config.getSnapshot().windowPanes;
    if (this.appId) {
      const app = panes.apps[this.appId];
      if (validWindowPaneUiState(app)) return app;
    }
    return validWindowPaneUiState(panes.os) ? panes.os : null;
  }

  save(state: WindowPaneUiState | null): void {
    this.updateLayer(this.appId, state);
    this.notify();
  }

  saveOs(state: WindowPaneUiState | null): void {
    this.updateLayer(undefined, state);
    this.notify();
  }

  reset(): void {
    this.config.update((current) => {
      const apps = { ...current.windowPanes.apps };
      if (this.appId) delete apps[this.appId];
      return { ...current, windowPanes: { apps } };
    });
    this.notify();
  }

  private updateLayer(appId: string | undefined, state: WindowPaneUiState | null): void {
    this.config.update((current) => {
      const apps = { ...current.windowPanes.apps };
      if (appId) {
        if (state) apps[appId] = state;
        else delete apps[appId];
        return { ...current, windowPanes: { ...current.windowPanes, apps } };
      }
      return { ...current, windowPanes: state ? { ...current.windowPanes, os: state } : { apps } };
    });
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

// 🚧️ `uiInspectorStepperField`/`uiInspectorToggleField`/`uiInspectorVec3Group`/
// `uiInspectorGroupsToTree`/`uiDeclarativeSectionsToTree`/`uiDeclarativeChildToTreeItem`/
// `isUiControlNode`/`UI_CONTROL_NODE_TYPES` (an "editable field embedded inside a tree row"
// inspector-panel builder) were removed here, not migrated — found NOT tractable within this
// packet's scope, for two independent reasons, both real gaps rather than a syntax rename:
// (1) `Interpreter/🟦️component.tsx`'s current `TreeView`/`treeItemToTreeData` (see its `#region
//     Tree`) never recurses into a non-`treeItem` child as an inline row control — the rendering
//     path this subsystem's output would need does not exist yet, so a type-correct `BuiltNode`
//     tree here would still be a dead, unrenderable shape, not a working feature.
// (2) Every field needs a real `ActionBinding` (`{trigger, action: ActionId, args, capability}`),
//     and `ActionId` is `{scope, name, version}` — versioned — while the old call sites here only
//     ever had an `ActionDescriptor` (`{controllerId, action, args?}`, no version). Picking a
//     version is a product decision (which registered action, which version), not a mechanical
//     rename, and not mine to invent.
// The only consumer was `ShellHost/🟦️component.tsx` (registrar-only, already broken end to end on
// the old `UiNode` shape independent of this), so nothing else in the tree references these names.
// `uiInspectorMixedText`/`Number`/`Select`/`Toggle`/`Slider`/`uiInspectorAllEqual` above (the pure
// mixed-value math, no `UiNode` typing) are untouched and still compile — whoever rebuilds this
// panel on the new contract can reuse them directly.

// #endregion 🖥️Platform
