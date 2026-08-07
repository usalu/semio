// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/Interpreter/component.tsx
/** @emoji 🌳️ `Interpreter` — turns declarative `UiNode` trees emitted by the wasm-hosted Rust engine
 * into `@semio-tech/ui-react` components; {@link InterpretedUiNode} is the sole entry point
 * plugin/window/panel code calls into. Also owns the `ComponentSceneHost` registry (which lazily
 * mounts `canvas-2d`/`world-3d`/etc. surface hosts) and the shared per-surface context-menu flow. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { createContext, lazy, memo, Suspense, useCallback, useContext, useMemo, useState, type ComponentType, type LazyExoticComponent, type ReactElement, type ReactNode } from "react";
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
  loadingBorderClass,
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
  type UiStatus,
  type UiTranslationKey,
} from "@semio-tech/ui-react";
import {
  resolveUiPresence,
  uiPresenceShowsSkeleton,
  type ActionDescriptor,
  type ComponentKind,
  type ComponentSceneHostProps,
  type ContextMenuItemSpec,
  type PluginContextMenuRequest,
  type UiControlNode,
  type UiNode,
  type UiPresence,
  type UiStackNode,
  type UiTreeItemNode,
  type UiTreeNode,
  type UiTreeSectionNode,
} from "@semio-tech/framework-core";
import { decodeScenePackField } from "@semio-tech/framework-os-core";
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

/** @emoji 🗣️ Resolves a chrome translation key outside hook context (plain node-builder functions run there) — an alias of {@link shellLabel} scoped to this region. */
function interpLabel(key: UiTranslationKey): UiLabel {
  return shellLabel(key);
}

/** @emoji 🕳️ Sanctioned wire-boundary mint point (see ui-react's `UiLabel` docstring): brands an
 * already plugin/manifest-resolved string as {@link UiLabel} at the point it crosses from the wire
 * representation (`ContextMenuItemSpec`/manifest strings, still plain `string`) into a ui-kit prop
 * that requires a real translation-key lookup or explicit runtime data. This is the one place in this
 * file allowed to do that minting — everywhere else must go through {@link useLabel} or `uiDataLabel`.
 * @emoji 🐢️ Also referenced (unqualified) from other still-barrel-resident `UiNode`-adjacent code
 * (window-kind title resolution, manifest label resolution) — exported so those keep resolving. */
export function wireLabel(value: string): UiLabel {
  return value as UiLabel;
}

function ComponentSceneFallback() {
  const loadingSurfaceLabel = useLabel("ui.common.loadingSurface");
  return (
    <p className={cn("text-muted-foreground p-2 text-xs", loadingBorderClass)} role="status">
      {loadingSurfaceLabel}
    </p>
  );
}

function renderComponentSceneHost(
  node: Extract<UiNode, { type: "componentScene" }>,
  onAction: (action: ActionDescriptor) => void,
  requestContextMenu?: UiInterpreterContext["requestContextMenu"],
): ReactNode {
  if (node.componentKind === "virtualFileSystem") {
    return (
      <ShellFaultBoundary boundaryId="surface-virtualFileSystem" fallbackLabel={shellLabel("ui.common.renderError")}>
        <VirtualFileSystemHost node={node} onAction={onAction} requestContextMenu={requestContextMenu} />
      </ShellFaultBoundary>
    );
  }
  const Host = resolveComponentSceneHost(node.componentKind as ComponentKind);
  if (!Host) {
    console.log("[DEBUG] resolveComponentSceneHost miss", node.componentKind);
    return (
      <p className="text-muted-foreground text-xs">
        {interpLabel("ui.common.unknownComponent")}: {node.componentKind}
      </p>
    );
  }
  return (
    <ShellFaultBoundary boundaryId={`surface-${node.componentKind}`} fallbackLabel={shellLabel("ui.common.renderError")}>
      <Host node={node} onAction={onAction} requestContextMenu={requestContextMenu} />
    </ShellFaultBoundary>
  );
}

//#region UiInterpreterContext
export type UiInterpreterContext = {
  readonly onAction: (action: ActionDescriptor) => void;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
};
//#endregion UiInterpreterContext

/** @emoji 🐢️ Also referenced (unqualified) from the still-barrel-resident window/panel-chrome mount
 * code (its `.Provider` wraps each rendered window) — exported so that outside reference keeps
 * resolving after this extraction. */
