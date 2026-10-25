// #region 🧲️Header
// 💻️ framework/ui/elements/🎨️Canvas/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { CanvasSkeleton } from "../🦴️Skeletons/🟦️.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../🔽️Select/🟦️.tsx";
import { Navbar, type NavbarItem } from "../🔝️Navbar/🟦️.tsx";
import { type UiLabel } from "../🎗️UiLabel/🟦️.tsx";
import { interactiveActiveFillClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
import { shellFloorFillClass } from "../../🔨️modules/🏠️shell-floor-presentation/🟦️.ts";
import { chromeStatusBorderClass } from "../../🔨️modules/🌀️status-border-presentation/🟦️.ts";
import { useLabel } from "../🏷️Label/🟦️.tsx";
import { useSurface, LevelProvider } from "../🌈️Surface/🟦️.tsx";
import { type UiStatus } from "@semio-tech/ui-styling";
import type { WindowLayoutNode } from "../../🔨️modules/🗺️window-layout/🟦️.ts";
import type { ControlIcon } from "../🔣️Icons/🟦️.tsx";
import { ButtonGroup, ButtonGroupItem } from "../🔳️ButtonGroup/🟦️.tsx";
import { type ModeCanvasDropTarget, type ModeWindowDescriptor, type WindowTemplateDropPayload } from "./🪟️template-drag/🟦️.ts";
import { computeModeSplitPreviewInBody } from "./🎯️drop/🟦️.ts";
import { MODE_DRAG_CURSOR_OFFSET_X, MODE_DRAG_CURSOR_OFFSET_Y, ModeDockDragPreview } from "./🧲️drag-preview/🟦️.tsx";
import { useModeLayoutState } from "./🧭️mode-layout-state-and-sync/🟦️.tsx";
import { useModeDragEffectsAndAxisCallbacks, useModeWindowCallbacks } from "./🖱️mode-drag-and-window-callbacks/🟦️.tsx";
import { useModeShortcutsAndContexts, useModeTemplateDropAndPreview } from "./📥️mode-template-drop-and-contexts/🟦️.tsx";

// #endregion 🔌️Adapters
// #region ⚙️Canvas

/**
 * 🌐️ Container component for canvas window layout.
 **/
export const Canvas: React.FC<{ children: React.ReactNode; id?: string; status?: UiStatus }> = ({ children, id, status = "idle" }) => {
  const parent = useSurface();
  const bgClass = shellFloorFillClass(parent);
  const busy = status === "loading" || status === "waiting";
  return (
    <LevelProvider level="base">
      <div id={id} data-slot="canvas" data-level="base" data-ui-status={busy ? status : undefined} className={cn("box-border h-full w-full p-[var(--padding-standard)]", bgClass, chromeStatusBorderClass(status))}>
        {busy ? <CanvasSkeleton /> : children}
      </div>
    </LevelProvider>
  );
};

/**
 * ↔️ Layout component arranging windows horizontally.
 **/
export const HorizontalWindows: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return <div className="flex flex-row h-full w-full gap-[var(--padding-standard)]">{children}</div>;
};

/**
 * ↕️ Layout component arranging windows vertically.
 **/
export const VerticalWindows: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return <div className="flex flex-col h-full w-full gap-[var(--padding-standard)]">{children}</div>;
};

