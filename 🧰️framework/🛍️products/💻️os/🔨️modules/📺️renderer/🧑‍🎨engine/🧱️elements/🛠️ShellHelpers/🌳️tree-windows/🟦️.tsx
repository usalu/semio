/** 🌳️ Utility registry, UI-intent helpers, the tree-window scheduler and the cached tree-panel configurations.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHelpers}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  type ActionDescriptor,
  type AppDefinition,
  type AppWindowKindDefinition,
  type BuiltNode,
  type DerivedUtilitySpec,
  deriveUtilityNodes,
  pendingPanelUiNode,
  type PluginAppLabelsOverlay,
  SET_ACTIVE_UTILITY_ACTION_ID,
  SHELL_LOCALES,
  type UiIntent,
  type UtilityDefinition,
  type UtilityNode,
  type ViewTreeWindowRequest
} from "@semio-tech/framework";
// #endregion 🔌️Adapters
import { type TreePanelConfig, UI_TERMINOLOGY_NATIVE } from "@semio-tech/ui-react";
import { ShellFaultBoundary } from "../../🐚️Shell/🟦️.tsx";
import { builtNodeToSnapshot, UiDocumentStore } from "../../📃️UiDocumentStore/🟦️.tsx";
import { InterpretedUiNode } from "../../🗣️Interpreter/🟦️.tsx";
import { TreeWindowContext, type TreeWindowContextValue, type TreeWindowReportV1 } from "../../🗣️Interpreter/🪟️tree-windows/🟦️.ts";
import {
  EMPTY_APP_LABELS_OVERLAY,
  FRAMEWORK_CHECKIN_CONTROLLER_ID,
  resolveAppLabel,
  resolveManifestLabel,
  ribbonParentLabel,
  shellLabel
} from "../🏷️labels/🟦️.tsx";

//#region ShellHelpers
/** 🎭️ The panels of an actor-bound (hub) document, rendered by the document's verified browser actor rather than
 * the shell's local instance, which never sees the live document (ticket 26/09/23 C10, audit G-P1-4). `stores`
 * holds one retained store per panel-tab id, patched in place by the actor's offers; `onIntent` routes that
 * panel's gestures back to the same actor, except the host-owned ones ({@link panelActionRoutesThroughHostV1}), which
 * go to the shell's `onAction`. A tab without a store yet shows the pending body — never the local instance's stale one. */
export type BrowserActorPanelHostV1 = Readonly<{
  stores: ReadonlyMap<string, UiDocumentStore>;
  onIntent: (tabId: string, intent: UiIntent) => void;
  onAction: (action: ActionDescriptor) => void;
}>;

//#region 🧰️UtilityRegistry
/**
 * 🧰️ Resolves the `UtilityDefinition`s in scope for one window kind against the app's utility registry:
 * the window kind's own `utilities` refs when non-empty, otherwise every utility the app declares (the
 * scoping fallback, mirroring `resolveWindowActions`' intent for utilities). Unresolvable refs are dropped.
 */
export function resolveUtilities(app: Pick<AppDefinition, "utilities">, windowKind: Pick<AppWindowKindDefinition, "utilities">): UtilityDefinition[] {
  const registry = app.utilities ?? [];
  const refs = windowKind.utilities ?? [];
  if (refs.length === 0) return [...registry];
  const resolved: UtilityDefinition[] = [];
  for (const ref of refs) {
    const utility = registry.find((entry) => entry.id === ref);
    if (utility) resolved.push(utility);
  }
  return resolved;
}

/** 🧰️ Resolves a `UtilityDefinition.group` id's display label: the app's own `groupLabels` overlay first, then the shared `ui.ribbon.parent.*` chrome vocabulary for known category ids ({@link ribbonParentLabel}), else the raw id. */
function resolveUtilityGroupLabel(group: string, appLabelsOverlay: PluginAppLabelsOverlay): string {
  return resolveAppLabel(appLabelsOverlay, "group", group, ribbonParentLabel(group) ?? group);
}