export const PluginSurfaceActionsContext = createContext<UiInterpreterContext["requestContextMenu"]>(undefined);

/** @emoji 🖱️ On-demand context menu from the active document app instance. */
export function usePluginSurfaceActions(): UiInterpreterContext["requestContextMenu"] {
  return useContext(PluginSurfaceActionsContext);
}

/** @emoji 🐢️ Also referenced (unqualified) from the still-barrel-resident window/panel-chrome mount
 * code (its `.Provider` wraps each rendered window) — exported so that outside reference keeps
 * resolving after this extraction. */
export const ShellContextMenuFallbackContext = createContext<(() => ContextMenuItem[]) | undefined>(undefined);

/** @emoji 🖱️ Shell-level fallback menu builder (the active window's declared actions plus the command
 * palette, see `buildShellContextMenuItems`) — surfaces read this via {@link openSurfaceContextMenu} so
 * a right-click over a scene with no plugin-declared menu still shows *something* instead of nothing. */
export function useShellContextMenuFallback(): (() => ContextMenuItem[]) | undefined {
  return useContext(ShellContextMenuFallbackContext);
}

/** @emoji 🖱️ Shared per-surface context-menu open flow — requests specs from the plugin at `request`,
 * maps them to UI items with the surface's own `mapSpecs` (its `useMapContextMenuSpecs`-bound mapper),
 * and falls back to the shell menu ({@link useShellContextMenuFallback}) when the plugin answers empty
 * or there's no `requestContextMenu` wired at all. Every `onContextMenu` surface (world3d, node-graph
 * ×3, text-editor, tiled-map, board2d) routes through this instead of hand-rolling the fallback. */
