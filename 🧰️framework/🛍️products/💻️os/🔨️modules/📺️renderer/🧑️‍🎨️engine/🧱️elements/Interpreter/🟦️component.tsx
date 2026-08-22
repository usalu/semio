/// <reference types="vitest/importMeta" />
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/Interpreter/component.tsx
/** @emoji 🌳️ `Interpreter` — turns the semantic UI contract (`semio-framework-ui-contract`'s
 * `UiNodeRecord`/`Component`/`LayoutSpec`/`StyleSpec`/`AccessibilitySpec`) retained by a
 * `UiDocumentStore` into `@semio-tech/ui-react` components. `InterpretedUiNode` is the entry point;
 * `UiNodeView` is the atomic per-node unit — each one subscribes to exactly its own record via
 * `useUiNode`, so a `SetComponent` on one node re-renders exactly that node's component, never its
 * ancestors or siblings (the whole payoff of the flat id-keyed table, see `UiDocumentStore`'s header
 * doc). Also owns the `ComponentSceneHost` registry (lazily mounts `canvas-2d`/`world-3d`/etc. surface
 * hosts behind `Component::Surface`) and the shared per-surface context-menu flow — both unchanged
 * from the pre-migration renderer, since neither depends on the old `UiNode` shape. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { createContext, memo, Profiler, useContext, useMemo, useState, type ComponentType, type CSSProperties, type ReactElement, type ReactNode } from "react";
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
  Textarea,
  Toggle,
  Tree,
  VirtualFileSystem,
  borderElementClass,
  borderNormalTopClass,
  catalogueTreeDragController,
  classifyIconSelectorMode,
  cn,
  elementSkeleton,
  loadingBorderElementClass,
  renderControlIcon,
  useLabel,
  waitingBorderElementClass,
  type ContextMenuItem,
  type ElementSkeletonKind,
  type IconName,
  type TreeDataItem,
  type TreeDataSection,
  type TreeDragAndDropController,
  type TreePanelConfig,
  type UiLabel,
  type UiTranslationKey,
} from "@semio-tech/ui-react";
import { uiSpacingRem } from "@semio-tech/ui-styling";
import {
  type ActionDescriptor,
  type ComponentKind,
  type ComponentSceneHostProps,
  type ContextMenuItemSpec,
  type PluginContextMenuRequest,
  type UiComponentSceneNode,
  type UiMenuRef,
} from "@semio-tech/framework";
import {
  DEFAULT_UI_DOCUMENT_LIMITS,
  UiDocumentStore,
  emitIntent,
  useUiDocumentRevision,
  useUiDocumentRoot,
  useUiNode,
  type UiDocumentState,
} from "../UiDocumentStore/🟦️component.tsx";
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
  type StackLayout,
  type StyleSpec,
  type PatchRejection,
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
import { decodePackValue, decodeScenePackField } from "@semio-tech/framework-os";
import { shellLabel } from "../ShellHelpers/🟦️component.tsx";
import { useMapContextMenuSpecs } from "../ShellHost/🟦️component.tsx";
import { ShellFaultBoundary } from "../Shell/🟦️component.tsx";
import { WindowInstanceIdContext, World3dHost } from "../World3dHost/🟦️component.tsx";
import { NodeGraphHost } from "../NodeGraph/🟦️component.tsx";
import { TextEditorHost } from "../TextEditor/🟦️component.tsx";
import { TableHost } from "../Table/🟦️component.tsx";
import { Paint2dHost } from "../Paint2dHost/🟦️component.tsx";
import { TiledMapHost } from "../TiledMapHost/🟦️component.tsx";
import { Board2dHost } from "../Board2dHost/🟦️component.tsx";
import { IconRenderHost } from "../IconRenderHost/🟦️component.tsx";
import { InkCanvasHost } from "../InkCanvasHost/🟦️component.tsx";
import { GraphTimelineHost } from "../GraphTimelineHost/🟦️component.tsx";
import { BlockListHost } from "../BlockListHost/🟦️component.tsx";
import { DiffViewHost } from "../DiffViewHost/🟦️component.tsx";
import { EventFeedHost } from "../EventFeedHost/🟦️component.tsx";
// 🐢️ Direct element-to-element import — `Interpreter` and `Canvas2dHost` landed in the same batch.
import { Canvas2dHost } from "../Canvas2dHost/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️UiInterpreter
//#region PresenceOverlay
/** 👥️ This session's own hover/selection/preview state on one node, keyed by `UiNodeRecord.key` (NOT
 * `UiNodeId` — presence must land on the right element across a reconciliation that reassigns ids but
 * keeps keys stable, mirroring `crate::PresenceUpdate`'s own doc). Populated from `PresenceUpdate`
 * wire messages by whoever owns the transport (host-side, outside this element); never derived from
 * or written into the `UiDocumentStore` — presence changes at input frequency and must never touch a
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
 * the `ui-w4-core` mirror regeneration) — the record's own `id` is the stable per-node identity now,
 * so it substitutes for both; `paneId`/`bindingId` have no contract equivalent and are simply absent
 * (both optional on `UiComponentSceneNode`). Returns `null`, never throws, on a malformed `docSchema`
 * or a decode failure — the caller renders a placeholder + logs the fault, per this ticket's own
 * "never throw, never drop the surrounding patch" rule for an unknown `doc_schema`. */
