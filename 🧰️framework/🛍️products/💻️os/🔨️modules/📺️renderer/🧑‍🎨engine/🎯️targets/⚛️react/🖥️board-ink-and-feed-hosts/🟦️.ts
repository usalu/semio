/** 🖥️ Board, ink and feed host exports of the renderer entry: the board 2d host, the icon render host, the ink canvas host and the graph timeline, block list, diff view and event feed hosts. */
import { Board2dHost } from "@semio-tech/framework-os/modules/renderer/engine/elements/Board2dHost";
import { board2dCameraActionArgs, parseBoardCatalogueDragPayload, parseBoard2dTransformFlags, parseBoard2dAreaBrushSize, board2dStatusJson, type Board2dStatus, board2dGranularityById, board2dHoverActionArgs, latestBoard2dHoverId, parseBoard2dSuggestionMenu, board2dSuggestionMenuOwnsWindow, board2dSuggestionMenuItems, type Board2dSuggestionMenu, type Board2dSuggestionCandidate } from "@semio-tech/framework-os/modules/renderer/engine/elements/Board2dHost/board-parsing";
import { coalesceBoard2dEvents, type BoardLiveMirrorMutations, collectBoardLiveMirrorMutations } from "@semio-tech/framework-os/modules/renderer/engine/elements/Board2dHost/board-events";
import { boardDropPreviewJson, boardScreenToWorld, boardWorldToScreen } from "@semio-tech/framework-os/modules/renderer/engine/elements/Board2dHost/drop-and-camera";
import { registerBoard2dPeer, unregisterBoard2dPeer, board2dPeers, beginBoardPeerGesture, endBoardPeerGesture, boardPeerOwnsGesture, pushBoardLiveMirrorMutations, notifyBoardPeersGestureEnded, pushBoardDropPreview, applyBoard2dHighlightedIds } from "@semio-tech/framework-os/modules/renderer/engine/elements/Board2dHost/session-sync";
import { IconRenderHost } from "@semio-tech/framework-os/modules/renderer/engine/elements/IconRenderHost";
import { InkCanvasHost } from "@semio-tech/framework-os/modules/renderer/engine/elements/InkCanvasHost";
import { type Vec2, type InkCamera, type InkTextRun, type InkTextParagraph, type InkItemBase, type InkTextItem, type InkImageAsset, type InkImageItem, type InkTableCell, type InkTableItem, type InkMathItem, type InkStrokeItem, type InkGroupItem, type InkItem, type InkItemKind, type InkDocument, type InkResizeHandle, type InkBounds, type InkCanvasEvent, parseSelectionIds } from "@semio-tech/framework-os/modules/renderer/engine/elements/InkCanvasHost/ink-model";
import { createInkHostId, inkPositiveMod, inkSnapWorldCoordinate, inkSnapWorldPoint, screenToWorld, worldToScreen, inkItemBounds, flattenInkItems, findInkItem, inkSelectionBounds, inkScaleItemWithinGroup, inkResizeBounds, inkItemsAtPoint, inkItemsIntersectingRect, inkTableCellAtPoint, eraseInkStrokeEventsAtPoint, eraseInkStrokePointsInItem, eraseInkStrokePointEventsNearPoint } from "@semio-tech/framework-os/modules/renderer/engine/elements/InkCanvasHost/ink-geometry";
import { inkTextParagraphsFromPlainText, inkTextPlainText, inkParagraphsToHtml, inkHtmlToParagraphs, inkImageAssetDataUrl, type InkClipboardPayload, inkClipboardPayload, inkItemsFromClipboardPayload, cloneInkItemsWithOffset, createInkItemByKind, applyInkCanvasEventLocal } from "@semio-tech/framework-os/modules/renderer/engine/elements/InkCanvasHost/ink-document-ops";
import { type InkMathRenderer, setInkMathRenderer } from "@semio-tech/framework-os/modules/renderer/engine/elements/InkCanvasHost/ink-block-views";
import { GraphTimelineHost } from "@semio-tech/framework-os/modules/renderer/engine/elements/GraphTimelineHost";
import { BlockListHost } from "@semio-tech/framework-os/modules/renderer/engine/elements/BlockListHost";
import { DiffViewHost } from "@semio-tech/framework-os/modules/renderer/engine/elements/DiffViewHost";
import { EventFeedHost } from "@semio-tech/framework-os/modules/renderer/engine/elements/EventFeedHost";

