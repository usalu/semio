/** 🖱️ The window, drag and axis callbacks and effects of the docking mode shell. */
import * as React from "react";
import { createWindowCloseAdmissionV1, type WindowCloseRequestV1 } from "../🚪️close-admission/🟦️.ts";
import type { ResizableGroupHandle } from "../../↔️Resizable/🟦️.tsx";
// 🧱️core: reactHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// reactHostPort.createContext/.forwardRef at module top level, which requires a non-circular import (see
// 🧱️elements/🔌️Ports/🟦️.tsx's header comment for why the barrel import caused a real bug).
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import type { ResizableJoinCornerSpec } from "../../↔️Resizable/🟦️.tsx";
import { setSurfaceActiveRoot } from "../../🌈️Surface/🟦️.tsx";
import { type WindowLayoutDropZone, type WindowLayoutNode, type WindowLayoutPath, type WindowStackCorner, WINDOW_STACK_CORNERS, applyWindowLayoutOp, collectWindowIds } from "@semio-tech/ui-layout";
import { endWindowTemplateDrag } from "../🪟️template-drag/🟦️.ts";
import { applyModeSplitPercents, modePathEquals, modePathFromKey, modePathKey } from "../🗺️layout/🟦️.ts";
import { type ModeDragState, type ModeStackDropTargets, applyModeDrop, computeModeDropZone } from "../🎯️drop/🟦️.ts";
import { applyAxisGroupLayoutDelta, modeAxisGroupLayout, readModeAxis, readModeAxisPixelSize, resolveJoinCornerPeerCrossAxes, resolveJoinCornerResizeDeltas, safePanelGroupSetLayout } from "../🔗️join-corners/🟦️.ts";
import type { useModeLayoutState } from "../🧭️mode-layout-state-and-sync/🟦️.tsx";

