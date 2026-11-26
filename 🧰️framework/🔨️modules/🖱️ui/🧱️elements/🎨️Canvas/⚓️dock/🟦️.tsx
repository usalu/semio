import type { IconName } from "@semio-tech/assets";
import { type WindowLayoutAxis, type WindowLayoutPanel, type WindowLayoutPath, type WindowLayoutStackNode, type WindowStackCorner, WINDOW_STACK_CORNERS, windowIdOf } from "@semio-tech/ui-layout";
import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { useLabel } from "../../🏷️Label/🟦️.tsx";
import { modeDockTabLabelClassName, modeDockActiveTabClass, modeDockInactiveTabClass, modeDockActiveTabFillClass } from "../../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
import { Icon, Minimize2Icon, Maximize2Icon, CloseIcon } from "../../🔣️Icons/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { modeDockTabClassName } from "../../../🔨️modules/🎛️chrome-control-presentation/🟦️.ts";
import { ChromeControlHint } from "../../💡️ChromeControlHint/🟦️.tsx";
import { DragHandle } from "../../🧱️DragHandle/🟦️.tsx";
import { useSurfaceActive, SurfaceScope } from "../../🌈️Surface/🟦️.tsx";
import { Window } from "../../🪟️Window/🟦️.tsx";
import { WindowChrome } from "../../🗂️WindowChrome/🟦️.tsx";
import { useChromePanelSafeArea } from "../../../🔨️modules/🛟️chrome-safe-area/🟦️.tsx";
import { type ResizableGroupHandle, type ResizableJoinCornerResizeHandler, ResizableHandle, ResizablePanel, ResizablePanelGroup } from "../../↔️Resizable/🟦️.tsx";
import { type ModeDragState, type ModeTabInsertPreview, modeDockTabsWithInsertPreview } from "../🎯️drop/🟦️.ts";
import { modePathKey, modeStackTabsByCorner } from "../🗺️layout/🟦️.ts";
import { modeDockTabInsertPreviewClass } from "../🧲️drag-preview/🟦️.tsx";
import { dropPlaceAttributes } from "../../../🔨️modules/🫳️pointer-drag/🟦️.tsx";
import type { ModeWindowDescriptor } from "../🪟️template-drag/🟦️.ts";
import { modeJoinCornerSpecsForCrossSeparator, modeJoinCornerSpecsForSeparator } from "../🔗️join-corners/🟦️.ts";

//#region 🧭️ModeDockTabBar

export interface ModeDockContextValue {
  dragState: ModeDragState | null;
  tabInsertPreview: ModeTabInsertPreview | null;
  draggedInsertTabs: readonly { id: string; title: string; iconId: IconName }[];
  registerStackDropTargets: (path: WindowLayoutPath, corner: WindowStackCorner, tabBarElement: HTMLElement | null, bodyElement: HTMLElement | null) => void;
  startTabDrag: (windowId: string, stackPath: WindowLayoutPath, tabIndex: number, label: string, event: React.PointerEvent<HTMLElement>) => void;
  clearPendingDrag: (pointerId: number) => void;
  closeWindow: (windowId: string) => void;
  activateWindow: (windowId: string) => void;
  deactivateActiveWindow: () => void;
  maximizedStackPath: WindowLayoutPath | null;
  /** ⛶️ False when the canvas has only one window — Focus/Unfocus has nothing to enlarge against. */
  canMaximize: boolean;
  toggleMaximize: (stackPath: WindowLayoutPath) => void;
}

export const ModeDockContext = reactHostPort.createContext<ModeDockContextValue | null>(null);

