/// <reference types="vitest/importMeta" />
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🟦️Interpreter/component.tsx
/** @emoji 🌳️ `🟦️Interpreter` — turns the semantic UI contract (`semio-framework-ui-contract`'s
 * `UiNodeRecord`/`Component`/`LayoutSpec`/`StyleSpec`/`AccessibilitySpec`) retained by a
 * `📃️UiDocumentStore` into `@semio-tech/ui-react` components. `InterpretedUiNode` is the entry point;
 * `UiNodeView` is the atomic per-node unit — each one subscribes to exactly its own record via
 * `useUiNode`, so a `SetComponent` on one node re-renders exactly that node's component, never its
 * ancestors or siblings (the whole payoff of the flat id-keyed table, see `📃️UiDocumentStore`'s header
 * doc). Also owns the `ComponentSceneHost` registry (lazily mounts `canvas-2d`/`world-3d`/etc. surface
 * hosts behind `Component::Surface`) and the shared per-surface context-menu flow — both unchanged
 * from the pre-migration renderer, since neither depends on the old `UiNode` shape. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { createContext, memo, Profiler, useCallback, useContext, useEffect, useMemo, useRef, useState, useSyncExternalStore, type ComponentType, type CSSProperties, type ReactElement, type ReactNode, type KeyboardEvent as ReactKeyboardEvent, type MouseEvent as ReactMouseEvent, type RefObject } from "react";
import { packedTextLeaf } from "../🔌️PluginRuntime/🧳️packed-text/🟦️.ts";
import { leftoverTreeItemSelectedV1, leftoverWorldSelectionOverlayV1, subscribeLeftoverWorldSelectionV1 } from "../🌐️World3dHost/🟦️.tsx";
import {
  Button,
  ContextMenuController,
  Field,
  Icon,
  IconSelector,
  Input,
  Ring,
  Section,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
  Stepper,
  TREE_WINDOW_BODY_NODE_BUDGET,
  TREE_WINDOW_OVERSCAN_ROWS,
  Textarea,
  Toggle,
  Tree,
  VirtualFileSystem,
  borderElementClass,
  borderNormalTopClass,
  catalogueTreeDragController,
  classifyIconSelectorMode,
  capTreeWindowRequests,
  cn,
  elementSkeleton,
  loadingBorderElementClass,
  interactionMergeFromModifiers,
  renderControlIcon,
  TREE_WINDOW_PATH_SEPARATOR,
  treeWindowPathOf,
  treeWindowRequestsForViewport,
  treeWindowVisibleRowsForViewport,
  useLabel,
  waitingBorderElementClass,
  type ContextMenuItem,
  type ElementSkeletonKind,
  type IconName,
  type TreeDataActivationContext,
  type TreeDataItem,
  type TreeDataSection,
  type TreeDragAndDropController,
  type TreePanelConfig,
  type TreeWindowContainerMeasure,
  type TreeWindowRequest,
  type TreeWindowRowMeasure,
  type UiLabel,
  type UiTranslationKey,
} from "@semio-tech/ui-react";
import { domSizePx, uiSpacingRem } from "@semio-tech/ui-styling";
import {
  type ActionBinding,
  type ActionDescriptor,
  type AppCatalogue,
  type ComponentKind,
  type ComponentSceneHostProps,
  type ContextMenuItemSpec,
  type ContinuousGestureLane,
  type PluginContextMenuRequest,
  type UiComponentSceneNode,
  type UiMenuRef,
  type SceneLane,
  type SceneLaneRef,
  BOARD2D_SCENE_LANES,
  CANVAS2D_SCENE_LANES,
  TILEDMAP_SCENE_LANES,
  WORLD3D_SCENE_LANES,
  WORLD3D_SCENE_LANE_KEY_PREFIX,
  board2dSceneFromLanes,
  canvas2dSceneFromLanes,
  createContinuousGestureLane,
  sceneFromLanes,
  world3dSceneFromLanes,
  world3dSceneLaneForBodyKey,
} from "@semio-tech/framework";
import {
  DEFAULT_UI_DOCUMENT_LIMITS,
  UiDocumentStore,
  emitIntent,
  guestPresenceTableV1,
  subscribeGuestPresenceV1,
  useUiDocumentRevision,
  useUiDocumentRoot,
  useUiNode,
  type UiDocumentState,
} from "../📃️UiDocumentStore/🟦️.tsx";
import {
  type AbsoluteLayout,
  type AccessibilitySpec,
  type Component,
  type EdgeSpace,
  type GridLayout,
  type GridTrack,
  type LayoutSpec,
  type LeafLayout,
  type OverlayLayout,
  type ScrollLayout,
  type Sizing,
  type SpaceToken,
  type MergeMode,
  type StackLayout,
  type StyleSpec,
  type PatchRejection,
  type SurfaceId,
  type SurfaceProps,
  type UiDocumentLimits,
  type UiIntent,
  type UiNodeId,
  type UiNodeRecord,
  type UiPatch,
  type UiSnapshot,
  type UiTrigger,
  type UiValue,
} from "@semio-tech/framework";
import { decodeScenePackField, decodeScenePackValue } from "@semio-tech/framework-os";
import { uiAccessibilityValueV1, uiProgressFractionV1 } from "../../../../../../../🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🟦️.ts";
import { shellLabel } from "../🛠️ShellHelpers/🟦️.tsx";
import { useMapContextMenuSpecs } from "../🏛️ShellHost/🟦️.tsx";
import { ShellFaultBoundary } from "../🐚️Shell/🟦️.tsx";
import { WindowInstanceIdContext, World3dHost } from "../🌐️World3dHost/🟦️.tsx";
import { NodeGraphHost } from "../🕸️NodeGraph/🟦️.tsx";
import { TextEditorHost } from "../✏️TextEditor/🟦️.tsx";
import { TableHost } from "../📊️Table/🟦️.tsx";
import { Paint2dHost } from "../🖌️Paint2dHost/🟦️.tsx";
import { TiledMapHost } from "../🧭️TiledMapHost/🟦️.tsx";
import { Board2dHost } from "../🖥️Board2dHost/🟦️.tsx";
import { IconRenderHost } from "../🖼️IconRenderHost/🟦️.tsx";
import { InkCanvasHost } from "../🖋️InkCanvasHost/🟦️.tsx";
import { GraphTimelineHost } from "../🌳️GraphTimelineHost/🟦️.tsx";
import { BlockListHost } from "../🧩️BlockListHost/🟦️.tsx";
import { DiffViewHost } from "../🔺️DiffViewHost/🟦️.tsx";
import { EventFeedHost } from "../📡️EventFeedHost/🟦️.tsx";
// 🐢️ Direct element-to-element import — `🟦️Interpreter` and `Canvas2dHost` landed in the same batch.
import { Canvas2dHost } from "../📐️Canvas2dHost/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️UiInterpreter
//#region PresenceOverlay
/** 👥️ This session's own hover/selection/preview state on one node, keyed by `UiNodeRecord.key` (NOT
 * `UiNodeId` — presence must land on the right element across a reconciliation that reassigns ids but
 * keeps keys stable, mirroring `crate::PresenceUpdate`'s own doc). Populated from `PresenceUpdate`
 * wire messages by whoever owns the transport (host-side, outside this element); never derived from
 * or written into the `📃️UiDocumentStore` — presence changes at input frequency and must never touch a
 * document revision. */
export type UiPresenceOverlayEntry = {
  readonly hovered?: boolean;
  readonly selected?: boolean;
  readonly previewed?: boolean;
};

export type UiPresenceOverlayValue = {
  readonly byKey: ReadonlyMap<string, UiPresenceOverlayEntry>;
};

const EMPTY_PRESENCE_OVERLAY: UiPresenceOverlayValue = { byKey: new Map() };

export const UiPresenceOverlayContext = createContext<UiPresenceOverlayValue>(EMPTY_PRESENCE_OVERLAY);

export function usePresenceOverlayEntry(key: string): UiPresenceOverlayEntry {
  const overlay = useContext(UiPresenceOverlayContext);
  return overlay.byKey.get(key) ?? {};
}

/** 👥️ The overlay a rendered tree actually reads: whatever a caller provided through
 * {@link UiPresenceOverlayContext} (the component tests' own channel), overlaid by the live table the
 * plugin runtime fills from each turn's `presence` array. Without the live half the context had no
 * production provider at all and every retained row rendered `aria-selected="false"` however the
 * guest's interaction domain moved (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gap F1). */
export function useUiPresenceOverlay(): UiPresenceOverlayValue {
  const provided = useContext(UiPresenceOverlayContext);
  const guest = useSyncExternalStore(subscribeGuestPresenceV1, guestPresenceTableV1, guestPresenceTableV1);
  return useMemo(() => {
    if (guest.byKey.size === 0) return provided;
    if (provided.byKey.size === 0) return guest;
    return { byKey: new Map([...provided.byKey, ...guest.byKey]) };
  }, [provided, guest]);
}
//#endregion PresenceOverlay

//#region 🌲️TreePanelBoundary
type PanelTreePresence = {
  readonly status?: "idle" | "loading" | "waiting";
  readonly selected?: boolean;
};

type PanelTreeItem = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly iconId?: IconName;
  readonly defaultOpen?: boolean;
  readonly presence?: PanelTreePresence;
  readonly dimmed?: boolean;
  readonly draggable?: boolean;
  readonly dragData?: Record<string, string>;
  readonly items?: readonly PanelTreeItem[];
  readonly action?: ActionDescriptor;
  readonly hoverAction?: ActionDescriptor;
  readonly unhoverAction?: ActionDescriptor;
  readonly actions?: readonly {
    readonly iconId: IconName;
    readonly label?: string;
    readonly placement?: "row" | "menu";
    readonly action: ActionDescriptor;
  }[];
};

type PanelTreeNode = {
  readonly sections: readonly {
    readonly id: string;
    readonly label?: string;
    readonly defaultOpen?: boolean;
    readonly presence?: PanelTreePresence;
    readonly items: readonly PanelTreeItem[];
  }[];
  readonly selectedIds?: readonly string[];
  readonly highlightedIds?: readonly string[];
  readonly selectionChange?: ActionDescriptor;
  readonly dropAction?: ActionDescriptor;
};

function dispatchPanelTreeAction(onAction: (action: ActionDescriptor) => void, descriptor: ActionDescriptor, patch: Record<string, unknown>): void {
  onAction({ ...descriptor, args: { ...(typeof descriptor.args === "object" && descriptor.args != null ? descriptor.args : {}), ...patch } });
}

function panelTreeItemsToData(items: readonly PanelTreeItem[], onAction: (action: ActionDescriptor) => void): TreeDataItem[] {
  return items.map((item) => ({
    id: item.id,
    label: item.label,
    description: item.description,
    icon: item.iconId ? renderControlIcon(item.iconId, 12) : undefined,
    defaultOpen: item.defaultOpen,
    isSelected: item.presence?.selected,
    loading: item.presence?.status === "loading",
    waiting: item.presence?.status === "waiting",
    isHidden: item.dimmed,
    draggable: item.draggable,
    dragData: item.dragData,
    items: item.items?.length ? panelTreeItemsToData(item.items, onAction) : undefined,
    onClick: item.action ? () => dispatchPanelTreeAction(onAction, item.action!, {}) : undefined,
    onPointerEnter: item.hoverAction ? () => dispatchPanelTreeAction(onAction, item.hoverAction!, {}) : undefined,
    onPointerLeave: item.unhoverAction ? () => dispatchPanelTreeAction(onAction, item.unhoverAction!, {}) : undefined,
    actions: item.actions?.map((action) => ({
      kind: "button" as const,
      icon: action.iconId,
      title: action.label ? wireLabel(action.label) : undefined,
      placement: action.placement ?? "row",
      onClick: () => dispatchPanelTreeAction(onAction, action.action, {}),
    })),
  }));
}

/** @emoji 🌲️ Maps the still-supported manifest tree payload onto the owned panel-tree contract. */
export function uiTreeNodeToTreePanelConfig(treeNode: PanelTreeNode, onAction: (action: ActionDescriptor) => void): TreePanelConfig {
  const sections: TreeDataSection[] = treeNode.sections.map((section) => ({
    id: section.id,
    label: section.label ?? "",
    defaultOpen: section.defaultOpen,
    loading: section.presence?.status === "loading",
    waiting: section.presence?.status === "waiting",
    items: panelTreeItemsToData(section.items, onAction),
  }));
  return {
    sections,
    selectedIds: treeNode.selectedIds ? [...treeNode.selectedIds] : undefined,
    highlightedIds: treeNode.highlightedIds,
    onSelectionChange: treeNode.selectionChange ? (selectedIds) => dispatchPanelTreeAction(onAction, treeNode.selectionChange!, { ids: selectedIds }) : undefined,
    sortableSections: Boolean(treeNode.dropAction) && sections.length > 1,
  };
}

/** @emoji 🖱️ First catalogue-transfer MIME on a declarative {@link Tree} document, if any row carries drag data. */
function treeSectionsCatalogueDragMime(sections: readonly TreeDataSection[]): string | undefined {
  const visit = (items: readonly TreeDataItem[]): string | undefined => {
    for (const item of items) {
      if (item.dragData) {
        const mime = Object.keys(item.dragData).find((key) => key === "application/x-semio-catalogue-item");
        if (mime) return mime;
      }
      if (item.items?.length) {
        const nested = visit(item.items);
        if (nested) return nested;
      }
    }
    return undefined;
  };
  for (const section of sections) {
    const mime = visit(section.items ?? []);
    if (mime) return mime;
  }
  return undefined;
}

function panelTreeDragMime(treeNode: PanelTreeNode): string | undefined {
  const visit = (items: readonly PanelTreeItem[]): string | undefined => {
    for (const item of items) {
      const mime = item.dragData ? Object.keys(item.dragData)[0] : undefined;
      if (mime) return mime;
      const nested = item.items?.length ? visit(item.items) : undefined;
      if (nested) return nested;
    }
    return undefined;
  };
  for (const section of treeNode.sections) {
    const mime = visit(section.items);
    if (mime) return mime;
  }
  return undefined;
}

/** @emoji 🖱️ Owns manifest-tree drag payload and drop-action routing. */
export function declarativeTreeDragController(treeNode: PanelTreeNode, onAction: (action: ActionDescriptor) => void): TreeDragAndDropController | undefined {
  const mime = panelTreeDragMime(treeNode);
  const source = mime ? catalogueTreeDragController(mime) : undefined;
  if (!treeNode.dropAction) return source;
  return {
    ...(source ?? {}),
    handleDrop: ({ data, target, dropPosition }) => {
      const encoded = Object.entries(data).find(([kind, value]) => kind.startsWith("application/x-semio-") && value.trim())?.[1];
      if (!encoded) return;
      let payload: Record<string, unknown>;
      try {
        payload = JSON.parse(encoded) as Record<string, unknown>;
      } catch {
        return;
      }
      dispatchPanelTreeAction(onAction, treeNode.dropAction!, { ...payload, targetId: target.id, dropPosition: dropPosition ?? "inside" });
    },
  };
}
//#endregion 🌲️TreePanelBoundary

//#region ComponentSceneHostRegistry
/** 🧭️ Resolve scene hosts at render time — these modules form a cycle with Interpreter
 * (`World3dHost` imports `openSurfaceContextMenu` from here), so a module-init
 * `Record` / fake `React.lazy(Promise.resolve({ Host }))` can capture `undefined`
 * and leave Suspense forever on "Loading surface…". Live bindings are ready by first paint. */