/** 🧰️ One `UtilityDefinition` → the lean `DerivedUtilitySpec` consumed by {@link deriveUtilityNodes}, resolving the label (and, for grouped utilities, the group label) through the app's locale/terminology overlay. `UtilityDefinition.label` is a manifest `LocalizedLabel` field. */
function utilityDefinitionToSpec(utility: UtilityDefinition, appLabelsOverlay: PluginAppLabelsOverlay, terminology: string, locale: string): DerivedUtilitySpec {
  return {
    id: utility.id,
    label: resolveAppLabel(appLabelsOverlay, "utility", utility.id, resolveManifestLabel(utility.label, terminology, locale)),
    iconId: utility.iconId,
    group: utility.group ?? undefined,
    groupLabel: utility.group ? resolveUtilityGroupLabel(utility.group, appLabelsOverlay) : undefined,
    category: utility.category ?? "utilities",
  };
}

/** 🧰️ Stamps the owning `windowId` onto every `setActiveUtility` descriptor in a derived utility tree so the shell's `onAction` interceptor targets the right window regardless of which window is globally active.
 * 🔢️ `generation` — the window's utility register generation this tree RENDERED — is stamped as `expectedGeneration` beside it, so the interceptor's compare-and-set refuses a press made against a register that has since moved (`🎯️input-ledger` L3) instead of a wall-clock echo-off window. */
function tagSetActiveUtilityWindow(nodes: readonly UtilityNode[], windowId: string, generation?: number): UtilityNode[] {
  return nodes.map((node) => {
    if (node.kind === "collection") return { ...node, children: tagSetActiveUtilityWindow(node.children, windowId, generation) };
    if (node.kind === "toggle" && "onChange" in node && node.onChange.action === SET_ACTIVE_UTILITY_ACTION_ID) {
      return { ...node, onChange: { ...node.onChange, args: { ...(node.onChange.args as object | undefined), windowId, ...(generation === undefined ? {} : { expectedGeneration: generation }) } } };
    }
    return node;
  });
}

/**
 * 🧰️ Builds the window utility bar `UtilityNode[]` for one window purely from the static utility registry plus
 * the host-owned active utility id — the replacement for the deleted program `list-tools` sourcing. Each
 * `setActiveUtility` descriptor is tagged with `windowId` so activation is scoped to this exact window.
 */
export function resolveUtilityNodes(
  app: Pick<AppDefinition, "utilities" | "controllerId">,
  windowKind: Pick<AppWindowKindDefinition, "utilities">,
  activeUtilityId: string | null | undefined,
  windowId: string,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
  generation?: number,
): UtilityNode[] {
  const utilities = resolveUtilities(app, windowKind);
  if (utilities.length === 0) return [];
  return tagSetActiveUtilityWindow(
    deriveUtilityNodes(
      app.controllerId,
      utilities.map((utility) => utilityDefinitionToSpec(utility, appLabelsOverlay, terminology, locale)),
      activeUtilityId ?? undefined,
    ),
    windowId,
    generation,
  );
}
//#endregion 🧰️UtilityRegistry

/** 🏷️ The argument FIELD a trigger's scalar payload travels under. An action reads named arguments —
 * `value` for an absolute edit (`engagement_input`, `engagement_control_select`,
 * `puzzle3d_absolute_or_delta`'s absolute half) and `delta` for a relative bump (its relative half) —
 * so the name belongs to the TRIGGER, not to the control that fired it. */
function uiInputField(trigger: UiIntent["trigger"]): string {
  return trigger === "delta" ? "delta" : "value";
}