/** 🪟 Builds the stack drop registration, the window activation, close and maximize callbacks and the pointer drag callbacks. */
export function useModeWindowCallbacks({ stackDropElementsRef, activeWindowId, setLayoutState, onActiveWindowChange, onWindowClose, setMaximizedStackPath, modeBodyRef, dropZoneRef, setDropZone, setPendingDrag }: Pick<ReturnType<typeof useModeLayoutState>, "stackDropElementsRef" | "setLayoutState" | "setMaximizedStackPath" | "modeBodyRef" | "dropZoneRef" | "setDropZone" | "setPendingDrag"> & { readonly activeWindowId: string | null; readonly onActiveWindowChange: ((windowId: string | null) => void) | undefined; readonly onWindowClose: WindowCloseRequestV1 | undefined }) {
  const registerStackDropTargets = reactHostPort.useCallback((path: WindowLayoutPath, corner: WindowStackCorner, tabBarElement: HTMLElement | null, bodyElement: HTMLElement | null) => {
    const key = modePathKey(path);
    if (!tabBarElement && !bodyElement) {
      const prev = stackDropElementsRef.current.get(key);
      if (!prev) return;
      const corners = { ...prev.corners, [corner]: null };
      const hasCorner = WINDOW_STACK_CORNERS.some((entry) => corners[entry]);
      if (!hasCorner) {
        stackDropElementsRef.current.delete(key);
        return;
      }
      stackDropElementsRef.current.set(key, { path, corners, body: prev.body });
      return;
    }
    const prev = stackDropElementsRef.current.get(key) ?? { path, corners: {}, body: null };
    stackDropElementsRef.current.set(key, {
      path,
      corners: { ...prev.corners, [corner]: tabBarElement ?? prev.corners[corner] ?? null },
      body: bodyElement ?? prev.body,
    });
  }, []);

  const activateWindow = reactHostPort.useCallback(
    (windowId: string) => {
      if (activeWindowId === windowId) return;
      setLayoutState((prev) => applyWindowLayoutOp(prev, { op: "setActive", windowId }));
      onActiveWindowChange?.(windowId);
    },
    [activeWindowId, onActiveWindowChange],
  );

  const deactivateActiveWindow = reactHostPort.useCallback(() => {
    setSurfaceActiveRoot(null);
    if (activeWindowId === null) return;
    onActiveWindowChange?.(null);
  }, [activeWindowId, onActiveWindowChange]);

  const handleCanvasBackgroundPointerDown = reactHostPort.useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!(event.target instanceof HTMLElement)) return;
      if (event.target.closest('[data-slot="mode-dock-stack"]')) return;
      if (event.target !== event.currentTarget && event.target.dataset.slot !== "resizable-panel") return;
      deactivateActiveWindow();
    },
    [deactivateActiveWindow],
  );

  const closeOwnerRef = reactHostPort.useRef({ activeWindowId, onActiveWindowChange, onWindowClose });
  closeOwnerRef.current = { activeWindowId, onActiveWindowChange, onWindowClose };
  const closeAdmissionRef = reactHostPort.useRef<ReturnType<typeof createWindowCloseAdmissionV1> | null>(null);
  reactHostPort.useEffect(() => {
    const admission = createWindowCloseAdmissionV1({
      request: windowId => closeOwnerRef.current.onWindowClose?.(windowId),
      commit: windowId => setLayoutState(prev => {
        const next = applyWindowLayoutOp(prev, { op: "closeWindow", windowId });
        if (closeOwnerRef.current.activeWindowId === windowId) closeOwnerRef.current.onActiveWindowChange?.(collectWindowIds(next)[0] ?? null);
        return next;
      }),
      report: error => console.error("[ui] window close admission failed", error),
    });
    closeAdmissionRef.current = admission;
    return () => { admission.dispose(); if (closeAdmissionRef.current === admission) closeAdmissionRef.current = null; };
  }, []);
  const closeWindow = reactHostPort.useCallback((windowId: string) => {
    void closeAdmissionRef.current?.close(windowId);
  }, []);

  const toggleMaximize = reactHostPort.useCallback((stackPath: WindowLayoutPath) => {
    setMaximizedStackPath((prev) => (modePathEquals(prev, stackPath) ? null : stackPath));
  }, []);

  const refreshDropZone = reactHostPort.useCallback((clientX: number, clientY: number) => {
    const targets = new Map<string, ModeStackDropTargets>();
    stackDropElementsRef.current.forEach((elements, key) => {
      const corners: ModeStackDropTargets["corners"] = {};
      for (const corner of WINDOW_STACK_CORNERS) {
        const element = elements.corners[corner];
        if (!element) continue;
        corners[corner] = { rect: element.getBoundingClientRect(), element };
      }
      targets.set(key, {
        path: elements.path,
        corners,
        body: elements.body?.getBoundingClientRect() ?? null,
      });
    });
    const modeRect = modeBodyRef.current?.getBoundingClientRect() ?? null;
    const zone = computeModeDropZone(clientX, clientY, targets, modeRect);
    dropZoneRef.current = zone;
    setDropZone(zone);
  }, []);

  const finishDrag = reactHostPort.useCallback(
    (drag: ModeDragState, zone: WindowLayoutDropZone | null) => {
      if (!zone) return;
      setLayoutState((prev) => applyModeDrop(prev, drag, zone));
      activateWindow(drag.windowId);
    },
    [activateWindow],
  );

  const clearPendingDrag = reactHostPort.useCallback((pointerId: number) => {
    setPendingDrag((prev) => (prev?.pointerId === pointerId ? null : prev));
  }, []);

  const startTabDrag = reactHostPort.useCallback((windowId: string, stackPath: WindowLayoutPath, tabIndex: number, label: string, event: React.PointerEvent<HTMLElement>) => {
    if (event.button !== 0) return;
    setPendingDrag({
      dragKind: "tab",
      windowId,
      stackPath,
      tabIndex,
      pointerId: event.pointerId,
      ghostLabel: label,
      startX: event.clientX,
      startY: event.clientY,
    });
  }, []);
  return { refreshDropZone, finishDrag, closeWindow, toggleMaximize, registerStackDropTargets, startTabDrag, clearPendingDrag, activateWindow, deactivateActiveWindow, handleCanvasBackgroundPointerDown };
}

