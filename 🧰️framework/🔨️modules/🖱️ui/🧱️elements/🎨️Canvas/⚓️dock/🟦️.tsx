import type { IconName } from "@semio-tech/assets";
import type { WindowStackCorner, WindowLayoutStackNode, WindowLayoutAxisNode } from "../../../🔨️modules/🗺️window-layout/🟦️.ts";
import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { useLabel } from "../../🏷️Label/🟦️.tsx";
import { modeDockTabLabelClassName, modeDockActiveTabClass, modeDockInactiveTabClass, modeDockActiveTabFillClass, dropZoneReadyClass, windowBodyFrameClass } from "../../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
import { Icon, Minimize2Icon, Maximize2Icon, CloseIcon } from "../../🔣️Icons/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { modeDockTabClassName } from "../../../🔨️modules/🎛️chrome-control-presentation/🟦️.ts";
import { ChromeControlHint } from "../../💡️ChromeControlHint/🟦️.tsx";
import { DragHandle } from "../../🧱️DragHandle/🟦️.tsx";
import { useSurfaceActive, SurfaceScope } from "../../🌈️Surface/🟦️.tsx";
import { Window } from "../../🪟️Window/🟦️.tsx";
import { WindowChrome } from "../../🗂️WindowChrome/🟦️.tsx";
import { type ResizableJoinCornerResizeHandler, ResizableHandle, ResizablePanel, ResizablePanelGroup } from "../../↔️Resizable/🟦️.tsx";
import * as ResizablePrimitive from "react-resizable-panels";
import { type ModeDragState, type ModeTabInsertPreview, modeDockTabsWithInsertPreview } from "../🎯️drop/🟦️.ts";
import { type ModeLayoutPath, modeJoinPath } from "../🗺️layout/🟦️.ts";
import { modeDockTabInsertPreviewClass } from "../🧲️drag-preview/🟦️.tsx";
import type { ModeWindowDescriptor } from "../🪟️template-drag/🟦️.ts";
import { WINDOW_STACK_CORNERS, modeStackTabsByCorner } from "../📐️corners/🟦️.ts";
import { modeJoinCornerSpecsForCrossSeparator, modeJoinCornerSpecsForSeparator } from "../🔗️join-corners/🟦️.ts";

//#region 🧭️ModeDockTabBar

export interface ModeDockContextValue {
  dragState: ModeDragState | null;
  tabInsertPreview: ModeTabInsertPreview | null;
  draggedInsertTabs: readonly { id: string; title: string; iconId: IconName }[];
  registerStackDropTargets: (path: ModeLayoutPath, corner: WindowStackCorner, tabBarElement: HTMLElement | null, bodyElement: HTMLElement | null) => void;
  startTabDrag: (windowId: string, stackPath: ModeLayoutPath, tabIndex: number, label: string, event: React.PointerEvent<HTMLElement>) => void;
  clearPendingDrag: (pointerId: number) => void;
  closeWindow: (windowId: string) => void;
  activateWindow: (windowId: string) => void;
  deactivateActiveWindow: () => void;
  maximizedStackPath: ModeLayoutPath | null;
  /** ⛶️ False when the canvas has only one window — Focus/Unfocus has nothing to enlarge against. */
  canMaximize: boolean;
  toggleMaximize: (stackPath: ModeLayoutPath) => void;
}

export const ModeDockContext = reactHostPort.createContext<ModeDockContextValue | null>(null);

interface ModeDockTabBarProps {
  stackPath: ModeLayoutPath;
  corner: WindowStackCorner;
  tabs: readonly { id: string; title: string; iconId: IconName }[];
  activeId: string | undefined;
  activeWindowId: string | null;
  onSelectTab: (windowId: string) => void;
  /** 📱️ Windows always take the full space on mobile — the Focus/Unfocus control is meaningless there and is hidden; Close stays. */
  mobile?: boolean;
  showMaximize?: boolean;
  isMaximized?: boolean;
}

