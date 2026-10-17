/** ✂️ InkCanvasHost editing and clipboard: the context menu, the text and table edit commits and the image paste, copy and paste handlers. */
import React, { useCallback } from "react";
import { openSurfaceContextMenu } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { type InkItem, type InkTextItem, type InkTextParagraph } from "../🧾️ink-model/🟦️.ts";
import { createInkHostId, findInkItem, inkItemsAtPoint, inkMaybeSnapWorldPoint, screenToWorld } from "../📐️ink-geometry/🟦️.ts";
import { cloneInkItemsWithOffset, createInkItemByKind, inkClipboardPayload, inkItemsFromClipboardPayload, inkTextParagraphsFromPlainText, inkTextPlainText } from "../🌳️ink-document-ops/🟦️.ts";
import type { InkCore3 } from "../👆️ink-pointer/🟦️.ts";

// #region InkCanvasHost
/** ✂️ Builds the context menu, edit commit and clipboard handlers of the ink canvas. */
export function useInkEditAndClipboard(core: InkCore3) {
  const { atomicGesture, doc, mapContextMenu, node, publishInteractionSelection, requestContextMenu, rootRef, selectedIds, selectedSet, setContextMenu, setTableEdit, setTextEdit, shellContextMenuFallback, textEdit, windowInstanceId } = core;
  //#region ContextMenu
  const handleContextMenu = useCallback(
    (event: React.MouseEvent<HTMLDivElement>): void => {
      if (!rootRef.current || !doc || !requestContextMenu) return;
      event.preventDefault();
      event.stopPropagation();
      const rect = rootRef.current.getBoundingClientRect();
      const [worldX, worldY] = screenToWorld(doc.camera, event.clientX - rect.left, event.clientY - rect.top);
      const hitItems = inkItemsAtPoint(doc.blocks, worldX, worldY);
      const top = hitItems[0];
      const selectionIds = top && !selectedSet.has(top.id) ? [top.id] : selectedIds;
      if (top && !selectedSet.has(top.id)) publishInteractionSelection([top], "replace", "pick");
      const hits = hitItems.map((item) => ({ domain: "block", id: item.id }));
      void (async () => {
        const menu = await openSurfaceContextMenu(
          requestContextMenu,
          {
            menu: { id: "inkCanvas", args: null },
            surface: {
              surfaceId: node.surfaceId,
              kind: "inkCanvas",
              hits,
              selection: selectionIds.length > 0 ? [{ domain: "block", ids: selectionIds }] : [],
            },
            windowInstanceId: windowInstanceId ?? undefined,
            point: { x: event.clientX, y: event.clientY },
          },
          mapContextMenu,
          shellContextMenuFallback,
        );
        setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
      })();
    },
    [doc, mapContextMenu, node.surfaceId, publishInteractionSelection, requestContextMenu, selectedIds, selectedSet, shellContextMenuFallback, windowInstanceId],
  );
  //#endregion ContextMenu

  const commitTextEdit = useCallback(
    (blockId: string, paragraphs: readonly InkTextParagraph[], created?: boolean) => {
      if (!doc) {
        setTextEdit(null);
        return;
      }
      const block = findInkItem(doc, blockId);
      if (!block || block.kind !== "text") {
        setTextEdit(null);
        return;
      }
      const plain = inkTextPlainText(paragraphs).trim();
      if (!plain && created) {
        atomicGesture([{ operation: "removeBlock", blockId }]);
        publishInteractionSelection([], "replace", "pick");
      } else {
        atomicGesture([{ operation: "updateBlock", blockId, block: { ...block, paragraphs } }]);
      }
      setTextEdit(null);
    },
    [atomicGesture, doc, publishInteractionSelection],
  );

  const commitTableEdit = useCallback(
    (blockId: string, row: number, col: number, content: string, advance?: boolean) => {
      if (!doc) {
        setTableEdit(null);
        return;
      }
      const block = findInkItem(doc, blockId);
      if (!block || block.kind !== "table") {
        setTableEdit(null);
        return;
      }
      const rows = block.rows.map((entry, rowIndex) => (rowIndex === row ? entry.map((cell, colIndex) => (colIndex === col ? { content } : cell)) : entry));
      atomicGesture([{ operation: "updateBlock", blockId, block: { ...block, rows } }]);
      if (advance) {
        const nextCol = col + 1 < block.columns.length ? col + 1 : 0;
        const nextRow = col + 1 < block.columns.length ? row : row + 1;
        if (nextRow < block.rows.length) setTableEdit({ blockId, row: nextRow, col: nextCol });
        else setTableEdit(null);
        return;
      }
      setTableEdit(null);
    },
    [atomicGesture, doc],
  );

  const pasteImageAsset = useCallback(
    (dataUrl: string, mime: string, worldX: number, worldY: number) => {
      const assetKey = `asset-${createInkHostId("image")}`;
      const imageBlock = createInkItemByKind("image", worldX - 120, worldY - 80);
      if (imageBlock.kind !== "image") return;
      atomicGesture(
        [
          { operation: "putAsset", key: assetKey, asset: { mime, data: dataUrl } },
          { operation: "addBlock", block: { ...imageBlock, imageKey: assetKey } },
        ],
        [imageBlock.id],
      );
    },
    [atomicGesture],
  );

  const handleCopy = useCallback(
    (event: React.ClipboardEvent<HTMLDivElement>) => {
      if (!doc) return;
      if (textEdit && (event.target as HTMLElement).closest("[contenteditable]")) return;
      if (!selectedIds.length) return;
      const blocks = selectedIds.map((id) => findInkItem(doc, id)).filter((block): block is InkItem => Boolean(block));
      if (!blocks.length) return;
      event.preventDefault();
      event.clipboardData.setData("text/plain", inkClipboardPayload(blocks));
    },
    [doc, selectedIds, textEdit],
  );

  const handlePaste = useCallback(
    (event: React.ClipboardEvent<HTMLDivElement>) => {
      if (!doc || !rootRef.current) return;
      if (textEdit && (event.target as HTMLElement).closest("[contenteditable]")) return;
      event.preventDefault();
      const rect = rootRef.current.getBoundingClientRect();
      const [worldX, worldY] = inkMaybeSnapWorldPoint(doc, ...screenToWorld(doc.camera, rect.width / 2, rect.height / 2));
      for (const item of event.clipboardData.items) {
        if (item.type.startsWith("image/")) {
          const file = item.getAsFile();
          if (!file) continue;
          const reader = new FileReader();
          reader.onload = () => {
            if (typeof reader.result === "string") pasteImageAsset(reader.result, file.type, worldX, worldY);
          };
          reader.readAsDataURL(file);
          return;
        }
      }
      const text = event.clipboardData.getData("text/plain");
      const clipboardBlocks = inkItemsFromClipboardPayload(text);
      if (clipboardBlocks) {
        const clones = cloneInkItemsWithOffset(clipboardBlocks, worldX, worldY);
        atomicGesture(
          clones.map((block) => ({ operation: "addBlock", block }) as const),
          clones.map((block) => block.id),
        );
        return;
      }
      if (text.trim().startsWith("<svg")) {
        const assetKey = `asset-${createInkHostId("image")}`;
        const imageBlock = createInkItemByKind("image", worldX - 120, worldY - 80);
        if (imageBlock.kind !== "image") return;
        atomicGesture(
          [
            { operation: "putAsset", key: assetKey, asset: { mime: "image/svg+xml", data: text.trim() } },
            { operation: "addBlock", block: { ...imageBlock, imageKey: assetKey } },
          ],
          [imageBlock.id],
        );
        return;
      }
      if (text.trim()) {
        const block = createInkItemByKind("text", worldX, worldY);
        const seeded: InkTextItem = { ...(block as InkTextItem), paragraphs: inkTextParagraphsFromPlainText(text.trim()) };
        atomicGesture([{ operation: "addBlock", block: seeded }], [seeded.id]);
      }
    },
    [atomicGesture, doc, pasteImageAsset, textEdit],
  );
  return { commitTableEdit, commitTextEdit, handleContextMenu, handleCopy, handlePaste };
}

export type InkCore4 = InkCore3 & ReturnType<typeof useInkEditAndClipboard>;
// #endregion InkCanvasHost