function resolveComponentSceneHost(kind: ComponentKind): ComponentType<ComponentSceneHostProps> | undefined {
  switch (kind) {
    case "canvas-2d":
      return Canvas2dHost;
    case "world-3d":
      return World3dHost;
    case "node-graph":
      return NodeGraphHost;
    case "text-editor":
      return TextEditorHost;
    case "table":
      return TableHost;
    case "paint-2d":
      return Paint2dHost;
    case "tiled-map":
      return TiledMapHost;
    case "board-2d":
      return Board2dHost;
    case "icon-render":
      return IconRenderHost;
    case "ink-canvas":
      return InkCanvasHost;
    case "graph-timeline":
      return GraphTimelineHost;
    case "block-list":
      return BlockListHost;
    case "diff-view":
      return DiffViewHost;
    case "event-feed":
      return EventFeedHost;
    default:
      return undefined;
  }
}
//#endregion ComponentSceneHostRegistry

function interpLabel(key: UiTranslationKey): UiLabel {
  return shellLabel(key);
}

/** @emoji 🕳️ Sanctioned wire-boundary mint point (see ui-react's `UiLabel` docstring): brands an
 * already plugin/manifest-resolved string as {@link UiLabel}. */
export function wireLabel(value: string): UiLabel {
  return value as UiLabel;
}

//#region SurfaceBridge
/** 🗺️ `SurfaceKind` wire tag → the `UiComponentSceneNode` optional field the matching host reads —
 * verbatim the same 15-entry convention `ComponentKind`/`resolveComponentSceneHost` already use,
 * duplicated here only because the bridge needs the FIELD NAME, not the dispatch target. */
const SURFACE_KIND_SCENE_FIELD: Record<string, string> = {
  "canvas-2d": "canvas2d",
  "world-3d": "world3d",
  "node-graph": "nodeGraph",
  "text-editor": "textEditor",
  table: "table",
  "paint-2d": "paint2d",
  "virtual-file-system": "virtualFileSystem",
  "tiled-map": "tiledMap",
  "board-2d": "board2d",
  "icon-render": "iconRender",
  "ink-canvas": "inkCanvas",
  "graph-timeline": "graphTimeline",
  "block-list": "blockList",
  "diff-view": "diffView",
  "event-feed": "eventFeed",
};

function menuRefFromContract(menu: UiNodeRecord["menu"]): UiMenuRef | undefined {
  if (!menu) return undefined;
  const args = menu.args && typeof menu.args === "object" && !Array.isArray(menu.args) ? (menu.args as Record<string, unknown>) : undefined;
  return { id: menu.id, args };
}

/** 🌉️ True when `docSchema` (`"<kind>@<version>"`) is a shape this bridge can even attempt to decode
 * — the contract itself never validates it against `kind` (see `SurfaceProps`'s own doc: "a mismatch
 * ... is a scene-crate-level authoring bug, not a contract violation"), so this Interpreter is exactly
 * where that gate has to live. Only the SHAPE is checked (non-empty name + numeric version), never a
 * per-kind version registry this file has no visibility into — an unrecognised but well-formed schema
 * still renders through `sceneField`'s per-kind switch; only a malformed one is refused outright. */
function isWellFormedDocSchema(docSchema: string): boolean {
  const at = docSchema.lastIndexOf("@");
  return at > 0 && /^\d+$/.test(docSchema.slice(at + 1));
}

/** 🌉️ Bridges `Component::Surface`'s `SurfaceProps` (one opaque pack-encoded `doc.bytes` payload,
 * keyed by `docSchema`) onto the OLD `UiComponentSceneNode` the 14 scene-host elements (Canvas2dHost,
 * World3dHost, ...) still expect — those elements are outside this packet's OWNS and are unchanged,
 * so this Interpreter decodes the new contract's opaque payload into the exact per-kind scene field
 * shape those hosts already know how to read, rather than duplicating 14 host components. The
 * contract never parses `doc.bytes` itself (see `🦀️surface.rs`'s own doc); this is the one place that
 * decodes it, and only to hand it straight through unmodified.
 *
 * `surfaceId`/`controllerId` no longer exist on `SurfaceProps` (six placement fields were dropped in
 * the `ui-w4-core` mirror regeneration). `surfaceId` is therefore the OWNING DOCUMENT's surface —
 * exactly the `pluginSurfaceRef(instance, target.key)` key the plugin host's `surface_contexts` table
 * is keyed by (`1:puzzle3d-main-perspective` / `1:puzzle3d-main-top`) — because a surface host's
 * identity is the window it was mounted into, not its position in that window's node tree. Taking
 * `record.id` for it instead collapsed BOTH panes of one app onto `"1"`: the id is a per-document
 * DFS-order integer, so two windows of one program mint the same one and nothing downstream (a
 * dispatch's `args.surfaceId`, `data-surface-id`, a per-pane probe) could tell them apart
 * (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B20 defect 1). `controllerId` stays the record id — it
 * scopes per-app host registries (catalogue drop preview) that are deliberately pane-independent.
 * `paneId` carries the program's own authored surface id (`record.key`, set by `scene_surface`'s
 * `try_id`), the one thing the contract really did drop; `bindingId` has no contract equivalent and is
 * simply absent (both optional on `UiComponentSceneNode`). Returns `null`, never throws, on a
 * malformed `docSchema` or a decode failure — the caller renders a placeholder + logs the fault, per
 * this ticket's own "never throw, never drop the surrounding patch" rule for an unknown `doc_schema`. */
/** 🪪️ The three identities one surface host is mounted under, and the one place they are decided.
 *
 * - `surfaceId` — the OWNING DOCUMENT's surface (`1:puzzle3d-main-perspective`), i.e. the plugin
 *   host's own `surface_contexts` key. A surface host's identity is the WINDOW it renders in.
 * - `controllerId` — the record id. Per-app registries keyed on it (the catalogue drop preview) are
 *   deliberately pane-independent, so this one must NOT gain the window.
 * - `paneId` — the program's own authored surface id, the `try_id` `scene_surface` set as the node's
 *   `key`. It is what the dropped `SurfaceProps.surfaceId` used to carry, and is stable across
 *   refreshes where the record id is not.
 *
 * Taking the record id for `surfaceId` collapsed every pane of one app onto `"1"` — a per-document
 * DFS-order integer two windows of one program both mint (ticket 26/09/02/PUZZLE-3D-END-TO-END B20). */
export function surfaceHostIdentityV1(surface: SurfaceId, key: string, recordId: UiNodeId): { readonly surfaceId: string; readonly controllerId: string; readonly paneId: string | undefined } {
  return { surfaceId: String(surface), controllerId: String(recordId), paneId: key || undefined };
}

function surfacePropsToComponentSceneNode(record: UiNodeRecord, props: SurfaceProps, surface: SurfaceId, assemble?: SurfaceSceneAssembler): UiComponentSceneNode | null {
  if (!isWellFormedDocSchema(props.docSchema)) {
    console.error("[Interpreter] malformed Component::Surface docSchema", { nodeId: record.id, kind: props.kind, docSchema: props.docSchema });
    return null;
  }
  let decoded: Record<string, unknown> | undefined;
  try {
    decoded = props.doc.bytes.length > 0 ? (decodeScenePackValue(new Uint8Array(props.doc.bytes)) as Record<string, unknown>) : undefined;
  } catch (error) {
    console.error("[Interpreter] failed to decode Component::Surface doc bytes", { nodeId: record.id, kind: props.kind, docSchema: props.docSchema, error });
    return null;
  }
  const sceneField = SURFACE_KIND_SCENE_FIELD[props.kind];
  const node: Record<string, unknown> = {
    type: "componentScene",
    ...surfaceHostIdentityV1(surface, record.key, record.id),
    componentKind: props.kind,
    menu: menuRefFromContract(record.menu),
  };
  if (sceneField && decoded !== undefined) node[sceneField] = assemble ? assemble(decoded) : decoded;
  return node as unknown as UiComponentSceneNode;
}

/** 🚚️ Reattaches a surface's out-of-doc payload lanes to the spine its `doc.bytes` decoded to. Only a
 * scene kind that declares lanes has one (`world-3d`); every other kind renders its decoded doc
 * verbatim. See {@link PagedSurfaceView}. */
type SurfaceSceneAssembler = (spine: Record<string, unknown>) => Record<string, unknown>;

/** 🖱️ The `onAction` a scene host receives: the shell's funnel with every dispatch stamped `origin: "gesture"`
 * (`🎯️input-ledger` `InputOriginV1`) — a canvas/world/map/board host only ever forwards pointer and camera
 * streams, whose refusals must be logged, never toasted. Memoised per underlying callback so a host's own
 * `useCallback([onAction])` chains (Canvas2dHost rebuilds its canvas session on `dispatch` identity) stay
 * stable across renders. A stamp the host already set (a tutorial replay through a host) wins. */
const gestureOnActionByFunnel = new WeakMap<(action: ActionDescriptor) => void | Promise<unknown>, (action: ActionDescriptor) => void | Promise<unknown>>();
function gestureOnActionFor(onAction: (action: ActionDescriptor) => void | Promise<unknown>): (action: ActionDescriptor) => void | Promise<unknown> {
  let wrapped = gestureOnActionByFunnel.get(onAction);
  if (wrapped === undefined) {
    wrapped = (action) => {
      const given = (action as { readonly provenance?: Record<string, unknown> }).provenance;
      const stamped = { ...action, provenance: { windowId: null, causedBy: null, ...given, origin: given?.origin ?? "gesture" } };
      return onAction(stamped as ActionDescriptor);
    };
    gestureOnActionByFunnel.set(onAction, wrapped);
  }
  return wrapped;
}

function renderComponentSceneHost(
  record: UiNodeRecord,
  props: SurfaceProps,
  funnel: (action: ActionDescriptor) => void | Promise<unknown>,
  surface: SurfaceId,
  requestContextMenu?: UiInterpreterContext["requestContextMenu"],
  assemble?: SurfaceSceneAssembler,
): ReactNode {
  const onAction = gestureOnActionFor(funnel);
  const node = surfacePropsToComponentSceneNode(record, props, surface, assemble);
  if (!node) {
    return (
      <p className="text-muted-foreground text-xs" data-unknown-surface-schema={props.docSchema}>
        {interpLabel("ui.common.unknownComponent")}: {props.kind}
      </p>
    );
  }
  if (props.kind === "virtual-file-system") {
    return (
      <ShellFaultBoundary boundaryId="surface-virtualFileSystem" fallbackLabel={shellLabel("ui.common.renderError")}>
        <VirtualFileSystemHost node={node} onAction={onAction} requestContextMenu={requestContextMenu} />
      </ShellFaultBoundary>
    );
  }
  const Host = resolveComponentSceneHost(props.kind as ComponentKind);
  if (!Host) {
    return (
      <p className="text-muted-foreground text-xs">
        {interpLabel("ui.common.unknownComponent")}: {props.kind}
      </p>
    );
  }
  return (
    <ShellFaultBoundary boundaryId={`surface-${props.kind}`} fallbackLabel={shellLabel("ui.common.renderError")}>
      <Host node={node} onAction={onAction} requestContextMenu={requestContextMenu} />
    </ShellFaultBoundary>
  );
}

//#region 🚚️SurfaceSceneLanes
/** 🚚️ Last COMPLETE text of one `${nodeId}:${lane}` carrier, keyed by the producer's own content
 * hash. Two jobs, both load-bearing:
 *
 * 1. **Incremental per lane.** A lane whose hash is unchanged is never re-walked or re-concatenated,
 *    so a camera nudge on a 57 KB Nakagin world costs one map lookup per lane instead of 57 KB of
 *    string building — which matters because the spine changes on every frame the camera moves while
 *    `instances` changes only when the document does.
 * 2. **Partial arrival.** A surface tree larger than `SURFACE_RECONCILE_PAGE_BYTES` arrives across
 *    several patches, so a spine can land before the leaves it declares. A lane whose concatenation
 *    does not yet match the declared byte length is omitted when its hash changed (a document swap
 *    must not keep serving the previous fixture) and is served from cache only when the hash still
 *    matches — never as truncated JSON that would parse to nothing.
 *
 * Bounded by {@link SURFACE_SCENE_LANE_CACHE_ENTRIES} with plain insertion-order eviction — an entry
 * is one lane of one live surface node, and a torn-down document simply stops touching its own. */
const SURFACE_SCENE_LANE_CACHE_ENTRIES = 512;
const surfaceSceneLaneCache = new Map<string, { readonly hash: string; readonly text: string }>();

function rememberSurfaceSceneLane(key: string, hash: string, text: string): void {
  surfaceSceneLaneCache.delete(key);
  surfaceSceneLaneCache.set(key, { hash, text });
  while (surfaceSceneLaneCache.size > SURFACE_SCENE_LANE_CACHE_ENTRIES) {
    const oldest = surfaceSceneLaneCache.keys().next();
    if (oldest.done) break;
    surfaceSceneLaneCache.delete(oldest.value);
  }
}

/** 📏️ UTF-8 length of a JS string without allocating a `Uint8Array` — the producer counts lane bytes
 * in UTF-8 (`String::len` in Rust), so a lane carrying a German label must be compared in the same
 * unit or its completeness check would never agree. */
function utf8ByteLength(value: string): number {
  let bytes = 0;
  for (let index = 0; index < value.length; index += 1) {
    const code = value.charCodeAt(index);
    if (code < 0x80) bytes += 1;
    else if (code < 0x800) bytes += 2;
    else if (code >= 0xd800 && code <= 0xdbff) {
      bytes += 4;
      index += 1;
    } else bytes += 3;
  }
  return bytes;
}

/** 🧩️ Depth-first concatenation of every packed `text` leaf (`value` then sorted `dataAttributes`) under `root` — the exact inverse of the plugin
 * host's `paged_text_carrier`, and the same walk `sectionValueFromBuiltNode` does for a reserved
 * refresh section. */
function surfaceSceneLaneText(state: UiDocumentState, root: UiNodeRecord): string {
  let payload = "";
  const stack: UiNodeId[] = [...(root.children ?? [])].reverse();
  while (stack.length > 0) {
    const id = stack.pop()!;
    const record = state.nodes.get(id);
    if (!record) continue;
    if (record.component.type === "text" && typeof record.component.value === "string") {
      payload += packedTextLeaf(record.component.value, record.component.dataAttributes);
    }
    for (let index = (record.children ?? []).length - 1; index >= 0; index -= 1) stack.push(record.children[index]!);
  }
  return payload;
}

/** 🚚️ Collects one paged surface's arrived lane texts, keyed by reserved carrier key, ready for
 * `sceneFromLanes` over the same `lanes` table. A lane the spine does not declare is ignored; a lane
 * that is declared but not yet fully arrived falls back to its last complete text, or is omitted when
 * there is none. */
function surfaceLaneTexts<S extends object>(record: UiNodeRecord, state: UiDocumentState, declared: readonly SceneLaneRef[], lanes: readonly SceneLane<S>[]): ReadonlyMap<string, string> {
  const texts = new Map<string, string>();
  if (declared.length === 0) return texts;
  const refByLane = new Map(declared.map((ref) => [ref.lane, ref]));
  for (const childId of record.children ?? []) {
    const child = state.nodes.get(childId);
    if (!child) continue;
    const lane = lanes.find((entry) => entry.bodyKey === String(child.key));
    const ref = lane && refByLane.get(lane.lane);
    if (!lane || !ref) continue;
    const cacheKey = `${record.id}:${lane.lane}`;
    const cached = surfaceSceneLaneCache.get(cacheKey);
    if (cached?.hash === ref.hash) {
      texts.set(lane.bodyKey, cached.text);
      continue;
    }
    const text = surfaceSceneLaneText(state, child);
    if (utf8ByteLength(text) === ref.bytes || (text.length > 0 && lane.optional)) {
      rememberSurfaceSceneLane(cacheKey, ref.hash, text);
      texts.set(lane.bodyKey, text);
    }
  }
  return texts;
}
//#endregion 🚚️SurfaceSceneLanes
//#endregion SurfaceBridge