const modeDockTabDomId = (stackPath: ModeLayoutPath, windowId: string) => `mode-dock-tab-${encodeURIComponent(stackPath || "root")}-${encodeURIComponent(windowId)}`;
const modeDockPanelDomId = (stackPath: ModeLayoutPath) => `mode-dock-panel-${encodeURIComponent(stackPath || "root")}`;
const modeDockTabActionClass = "flex h-medium w-medium shrink-0 items-center justify-center border-0 bg-transparent transition-colors";

const ModeDockTabBar = reactHostPort.forwardRef<HTMLDivElement, ModeDockTabBarProps>(({ stackPath, corner, tabs, activeId, activeWindowId, onSelectTab, mobile = false, showMaximize = false, isMaximized = false }, ref) => {
  const dock = reactHostPort.useContext(ModeDockContext);
  const modeDragActive = Boolean(dock?.dragState);
  const stackGloballyActive = Boolean(activeId && activeWindowId === activeId);
  const displayTabs = reactHostPort.useMemo(() => modeDockTabsWithInsertPreview(tabs, dock?.tabInsertPreview ?? null, stackPath, corner, dock?.draggedInsertTabs ?? []), [tabs, dock?.tabInsertPreview, stackPath, corner, dock?.draggedInsertTabs]);
  const tabRefs = reactHostPort.useRef(new Map<string, HTMLButtonElement>());
  const focusLabel = useLabel("ui.window.focus");
  const unfocusLabel = useLabel("ui.window.unfocus");
  const closeLabel = useLabel("ui.window.close");
  const tabsLabel = useLabel("ui.window.tabs");

  const focusTab = (index: number) => {
    const tab = tabs[(index + tabs.length) % tabs.length];
    if (!tab) return;
    tabRefs.current.get(tab.id)?.focus();
  };

  const onTabKeyDown = (event: React.KeyboardEvent<HTMLButtonElement>, tabIndex: number, windowId: string) => {
    switch (event.key) {
      case "ArrowLeft":
        event.preventDefault();
        focusTab(tabIndex - 1);
        break;
      case "ArrowRight":
        event.preventDefault();
        focusTab(tabIndex + 1);
        break;
      case "Home":
        event.preventDefault();
        focusTab(0);
        break;
      case "End":
        event.preventDefault();
        focusTab(tabs.length - 1);
        break;
      case "Enter":
      case " ":
        event.preventDefault();
        onSelectTab(windowId);
        break;
    }
  };

  const renderGhostTab = (tab: { id: string; title: string; iconId: IconName }) => (
    <div data-slot="mode-dock-tab-insert-preview" className={modeDockTabInsertPreviewClass} aria-hidden>
      <div className={modeDockTabLabelClassName}>
        <Icon icon={tab.iconId} size="small" className="shrink-0" />
        <span data-slot="inline-label" className="truncate">
          {tab.title}
        </span>
      </div>
    </div>
  );

  const renderTab = (tab: (typeof tabs)[number], stackIndex: number) => {
    const tabActive = activeId === tab.id && stackGloballyActive;
    const tabWindowActive = activeWindowId === tab.id;
    const controlPath = stackPath || "root";
    return (
      <div
        key={tab.id}
        data-slot="mode-dock-tab"
        data-hover-scope
        data-window-id={tab.id}
        data-stack-active={activeId === tab.id ? "true" : undefined}
        data-active={activeWindowId === tab.id ? "true" : undefined}
        className={cn(
          "pointer-events-auto flex items-stretch",
          modeDockTabClassName,
          tabActive ? modeDockActiveTabClass : modeDockInactiveTabClass,
          activeWindowId === tab.id && modeDockActiveTabFillClass,
        )}
      >
        <button
          type="button"
          id={modeDockTabDomId(stackPath, tab.id)}
          role="tab"
          aria-selected={activeId === tab.id}
          aria-controls={modeDockPanelDomId(stackPath)}
          tabIndex={activeId === tab.id ? 0 : -1}
          ref={(element) => {
            if (element) tabRefs.current.set(tab.id, element);
            else tabRefs.current.delete(tab.id);
          }}
          className="flex min-w-0 flex-1 items-center border-0 bg-transparent px-0 text-inherit"
          onClick={() => onSelectTab(tab.id)}
          onKeyDown={(event) => onTabKeyDown(event, stackIndex, tab.id)}
          onPointerUp={(event) => {
            if (event.button !== 0) return;
            dock?.clearPendingDrag?.(event.pointerId);
          }}
        >
          <div className={modeDockTabLabelClassName}>
            <Icon icon={tab.iconId} size="small" className="shrink-0" />
            <span data-slot="inline-label" className="truncate">
              {tab.title}
            </span>
          </div>
        </button>
        {!mobile && showMaximize ? (
          <ChromeControlHint id={`framework.modeDock.${controlPath}.tab.${tab.id}.focus`} text={isMaximized ? unfocusLabel : focusLabel} always>
            <button
              type="button"
              data-slot="mode-dock-tab-focus"
              className={modeDockTabActionClass}
              onClick={(event) => {
                event.stopPropagation();
                dock?.activateWindow(tab.id);
                dock?.toggleMaximize(stackPath);
              }}
            >
              {isMaximized ? <Minimize2Icon className="size-small" /> : <Maximize2Icon className="size-small" />}
            </button>
          </ChromeControlHint>
        ) : null}
        <ChromeControlHint id={`framework.modeDock.${controlPath}.tab.${tab.id}.close`} text={closeLabel} always>
          <button
            type="button"
            data-slot="mode-dock-tab-close"
            className={modeDockTabActionClass}
            onClick={(event) => {
              event.stopPropagation();
              dock?.closeWindow(tab.id);
            }}
          >
            <CloseIcon className="size-small" />
          </button>
        </ChromeControlHint>
        <DragHandle labelId="ui.tree.drag.sort" subject={tab.title} onPointerDown={(event) => dock?.startTabDrag(tab.id, stackPath, stackIndex, tab.title, event)} onClick={(event) => event.stopPropagation()} emphasized={tabWindowActive} />
      </div>
    );
  };

  if (tabs.length === 0 && !modeDragActive && displayTabs.length === 0) return null;

  return (
    <div ref={ref} data-slot="mode-dock-tabs" data-corner={corner} role="tablist" aria-label={tabsLabel} className={cn("flex min-w-0 items-stretch justify-start overflow-x-auto overflow-y-hidden", modeDragActive && dropZoneReadyClass)}>
      {displayTabs.map((tab) =>
        tab.preview === "ghost" ? (
          <div key={`ghost-${tab.id}`}>{renderGhostTab(tab)}</div>
        ) : (
          renderTab(
            tab,
            tabs.findIndex((row) => row.id === tab.id),
          )
        ),
      )}
      {modeDragActive && tabs.length === 0 ? <div data-slot="mode-dock-corner-drop-pad" className="min-h-medium min-w-medium shrink-0" aria-hidden /> : null}
    </div>
  );
});