function surfacePropsToComponentSceneNode(record: UiNodeRecord, props: SurfaceProps): UiComponentSceneNode | null {
  if (!isWellFormedDocSchema(props.docSchema)) {
    console.error("[Interpreter] malformed Component::Surface docSchema", { nodeId: record.id, kind: props.kind, docSchema: props.docSchema });
    return null;
  }
  let decoded: Record<string, unknown> | undefined;
  try {
    // 🧭️ `SurfaceDoc.bytes` is `Vec<u8>` — owned schema exporter renders it as a plain `number[]`, not a `Uint8Array`
    // or the old `"pk:"`-prefixed string `decodeScenePackField` (used by `parseSceneJsonField` below
    // for still-string-shaped scene sub-fields) expects. Raw bytes decode via `decodePackValue`.
    decoded = props.doc.bytes.length > 0 ? (decodePackValue(new Uint8Array(props.doc.bytes)) as Record<string, unknown>) : undefined;
  } catch (error) {
    console.error("[Interpreter] failed to decode Component::Surface doc bytes", { nodeId: record.id, kind: props.kind, docSchema: props.docSchema, error });
    return null;
  }
  const sceneField = SURFACE_KIND_SCENE_FIELD[props.kind];
  const node: Record<string, unknown> = {
    type: "componentScene",
    surfaceId: String(record.id),
    controllerId: String(record.id),
    componentKind: props.kind,
    menu: menuRefFromContract(record.menu),
  };
  if (sceneField && decoded !== undefined) node[sceneField] = decoded;
  return node as unknown as UiComponentSceneNode;
}