/** 🎬️ The arguments one intent dispatches: the node's AUTHORED args with the gesture's own payload
 * merged over them.
 *
 * 🧯️ A scalar payload used to REPLACE the authored args wholesale — a `NumberStepper`'s `onChange`
 * reports the number `10.5`, so the action received the bare scalar `10.5` and every guest, which reads
 * `args.get("value")`, saw nothing at all while the authored `{windowId}` was thrown away with it. The
 * whole Settings panel was mute for exactly that reason: browser-measured on `:6013`, `setGridSpacing`
 * reaching the guest and settling with `historyUpserts: 0, effects: 0` while the stepper rendered its
 * optimistic 10.5 and both windows' rails stayed at 10 for 30 s (ticket 26/09/02/PUZZLE-3D-END-TO-END
 * wave B47 §2). A scalar is NAMED by its trigger and merged; a map payload merges as it always did. */
function uiIntentPayload(intent: UiIntent): unknown {
  if (intent.input === null) return intent.args ?? undefined;
  const scalar = typeof intent.input === "string" || typeof intent.input === "number" || typeof intent.input === "boolean" || typeof intent.input === "bigint";
  const named = scalar ? { [uiInputField(intent.trigger)]: intent.input } : intent.input;
  if (intent.args === null) return named;
  if (typeof intent.args === "object" && !Array.isArray(intent.args) && typeof named === "object" && !Array.isArray(named)) return { ...intent.args, ...named };
  return named;
}

/** 🌉️ Bridges one semantic UI intent onto the existing plugin action address. Version one is the direct `ActionFactory` mapping; later versions stay explicit in the action name until the host wire owns a version field. */
export function uiIntentToActionDescriptor(intent: UiIntent): ActionDescriptor {
  const payload = uiIntentPayload(intent);
  return { ...actionBindingToActionDescriptor({ action: intent.action, args: null }), ...(payload === undefined ? {} : { args: payload }) };
}

/** 🌉️ The plugin action address an authored binding fires — the mapping {@link uiIntentToActionDescriptor} applies
 * to a fired intent, for a host control that re-offers a guest's own binding (the Tasks window pausing a tool run with
 * the run's own ToolRun-panel Pause). */
export function actionBindingToActionDescriptor(binding: { readonly action: UiIntent["action"]; readonly args: unknown }): ActionDescriptor {
  return {
    controllerId: binding.action.scope,
    action: binding.action.version === 1 ? binding.action.name : `${binding.action.name}@${binding.action.version}`,
    ...(binding.args === null || binding.args === undefined ? {} : { args: binding.args }),
  };
}

//#region 🪟️TreeWindows
/** 🪟️ Trailing debounce on a scroll's window reports. A scroll gesture re-measures once per animation
 * frame; one partial refresh per frame would be one wasm crossing per frame. 40 ms is under the ~60 ms
 * a reader notices as lag and above the frame period, so a continuous drag sends at most ~25 requests
 * per second and a flick sends exactly one — the settle. An OPEN toggle is not debounced at all: it is
 * a discrete gesture whose whole feedback is the rows appearing. */
export const TREE_WINDOW_REPORT_DEBOUNCE_MS = 40;

/** 🪟️ Rows a container the host has just opened asks for before the observer has measured it —
 * mirrors the SDK's own `TREE_WINDOW_DEFAULT_ROWS` (`🔌️plugin/🦀️.rs`, region `🔖️PanelPaging`). Only
 * ever the fallback: `treeViewportRows`, once any body has reported one, is the better number and wins. */
export const TREE_WINDOW_DEFAULT_ROWS = 48;

/** 🪟️ One panel body's host-owned tree state. Keyed by AUTHORED node key throughout, so it survives a
 * body refresh, a `UiDocumentStore` re-mint and the DFS renumbering both of those cause. */
type TreeWindowBodyState = {
  readonly open: Map<string, boolean>;
  readonly windows: Map<string, { readonly offset: number; readonly rows: number }>;
  /** 🪟️ Every node key the observer has ever reported for this body. What it separates is "opened, not in
   * the body yet" — which asks for one viewport of rows so the chevron's promise is kept — from "was in the
   * body, is not now" (a folded ancestor, a document switch), which asks for nothing at all. Without it a
   * container that has left the body keeps claiming a viewport's worth of the guest's node ledger from the
   * containers the reader is actually looking at (📓️s3-review-streaming-loop.md §3). An explicit re-open
   * takes a key back out: that gesture IS "give me rows". */
  readonly measured: Set<string>;
};