//#region UiInterpreterContext
export type UiInterpreterContext = {
  readonly store: UiDocumentStore;
  /** 🌉️ Legacy ActionDescriptor channel — the seam this Interpreter still speaks to the 14 unowned
   * scene-host elements through (see `🌉️SurfaceBridge`). Semantic components (button/input/select/…)
   * never use this; they go through `emitIntent`/`UiIntent` instead. */
  readonly onAction: (action: ActionDescriptor) => void;
  /** 🎬️ Semantic dispatch — fires a `UiIntent` built from the node's own `ActionBinding`s. */
  /** 🔁️ Answers with the promise the shell's own dispatch settles on, so a CONTINUOUS control (a
   * dragged slider, a held spinner) can tell whether its last value has landed. `void` is still
   * accepted for a sink that has nothing to settle. */
  readonly onIntent: (intent: UiIntent) => void | Promise<void>;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
};
//#endregion UiInterpreterContext

/** @emoji 🛍️ The APP-STATIC operator/palette catalogue for the app owning this subtree — fetched ONCE
 * per app instance from the reserved `framework.section.catalogue` retained surface and cached by
 * `ShellHost` for that instance's whole lifetime. Never read off a scene: with the real `brep`/`math`
 * operator sets installed the payload is ~100 KB, more than three times the 32 KiB fixed per-surface
 * admission every scene is encoded against, so carrying it per scene made the node-graph window
 * unrenderable (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). A scene names an operator by KIND ID;
 * this is where that kind's record, ports and palette entry come from. Declared here, beside the other
 * scene-host contexts, so neither `ShellHost` (the provider) nor `NodeGraph` (the consumer) has to
 * import the other for it. */
export const AppCatalogueContext = createContext<AppCatalogue>(Object.freeze({}));

/** @emoji 🛍️ Reads the nearest {@link AppCatalogueContext} — `{}` when no app instance provides one. */
export function useAppCatalogue(): AppCatalogue {
  return useContext(AppCatalogueContext);
}

export const PluginSurfaceActionsContext = createContext<UiInterpreterContext["requestContextMenu"]>(undefined);

export function usePluginSurfaceActions(): UiInterpreterContext["requestContextMenu"] {
  return useContext(PluginSurfaceActionsContext);
}

export const ShellContextMenuFallbackContext = createContext<(() => ContextMenuItem[]) | undefined>(undefined);

export function useShellContextMenuFallback(): (() => ContextMenuItem[]) | undefined {
  return useContext(ShellContextMenuFallbackContext);
}

export type SurfaceContextMenuResult = {
  readonly items: ContextMenuItem[];
  readonly titleKey: UiTranslationKey;
};

const contextMenuSurfaceTitleKeys = {
  blockList: "ui.surfaceContextMenu.step",
  board2d: "ui.surfaceContextMenu.board",
  canvas2d: "ui.surfaceContextMenu.canvas",
  diffView: "ui.surfaceContextMenu.diff",
  eventFeed: "ui.surfaceContextMenu.event",
  graphTimeline: "ui.surfaceContextMenu.history",
  inkCanvas: "ui.surfaceContextMenu.ink",
  nodeGraph: "ui.surfaceContextMenu.flow",
  paint2d: "ui.surfaceContextMenu.paint",
  table: "ui.surfaceContextMenu.row",
  textEditor: "ui.surfaceContextMenu.editor",
  tiledMap: "ui.surfaceContextMenu.map",
  virtualFileSystem: "ui.surfaceContextMenu.file",
  world3d: "ui.surfaceContextMenu.scene",
} as const satisfies Record<string, UiTranslationKey>;

const contextMenuTargetTitleKeys = {
  architecture: "ui.surfaceContextMenu.architecture",
  attraction: "ui.surfaceContextMenu.attraction",
  block: "ui.surfaceContextMenu.block",
  edge: "ui.surfaceContextMenu.edge",
  entry: "ui.surfaceContextMenu.entry",
  feature: "ui.surfaceContextMenu.feature",
  group: "ui.surfaceContextMenu.group",
  handle: "ui.surfaceContextMenu.handle",
  layer: "ui.surfaceContextMenu.layer",
  node: "ui.surfaceContextMenu.node",
  object: "ui.surfaceContextMenu.object",
  part: "ui.surfaceContextMenu.part",
  path: "ui.surfaceContextMenu.path",
  pixel: "ui.surfaceContextMenu.pixel",
  position: "ui.surfaceContextMenu.position",
  reference: "ui.surfaceContextMenu.reference",
  route: "ui.surfaceContextMenu.route",
  row: "ui.surfaceContextMenu.row",
  slider: "ui.surfaceContextMenu.slider",
  vortex: "ui.surfaceContextMenu.vortex",
} as const satisfies Record<string, UiTranslationKey>;

export function surfaceContextMenuTitleKey(request: PluginContextMenuRequest): UiTranslationKey {
  const hitDomain = request.surface?.hits?.[0]?.domain;
  if (hitDomain && hitDomain in contextMenuTargetTitleKeys) return contextMenuTargetTitleKeys[hitDomain as keyof typeof contextMenuTargetTitleKeys];
  const surfaceKind = request.surface?.kind;
  if (surfaceKind && surfaceKind in contextMenuSurfaceTitleKeys) return contextMenuSurfaceTitleKeys[surfaceKind as keyof typeof contextMenuSurfaceTitleKeys];
  return "ui.surfaceContextMenu.workspace";
}

export async function openSurfaceContextMenu(
  requestContextMenu: ((request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>) | undefined,
  request: PluginContextMenuRequest,
  mapSpecs: (specs: readonly ContextMenuItemSpec[]) => ContextMenuItem[],
  shellFallback: (() => ContextMenuItem[]) | undefined,
): Promise<SurfaceContextMenuResult> {
  const titleKey = surfaceContextMenuTitleKey(request);
  if (!requestContextMenu) return { items: shellFallback?.() ?? [], titleKey };
  try {
    // 🖱️ An EMPTY plugin answer stays empty — it is the surface saying "nothing here", and the shell
    // fallback is deliberately NOT substituted for it (law: "openSurfaceContextMenu keeps an empty
    // plugin answer off the shell fallback"). The fallback is only for a surface with no resolver at all.
    return { items: mapSpecs(await requestContextMenu(request)), titleKey };
  } catch {
    return { items: [], titleKey };
  }
}

//#region VirtualFileSystemHost
export function parseSceneJsonField<T>(encoded: string): T {
  if (encoded.startsWith("pk:")) return decodeScenePackField(encoded) as T;
  return JSON.parse(encoded) as T;
}

function VirtualFileSystemHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.virtualFileSystem;
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.file");
  const dispatch = (action: string, args?: Record<string, unknown>): void => {
    onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
  };
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  if (!scene) return <div className="semio-vfs-empty">{emptySceneLabel}</div>;
  const schema = parseSceneJsonField<Parameters<typeof VirtualFileSystem>[0]["schema"]>(scene.schemaJson);
  const rows = parseSceneJsonField<Parameters<typeof VirtualFileSystem>[0]["rows"]>(scene.rowsJson);
  const selectedRowIds = scene.selectedRowIdsJson ? parseSceneJsonField<string[]>(scene.selectedRowIdsJson) : undefined;
  return (
    <>
      <VirtualFileSystem
        className="min-h-0 flex-1"
        schema={schema}
        rows={rows}
        selectedRowIds={selectedRowIds}
        emptyMessage={scene.emptyMessage !== undefined ? wireLabel(scene.emptyMessage) : undefined}
        dragDrop={scene.dragDropEnabled ? { enabled: true } : undefined}
        onSelectionChange={(ids) => onAction({ controllerId: node.controllerId, action: "selectRows", args: { surfaceId: node.surfaceId, ids } })}
        onRowContextMenu={(row, index, event) => {
          if (!requestContextMenu) return;
          event.preventDefault();
          event.stopPropagation();
          const rowId = String(row.id ?? index);
          void (async () => {
            const menu = await openSurfaceContextMenu(
              requestContextMenu,
              {
                menu: { id: "virtualFileSystem", args: null },
                surface: { surfaceId: node.surfaceId, kind: "virtualFileSystem", hits: [{ domain: "row", id: rowId }], selection: selectedRowIds && selectedRowIds.length > 0 ? [{ domain: "row", ids: selectedRowIds }] : [] },
                windowInstanceId: windowInstanceId ?? undefined,
                point: { x: event.clientX, y: event.clientY },
              },
              mapContextMenu,
              shellContextMenuFallback,
            );
            setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
          })();
        }}
      />
      <ContextMenuController title={contextMenuTitleLabel} open={contextMenu != null} position={contextMenu ?? { x: 0, y: 0 }} items={contextMenu?.items ?? []} onOpenChange={(open) => { if (!open) setContextMenu(null); }} />
    </>
  );
}
//#endregion VirtualFileSystemHost

//#region LayoutAndStyle
const SPACE_TOKEN_MULTIPLIER: Record<SpaceToken, number> = { none: 0, xs: 1, sm: 2, md: 4, lg: 6, xl: 8, xxl: 12 };

/** 📐️ Resolves a closed `SpaceToken` against the theme's own `--ui-spacing` ramp
 * (`@semio-tech/ui-styling`'s `uiSpacingRem`) — never a raw pixel value shipped over the wire. The
 * per-token multiplier is the one place this renderer decides "how big is `md`"; it can be retuned
 * without touching the contract, which only ever carries the token name. */
function spaceTokenRem(token: SpaceToken): string {
  return uiSpacingRem(SPACE_TOKEN_MULTIPLIER[token]);
}

function edgeSpaceToPadding(edge: EdgeSpace): string {
  if ("all" in edge) return spaceTokenRem(edge.all);
  if ("symmetric" in edge) return `${spaceTokenRem(edge.symmetric.vertical)} ${spaceTokenRem(edge.symmetric.horizontal)}`;
  return `${spaceTokenRem(edge.each.top)} ${spaceTokenRem(edge.each.right)} ${spaceTokenRem(edge.each.bottom)} ${spaceTokenRem(edge.each.left)}`;
}

function sizingToCss(sizing: Sizing): string | undefined {
  if (sizing === "hug") return undefined;
  if (sizing === "fill") return "100%";
  if (typeof sizing === "object" && "fixed" in sizing) return spaceTokenRem(sizing.fixed);
  return undefined;
}

const ALIGN_CSS: Record<string, string> = { start: "flex-start", center: "center", end: "flex-end", stretch: "stretch", baseline: "baseline" };
const JUSTIFY_CSS: Record<string, string> = { start: "flex-start", center: "center", end: "flex-end", spaceBetween: "space-between", spaceAround: "space-around", spaceEvenly: "space-evenly" };

function gridTrackToCss(track: GridTrack): string {
  if (track === "auto") return "auto";
  if (track === "minContent") return "min-content";
  if (track === "maxContent") return "max-content";
  if (typeof track === "object" && "fraction" in track) return `${track.fraction}fr`;
  if (typeof track === "object" && "fixed" in track) return spaceTokenRem(track.fixed);
  return "auto";
}

/** 🧬️ Resolves one `LayoutSpec` variant into inline flex/grid/overflow CSS — the renderer-neutral
 * vocabulary's React reading. Every metric traces back to a closed enum; nothing here is a value that
 * came off the wire directly. */
function layoutSpecStyle(layout: LayoutSpec): CSSProperties {
  switch (layout.kind) {
    case "leaf": {
      const l = layout as LeafLayout & { kind: "leaf" };
      return { width: sizingToCss(l.width), height: sizingToCss(l.height), minWidth: 0, minHeight: 0 };
    }
    case "stack": {
      const l = layout as StackLayout & { kind: "stack" };
      return {
        display: "flex",
        flexDirection: l.axis === "horizontal" ? "row" : "column",
        gap: spaceTokenRem(l.gap),
        padding: edgeSpaceToPadding(l.padding),
        alignItems: ALIGN_CSS[l.align],
        justifyContent: JUSTIFY_CSS[l.justify],
        flex: l.grow ? "1 1 auto" : undefined,
        flexWrap: l.wrap ? "wrap" : "nowrap",
        minWidth: 0,
        minHeight: 0,
      };
    }
    case "grid": {
      const l = layout as GridLayout & { kind: "grid" };
      return {
        display: "grid",
        gridTemplateColumns: l.columns.map(gridTrackToCss).join(" ") || undefined,
        gridTemplateRows: l.rows.map(gridTrackToCss).join(" ") || undefined,
        columnGap: spaceTokenRem(l.columnGap),
        rowGap: spaceTokenRem(l.rowGap),
        padding: edgeSpaceToPadding(l.padding),
        alignItems: ALIGN_CSS[l.align],
        justifyContent: JUSTIFY_CSS[l.justify],
        minWidth: 0,
        minHeight: 0,
      };
    }
    case "overlay": {
      const l = layout as OverlayLayout & { kind: "overlay" };
      return { position: "relative", padding: edgeSpaceToPadding(l.inset) };
    }
    case "scroll": {
      const l = layout as ScrollLayout & { kind: "scroll" };
      const axes = l.axes;
      return {
        overflowX: axes === "horizontal" || axes === "both" ? "auto" : "hidden",
        overflowY: axes === "vertical" || axes === "both" ? "auto" : "hidden",
        padding: edgeSpaceToPadding(l.padding),
        width: sizingToCss(l.sizing),
        minWidth: 0,
        minHeight: 0,
      };
    }
    case "absolute": {
      const l = layout as AbsoluteLayout & { kind: "absolute" };
      return { position: "absolute", width: sizingToCss(l.sizingWidth), height: sizingToCss(l.sizingHeight) };
    }
    default:
      return { minWidth: 0, minHeight: 0 };
  }
}

/** 🎨️ `StyleSpec` is token-only — never a raw color/pixel. This renderer's first-pass reading exposes
 * every token as a `data-*` attribute rather than guessing a color mapping tokens.json does not yet
 * define a ramp for (flagged by this contract's own upstream packet reports); a theme stylesheet
 * targets `[data-tone="danger"]` etc. Swapping this for direct CSS-variable resolution later is a
 * pure addition, not a wire change. */
function styleSpecDataAttributes(style: StyleSpec): Record<string, string> {
  return {
    "data-variant": style.variant ?? "solid",
    "data-size": style.size ?? "md",
    "data-density": style.density ?? "standard",
    "data-tone": style.tone ?? "neutral",
    "data-emphasis": style.emphasis ?? "regular",
  };
}
//#endregion LayoutAndStyle

//#region Accessibility
/** ♿️ `AccessibilitySpec` → real ARIA props, plus an optional rendered visually-hidden description
 * span (its id feeds `aria-describedby`) — no `role` here, the semantic role comes from `Component`
 * itself (a `Component::Button` is a button on every renderer; see `🦀️accessibility.rs`'s own doc). */
export function accessibilityAriaProps(spec: AccessibilitySpec, idBase: string): { readonly props: Record<string, unknown>; readonly describedBy?: ReactNode } {
  const describedById = spec.description ? `${idBase}-desc` : undefined;
  const props: Record<string, unknown> = {
    "aria-label": spec.label ?? undefined,
    "aria-describedby": describedById,
    "aria-live": spec.live && spec.live !== "off" ? spec.live : undefined,
    "aria-keyshortcuts": spec.shortcut ?? undefined,
    "aria-hidden": spec.hidden ? true : undefined,
  };
  const describedBy = describedById ? (
    <span id={describedById} className="sr-only">
      {spec.description}
    </span>
  ) : undefined;
  return { props, describedBy };
}
//#endregion Accessibility

