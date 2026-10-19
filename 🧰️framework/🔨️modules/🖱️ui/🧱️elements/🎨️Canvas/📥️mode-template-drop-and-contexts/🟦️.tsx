/** 📥️ The template drop, preview, shortcut and context bindings of the docking mode shell. */
import * as React from "react";
// 🧱️core: reactHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// reactHostPort.createContext/.forwardRef at module top level, which requires a non-circular import (see
// 🧱️elements/🔌️Ports/🟦️.tsx's header comment for why the barrel import caused a real bug).
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { resolveTranslationLabel } from "../../🏷️Label/🟦️.tsx";
import type { WindowLayoutAxisNode, WindowLayoutStackNode } from "../../../🔨️modules/🗺️window-layout/🟦️.ts";
import { useControlKeybinding } from "../../../🔨️modules/🕹️control-keybinding-context/🟦️.tsx";
import { COMPOSE_WINDOW_TEMPLATE_MIME, MODE_TEMPLATE_PREVIEW_WINDOW_ID, type ModeCanvasDropTarget, type WindowTemplateDropPayload, cancelWindowTemplatePointerDrag, endWindowTemplateDrag, readActiveWindowTemplateDragSession, windowTemplatePointerDragRef } from "../🪟️template-drag/🟦️.ts";
import { type ModeLayoutPath, mapLayoutStacks, modeCollectWindowIds, resolveStackPathForWindowId } from "../🗺️layout/🟦️.ts";
import { type ModeDragState, type ModeDropZone, type ModeStackDropTargets, computeModeDropZone, modeDockDragInsertTabs, modeDockOutLayout, resolveModeTabInsertPreview } from "../🎯️drop/🟦️.ts";
import { WINDOW_STACK_CORNERS } from "../📐️corners/🟦️.ts";
import { ModeDockContext, type ModeDockContextValue, ModeDockStack, type ModeRenderContext, renderModeDockNode } from "../⚓️dock/🟦️.tsx";
import type { useModeLayoutState } from "../🧭️mode-layout-state-and-sync/🟦️.tsx";
import type { useModeDragEffectsAndAxisCallbacks, useModeWindowCallbacks } from "../🖱️mode-drag-and-window-callbacks/🟦️.tsx";