/** 🧲 Runs the pointer drag effects and builds the axis layout, group, element and join corner callbacks. */
export function useModeDragEffectsAndAxisCallbacks({ pendingDrag, dragState, dragLayoutSnapshotRef, layoutStateRef, setDragState, setPendingDrag, refreshDropZone, finishDrag, dropZoneRef, setDropZone, setLayoutState, axisGroupRefsRef, axisGroupElementsRef, setTemplateDrag, templateDrag }: Pick<ReturnType<typeof useModeLayoutState>, "pendingDrag" | "dragState" | "dragLayoutSnapshotRef" | "layoutStateRef" | "setDragState" | "setPendingDrag" | "dropZoneRef" | "setDropZone" | "setLayoutState" | "axisGroupRefsRef" | "axisGroupElementsRef" | "setTemplateDrag" | "templateDrag"> & Pick<ReturnType<typeof useModeWindowCallbacks>, "refreshDropZone" | "finishDrag">) {
  reactHostPort.useEffect(() => {
    if (!pendingDrag && !dragState) return;
    const handleMove = (event: PointerEvent) => {
      const activePointerId = dragState?.pointerId ?? pendingDrag?.pointerId;
      if (activePointerId === undefined || event.pointerId !== activePointerId) return;
      if (pendingDrag && !dragState) {
        const distance = Math.hypot(event.clientX - pendingDrag.startX, event.clientY - pendingDrag.startY);
        if (distance < 6) return;
        dragLayoutSnapshotRef.current = layoutStateRef.current;
        setDragState({
          dragKind: pendingDrag.dragKind,
          windowId: pendingDrag.windowId,
          stackPath: pendingDrag.stackPath,
          tabIndex: pendingDrag.tabIndex,
          pointerId: pendingDrag.pointerId,
          ghostLabel: pendingDrag.ghostLabel,
          x: event.clientX,
          y: event.clientY,
        });
        setPendingDrag(null);
        refreshDropZone(event.clientX, event.clientY);
        return;
      }
      if (!dragState) return;
      setDragState((prev) => (prev ? { ...prev, x: event.clientX, y: event.clientY } : prev));
      refreshDropZone(event.clientX, event.clientY);
    };
    const handleUp = (event: PointerEvent) => {
      const activePointerId = dragState?.pointerId ?? pendingDrag?.pointerId;
      if (activePointerId === undefined || event.pointerId !== activePointerId) return;
      if (dragState) finishDrag(dragState, dropZoneRef.current);
      dragLayoutSnapshotRef.current = null;
      setDragState(null);
      setPendingDrag(null);
      dropZoneRef.current = null;
      setDropZone(null);
    };
    document.addEventListener("pointermove", handleMove);
    document.addEventListener("pointerup", handleUp);
    return () => {
      document.removeEventListener("pointermove", handleMove);
      document.removeEventListener("pointerup", handleUp);
    };
  }, [pendingDrag, dragState, finishDrag, refreshDropZone]);

  reactHostPort.useEffect(() => {
    if (!dragState) return;
    const cancelDrag = () => {
      if (dragLayoutSnapshotRef.current) setLayoutState(dragLayoutSnapshotRef.current);
      dragLayoutSnapshotRef.current = null;
      setDragState(null);
      setPendingDrag(null);
      dropZoneRef.current = null;
      setDropZone(null);
    };
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      event.preventDefault();
      cancelDrag();
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [dragState]);

  const onAxisLayoutChanged = reactHostPort.useCallback((axisPath: WindowLayoutPath, sizes: Record<string, number>) => {
    setLayoutState((prev) => applyModeSplitPercents(prev, axisPath, sizes));
  }, []);

  const registerAxisGroup = reactHostPort.useCallback((axisPath: WindowLayoutPath, group: ResizableGroupHandle | null) => {
    const key = modePathKey(axisPath);
    if (group) axisGroupRefsRef.current.set(key, group);
    else axisGroupRefsRef.current.delete(key);
  }, []);

  const registerAxisElement = reactHostPort.useCallback((axisPath: WindowLayoutPath, element: HTMLDivElement | null) => {
    const key = modePathKey(axisPath);
    if (element) axisGroupElementsRef.current.set(key, element);
    else axisGroupElementsRef.current.delete(key);
  }, []);

  const onJoinCornerResize = reactHostPort.useCallback((spec: ResizableJoinCornerSpec, deltaXPx: number, deltaYPx: number) => {
    const current = layoutStateRef.current;
    const mainPath = modePathFromKey(spec.mainAxisPath);
    const crossPath = modePathFromKey(spec.crossAxisPath);
    const mainAxis = readModeAxis(current, mainPath);
    const crossAxis = readModeAxis(current, crossPath);
    const mainGroup = axisGroupRefsRef.current.get(spec.mainAxisPath);
    const crossGroup = axisGroupRefsRef.current.get(spec.crossAxisPath);
    if (!mainAxis || !crossAxis || !mainGroup || !crossGroup) return;
    const peers = resolveJoinCornerPeerCrossAxes(current, spec);
    const deltas = resolveJoinCornerResizeDeltas(
      spec.parentAxis,
      deltaXPx,
      deltaYPx,
      readModeAxisPixelSize(axisGroupElementsRef.current.get(spec.mainAxisPath), mainAxis),
      readModeAxisPixelSize(axisGroupElementsRef.current.get(spec.crossAxisPath), crossAxis),
    );
    const liveMainLayout = mainGroup.getLayout();
    const currentMainLayout = Object.keys(liveMainLayout).length > 0 ? liveMainLayout : modeAxisGroupLayout(current, mainPath);
    const mainLayout = applyAxisGroupLayoutDelta(currentMainLayout, mainPath, spec.mainSeparatorIndex, deltas.mainDeltaPct);
    safePanelGroupSetLayout(mainGroup, mainLayout);
    const peerLayouts: { path: WindowLayoutPath; layout: Record<string, number> }[] = [];
    for (const peer of peers) {
      const peerGroup = axisGroupRefsRef.current.get(modePathKey(peer.path));
      if (!peerGroup) continue;
      const livePeerLayout = peerGroup.getLayout();
      const currentPeerLayout = Object.keys(livePeerLayout).length > 0 ? livePeerLayout : modeAxisGroupLayout(current, peer.path);
      const peerLayout = applyAxisGroupLayoutDelta(currentPeerLayout, peer.path, peer.separatorIndex, deltas.crossDeltaPct);
      safePanelGroupSetLayout(peerGroup, peerLayout);
      peerLayouts.push({ path: peer.path, layout: peerLayout });
    }
    setLayoutState((prev) => peerLayouts.reduce((next: WindowLayoutNode, peer) => applyModeSplitPercents(next, peer.path, peer.layout), applyModeSplitPercents(prev, mainPath, mainLayout)));
  }, []);

  const clearTemplateDragPreview = reactHostPort.useCallback(() => {
    setTemplateDrag(null);
    dropZoneRef.current = null;
    setDropZone(null);
  }, []);

  reactHostPort.useEffect(() => {
    if (!templateDrag) return;
    const onDragEnd = () => {
      endWindowTemplateDrag();
      clearTemplateDragPreview();
    };
    document.addEventListener("dragend", onDragEnd);
    return () => document.removeEventListener("dragend", onDragEnd);
  }, [templateDrag, clearTemplateDragPreview]);
  return { clearTemplateDragPreview, onAxisLayoutChanged, onJoinCornerResize, registerAxisGroup, registerAxisElement };
}