export async function openSurfaceContextMenu(
  requestContextMenu: ((request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>) | undefined,
  request: PluginContextMenuRequest,
  mapSpecs: (specs: readonly ContextMenuItemSpec[]) => ContextMenuItem[],
  shellFallback: (() => ContextMenuItem[]) | undefined,
): Promise<ContextMenuItem[]> {
  const specs = requestContextMenu ? await requestContextMenu(request) : [];
  return specs.length > 0 ? mapSpecs(specs) : (shellFallback?.() ?? []);
}

//#region ActionDispatch
function dispatchUiAction(onAction: UiInterpreterContext["onAction"], descriptor: ActionDescriptor, patch: Record<string, unknown>): void {
  onAction({
    ...descriptor,
    args: { ...(typeof descriptor.args === "object" && descriptor.args != null ? descriptor.args : {}), ...patch },
  });
}

function resolveDeclarativeControlIcon(iconId: IconName, size: number | "tiny" | "small" | "base" | "large" = "small"): ReactNode {
  return <Icon icon={iconId} size={size} />;
}
//#endregion ActionDispatch

//#region RenderUiControl
/** @emoji 🎛️ Renders a declarative control node with ui-react primitives. `path` is this control's own full
 * structural path when it's rendered as a top-level {@link UiNode}; omitted (and so no `data-ui-path` is
 * attached) when this is called for a {@link UiTreeItemNode}'s inline `control`, which isn't part of the
 * top-level `UiNode`-tree recursion the wgpu↔React path grammar covers. */
export function renderUiControl(control: UiControlNode, onAction: UiInterpreterContext["onAction"], path?: string): ReactElement {
  switch (control.type) {
    case "input": {
      const commitOnBlur = control.commit === "blur";
      const commitValue = (raw: string) => {
        const value = control.inputKind === "number" ? Number(raw) : raw;
        dispatchUiAction(onAction, control.onChange, { value });
      };
      if (control.inputKind === "longText") {
        return (
          <Textarea
            id={control.id}
            data-ui-path={path}
            className="min-h-[4.5rem] w-full min-w-0"
            value={control.value}
            placeholder={control.placeholder}
            onChange={commitOnBlur ? undefined : (event) => commitValue(event.target.value)}
            onBlur={commitOnBlur ? (event) => commitValue(event.target.value) : undefined}
          />
        );
      }
      const inputType = control.inputKind === "number" ? "number" : control.inputKind === "date" ? "date" : control.inputKind === "color" ? "color" : control.inputKind === "file" ? "file" : "text";
      return (
        <Input
          id={control.id}
          data-ui-path={path}
          type={inputType}
          className="h-medium w-full min-w-0"
          value={control.inputKind === "file" ? undefined : control.value}
          placeholder={control.placeholder}
          min={control.min}
          max={control.max}
          step={control.step}
          accept={control.inputKind === "file" ? control.accept : undefined}
          onChange={
            commitOnBlur
              ? undefined
              : (event) => {
                  if (control.inputKind === "file") {
                    commitValue(event.target.files?.[0]?.name ?? "");
                    return;
                  }
                  commitValue(event.target.value);
                }
          }
          onBlur={
            commitOnBlur
              ? (event) => {
                  if (control.inputKind === "file") {
                    commitValue(event.target.files?.[0]?.name ?? "");
                    return;
                  }
                  commitValue(event.target.value);
                }
              : undefined
          }
        />
      );
    }
    case "select":
      return (
        <Select value={control.value || undefined} onValueChange={(value) => dispatchUiAction(onAction, control.onChange, { value })}>
          <SelectTrigger id={control.id} data-ui-path={path} className="h-medium w-full min-w-0" size="sm">
            <SelectValue placeholder={control.placeholder ?? interpLabel("ui.common.select")} />
          </SelectTrigger>
          <SelectContent>
            {control.items.map((item, index) => (
              <SelectItem key={`${control.id}:${index}:${item.value}`} value={item.value}>
                {item.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      );
    case "toggle":
      // 🧭️ `Toggle` (`framework/ui/js/react/index.tsx`) has a closed prop type with no passthrough/`data-*`
      // forwarding — `path` best-effort only, via a parent stack/section/group's wrapper (see
      // `UI_NODE_TYPES_NEEDING_WRAPPER_PATH_FALLBACK`).
      return <Toggle id={control.id} pressed={control.pressed} text={control.text} icon={resolveDeclarativeControlIcon(control.iconId as IconName)} onPressedChange={(pressed) => dispatchUiAction(onAction, control.onChange, { pressed })} />;
    case "keyValue":
      return (
        <dl className="grid grid-cols-[auto_1fr] gap-x-single gap-y-single text-xs" data-ui-path={path}>
          {control.entries.map((entry) => (
            <div key={entry.label} className="contents">
              <dt className="text-muted-foreground">{entry.label}</dt>
              <dd className="tabular-nums">{entry.value}</dd>
            </div>
          ))}
        </dl>
      );
    case "slider": {
      const slider = (
        <Slider
          id={control.id}
          data-ui-path={path}
          className="w-full min-w-0"
          max={control.max}
          min={control.min}
          step={control.step}
          value={[control.value]}
          onValueChange={(values) => dispatchUiAction(onAction, control.onChange, { value: values[0] ?? control.value })}
        />
      );
      if (!control.unit) return slider;
      return (
        <div className="flex min-w-0 w-full items-center gap-single">
          {slider}
          <span className="text-muted-foreground shrink-0 text-xs tabular-nums">
            {control.value} {control.unit}
          </span>
        </div>
      );
    }
    case "numberStepper":
      // 🧭️ `Stepper` has a closed prop type with no passthrough/`data-*` forwarding — `path` best-effort
      // only, via a parent stack/section/group's wrapper.
      return (
        <Stepper
          id={control.id}
          step={control.step}
          value={control.uniform ? control.value : undefined}
          mixed={!control.uniform}
          onChange={(value) => dispatchUiAction(onAction, control.onAbsolute, { value })}
          onDelta={(delta) => dispatchUiAction(onAction, control.onDelta, { delta })}
        />
      );
    case "ring": {
      const presence = resolveUiPresence(control.presence);
      return <Ring id={control.id} onOrbChange={(_orbId, _oldT, newT) => dispatchUiAction(onAction, control.onChange, { t: newT })} orbs={[{ disabled: presence.state === "disabled", id: control.orbId, selected: true, t: control.t }]} />;
    }
    case "iconSelect":
      // 🧭️ `IconSelector` has a closed prop type with no passthrough/`data-*` forwarding — `path`
      // best-effort only, via a parent stack/section/group's wrapper.
      return (
        <IconSelector
          classifyIconSelectorMode={control.classifierKind === "puzzle2d" ? classifyIconSelectorMode : undefined}
          id={control.id}
          onChange={(next) => dispatchUiAction(onAction, control.onChange, { value: next })}
          uniform={control.uniform}
          value={control.value}
        />
      );
    case "button": {
      const presence = resolveUiPresence(control.presence);
      return (
        <Button
          id={control.id}
          data-ui-path={path}
          text={control.label}
          icon={resolveDeclarativeControlIcon(control.iconId as IconName)}
          disabled={presence.state === "disabled"}
          onClick={() => onAction(control.action)}
          className={presence.status === "loading" ? loadingBorderElementClass : presence.status === "waiting" ? waitingBorderElementClass : undefined}
          aria-busy={presence.status === "loading" || presence.status === "waiting" || undefined}
        />
      );
    }
  }
}
//#endregion RenderUiControl

/** @emoji 🐢️ Also referenced (unqualified) from the still-barrel-resident window/panel-chrome mount
 * code (window/panel status derivation) — exported so that outside reference keeps resolving after
 * this extraction. */
export function declarativeSurfaceStatus(node: UiNode | undefined): UiStatus {
  if (!node) return "loading";
  if (uiPresenceShowsSkeleton(declarativeNodePresence(node))) return resolveUiPresence(declarativeNodePresence(node)).status;
  return "idle";
}

//#region UiTreePanel
function uiTreeItemsToTreeData(items: readonly UiTreeItemNode[], onAction: UiInterpreterContext["onAction"]): TreeDataItem[] {
  return items.map((item) => {
    const presence = resolveUiPresence(item.presence);
    return {
      id: item.id,
      label: item.label,
      description: item.description,
      icon: item.iconId ? renderControlIcon(item.iconId, 12) : undefined,
      control: item.control ? renderUiControl(item.control, onAction) : undefined,
      defaultOpen: item.defaultOpen,
      isSelected: presence.selected,
      loading: presence.status === "loading",
      waiting: presence.status === "waiting",
      isHidden: item.dimmed,
      draggable: item.draggable,
      dragData: item.dragData,
      items: item.items?.length ? uiTreeItemsToTreeData(item.items, onAction) : undefined,
      onClick: item.action ? () => dispatchUiAction(onAction, item.action!, {}) : undefined,
      onPointerEnter: item.hoverAction ? () => dispatchUiAction(onAction, item.hoverAction!, {}) : undefined,
      onPointerLeave: item.unhoverAction ? () => dispatchUiAction(onAction, item.unhoverAction!, {}) : undefined,
      actions: item.actions?.map((action) => ({
        kind: "button" as const,
        icon: renderControlIcon(action.iconId, 12),
        title: action.label,
        placement: action.placement ?? "row",
        onClick: () => dispatchUiAction(onAction, action.action, {}),
      })),
    };
  });
}

/** @emoji 🌲️ Maps a declarative {@link UiTreeNode} to a {@link TreePanelConfig}. */
export function uiTreeNodeToTreePanelConfig(treeNode: UiTreeNode, onAction: UiInterpreterContext["onAction"]): TreePanelConfig {
  const sections: TreeDataSection[] = treeNode.sections.map((section: UiTreeSectionNode) => {
    const sectionPresence = resolveUiPresence(section.presence);
    return {
      id: section.id,
      label: section.label ?? "",
      defaultOpen: section.defaultOpen,
      loading: sectionPresence.status === "loading",
      waiting: sectionPresence.status === "waiting",
      items: uiTreeItemsToTreeData(section.items, onAction),
    };
  });
  return {
    sections,
    selectedIds: treeNode.selectedIds as string[] | undefined,
    highlightedIds: treeNode.highlightedIds,
    onSelectionChange: treeNode.selectionChange ? (selectedIds) => dispatchUiAction(onAction, treeNode.selectionChange!, { ids: selectedIds }) : undefined,
    // 🌲️ Match Settings/Theme: section drag handles only when the tree declares drop/reorder intent.
    sortableSections: Boolean(treeNode.dropAction) && sections.length > 1,
  };
}

function treeDragPayloadMime(treeNode: UiTreeNode): string | undefined {
  for (const section of treeNode.sections) {
    const visit = (items: readonly UiTreeItemNode[]): string | undefined => {
      for (const item of items) {
        const mime = item.dragData ? Object.keys(item.dragData)[0] : undefined;
        if (mime) return mime;
        const nested = item.items?.length ? visit(item.items) : undefined;
        if (nested) return nested;
      }
      return undefined;
    };
    const mime = visit(section.items);
    if (mime) return mime;
  }
  return undefined;
}

/** @emoji 🖱️ Builds the drag/drop controller for a declarative tree — palette source when items carry drag payloads, drop dispatch when the tree declares a drop action. */
export function declarativeTreeDragController(treeNode: UiTreeNode, onAction: UiInterpreterContext["onAction"]): TreeDragAndDropController | undefined {
  const mime = treeDragPayloadMime(treeNode);
  const source = mime ? catalogueTreeDragController(mime) : undefined;
  const dropAction = treeNode.dropAction;
  if (!dropAction) return source;
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
      dispatchUiAction(onAction, dropAction, { ...payload, targetId: target.id, dropPosition: dropPosition ?? "inside" });
    },
  };
}

function DeclarativeTreePanel({ treeNode, onAction }: { readonly treeNode: UiTreeNode; readonly onAction: UiInterpreterContext["onAction"] }) {
  const sectionConfig = useMemo(() => {
    const sections: TreeDataSection[] = treeNode.sections.map((section: UiTreeSectionNode) => {
      const sectionPresence = resolveUiPresence(section.presence);
      return {
        id: section.id,
        label: section.label ?? "",
        defaultOpen: section.defaultOpen,
        loading: sectionPresence.status === "loading",
        waiting: sectionPresence.status === "waiting",
        items: uiTreeItemsToTreeData(section.items, onAction),
      };
    });
    return { sections, sortableSections: Boolean(treeNode.dropAction) && sections.length > 1 };
  }, [onAction, treeNode.dropAction, treeNode.sections]);
  const config = useMemo(
    (): TreePanelConfig => ({
      ...sectionConfig,
      selectedIds: treeNode.selectedIds as string[] | undefined,
      highlightedIds: treeNode.highlightedIds,
      onSelectionChange: treeNode.selectionChange ? (selectedIds) => dispatchUiAction(onAction, treeNode.selectionChange!, { ids: selectedIds }) : undefined,
    }),
    [sectionConfig, treeNode.highlightedIds, treeNode.selectedIds, treeNode.selectionChange, onAction],
  );
  const dragController = useMemo(() => declarativeTreeDragController(treeNode, onAction), [onAction, treeNode.dropAction, treeNode.sections]);
  return (
    <Tree
      className="min-h-0 min-w-0 flex-1 overflow-auto"
      sections={config.sections}
      selectionMode={config.selectedIds?.length ? "multiple" : "single"}
      showLines
      selectedIds={config.selectedIds}
      highlightedIds={config.highlightedIds}
      onSelectionChange={config.onSelectionChange}
      dragAndDropController={dragController}
      sortableSections={config.sortableSections ?? config.sections.length > 1}
      onSectionsReorder={config.onSectionsReorder}
    />
  );
}
//#endregion UiTreePanel

//#region VirtualFileSystemHost
/** @emoji 🐢️ Also referenced (unqualified) from other still-barrel-resident scene-field parsers
 * (table/text-editor/tiled-map/history/palette/event-feed hosts) — exported so those keep resolving. */
export function parseSceneJsonField<T>(encoded: string): T {
  if (encoded.startsWith("pk:")) return decodeScenePackField(encoded) as T;
  return JSON.parse(encoded) as T;
}

function VirtualFileSystemHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.virtualFileSystem;
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const contextMenuTitleLabel = useLabel("ui.surfaceContextMenu.file");
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number; readonly items: readonly ContextMenuItem[] } | null>(null);
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
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
      onSelectionChange={(ids) =>
        onAction({
          controllerId: node.controllerId,
          action: "selectRows",
          args: { surfaceId: node.surfaceId, ids },
        })
      }
      onRowContextMenu={(row, index, event) => {
        if (!requestContextMenu) return;
        event.preventDefault();
        event.stopPropagation();
        const rowId = String(row.id ?? index);
        void (async () => {
          const items = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "virtualFileSystem" },
              surface: {
                surfaceId: node.surfaceId,
                kind: "virtualFileSystem",
                hits: [{ domain: "row", id: rowId }],
                selection: selectedRowIds && selectedRowIds.length > 0 ? [{ domain: "row", ids: selectedRowIds }] : [],
              },
              windowInstanceId: windowInstanceId ?? undefined,
              point: { x: event.clientX, y: event.clientY },
            },
            mapContextMenu,
            shellContextMenuFallback,
          );
          setContextMenu({ x: event.clientX, y: event.clientY, items });
        })();
      }}
      onRowDoubleClick={(row) => {
        const uri = row.navigateUri;
        if (!uri) return;
        if (uri.startsWith("os://instance/")) {
          onAction({
            controllerId: node.controllerId,
            action: "openInstance",
            args: { surfaceId: node.surfaceId, instanceId: uri.slice("os://instance/".length) },
          });
          return;
        }
        if (uri.startsWith("os://export/")) {
          const [, , , instanceId, , format] = uri.split("/");
          if (instanceId && format) {
            onAction({
              controllerId: node.controllerId,
              action: "exportMedia",
              args: { surfaceId: node.surfaceId, instanceId, format },
            });
          }
          return;
        }
        if (uri.startsWith("os://import/")) {
          const [, , , instanceId, resourceKind, format] = uri.split("/");
          if (instanceId && format) {
            onAction({
              controllerId: node.controllerId,
              action: "importMedia",
              args: { surfaceId: node.surfaceId, instanceId, resourceKind, format },
            });
          }
          return;
        }
        if (uri.startsWith("/spaces/")) {
          const spaceId = uri.split("/")[2];
          if (spaceId) {
            onAction({
              controllerId: node.controllerId,
              action: "navigateVirtualFileSystemNode",
              args: { surfaceId: node.surfaceId, spaceId },
            });
          }
          return;
        }
        if (uri.startsWith("studio:")) {
          onAction({
            controllerId: node.controllerId,
            action: "navigateVirtualFileSystemNode",
            args: { surfaceId: node.surfaceId, spaceId: uri.slice("studio:".length) },
          });
        }
      }}
    />
    <ContextMenuController
      title={contextMenuTitleLabel}
      open={contextMenu != null}
      position={contextMenu ?? { x: 0, y: 0 }}
      items={contextMenu?.items ?? []}
      onOpenChange={(open) => {
        if (!open) setContextMenu(null);
      }}
    />
    </>
  );
}
//#endregion VirtualFileSystemHost