interface ModeDockTabBarProps {
  stackPath: WindowLayoutPath;
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

const modeDockTabDomId = (stackPath: WindowLayoutPath, windowId: string) => `mode-dock-tab-${encodeURIComponent(modePathKey(stackPath) || "root")}-${encodeURIComponent(windowId)}`;
const modeDockPanelDomId = (stackPath: WindowLayoutPath) => `mode-dock-panel-${encodeURIComponent(modePathKey(stackPath) || "root")}`;
const modeDockTabActionClass = "flex h-medium w-medium shrink-0 items-center justify-center border-0 bg-transparent transition-colors";

const ModeDockTabBar = reactHostPort.forwardRef<HTMLDivElement, ModeDockTabBarProps>(({ stackPath, corner, tabs, activeId, activeWindowId, onSelectTab, mobile = false, showMaximize = false, isMaximized = false }, ref) => {
  const dock = reactHostPort.useContext(ModeDockContext);
  const modeDragActive = Boolean(dock?.dragState);
  const stackGloballyActive = Boolean(activeId && activeWindowId === activeId);
  const displayTabs = reactHostPort.useMemo(() => modeDockTabsWithInsertPreview(tabs, dock?.tabInsertPreview ?? null, stackPath, corner, dock?.draggedInsertTabs ?? []), [tabs, dock?.tabInsertPreview, modePathKey(stackPath), corner, dock?.draggedInsertTabs]);
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
    <div data-slot="mode-dock-tab-insert-preview" data-shared-border-cell="" data-shared-border-state="active" {...dropPlaceAttributes("active")} className={modeDockTabInsertPreviewClass} aria-hidden>
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
    const controlPath = modePathKey(stackPath) || "root";
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
    <div ref={ref} data-slot="mode-dock-tabs" data-corner={corner} role="tablist" aria-label={tabsLabel} {...dropPlaceAttributes(modeDragActive ? "available" : undefined)} className="flex min-w-0 items-stretch justify-start overflow-x-auto overflow-y-hidden">
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
  stackPath: WindowLayoutPath;
  node: WindowLayoutStackNode;
  windowsById: ReadonlyMap<string, ModeWindowDescriptor>;
  activeWindowId: string | null;
  /** 📲️ Skips the per-tab active-chrome grid (which sizes tab columns to their content and doesn't scroll) in favor of the plain scrollable tab strip, and drops the Focus control (windows always take the full space on mobile) so only Close stays reachable when many windows collapse into one mobile tab stack. Desktop also hides Focus when the canvas has only one window. */
  mobile?: boolean;
}

/** 🧭️ How far the top-left tab chips must step down to clear the navbar band painted over the first row of every stack that touches the top edge. Left under the band, Focus, Unfocus and Close of those windows were covered by the navbar controls and unreachable by pointer; the chips now start where the band ends (the window body follows the chips, which it already measures), while stacks below the band keep their own row. */
function useNavbarBandShift(hostRef: React.RefObject<HTMLElement | null>): number {
  const [shift, setShift] = reactHostPort.useState(0);
  reactHostPort.useLayoutEffect(() => {
    const host = hostRef.current;
    if (!host) return;
    const measure = () => {
      const stackTop = host.getBoundingClientRect().top;
      const bandBottom = Math.max(0, ...[...host.ownerDocument.querySelectorAll<HTMLElement>('[data-layout-band="start"]')].map((band) => band.getBoundingClientRect().bottom));
      const next = stackTop < bandBottom ? Math.ceil(bandBottom - stackTop) : 0;
      setShift((previous) => (previous === next ? previous : next));
    };
    measure();
    const observer = typeof ResizeObserver === "function" ? new ResizeObserver(measure) : undefined;
    observer?.observe(host);
    host.ownerDocument.querySelectorAll<HTMLElement>('[data-layout-band="start"]').forEach((band) => observer?.observe(band));
    window.addEventListener("resize", measure);
    return () => {
      observer?.disconnect();
      window.removeEventListener("resize", measure);
    };
  });
  return shift;
}

/** 🛟️ Keeps a stack's top-left tab chips (title, Focus, Close) clear of the anchored chrome panels that float over the canvas region (inline) and of the navbar band above it (block): the chips step past them instead of hiding under them. */
const ModeDockTabsClear: React.FC<{ readonly hostRef: React.RefObject<HTMLElement | null>; readonly children: React.ReactNode }> = ({ hostRef, children }) => {
  const affordanceRef = reactHostPort.useRef<HTMLDivElement>(null);
  const safeArea = useChromePanelSafeArea({ hostRef, affordanceRef, anchor: "top-left", yieldAxis: "inline" });
  const bandShift = useNavbarBandShift(hostRef);
  const style: React.CSSProperties | undefined = safeArea.inlinePx > 0 || bandShift > 0 ? { marginInlineStart: safeArea.inlinePx > 0 ? safeArea.inlinePx : undefined, marginBlockStart: bandShift > 0 ? bandShift : undefined } : undefined;
  return (
    <div ref={affordanceRef} data-slot="mode-dock-tabs-clear" data-safe-area-inline={safeArea.inlinePx || undefined} data-band-shift={bandShift || undefined} className="flex min-w-0" style={style}>
      {children}
    </div>
  );
};

export const ModeDockStack: React.FC<ModeDockStackProps> = ({ stackPath, node, windowsById, activeWindowId, mobile = false }) => {
  const dock = reactHostPort.useContext(ModeDockContext);
  const stackRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [, surfaceActiveProps] = useSurfaceActive(stackRef);
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
  const activeId = node.activeWindowId ?? (node.children[0] && windowIdOf(node.children[0]));
  const byCorner = modeStackTabsByCorner(node.children);
  const dragActive = Boolean(dock?.dragState);
  const isMaximized = dock?.maximizedStackPath != null && modePathKey(dock.maximizedStackPath) === modePathKey(stackPath);
  const showMaximize = !mobile && Boolean(dock?.canMaximize);

  const mapCornerTabs = (corner: WindowStackCorner) =>
    byCorner[corner].map((child) => {
      const id = windowIdOf(child);
      return { id, title: child.title ?? windowsById.get(id)?.title ?? id, iconId: windowsById.get(id)?.iconId ?? "app-window" };
    });

  reactHostPort.useLayoutEffect(() => {
    for (const corner of WINDOW_STACK_CORNERS) {
      dock?.registerStackDropTargets(stackPath, corner, cornerTabBarRefs.current[corner], stackRef.current);
    }
    return () => {
      for (const corner of WINDOW_STACK_CORNERS) {
        dock?.registerStackDropTargets(stackPath, corner, null, null);
      }
    };
  }, [dock, modePathKey(stackPath), node.children.length, dragActive, byCorner.topLeft.length, byCorner.topRight.length, byCorner.bottomLeft.length, byCorner.bottomRight.length]);

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

  const topLeftBar = renderCornerBar("topLeft");

  const stackBody = (
    <div
      id={modeDockPanelDomId(stackPath)}
      role="tabpanel"
      aria-labelledby={activeId ? modeDockTabDomId(stackPath, activeId) : undefined}
      tabIndex={0}
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
  );

  return (
    <SurfaceScope level="window">
      <WindowChrome
        ref={setStackNode}
        active={Boolean(activeId && activeWindowId === activeId)}
        level="window"
        stackSlot="mode-dock-stack"
        capSlot="mode-dock-tabbar"
        bodySlot="mode-dock-stack-body"
        chipSlot="mode-dock-tab-cap"
        controlsSlot="mode-dock-controls-cap"
        frameBorderSlot="mode-dock-frame-border"
        bodyRef={bodyRef}
        stackBindProps={surfaceActiveProps}
        stackDataAttrs={{ "data-stack-path": modePathKey(stackPath) }}
        stackClassName="relative z-window h-full min-h-0 w-full min-w-0 overflow-hidden bg-transparent"
        bodyClassName="pointer-events-auto flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden p-0"
        bodySurfaceFill="surface"
        titleChips={topLeftBar && <ModeDockTabsClear hostRef={stackRef}>{topLeftBar}</ModeDockTabsClear>}
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
  onAxisLayoutChanged: (axisPath: WindowLayoutPath, sizes: Record<string, number>) => void;
  onJoinCornerResize: ResizableJoinCornerResizeHandler;
  registerAxisGroup: (axisPath: WindowLayoutPath, group: ResizableGroupHandle | null) => void;
  registerAxisElement: (axisPath: WindowLayoutPath, element: HTMLDivElement | null) => void;
}

interface ModeRenderParentAxis {
  path: WindowLayoutPath;
  axis: WindowLayoutAxis;
  panelIndex: number;
}

export function renderModeDockNode(node: WindowLayoutPanel, path: WindowLayoutPath, ctx: ModeRenderContext, parentAxis?: ModeRenderParentAxis): React.ReactNode {
  if (node.kind === "stack") {
    return <ModeDockStack key={modePathKey(path) || "root-stack"} stackPath={path} node={node} windowsById={ctx.windowsById} activeWindowId={ctx.activeWindowId} />;
  }
  const orientation = node.axis;
  const childCount = node.children.length;
  const rawSizes = node.children.map((child) => child.size);
  const hasAllDefinedSizes = rawSizes.length === childCount && rawSizes.every((s) => typeof s === "number" && s > 0);
  let sizeList: number[];
  if (hasAllDefinedSizes) {
    const sum = (rawSizes as number[]).reduce((a, b) => a + b, 0);
    if (Math.abs(sum - 1) < 0.0001 && sum > 0) {
      sizeList = (rawSizes as number[]).map((s) => s * 100);
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
    const childPath = [...path, index];
    const childKey = modePathKey(childPath);
    const size = sizeList[index] ?? 100 / childCount;
    defaultLayout[childKey] = size;
    if (index > 0) {
      const prevChild = node.children[index - 1]!;
      const joinCorners = [...modeJoinCornerSpecsForSeparator(path, node.axis, index, prevChild, child), ...(parentAxis ? modeJoinCornerSpecsForCrossSeparator(path, node.axis, index, parentAxis) : [])];
      panels.push(<ResizableHandle key={`sep-${childKey}`} joinCorners={joinCorners} onJoinCornerResize={ctx.onJoinCornerResize} orientation={orientation} />);
    }
    panels.push(
      <ResizablePanel key={childKey} id={childKey} defaultSize={size} minSize={8} className="box-border min-h-0 min-w-0 overflow-hidden">
        {renderModeDockNode(child, childPath, ctx, { path, axis: node.axis, panelIndex: index })}
      </ResizablePanel>,
    );
  });
  const axisIdentity = `${modePathKey(path) || "root"}-n${childCount}-${sizeList.map((s) => Math.round(s)).join(".")}`;
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