//#region 🔖️Board2dHost
export {
  board2dCameraActionArgs,
  parseBoardCatalogueDragPayload,
  coalesceBoard2dEvents,
  parseBoard2dTransformFlags,
  parseBoard2dAreaBrushSize,
  board2dStatusJson,
  type Board2dStatus,
  board2dGranularityById,
  board2dHoverActionArgs,
  latestBoard2dHoverId,
  parseBoard2dSuggestionMenu,
  board2dSuggestionMenuOwnsWindow,
  board2dSuggestionMenuItems,
  type Board2dSuggestionMenu,
  type Board2dSuggestionCandidate,
  type BoardLiveMirrorMutations,
  collectBoardLiveMirrorMutations,
  boardDropPreviewJson,
  boardScreenToWorld,
  boardWorldToScreen,
  registerBoard2dPeer,
  unregisterBoard2dPeer,
  board2dPeers,
  beginBoardPeerGesture,
  endBoardPeerGesture,
  boardPeerOwnsGesture,
  pushBoardLiveMirrorMutations,
  notifyBoardPeersGestureEnded,
  pushBoardDropPreview,
  applyBoard2dHighlightedIds,
  Board2dHost,
};
//#endregion 🔖️Board2dHost

//#region 🔖️IconRenderHost
export { IconRenderHost };
//#endregion 🔖️IconRenderHost

//#region 🔖️InkCanvasHost
export {
  type Vec2,
  type InkCamera,
  type InkTextRun,
  type InkTextParagraph,
  type InkItemBase,
  type InkTextItem,
  type InkImageAsset,
  type InkImageItem,
  type InkTableCell,
  type InkTableItem,
  type InkMathItem,
  type InkStrokeItem,
  type InkGroupItem,
  type InkItem,
  type InkItemKind,
  type InkDocument,
  type InkResizeHandle,
  type InkBounds,
  type InkCanvasEvent,
  parseSelectionIds,
  createInkHostId,
  inkPositiveMod,
  inkSnapWorldCoordinate,
  inkSnapWorldPoint,
  screenToWorld,
  worldToScreen,
  inkItemBounds,
  flattenInkItems,
  findInkItem,
  inkSelectionBounds,
  inkScaleItemWithinGroup,
  inkResizeBounds,
  inkItemsAtPoint,
  inkItemsIntersectingRect,
  inkTableCellAtPoint,
  eraseInkStrokeEventsAtPoint,
  eraseInkStrokePointsInItem,
  eraseInkStrokePointEventsNearPoint,
  inkTextParagraphsFromPlainText,
  inkTextPlainText,
  inkParagraphsToHtml,
  inkHtmlToParagraphs,
  inkImageAssetDataUrl,
  type InkClipboardPayload,
  inkClipboardPayload,
  inkItemsFromClipboardPayload,
  cloneInkItemsWithOffset,
  createInkItemByKind,
  applyInkCanvasEventLocal,
  type InkMathRenderer,
  setInkMathRenderer,
  InkCanvasHost,
};
//#endregion 🔖️InkCanvasHost

//#region 🔖️GraphTimelineHost
export { GraphTimelineHost };
//#endregion 🔖️GraphTimelineHost

//#region 🔖️BlockListHost
export { BlockListHost };
//#endregion 🔖️BlockListHost

//#region 🔖️DiffViewHost
export { DiffViewHost };
//#endregion 🔖️DiffViewHost

//#region 🔖️EventFeedHost
export { EventFeedHost };
//#endregion 🔖️EventFeedHost