//#region ActionDispatch
function resolveControlIconNode(iconId: string, size: number | "tiny" | "small" | "base" | "large" = "small"): ReactElement {
  return <Icon icon={iconId as IconName} size={size} />;
}

function dispatchTrigger(context: UiInterpreterContext, record: UiNodeRecord, trigger: UiTrigger, input?: UiValue): void | Promise<void> {
  const intent = emitIntent(context.store, record, trigger, input);
  return intent ? context.onIntent(intent) : undefined;
}

//#region 🎚️ContinuousControlLane
/** 🎚️ The coalescing lane a CONTINUOUS control's `change` trigger rides, one per mounted control.
 *
 * A slider dragged at 60 Hz, or a spinner held down, emits a value every frame. Dispatching each one
 * is one retained command, one document edit, one history entry and one preview re-evaluation EACH:
 * measured on 6018, a one-second drag of the Inspection panel's number field cost 29
 * `patchFlowWidgets`, 24 `toolRunStart`s and 29 history entries, and the mesh arrived 3.0 s behind the
 * value (`📓️slider-preview-update-2026-09-15.md`). The lane keeps ONE value in flight and ONE owed —
 * always the newest — and always sends the release.
 *
 * `context` and `record` are read through a ref because both identities change on every render while
 * the lane must outlive them: a lane recreated per render is not a lane. */
function useContinuousTriggerLane(context: UiInterpreterContext, record: UiNodeRecord): ContinuousGestureLane<UiValue> {
  const bindingRef = useRef({ context, record });
  bindingRef.current = { context, record };
  const gestureRef = useRef<string | null>(null);
  const laneRef = useRef<ContinuousGestureLane<UiValue> | null>(null);
  if (laneRef.current === null) {
    laneRef.current = createContinuousGestureLane<UiValue>({
      // 🎚️ A continuous control's `Change` payload is a RECORD, not a bare scalar: the value the
      // program reads as `value` exactly as before, plus the identity of the press it belongs to and
      // whether this is the release. `uiIntentPayload` merges a record input's own keys into the
      // binding's args, so `value` still arrives under its own name for every existing consumer,
      // while a program that cares can fold a whole press into ONE undoable edit
      // (`📓️slider-preview-update-2026-09-15.md`).
      send: (value, phase) => {
        if (gestureRef.current === null) gestureRef.current = `${bindingRef.current.record.key}:${Date.now()}`;
        const gesture = gestureRef.current;
        if (phase === "commit") gestureRef.current = null;
        return dispatchTrigger(bindingRef.current.context, bindingRef.current.record, "change", { value, gesture, commit: phase === "commit" } as UiValue);
      },
      onFault: (error) => undefined,
    });
  }
  return laneRef.current;
}
//#endregion 🎚️ContinuousControlLane

/** 🧬️ Widens a primitive into the untagged `UiValue` union — every `Change`/`Delta` trigger's own
 * payload is always one of these three JS-native shapes, never a nested list/map, at this call site. */
function toUiValue(value: string | number | boolean): UiValue {
  return value as UiValue;
}
//#endregion ActionDispatch

//#region DeclarativeControlBoundary
type DeclarativeControlBase = {
  readonly id?: string;
  readonly disabled?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly presence?: { readonly state?: string; readonly status?: string };
};

type DeclarativeUiControl =
  | (DeclarativeControlBase & { readonly type: "input"; readonly id: string; readonly inputKind: string; readonly value: string; readonly placeholder?: string; readonly commit?: string; readonly min?: number; readonly max?: number; readonly step?: number; readonly accept?: string; readonly onChange: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "select"; readonly id: string; readonly value: string; readonly items: readonly { readonly value: string; readonly label: string }[]; readonly placeholder?: string; readonly onChange: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "toggle"; readonly id: string; readonly iconId: string; readonly pressed: boolean; readonly text?: string; readonly onChange: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "keyValue"; readonly entries: readonly { readonly label: string; readonly value: string }[] })
  | (DeclarativeControlBase & { readonly type: "slider"; readonly id: string; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly unit?: string; readonly onChange: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "numberStepper"; readonly id: string; readonly value: number; readonly step: number; readonly uniform: boolean; readonly onAbsolute: ActionDescriptor; readonly onDelta: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "ring"; readonly id: string; readonly orbId: string; readonly t: number; readonly onChange: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "iconSelect"; readonly id: string; readonly value: string; readonly uniform: boolean; readonly classifierKind: string; readonly onChange: ActionDescriptor })
  | (DeclarativeControlBase & { readonly type: "button"; readonly iconId: string; readonly label: string; readonly action: ActionDescriptor });

function declarativeControlDisabled(control: DeclarativeControlBase): boolean {
  return control.disabled === true || control.presence?.state === "disabled";
}

function declarativeControlActivityClass(control: DeclarativeControlBase): string | undefined {
  if (control.loading || control.presence?.status === "loading") return loadingBorderElementClass;
  if (control.waiting || control.presence?.status === "waiting") return waitingBorderElementClass;
  return undefined;
}

function dispatchDeclarativeControlAction(onAction: (action: ActionDescriptor) => void, descriptor: ActionDescriptor, patch: Record<string, unknown>): void {
  onAction({ ...descriptor, args: { ...(typeof descriptor.args === "object" && descriptor.args != null ? descriptor.args : {}), ...patch } });
}

/** @emoji 🎛️ Renders the owned structural control payload retained by panel-tree composition. */
export function renderUiControl(control: DeclarativeUiControl, onAction: (action: ActionDescriptor) => void, path?: string): ReactElement {
  switch (control.type) {
    case "input": {
      const commitOnBlur = control.commit === "blur";
      const commitValue = (raw: string) => dispatchDeclarativeControlAction(onAction, control.onChange, { value: control.inputKind === "number" ? Number(raw) : raw });
      if (control.inputKind === "longText") {
        return <Textarea id={control.id} data-ui-path={path} className="min-h-[4.5rem] w-full min-w-0" value={control.value} placeholder={control.placeholder} onChange={commitOnBlur ? undefined : (event) => commitValue(event.target.value)} onBlur={commitOnBlur ? (event) => commitValue(event.target.value) : undefined} />;
      }
      const inputType = control.inputKind === "number" ? "number" : control.inputKind === "date" ? "date" : control.inputKind === "color" ? "color" : control.inputKind === "file" ? "file" : "text";
      return <Input id={control.id} data-ui-path={path} type={inputType} className="h-medium w-full min-w-0" value={control.inputKind === "file" ? undefined : control.value} placeholder={control.placeholder} min={control.min} max={control.max} step={control.step} accept={control.inputKind === "file" ? control.accept : undefined} onChange={commitOnBlur ? undefined : (event) => commitValue(control.inputKind === "file" ? (event.target.files?.[0]?.name ?? "") : event.target.value)} onBlur={commitOnBlur ? (event) => commitValue(control.inputKind === "file" ? (event.target.files?.[0]?.name ?? "") : event.target.value) : undefined} />;
    }
    case "select":
      return (
        <Select id={control.id} value={control.value || undefined} onValueChange={(value) => dispatchDeclarativeControlAction(onAction, control.onChange, { value })}>
          <SelectTrigger id={control.id} data-ui-path={path} className="h-medium w-full min-w-0" size="sm"><SelectValue placeholder={control.placeholder ?? interpLabel("ui.common.select")} /></SelectTrigger>
          <SelectContent>{control.items.map((item, index) => <SelectItem key={`${control.id}:${index}:${item.value}`} value={item.value}>{item.label}</SelectItem>)}</SelectContent>
        </Select>
      );
    case "toggle":
      return <Toggle id={control.id} pressed={control.pressed} text={control.text} icon={resolveControlIconNode(control.iconId)} onPressedChange={(pressed) => dispatchDeclarativeControlAction(onAction, control.onChange, { pressed })} />;
    case "keyValue":
      return <dl className="grid grid-cols-[auto_1fr] gap-x-single gap-y-single text-xs" data-ui-path={path}>{control.entries.map((entry, index) => <div key={`${entry.label}:${index}`} className="contents"><dt className="text-muted-foreground">{entry.label}</dt><dd className="tabular-nums">{entry.value}</dd></div>)}</dl>;
    case "slider": {
      const slider = <Slider id={control.id} data-ui-path={path} className="w-full min-w-0" max={control.max} min={control.min} step={control.step} value={[control.value]} onValueChange={(values) => dispatchDeclarativeControlAction(onAction, control.onChange, { value: values[0] ?? control.value })} />;
      if (!control.unit) return slider;
      return <div className="flex min-w-0 w-full items-center gap-single">{slider}<span className="text-muted-foreground shrink-0 text-xs tabular-nums">{control.value} {control.unit}</span></div>;
    }
    case "numberStepper":
      return <Stepper id={control.id} step={control.step} value={control.uniform ? control.value : undefined} mixed={!control.uniform} onChange={(value) => dispatchDeclarativeControlAction(onAction, control.onAbsolute, { value })} onDelta={(delta) => dispatchDeclarativeControlAction(onAction, control.onDelta, { delta })} />;
    case "ring":
      return <Ring id={control.id} onOrbChange={(_orbId, _oldT, newT) => dispatchDeclarativeControlAction(onAction, control.onChange, { t: newT })} orbs={[{ disabled: declarativeControlDisabled(control), id: control.orbId, selected: true, t: control.t }]} />;
    case "iconSelect":
      return <IconSelector classifyIconSelectorMode={control.classifierKind === "puzzle2d" ? classifyIconSelectorMode : undefined} id={control.id} onChange={(next) => dispatchDeclarativeControlAction(onAction, control.onChange, { value: next })} uniform={control.uniform} value={control.value} />;
    case "button": {
      const activityClass = declarativeControlActivityClass(control);
      return <Button id={control.id} data-ui-path={path} text={control.label} icon={resolveControlIconNode(control.iconId)} disabled={declarativeControlDisabled(control)} onClick={() => onAction(control.action)} className={activityClass} aria-busy={Boolean(activityClass) || undefined} />;
    }
  }
}
//#endregion DeclarativeControlBoundary

//#region ComponentRenderers
function activityBorderClass(record: UiNodeRecord): string | undefined {
  if (record.activity === "loading") return loadingBorderElementClass;
  if (record.activity === "waiting") return waitingBorderElementClass;
  return undefined;
}

function ContainerView({ store, record, context }: { readonly store: UiDocumentStore; readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "container" }>;
  const { props: aria, describedBy } = accessibilityAriaProps(record.accessibility, `node-${record.id}`);
  const presence = usePresenceOverlayEntry(record.key);
  const style = layoutSpecStyle(record.layout);
  const dataAttrs = styleSpecDataAttributes(record.style);
  const childIds = record.children ?? [];
  const childReactKeys = uiChildReactKeys(store.getState(), childIds);
  const children = childIds.map((childId, index) => <UiNodeView key={childReactKeys[index]} store={store} id={childId} context={context} />);
  const role = component.role === "form" ? "form" : component.role === "toolbar" ? "toolbar" : undefined;
  const activateBinding = (record.bindings ?? []).find((binding) => binding.trigger === "activate");

  // 🪪️ A section and a field carry the SAME stable DOM id every other interpreted node gets. Both
  // wrappers accept `id` and both dropped it here, so an app that authored `ui::section(...).try_id(…)`
  // / `ui::field(...).try_id(…)` — puzzle 3d's whole Settings panel does — rendered a tree whose only
  // addressable node was the innermost control, with no row or section to reach from a keybinding, an
  // introduction anchor, a scripted driver or assistive technology (ticket 26/09/02 wave B12).
  if (component.role === "section" || component.role === "group") {
    return (
      <Section id={nodeDomId(store, record)} title={component.label ? wireLabel(component.label) : undefined} className={cn(presence.selected && "ring-primary ring-1")}>
        {describedBy}
        {children}
      </Section>
    );
  }
  if (component.role === "field") {
    return (
      <Field id={nodeDomId(store, record)} label={component.label ? wireLabel(component.label) : ""} description={component.description ?? undefined} required={component.required ?? undefined} error={component.error ?? undefined}>
        {describedBy}
        {children}
      </Field>
    );
  }
  return (
    <div
      style={style}
      {...dataAttrs}
      {...aria}
      role={activateBinding ? "button" : role}
      id={nodeDomId(store, record)}
      data-ui-node-id={record.id} data-ui-node-key={record.key}
      data-activity={record.activity}
      className={cn(activityBorderClass(record), activateBinding && cn(borderElementClass, "border cursor-pointer rounded-md"), presence.selected && "ring-primary ring-1", presence.hovered && "outline-primary/50 outline-1")}
      aria-busy={record.activity === "loading" || record.activity === "waiting" || undefined}
      onClick={activateBinding ? (event) => { event.stopPropagation(); dispatchTrigger(context, record, "activate"); } : undefined}
    >
      {describedBy}
      {children}
    </div>
  );
}

function TextView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "text" }>;
  const { props: aria, describedBy } = accessibilityAriaProps(record.accessibility, `node-${record.id}`);
  return (
    <p className={cn("text-foreground", component.emphasize ? "font-semibold" : "text-sm")} data-ui-node-id={record.id} data-ui-node-key={record.key} {...aria}>
      {describedBy}
      {component.value}
    </p>
  );
}

function ButtonView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "button" }>;
  return (
    <Button
      id={nodeDomId(context.store, record)}
      data-ui-node-id={record.id} data-ui-node-key={record.key}
      text={component.label}
      icon={resolveControlIconNode(component.icon)}
      disabled={record.disabled}
      aria-label={record.accessibility.label ?? undefined}
      className={activityBorderClass(record)}
      aria-busy={record.activity === "loading" || record.activity === "waiting" || undefined}
      onClick={() => dispatchTrigger(context, record, "activate")}
    />
  );
}

/** 🖊️ A commit-on-blur input's LOCAL draft: what the user has typed but not yet committed.
 *
 * 🐛️ `InputProps.commit === "blur"` used to render a CONTROLLED input (`value={component.value}`) with
 * no `onChange` at all, which is React's read-only spelling: every keystroke was reverted on the next
 * render, so the field could not be edited, and the `blur` handler then committed the value the field
 * already had. Measured on 6018: typing "Balcony Study" into a generation's inline rename editor
 * dispatched `renameGeneration` and settled it, with the history label reading
 * `rename-generation id=generation-2 new-name="Generation 2"` — the OLD name
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `🗑️generated/generate-interactions/probe-2`). Every
 * commit-on-blur input in the fleet was uneditable for the same reason.
 *
 * The draft follows the published value whenever the GUEST changes it (the render-phase adjustment
 * React documents for derived state), so an outside edit still wins, while local typing is kept. */
function useCommitDraft(published: string): [string, (next: string) => void] {
  const [draft, setDraft] = useState(published);
  const publishedRef = useRef(published);
  if (publishedRef.current !== published) {
    publishedRef.current = published;
    if (draft !== published) setDraft(published);
  }
  return [draft, setDraft];
}

