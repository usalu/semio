import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/🟦️.tsx";
import { useShellScopeOptional, useShellFloatingSurfaceHost } from "../../🐚️ShellScope/🟦️.tsx";
import { createPortal } from "react-dom";
import { ephemeralBox } from "../../../../🫧️transient/🟦️.ts";
import type { TreeDataItem, TreeDataSection } from "../📋️data/🟦️.ts";

/** 🖱️ Pointer-driven external drag when native `draggable` does not start inside scroll panels. */
export interface TreePointerPaletteDragController {
  readEncodedDragPayload: (dragData: Record<string, string>) => string | undefined;
  begin: (encoded: string, types: readonly string[]) => void;
  cancel: () => void;
}

export interface TreeDragAndDropController {
  getDragData?: (context: { items: TreeDataItem[]; sourceItem: TreeDataItem; section: TreeDataSection }) => Record<string, string> | undefined;
  pointerPaletteDrag?: TreePointerPaletteDragController;
  onDragStart?: (context: { items: TreeDataItem[]; sourceItem: TreeDataItem; section: TreeDataSection }) => void;
  onDragEnd?: (context: { items: TreeDataItem[]; sourceItem: TreeDataItem; section: TreeDataSection }) => void;
  handleDrop?: (context: { target: TreeDataItem | TreeDataSection; targetKind: "item" | "section"; data: Record<string, string>; sourceItems: TreeDataItem[]; section: TreeDataSection; dropPosition?: TreeDropPosition }) => void | Promise<void>;
}

export type TreeDropPosition = "before" | "after" | "inside";

/** 📍️ Resolves whether a tree drop lands before, after, or inside a row. */
export function resolveTreeDropPosition(event: React.DragEvent<HTMLElement>): TreeDropPosition {
  const rect = event.currentTarget.getBoundingClientRect();
  const y = event.clientY - rect.top;
  if (y < rect.height * 0.25) return "before";
  if (y > rect.height * 0.75) return "after";
  return "inside";
}

/** 🔀️ True when a drag carries only internal tree reorder data (not palette payloads). */
export function isTreeReorderDragEvent(event: React.DragEvent): boolean {
  const types = [...event.dataTransfer.types];
  if (!types.includes("application/vnd.code.tree.item")) return false;
  return !types.some((kind) => kind !== "application/vnd.code.tree.item" && kind.startsWith("application/"));
}

/** 👁️ Fixed overlay showing where a tree reorder drop will land. */
export function TreeReorderDropPreview(props: { readonly preview: { readonly targetId: string; readonly position: TreeDropPosition } | null }): React.ReactElement | null {
  const [frame, setFrame] = reactHostPort.useState<DOMRect | null>(null);
  // 🐚️ Falls back to `document`/`document.body` outside any shell — inside one, scopes the id lookup and
  // portal target to that shell's own root/overlay layer (a tree row id is not guaranteed unique across
  // several mounted shells of the same plugin).
  const shellScope = useShellScopeOptional();
  reactHostPort.useLayoutEffect(() => {
    if (!props.preview || typeof document === "undefined") {
      setFrame(null);
      return;
    }
    const searchRoot: ParentNode = shellScope?.rootRef.current ?? document;
    const row = searchRoot.querySelector(`[id=${JSON.stringify(props.preview.targetId)}]`);
    setFrame(row?.getBoundingClientRect() ?? null);
  }, [props.preview, shellScope]);
  const portalTarget = useShellFloatingSurfaceHost();
  if (!props.preview || !frame || !portalTarget) return null;
  if (props.preview.position === "before") {
    return createPortal(<div data-slot="tree-drop-preview" className="pointer-events-none fixed z-tutorial h-0.5 bg-primary" style={{ left: frame.left, top: frame.top, width: frame.width }} />, portalTarget);
  }
  if (props.preview.position === "after") {
    return createPortal(<div data-slot="tree-drop-preview" className="pointer-events-none fixed z-tutorial h-0.5 bg-primary" style={{ left: frame.left, top: frame.bottom - 2, width: frame.width }} />, portalTarget);
  }
  return createPortal(
    <div data-slot="tree-drop-preview" className="pointer-events-none fixed z-tutorial border-2 border-primary/80 bg-primary/10" style={{ left: frame.left, top: frame.top, width: frame.width, height: frame.height }} />,
    portalTarget,
  );
}

export interface TreeReorderMove {
  readonly itemId: string;
  readonly fromParentId: string;
  readonly toParentId: string;
  readonly index: number;
  readonly position: TreeDropPosition;
}

export interface TreeReorderControllerOptions {
  readonly onMove: (move: TreeReorderMove) => void;
  readonly resolveParentId?: (item: TreeDataItem) => string | undefined;
}