export type TreeWindowSchedulerOptionsV1 = {
  /** 🔁️ Fires ONE `refreshUi(session, { kind: "partial", panelBodies: [bodyKey] })`. The caller owns
   * the promise and must call {@link TreeWindowSchedulerV1.settled} when it resolves. */
  readonly refresh: (bodyKey: string) => void;
  readonly debounceMs?: number;
  readonly setTimer?: (run: () => void, ms: number) => unknown;
  readonly clearTimer?: (handle: unknown) => void;
};

export type TreeWindowSchedulerV1 = {
  readonly setOpen: (bodyKey: string, nodeKey: string, open: boolean) => void;
  readonly reportWindows: (bodyKey: string, requests: readonly { readonly nodeKey: string; readonly offset: number; readonly rows: number }[], viewportRows: number) => void;
  readonly openStatesFor: (bodyKey: string) => Readonly<Record<string, boolean>>;
  readonly viewStateFields: () => { readonly treeWindows?: readonly ViewTreeWindowRequest[]; readonly treeViewportRows?: number };
  readonly settled: (bodyKey: string) => void;
  readonly reset: () => void;
};

/**
 * 🪟️ The ONE place host-owned tree expansion and scroll windows are held and turned into refreshes.
 *
 * Pure of React and of the shell: state in plain maps, time injected. The three rules the panel bodies
 * depend on, all testable without a DOM:
 * - an OPEN toggle schedules immediately (a chevron must not wait on a debounce),
 * - window reports coalesce on a {@link TREE_WINDOW_REPORT_DEBOUNCE_MS} trailing edge,
 * - at most ONE partial refresh per body is in flight; a body whose state moved while its refresh was
 *   crossing the wasm boundary is re-sent — with the LATEST state, not the superseded one — the moment
 *   {@link TreeWindowSchedulerV1.settled} reports that crossing done.
 *
 * A report that changes nothing schedules nothing: the observer re-measures on every store revision, and
 * a refresh that re-asks for exactly the rows the guest already rendered is an infinite refresh loop.
 */
