import { type PanelTabSelectionOptions, usePanelTabSelection, PanelTabBar } from "@semio-tech/ui/elements/PanelTabBar";
import { type Anchor, flowFromAnchor } from "@semio-tech/ui/modules/panel-anchor";
import * as React from "react";
import { usePanelDockContext } from "@semio-tech/ui/modules/panel-dock";
import { reactHostPort } from "@semio-tech/ui/elements/Ports/react";
import { LevelProvider } from "@semio-tech/ui/elements/Surface";
import { GhostRegionShell } from "@semio-tech/ui/elements/PanelGhost";
import { cn } from "@semio-tech/ui/modules/class-name-composition";
import { PanelEmptyDockZone } from "@semio-tech/ui/elements/Panel";
import { WindowChrome } from "@semio-tech/ui/elements/WindowChrome";

//#region 🎛️PanelChromeTabBar

/** 🎛️ Props for {@link PanelChromeTabBar}: the anchor's own tab-selection state (see {@link usePanelTabSelection}) plus which anchor it drives. */
export interface PanelChromeTabBarProps extends PanelTabSelectionOptions {
  readonly anchor: Anchor;
  readonly className?: string;
  /** 🗜️ Icon-only chips — see `PanelTabBarProps.compactLabels`. */
  readonly compactLabels?: boolean;
}

/**
 * 🎛️ Hosts an anchor's root tab row inline in the navbar/footer chrome while the floating panel is
 * folded — the SAME {@link PanelTabBar} mechanism (progressive reveal, drill-down memory, drag-and-drop)
 * as a panel-hosted bar, just placed elsewhere. When the panel is open (`visible`), the floating
 * {@link Panel}'s {@link WindowChrome} owns the tab chips; this host keeps a width-matched placeholder so
 * trailing navbar controls (fullscreen) do not reflow. Renders nothing at rest once `tabs` is empty; becomes
 * a drop target only while a dock drag is in flight, mirroring {@link PanelEmptyDockZone}'s empty-anchor
 * behavior. Pair with `Panel`'s `tabBarHost="chrome"` for the matching anchor, and pass the SAME controlled
 * selection props to both — chrome hosting requires controlled state so the two hosts never fork.
 **/
export const PanelChromeTabBar: React.FC<PanelChromeTabBarProps> = ({ anchor, className = "", compactLabels = false, ...selection }) => {
  const dock = usePanelDockContext();
  const { resolvedPath, handlePathChange } = usePanelTabSelection(selection);
  const { tabs, visible } = selection;
  const shellRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [parkedWidth, setParkedWidth] = reactHostPort.useState(0);

  reactHostPort.useLayoutEffect(() => {
    if (visible || tabs.length === 0) return;
    const shell = shellRef.current;
    if (!shell) return;
    const width = shell.getBoundingClientRect().width;
    if (width > 0) setParkedWidth(width);
  }, [visible, tabs, resolvedPath]);

  if (visible) {
    if (tabs.length === 0) return null;
    return (
      <LevelProvider level="panel">
        <GhostRegionShell
          ref={shellRef}
          sessionGhost={false}
          data-level="panel"
          data-slot="panel-chrome-tab-bar"
          data-anchor={anchor}
          data-panel-chrome-tab-bar-placeholder="true"
          aria-hidden="true"
          className={cn("pointer-events-none flex shrink-0 items-center", className)}
          style={parkedWidth > 0 ? { width: parkedWidth, minWidth: parkedWidth } : undefined}
        />
      </LevelProvider>
    );
  }

  if (tabs.length === 0) {
    if (!dock?.dragTabId) return null;
    return (
      <LevelProvider level="panel">
        <GhostRegionShell ref={shellRef} sessionGhost={false} data-level="panel" data-slot="panel-chrome-tab-bar" data-anchor={anchor} className={cn("flex shrink-0 items-center", className)}>
          <PanelEmptyDockZone anchor={anchor} />
        </GhostRegionShell>
      </LevelProvider>
    );
  }

  return (
    <LevelProvider level="panel">
      <GhostRegionShell ref={shellRef} sessionGhost={false} data-level="panel" data-slot="panel-chrome-tab-bar" data-anchor={anchor} className={cn("flex shrink-0 items-center", className)}>
        <WindowChrome
          chipOnly
          level="panel"
          stackSlot="window-chrome-stack"
          titleChips={<PanelTabBar variant="panel" anchor={anchor} tabs={tabs} activePath={resolvedPath} onActivePathChange={handlePathChange} maxRows={1} direction={flowFromAnchor(anchor).block} showActiveColor={visible} compactLabels={compactLabels} />}
        />
      </GhostRegionShell>
    </LevelProvider>
  );
};

//#endregion 🎛️PanelChromeTabBar

/** 📐️ Grid tracks for multi-tab active chrome: one column per tab, then flex gap, then controls. */
export interface ModeDockChromeGrid {
  readonly templateColumns: string;
  readonly activeCol: number;
  readonly gapCol: number;
  readonly controlsCol: number;
  readonly bodyColumnSpan: string;
  readonly activeTabIndex: number;
  readonly tabCol: (tabIndex: number) => number;
}

/** 📐️ Computes {@link ModeDockChromeGrid} column indices for a tab stack. */
export function modeDockChromeGridPlacement(tabs: readonly { id: string; title: string }[], activeId: string | undefined): ModeDockChromeGrid {
  const activeTabIndex = Math.max(
    0,
    tabs.findIndex((tab) => tab.id === activeId),
  );
  const gapCol = tabs.length + 1;
  const controlsCol = tabs.length + 2;
  const activeCol = activeTabIndex + 1;
  const templateParts = [...tabs.map(() => "max-content"), "minmax(0, 1fr)", "max-content"];
  return {
    templateColumns: templateParts.join(" "),
    activeCol,
    gapCol,
    controlsCol,
    bodyColumnSpan: `${activeCol} / ${gapCol + 1}`,
    activeTabIndex,
    tabCol: (tabIndex) => tabIndex + 1,
  };
}