function renderComponentSceneHost(record: UiNodeRecord, props: SurfaceProps, onAction: (action: ActionDescriptor) => void, requestContextMenu?: UiInterpreterContext["requestContextMenu"]): ReactNode {
  const node = surfacePropsToComponentSceneNode(record, props);
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
    console.log("[DEBUG] resolveComponentSceneHost miss", props.kind);
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
//#endregion SurfaceBridge

//#region UiInterpreterContext
export type UiInterpreterContext = {
  readonly store: UiDocumentStore;
  /** 🌉️ Legacy ActionDescriptor channel — the seam this Interpreter still speaks to the 14 unowned
   * scene-host elements through (see `🌉️SurfaceBridge`). Semantic components (button/input/select/…)
   * never use this; they go through `emitIntent`/`UiIntent` instead. */
  readonly onAction: (action: ActionDescriptor) => void;
  /** 🎬️ Semantic dispatch — fires a `UiIntent` built from the node's own `ActionBinding`s. */
  readonly onIntent: (intent: UiIntent) => void;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
};
//#endregion UiInterpreterContext

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
  const specs = requestContextMenu ? await requestContextMenu(request) : [];
  return {
    items: specs.length > 0 ? mapSpecs(specs) : (shellFallback?.() ?? []),
    titleKey: surfaceContextMenuTitleKey(request),
  };
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
      return { position: "absolute", inset: edgeSpaceToPadding(l.inset) };
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
function accessibilityAriaProps(spec: AccessibilitySpec, idBase: string): { readonly props: Record<string, unknown>; readonly describedBy?: ReactNode } {
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

function dispatchTrigger(context: UiInterpreterContext, record: UiNodeRecord, trigger: UiTrigger, input?: UiValue): void {
  const intent = emitIntent(context.store, record, trigger, input);
  if (intent) context.onIntent(intent);
}

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
        <Select value={control.value || undefined} onValueChange={(value) => dispatchDeclarativeControlAction(onAction, control.onChange, { value })}>
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
  const style: CSSProperties = { ...layoutSpecStyle(record.layout), position: record.layout.kind === "overlay" ? "relative" : undefined };
  const dataAttrs = styleSpecDataAttributes(record.style);
  const children = (record.children ?? []).map((childId) => <UiNodeView key={childId} store={store} id={childId} context={context} />);
  const role = component.role === "form" ? "form" : component.role === "toolbar" ? "toolbar" : undefined;
  const activateBinding = (record.bindings ?? []).find((binding) => binding.trigger === "activate");

  if (component.role === "section" || component.role === "group") {
    return (
      <Section title={component.label ? wireLabel(component.label) : undefined} className={cn(presence.selected && "ring-primary ring-1")}>
        {describedBy}
        {children}
      </Section>
    );
  }
  if (component.role === "field") {
    return (
      <Field label={component.label ? wireLabel(component.label) : ""} description={component.description ?? undefined} required={component.required ?? undefined} error={component.error ?? undefined}>
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
      data-ui-node-id={record.id}
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
    <p className={cn("text-foreground", component.emphasize ? "font-semibold" : "text-sm")} data-ui-node-id={record.id} {...aria}>
      {describedBy}
      {component.value}
    </p>
  );
}

function ButtonView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "button" }>;
  return (
    <Button
      id={`node-${record.id}`}
      data-ui-node-id={record.id}
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

function InputView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "input" }>;
  const commitOnBlur = component.commit === "blur";
  const commitValue = (raw: string) => {
    const value: UiValue = component.kind === "number" ? toUiValue(Number(raw)) : toUiValue(raw);
    dispatchTrigger(context, record, commitOnBlur ? "commit" : "change", value);
  };
  if (component.kind === "longText") {
    return (
      <Textarea
        id={`node-${record.id}`}
        data-ui-node-id={record.id}
        className="min-h-[4.5rem] w-full min-w-0"
        value={component.value}
        placeholder={component.placeholder ?? undefined}
        onChange={commitOnBlur ? undefined : (event) => commitValue(event.target.value)}
        onBlur={commitOnBlur ? (event) => commitValue(event.target.value) : undefined}
      />
    );
  }
  const inputType = component.kind === "number" ? "number" : component.kind === "date" ? "date" : component.kind === "color" ? "color" : component.kind === "file" ? "file" : "text";
  return (
    <Input
      id={`node-${record.id}`}
      data-ui-node-id={record.id}
      type={inputType}
      className="h-medium w-full min-w-0"
      value={component.kind === "file" ? undefined : component.value}
      placeholder={component.placeholder ?? undefined}
      min={component.min ?? undefined}
      max={component.max ?? undefined}
      step={component.step ?? undefined}
      accept={component.kind === "file" ? (component.accept ?? undefined) : undefined}
      onChange={commitOnBlur ? undefined : (event) => commitValue(component.kind === "file" ? (event.target.files?.[0]?.name ?? "") : event.target.value)}
      onBlur={commitOnBlur ? (event) => commitValue(component.kind === "file" ? (event.target.files?.[0]?.name ?? "") : event.target.value) : undefined}
    />
  );
}

function SelectView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "select" }>;
  return (
    <Select id={`node-${record.id}-select`} value={component.value || undefined} onValueChange={(value) => dispatchTrigger(context, record, "change", toUiValue(value))}>
      <SelectTrigger id={`node-${record.id}`} data-ui-node-id={record.id} className="h-medium w-full min-w-0" size="sm">
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
  return <Toggle id={`node-${record.id}`} pressed={component.on} text={component.text ?? undefined} icon={resolveControlIconNode(component.icon)} onPressedChange={(pressed) => dispatchTrigger(context, record, "change", toUiValue(pressed))} />;
}