function InputView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "input" }>;
  const commitOnBlur = component.commit === "blur";
  const [draft, setDraft] = useCommitDraft(component.value);
  const lane = useContinuousTriggerLane(context, record);
  // 🎚️ A number field with no `commit` mode IS a continuous control: a held spinner, an arrow key on
  // repeat and a scripted value stream all emit a value per frame, and each one costs a whole
  // document round trip. It rides the same coalescing lane as a slider, and its blur is the release.
  const continuous = component.kind === "number" && !commitOnBlur;
  const commitValue = (raw: string) => {
    const value: UiValue = component.kind === "number" ? toUiValue(Number(raw)) : toUiValue(raw);
    if (continuous) {
      lane.offer(value);
      return;
    }
    dispatchTrigger(context, record, commitOnBlur ? "commit" : "change", value);
  };
  /** ⌨️ Enter commits without waiting for focus to leave — the gesture a user expects from an inline
   * editor, and the one a keyboard-only user has. */
  const commitOnEnter = (event: ReactKeyboardEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    if (event.key !== "Enter" || event.shiftKey) return;
    event.preventDefault();
    commitValue((event.target as HTMLInputElement | HTMLTextAreaElement).value);
    (event.target as HTMLInputElement | HTMLTextAreaElement).blur();
  };
  if (component.kind === "longText") {
    return (
      <Textarea
        id={nodeDomId(context.store, record)}
        data-ui-node-id={record.id} data-ui-node-key={record.key}
        className="min-h-[4.5rem] w-full min-w-0"
        value={commitOnBlur ? draft : component.value}
        placeholder={component.placeholder ?? undefined}
        onChange={commitOnBlur ? (event) => setDraft(event.target.value) : (event) => commitValue(event.target.value)}
        onKeyDown={commitOnBlur ? commitOnEnter : undefined}
        onBlur={commitOnBlur ? (event) => commitValue(event.target.value) : undefined}
      />
    );
  }
  const inputType = component.kind === "number" ? "number" : component.kind === "date" ? "date" : component.kind === "color" ? "color" : component.kind === "file" ? "file" : "text";
  return (
    <Input
      id={nodeDomId(context.store, record)}
      data-ui-node-id={record.id} data-ui-node-key={record.key}
      type={inputType}
      className="h-medium w-full min-w-0"
      value={component.kind === "file" ? undefined : commitOnBlur ? draft : component.value}
      placeholder={component.placeholder ?? undefined}
      min={component.min ?? undefined}
      max={component.max ?? undefined}
      step={component.step ?? undefined}
      accept={component.kind === "file" ? (component.accept ?? undefined) : undefined}
      onChange={commitOnBlur && component.kind !== "file" ? (event) => setDraft(event.target.value) : (event) => commitValue(component.kind === "file" ? (event.target.files?.[0]?.name ?? "") : event.target.value)}
      onKeyDown={commitOnBlur ? commitOnEnter : undefined}
      onBlur={commitOnBlur ? (event) => commitValue(component.kind === "file" ? (event.target.files?.[0]?.name ?? "") : event.target.value) : continuous ? (event) => lane.commit(toUiValue(Number(event.target.value))) : undefined}
    />
  );
}

function SelectView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "select" }>;
  return (
    <Select id={`${nodeDomId(context.store, record)}-select`} value={component.value || undefined} onValueChange={(value) => dispatchTrigger(context, record, "change", toUiValue(value))}>
      <SelectTrigger id={nodeDomId(context.store, record)} data-ui-node-id={record.id} data-ui-node-key={record.key} className="h-medium w-full min-w-0" size="sm">
        <SelectValue placeholder={component.placeholder ?? interpLabel("ui.common.select")} />
      </SelectTrigger>
      <SelectContent>
        {component.items.map((item, index) => (
          <SelectItem key={`${record.id}:${index}:${item.value}`} value={item.value}>
            {item.label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}

function ToggleView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "toggle" }>;
  return <Toggle id={nodeDomId(context.store, record)} data-ui-node-key={record.key} pressed={component.on} text={component.text ?? undefined} icon={resolveControlIconNode(component.icon)} onPressedChange={(pressed) => dispatchTrigger(context, record, "change", toUiValue(pressed))} />;
}

function KeyValueListView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "keyValueList" }>;
  return (
    <dl className="grid grid-cols-[auto_1fr] gap-x-single gap-y-single text-xs" data-ui-node-id={record.id} data-ui-node-key={record.key}>
      {component.entries.map((entry, index) => (
        <div key={`${entry.label}:${index}`} className="contents">
          <dt className="text-muted-foreground">{entry.label}</dt>
          <dd className="tabular-nums">{entry.value}</dd>
        </div>
      ))}
    </dl>
  );
}

function SliderView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "slider" }>;
  const lane = useContinuousTriggerLane(context, record);
  const slider = (
    <Slider
      id={nodeDomId(context.store, record)}
      data-ui-node-id={record.id} data-ui-node-key={record.key}
      className="w-full min-w-0"
      max={component.max}
      min={component.min}
      step={component.step}
      value={[component.value]}
      onValueChange={(values) => lane.offer(toUiValue(values[0] ?? component.value))}
      onValueCommit={(values) => lane.commit(toUiValue(values[0] ?? component.value))}
    />
  );
  if (!component.unit) return slider;
  return (
    <div className="flex min-w-0 w-full items-center gap-single">
      {slider}
      <span className="text-muted-foreground shrink-0 text-xs tabular-nums">
        {component.value} {component.unit}
      </span>
    </div>
  );
}

function NumberStepperView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "numberStepper" }>;
  return (
    <Stepper
      id={nodeDomId(context.store, record)}
      step={component.step}
      value={component.uniform ? component.value : undefined}
      mixed={!component.uniform}
      onChange={(value) => dispatchTrigger(context, record, "change", toUiValue(value))}
      // ➕️➖️ Only a node that DECLARES a `delta` binding gets the relative path. `Stepper`'s own
      // contract is "reports a relative delta via `onDelta` when provided, otherwise falls back to
      // computing an absolute `onChange`" — so supplying it unconditionally, as this did, sent every
      // +/− click down a trigger most programs never bind and swallowed the gesture entirely: puzzle
      // 3d's four Settings steppers all declare `Trigger::Change` only, and not one of their +/−
      // buttons reached the guest (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12, browser-measured).
      onDelta={(record.bindings ?? []).some((binding) => binding.trigger === "delta") ? (delta) => dispatchTrigger(context, record, "delta", toUiValue(delta)) : undefined}
    />
  );
}

function RingView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "ring" }>;
  return <Ring id={nodeDomId(context.store, record)} onOrbChange={(_orbId, _oldT, newT) => dispatchTrigger(context, record, "change", toUiValue(newT))} orbs={[{ disabled: record.disabled, id: component.orbId, selected: true, t: component.t }]} />;
}

function IconSelectView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "iconSelect" }>;
  return (
    <IconSelector
      classifyIconSelectorMode={component.classifierKind === "puzzle2d" ? classifyIconSelectorMode : undefined}
      id={nodeDomId(context.store, record)}
      onChange={(next) => dispatchTrigger(context, record, "change", toUiValue(next))}
      uniform={component.uniform}
      value={component.value}
    />
  );
}

//#region Tree
type TreeWalkNode = { readonly record: UiNodeRecord; readonly props: Extract<Component, { type: "treeItem" }> };

function collectTreeItems(state: UiDocumentState, ids: readonly UiNodeId[]): readonly TreeWalkNode[] {
  const out: TreeWalkNode[] = [];
  for (const id of ids) {
    const record = state.nodes.get(id);
    if (!record || record.component.type !== "treeItem") continue;
    out.push({ record, props: record.component });
  }
  return out;
}

/** 🎛️ Non-`treeItem` children of a tree item are its inline row controls (the History panel's
 * `framework.history.undo.run` button, the filter's select) — mounted through {@link UiNodeView}
 * into {@link TreeDataItem.control} so their own bindings stay live; `collectTreeItems` alone
 * silently dropped them (browser-measured 2026-09-10: `.run` never mounted, Undo row inert). */
function collectTreeItemControls(state: UiDocumentState, ids: readonly UiNodeId[]): readonly UiNodeRecord[] {
  const out: UiNodeRecord[] = [];
  for (const id of ids) {
    const record = state.nodes.get(id);
    if (!record || record.component.type === "treeItem") continue;
    out.push(record);
  }
  return out;
}

//#region 🪪️StableDomIds
/** 🪪️ THE stable DOM id of one retained UI node: the surface it belongs to, then the node's own
 * Rust-authored `key`. `UiNodeRecord.id` is a DFS-order integer re-minted on EVERY full-body
 * reconciliation (`builtNodeToSnapshot`, `📃️UiDocumentStore/🟦️.tsx`), so an id built from it names a
 * different row after the next refresh — a scripted or assistive click keyed on `#5` silently targets
 * the wrong node. `key` is authored by the program (`procedural3d-play-generate.add-generation`) and
 * survives every refresh, and the surface prefix is what namespaces it per window, since two windows of
 * one app can render the same authored key. Falls back to the volatile id only for a keyless node
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function uiNodeDomId(surface: SurfaceId, key: string, fallbackNodeId: UiNodeId): string {
  return key ? `${surface}/${key}` : `node-${fallbackNodeId}`;
}

/** 🪪️ {@link uiNodeDomId} for a record held by a live store — the surface comes off the store's own state. */
function nodeDomId(store: UiDocumentStore, record: UiNodeRecord): string {
  return uiNodeDomId(store.getState().surface, record.key, record.id);
}

/** 🪪️ React reconciliation keys for a sibling run — the SAME authored identity {@link uiNodeDomId}
 * already addresses a node by, applied to reconciliation instead of to the DOM.
 *
 * `builtNodeToSnapshot` mints `UiNodeId`s by pre-order DFS over the WHOLE body, so inserting one node
 * anywhere ahead of a sibling renumbers that sibling and everything after it. Keying children on that
 * number therefore turns any upstream shape change into a React key change — and a key change is an
 * unmount of the entire subtree, however unchanged it is. Measured on `window:procedural-main`: one
 * eval-status refresh grew the outline tree by four port rows (`profile@wire`,
 * `extrusion-axis@vectorOut`, `extrusion-axis@errors`, `extrude@solid`), which moved the node-graph
 * surface from id 30 to 34 and its container from 29 to 33, and React tore down and rebuilt the flow
 * host — a second wasm flow session, a second canvas, a second wasm-side surface, ~5.7 s of attach and
 * a graph that stayed blank for 39 s — for a surface node whose own content had not changed
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * A node's authored `key` survives renumbering, so it IS the key wherever it exists and is unique
 * among its siblings; a keyless or ambiguous sibling falls back to the minted id, which stays correct
 * (React only needs sibling-local uniqueness) and is simply not stable. The `k:`/`#` prefixes keep the
 * two namespaces from ever colliding. */
export function uiSiblingReactKeys(siblings: readonly { readonly id: UiNodeId; readonly key: string }[]): readonly string[] {
  const occurrences = new Map<string, number>();
  for (const sibling of siblings) if (sibling.key) occurrences.set(sibling.key, (occurrences.get(sibling.key) ?? 0) + 1);
  return siblings.map((sibling) => (sibling.key && occurrences.get(sibling.key) === 1 ? `k:${sibling.key}` : `#${sibling.id}`));
}

/** 🪪️ {@link uiSiblingReactKeys} for a child-id run resolved against a live document state. A child id
 * with no record keeps the id fallback rather than being dropped — the caller still renders it. */
export function uiChildReactKeys(state: UiDocumentState, children: readonly UiNodeId[]): readonly string[] {
  return uiSiblingReactKeys(children.map((id) => ({ id, key: state.nodes.get(id)?.key ?? "" })));
}
//#endregion 🪪️StableDomIds

/** 🌲️ One tree row's inline controls, reconciled on {@link uiSiblingReactKeys} like every other
 * sibling run — a row's control renumbers with the body exactly as its owning surface does. */
function renderTreeItemControls(store: UiDocumentStore, controls: readonly UiNodeRecord[], context: UiInterpreterContext): readonly ReactElement[] {
  const keys = uiSiblingReactKeys(controls);
  return controls.map((child, index) => <UiNodeView key={keys[index]} store={store} id={child.id} context={context} />);
}

//#region 🪟️TreeWindows
/** 🪟️ One container's on-screen row window, keyed by the AUTHORED node key the guest stamped on
 * `data-tree-window-key` — never a DOM id. The host's map has to survive a body refresh and a store
 * re-mint, and `UiNodeRecord.id` is renumbered by every `builtNodeToSnapshot` (see {@link uiNodeDomId}). */
export type TreeWindowReportV1 = { readonly nodeKey: string; readonly offset: number; readonly rows: number };

/** 🪟️ The per-panel-body channel a guest tree reports expansion and scroll through, provided by
 * `ShellHost` around each `InterpretedUiNode` (`uiNodeToTreePanelConfig`). Absent (the default `null`)
 * means "nobody is listening": the tree keeps `<Tree>`'s own uncontrolled per-mount open state and
 * measures nothing, which is exactly what a story, a fixture or a wgpu-side mount wants.
 *
 * `openStates` is keyed by WINDOW PATH for a windowed container ({@link treeWindowPathOf}: its enclosing
 * windowed containers' node keys and its own, joined by `TREE_WINDOW_PATH_SEPARATOR`) and by authored node key
 * for anything else — the same identity `setOpen` reports back and `ViewModel::tree_windows` carries to the
 * guest. Never the node key alone for a window: that string is also the pick target id and two containers
 * under different parents share it by design (📓️f2-sdk-body-node-ledger.md §10). `TreeView` translates to and
 * from `<Tree>`'s DOM ids ({@link uiNodeDomId}) at the boundary, so no host map ever holds a volatile id. */
export type TreeWindowContextValue = {
  readonly bodyKey: string;
  readonly openStates: Readonly<Record<string, boolean>>;
  readonly setOpen: (nodeKey: string, open: boolean) => void;
  readonly reportWindows: (requests: readonly TreeWindowReportV1[], viewportRows: number) => void;
};

export const TreeWindowContext = createContext<TreeWindowContextValue | null>(null);

/** 🪟️ The nearest {@link TreeWindowContext}, or `null` outside a host-provided panel body. */
export function useTreeWindowContext(): TreeWindowContextValue | null {
  return useContext(TreeWindowContext);
}

/** 🪟️ The ONE tree row pitch, read off the same `treeRowUiSpacing` design token the `🌳️Tree` element's
 * own `treeRowHeightPx` and the wgpu target's `TREE_ROW_HEIGHT` are computed from — a windowed tree is
 * a fixed-row-height virtualiser and all three renderers must agree on the pitch or the spacers and the
 * requested rows drift apart. Never zero, so the row arithmetic can never divide by it. */
export function treeWindowRowHeightPx(): number {
  const height = domSizePx("treeRowUiSpacing");
  return Number.isFinite(height) && height > 0 ? height : 1;
}

/** 🪟️ The bounded scroll container a guest tree actually scrolls inside — the ancestor `🖼️Panel`'s
 * `📜️Scrollable`, not the guest `<Tree>`'s own root div, whose `overflow-auto` never engages because the
 * chain above it is a natural-height stack (📓️audit-host-tree-pipeline.md §3).
 *
 * 🧯️ It is the element that CARRIES the overflow, `[data-slot="scroll-area"]`, never its inner
 * `[data-slot="scroll-area-viewport"]` content div: that inner div is unbounded
 * (`📜️Scrollable/🟦️.tsx:43` — `min-h-0 min-w-0 w-full`, no height), so its `scrollHeight` always equals
 * its `clientHeight` and its `scrollTop` is permanently `0`. Binding there measured 3720/3720 (extent 0)
 * against the real 466/3722 in a browser, which made every window request answer `offset: 0` and a
 * windowed tree could never stream past its first page (📓️w3-browser-verification.md §5).
 *
 * So the rule is behavioural, not a selector: collect every candidate scroller between the guest `<Tree>`'s
 * own root (it carries `overflow-auto` and IS the scroller wherever the chain above it bounds its height)
 * and the document, nearest first, then take
 * 1. the nearest one that ACTUALLY overflows (`scrollHeight − clientHeight > 1`) — the only proof,
 * 2. else the nearest `📜️Scrollable`, which is where the panel chain will scroll once the body has grown,
 * 3. else the nearest candidate at all, so a tree mounted outside a `📜️Scrollable` still streams,
 * 4. else the document's own scrolling element, for a tree on a plain scrolling page.
 * The effect re-runs on every store revision, so a first-paint body that overflows only after its rows
 * arrive is re-resolved against the grown document rather than measured forever against the wrong box. */