export function createTreeWindowSchedulerV1(options: TreeWindowSchedulerOptionsV1): TreeWindowSchedulerV1 {
  const debounceMs = options.debounceMs ?? TREE_WINDOW_REPORT_DEBOUNCE_MS;
  const setTimer = options.setTimer ?? ((run: () => void, ms: number) => setTimeout(run, ms) as unknown);
  const clearTimer = options.clearTimer ?? ((handle: unknown) => clearTimeout(handle as ReturnType<typeof setTimeout>));
  const bodies = new Map<string, TreeWindowBodyState>();
  const pending = new Set<string>();
  const inFlight = new Set<string>();
  const unsendable = new Set<string>();
  let viewportRows: number | undefined;
  let timer: unknown = null;

  const bodyState = (bodyKey: string): TreeWindowBodyState => {
    const existing = bodies.get(bodyKey);
    if (existing) return existing;
    const created: TreeWindowBodyState = { open: new Map(), windows: new Map(), measured: new Set() };
    bodies.set(bodyKey, created);
    return created;
  };
  const cancelTimer = () => {
    if (timer === null) return;
    clearTimer(timer);
    timer = null;
  };
  const flush = () => {
    cancelTimer();
    for (const bodyKey of [...pending]) {
      if (inFlight.has(bodyKey)) continue;
      pending.delete(bodyKey);
      inFlight.add(bodyKey);
      options.refresh(bodyKey);
    }
  };
  const arm = () => {
    if (timer !== null) return;
    timer = setTimer(() => {
      timer = null;
      flush();
    }, debounceMs);
  };

  return {
    setOpen: (bodyKey, nodeKey, open) => {
      const state = bodyState(bodyKey);
      if (state.open.get(nodeKey) === open) return;
      state.open.set(nodeKey, open);
      // 🪟️ Opening is a request for rows: forget what the observer last measured for this container, so
      // the first-paint fallback speaks for it again until the tree it is about to render is measured.
      if (open) {
        state.measured.delete(nodeKey);
        state.windows.delete(nodeKey);
      }
      pending.add(bodyKey);
      flush();
    },
    reportWindows: (bodyKey, requests, rows) => {
      const state = bodyState(bodyKey);
      const next = new Map(requests.map((request) => [request.nodeKey, { offset: Math.max(0, Math.floor(request.offset)), rows: Math.max(0, Math.floor(request.rows)) }] as const));
      // 🪟️ The report is the WHOLE body: every windowed container the tree currently renders, the
      // off-screen ones as spacer-only windows. So replacing the map wholesale is also how state for
      // containers that left the body (a document switch, a folded ancestor) is pruned, and an unreported
      // key is by definition one that has never been rendered — the only case the first-paint row
      // fallback in `viewStateFields` may speak for.
      const sameWindows = next.size === state.windows.size && [...next].every(([nodeKey, window]) => state.windows.get(nodeKey)?.offset === window.offset && state.windows.get(nodeKey)?.rows === window.rows);
      // 🪟️ A measured container is a RENDERED container, so it is open — record that unless the host has
      // been told otherwise, and an explicit fold always wins (a report in flight when the user folds
      // must never re-open what they just closed).
      let openedAny = false;
      for (const nodeKey of next.keys()) {
        state.measured.add(nodeKey);
        if (state.open.has(nodeKey)) continue;
        state.open.set(nodeKey, true);
        openedAny = true;
      }
      const sameRows = viewportRows === Math.max(viewportRows ?? 0, rows);
      if (sameWindows && sameRows && !openedAny) return;
      state.windows.clear();
      for (const [nodeKey, window] of next) state.windows.set(nodeKey, window);
      viewportRows = Math.max(viewportRows ?? 0, rows) || undefined;
      pending.add(bodyKey);
      arm();
    },
    openStatesFor: (bodyKey) => Object.fromEntries(bodies.get(bodyKey)?.open ?? []),
    viewStateFields: () => {
      const flattened: ViewTreeWindowRequest[] = [];
      for (const bodyKey of [...bodies.keys()].sort()) {
        const state = bodies.get(bodyKey)!;
        const nodeKeys = [...new Set([...state.open.keys(), ...state.windows.keys()])].sort();
        for (const nodeKey of nodeKeys) {
          if (!treeWindowSendableIdentifierV1(bodyKey, nodeKey, unsendable)) continue;
          const open = state.open.get(nodeKey);
          const window = state.windows.get(nodeKey);
          // 🪟️ A container the user just opened has no measurement yet — it was not in the body when the
          // observer last looked — so it asks for one viewport's worth, which the observer's first real
          // report then narrows. `rows: 0` would materialise nothing at all and the row the chevron
          // promised would never appear. This fallback speaks ONLY for a never-measured container: an
          // open one that is merely scrolled off screen is in every report, at `rows: 0`, and must not be
          // handed a viewport's worth of the body's budget for rows nobody is looking at.
          const rows = window?.rows ?? (open === false || state.measured.has(nodeKey) ? 0 : (viewportRows ?? TREE_WINDOW_DEFAULT_ROWS));
          flattened.push({ bodyKey, nodeKey, ...(open === undefined ? {} : { open }), offset: window?.offset ?? 0, rows });
        }
      }
      return { ...(flattened.length > 0 ? { treeWindows: flattened } : {}), ...(viewportRows === undefined ? {} : { treeViewportRows: viewportRows }) };
    },
    settled: (bodyKey) => {
      inFlight.delete(bodyKey);
      if (pending.has(bodyKey)) flush();
    },
    reset: () => {
      cancelTimer();
      bodies.clear();
      pending.clear();
      inFlight.clear();
      unsendable.clear();
      viewportRows = undefined;
    },
  };
}