/** 🧺 Builds the template drop callbacks and effects and the drag preview values. */
export function useModeTemplateDropAndPreview({ stackDropElementsRef, modeBodyRef, onTemplateDrop, clearTemplateDragPreview, setTemplateDrag, refreshDropZone, templateDrag, dragState, windowsById, dropZone, layoutState }: Pick<ReturnType<typeof useModeLayoutState>, "stackDropElementsRef" | "modeBodyRef" | "setTemplateDrag" | "templateDrag" | "dragState" | "windowsById" | "dropZone" | "layoutState"> & { readonly onTemplateDrop: ((payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => void) | undefined } & Pick<ReturnType<typeof useModeDragEffectsAndAxisCallbacks>, "clearTemplateDragPreview"> & Pick<ReturnType<typeof useModeWindowCallbacks>, "refreshDropZone">) {
  const resolveTemplateDropZone = reactHostPort.useCallback((clientX: number, clientY: number): ModeDropZone | null => {
    const targets = new Map<ModeLayoutPath, ModeStackDropTargets>();
    stackDropElementsRef.current.forEach((elements, path) => {
      const corners: ModeStackDropTargets["corners"] = {};
      for (const corner of WINDOW_STACK_CORNERS) {
        const element = elements.corners[corner];
        if (!element) continue;
        corners[corner] = { rect: element.getBoundingClientRect(), element };
      }
      targets.set(path, {
        corners,
        body: elements.body?.getBoundingClientRect() ?? null,
      });
    });
    const modeRect = modeBodyRef.current?.getBoundingClientRect() ?? null;
    return computeModeDropZone(clientX, clientY, targets, modeRect);
  }, []);

  const completeExternalTemplateDrop = reactHostPort.useCallback(
    (clientX: number, clientY: number, payload: WindowTemplateDropPayload): boolean => {
      if (!onTemplateDrop || typeof payload.windowKindId !== "string") {
        return false;
      }
      const zone = resolveTemplateDropZone(clientX, clientY);
      if (!zone) {
        return false;
      }
      endWindowTemplateDrag();
      windowTemplatePointerDragRef.active = false;
      clearTemplateDragPreview();
      onTemplateDrop(payload, zone);
      return true;
    },
    [clearTemplateDragPreview, onTemplateDrop, resolveTemplateDropZone],
  );

  const handleExternalTemplateDragOver = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      if (!onTemplateDrop || !event.dataTransfer.types.includes(COMPOSE_WINDOW_TEMPLATE_MIME)) return;
      const session = readActiveWindowTemplateDragSession();
      if (!session) return;
      event.preventDefault();
      event.dataTransfer.dropEffect = "copy";
      setTemplateDrag({ label: session.label, x: event.clientX, y: event.clientY });
      refreshDropZone(event.clientX, event.clientY);
    },
    [onTemplateDrop, refreshDropZone],
  );

  const handleExternalTemplateDrop = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      if (!onTemplateDrop) return;
      const raw = event.dataTransfer.getData(COMPOSE_WINDOW_TEMPLATE_MIME);
      if (!raw) return;
      event.preventDefault();
      let payload: WindowTemplateDropPayload;
      try {
        payload = JSON.parse(raw) as WindowTemplateDropPayload;
      } catch {
        return;
      }
      completeExternalTemplateDrop(event.clientX, event.clientY, payload);
    },
    [completeExternalTemplateDrop, onTemplateDrop],
  );

  reactHostPort.useEffect(() => {
    if (!onTemplateDrop) return;
    const handlePointerMove = (event: PointerEvent) => {
      if (!windowTemplatePointerDragRef.active) return;
      const session = readActiveWindowTemplateDragSession();
      if (!session) return;
      setTemplateDrag({ label: session.label, x: event.clientX, y: event.clientY });
      refreshDropZone(event.clientX, event.clientY);
    };
    const handlePointerUp = (event: PointerEvent) => {
      if (!windowTemplatePointerDragRef.active) return;
      const session = readActiveWindowTemplateDragSession();
      if (!session) {
        windowTemplatePointerDragRef.active = false;
        clearTemplateDragPreview();
        return;
      }
      const modeRect = modeBodyRef.current?.getBoundingClientRect();
      if (!modeRect || event.clientX < modeRect.left || event.clientX > modeRect.right || event.clientY < modeRect.top || event.clientY > modeRect.bottom) {
        cancelWindowTemplatePointerDrag();
        clearTemplateDragPreview();
        return;
      }
      if (!completeExternalTemplateDrop(event.clientX, event.clientY, session.payload)) {
        cancelWindowTemplatePointerDrag();
        clearTemplateDragPreview();
      }
    };
    window.addEventListener("pointermove", handlePointerMove, true);
    window.addEventListener("pointerup", handlePointerUp, true);
    window.addEventListener("pointercancel", handlePointerUp, true);
    return () => {
      window.removeEventListener("pointermove", handlePointerMove, true);
      window.removeEventListener("pointerup", handlePointerUp, true);
      window.removeEventListener("pointercancel", handlePointerUp, true);
    };
  }, [clearTemplateDragPreview, completeExternalTemplateDrop, onTemplateDrop, refreshDropZone]);

  const templatePreviewDrag = reactHostPort.useMemo((): ModeDragState | null => {
    if (!templateDrag) return null;
    return {
      dragKind: "tab",
      windowId: MODE_TEMPLATE_PREVIEW_WINDOW_ID,
      stackPath: "",
      tabIndex: -1,
      pointerId: -1,
      ghostLabel: templateDrag.label,
      x: templateDrag.x,
      y: templateDrag.y,
    };
  }, [templateDrag]);

  const previewDragState = dragState ?? templatePreviewDrag;

  const draggedPreviewTitle = previewDragState ? (windowsById.get(previewDragState.windowId)?.title ?? previewDragState.ghostLabel) : "";
  const draggedPreviewIconId = previewDragState ? windowsById.get(previewDragState.windowId)?.iconId : undefined;
  const tabInsertPreview = resolveModeTabInsertPreview(previewDragState, dropZone);
  const draggedInsertTabs = reactHostPort.useMemo<ModeDockContextValue["draggedInsertTabs"]>(() => {
    if (templateDrag) return [{ id: MODE_TEMPLATE_PREVIEW_WINDOW_ID, title: templateDrag.label, iconId: "app-window" }];
    if (!dragState) return [];
    return modeDockDragInsertTabs(
      layoutState,
      dragState,
      (windowId) => windowsById.get(windowId)?.title ?? windowId,
      (windowId) => windowsById.get(windowId)?.iconId ?? "app-window",
    );
  }, [dragState, layoutState, templateDrag, windowsById]);

  const noopDrag = reactHostPort.useCallback(() => {}, []);
  const canMaximize = modeCollectWindowIds(layoutState).length > 1;
  return { canMaximize, previewDragState, tabInsertPreview, draggedInsertTabs, noopDrag, handleExternalTemplateDragOver, handleExternalTemplateDrop, draggedPreviewTitle, draggedPreviewIconId };
}