export function treeWindowScrollViewport(root: HTMLElement): HTMLElement | null {
  const view = root.ownerDocument?.defaultView ?? null;
  const isScroller = (candidate: HTMLElement): boolean => {
    if (candidate.getAttribute("data-slot") === "scroll-area") return true;
    const overflowY = view?.getComputedStyle(candidate).overflowY;
    return overflowY === "auto" || overflowY === "scroll" || overflowY === "overlay";
  };
  const candidates: HTMLElement[] = [];
  // 🌳️ `TreeView` wraps `<Tree>` in a `display: contents` div, so the tree's own root is the first
  // element child of `root` and belongs in the chain — skipping it would miss a bounded guest tree.
  const inner = root.firstElementChild;
  if (inner instanceof HTMLElement && isScroller(inner)) candidates.push(inner);
  for (let candidate = root.parentElement; candidate; candidate = candidate.parentElement) if (isScroller(candidate)) candidates.push(candidate);
  return candidates.find((candidate) => candidate.scrollHeight - candidate.clientHeight > 1) ?? candidates.find((candidate) => candidate.getAttribute("data-slot") === "scroll-area") ?? candidates[0] ?? treeWindowDocumentScroller(root);
}

/** 🪟️ The page itself, for a tree that has no scrolling ancestor at all. `scrollingElement` is the element
 * whose `scrollTop` a page scroll moves; the `scroll` event for it fires on the DOCUMENT, which is why
 * {@link useTreeWindowObserver} listens on the window too whenever this is the viewport. */
export function treeWindowDocumentScroller(root: HTMLElement): HTMLElement | null {
  const owner = root.ownerDocument ?? null;
  const scrolling = owner?.scrollingElement ?? owner?.documentElement ?? null;
  return scrolling instanceof HTMLElement ? scrolling : null;
}

/** 🪟️ The viewport's own visible box, in the client coordinates every `getBoundingClientRect` already
 * speaks: where its CONTENT box starts on screen, and how tall the part of it a reader can see is.
 *
 * 🧯️ `clientHeight`, never `getBoundingClientRect().height` — the rect is the BORDER box and includes the
 * element's borders and (when it has one) a horizontal scrollbar, so a rect-based `viewportRows` reports
 * rows the user cannot see and a rect-based overlap runs a row past the bottom edge. `clientTop` is the
 * top border, the offset between the rect's top and the first pixel of scrollable content.
 *
 * 🧯️ The document's scrolling element is the one box whose own rect MOVES with the scroll (its `top` is
 * `−scrollY`), so its content origin is the client origin `0` and its visible height is the window's, not
 * its own `clientHeight` (which is the whole document on a quirks-mode page). */
export function treeWindowViewportMetrics(viewport: HTMLElement): { readonly originTop: number; readonly height: number } {
  const owner = viewport.ownerDocument ?? null;
  if (viewport === owner?.scrollingElement || viewport === owner?.documentElement) return { originTop: 0, height: Math.max(0, owner?.defaultView?.innerHeight ?? viewport.clientHeight) };
  const rect = viewport.getBoundingClientRect();
  return { originTop: rect.top + viewport.clientTop, height: Math.max(0, viewport.clientHeight || rect.height) };
}

/** 🪟️ Every windowed container under `root`, measured RELATIVE TO the viewport's content origin — i.e.
 * `top: 0` is the first visible pixel of the scroll container — in the shape
 * `treeWindowRequestsForViewport` consumes with `viewportTop: 0`.
 *
 * Client coordinates, not scroll-content coordinates: only the DIFFERENCE `viewportTop − top` and the
 * container extents reach the rule, both are identical in either space, and this one needs no `scrollTop`
 * bookkeeping — which is what made the document scroller (whose own rect moves with the scroll) and any
 * bordered scroller misreport. A container with `total <= 0` is not windowed and is skipped: reporting it
 * would ask the guest for rows that do not exist. */
export function treeWindowContainersUnder(root: HTMLElement, viewport: HTMLElement): readonly TreeWindowContainerMeasure[] {
  const originTop = treeWindowViewportMetrics(viewport).originTop;
  const measured: TreeWindowContainerMeasure[] = [];
  for (const element of Array.from(root.querySelectorAll("[data-tree-window-key]"))) {
    if (!(element instanceof HTMLElement)) continue;
    // 🪟️ The window's IDENTITY is its path; `-key` is the authored node key, which is also the pick target
    // id and is legitimately shared by containers under different parents. `-key` is the fallback only for a
    // top-level container, where the two are the same string anyway.
    const key = element.getAttribute("data-tree-window-path") || element.getAttribute("data-tree-window-key");
    if (!key) continue;
    const total = treeWindowAttributeNumber(element, "data-tree-window-total");
    if (total <= 0) continue;
    const rect = element.getBoundingClientRect();
    // 🧯️ A CLOSED section still renders its content element, `hidden`, so its rect is zero — it is not on
    // screen, it materialises nothing, and measuring it would ask for rows nobody is looking at.
    if (rect.height <= 0) continue;
    measured.push({ key, total, offset: treeWindowAttributeNumber(element, "data-tree-window-offset"), length: treeWindowAttributeNumber(element, "data-tree-window-length"), top: rect.top - originTop, height: rect.height, rows: treeWindowRowsUnder(element, originTop) });
  }
  return measured;
}

/** 📐️ One container's OWN materialised rows, as `{index, top}` in the container's own space. This is what
 * replaces "pixels ÷ one row height" inside a container: a materialised row that is itself an open windowed
 * group is many rows tall, and only its real rect says where the row after it begins
 * (📓️s3-review-streaming-loop.md §2).
 *
 * 🧯️ Ownership is `closest("[data-tree-window-key]")`, not `:scope >` — a row with a context menu is wrapped
 * in one and would vanish from a direct-child query, while a nested group's own rows must stay with that
 * group. Sorted by TOP, not by index: the position rule searches on pixels, and a `direction === "up"` tree
 * paints the slice reversed. */
export function treeWindowRowsUnder(container: HTMLElement, originTop: number): readonly TreeWindowRowMeasure[] {
  const rows: TreeWindowRowMeasure[] = [];
  for (const element of Array.from(container.querySelectorAll("[data-tree-window-row]"))) {
    if (!(element instanceof HTMLElement)) continue;
    if (element.closest("[data-tree-window-key]") !== container) continue;
    const index = Number(element.getAttribute("data-tree-window-row"));
    if (!Number.isFinite(index) || index < 0) continue;
    rows.push({ index: Math.floor(index), top: element.getBoundingClientRect().top - originTop });
  }
  rows.sort((left, right) => left.top - right.top || left.index - right.index);
  return rows;
}

function treeWindowAttributeNumber(element: HTMLElement, attribute: string): number {
  const parsed = Number(element.getAttribute(attribute) ?? "0");
  return Number.isFinite(parsed) && parsed > 0 ? Math.floor(parsed) : 0;
}

/** 🪟️ The one line a report is compared on — the host is refreshed only when the ANSWER changed, not
 * every time a scroll frame recomputed the same answer (a panel scrolled one pixel inside a row still
 * wants exactly the rows it already has). */
export function treeWindowReportSignatureV1(requests: readonly TreeWindowReportV1[], viewportRows: number): string {
  return `${viewportRows}|${requests.map((request) => `${request.nodeKey}:${request.offset}:${request.rows}`).join(",")}`;
}

/** 🪟️ The rows an off-screen container that has already materialised something is asked for. Zero: it is in
 * the body, so it costs its own node in the guest's ledger either way, but nobody is looking at its rows and
 * every one of them would be a row a VISIBLE container does not get. It keeps the offset it already has, so
 * scrolling back to it lands where the reader left it rather than at row 0 (📓️s3-review-streaming-loop.md §3). */
const TREE_WINDOW_OFFSCREEN_ROWS = 0;

/** 🌱️ …but an off-screen container that has materialised NOTHING yet is asked for one row, not zero.
 *
 * 🧯️ A default-open container below the fold is answered `rows: 0` on first paint, and `rows: 0` renders as an
 * open container with a full-height spacer and no rows — indistinguishable, to a reader, from an empty or
 * collapsed one, wearing the pending ring for as long as it stays off screen (the cad lane's
 * `structure-classic`, 0/11 at a 440 px viewport). One row costs one node, proves the container is not empty
 * and clears the ring; the real window arrives the moment it is scrolled into view. */
const TREE_WINDOW_OFFSCREEN_SEED_ROWS = 1;

/** 🪟️ Every windowed container of one body, priced together: the ones the viewport covers with the rows
 * they need, the rest as spacer-only windows at the offset they already hold. The whole set goes through
 * {@link capTreeWindowRequests} because the guest's ledger charges `1 + rows` for EVERY container in the
 * body, on-screen or not — pricing only the visible ones is what made the host ask 112 and be answered 110
 * on every render (📓️s3-review-streaming-loop.md §1). */
export function treeWindowBodyRequestsV1(containers: readonly TreeWindowContainerMeasure[], viewportHeight: number, rowHeightPx: number): readonly TreeWindowRequest[] {
  const visible = treeWindowVisibleRowsForViewport(containers, 0, viewportHeight, rowHeightPx);
  const wanted = new Map(treeWindowRequestsForViewport(containers, 0, viewportHeight, rowHeightPx, TREE_WINDOW_OVERSCAN_ROWS).map((request) => [request.key, request] as const));
  const requests = containers.map((container) => {
    const known = wanted.get(container.key);
    if (known) return known;
    const total = Math.max(0, Math.floor(container.total));
    const rows = Math.floor(container.length) > 0 ? TREE_WINDOW_OFFSCREEN_ROWS : Math.min(TREE_WINDOW_OFFSCREEN_SEED_ROWS, total);
    return { key: container.key, offset: Math.min(Math.max(0, Math.floor(container.offset)), Math.max(0, total - rows)), rows };
  });
  return capTreeWindowRequests(requests, visible, TREE_WINDOW_BODY_NODE_BUDGET);
}

/** 🔑️ A duplicate `data-tree-window-key` inside one body is an authoring fault, not a host one: two
 * containers would share one open state and one window, and each other's measurements would silently
 * overwrite the other's in the report (📓️s3-review-streaming-loop.md §4). The host cannot repair it — the
 * key is what the guest addresses its containers by — so it says so, loudly, once per body. */
function reportDuplicateTreeWindowKeys(containers: readonly TreeWindowContainerMeasure[], bodyKey: string, reported: Set<string>): void {
  const seen = new Set<string>();
  for (const container of containers) {
    if (!seen.has(container.key)) {
      seen.add(container.key);
      continue;
    }
    const once = `${bodyKey} ${container.key}`;
    if (reported.has(once)) continue;
    reported.add(once);
    console.error(`[tree-window] duplicate key ${JSON.stringify(container.key.split(TREE_WINDOW_PATH_SEPARATOR).join(" › "))} in panel body ${JSON.stringify(bodyKey)} — two windowed containers share one window path, so they share one window and one open state; give sibling containers distinct node keys`);
  }
}

/** 🪟️ Measures the windowed containers under `rootRef` against their scroll viewport and reports the
 * rows the guest should materialise, coalesced to one measurement per animation frame and re-run on
 * every `scroll`, on a viewport resize, and on every store revision AND store identity (a body refresh
 * both bumps the revision and — when the panel config cache misses — hands `TreeView` a whole new
 * `UiDocumentStore` whose revision restarts at its own snapshot's, which a revision-only dependency can
 * read as "unchanged" and never re-measure the tree that just replaced the old one). The diff against
 * the last report is what keeps a scroll gesture from firing one partial refresh per frame, and what
 * makes a SETTLED window silent: the answer is a function of the container geometry alone, and the
 * geometry a window's own answer produces is the geometry that asked for it. */
function useTreeWindowObserver(rootRef: RefObject<HTMLDivElement | null>, windows: TreeWindowContextValue | null, revision: unknown, store: unknown): void {
  const windowsRef = useRef<TreeWindowContextValue | null>(windows);
  windowsRef.current = windows;
  const lastReportRef = useRef<string>("");
  const duplicateKeysRef = useRef<Set<string>>(new Set());
  const frameRef = useRef<number | null>(null);
  const bodyKey = windows?.bodyKey ?? null;
  useEffect(() => {
    const root = rootRef.current;
    if (!root || !windowsRef.current) return;
    const viewport = treeWindowScrollViewport(root);
    if (!viewport) return;
    const view = root.ownerDocument?.defaultView;
    if (!view) return;
    const measure = () => {
      frameRef.current = null;
      const channel = windowsRef.current;
      const live = rootRef.current;
      if (!channel || !live || !live.isConnected) return;
      const rowHeight = treeWindowRowHeightPx();
      // 🪟️ Client space with the viewport's content origin at 0 — see `treeWindowContainersUnder`.
      const viewportHeight = treeWindowViewportMetrics(viewport).height;
      const containers = treeWindowContainersUnder(live, viewport);
      reportDuplicateTreeWindowKeys(containers, channel.bodyKey, duplicateKeysRef.current);
      const requests = treeWindowBodyRequestsV1(containers, viewportHeight, rowHeight).map((request) => ({ nodeKey: request.key, offset: request.offset, rows: request.rows }));
      const viewportRows = Math.max(1, Math.ceil(viewportHeight / rowHeight));
      const signature = treeWindowReportSignatureV1(requests, viewportRows);
      if (signature === lastReportRef.current) return;
      lastReportRef.current = signature;
      channel.reportWindows(requests, viewportRows);
    };
    const schedule = () => {
      if (frameRef.current !== null) return;
      frameRef.current = typeof view.requestAnimationFrame === "function" ? view.requestAnimationFrame(measure) : (view.setTimeout(measure, 0) as unknown as number);
    };
    schedule();
    viewport.addEventListener("scroll", schedule, { passive: true });
    // 🧯️ A page scroll does not fire on the scrolling ELEMENT — it fires on the document — so a tree that
    // has no scrolling ancestor of its own would never re-measure without this second listener.
    const pageScroller = viewport === treeWindowDocumentScroller(root) ? view : null;
    pageScroller?.addEventListener("scroll", schedule, { passive: true });
    const observer = typeof view.ResizeObserver === "function" ? new view.ResizeObserver(schedule) : null;
    observer?.observe(viewport);
    // 🪟️ The tree's own extent moves under a fixed viewport too (a sibling branch folding, a nested window
    // streaming in), and that changes which rows this one must show. Measuring the root as well is what
    // makes a nested container re-report without waiting for the next store revision.
    if (observer && root !== viewport) observer.observe(root.firstElementChild instanceof HTMLElement ? root.firstElementChild : root);
    return () => {
      viewport.removeEventListener("scroll", schedule);
      pageScroller?.removeEventListener("scroll", schedule);
      observer?.disconnect();
      if (frameRef.current === null) return;
      if (typeof view.cancelAnimationFrame === "function") view.cancelAnimationFrame(frameRef.current);
      else view.clearTimeout(frameRef.current);
      frameRef.current = null;
    };
  }, [rootRef, revision, store, bodyKey]);
}

/** 🕹️ What one conversion pass of a guest tree collects and carries down the recursion — every field is
 * filled WHILE walking, so a row's own click closure can read the whole tree's pick table (a range pick
 * resolves ids the walk had not reached yet when that row was converted). */
