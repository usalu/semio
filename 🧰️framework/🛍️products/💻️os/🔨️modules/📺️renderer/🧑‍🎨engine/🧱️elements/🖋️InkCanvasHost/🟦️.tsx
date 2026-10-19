import { openSurfaceContextMenu, useShellContextMenuFallback, type SurfaceContextMenuResult } from "../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/InkCanvasHost/component.tsx
/** 🖊️ `InkCanvasHost` — the freeform ink/note canvas scene host: block document model (text,
 * image, table, math, stroke, group), local-first gesture application for optimistic in-gesture
 * rendering, KaTeX-backed math rendering with a plain-text fallback, and pointer/keyboard/clipboard
 * wiring for direct-select, marquee-select, pan, pencil, eraser, and block-placement utilities. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { type ComponentSceneHostProps } from "@semio-tech/framework";
import { cn, ContextMenuController, marqueeCoverageFromGesture, screenRectFromPoints, SelectionMarquee, surfaceClass } from "@semio-tech/ui-react";
import { resolveSemanticColorHex } from "@semio-tech/ui-styling";
import { CanvasPresenceOverlayV1 } from "../👕️canvas-presence/🟦️.tsx";
import { type InkTableItem, type InkTextItem } from "./🧾️ink-model/🟦️.ts";
import { findInkItem, flattenInkItems, inkPresenceCanvasV1, worldToScreen } from "./📐️ink-geometry/🟦️.ts";
import { InkItemView, InkSelectionChrome, InkViewportGrid } from "./🧱️ink-block-views/🟦️.tsx";
import { InkTableCellEditorOverlay, InkTextEditorOverlay } from "./🪟️ink-overlays/🟦️.tsx";
import { INK_CANVAS_HOST_CLASS } from "./🧲️ink-host-state/🟦️.ts";
import { useInkSessionState } from "./🧰️ink-session-state/🟦️.ts";
import { useInkGestures } from "./🎬️ink-gestures/🟦️.ts";
import { useInkPointer } from "./👆️ink-pointer/🟦️.ts";
import { useInkEditAndClipboard, type InkCore4 } from "./✂️ink-edit-and-clipboard/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️InkCanvasHost
//#region InkCanvasHost
export function InkCanvasHost(props: ComponentSceneHostProps) {
  const core = {} as InkCore4;
  Object.assign(core, useInkSessionState(props));
  Object.assign(core, useInkGestures(core));
  Object.assign(core, useInkPointer(core));
  Object.assign(core, useInkEditAndClipboard(core));
  const { atomicGesture, commitTableEdit, commitTextEdit, contextMenu, contextMenuTitleLabel, doc, emptySceneLabel, handleBlockPointerDown, handleContextMenu, handleCopy, handleDoubleClick, handlePaste, handlePointerCancel, handlePointerDown, handlePointerDownCapture, handlePointerMove, handlePointerUp, handleResizePointerDown, handleWheel, hoveredId, isNavigator, localPresenceActor, marqueePoints, node, presencePointerRef, presenceWindowId, publishInkPresenceView, releaseContact, rootRef, scene, selectedIds, selectionBounds, setContextMenu, setTableEdit, setTextEdit, showResizeHandles, tableEdit, textEdit , presenceRuntimeKey } = core;

  if (!scene || !doc) return <div className="text-muted-foreground p-2 text-xs">{emptySceneLabel}</div>;

  const camera = doc.camera;
  const visibleBlocks = flattenInkItems(doc.blocks);
  const gridColor = resolveSemanticColorHex("border");
  const gridSpacing = doc.gridSpacing ?? 32;
  const gridSubdivisions = doc.gridSubdivisions ?? 4;
  const gridOpacity = doc.gridOpacity ?? 0.35;
  const scale = isNavigator ? Math.min(0.2, 1 / Math.max(camera.zoom, 1)) : camera.zoom;
  const editingTextBlock = textEdit ? (findInkItem(doc, textEdit.blockId) as InkTextItem | null) : null;
  const editingTableBlock = tableEdit ? (findInkItem(doc, tableEdit.blockId) as InkTableItem | null) : null;

  return (
    <div
      ref={rootRef}
      tabIndex={0}
      data-surface-id={node.surfaceId}
      data-level="base"
      className={cn(INK_CANVAS_HOST_CLASS, surfaceClass)}
      onPointerDownCapture={handlePointerDownCapture}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerLeave={() => {
        presencePointerRef.current = null;
        publishInkPresenceView(true);
      }}
      onPointerUp={(event) => {
        if (releaseContact(event.pointerId)) handlePointerUp();
      }}
      onPointerCancel={(event) => {
        if (releaseContact(event.pointerId)) handlePointerCancel();
      }}
      onWheel={handleWheel}
      onDoubleClick={handleDoubleClick}
      onContextMenu={handleContextMenu}
      onCopy={handleCopy}
      onPaste={handlePaste}
    >
      {doc.gridVisible !== false && !isNavigator ? <InkViewportGrid camera={camera} spacing={gridSpacing} subdivisions={gridSubdivisions} opacity={gridOpacity} color={gridColor} /> : null}
      <div className="absolute origin-top-left" style={{ transform: `translate(${camera.x}px, ${camera.y}px) scale(${scale})`, width: isNavigator ? 4000 : undefined, height: isNavigator ? 3000 : undefined }}>
        {visibleBlocks.map((block) => (
          <InkItemView key={block.id} block={block} assets={doc.assets} selected={selectedIds.includes(block.id)} hovered={hoveredId === block.id} hidden={textEdit?.blockId === block.id} onPointerDown={handleBlockPointerDown} />
        ))}
      </div>
      {showResizeHandles && selectionBounds ? <InkSelectionChrome camera={camera} bounds={selectionBounds} onResizePointerDown={handleResizePointerDown} /> : null}
      {editingTextBlock && textEdit?.blockId === editingTextBlock.id ? (
        <InkTextEditorOverlay
          block={editingTextBlock}
          screenBounds={{
            x: worldToScreen(camera, editingTextBlock.x, editingTextBlock.y).x,
            y: worldToScreen(camera, editingTextBlock.x, editingTextBlock.y).y,
            width: editingTextBlock.width * camera.zoom,
            height: editingTextBlock.height * camera.zoom,
          }}
          onCommit={(paragraphs) => commitTextEdit(editingTextBlock.id, paragraphs, textEdit.created)}
          onCancel={() => {
            if (textEdit.created) atomicGesture([{ operation: "removeBlock", blockId: editingTextBlock.id }]);
            setTextEdit(null);
          }}
        />
      ) : null}
      {editingTableBlock && tableEdit
        ? (() => {
            const rowHeight = editingTableBlock.height / (editingTableBlock.rows.length + 1);
            const colWidth = editingTableBlock.width / editingTableBlock.columns.length;
            const cellX = editingTableBlock.x + tableEdit.col * colWidth;
            const cellY = editingTableBlock.y + (tableEdit.row + 1) * rowHeight;
            const screen = worldToScreen(camera, cellX, cellY);
            return (
              <InkTableCellEditorOverlay
                key={`${editingTableBlock.id}:${tableEdit.row}:${tableEdit.col}`}
                block={editingTableBlock}
                row={tableEdit.row}
                col={tableEdit.col}
                screenBounds={{ x: screen.x, y: screen.y, width: colWidth * camera.zoom, height: rowHeight * camera.zoom }}
                onCommit={(content, advance) => commitTableEdit(editingTableBlock.id, tableEdit.row, tableEdit.col, content, advance)}
                onCancel={() => setTableEdit(null)}
              />
            );
          })()
        : null}
      {isNavigator ? null : (
        <CanvasPresenceOverlayV1
          runtimeKey={presenceRuntimeKey}
          windowId={presenceWindowId}
          space="canvas"
          myActor={localPresenceActor ?? ""}
          locale={typeof document !== "undefined" ? document.documentElement.lang : undefined}
          localCanvas={inkPresenceCanvasV1(camera, [rootRef.current?.clientWidth ?? 0, rootRef.current?.clientHeight ?? 0])}
          localSizePx={[rootRef.current?.clientWidth ?? 0, rootRef.current?.clientHeight ?? 0]}
          domain={scene?.interactionDomain?.id}
          scenePath={`ink/${presenceWindowId}`}
        />
      )}
      {marqueePoints.length >= 2 ? (
        <SelectionMarquee
          shape="rect"
          coverage={marqueeCoverageFromGesture({ method: "rectangle", startX: marqueePoints[0]!.x, endX: marqueePoints[marqueePoints.length - 1]!.x, path: marqueePoints })}
          rect={screenRectFromPoints(marqueePoints) ?? { x: 0, y: 0, width: 0, height: 0 }}
        />
      ) : null}
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion InkCanvasHost
//#endregion 🔖️InkCanvasHost