function KeyValueListView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "keyValueList" }>;
  return (
    <dl className="grid grid-cols-[auto_1fr] gap-x-single gap-y-single text-xs" data-ui-node-id={record.id}>
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
  const slider = (
    <Slider
      id={`node-${record.id}`}
      data-ui-node-id={record.id}
      className="w-full min-w-0"
      max={component.max}
      min={component.min}
      step={component.step}
      value={[component.value]}
      onValueChange={(values) => dispatchTrigger(context, record, "change", toUiValue(values[0] ?? component.value))}
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
      id={`node-${record.id}`}
      step={component.step}
      value={component.uniform ? component.value : undefined}
      mixed={!component.uniform}
      onChange={(value) => dispatchTrigger(context, record, "change", toUiValue(value))}
      onDelta={(delta) => dispatchTrigger(context, record, "delta", toUiValue(delta))}
    />
  );
}

function RingView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "ring" }>;
  return <Ring id={`node-${record.id}`} onOrbChange={(_orbId, _oldT, newT) => dispatchTrigger(context, record, "change", toUiValue(newT))} orbs={[{ disabled: record.disabled, id: component.orbId, selected: true, t: component.t }]} />;
}

function IconSelectView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "iconSelect" }>;
  return (
    <IconSelector
      classifyIconSelectorMode={component.classifierKind === "puzzle2d" ? classifyIconSelectorMode : undefined}
      id={`node-${record.id}`}
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

function treeItemToTreeData(state: UiDocumentState, node: TreeWalkNode, context: UiInterpreterContext, overlay: UiPresenceOverlayValue): TreeDataItem {
  const { record, props } = node;
  const presence = overlay.byKey.get(record.key) ?? {};
  const activateBinding = (record.bindings ?? []).find((b) => b.trigger === "activate");
  const hoverBinding = (record.bindings ?? []).find((b) => b.trigger === "hoverPreview");
  const childItems = collectTreeItems(state, record.children ?? []);
  return {
    id: String(record.id),
    label: props.label,
    description: props.description,
    icon: props.icon ? resolveControlIconNode(props.icon, 12) : undefined,
    defaultOpen: props.defaultOpen ?? undefined,
    isSelected: presence.selected ?? false,
    loading: record.activity === "loading",
    waiting: record.activity === "waiting",
    isHidden: props.dimmed ?? undefined,
    draggable: props.draggable ?? undefined,
    dragData: props.dragData ? (Object.fromEntries(Object.entries(props.dragData).filter((entry): entry is [string, string] => entry[1] !== undefined)) as Record<string, string>) : undefined,
    items: childItems.length > 0 ? childItems.map((child) => treeItemToTreeData(state, child, context, overlay)) : undefined,
    onClick: activateBinding ? () => dispatchTrigger(context, record, "activate") : undefined,
    onPointerEnter: hoverBinding ? () => dispatchTrigger(context, record, "hoverPreview") : undefined,
    actions: props.rowActions.length > 0 ? props.rowActions.map((action) => ({ kind: "button" as const, icon: resolveControlIconNode(action.icon, 12), title: action.label ? wireLabel(action.label) : undefined, placement: action.placement ?? "row", onClick: () => context.onIntent(context.store.buildIntent(record, action.action)) })) : undefined,
  };
}

function TreeView({ store, record, context }: { readonly store: UiDocumentStore; readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const revision = useUiDocumentRevision(store);
  const overlay = useContext(UiPresenceOverlayContext);
  const sections = useMemo((): TreeDataSection[] => {
    void revision;
    const state = store.getState();
    const sectionRecords = (record.children ?? []).map((id) => state.nodes.get(id)).filter((r): r is UiNodeRecord => !!r && r.component.type === "treeSection");
    return sectionRecords.map((sectionRecord) => {
      const sectionProps = sectionRecord.component as Extract<Component, { type: "treeSection" }>;
      const items = collectTreeItems(state, sectionRecord.children ?? []);
      return {
        id: String(sectionRecord.id),
        label: sectionProps.label ?? "",
        defaultOpen: sectionProps.defaultOpen ?? undefined,
        loading: sectionRecord.activity === "loading",
        waiting: sectionRecord.activity === "waiting",
        items: items.map((item) => treeItemToTreeData(state, item, context, overlay)),
      };
    });
  }, [store, record, revision, context, overlay]);
  const dragController: TreeDragAndDropController | undefined = useMemo(() => {
    const dropBinding = (record.bindings ?? []).find((b) => b.trigger === "drop");
    if (!dropBinding) return undefined;
    return { handleDrop: () => dispatchTrigger(context, record, "drop") };
  }, [record, context]);
  return (
    <Tree className="min-h-0 min-w-0 flex-1 overflow-auto" sections={sections} selectionMode="single" showLines dragAndDropController={dragController} sortableSections={sections.length > 1} />
  );
}
//#endregion Tree

function ImageView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "image" }>;
  return <img id={`node-${record.id}`} src={component.src} alt={component.alt ?? ""} className="max-h-64 max-w-full rounded-md object-contain" data-ui-node-id={record.id} />;
}