//#region InterpretUiNode
function uiNodeKey(node: UiNode, index: number): string {
  if ("id" in node && typeof node.id === "string" && node.id) return node.id;
  return `${node.type}:${index}`;
}

/** @emoji 🧭️ Computes a {@link UiNode}'s own structural-path segment — `type[index]`, or `type[index]#id`
 * when the node carries a non-empty string `id` — per the wgpu↔React `data-ui-path` join grammar. */
function uiNodePathSegment(node: UiNode, index: number): string {
  const id = "id" in node && typeof node.id === "string" && node.id ? node.id : undefined;
  return id ? `${node.type}[${index}]#${id}` : `${node.type}[${index}]`;
}

/** @emoji 🧭️ Extends a parent's full structural path with a child's own segment. */
function uiChildPath(parentPath: string, node: UiNode, index: number): string {
  return `${parentPath}/${uiNodePathSegment(node, index)}`;
}

/** @emoji 🧭️ `UiNode` kinds whose `@semio-tech/ui-react` component doesn't forward passthrough/`data-*`
 * props to its own root DOM element (verified against each component's prop type in `framework/ui/js/react/index.tsx`),
 * so `data-ui-path` for these falls back onto the nearest existing per-child wrapper `<div>` — present only
 * when the node is a child of a `stack`/`section`/`group` — instead of the node's own rendered element. */
