/** 🖥️ Board, ink and feed host exports of the renderer entry: the board 2d host, the icon render host, the ink canvas host and the graph timeline, block list, diff view and event feed hosts. */
import { Board2dHost } from "../../../🧱️elements/🖥️Board2dHost/🟦️.tsx";
import { board2dCameraActionArgs, parseBoardCatalogueDragPayload, parseBoard2dTransformFlags, parseBoard2dAreaBrushSize, board2dStatusJson, type Board2dStatus, board2dGranularityById, board2dHoverActionArgs, latestBoard2dHoverId, parseBoard2dSuggestionMenu, board2dSuggestionMenuOwnsWindow, board2dSuggestionMenuItems, type Board2dSuggestionMenu, type Board2dSuggestionCandidate } from "../../../🧱️elements/🖥️Board2dHost/🔬️board-parsing/🟦️.ts";
import { coalesceBoard2dEvents, type BoardLiveMirrorMutations, collectBoardLiveMirrorMutations } from "../../../🧱️elements/🖥️Board2dHost/📡️board-events/🟦️.ts";
import { boardDropPreviewJson, boardScreenToWorld, boardWorldToScreen } from "../../../🧱️elements/🖥️Board2dHost/📍️drop-and-camera/🟦️.ts";
import { registerBoard2dPeer, unregisterBoard2dPeer, board2dPeers, beginBoardPeerGesture, endBoardPeerGesture, boardPeerOwnsGesture, pushBoardLiveMirrorMutations, notifyBoardPeersGestureEnded, pushBoardDropPreview, applyBoard2dHighlightedIds } from "../../../🧱️elements/🖥️Board2dHost/🔄️session-sync/🟦️.ts";
import { IconRenderHost } from "../../../🧱️elements/🖼️IconRenderHost/🟦️.tsx";
import { InkCanvasHost } from "../../../🧱️elements/🖋️InkCanvasHost/🟦️.tsx";
import { type Vec2, type InkCamera, type InkTextRun, type InkTextParagraph, type InkItemBase, type InkTextItem, type InkImageAsset, type InkImageItem, type InkTableCell, type InkTableItem, type InkMathItem, type InkStrokeItem, type InkGroupItem, type InkItem, type InkItemKind, type InkDocument, type InkResizeHandle, type InkBounds, type InkCanvasEvent, parseSelectionIds } from "../../../🧱️elements/🖋️InkCanvasHost/🧾️ink-model/🟦️.ts";
import { createInkHostId, inkPositiveMod, inkSnapWorldCoordinate, inkSnapWorldPoint, screenToWorld, worldToScreen, inkItemBounds, flattenInkItems, findInkItem, inkSelectionBounds, inkScaleItemWithinGroup, inkResizeBounds, inkItemsAtPoint, inkItemsIntersectingRect, inkTableCellAtPoint, eraseInkStrokeEventsAtPoint, eraseInkStrokePointsInItem, eraseInkStrokePointEventsNearPoint } from "../../../🧱️elements/🖋️InkCanvasHost/📐️ink-geometry/🟦️.ts";
import { inkTextParagraphsFromPlainText, inkTextPlainText, inkParagraphsToHtml, inkHtmlToParagraphs, inkImageAssetDataUrl, type InkClipboardPayload, inkClipboardPayload, inkItemsFromClipboardPayload, cloneInkItemsWithOffset, createInkItemByKind, applyInkCanvasEventLocal } from "../../../🧱️elements/🖋️InkCanvasHost/🌳️ink-document-ops/🟦️.ts";
import { type InkMathRenderer, setInkMathRenderer } from "../../../🧱️elements/🖋️InkCanvasHost/🧱️ink-block-views/🟦️.tsx";
import { GraphTimelineHost } from "../../../🧱️elements/🌳️GraphTimelineHost/🟦️.tsx";
import { BlockListHost } from "../../../🧱️elements/🧩️BlockListHost/🟦️.tsx";
import { DiffViewHost } from "../../../🧱️elements/🔺️DiffViewHost/🟦️.tsx";
import { EventFeedHost } from "../../../🧱️elements/📡️EventFeedHost/🟦️.tsx";

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