export interface ModeProps {
  windows: ModeWindowDescriptor[];
  activeWindowId: string | null;
  onActiveWindowChange?: (windowId: string | null) => void;
  onWindowClose?: (windowId: string) => void;
  onWindowOpenInNewWindow?: (windowId: string) => void;
  layout?: WindowLayoutNode;
  onLayoutChange?: (layout: WindowLayoutNode) => void;
  onTemplateDrop?: (payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => void;
  children?: React.ReactNode;
  className?: string;
  /** 📱️ Renders only the active window full-bleed with no tab bar, drag, dock, or maximize chrome. */
  mobile?: boolean;
}

//#region 🧭️ModeCanvasSpacing

/** 📐️ Canvas inset on {@link Mode} body; inter-panel splitters use the same {@link --spacing-single} step as navbar and footer chrome. */
export const MODE_CANVAS_INSET_CLASS = "p-[var(--panel-inset)]";

//#endregion 🧭️ModeRender
/** 🪟️ Golden-Layout-style docking mode shell with tab stacks, drag-dock, resize, maximize, and close. */
const Mode: React.FC<ModeProps> = ({ windows, activeWindowId, onActiveWindowChange, onWindowClose, onWindowOpenInNewWindow, layout, onLayoutChange, onTemplateDrop, children, className = "", mobile = false }) => {
  const { stackDropElementsRef, setLayoutState, setMaximizedStackPath, modeBodyRef, dropZoneRef, setDropZone, setPendingDrag, pendingDrag, dragState, dragLayoutSnapshotRef, layoutStateRef, setDragState, axisGroupRefsRef, axisGroupElementsRef, setTemplateDrag, templateDrag, windowsById, dropZone, layoutState, maximizedStackPath, modeT, modeBodyFillClass } = useModeLayoutState({ windows, layout, onLayoutChange, activeWindowId });
  const { refreshDropZone, finishDrag, closeWindow, toggleMaximize, registerStackDropTargets, startTabDrag, clearPendingDrag, activateWindow, deactivateActiveWindow, handleCanvasBackgroundPointerDown } = useModeWindowCallbacks({ stackDropElementsRef, activeWindowId, setLayoutState, onActiveWindowChange, onWindowClose, setMaximizedStackPath, modeBodyRef, dropZoneRef, setDropZone, setPendingDrag });
  const { clearTemplateDragPreview, onAxisLayoutChanged, onJoinCornerResize, registerAxisGroup, registerAxisElement } = useModeDragEffectsAndAxisCallbacks({ pendingDrag, dragState, dragLayoutSnapshotRef, layoutStateRef, setDragState, setPendingDrag, refreshDropZone, finishDrag, dropZoneRef, setDropZone, setLayoutState, axisGroupRefsRef, axisGroupElementsRef, setTemplateDrag, templateDrag });
  const { canMaximize, previewDragState, tabInsertPreview, draggedInsertTabs, noopDrag, handleExternalTemplateDragOver, handleExternalTemplateDrop, draggedPreviewTitle, draggedPreviewIconId } = useModeTemplateDropAndPreview({ stackDropElementsRef, modeBodyRef, onTemplateDrop, clearTemplateDragPreview, setTemplateDrag, refreshDropZone, templateDrag, dragState, windowsById, dropZone, layoutState });
  const { hasWindows, emptyShellNotice, body } = useModeShortcutsAndContexts({ canMaximize, maximizedStackPath, setMaximizedStackPath, onWindowOpenInNewWindow, activeWindowId, closeWindow, layoutState, toggleMaximize, previewDragState, tabInsertPreview, draggedInsertTabs, registerStackDropTargets, mobile, noopDrag, startTabDrag, clearPendingDrag, activateWindow, deactivateActiveWindow, windowsById, onAxisLayoutChanged, onJoinCornerResize, registerAxisGroup, registerAxisElement, dragState, modeT, children });
  return (
    <div data-slot="mode" data-mobile={mobile ? "true" : undefined} data-dragging={previewDragState ? "true" : undefined} data-maximized-path={maximizedStackPath ?? undefined} className={cn("relative flex h-full min-h-0 w-full flex-col", className)}>
      <LevelProvider level="base">
        <div
          ref={modeBodyRef}
          data-slot="mode-body"
          data-level="base"
          className={cn("relative box-border flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden", modeBodyFillClass, MODE_CANVAS_INSET_CLASS)}
          onPointerDownCapture={handleCanvasBackgroundPointerDown}
          onDragOver={!mobile && onTemplateDrop ? handleExternalTemplateDragOver : undefined}
          onDrop={!mobile && onTemplateDrop ? handleExternalTemplateDrop : undefined}
        >
          {!hasWindows ? (
            <div data-slot="mode-empty" className="flex flex-1 items-center justify-center p-large text-center text-sm text-muted-foreground">
              {emptyShellNotice}
            </div>
          ) : (
            body
          )}
          {!mobile && previewDragState ? (
            <>
              {dropZone?.kind !== "tab" ? (
                <ModeDockDragPreview
                  title={draggedPreviewTitle}
                  iconId={draggedPreviewIconId}
                  content={previewDragState.dragKind === "stack" ? windowsById.get(previewDragState.windowId)?.children : undefined}
                  tabOnly={previewDragState.dragKind === "tab"}
                  style={{
                    position: "fixed",
                    left: previewDragState.x + MODE_DRAG_CURSOR_OFFSET_X,
                    top: previewDragState.y - MODE_DRAG_CURSOR_OFFSET_Y,
                    zIndex: 70,
                  }}
                />
              ) : null}
              {dropZone && (dropZone.kind === "split" || dropZone.kind === "root-split") ? (
                <div data-slot="mode-dock-drop-indicator" className="pointer-events-none absolute inset-0 z-panel">
                  <div
                    className="absolute rounded-sm border-2 border-accent bg-accent/20"
                    style={(() => {
                      if (dropZone.kind === "root-split") {
                        const side = dropZone.side;
                        if (side === "left") return { left: 0, top: 0, width: "50%", height: "100%" };
                        if (side === "right") return { right: 0, top: 0, width: "50%", height: "100%" };
                        if (side === "top") return { left: 0, top: 0, width: "100%", height: "50%" };
                        return { left: 0, bottom: 0, width: "100%", height: "50%" };
                      }
                      const elements = stackDropElementsRef.current.get(dropZone.stackPath);
                      const rect = elements?.body?.getBoundingClientRect();
                      const modeRect = modeBodyRef.current?.getBoundingClientRect();
                      if (!rect || !modeRect) return { display: "none" };
                      const bodyOriginLeft = rect.left - modeRect.left;
                      const bodyOriginTop = rect.top - modeRect.top;
                      const preview = computeModeSplitPreviewInBody(rect.width, rect.height, dropZone.side);
                      return {
                        left: bodyOriginLeft + preview.left,
                        top: bodyOriginTop + preview.top,
                        width: preview.width,
                        height: preview.height,
                      };
                    })()}
                  />
                </div>
              ) : null}
            </>
          ) : null}
        </div>
      </LevelProvider>
    </div>
  );
};

export { Mode };

// #endregion 🧭️Mode

// #region 🧭️App

/** 🎭️ Mode descriptor rendered inside {@link App}. */
export interface AppModeDescriptor {
  id: string;
  label?: UiLabel;
  icon?: React.ReactNode;
  children: React.ReactNode;
}

export interface AppProps {
  modes: AppModeDescriptor[];
  activeModeId: string;
  onActiveModeChange?: (modeId: string) => void;
  children?: React.ReactNode;
  className?: string;
  chrome?: boolean;
}

/** 🧩️ App shell with optional mode switcher and one active mode body. */
const App: React.FC<AppProps> = ({ modes, activeModeId, onActiveModeChange, children, className = "", chrome = true }) => {
  const modeLabel = useLabel("ui.selection.mode");
  const activeMode = modes.find((mode) => mode.id === activeModeId) ?? modes[0];
  const body = children ?? activeMode?.children;
  const showModeNav = chrome && modes.length > 1 && !!onActiveModeChange;

  return (
    <div data-slot="app" className={cn("flex h-full min-h-0 w-full flex-col", className)}>
      {showModeNav ? (
        <div data-slot="app-mode-nav" className="flex shrink-0 items-center gap-[var(--padding-standard)] border-b p-[var(--padding-standard)]">
          <Select id="app.mode.select" value={activeModeId} onValueChange={onActiveModeChange}>
            <SelectTrigger className="w-[min(100%,16rem)]">
              <SelectValue placeholder={modeLabel} />
            </SelectTrigger>
            <SelectContent>
              {modes.map((mode) => (
                <SelectItem key={mode.id} value={mode.id}>
                  {mode.label ?? mode.id}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      ) : null}
      <div data-slot="app-body" className="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        {body}
      </div>
    </div>
  );
};

export { App };

// #endregion 🧭️App

/** 🖥️ App descriptor rendered inside {@link Ui}. */
export interface UiAppDescriptor {
  id: string;
  label?: UiLabel;
  icon?: ControlIcon;
  children: React.ReactNode;
}

export interface UiProps {
  apps: UiAppDescriptor[];
  activeAppId: string;
  onActiveAppChange?: (appId: string) => void;
  navbar?: React.ReactNode;
  footer?: React.ReactNode;
  children?: React.ReactNode;
  className?: string;
  chrome?: boolean;
}

/** 🔝️ Top-level UI shell with optional app switcher and one active app body. */
const Ui: React.FC<UiProps> = ({ apps, activeAppId, onActiveAppChange, navbar, footer, children, className = "", chrome = true }) => {
  const activeApp = apps.find((app) => app.id === activeAppId) ?? apps[0];
  const body = children ?? activeApp?.children;
  const showAppNav = chrome && apps.length > 1 && !!onActiveAppChange;

  const navbarItems: NavbarItem[] = [];
  if (showAppNav) {
    navbarItems.push({
      key: "appNav",
      content: (
        <ButtonGroup id="ui.appNav">
          {apps.map((app) => (
            <ButtonGroupItem
              key={app.id}
              id={`ui.appNav.${app.id}`}
              className={cn(activeAppId === app.id && interactiveActiveFillClass)}
              data-state={activeAppId === app.id ? "on" : undefined}
              onClick={() => onActiveAppChange?.(app.id)}
              icon={app.icon ?? "layout-grid"}
              text={app.label}
            />
          ))}
        </ButtonGroup>
      ),
    });
  }
  if (navbar) {
    navbarItems.push({ key: "navbar", className: "flex-1 min-w-0", content: navbar });
  }

  return (
    <div data-slot="ui" className={cn("relative flex h-full min-h-0 w-full flex-col", className)}>
      {navbarItems.length > 0 ? <Navbar items={navbarItems} /> : null}
      <div data-slot="ui-body" className="relative flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        {body}
      </div>
      {footer && (
        <div data-slot="ui-footer" className="shrink-0">
          {footer}
        </div>
      )}
    </div>
  );
};

export { Ui };