const UI_NODE_TYPES_NEEDING_WRAPPER_PATH_FALLBACK = new Set<UiNode["type"]>(["field", "section", "group", "toggle", "numberStepper", "ring", "iconSelect", "tree", "componentScene"]);

/** @emoji 🫳️ Stateful host for a {@link UiStackNode} — the plain stack layout/click/drop wiring plus local drag-over tracking so `dropOverlay` can show a full-bleed hint while a drag hovers, ahead of `dropAction` firing on release. */
function UiStackHost({ node, context, path }: { readonly node: UiStackNode; readonly context: UiInterpreterContext; readonly path: string }) {
  const [dragOver, setDragOver] = useState(false);
  const activate = node.activate;
  const dropAction = node.dropAction;
  const dropOverlay = node.dropOverlay;
  const stackPresence = resolveUiPresence(node.presence);
  return (
    <div
      className={cn(
        "relative flex min-h-0 min-w-0 flex-1",
        node.direction === "horizontal" ? "flex-row" : "flex-col",
        node.gap === "none" ? "gap-0" : node.gap === "tight" ? "gap-single" : node.gap === "relaxed" ? "gap-small" : "gap-double",
        node.padding === "none" ? "p-0" : "p-double",
        `semio-ui-stack semio-ui-stack--${node.direction}`,
        activate && cn(borderElementClass, "border cursor-pointer rounded-md"),
        stackPresence.selected && "ring-primary border-primary ring-1",
      )}
      data-ui-path={path}
      role={activate ? "button" : undefined}
      onClick={
        activate
          ? (event) => {
              event.stopPropagation();
              dispatchUiAction(context.onAction, activate, {});
            }
          : undefined
      }
      onDragOver={
        dropAction
          ? (event) => {
              event.preventDefault();
              event.dataTransfer.dropEffect = "copy";
              if (dropOverlay && !dragOver) setDragOver(true);
            }
          : undefined
      }
      onDragLeave={
        dropAction && dropOverlay
          ? (event) => {
              event.preventDefault();
              setDragOver(false);
            }
          : undefined
      }
      onDrop={
        dropAction
          ? (event) => {
              event.preventDefault();
              event.stopPropagation();
              setDragOver(false);
              const encoded = [...event.dataTransfer.types].filter((kind) => kind.startsWith("application/x-semio-")).map((kind) => event.dataTransfer.getData(kind))[0];
              if (!encoded?.trim()) return;
              try {
                dispatchUiAction(context.onAction, dropAction, JSON.parse(encoded) as Record<string, unknown>);
              } catch {
                return;
              }
            }
          : undefined
      }
    >
      {node.children.map((child, index) => {
        const childPath = uiChildPath(path, child, index);
        return (
          <div key={uiNodeKey(child, index)} className="flex-auto" data-ui-path={UI_NODE_TYPES_NEEDING_WRAPPER_PATH_FALLBACK.has(child.type) ? childPath : undefined}>
            {interpretUiNode(child, context, childPath)}
          </div>
        );
      })}
      {dropOverlay && dragOver ? (
        <div
          className="border-primary pointer-events-none absolute inset-0 z-10 flex flex-col items-center justify-center gap-single rounded-md border-2 border-dashed p-double text-center"
          style={{ background: "color-mix(in oklab, var(--panel) 92%, transparent)" }}
        >
          <p className="text-sm font-semibold">{dropOverlay.title}</p>
          <p className="text-muted-foreground text-xs">{dropOverlay.hint}</p>
        </div>
      ) : null}
    </div>
  );
}