/**
 * 🚧️ Whether one window path may cross the wasm boundary at all, by the SAME law the view context itself is
 * admitted under — `parseResolvedPluginViewState`'s `identifier` (`🛂️manifest/🟦️.ts:977`): non-empty, at most
 * 256 code points, no C0 control character and no DEL. Mirrored rather than imported because this module must
 * not depend on the manifest parser, and pinned by a law that runs the REAL parser over this function's output.
 *
 * 🧯️ The view context is validated as ONE object, so a single unsendable path does not lose that path — it
 * takes the whole crossing down. Measured on the fem3d House lane: every `refreshUi` answered
 * `view context: invalid identifier` and every unrelated `interactionSelect` was refused `dispatch-failed`,
 * because one nested window path carried a control character. Tree-window state is a convenience; it must
 * never be able to break a refresh or an action. So an unsendable path is DROPPED, loudly, once.
 */
function treeWindowSendableIdentifierV1(bodyKey: string, nodeKey: string, reported: Set<string>): boolean {
  if (nodeKey.length > 0 && Array.from(nodeKey).length <= 256 && !/[\u0000-\u001f\u007f]/u.test(nodeKey)) return true;
  const once = `${bodyKey}/${nodeKey}`;
  if (!reported.has(once)) {
    reported.add(once);
    console.error(`[tree-window] unsendable path ${JSON.stringify(nodeKey)} in panel body ${JSON.stringify(bodyKey)} — a window path must be a view-context identifier (1…256 code points, no control characters); this container will not stream`);
  }
  return false;
}

/** 🪟️ What `ShellHost` hands the panel builders — one stable object per shell, so the per-tab memo
 * cache below keys on identity rather than on a closure rebuilt every render. */
export type TreeWindowHostV1 = {
  readonly openStatesFor: (bodyKey: string) => Readonly<Record<string, boolean>>;
  readonly setOpen: (bodyKey: string, nodeKey: string, open: boolean) => void;
  readonly reportWindows: (bodyKey: string, requests: readonly TreeWindowReportV1[], viewportRows: number) => void;
};

/** 🌿️ Keeps a body's channel stable while reading the current host-owned disclosure map. */
export function createTreeWindowContextV1(bodyKey: string, host: TreeWindowHostV1): TreeWindowContextValue {
  return {
    bodyKey,
    get openStates() { return host.openStatesFor(bodyKey); },
    setOpen: (nodeKey, open) => host.setOpen(bodyKey, nodeKey, open),
    reportWindows: (requests, viewportRows) => host.reportWindows(bodyKey, requests, viewportRows),
  };
}

/** 🪟️ The one line a body's open map is compared on for memo invalidation. */
function treeWindowOpenSignatureV1(openStates: Readonly<Record<string, boolean>>): string {
  return Object.keys(openStates)
    .sort()
    .map((key) => `${key}:${openStates[key] ? 1 : 0}`)
    .join(",");
}

/** 🌲️ One panel tab's already-built {@link TreePanelConfig}, reused by reference until THAT tab's own
 * inputs move.
 *
 * `uiNodeToTreePanelConfig` mints a fresh `UiDocumentStore` and re-snapshots the whole body on every
 * call, and the panel-body `useMemo`s in `ShellHost` list the whole record as a dependency — so one
 * body refreshing re-parsed and remounted EVERY open tab's tree (📓️audit-host-tree-pipeline.md §8).
 * With a windowed tree that is not merely wasteful: a remount throws away the `<Tree>` instance the
 * scroll observer is attached to, on every scroll of a sibling panel. */
export type PanelTreeConfigCacheV1 = Map<string, { readonly source: BuiltNode | UiDocumentStore; readonly onAction: unknown; readonly bodyKey: string; readonly treeWindows: TreeWindowHostV1 | null; readonly openSignature: string; readonly config: TreePanelConfig }>;

