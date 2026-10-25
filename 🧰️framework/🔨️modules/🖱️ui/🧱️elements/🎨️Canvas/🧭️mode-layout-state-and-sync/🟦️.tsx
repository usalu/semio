/** 🧭️ The layout state, the refs and the prop synchronization effects of the docking mode shell. */
import type { ResizableGroupHandle } from "../../↔️Resizable/🟦️.tsx";
// 🧱️core: reactHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// reactHostPort.createContext/.forwardRef at module top level, which requires a non-circular import (see
// 🧱️elements/🔌️Ports/🟦️.tsx's header comment for why the barrel import caused a real bug).
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { shellFloorFillClass } from "../../../🔨️modules/🏠️shell-floor-presentation/🟦️.ts";
import { useUiTranslation } from "../../🏷️Label/🟦️.tsx";
import { useShellKeydown, useShellScopeOptional, NULL_SHELL_ROOT_REF } from "../../🐚️ShellScope/🟦️.tsx";
import { useSurface } from "../../🌈️Surface/🟦️.tsx";
import type { WindowLayoutNode, WindowStackCorner } from "../../../🔨️modules/🗺️window-layout/🟦️.ts";
import { focusActiveSearchInput, routeWindowSearchEscape, routeWindowSearchKeydown, routeWindowSearchSpace } from "../../🔍️WindowSearch/🟦️.tsx";
import { type ModeWindowDescriptor } from "../🪟️template-drag/🟦️.ts";
import { type ModeLayoutPath, resolveModeLayout, setActiveWindowInLayout } from "../🗺️layout/🟦️.ts";
import { type ModeDragState, type ModeDropZone, type ModePendingDrag } from "../🎯️drop/🟦️.ts";

/** 🗺 Binds the translation, the window indexes, the layout state, the refs and the prop synchronization effects. */
export function useModeLayoutState({ windows, layout, onLayoutChange, activeWindowId }: { readonly windows: ModeWindowDescriptor[]; readonly layout: WindowLayoutNode | undefined; readonly onLayoutChange: ((layout: WindowLayoutNode) => void) | undefined; readonly activeWindowId: string | null }) {
  const shellScope = useShellScopeOptional();
  const { t: modeT } = useUiTranslation();
  const parentSurface = useSurface();
  const modeBodyFillClass = shellFloorFillClass(parentSurface);
  const windowsById = reactHostPort.useMemo(() => new Map(windows.map((window) => [window.id, window])), [windows]);
  const windowsKey = reactHostPort.useMemo(() => windows.map((window) => window.id).join("|"), [windows]);
  const layoutKey = reactHostPort.useMemo(() => JSON.stringify(layout ?? null), [layout]);
  const layoutGeometryKey = reactHostPort.useMemo(() => JSON.stringify(layout ?? null, (key, value) => key === "title" ? undefined : value), [layout]);
  const initialLayout = reactHostPort.useMemo(() => resolveModeLayout(windows, layout), [layout, windows]);
  const [layoutState, setLayoutState] = reactHostPort.useState<WindowLayoutNode>(() => initialLayout);
  const [maximizedStackPath, setMaximizedStackPath] = reactHostPort.useState<ModeLayoutPath | null>(null);
  const [dragState, setDragState] = reactHostPort.useState<ModeDragState | null>(null);
  const [templateDrag, setTemplateDrag] = reactHostPort.useState<{ readonly label: string; readonly x: number; readonly y: number } | null>(null);
  const [pendingDrag, setPendingDrag] = reactHostPort.useState<ModePendingDrag | null>(null);
  const [dropZone, setDropZone] = reactHostPort.useState<ModeDropZone | null>(null);
  const dropZoneRef = reactHostPort.useRef<ModeDropZone | null>(null);
  const modeBodyRef = reactHostPort.useRef<HTMLDivElement>(null);
  const axisGroupRefsRef = reactHostPort.useRef(new Map<ModeLayoutPath, ResizableGroupHandle>());
  const axisGroupElementsRef = reactHostPort.useRef(new Map<ModeLayoutPath, HTMLDivElement>());
  const stackDropElementsRef = reactHostPort.useRef(new Map<ModeLayoutPath, { corners: Partial<Record<WindowStackCorner, HTMLElement | null>>; body: HTMLElement | null }>());
  const layoutStateRef = reactHostPort.useRef(layoutState);
  const dragLayoutSnapshotRef = reactHostPort.useRef<WindowLayoutNode | null>(null);
  const layoutKeyRef = reactHostPort.useRef(layoutKey);
  const layoutGeometryKeyRef = reactHostPort.useRef(layoutGeometryKey);
  const windowsKeyRef = reactHostPort.useRef(windowsKey);
  layoutStateRef.current = layoutState;

  const propDerivedLayoutJsonRef = reactHostPort.useRef(JSON.stringify(initialLayout));
  reactHostPort.useEffect(() => {
    if (JSON.stringify(layoutState) === propDerivedLayoutJsonRef.current) return;
    onLayoutChange?.(layoutState);
  }, [layoutState, onLayoutChange]);

  reactHostPort.useEffect(() => {
    const layoutChanged = layoutKeyRef.current !== layoutKey;
    const geometryChanged = layoutGeometryKeyRef.current !== layoutGeometryKey;
    const windowsChanged = windowsKeyRef.current !== windowsKey;
    if (!layoutChanged && !windowsChanged) return;
    layoutKeyRef.current = layoutKey;
    layoutGeometryKeyRef.current = layoutGeometryKey;
    windowsKeyRef.current = windowsKey;
    const resolved = resolveModeLayout(windows, layout);
    propDerivedLayoutJsonRef.current = JSON.stringify(resolved);
    setLayoutState(resolved);
    if (geometryChanged || windowsChanged) setMaximizedStackPath(null);
  }, [layout, layoutKey, layoutGeometryKey, windows, windowsKey]);

  reactHostPort.useEffect(() => {
    if (!activeWindowId) return;
    setLayoutState((prev) => setActiveWindowInLayout(prev, activeWindowId));
  }, [activeWindowId]);

  useShellKeydown(
    shellScope?.rootRef ?? NULL_SHELL_ROOT_REF,
    (event) => {
      if (!activeWindowId) return;
      const search = windowsById.get(activeWindowId)?.search;
      if (!search?.input) return;
      if (
        routeWindowSearchEscape(search, event, {
          chromeVisible: true,
          actionActive: true,
        })
      ) {
        event.preventDefault();
        event.stopPropagation();
        return;
      }
      if (routeWindowSearchSpace(search, event)) {
        event.preventDefault();
        event.stopPropagation();
        return;
      }
      if (!routeWindowSearchKeydown(search, event)) return;
      event.preventDefault();
      event.stopPropagation();
      queueMicrotask(() => focusActiveSearchInput());
    },
    [activeWindowId, windowsById, windowsKey],
  );
  return { stackDropElementsRef, setLayoutState, setMaximizedStackPath, modeBodyRef, dropZoneRef, setDropZone, setPendingDrag, pendingDrag, dragState, dragLayoutSnapshotRef, layoutStateRef, setDragState, axisGroupRefsRef, axisGroupElementsRef, setTemplateDrag, templateDrag, windowsById, dropZone, layoutState, maximizedStackPath, modeT, modeBodyFillClass };
}