function declarativeNodePresence(node: UiNode): UiPresence | undefined {
  return "presence" in node ? node.presence : undefined;
}

function interpretUiNodeBusyShell(node: UiNode, path: string): ReactNode | null {
  if (!uiPresenceShowsSkeleton(declarativeNodePresence(node))) return null;
  const status = resolveUiPresence(declarativeNodePresence(node)).status;
  return (
    <div data-ui-path={path} data-ui-status={status} className={cn("p-single w-full min-w-0", status === "waiting" ? waitingBorderElementClass : loadingBorderElementClass)} role="status" aria-busy="true">
      {elementSkeleton(node.type as ElementSkeletonKind)}
    </div>
  );
}

/** @emoji 🌳️ Interprets a declarative {@link UiNode} tree into ui-react components. `path` is this node's
 * own full structural path (see {@link uiNodePathSegment}); defaults to a root segment at index 0 so
 * existing/test call sites that omit it still behave as the root of a window/panel body. */
export function interpretUiNode(node: UiNode, context: UiInterpreterContext, path: string = uiNodePathSegment(node, 0)): ReactNode {
  const busyShell = interpretUiNodeBusyShell(node, path);
  if (busyShell) return busyShell;
  switch (node.type) {
    case "stack":
      return <UiStackHost node={node} context={context} path={path} />;
    case "text":
      return (
        <p className={cn("text-foreground", node.emphasize ? "font-semibold" : "text-sm")} data-ui-path={path}>
          {node.value}
        </p>
      );
    case "button": {
      const presence = resolveUiPresence(node.presence);
      return (
        <Button
          id={node.id}
          data-ui-path={path}
          text={node.label}
          icon={resolveDeclarativeControlIcon(node.iconId as IconName)}
          disabled={presence.state === "disabled"}
          className={presence.status === "loading" ? loadingBorderElementClass : presence.status === "waiting" ? waitingBorderElementClass : undefined}
          aria-busy={presence.status === "loading" || presence.status === "waiting" || undefined}
          onClick={() => context.onAction(node.action)}
        />
      );
    }
    case "separator":
      return <hr className={cn("border-0", borderNormalTopClass)} data-ui-path={path} />;
    case "image":
      return <img id={node.id} src={node.src} alt={node.alt ?? ""} className="max-h-64 max-w-full rounded-md object-contain" data-ui-path={path} />;
    case "input":
      return renderUiControl(node, context.onAction, path);
    case "select":
      return renderUiControl(node, context.onAction, path);
    case "toggle":
      return renderUiControl(node, context.onAction, path);
    case "keyValue":
      return renderUiControl(node, context.onAction, path);
    case "slider":
      return renderUiControl(node, context.onAction, path);
    case "numberStepper":
      return renderUiControl(node, context.onAction, path);
    case "ring":
      return renderUiControl(node, context.onAction, path);
    case "iconSelect":
      return renderUiControl(node, context.onAction, path);
    case "field":
      // 🧭️ `Field` (`framework/ui/js/react/index.tsx`) has a closed prop type with no passthrough/`data-*` forwarding
      // and index.tsx renders no wrapper `<div>` of its own here, so `field`'s own `data-ui-path` has no
      // attachable element at this call site — best-effort only, via a parent stack/section/group's wrapper.
      return (
        <Field id={node.id} label={node.label} description={node.description} required={node.required} error={node.error}>
          {interpretUiNode(node.child, context, uiChildPath(path, node.child, 0))}
        </Field>
      );
    case "section":
      // 🧭️ Same best-effort caveat as `field`: `Section` only forwards `id`/`className`, not `data-*`.
      return (
        <Section id={node.id} title={wireLabel(node.label)}>
          {node.children.map((child, index) => {
            const childPath = uiChildPath(path, child, index);
            return (
              <div key={uiNodeKey(child, index)} data-ui-path={UI_NODE_TYPES_NEEDING_WRAPPER_PATH_FALLBACK.has(child.type) ? childPath : undefined}>
                {interpretUiNode(child, context, childPath)}
              </div>
            );
          })}
        </Section>
      );
    case "group":
      // 🧭️ `group` renders through the same `Section` component/caveat as `section` above.
      return (
        <Section id={node.id} title={wireLabel(node.label)}>
          {node.children.map((child, index) => {
            const childPath = uiChildPath(path, child, index);
            return (
              <div key={uiNodeKey(child, index)} data-ui-path={UI_NODE_TYPES_NEEDING_WRAPPER_PATH_FALLBACK.has(child.type) ? childPath : undefined}>
                {interpretUiNode(child, context, childPath)}
              </div>
            );
          })}
        </Section>
      );
    case "tree":
      // 🧭️ `Tree` (`framework/ui/js/react/index.tsx`) doesn't forward passthrough/`data-*` props to its root either
      // — best-effort only, via a parent stack/section/group's wrapper.
      return <DeclarativeTreePanel treeNode={node} onAction={context.onAction} />;
    case "componentScene":
      // 🧭️ Dispatches through `<Suspense>` into one of 14 lazily-loaded host components (or
      // `VirtualFileSystemHost`) — no DOM element of index.tsx's own to attach to — best-effort only, via a
      // parent stack/section/group's wrapper.
      return renderComponentSceneHost(node, context.onAction, context.requestContextMenu);
    case "externalSlot":
      return (
        <ShellFaultBoundary boundaryId={`extension-${node.pluginId}`} fallbackLabel={shellLabel("ui.common.renderError")}>
          <p className="text-muted-foreground text-xs" data-ui-path={path}>
            Extension unavailable: {node.pluginId}
          </p>
        </ShellFaultBoundary>
      );
  }
}

/**
 * @emoji 🐢️ `React.memo`'d entry point into `interpretUiNode` — bails on re-interpreting (and
 * reconciling) an entire window/panel subtree when both `node` and `onAction` keep the same object
 * identity as last render. Only pays off when callers pass a stable `onAction` (see `os-shell.tsx`'s
 * `onActionStable`) and a `node` whose identity is preserved across no-operation refreshes (see
 * `os-shell.tsx`'s `preserveJsonIdentity`/`mergeRecordPreservingIdentity`) — without both, `node`/
 * `onAction` are fresh every render and this degenerates to the unmemoized call.
 */
export const InterpretedUiNode = memo(function InterpretedUiNode({ node, onAction }: { readonly node: UiNode; readonly onAction: UiInterpreterContext["onAction"] }): ReactNode {
  const requestContextMenu = usePluginSurfaceActions();
  return interpretUiNode(node, { onAction, requestContextMenu });
});
//#endregion InterpretUiNode
//#endregion 🔖️UiInterpreter