export type TreeWalkContextV1 = {
  /** 🪟️ Host-owned expansion, keyed by window path (windowed containers) or authored node key (everything else). */
  readonly openStates?: Readonly<Record<string, boolean>>;
  /** 🪟️ Filled while walking: the same expansion re-keyed onto `<Tree>`'s DOM ids. */
  readonly domOpenStates?: Record<string, boolean>;
  /** 🪟️ Filled while walking: DOM id → authored key, so an `onOpenStateChange` maps back. */
  readonly keysByDomId?: Map<string, string>;
  /** 🕹️ The tree root's own `activate` binding — the SDK's tree-level `interactionSelect`, bound once by
   * `PanelTreeBuilder::interaction_domain`, which is what makes a bare `granularity` row a pick target
   * that costs zero argument arena. */
  readonly pick?: { readonly record: UiNodeRecord; readonly binding: ActionBinding };
  /** 🕹️ Filled while walking: DOM id → the interaction target that row stands for. */
  readonly pickTargets?: Map<string, { readonly key: string; readonly granularity: string }>;
};

/** 🕹️ The interaction targets one pick dispatches. A plain pick is the clicked row alone; a `range`
 * pick is the `<Tree>`'s resolved selection (already including the clicked row — `handleSelectItem`
 * hands `onClick` the NEXT selection, not the previous one) mapped through the walk's table, with the
 * clicked row appended as the floor so a selection the table cannot resolve still picks something.
 * Deduplicated on `(granularity, id)`, exactly as `world3dSelectionActionArgs` deduplicates ids. */
export function treePickTargetsV1(walk: TreeWalkContextV1, key: string, granularity: string, merge: MergeMode, selectedIds: readonly string[]): readonly { readonly granularity: string; readonly id: string }[] {
  if (merge !== "range") return [{ granularity, id: key }];
  const seen = new Set<string>();
  const targets: { readonly granularity: string; readonly id: string }[] = [];
  const push = (granularityId: string, id: string) => {
    const dedupe = `${granularityId} ${id}`;
    if (seen.has(dedupe)) return;
    seen.add(dedupe);
    targets.push({ granularity: granularityId, id });
  };
  for (const domId of selectedIds) {
    const entry = walk.pickTargets?.get(domId);
    if (entry) push(entry.granularity, entry.key);
  }
  push(granularity, key);
  return targets;
}

/** 🕹️ The `UiValue` map a tree pick puts on the wire — the SAME shape `world3dSelectionActionArgs`
 * builds (`targets` a JSON string of `{granularity, id}` records, `method: "pick"`, `merge` a raw
 * `MergeMode` word the framework's `parse_merge_mode` accepts verbatim), minus `domainId`: the tree's
 * own `Activate` binding already carries `{domainId}` in its authored args, and `uiIntentPayload`
 * merges this map OVER them. `"range"` has no ordered topology on the wire, so a range gesture is
 * resolved host-side into the full id set and sent as a `"replace"`. */
export function treePickIntentInputV1(merge: MergeMode, targets: readonly { readonly granularity: string; readonly id: string }[]): UiValue {
  return { merge: merge === "range" ? "replace" : merge, method: "pick", targets: JSON.stringify(targets.map((target) => ({ granularity: target.granularity, id: target.id }))) } as unknown as UiValue;
}

function dispatchTreePick(context: UiInterpreterContext, walk: TreeWalkContextV1, key: string, granularity: string, event: ReactMouseEvent, activation: TreeDataActivationContext): void {
  const pick = walk.pick;
  if (!pick) return;
  const merge = interactionMergeFromModifiers(event);
  void context.onIntent(context.store.buildIntent(pick.record, pick.binding, treePickIntentInputV1(merge, treePickTargetsV1(walk, key, granularity, merge, activation.selectedIds))));
}

/** 🔑️ `🌳️Tree` keys its expansion map by a ROLE-PREFIXED form of the row's DOM id
 * (`getTreeSectionStateId` → `tree-section-<id>`, `getTreeItemStateId` → `tree-item-<id>`; both are
 * private to that element and neither is exported). A host-controlled map therefore carries EVERY
 * spelling of a row it knows about and strips the prefix on the way back. Unambiguous because a
 * {@link uiNodeDomId} is `<surface>/<key>` and a panel surface is always `panel:…`, so no DOM id can
 * itself begin with one of these prefixes. */
const TREE_OPEN_STATE_ID_PREFIXES = ["tree-section-", "tree-item-"] as const;

export function treeOpenStateIdsForDomIdV1(domId: string): readonly string[] {
  return [domId, ...TREE_OPEN_STATE_ID_PREFIXES.map((prefix) => `${prefix}${domId}`)];
}

export function treeDomIdFromOpenStateIdV1(stateId: string): string {
  for (const prefix of TREE_OPEN_STATE_ID_PREFIXES) if (stateId.startsWith(prefix)) return stateId.slice(prefix.length);
  return stateId;
}

/** 🪟️ Records one converted row's identity in the walk's translation tables. `identity` is the row's WINDOW
 * PATH when it is a windowed container and its authored key otherwise — the same string the host's open map,
 * its window map and `ViewModel::tree_windows` are keyed by, and the one an `onOpenStateChange` reports back. */
function registerTreeWalkRow(walk: TreeWalkContextV1 | undefined, identity: string, domId: string): void {
  if (!walk || !identity) return;
  const key = identity;
  walk.keysByDomId?.set(domId, key);
  const open = walk.openStates?.[key];
  if (open === undefined || !walk.domOpenStates) return;
  for (const stateId of treeOpenStateIdsForDomIdV1(domId)) walk.domOpenStates[stateId] = open;
}
//#endregion 🪟️TreeWindows

export function treeItemToTreeData(store: UiDocumentStore, state: UiDocumentState, node: TreeWalkNode, context: UiInterpreterContext, overlay: UiPresenceOverlayValue, leftoverIds?: readonly string[], walk?: TreeWalkContextV1, parentWindowPath?: string): TreeDataItem {
  const { record, props } = node;
  // 🪟️ A WINDOW is identified by its path; an unwindowed row has no window identity and keeps its authored
  // key, which is the only thing the host ever holds for it (a fold).
  const windowPath = props.window ? treeWindowPathOf(parentWindowPath, record.key) : undefined;
  // 🔑️ …and the window's ROW id is the path too, because `🌳️Tree` keys its expansion map by row id and the
  // authored key is legitimately shared across a body (`📐️cad`'s one `object.id` under four pane sections):
  // two rows under one id would fold together. Identical to the key for a top-level container, so nothing
  // flat changes; `walk.pickTargets` still resolves this id back to the bare `record.key`, so picks do not
  // notice (📓️f2-sdk-body-node-ledger.md §10).
  const domId = uiNodeDomId(state.surface, windowPath ?? record.key, record.id);
  registerTreeWalkRow(walk, windowPath ?? record.key, domId);
  const presence = overlay.byKey.get(record.key) ?? {};
  const activateBinding = (record.bindings ?? []).find((b) => b.trigger === "activate");
  const hoverBinding = (record.bindings ?? []).find((b) => b.trigger === "hoverPreview");
  const childItems = collectTreeItems(state, record.children ?? []);
  const controlRecords = collectTreeItemControls(state, record.children ?? []);
  const activatableControl = activateBinding ? undefined : controlRecords.find((child) => !child.disabled && (child.bindings ?? []).some((b) => b.trigger === "activate"));
  const granularity = typeof props.granularity === "string" && props.granularity.length > 0 ? props.granularity : undefined;
  if (granularity && walk?.pick) walk.pickTargets?.set(domId, { key: record.key, granularity });
  const pickClick = !activateBinding && granularity && walk?.pick ? (event: ReactMouseEvent, activation: TreeDataActivationContext) => dispatchTreePick(context, walk, record.key, granularity, event, activation) : undefined;
  return {
    id: domId,
    window: props.window ?? undefined,
    windowKey: record.key || undefined,
    windowPath,
    label: props.label,
    description: props.description,
    icon: props.icon ? resolveControlIconNode(props.icon, 12) : undefined,
    defaultOpen: props.defaultOpen ?? undefined,
    isSelected: Boolean(presence.selected) || leftoverTreeItemSelectedV1(record.key, leftoverIds),
    isHighlighted: presence.hovered === true || presence.previewed === true ? true : undefined,
    loading: record.activity === "loading",
    waiting: record.activity === "waiting",
    isHidden: props.dimmed ?? undefined,
    draggable: props.draggable ?? undefined,
    dragData: props.dragData ? (Object.fromEntries(Object.entries(props.dragData).filter((entry): entry is [string, string] => entry[1] !== undefined)) as Record<string, string>) : undefined,
    control: controlRecords.length > 0 && controlRecords.length !== (activatableControl ? 1 : 0) ? <>{renderTreeItemControls(store, controlRecords.filter((child) => child !== activatableControl), context)}</> : undefined,
    items: childItems.length > 0 ? childItems.map((child) => treeItemToTreeData(store, state, child, context, overlay, leftoverIds, walk, windowPath ?? parentWindowPath)) : undefined,
    onClick: activateBinding ? () => dispatchTrigger(context, record, "activate") : (pickClick ?? (activatableControl ? () => dispatchTrigger(context, activatableControl, "activate") : undefined)),
    onPointerEnter: hoverBinding ? () => dispatchTrigger(context, record, "hoverPreview") : undefined,
    actions: (props.rowActions ?? []).length > 0 ? (props.rowActions ?? []).map((action) => ({ kind: "button" as const, icon: resolveControlIconNode(action.icon, 12), title: action.label ? wireLabel(action.label) : undefined, placement: action.placement ?? "row", onClick: () => context.onIntent(context.store.buildIntent(record, action.action)) })) : undefined,
  };
}

function TreeView({ store, record, context }: { readonly store: UiDocumentStore; readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const revision = useUiDocumentRevision(store);
  const overlay = useUiPresenceOverlay();
  const leftover = useSyncExternalStore(subscribeLeftoverWorldSelectionV1, leftoverWorldSelectionOverlayV1, leftoverWorldSelectionOverlayV1);
  const leftoverIds = leftover?.ids;
  const windows = useTreeWindowContext();
  const rootRef = useRef<HTMLDivElement | null>(null);
  const walked = useMemo((): { readonly sections: TreeDataSection[]; readonly walk: TreeWalkContextV1 } => {
    void revision;
    const state = store.getState();
    const rootActivate = (record.bindings ?? []).find((b) => b.trigger === "activate");
    const walk: TreeWalkContextV1 = {
      openStates: windows?.openStates,
      domOpenStates: {},
      keysByDomId: new Map<string, string>(),
      pick: rootActivate ? { record, binding: rootActivate } : undefined,
      pickTargets: new Map<string, { readonly key: string; readonly granularity: string }>(),
    };
    const sectionRecords = (record.children ?? []).map((id) => state.nodes.get(id)).filter((r): r is UiNodeRecord => !!r && r.component.type === "treeSection");
    const sections = sectionRecords.map((sectionRecord) => {
      const sectionProps = sectionRecord.component as Extract<Component, { type: "treeSection" }>;
      const items = collectTreeItems(state, sectionRecord.children ?? []);
      const domId = uiNodeDomId(state.surface, sectionRecord.key, sectionRecord.id);
      // 🪟️ A section is top-level, so its window path IS its key — a flat body is unchanged by path identity.
      const windowPath = sectionProps.window ? treeWindowPathOf(undefined, sectionRecord.key) : undefined;
      registerTreeWalkRow(walk, windowPath ?? sectionRecord.key, domId);
      return {
        id: domId,
        window: sectionProps.window ?? undefined,
        windowKey: sectionRecord.key || undefined,
        windowPath,
        label: sectionProps.label ?? "",
        defaultOpen: sectionProps.defaultOpen ?? undefined,
        loading: sectionRecord.activity === "loading",
        waiting: sectionRecord.activity === "waiting",
        items: items.map((item) => treeItemToTreeData(store, state, item, context, overlay, leftoverIds, walk, windowPath)),
      };
    });
    return { sections, walk };
  }, [store, record, revision, context, overlay, leftoverIds, windows]);
  const sections = walked.sections;
  // 🔑️ `TreeSection`/`TreeItem` each route a fold through the provider TWICE — once from the data view
  // that owns the controlled `open` prop and once from the row component's own `useTreeOpenState`, both
  // under the same state id. The host's map is idempotent (`createTreeWindowSchedulerV1.setOpen` returns
  // on an unchanged value), so the duplicate costs nothing and is deliberately not filtered here.
  const handleOpenStateChange = useCallback((stateId: string, open: boolean) => windows?.setOpen(walked.walk.keysByDomId?.get(treeDomIdFromOpenStateIdV1(stateId)) ?? stateId, open), [windows, walked]);
  useTreeWindowObserver(rootRef, windows, revision, store);
  const dragController: TreeDragAndDropController | undefined = useMemo(() => {
    const dropBinding = (record.bindings ?? []).find((b) => b.trigger === "drop");
    const catalogueMime = treeSectionsCatalogueDragMime(sections);
    const catalogue = catalogueMime ? catalogueTreeDragController(catalogueMime) : undefined;
    if (!dropBinding && !catalogue) return undefined;
    return {
      ...(catalogue ?? {}),
      ...(dropBinding ? { handleDrop: () => dispatchTrigger(context, record, "drop") } : {}),
    };
  }, [record, context, sections]);
  // 🪟️ `display: contents` — the wrapper exists ONLY to give the window observer a DOM handle on the
  // tree it must measure (`<Tree>` exposes no ref). It generates no box, so the `Panel`/`Scrollable`
  // layout chain above and the `Tree` root's own classes below are byte-for-byte what they were.
  return (
    <div ref={rootRef} className="contents">
      <Tree
        className="min-h-0 min-w-0 flex-1 overflow-auto"
        sections={sections.length > 0 ? sections : [treeStatusSection(store, record)]}
        selectionMode="single"
        showLines
        dragAndDropController={dragController}
        sortableSections={sections.length > 1}
        {...(windows ? { openStates: walked.walk.domOpenStates, onOpenStateChange: handleOpenStateChange } : {})}
      />
    </div>
  );
}

/** @emoji 🦴 The one section a tree with NO resolvable `treeSection` child renders, so "still loading"
 * and "genuinely nothing to show" are never the same blank rectangle.
 *
 * 🧯️ A panel body the shell has not received yet is `pendingPanelUiNode()` — a `tree` node with
 * `activity: "loading"` and no children (`🖥️platform/🟦️.ts`) — and `<Tree sections={[]}/>` draws
 * exactly nothing for it, which is how the inspection panel presented as an empty panel in the browser
 * (measured 2026-09-09 21:05) while `refreshUi` reported ok. The root record's own `activity` is the
 * contract's declared mechanism for this state, and it was the one thing this view dropped: a
 * `treeSection` carries `loading`/`waiting` for exactly this purpose. An idle tree with no sections is
 * a real, rendered empty body and says so instead. */
function treeStatusSection(store: UiDocumentStore, record: UiNodeRecord): TreeDataSection {
  const loading = record.activity === "loading";
  const waiting = record.activity === "waiting";
  const status = loading || waiting ? interpLabel("ui.common.loadingSurface") : interpLabel("ui.common.noData");
  return { id: `${nodeDomId(store, record)}-status`, label: status, defaultOpen: true, loading, waiting, items: [], emptyState: status };
}
//#endregion Tree

function ImageView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "image" }>;
  return <img id={`node-${record.id}`} src={component.src} alt={component.alt ?? ""} className="max-h-64 max-w-full rounded-md object-contain" data-ui-node-id={record.id} data-ui-node-key={record.key} />;
}

/** 🚚️ A surface whose scene declares out-of-doc payload lanes (`world-3d`, `canvas-2d`, `board-2d`). Its `doc.bytes` carry only
 * the spine; the lanes are retained text-leaf subtrees hanging off this very node, so this view — and
 * only this view — subscribes to the whole document's revision: a lane leaf changing does NOT change
 * this node's own record, and a tree larger than one reconcile page arrives across several patches. */