ModeDockTabBar.displayName = "ModeDockTabBar";

//#endregion 🧭️ModeDockTabBar

//#region 🧭️ModeDockStack

interface ModeDockStackProps {
  stackPath: ModeLayoutPath;
  node: WindowLayoutStackNode;
  windowsById: ReadonlyMap<string, ModeWindowDescriptor>;
  activeWindowId: string | null;
  /** 📱️ Skips the per-tab active-chrome grid (which sizes tab columns to their content and doesn't scroll) in favor of the plain scrollable tab strip, and drops the Focus control (windows always take the full space on mobile) so only Close stays reachable when many windows collapse into one mobile tab stack. Desktop also hides Focus when the canvas has only one window. */
  mobile?: boolean;
}

export const ModeDockStack: React.FC<ModeDockStackProps> = ({ stackPath, node, windowsById, activeWindowId, mobile = false }) => {
  const dock = reactHostPort.useContext(ModeDockContext);
  const stackRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [surfaceActive, surfaceActiveProps] = useSurfaceActive(stackRef);
  const setStackNode = reactHostPort.useCallback((element: HTMLDivElement | null) => {
    stackRef.current = element;
  }, []);
  const cornerTabBarRefs = reactHostPort.useRef<Record<WindowStackCorner, HTMLDivElement | null>>({
    topLeft: null,
    topRight: null,
    bottomLeft: null,
    bottomRight: null,
  });
  const bodyRef = reactHostPort.useRef<HTMLDivElement>(null);
  const activeId = node.activeId ?? node.children[0]?.id;
  const byCorner = modeStackTabsByCorner(node.children);
  const dragActive = Boolean(dock?.dragState);
  const isMaximized = dock?.maximizedStackPath === stackPath;
  const showMaximize = !mobile && Boolean(dock?.canMaximize);

  const mapCornerTabs = (corner: WindowStackCorner) =>
    byCorner[corner].map((child) => ({
      id: child.id,
      title: child.title ?? windowsById.get(child.id)?.title ?? child.id,
      iconId: windowsById.get(child.id)?.iconId ?? "app-window",
    }));

  reactHostPort.useLayoutEffect(() => {
    for (const corner of WINDOW_STACK_CORNERS) {
      dock?.registerStackDropTargets(stackPath, corner, cornerTabBarRefs.current[corner], bodyRef.current);
    }
    return () => {
      for (const corner of WINDOW_STACK_CORNERS) {
        dock?.registerStackDropTargets(stackPath, corner, null, null);
      }
    };
  }, [dock, stackPath, node.children.length, dragActive, byCorner.topLeft.length, byCorner.topRight.length, byCorner.bottomLeft.length, byCorner.bottomRight.length]);

  const activeDescriptor = activeId ? windowsById.get(activeId) : undefined;

  const renderCornerBar = (corner: WindowStackCorner) => {
    const tabs = mapCornerTabs(corner);
    if (tabs.length === 0 && !dragActive) return undefined;
    return (
      <ModeDockTabBar
        ref={(element) => {
          cornerTabBarRefs.current[corner] = element;
        }}
        stackPath={stackPath}
        corner={corner}
        tabs={tabs}
        activeId={activeId}
        activeWindowId={activeWindowId}
        onSelectTab={(windowId) => dock?.activateWindow(windowId)}
        mobile={mobile}
        showMaximize={showMaximize}
        isMaximized={isMaximized}
      />
    );
  };

  const stackBody = (
    <SurfaceScope level="base" fill="surface">
      <div
        id={modeDockPanelDomId(stackPath)}
        role="tabpanel"
        aria-labelledby={activeId ? modeDockTabDomId(stackPath, activeId) : undefined}
        tabIndex={0}
        data-level="base"
        className="flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
      >
        {activeDescriptor
          ? (() => {
              const { children, engagement, ...windowProps } = activeDescriptor;
              return (
                <Window {...windowProps} fill={activeDescriptor.fill ?? true} engagement={engagement} active={activeWindowId === activeId} onActivate={() => dock?.activateWindow(activeId!)}>
                  {children}
                </Window>
              );
            })()
          : null}
      </div>
    </SurfaceScope>
  );

  return (
    <SurfaceScope level="window">
      <WindowChrome
        ref={setStackNode}
        active={surfaceActive}
        level="window"
        stackSlot="mode-dock-stack"
        capSlot="mode-dock-tabbar"
        bodySlot="mode-dock-stack-body"
        chipSlot="mode-dock-tab-cap"
        controlsSlot="mode-dock-controls-cap"
        silhouetteSlot="mode-dock-silhouette-border"
        bodyRef={bodyRef}
        stackBindProps={surfaceActiveProps}
        stackDataAttrs={{ "data-stack-path": stackPath }}
        stackClassName="pointer-events-none relative z-window h-full min-h-0 w-full min-w-0 overflow-hidden bg-transparent"
        bodyClassName={cn("pointer-events-auto flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden p-[var(--padding-standard)]", windowBodyFrameClass)}
        bodySurfaceClassName={windowBodyFrameClass}
        bodySurfaceLevel="base"
        gapProps={{ "data-slot": "mode-dock-tab-gap" } as React.HTMLAttributes<HTMLDivElement>}
        titleChips={renderCornerBar("topLeft")}
        capRightChips={renderCornerBar("topRight")}
        footerLeftChips={renderCornerBar("bottomLeft")}
        footerRightChips={renderCornerBar("bottomRight")}
        body={stackBody}
      />
    </SurfaceScope>
  );
};