/** 🔀️ Builds a {@link TreeDragAndDropController} that emits cross-container reorder moves. */
export function treeReorderDragController(options: TreeReorderControllerOptions): TreeDragAndDropController {
  return {
    handleDrop: ({ target, targetKind, sourceItems, dropPosition }) => {
      const sourceItem = sourceItems[0];
      if (!sourceItem || targetKind !== "item") return;
      const targetItem = target as TreeDataItem;
      const fromParentId = options.resolveParentId?.(sourceItem) ?? sourceItem.id.split(":")[0] ?? "";
      const toParentId = options.resolveParentId?.(targetItem) ?? targetItem.id.split(":")[0] ?? "";
      const position = dropPosition ?? "inside";
      const index = position === "before" ? 0 : position === "after" ? Number.MAX_SAFE_INTEGER : 0;
      options.onMove({ itemId: sourceItem.id, fromParentId, toParentId, index, position });
    },
  };
}

export interface UseTreeReorderResult {
  readonly dropIndicatorId: string | null;
  readonly dropPosition: TreeDropPosition | null;
  readonly dragController: TreeDragAndDropController;
}

/** 🔀️ Hook wiring tree reorder callbacks to drop-position aware drag handling. */
export function useTreeReorder(onMove: (move: TreeReorderMove) => void, resolveParentId?: (item: TreeDataItem) => string | undefined): UseTreeReorderResult {
  const dropStateRef = reactHostPort.useRef<{ id: string | null; position: TreeDropPosition | null }>({ id: null, position: null });
  const [, bump] = reactHostPort.useState(0);
  const dragController = reactHostPort.useMemo(
    () =>
      treeReorderDragController({
        onMove,
        resolveParentId,
      }),
    [onMove, resolveParentId],
  );
  return {
    dropIndicatorId: dropStateRef.current.id,
    dropPosition: dropStateRef.current.position,
    dragController,
  };
}

export const CATALOGUE_DRAG_MIME = "application/x-semio-catalogue-item";

const activeCatalogueDragPayload = ephemeralBox<string | null>("framework.modules.ui.elements.Tree.component.tsx.activeCatalogueDragPayload", null);
const activeCataloguePointerDragData = ephemeralBox<{ readonly payload: string; readonly types: readonly string[] } | null>("framework.modules.ui.elements.Tree.component.tsx.activeCataloguePointerDragData", null);

/** 🖱️ Payload of the catalogue drag currently in flight — native HTML5 `dragover` can't read `dataTransfer` until drop, so drop targets (e.g. a canvas host previewing a fixture drop) read this instead. */
export function getActiveCatalogueDragPayload(): string | null {
  return activeCatalogueDragPayload.current;
}

/** 🖱️ Payload owned specifically by the pointer transport, separate from native HTML drag. */
export function getActiveCataloguePointerDragData(): { readonly payload: string; readonly types: readonly string[] } | null {
  return activeCataloguePointerDragData.current;
}

/** 🖱️ {@link TreeDragAndDropController} for catalogue rows carrying encoded payloads. */
export function catalogueTreeDragController(mime: string = CATALOGUE_DRAG_MIME): TreeDragAndDropController {
  const pointerRef = { active: false };
  const readEncoded = (dragData: Record<string, string> | undefined): string | undefined => {
    const payload = dragData?.[mime];
    return payload?.trim() ? payload : undefined;
  };
  return {
    pointerPaletteDrag: {
      readEncodedDragPayload: readEncoded,
      begin: (encoded, types) => {
        pointerRef.active = true;
        activeCatalogueDragPayload.current = encoded;
        activeCataloguePointerDragData.current = { payload: encoded, types: [...types] };
      },
      cancel: () => {
        pointerRef.active = false;
        activeCatalogueDragPayload.current = null;
        activeCataloguePointerDragData.current = null;
      },
    },
    onDragStart: ({ sourceItem }) => {
      const encoded = readEncoded(sourceItem.dragData) ?? null;
      activeCatalogueDragPayload.current = encoded;
      if (!pointerRef.active) activeCataloguePointerDragData.current = null;
    },
    onDragEnd: () => {
      pointerRef.active = false;
      activeCatalogueDragPayload.current = null;
      activeCataloguePointerDragData.current = null;
    },
    handleDrop: ({ data, target, targetKind, dropPosition }) => {
      const encoded = readEncoded(data);
      if (!encoded) return;
      const payload = JSON.parse(encoded) as Record<string, unknown>;
      if (targetKind === "item" && typeof (target as TreeDataItem).onClick === "function") {
        (target as TreeDataItem).onClick?.({} as React.MouseEvent, {
          path: [],
          selectedIds: [],
          sectionId: "catalogue",
        });
      }
      void payload;
      void dropPosition;
    },
  };
}