/** ⌨ Builds the new window callback, the keybindings, the dock and render contexts and the rendered body. */
export function useModeShortcutsAndContexts({ canMaximize, maximizedStackPath, setMaximizedStackPath, onWindowOpenInNewWindow, activeWindowId, closeWindow, layoutState, toggleMaximize, previewDragState, tabInsertPreview, draggedInsertTabs, registerStackDropTargets, mobile, noopDrag, startTabDrag, clearPendingDrag, activateWindow, deactivateActiveWindow, windowsById, onAxisLayoutChanged, onJoinCornerResize, registerAxisGroup, registerAxisElement, dragState, modeT, children }: Pick<ReturnType<typeof useModeTemplateDropAndPreview>, "canMaximize" | "previewDragState" | "tabInsertPreview" | "draggedInsertTabs" | "noopDrag"> & Pick<ReturnType<typeof useModeLayoutState>, "maximizedStackPath" | "setMaximizedStackPath" | "layoutState" | "windowsById" | "dragState" | "modeT"> & { readonly onWindowOpenInNewWindow: ((windowId: string) => void) | undefined; readonly activeWindowId: string | null; readonly mobile: boolean; readonly children: React.ReactNode } & Pick<ReturnType<typeof useModeWindowCallbacks>, "closeWindow" | "toggleMaximize" | "registerStackDropTargets" | "startTabDrag" | "clearPendingDrag" | "activateWindow" | "deactivateActiveWindow"> & Pick<ReturnType<typeof useModeDragEffectsAndAxisCallbacks>, "onAxisLayoutChanged" | "onJoinCornerResize" | "registerAxisGroup" | "registerAxisElement">) {
  reactHostPort.useEffect(() => {
    if (canMaximize || maximizedStackPath === null) return;
    setMaximizedStackPath(null);
  }, [canMaximize, maximizedStackPath]);

  const openWindowInNewWindow = reactHostPort.useCallback(
    (windowId: string) => {
      onWindowOpenInNewWindow?.(windowId);
    },
    [onWindowOpenInNewWindow],
  );

  useControlKeybinding(
    "ui.window.close",
    () => {
      if (!activeWindowId) return;
      closeWindow(activeWindowId);
    },
    { enabled: Boolean(activeWindowId) },
    [activeWindowId, closeWindow],
  );

  useControlKeybinding(
    "ui.window.focus",
    () => {
      if (!activeWindowId) return;
      const stackPath = resolveStackPathForWindowId(layoutState, activeWindowId);
      if (stackPath === null) return;
      toggleMaximize(stackPath);
    },
    { enabled: Boolean(activeWindowId) && canMaximize },
    [activeWindowId, canMaximize, layoutState, toggleMaximize],
  );

  useControlKeybinding(
    "ui.window.newWindow",
    () => {
      if (!activeWindowId) return;
      openWindowInNewWindow(activeWindowId);
    },
    { enabled: Boolean(activeWindowId) && Boolean(onWindowOpenInNewWindow) },
    [activeWindowId, onWindowOpenInNewWindow, openWindowInNewWindow],
  );

  const dockContext = reactHostPort.useMemo<ModeDockContextValue>(
    () => ({
      dragState: previewDragState,
      tabInsertPreview,
      draggedInsertTabs,
      registerStackDropTargets,
      startTabDrag: mobile ? noopDrag : startTabDrag,
      clearPendingDrag,
      closeWindow,
      activateWindow,
      deactivateActiveWindow,
      maximizedStackPath,
      canMaximize,
      toggleMaximize,
    }),
    [mobile, noopDrag, previewDragState, tabInsertPreview, draggedInsertTabs, registerStackDropTargets, startTabDrag, clearPendingDrag, closeWindow, activateWindow, deactivateActiveWindow, maximizedStackPath, canMaximize, toggleMaximize],
  );

  const renderContext = reactHostPort.useMemo<ModeRenderContext>(
    () => ({ windowsById, activeWindowId, onAxisLayoutChanged, onJoinCornerResize, registerAxisGroup, registerAxisElement }),
    [windowsById, activeWindowId, onAxisLayoutChanged, onJoinCornerResize, registerAxisGroup, registerAxisElement],
  );

  const dockOutLayout = reactHostPort.useMemo(() => (dragState ? modeDockOutLayout(layoutState, dragState) : layoutState), [layoutState, dragState]);

  const maximizedStack =
    maximizedStackPath !== null
      ? (() => {
          let found: WindowLayoutStackNode | null = null;
          mapLayoutStacks(dockOutLayout, (stack, path) => {
            if (path === maximizedStackPath) found = stack;
            return stack;
          });
          return found;
        })()
      : null;

  const orderedWindowIds = modeCollectWindowIds(dockOutLayout);
  const hasWindows = orderedWindowIds.length > 0;
  const emptyShellNotice = resolveTranslationLabel(modeT("ui.display.emptyShell"));

  /** 📱️ Mobile has no split-pane window manager: every window collapses into one tab stack, rendered through the same {@link ModeDockStack} chrome (tab bar, utility bar, measures, engagement) as desktop. */
  const mobileFlatStack: WindowLayoutStackNode | null = mobile
    ? {
        kind: "stack",
        children: orderedWindowIds.map((id) => ({ kind: "window", id })),
        activeId: (activeWindowId && orderedWindowIds.includes(activeWindowId) ? activeWindowId : orderedWindowIds[0]) ?? undefined,
      }
    : null;

  const body =
    children ??
    (mobile ? (
      mobileFlatStack && mobileFlatStack.children.length > 0 ? (
        <ModeDockContext.Provider value={dockContext}>
          <ModeDockStack stackPath="" node={mobileFlatStack} windowsById={windowsById} activeWindowId={activeWindowId} mobile />
        </ModeDockContext.Provider>
      ) : null
    ) : maximizedStack ? (
      <ModeDockContext.Provider value={dockContext}>
        <ModeDockStack stackPath={maximizedStackPath!} node={maximizedStack} windowsById={windowsById} activeWindowId={activeWindowId} />
      </ModeDockContext.Provider>
    ) : (
      <ModeDockContext.Provider value={dockContext}>{renderModeDockNode(dockOutLayout as WindowLayoutAxisNode | WindowLayoutStackNode, "", renderContext)}</ModeDockContext.Provider>
    ));
  return { hasWindows, emptyShellNotice, body };
}