function cachedTreePanelConfigV1(cache: PanelTreeConfigCacheV1 | undefined, tabId: string, node: BuiltNode, bodyKey: string, onAction: (action: ActionDescriptor) => void, treeWindows: TreeWindowHostV1 | null): TreePanelConfig {
  const openSignature = treeWindows ? treeWindowOpenSignatureV1(treeWindows.openStatesFor(bodyKey)) : "";
  const entry = cache?.get(tabId);
  if (entry && entry.source === node && entry.onAction === onAction && entry.bodyKey === bodyKey && entry.treeWindows === treeWindows && entry.openSignature === openSignature) return entry.config;
  const config = uiNodeToTreePanelConfig(node, onAction, bodyKey, treeWindows);
  cache?.set(tabId, { source: node, onAction, bodyKey, treeWindows, openSignature, config });
  return config;
}

/** 🗂️ A tree that hosts a retained body store itself — the shell's own panel body store or an actor's — so a reload or a
 * patch updates the mounted panel in place; the config is rebuilt only when the store, the intent `route` or the
 * tree-window inputs move. */
function cachedStoreTreePanelConfigV1(cache: PanelTreeConfigCacheV1 | undefined, tabId: string, store: UiDocumentStore, bodyKey: string, treeWindows: TreeWindowHostV1 | null, route: unknown, build: () => TreePanelConfig): TreePanelConfig {
  const openSignature = treeWindows ? treeWindowOpenSignatureV1(treeWindows.openStatesFor(bodyKey)) : "";
  const entry = cache?.get(tabId);
  if (entry && entry.source === store && entry.onAction === route && entry.bodyKey === bodyKey && entry.treeWindows === treeWindows && entry.openSignature === openSignature) return entry.config;
  const config = build();
  cache?.set(tabId, { source: store, onAction: route, bodyKey, treeWindows, openSignature, config });
  return config;
}

/** 🎭️ Whether a panel gesture of an actor-bound document belongs to the host rather than the actor: `undo` takes the
 * host's undo route (the remote inference history when one is mounted, else the actor), and the explicit check-in
 * (`framework.checkin`) is shell state no actor holds. */
export function panelActionRoutesThroughHostV1(action: ActionDescriptor): boolean {
  return action.action === "undo" || action.controllerId === FRAMEWORK_CHECKIN_CONTROLLER_ID;
}

/** 🗂️ A shell-rendered panel over its body store ({@link cachedStoreTreePanelConfigV1}); a tab whose body has no store yet
 * shows the pending body. */
export function cachedBodyStoreTreePanelConfigV1(cache: PanelTreeConfigCacheV1 | undefined, tabId: string, store: UiDocumentStore | undefined, bodyKey: string, onAction: (action: ActionDescriptor) => void, treeWindows: TreeWindowHostV1 | null): TreePanelConfig {
  if (store === undefined) return cachedTreePanelConfigV1(cache, tabId, pendingPanelUiNodeV1(), bodyKey, onAction, treeWindows);
  return cachedStoreTreePanelConfigV1(cache, tabId, store, bodyKey, treeWindows, onAction, () => interpretedTreePanelConfigV1(store, tabId, onAction, (intent) => onAction(uiIntentToActionDescriptor(intent)), bodyKey, treeWindows));
}

/** 🎭️ An actor-rendered panel over the actor's retained store ({@link cachedStoreTreePanelConfigV1}). */
export function cachedActorTreePanelConfigV1(cache: PanelTreeConfigCacheV1 | undefined, tabId: string, actorPanels: BrowserActorPanelHostV1, bodyKey: string, treeWindows: TreeWindowHostV1 | null): TreePanelConfig {
  const store = actorPanels.stores.get(tabId);
  if (store === undefined) return cachedTreePanelConfigV1(cache, tabId, pendingPanelUiNodeV1(), bodyKey, actorPanels.onAction, treeWindows);
  return cachedStoreTreePanelConfigV1(cache, tabId, store, bodyKey, treeWindows, actorPanels.onIntent, () => interpretedTreePanelConfigV1(store, tabId, actorPanels.onAction, (intent) => {
    const action = uiIntentToActionDescriptor(intent);
    return panelActionRoutesThroughHostV1(action) ? actorPanels.onAction(action) : actorPanels.onIntent(tabId, intent);
  }, bodyKey, treeWindows));
}