//#endregion 🧭️ModeDockStack

export interface ModeRenderContext {
  windowsById: ReadonlyMap<string, ModeWindowDescriptor>;
  activeWindowId: string | null;
  onAxisLayoutChanged: (axisPath: ModeLayoutPath, sizes: Record<string, number>) => void;
  onJoinCornerResize: ResizableJoinCornerResizeHandler;
  registerAxisGroup: (axisPath: ModeLayoutPath, group: ResizablePrimitive.GroupImperativeHandle | null) => void;
  registerAxisElement: (axisPath: ModeLayoutPath, element: HTMLDivElement | null) => void;
}

interface ModeRenderParentAxis {
  path: ModeLayoutPath;
  kind: "row" | "column";
  panelIndex: number;
}

export function renderModeDockNode(node: WindowLayoutAxisNode | WindowLayoutStackNode, path: ModeLayoutPath, ctx: ModeRenderContext, parentAxis?: ModeRenderParentAxis): React.ReactNode {
  if (node.kind === "stack") {
    return <ModeDockStack key={path || "root-stack"} stackPath={path} node={node} windowsById={ctx.windowsById} activeWindowId={ctx.activeWindowId} />;
  }
  const orientation = node.kind === "row" ? "horizontal" : "vertical";
  const childCount = node.children.length;
  const rawSizes = node.children.map((child) => child.size);
  const hasAllDefinedSizes = rawSizes.length === childCount && rawSizes.every((s) => typeof s === "number" && s > 0);
  let sizeList: number[];
  if (hasAllDefinedSizes) {
    const sum = (rawSizes as number[]).reduce((a, b) => a + b, 0);
    if (Math.abs(sum - 100) < 0.01 && sum > 0) {
      sizeList = rawSizes as number[];
    } else if (sum > 0) {
      sizeList = (rawSizes as number[]).map((s) => (s / sum) * 100);
    } else {
      sizeList = node.children.map(() => 100 / childCount);
    }
  } else {
    sizeList = node.children.map(() => 100 / childCount);
  }
  if (sizeList.length !== childCount) {
    sizeList = node.children.map(() => 100 / childCount);
  }
  const defaultLayout: Record<string, number> = {};
  const panels: React.ReactNode[] = [];
  node.children.forEach((child, index) => {
    const childPath = modeJoinPath(path, index);
    const size = sizeList[index] ?? 100 / childCount;
    defaultLayout[childPath] = size;
    if (index > 0) {
      const prevChild = node.children[index - 1]!;
      const joinCorners = [...modeJoinCornerSpecsForSeparator(path, node.kind, index, prevChild, child), ...(parentAxis ? modeJoinCornerSpecsForCrossSeparator(path, node.kind, index, parentAxis) : [])];
      panels.push(<ResizableHandle key={`sep-${childPath}`} joinCorners={joinCorners} onJoinCornerResize={ctx.onJoinCornerResize} orientation={orientation} />);
    }
    panels.push(
      <ResizablePanel key={childPath} id={childPath} defaultSize={size} minSize={8} className="box-border min-h-0 min-w-0 overflow-hidden">
        {renderModeDockNode(child, childPath, ctx, { path, kind: node.kind, panelIndex: index })}
      </ResizablePanel>,
    );
  });
  const axisIdentity = `${path || "root"}-n${childCount}-${sizeList.map((s) => Math.round(s)).join(".")}`;
  return (
    <ResizablePanelGroup
      key={`root-axis-${axisIdentity}`}
      id={`mode-axis-${axisIdentity}`}
      elementRef={(element) => ctx.registerAxisElement(path, element)}
      groupRef={(group) => ctx.registerAxisGroup(path, group)}
      orientation={orientation}
      defaultLayout={defaultLayout}
      onLayoutChanged={(sizes) => ctx.onAxisLayoutChanged(path, sizes)}
      className="h-full min-h-0 w-full min-w-0"
    >
      {panels}
    </ResizablePanelGroup>
  );
}