function SurfaceView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "surface" }>;
  return <>{renderComponentSceneHost(record, component, context.onAction, context.requestContextMenu)}</>;
}

function ExtensionView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "extension" }>;
  return (
    <ShellFaultBoundary boundaryId={`extension-${component.extension}`} fallbackLabel={shellLabel("ui.common.renderError")}>
      <p className="text-muted-foreground text-xs" data-ui-node-id={record.id}>
        Extension unavailable: {component.extension}
      </p>
    </ShellFaultBoundary>
  );
}

/** 🚧️ An unregistered/unknown `Component::type` — never renders nothing (a silent blank is the
 * failure mode that makes a missing renderer look like a broken document, per the packet brief). */
function UnknownComponentView({ record }: { readonly record: UiNodeRecord }) {
  const kind = (record.component as { readonly type: string }).type;
  console.error(`[DEBUG] Interpreter: unknown component type ${JSON.stringify(kind)} on node ${record.id} ("${record.key}")`);
  return (
    <div role="alert" className="border-destructive text-destructive rounded-md border border-dashed p-single text-xs" data-ui-node-id={record.id} data-unknown-component={kind}>
      Unrecognized component: {kind}
    </div>
  );
}
//#endregion ComponentRenderers

function interpretUiNodeBusyShell(record: UiNodeRecord): ReactNode | null {
  if (record.activity !== "loading" && record.activity !== "waiting") return null;
  return (
    <div data-ui-node-id={record.id} data-ui-status={record.activity} className={cn("p-single w-full min-w-0", record.activity === "waiting" ? waitingBorderElementClass : loadingBorderElementClass)} role="status" aria-busy="true">
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
      return <hr className={cn("border-0", borderNormalTopClass)} data-ui-node-id={record.id} />;
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
  const root = useUiDocumentRoot(store);
  if (root === null) return null;
  return <UiNodeView store={store} id={root} context={{ store, onAction, onIntent, requestContextMenu }} />;
});
//#endregion 🔖️UiInterpreter

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;

  const TEST_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
  const TEST_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };

  function leaf(id: number, key: string, component: Component, children: readonly number[] = []): UiNodeRecord {
    return { id, key, component, layout: { kind: "leaf", width: "hug", height: "hug" }, style: TEST_STYLE, activity: "idle", disabled: false, transition: null, accessibility: TEST_ACCESSIBILITY, bindings: [], menu: null, children: [...children] };
  }

  function snapshot(root: number, nodes: readonly UiNodeRecord[]): UiSnapshot {
    return { surface: "s", revision: 0, root, nodes: [...nodes], layoutEpoch: 0n };
  }

  const noopContext: UiInterpreterContext = { store: new UiDocumentStore("noop"), onAction: () => {}, onIntent: () => {} };

  describe("unknown component placeholder", () => {
    it("renders a visible placeholder and never nothing for an unregistered component type", async () => {
      const { render, cleanup } = await import("@semio-tech/ui-react/test");
      const store = new UiDocumentStore("s");
      store.loadSnapshot(snapshot(0, [leaf(0, "root", { type: "future-widget" } as unknown as Component)]));
      const errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});
      const { container } = render(<UiNodeView store={store} id={0} context={{ ...noopContext, store }} />);
      expect(container.querySelector("[data-unknown-component]")).not.toBeNull();
      expect(container.textContent).toMatch(/Unrecognized component/);
      expect(errorSpy).toHaveBeenCalled();
      errorSpy.mockRestore();
      cleanup();
    });
  });

  describe("per-node render granularity (React level)", () => {
    it("re-renders only the component whose own record changed", async () => {
      const { act, render, cleanup } = await import("@semio-tech/ui-react/test");
      const store = new UiDocumentStore("s");
      // 🧭️ `root` owns `a`/`b` as real document children (required — every node must be reachable
      // from the root or `validateUiDocumentCore` rejects the whole document as `danglingRoot`), but
      // this test deliberately mounts `a`/`b` as two INDEPENDENT top-level `UiNodeView` trees rather
      // than rendering `root` and letting `ContainerView` recurse into them — mounting `root` too
      // would nest `a`/`b` a second time inside it, and `root`'s own `Profiler` would then correctly
      // fire on any descendant commit, which is not what this test measures.
      store.loadSnapshot(
        snapshot(0, [
          leaf(0, "root", { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }, [1, 2]),
          leaf(1, "a", { type: "text", value: "A", emphasize: null, dataAttributes: null }),
          leaf(2, "b", { type: "text", value: "B", emphasize: null, dataAttributes: null }),
        ]),
      );

      const aRenders = vi.fn();
      const bRenders = vi.fn();
      const context: UiInterpreterContext = { ...noopContext, store };
      const { unmount } = render(
        <>
          <Profiler id="a" onRender={aRenders}>
            <UiNodeView store={store} id={1} context={context} />
          </Profiler>
          <Profiler id="b" onRender={bRenders}>
            <UiNodeView store={store} id={2} context={context} />
          </Profiler>
        </>,
      );
      aRenders.mockClear();
      bRenders.mockClear();

      act(() => {
        const result = store.applyPatch({ surface: "s", baseRevision: 0, revision: 1, ops: [{ type: "setComponent", id: 1, component: { type: "text", value: "A changed", emphasize: null, dataAttributes: null } }] });
        expect(result.ok).toBe(true);
      });

      expect(aRenders).toHaveBeenCalledTimes(1);
      expect(bRenders).toHaveBeenCalledTimes(0);
      unmount();
      cleanup();
    });
  });

  //#region CorpusConformance
  /** 🧪️ Consumes the shared conformance corpus (`🧬️contract/📚️examples/🧪️conformance/`, 62 cases) —
   * the load-bearing proof that this React store agrees with the Rust `apply_patch`/`validate_snapshot`
   * the GPU renderer also builds on. For each accept case: loads the snapshot (+ patch, if present)
   * into a real `UiDocumentStore` and asserts the retained tree shape, every node's accessibility
   * fields, and the full set of reachable `ActionId`s (formatted `scope.name@version`, matching
   * `ActionId::Display`) against the fixture's `.expect.json`. For each reject case: asserts
   * `applyPatch` rejects with the exact named `PatchRejection` AND that the store's state is left
   * reference-identical (not just value-equal) to before, mirroring `UiDocumentStore`'s own guarantee. */
  describe("conformance corpus", async () => {
    // 🧭️ Dynamic imports (never static) — this file ships to the browser in production, and
    // `node:fs`/`node:path`/`node:url` must never enter that bundle. `import.meta.vitest` dead-code
    // elimination strips this whole block (dynamic imports included) from non-test builds.
    const { readFileSync, readdirSync } = await import("node:fs");
    const { dirname, join } = await import("node:path");
    const { fileURLToPath } = await import("node:url");

    const here = dirname(fileURLToPath(import.meta.url));
    const corpusRoot = join(here, "../../../../../../../🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance");

    type CorpusExpectation = {
      readonly case: string;
      readonly kind: string;
      readonly outcome: "accept" | "reject";
      readonly limits: UiDocumentLimits | null;
      readonly tree?: { readonly root: number; readonly nodeCount: number; readonly shape: readonly { readonly id: number; readonly key: string; readonly type: string; readonly children: readonly number[] }[] };
      readonly accessibility?: readonly { readonly id: number; readonly label: string | null; readonly description: string | null; readonly live: string; readonly shortcut: string | null; readonly hidden: boolean }[];
      readonly actionIds?: readonly string[];
      readonly baseRevision?: number;
      readonly patchRejection?: PatchRejection;
    };

    type CorpusCase = { readonly group: string; readonly name: string; readonly expect: CorpusExpectation; readonly snapshot: UiSnapshot; readonly patch: UiPatch | null };

    function loadCorpus(): readonly CorpusCase[] {
      const cases: CorpusCase[] = [];
      for (const group of readdirSync(corpusRoot)) {
        const groupDir = join(corpusRoot, group);
        for (const file of readdirSync(groupDir)) {
          if (!file.endsWith(".expect.json")) continue;
          const name = file.slice(0, -".expect.json".length);
          const expectation = JSON.parse(readFileSync(join(groupDir, file), "utf8")) as CorpusExpectation;
          const snap = JSON.parse(readFileSync(join(groupDir, `${name}.snapshot.json`), "utf8")) as UiSnapshot;
          const patchPath = join(groupDir, `${name}.patch.json`);
          let patch: UiPatch | null = null;
          try {
            patch = JSON.parse(readFileSync(patchPath, "utf8")) as UiPatch;
          } catch {
            patch = null;
          }
          cases.push({ group, name, expect: expectation, snapshot: snap, patch });
        }
      }
      return cases;
    }

    function allActionIds(state: ReturnType<UiDocumentStore["getState"]>): string[] {
      const ids = new Set<string>();
      for (const record of state.nodes.values()) {
        for (const binding of record.bindings ?? []) ids.add(`${binding.action.scope}.${binding.action.name}@${binding.action.version}`);
      }
      return [...ids].sort();
    }

    const cases = loadCorpus();
    it("loads all 62 corpus fixtures", () => {
      expect(cases.length).toBe(62);
    });

    for (const testCase of cases) {
      it(`${testCase.group}/${testCase.name} — ${testCase.expect.outcome}`, () => {
        const limits = testCase.expect.limits ?? DEFAULT_UI_DOCUMENT_LIMITS;
        const store = new UiDocumentStore(testCase.snapshot.surface, limits);
        store.loadSnapshot(testCase.snapshot);

        if (!testCase.patch) {
          expect(testCase.expect.outcome).toBe("accept");
        } else if (testCase.expect.outcome === "reject") {
          const before = store.getState();
          const result = store.applyPatch(testCase.patch);
          expect(result.ok).toBe(false);
          if (!result.ok) expect(result.rejection).toEqual(testCase.expect.patchRejection);
          expect(store.getState()).toBe(before);
          return;
        } else {
          const applied = store.applyPatch(testCase.patch);
          expect(applied.ok).toBe(true);
        }

        const state = store.getState();
        if (testCase.expect.tree) {
          expect(state.root).toBe(testCase.expect.tree.root);
          expect(state.nodes.size).toBe(testCase.expect.tree.nodeCount);
          for (const expected of testCase.expect.tree.shape) {
            const record = state.nodes.get(expected.id);
            expect(record, `node ${expected.id} should exist`).toBeDefined();
            expect(record!.key).toBe(expected.key);
            expect(record!.component.type).toBe(expected.type);
            expect([...(record!.children ?? [])]).toEqual(expected.children);
          }
        }
        if (testCase.expect.accessibility) {
          for (const expected of testCase.expect.accessibility) {
            const record = state.nodes.get(expected.id)!;
            const { props: aria } = accessibilityAriaProps(record.accessibility, `node-${record.id}`);
            expect(record.accessibility.label ?? null).toBe(expected.label);
            expect(record.accessibility.description ?? null).toBe(expected.description);
            expect(record.accessibility.live ?? "off").toBe(expected.live);
            expect(record.accessibility.shortcut ?? null).toBe(expected.shortcut);
            expect(record.accessibility.hidden ?? false).toBe(expected.hidden);
            if (expected.label) expect(aria["aria-label"]).toBe(expected.label);
            if (expected.hidden) expect(aria["aria-hidden"]).toBe(true);
          }
        }
        if (testCase.expect.actionIds) {
          expect(allActionIds(state)).toEqual([...testCase.expect.actionIds].sort());
        }
      });
    }
  });
  //#endregion CorpusConformance
}
//#endregion 🧪️Tests