/** 🦴 The ONE pending body — `pendingPanelUiNode()` mints a fresh object per call, which would miss
 * {@link cachedTreePanelConfigV1} forever for every tab whose body has not arrived yet. */
let pendingPanelUiNodeSingleton: BuiltNode | null = null;
function pendingPanelUiNodeV1(): BuiltNode {
  pendingPanelUiNodeSingleton ??= pendingPanelUiNode();
  return pendingPanelUiNodeSingleton;
}
//#endregion 🪟️TreeWindows

/** 🌲️ Hosts an authored semantic {@link BuiltNode} in the shell's panel-tree leaf without reviving the removed recursive `UiNode` compatibility model.
 *
 * 🪟️ `bodyKey` is the GUEST's panel body key (`AppPanelTabDefinition.bodyKey`, what `UiDirtyScope`'s
 * `panelBodies` and `ViewModel::tree_windows.body_key` both name) — not the tab id the host caches
 * bodies under. `treeWindows`, when present, is scoped to that body and provided to the interpreted
 * subtree, which is what turns the guest tree's expansion and scroll into host state. */
export function uiNodeToTreePanelConfig(node: BuiltNode, onAction: (action: ActionDescriptor) => void, bodyKey: string, treeWindows?: TreeWindowHostV1 | null): TreePanelConfig {
  const store = new UiDocumentStore(`panel:${node.key}`);
  store.loadSnapshot(builtNodeToSnapshot(`panel:${node.key}`, node));
  return interpretedTreePanelConfigV1(store, node.key, onAction, (intent) => onAction(uiIntentToActionDescriptor(intent)), bodyKey, treeWindows);
}

/** 🌲️ Hosts one retained panel store full-width in a tree leaf, with its tree-window context — shared by the
 * authored-body path ({@link uiNodeToTreePanelConfig}) and the actor-rendered path ({@link BrowserActorPanelHostV1}). */
function interpretedTreePanelConfigV1(store: UiDocumentStore, boundaryKey: string, onAction: (action: ActionDescriptor) => void, onIntent: (intent: UiIntent) => void | Promise<void>, bodyKey: string, treeWindows?: TreeWindowHostV1 | null): TreePanelConfig {
  const treeWindowContext = treeWindows ? createTreeWindowContextV1(bodyKey, treeWindows) : null;
  // 🧭️ Never park the interpreted body on an empty-label `TreeDataItem.control` — property-layout rows
  // split every row into a wide label column plus a fixed value column, which parked the lone default
  // `file-text` icon in the left column and squeezed the whole inspector/document/catalogue tree into
  // the value column (ticket 26/08/01 FIX-INSPECTOR-TREES-TO-MATCH-DOCUMENT, remeasured 2026-09-13).
  // Host the body as the tree's full-width `emptyState` instead; the inner document still renders its
  // own `Tree` when the snapshot root is `component.type === "tree"`.
  return {
    sections: [],
    emptyState: (
      <ShellFaultBoundary boundaryId={`panel-${boundaryKey}`} fallbackLabel={shellLabel("ui.common.renderError")}>
        <div className="min-h-0 min-w-0 w-full flex-1">
          <TreeWindowContext.Provider value={treeWindowContext}>
            <InterpretedUiNode store={store} onAction={onAction} onIntent={onIntent} />
          </TreeWindowContext.Provider>
        </div>
      </ShellFaultBoundary>
    ),
    className: "min-w-0 w-full",
    sortableSections: false,
  };
}
//#endregion ShellHelpers