function surfaceCarrierEpoch(store: UiDocumentStore, record: UiNodeRecord): string {
  const state = store.getState();
  const stack = [...(record.children ?? [])];
  const parts: string[] = [];
  while (stack.length > 0) {
    const id = stack.pop()!;
    const child = state.nodes.get(id);
    if (!child) continue;
    parts.push(`${id}:${String(child.key)}:${child.component.type === "text" && typeof child.component.value === "string" ? child.component.value.length : 0}`);
    for (const nested of child.children ?? []) stack.push(nested);
  }
  return parts.join("|");
}

function useSurfaceCarrierEpoch(store: UiDocumentStore, record: UiNodeRecord): string {
  const childKey = (record.children ?? []).join(",");
  return useSyncExternalStore(
    (onChange) => {
      const state = store.getState();
      const ids: number[] = [];
      const stack = [...(record.children ?? [])];
      while (stack.length > 0) {
        const id = stack.pop()!;
        ids.push(id);
        const child = state.nodes.get(id);
        for (const nested of child?.children ?? []) stack.push(nested);
      }
      const unsubs = ids.map((id) => store.subscribeNode(id)(onChange));
      return () => {
        for (const unsub of unsubs) unsub();
      };
    },
    () => surfaceCarrierEpoch(store, record),
    () => surfaceCarrierEpoch(store, record),
  );
}

function PagedSurfaceView({ record, component, context }: { readonly record: UiNodeRecord; readonly component: Extract<Component, { type: "surface" }>; readonly context: UiInterpreterContext }) {
  const store = context.store;
  const revision = useUiDocumentRevision(store);
  const carrierEpoch = useSurfaceCarrierEpoch(store, record);
  const lanes = (component.kind === "canvas-2d" ? CANVAS2D_SCENE_LANES : component.kind === "board-2d" ? BOARD2D_SCENE_LANES : component.kind === "tiled-map" ? TILEDMAP_SCENE_LANES : WORLD3D_SCENE_LANES) as readonly SceneLane<Record<string, unknown>>[];
  const assemble = useCallback(
    (spine: Record<string, unknown>): Record<string, unknown> => {
      void revision;
      void carrierEpoch;
      const declared = Array.isArray(spine.lanes) ? (spine.lanes as readonly SceneLaneRef[]) : [];
      return sceneFromLanes(spine, surfaceLaneTexts(record, store.getState(), declared, lanes), lanes);
    },
    [record, store, revision, carrierEpoch, lanes],
  );
  return <>{renderComponentSceneHost(record, component, context.onAction, store.getState().surface, context.requestContextMenu, assemble)}</>;
}

/**
 * @emoji ♿️ The accessible door to a canvas.
 *
 * Every other component view reaches its ARIA for free, because it renders a real HTML element that
 * already carries the role and takes focus. A surface renders a scene host that paints into a
 * `<canvas>` — an opaque texture with no accessible children at all — so without this wrapper the
 * node-graph and World3d surfaces were unnamed, unfocusable and silent: the `AccessibilitySpec` the
 * contract carries on the record was simply dropped on the floor (`renderComponentSceneHost` never
 * received it), which an accessibility-tree probe on 6018 confirmed — three canvases, no `role`, no
 * `aria-label`, no `tabindex`.
 *
 * `role="application"` is the ARIA contract for "this widget handles its own arrow/Enter keys", which
 * is exactly what a scene host does, and it is the role `accessibility_role` already implies for
 * `Component::Surface` on every renderer. `tabIndex` puts the canvas in the Tab order, matching the
 * contract's own `accessibility_is_focusable`, so React and the wgpu `EventRouter` agree about who is
 * reachable. The layout classes mirror what every scene host's own root already sets
 * (`relative h-full min-h-0 w-full`), so inserting this element changes no geometry.
 *
 * @see ../../../../../../🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs
 */
function SurfaceAccessibilityShell({ record, context, children }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext; readonly children: ReactNode }) {
  const { props: aria, describedBy } = accessibilityAriaProps(record.accessibility, `node-${record.id}`);
  const activateBinding = (record.bindings ?? []).find((binding) => binding.trigger === "activate");
  return (
    <div
      data-ui-node-id={record.id}
      data-ui-node-key={record.key}
      data-ui-surface-shell=""
      role="application"
      tabIndex={record.disabled ? -1 : 0}
      aria-disabled={record.disabled || undefined}
      aria-busy={record.activity === "loading" || record.activity === "waiting" || undefined}
      className="focus-visible:ring-primary relative h-full min-h-0 w-full min-w-0 outline-none focus-visible:ring-2"
      onKeyDown={
        activateBinding && !record.disabled
          ? (event) => {
              if (event.key !== "Enter" && event.key !== " ") return;
              event.preventDefault();
              dispatchTrigger(context, record, "activate");
            }
          : undefined
      }
      {...aria}
    >
      {describedBy}
      {children}
    </div>
  );
}

function SurfaceView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "surface" }>;
  const body = component.kind === "world-3d" || component.kind === "canvas-2d" || component.kind === "board-2d" || component.kind === "tiled-map" ? <PagedSurfaceView record={record} component={component} context={context} /> : <>{renderComponentSceneHost(record, component, context.onAction, context.store.getState().surface, context.requestContextMenu)}</>;
  return (
    <SurfaceAccessibilityShell record={record} context={context}>
      {body}
    </SurfaceAccessibilityShell>
  );
}

/** 📶️ `Component::Progress` → a real `role="progressbar"`. Determinate bars carry
 * `aria-valuemin`/`max`/`now`/`valuetext` and a fill sized by the shared fraction law; indeterminate bars
 * carry only `aria-busy` and a centred sweep that pulses only when the user has not asked for reduced
 * motion (`motion-safe:`). */
function ProgressView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "progress" }>;
  const value = uiAccessibilityValueV1(component);
  const fraction = uiProgressFractionV1(component.completed, component.total);
  const { props: aria, describedBy } = accessibilityAriaProps(record.accessibility, `node-${record.id}`);
  return (
    <div
      role="progressbar"
      data-ui-node-id={record.id}
      data-ui-node-key={record.key}
      aria-valuemin={value.valueMin ?? undefined}
      aria-valuemax={value.valueMax ?? undefined}
      aria-valuenow={value.valueNow ?? undefined}
      aria-valuetext={value.valueText ?? undefined}
      aria-busy={value.busy ? true : undefined}
      {...aria}
      className="bg-muted h-tiny w-full min-w-0 overflow-hidden rounded-full"
    >
      {describedBy}
      <div data-slot="progress-fill" className={fraction === null ? "bg-accent mx-auto h-full w-1/3 motion-safe:animate-pulse" : "bg-accent h-full"} style={fraction === null ? undefined : { width: `${fraction * 100}%` }} />
    </div>
  );
}

function ExtensionView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "extension" }>;
  return (
    <ShellFaultBoundary boundaryId={`extension-${component.extension}`} fallbackLabel={shellLabel("ui.common.renderError")}>
      <p className="text-muted-foreground text-xs" data-ui-node-id={record.id} data-ui-node-key={record.key}>
        Extension unavailable: {component.extension}
      </p>
    </ShellFaultBoundary>
  );
}

/** 🚧️ An unregistered/unknown `Component::type` — never renders nothing (a silent blank is the
 * failure mode that makes a missing renderer look like a broken document, per the packet brief). */
function UnknownComponentView({ record }: { readonly record: UiNodeRecord }) {
  const kind = (record.component as { readonly type: string }).type;
  console.error(`Interpreter: unknown component type ${JSON.stringify(kind)} on node ${record.id} ("${record.key}")`);
  return (
    <div role="alert" className="border-destructive text-destructive rounded-md border border-dashed p-single text-xs" data-ui-node-id={record.id} data-ui-node-key={record.key} data-unknown-component={kind}>
      Unrecognized component: {kind}
    </div>
  );
}
//#endregion ComponentRenderers

function interpretUiNodeBusyShell(record: UiNodeRecord): ReactNode | null {
  if (record.component.type === "progress" || (record.activity !== "loading" && record.activity !== "waiting")) return null;
  return (
    <div data-ui-node-id={record.id} data-ui-node-key={record.key} data-ui-status={record.activity} className={cn("p-single w-full min-w-0", record.activity === "waiting" ? waitingBorderElementClass : loadingBorderElementClass)} role="status" aria-busy="true">
      {elementSkeleton(record.component.type as ElementSkeletonKind)}
    </div>
  );
}

/** 🌳️ Renders one record's component, recursing into children via {@link UiNodeView} — never reads a
 * child's own record directly, only its id, so a child's change never re-renders this switch. */
function renderComponent(store: UiDocumentStore, record: UiNodeRecord, context: UiInterpreterContext): ReactNode {
  const busyShell = interpretUiNodeBusyShell(record);
  if (busyShell) return busyShell;
  switch (record.component.type) {
    case "container":
      return <ContainerView store={store} record={record} context={context} />;
    case "text":
      return <TextView record={record} />;
    case "button":
      return <ButtonView record={record} context={context} />;
    case "separator":
      return <hr className={cn("border-0", borderNormalTopClass)} data-ui-node-id={record.id} data-ui-node-key={record.key} />;
    case "input":
      return <InputView record={record} context={context} />;
    case "select":
      return <SelectView record={record} context={context} />;
    case "toggle":
      return <ToggleView record={record} context={context} />;
    case "keyValueList":
      return <KeyValueListView record={record} />;
    case "slider":
      return <SliderView record={record} context={context} />;
    case "numberStepper":
      return <NumberStepperView record={record} context={context} />;
    case "ring":
      return <RingView record={record} context={context} />;
    case "iconSelect":
      return <IconSelectView record={record} context={context} />;
    case "progress":
      return <ProgressView record={record} />;
    case "tree":
      return <TreeView store={store} record={record} context={context} />;
    case "treeSection":
    case "treeItem":
      // 🌲️ Reached only when a section/item is rendered OUTSIDE a `tree` parent (malformed document) —
      // `TreeView` walks these directly, never through `UiNodeView`, in the well-formed case.
      return <ContainerView store={store} record={record} context={context} />;
    case "image":
      return <ImageView record={record} />;
    case "surface":
      return <SurfaceView record={record} context={context} />;
    case "extension":
      return <ExtensionView record={record} />;
    default:
      return <UnknownComponentView record={record} />;
  }
}

/** 🌳️ The atomic per-node subscribing unit — reads exactly `id`'s own record via `useUiNode`, so it
 * re-renders when (and only when) THAT record changes. */
export function UiNodeView({ store, id, context }: { readonly store: UiDocumentStore; readonly id: UiNodeId; readonly context: UiInterpreterContext }): ReactElement | null {
  const record = useUiNode(store, id);
  if (!record) return null;
  return <>{renderComponent(store, record, context)}</>;
}

/** 🌳️ Entry point — resolves `store`'s current root and renders it. */
export function interpretUiNode(store: UiDocumentStore, context: UiInterpreterContext): ReactNode {
  const root = store.getState().root;
  if (root === null) return null;
  return <UiNodeView store={store} id={root} context={context} />;
}

/**
 * @emoji 🐢️ `React.memo`'d entry point — `store` is a stable per-surface identity, so only the root id
 * changing (a `SetRoot`/full `loadSnapshot`) causes this to re-subscribe; ordinary node mutations are
 * handled entirely by `UiNodeView`'s own per-id subscription several levels down, never by
 * re-rendering from here.
 */
export const InterpretedUiNode = memo(function InterpretedUiNode({ store, onAction, onIntent, requestContextMenu }: { readonly store: UiDocumentStore } & Pick<UiInterpreterContext, "onAction" | "onIntent" | "requestContextMenu">): ReactNode {
  // 🖱️ The shell publishes the plugin's on-demand menu resolver through `PluginSurfaceActionsContext`
  // (ShellHost's `requestContextMenu`), and NO `<InterpretedUiNode>` call site has ever passed it as a
  // prop — so every `ComponentSceneHost`'s `requestContextMenu` was `undefined` and its whole
  // plugin-context-menu branch (`openSurfaceContextMenu`) was dead code in the React renderer: a
  // right-click on a world/board/canvas surface only ever produced ShellHost's window-level fallback
  // menu. Reading the context here wires every surface host at once, which is what that context is for.
  const surfaceActions = usePluginSurfaceActions();
  const root = useUiDocumentRoot(store);
  if (root === null) return null;
  return <UiNodeView store={store} id={root} context={{ store, onAction, onIntent, requestContextMenu: requestContextMenu ?? surfaceActions }} />;
});
//#endregion 🔖️UiInterpreter

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️unknown-component-placeholder/🟦️.tsx");
  await registerTests1(import.meta.vitest, { DEFAULT_UI_DOCUMENT_LIMITS, Profiler, UiDocumentStore, UiNodeView, accessibilityAriaProps }, { directory: import.meta.dir, url: import.meta.url });
  const { registerTests1: registerContainerNodeIdTests } = await import("./🧪️tests/🪪️container-node-ids/🟦️.tsx");
  await registerContainerNodeIdTests(import.meta.vitest, { UiDocumentStore, UiNodeView, uiChildReactKeys, uiSiblingReactKeys }, { url: import.meta.url });
  const { registerTests1: registerTreeWindowTests } = await import("./🧪️tests/🪟️tree-windows/🟦️.tsx");
  await registerTreeWindowTests(import.meta.vitest, { TreeWindowContext, UiDocumentStore, UiNodeView, treeItemToTreeData, treePickIntentInputV1, treePickTargetsV1, treeWindowBodyRequestsV1, treeWindowContainersUnder, treeWindowRowHeightPx, treeWindowScrollViewport, treeWindowViewportMetrics }, { url: import.meta.url });
  const { registerTests1: registerProgressTests } = await import("./🧪️tests/📶️progress/🟦️.tsx");
  await registerProgressTests(import.meta.vitest, { UiDocumentStore, UiNodeView }, { url: import.meta.url });
  const { registerTests1: registerOverlayFlowTests } = await import("./🧪️tests/📐️overlay-flow/🟦️.tsx");
  await registerOverlayFlowTests(import.meta.vitest, { UiDocumentStore, UiNodeView, layoutSpecStyle }, { url: import.meta.url });
  const { registerTests1: registerSurfaceSceneLaneTests } = await import("./🧪️tests/🚚️surface-scene-lanes/🟦️.tsx");
  await registerSurfaceSceneLaneTests(
    import.meta.vitest,
    {
      UiDocumentStore,
      UiNodeView,
      surfaceSceneLaneText,
      surfaceSceneLaneCache,
      utf8ByteLength,
      world3dSurfaceLaneTexts: (record: UiNodeRecord, state: UiDocumentState, declared: readonly SceneLaneRef[]) => surfaceLaneTexts(record, state, declared, WORLD3D_SCENE_LANES),
      canvas2dSurfaceLaneTexts: (record: UiNodeRecord, state: UiDocumentState, declared: readonly SceneLaneRef[]) => surfaceLaneTexts(record, state, declared, CANVAS2D_SCENE_LANES),
      board2dSurfaceLaneTexts: (record: UiNodeRecord, state: UiDocumentState, declared: readonly SceneLaneRef[]) => surfaceLaneTexts(record, state, declared, BOARD2D_SCENE_LANES),
      world3dSceneFromLanes,
      canvas2dSceneFromLanes,
      board2dSceneFromLanes,
      WORLD3D_SCENE_LANES,
      CANVAS2D_SCENE_LANES,
      BOARD2D_SCENE_LANES,
      WORLD3D_SCENE_LANE_KEY_PREFIX,
      world3dSceneLaneForBodyKey,
    },
    { url: import.meta.url },
  );
}
//#endregion 🧪️Tests
