import { useMapContextMenuSpecs } from "../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { WindowInstanceIdContext } from "../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🖥️Board2dHost/component.tsx
/** 🧩️ `🖥️Board2dHost` — board-2d `ComponentSceneHost`: drives the board wasm session (snapshot
 * sync, coalesced event drain/flush, marquee/pick pointer routing, catalogue catalogue-drop preview),
 * plus the cross-pane live-mirror peer registry that keeps a triptych of panes on the same
 * `controllerId` in sync during a gesture without a plugin round trip. Reuses `World3dHost`'s
 * window-instance context and `🟦️Interpreter`'s surface context-menu plumbing. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ContextMenuController } from "@semio-tech/ui-react";
import { type ComponentSceneHostProps } from "@semio-tech/framework";
import { withSuggestionSubmenu } from "../🎣️suggestion-submenu/🟦️.ts";
// 🐢️ Direct element-to-element imports — `World3dHost`/`🟦️Interpreter` already landed in a prior batch.
import { ToolRunTrace2dLayer } from "../📐️Canvas2dHost/⏯️tool-run-trace/🟦️.tsx";
import { CanvasPresenceOverlayV1 } from "../👕️canvas-presence/🟦️.tsx";
import { board2dStatusJson } from "./🔬️board-parsing/🟦️.ts";
import { useBoardSessionState } from "./🧰️board-session-state/🟦️.ts";
import { useBoardSuggestionsAndEvents } from "./💡️board-suggestions-and-events/🟦️.ts";
import { useBoardLifecycleAndSync } from "./🔁️board-lifecycle-and-sync/🟦️.ts";
import { useBoardPointerAndKeyboard } from "./👆️board-pointer-and-keyboard/🟦️.ts";
import { useBoardContextMenuAndDrop, type BoardCore5 } from "./🖱️board-context-menu-and-drop/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️Board2dHost
//#region Board2dHost
export function Board2dHost(props: ComponentSceneHostProps) {
  const core = {} as BoardCore5;
  Object.assign(core, useBoardSessionState(props));
  Object.assign(core, useBoardSuggestionsAndEvents(core));
  Object.assign(core, useBoardLifecycleAndSync(core));
  Object.assign(core, useBoardPointerAndKeyboard(core));
  Object.assign(core, useBoardContextMenuAndDrop(core));
  const { boardStatusRef, boardVitals, canvasRef, closeSuggestionMenu, containerRef, contextMenu, contextMenuTitleLabel, emptySceneLabel, interactionDomainId, localPresenceActor, localSelectionJson, node, onContextMenu, onDragLeave, onDragOver, onDrop, onToolRunTraceCursor, presenceWindowId, readContainerSize, scene, sessionError, setContextMenu, suggestionFocus, suggestionMenu, suggestionMenuTitleLabel, suggestionPopupOwnsThisWindow, suggestionRows, toolRunTraceCamera, toolRunTracePathForShape, windowInstanceId , presenceRuntimeKey } = core;

  if (sessionError) throw sessionError;
  if (!scene) return <div className="semio-board-2d-empty text-muted-foreground p-2 text-xs">{emptySceneLabel}</div>;

  return (
    <div
      ref={containerRef}
      className="semio-board-2d-host absolute inset-0 box-border min-h-0 min-w-0 overflow-hidden select-none"
      data-surface-id={node.surfaceId}
      data-window-instance-id={windowInstanceId ?? ""}
      data-board-nodes={boardVitals.nodes}
      data-board-edges={boardVitals.edges}
      data-board-handles={boardVitals.handles}
      data-board-positions-json={boardVitals.positionsJson}
      data-board-selection-json={localSelectionJson ?? scene.selectionJson}
      data-board-guest-selection-json={scene.selectionJson}
      data-board-camera-json={scene.cameraJson}
      data-board-hovered-id={scene.hoveredId ?? ""}
      data-board-highlighted-ids-json={scene.highlightedIdsJson ?? "[]"}
      data-board-active-utility={scene.activeUtility ?? ""}
      data-board-suggestion-menu-json={scene.suggestionMenuJson ?? ""}
      data-suggestion-focus={suggestionFocus === null ? undefined : String(suggestionFocus)}
      data-board-status-json={board2dStatusJson(boardStatusRef.current)}
      style={{ touchAction: "none" }}
      onContextMenu={onContextMenu}
      onDragOver={onDragOver}
      onDragLeave={onDragLeave}
      onDrop={onDrop}
    >
      <canvas ref={canvasRef} className="absolute inset-0 block size-full touch-none outline-none focus:outline-none" />
      <ToolRunTrace2dLayer lane={scene.toolRunTrace} camera={toolRunTraceCamera} pathForShape={toolRunTracePathForShape} onCursor={onToolRunTraceCursor} focus={suggestionFocus} />
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null && !suggestionPopupOwnsThisWindow}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.suggestTarget ? withSuggestionSubmenu(contextMenu.items, suggestionRows) : (contextMenu?.items ?? [])}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
      {suggestionPopupOwnsThisWindow && suggestionMenu ? (
        <ContextMenuController
          title={suggestionMenuTitleLabel}
          open
          closeOnSelect={false}
          position={{ x: suggestionMenu.x, y: suggestionMenu.y }}
          items={suggestionRows}
          onOpenChange={(open) => {
            if (!open) closeSuggestionMenu();
          }}
        />
      ) : null}
      <CanvasPresenceOverlayV1
        runtimeKey={presenceRuntimeKey}
        windowId={presenceWindowId}
        space="canvas"
        myActor={localPresenceActor ?? ""}
        locale={typeof document !== "undefined" ? document.documentElement.lang : undefined}
        localCanvas={toolRunTraceCamera}
        localSizePx={[readContainerSize().w, readContainerSize().h]}
        domain={interactionDomainId}
        scenePath={`board/${presenceWindowId}`}
      />
    </div>
  );
}
//#endregion Board2dHost
//#endregion 🔖️Board2dHost
