import type { WindowConfig } from "../../🪟️Window/🟦️.tsx";
import type { UiLabel } from "../../🎗️UiLabel/🟦️.tsx";
import type { IconName } from "@semio-tech/assets";
import * as React from "react";
import { ephemeralBox } from "@semio-tech/framework-transient";
import { panelGhostSessionBridge } from "../../👻️PanelGhost/🟦️.tsx";
import type { TreeDragAndDropController } from "../../🌳️Tree/🧲️drag-drop/🟦️.tsx";
import type { WindowStackCorner } from "../../../🔨️modules/🗺️window-layout/🟦️.ts";

/** 🪟️ Window descriptor rendered inside {@link Mode}. */
export interface ModeWindowDescriptor extends Omit<WindowConfig, "children" | "onOpenInNewWindow" | "onMaximize" | "onMinimize" | "onClose"> {
  title?: UiLabel;
  iconId: IconName;
  children: React.ReactNode;
}

export const COMPOSE_WINDOW_TEMPLATE_MIME = "application/x-compose-window-template";

/** 👻️ Ephemeral window id used while previewing an external template drag on {@link Mode}. */
export const MODE_TEMPLATE_PREVIEW_WINDOW_ID = "__compose-mode-template-preview__";

export interface WindowTemplateDragSession {
  readonly payload: WindowTemplateDropPayload;
  readonly label: string;
}

const activeWindowTemplateDragSession = ephemeralBox<WindowTemplateDragSession | null>("framework.modules.ui.elements.Canvas.component.tsx.activeWindowTemplateDragSession", null);

/** 🪟️ Records the active palette template drag until drop or dragend. */
export function beginWindowTemplateDrag(session: WindowTemplateDragSession): void {
  activeWindowTemplateDragSession.current = session;
  panelGhostSessionBridge?.begin(null);
}

/** 🪟️ Clears the active palette template drag session. */
export function endWindowTemplateDrag(): void {
  activeWindowTemplateDragSession.current = null;
  panelGhostSessionBridge?.end();
}

/** 🪟️ Returns the in-flight palette template drag, if any. */
export function readActiveWindowTemplateDragSession(): WindowTemplateDragSession | null {
  return activeWindowTemplateDragSession.current;
}

/** 🖱️ Active pointer-driven window-template drag from the Display tree. */
export const windowTemplatePointerDragRef = { active: false };

/** 🖱️ Marks a pointer-driven window-template drag as active. */
export function beginWindowTemplatePointerDrag(_encoded: string): void {
  windowTemplatePointerDragRef.active = true;
}

/** 🖱️ Cancels a pointer-driven window-template drag. */
export function cancelWindowTemplatePointerDrag(): void {
  windowTemplatePointerDragRef.active = false;
  endWindowTemplateDrag();
}

/** 🖱️ {@link TreeDragAndDropController} for Display rows that carry window-template `dragData`. */
export function windowTemplatePaletteTreeDragController(): TreeDragAndDropController {
  const readEncoded = (dragData: Record<string, string> | undefined): string | undefined => {
    const payload = dragData?.[COMPOSE_WINDOW_TEMPLATE_MIME];
    return payload?.trim() ? payload : undefined;
  };
  return {
    pointerPaletteDrag: {
      readEncodedDragPayload: readEncoded,
      begin: beginWindowTemplatePointerDrag,
      cancel: cancelWindowTemplatePointerDrag,
    },
    onDragStart: ({ sourceItem }) => {
      const encoded = readEncoded(sourceItem.dragData);
      if (!encoded) {
        return;
      }
      try {
        const payload = JSON.parse(encoded) as WindowTemplateDropPayload;
        if (typeof payload.windowKindId !== "string") {
          return;
        }
        const label = typeof sourceItem.label === "string" ? sourceItem.label : typeof sourceItem.label === "number" ? String(sourceItem.label) : "Window";
        beginWindowTemplateDrag({ payload, label });
      } catch {}
    },
    onDragEnd: () => {
      if (windowTemplatePointerDragRef.active) {
        return;
      }
      endWindowTemplateDrag();
    },
  };
}

export type ModeCanvasDropTarget = { kind: "tab"; stackPath: string; index: number; corner?: WindowStackCorner } | { kind: "split"; stackPath: string; side: "left" | "right" | "top" | "bottom" } | { kind: "root-split"; side: "left" | "right" | "top" | "bottom" };

export interface WindowTemplateDropPayload {
  readonly windowKindId: string;
  readonly templateId?: string;
}
