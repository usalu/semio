// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/InkCanvasHost/component.tsx
/** @emoji 🖊️ `InkCanvasHost` — the freeform ink/note canvas scene host: block document model (text,
 * image, table, math, stroke, group), local-first gesture application for optimistic in-gesture
 * rendering, KaTeX-backed math rendering with a plain-text fallback, and pointer/keyboard/clipboard
 * wiring for direct-select, marquee-select, pan, pencil, eraser, and block-placement utilities. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import React, { useCallback, useContext, useEffect, useMemo, useRef, useState, type KeyboardEvent } from "react";
import { type ComponentSceneHostProps, inkCanvasActions, windowElementId } from "@semio-tech/framework-core";
import {
  cn,
  ContextMenuController,
  marqueeCoverageFromGesture,
  registerIntroductionSurfaceResolver,
  screenRectFromPoints,
  SelectionMarquee,
  surfaceClass,
  useLabel,
  type ContextMenuItem,
  type IntroductionResolvedGeometry,
  type SelectionMarqueePoint,
} from "@semio-tech/ui-react";
import { resolveSemanticColorHex } from "@semio-tech/ui-styling";
import { openSurfaceContextMenu, useShellContextMenuFallback } from "../Interpreter/🟦️component.tsx";
import { hostLabel } from "../TextEditor/🟦️component.tsx";
import { WindowInstanceIdContext } from "../World3dHost/🟦️component.tsx";
import { useMapContextMenuSpecs } from "../ShellHost/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️InkCanvasHost
//#region Types
export type Vec2 = readonly [number, number];

export type InkCamera = { readonly x: number; readonly y: number; readonly zoom: number };

export interface InkTextRun {
  readonly text: string;
  readonly bold?: boolean;
  readonly italic?: boolean;
  readonly underline?: boolean;
  readonly link?: string;
}

export interface InkTextParagraph {
  readonly runs: readonly InkTextRun[];
}

export interface InkItemBase {
  readonly id: string;
  readonly name: string;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly rotation?: number;
  readonly visible: boolean;
  readonly locked: boolean;
}

export interface InkTextItem extends InkItemBase {
  readonly kind: "text";
  readonly paragraphs: readonly InkTextParagraph[];
  readonly fontSize: number;
  readonly fontWeight: "normal" | "bold";
  readonly align: "left" | "center" | "right";
}

export interface InkImageAsset {
  readonly mime: string;
  readonly data: string;
  readonly width?: number;
  readonly height?: number;
}

export interface InkImageItem extends InkItemBase {
  readonly kind: "image";
  readonly imageKey: string;
}

export interface InkTableCell {
  readonly content: string;
}

export interface InkTableItem extends InkItemBase {
  readonly kind: "table";
  readonly columns: readonly string[];
  readonly rows: readonly (readonly InkTableCell[])[];
}

export interface InkMathItem extends InkItemBase {
  readonly kind: "math";
  readonly tex: string;
  readonly displayMode: boolean;
}

export interface InkStrokeItem extends InkItemBase {
  readonly kind: "stroke";
  readonly points: readonly Vec2[];
  readonly strokeWidth: number;
  readonly color: readonly [number, number, number, number];
}

export interface InkGroupItem extends InkItemBase {
  readonly kind: "group";
  readonly children: readonly InkItem[];
}

export type InkItem = InkTextItem | InkImageItem | InkTableItem | InkMathItem | InkStrokeItem | InkGroupItem;
export type InkItemKind = InkItem["kind"];

export interface InkDocument {
  readonly schema: "ink.document";
  readonly id: string;
  readonly title?: string;
  readonly camera: InkCamera;
  readonly blocks: readonly InkItem[];
  readonly assets?: Readonly<Record<string, InkImageAsset>>;
  readonly activeUtility?: string;
  readonly gridVisible?: boolean;
  readonly gridSpacing?: number;
  readonly gridSubdivisions?: number;
  readonly gridOpacity?: number;
  readonly snapEnabled?: boolean;
  readonly snapGridSpacing?: number;
  readonly pencilWidth?: number;
  readonly eraserRadius?: number;
}

export type InkResizeHandle = "nw" | "n" | "ne" | "e" | "se" | "s" | "sw" | "w";

export interface InkBounds {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

export type InkCanvasEvent =
  | { readonly operation: "addBlock"; readonly block: InkItem; readonly parentId?: string | null; readonly index?: number | null }
  | { readonly operation: "updateBlock"; readonly blockId: string; readonly block: InkItem }
  | { readonly operation: "removeBlock"; readonly blockId: string }
  | { readonly operation: "putAsset"; readonly key: string; readonly asset: InkImageAsset }
  | { readonly operation: "setCamera"; readonly camera: InkCamera };

type InkGesturePhase = "begin" | "live" | "commit" | "atomic";

function parseInkScene(documentJson: string | undefined): InkDocument | null {
  if (!documentJson) return null;
  try {
    const parsed = JSON.parse(documentJson) as Partial<InkDocument>;
    if (parsed.schema !== "ink.document" || !Array.isArray(parsed.blocks)) return null;
    return parsed as InkDocument;
  } catch {
    return null;
  }
}

export function parseSelectionIds(json: string | undefined): readonly string[] {
  if (!json) return [];
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? parsed.filter((id): id is string => typeof id === "string") : [];
  } catch {
    return [];
  }
}
//#endregion Types

//#region GeometryHelpers
let inkHostIdCounter = 0;

/** @emoji 🆔️ Host-generated ids only need to be unique client-side (Rust re-derives its own on the next round-trip). */
export function createInkHostId(prefix: string): string {
  inkHostIdCounter += 1;
  return `${prefix}-host-${inkHostIdCounter}`;
}

export function inkPositiveMod(value: number, modulus: number): number {
  if (modulus <= 0) return 0;
  return ((value % modulus) + modulus) % modulus;
}

export function inkSnapWorldCoordinate(value: number, spacing: number): number {
  if (spacing <= 0) return value;
  return Math.round(value / spacing) * spacing;
}

export function inkSnapWorldPoint(x: number, y: number, spacing: number): Vec2 {
  return [inkSnapWorldCoordinate(x, spacing), inkSnapWorldCoordinate(y, spacing)];
}

function inkMaybeSnapWorldPoint(doc: InkDocument, x: number, y: number): Vec2 {
  if (!doc.snapEnabled) return [x, y];
  return inkSnapWorldPoint(x, y, doc.snapGridSpacing ?? 8);
}

export function screenToWorld(camera: InkCamera, screenX: number, screenY: number): Vec2 {
  return [(screenX - camera.x) / camera.zoom, (screenY - camera.y) / camera.zoom];
}

export function worldToScreen(camera: InkCamera, worldX: number, worldY: number): { readonly x: number; readonly y: number } {
  return { x: worldX * camera.zoom + camera.x, y: worldY * camera.zoom + camera.y };
}

export function inkItemBounds(block: InkItem): InkBounds {
  if (block.kind === "stroke" && block.points.length > 0) {
    let minX = block.points[0]![0];
    let minY = block.points[0]![1];
    let maxX = minX;
    let maxY = minY;
    for (const point of block.points) {
      minX = Math.min(minX, point[0]);
      minY = Math.min(minY, point[1]);
      maxX = Math.max(maxX, point[0]);
      maxY = Math.max(maxY, point[1]);
    }
    return { x: block.x + minX, y: block.y + minY, width: Math.max(1, maxX - minX), height: Math.max(1, maxY - minY) };
  }
  return { x: block.x, y: block.y, width: block.width, height: block.height };
}

export function flattenInkItems(blocks: readonly InkItem[]): InkItem[] {
  const out: InkItem[] = [];
  for (const block of blocks) {
    out.push(block);
    if (block.kind === "group") out.push(...flattenInkItems(block.children));
  }
  return out;
}

export function findInkItem(doc: InkDocument, blockId: string): InkItem | null {
  function visit(node: InkItem): InkItem | null {
    if (node.id === blockId) return node;
    if (node.kind === "group") {
      for (const child of node.children) {
        const found = visit(child);
        if (found) return found;
      }
    }
    return null;
  }
  for (const block of doc.blocks) {
    const found = visit(block);
    if (found) return found;
  }
  return null;
}

export function inkSelectionBounds(blocks: readonly InkItem[], ids: readonly string[]): InkBounds | null {
  const idSet = new Set(ids);
  const selected = flattenInkItems(blocks).filter((block) => idSet.has(block.id));
  if (!selected.length) return null;
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  for (const block of selected) {
    const bounds = inkItemBounds(block);
    minX = Math.min(minX, bounds.x);
    minY = Math.min(minY, bounds.y);
    maxX = Math.max(maxX, bounds.x + bounds.width);
    maxY = Math.max(maxY, bounds.y + bounds.height);
  }
  return { x: minX, y: minY, width: Math.max(1, maxX - minX), height: Math.max(1, maxY - minY) };
}

function scaleValue(value: number, fromMin: number, fromSize: number, toMin: number, toSize: number): number {
  if (fromSize <= 0) return toMin;
  return toMin + ((value - fromMin) / fromSize) * toSize;
}

export function inkScaleItemWithinGroup(block: InkItem, fromBounds: InkBounds, toBounds: InkBounds): InkItem {
  const bounds = inkItemBounds(block);
  const nextX = scaleValue(bounds.x, fromBounds.x, fromBounds.width, toBounds.x, toBounds.width);
  const nextY = scaleValue(bounds.y, fromBounds.y, fromBounds.height, toBounds.y, toBounds.height);
  const nextWidth = Math.max(8, scaleValue(bounds.x + bounds.width, fromBounds.x, fromBounds.width, toBounds.x, toBounds.width) - nextX);
  const nextHeight = Math.max(8, scaleValue(bounds.y + bounds.height, fromBounds.y, fromBounds.height, toBounds.y, toBounds.height) - nextY);
  if (block.kind === "stroke") {
    const scaleX = fromBounds.width > 0 ? toBounds.width / fromBounds.width : 1;
    const scaleY = fromBounds.height > 0 ? toBounds.height / fromBounds.height : 1;
    const points = block.points.map(([px, py]) => [px * scaleX, py * scaleY] as Vec2);
    return { ...block, x: nextX, y: nextY, width: nextWidth, height: nextHeight, points };
  }
  if (block.kind === "group") {
    return { ...block, x: nextX, y: nextY, width: nextWidth, height: nextHeight, children: block.children.map((child) => inkScaleItemWithinGroup(child, fromBounds, toBounds)) };
  }
  return { ...block, x: nextX, y: nextY, width: nextWidth, height: nextHeight };
}

export function inkResizeBounds(fromBounds: InkBounds, handle: InkResizeHandle, dx: number, dy: number, minSize = 8): InkBounds {
  let { x, y, width, height } = fromBounds;
  if (handle.includes("e")) width = Math.max(minSize, width + dx);
  if (handle.includes("w")) {
    const nextWidth = Math.max(minSize, width - dx);
    x += width - nextWidth;
    width = nextWidth;
  }
  if (handle.includes("s")) height = Math.max(minSize, height + dy);
  if (handle.includes("n")) {
    const nextHeight = Math.max(minSize, height - dy);
    y += height - nextHeight;
    height = nextHeight;
  }
  return { x, y, width, height };
}

export function inkItemsAtPoint(blocks: readonly InkItem[], x: number, y: number): InkItem[] {
  const hits: InkItem[] = [];
  for (const block of [...flattenInkItems(blocks)].reverse()) {
    const bounds = inkItemBounds(block);
    if (x >= bounds.x && x <= bounds.x + bounds.width && y >= bounds.y && y <= bounds.y + bounds.height) hits.push(block);
  }
  return hits;
}

export function inkItemsIntersectingRect(blocks: readonly InkItem[], rect: InkBounds): string[] {
  const hits: string[] = [];
  for (const block of flattenInkItems(blocks)) {
    const bounds = inkItemBounds(block);
    const intersects = bounds.x < rect.x + rect.width && bounds.x + bounds.width > rect.x && bounds.y < rect.y + rect.height && bounds.y + bounds.height > rect.y;
    if (intersects) hits.push(block.id);
  }
  return hits;
}

export function inkTableCellAtPoint(block: InkTableItem, localX: number, localY: number): { readonly row: number; readonly col: number } | null {
  const rowCount = block.rows.length + 1;
  const colCount = block.columns.length;
  if (rowCount <= 0 || colCount <= 0) return null;
  const rowHeight = block.height / rowCount;
  const colWidth = block.width / colCount;
  const row = Math.floor(localY / rowHeight) - 1;
  const col = Math.floor(localX / colWidth);
  if (row < 0 || row >= block.rows.length || col < 0 || col >= colCount) return null;
  return { row, col };
}

function pointToSegmentDistance(px: number, py: number, x1: number, y1: number, x2: number, y2: number): number {
  const dx = x2 - x1;
  const dy = y2 - y1;
  if (dx === 0 && dy === 0) return Math.hypot(px - x1, py - y1);
  const t = Math.max(0, Math.min(1, ((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy)));
  return Math.hypot(px - (x1 + t * dx), py - (y1 + t * dy));
}

function inkWorldPoints(block: InkStrokeItem): Vec2[] {
  return block.points.map(([px, py]) => [block.x + px, block.y + py] as Vec2);
}

function inkHitsPoint(block: InkStrokeItem, x: number, y: number, threshold: number): boolean {
  const points = inkWorldPoints(block);
  if (points.length < 2) {
    if (!points[0]) return false;
    return Math.hypot(x - points[0][0], y - points[0][1]) <= threshold;
  }
  for (let index = 1; index < points.length; index += 1) {
    const prev = points[index - 1]!;
    const next = points[index]!;
    if (pointToSegmentDistance(x, y, prev[0], prev[1], next[0], next[1]) <= threshold + block.strokeWidth / 2) return true;
  }
  return false;
}

/** @emoji 🧹️ Whole-stroke eraser: returns removeBlock events for every ink stroke under the point. */
export function eraseInkStrokeEventsAtPoint(doc: InkDocument, x: number, y: number, threshold = 8): readonly InkCanvasEvent[] {
  const hits = flattenInkItems(doc.blocks).filter((block): block is InkStrokeItem => block.kind === "stroke" && inkHitsPoint(block, x, y, threshold));
  return hits.map((block) => ({ operation: "removeBlock", blockId: block.id }));
}

/** @emoji ✂️ Splits an ink stroke into surviving point-runs after removing points within `radius` of (x, y). */
export function eraseInkStrokePointsInItem(block: InkStrokeItem, x: number, y: number, radius: number): InkStrokeItem[] {
  const keptIndices: number[] = [];
  for (let index = 0; index < block.points.length; index += 1) {
    const point = block.points[index]!;
    if (Math.hypot(block.x + point[0] - x, block.y + point[1] - y) > radius) keptIndices.push(index);
  }
  if (keptIndices.length === block.points.length) return [block];
  if (!keptIndices.length) return [];
  const runs: Vec2[][] = [];
  let current: Vec2[] = [block.points[keptIndices[0]!]!];
  for (let index = 1; index < keptIndices.length; index += 1) {
    if (keptIndices[index]! - keptIndices[index - 1]! > 1) {
      if (current.length >= 2) runs.push(current);
      current = [block.points[keptIndices[index]!]!];
    } else {
      current.push(block.points[keptIndices[index]!]!);
    }
  }
  if (current.length >= 2) runs.push(current);
  return runs.map((points, index) => ({ ...block, id: index === 0 ? block.id : createInkHostId("stroke"), name: index === 0 ? block.name : `${block.name} fragment`, points }));
}

/** @emoji ✂️ Point-eraser events: removeBlock for the original stroke, addBlock for each surviving fragment (skipped if untouched). */
export function eraseInkStrokePointEventsNearPoint(doc: InkDocument, x: number, y: number, radius: number): readonly InkCanvasEvent[] {
  const events: InkCanvasEvent[] = [];
  const inkBlocks = flattenInkItems(doc.blocks).filter((block): block is InkStrokeItem => block.kind === "stroke");
  for (const block of inkBlocks) {
    const fragments = eraseInkStrokePointsInItem(block, x, y, radius);
    if (fragments.length === 1 && fragments[0] === block) continue;
    events.push({ operation: "removeBlock", blockId: block.id });
    for (const fragment of fragments) events.push({ operation: "addBlock", block: fragment });
  }
  return events;
}

export function inkTextParagraphsFromPlainText(text: string): readonly InkTextParagraph[] {
  return text.split(/\n/).map((line) => ({ runs: [{ text: line }] }));
}

export function inkTextPlainText(paragraphs: readonly InkTextParagraph[]): string {
  return paragraphs.map((paragraph) => paragraph.runs.map((run) => run.text).join("")).join("\n");
}

export function inkParagraphsToHtml(paragraphs: readonly InkTextParagraph[]): string {
  return paragraphs
    .map((paragraph) => {
      const inner = paragraph.runs
        .map((run) => {
          let text = run.text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
          if (run.link) text = `<a href="${run.link}">${text}</a>`;
          if (run.underline) text = `<u>${text}</u>`;
          if (run.italic) text = `<em>${text}</em>`;
          if (run.bold) text = `<strong>${text}</strong>`;
          return text;
        })
        .join("");
      return `<div>${inner || "<br>"}</div>`;
    })
    .join("");
}

export function inkHtmlToParagraphs(root: HTMLElement): readonly InkTextParagraph[] {
  const paragraphs: InkTextParagraph[] = [];
  const children = root.childNodes.length ? [...root.childNodes] : [root];
  for (const child of children) {
    if (child.nodeType === Node.TEXT_NODE) {
      const text = child.textContent ?? "";
      if (text) paragraphs.push({ runs: [{ text }] });
      continue;
    }
    if (!(child instanceof HTMLElement)) continue;
    const tag = child.tagName.toLowerCase();
    if (tag === "br") {
      paragraphs.push({ runs: [{ text: "" }] });
      continue;
    }
    const runs: InkTextRun[] = [];
    const walk = (node: Node, marks: Partial<InkTextRun>) => {
      if (node.nodeType === Node.TEXT_NODE) {
        const text = node.textContent ?? "";
        if (text) runs.push({ text, ...marks });
        return;
      }
      if (!(node instanceof HTMLElement)) return;
      const nextMarks = { ...marks };
      const nodeTag = node.tagName.toLowerCase();
      if (nodeTag === "strong" || nodeTag === "b") nextMarks.bold = true;
      if (nodeTag === "em" || nodeTag === "i") nextMarks.italic = true;
      if (nodeTag === "u") nextMarks.underline = true;
      if (nodeTag === "a") nextMarks.link = node.getAttribute("href") ?? undefined;
      for (const childNode of node.childNodes) walk(childNode, nextMarks);
    };
    for (const childNode of child.childNodes) walk(childNode, {});
    if (!runs.length) runs.push({ text: "" });
    paragraphs.push({ runs });
  }
  return paragraphs.length ? paragraphs : [{ runs: [{ text: "" }] }];
}

export function inkImageAssetDataUrl(asset: InkImageAsset): string {
  if (asset.data.startsWith("data:")) return asset.data;
  if (asset.mime === "image/svg+xml") return `data:image/svg+xml;utf8,${encodeURIComponent(asset.data)}`;
  return `data:${asset.mime};base64,${asset.data}`;
}

export interface InkClipboardPayload {
  readonly schema: "ink.clipboard";
  readonly blocks: readonly InkItem[];
}

export function inkClipboardPayload(blocks: readonly InkItem[]): string {
  const payload: InkClipboardPayload = { schema: "ink.clipboard", blocks: [...blocks] };
  return JSON.stringify(payload);
}

export function inkItemsFromClipboardPayload(json: string): readonly InkItem[] | null {
  try {
    const parsed = JSON.parse(json) as InkClipboardPayload;
    if (parsed.schema !== "ink.clipboard" || !Array.isArray(parsed.blocks)) return null;
    return parsed.blocks;
  } catch {
    return null;
  }
}

function reidItemTree(block: InkItem, renameTop: boolean): InkItem {
  const id = createInkHostId(block.kind);
  const name = renameTop ? `${block.name} copy` : block.name;
  if (block.kind === "group") return { ...block, id, name, children: block.children.map((child) => reidItemTree(child, false)) };
  return { ...block, id, name };
}

export function cloneInkItemsWithOffset(blocks: readonly InkItem[], dx: number, dy: number): InkItem[] {
  return blocks.map((block) => {
    const clone = reidItemTree(block, false);
    return { ...clone, x: clone.x + dx, y: clone.y + dy };
  });
}

const INK_ITEM_DEFAULT_SIZE: Record<InkItemKind, { readonly width: number; readonly height: number }> = {
  text: { width: 280, height: 120 },
  image: { width: 240, height: 160 },
  table: { width: 320, height: 160 },
  math: { width: 200, height: 80 },
  stroke: { width: 1, height: 1 },
  group: { width: 280, height: 120 },
};

export function createInkItemByKind(kind: InkItemKind, x: number, y: number): InkItem {
  const size = INK_ITEM_DEFAULT_SIZE[kind];
  const base = { id: createInkHostId(kind), x, y, width: size.width, height: size.height, rotation: 0, visible: true, locked: false };
  if (kind === "image") return { ...base, kind, name: hostLabel("ui.host.blockImage"), imageKey: "placeholder" };
  if (kind === "table")
    return {
      ...base,
      kind,
      name: hostLabel("ui.host.blockTable"),
      columns: ["A", "B", "C"],
      rows: [
        [{ content: "" }, { content: "" }, { content: "" }],
        [{ content: "" }, { content: "" }, { content: "" }],
      ],
    };
  if (kind === "math") return { ...base, kind, name: hostLabel("ui.host.blockMath"), tex: "E = mc^2", displayMode: true };
  if (kind === "stroke") return { ...base, kind, name: hostLabel("ui.host.blockInk"), points: [], strokeWidth: 3, color: [0, 0, 0, 1] };
  if (kind === "group") return { ...base, kind, name: hostLabel("ui.host.blockGroup"), children: [] };
  return { ...base, kind: "text", name: hostLabel("ui.host.blockText"), paragraphs: [{ runs: [{ text: "" }] }], fontSize: 18, fontWeight: "normal", align: "left" };
}

/** @emoji 🖊️ Local pure application of the generic ink-apply-events operation vocabulary — mirrors the note plugin's event-apply function for optimistic in-gesture rendering. */
export function applyInkCanvasEventLocal(doc: InkDocument, event: InkCanvasEvent): InkDocument {
  switch (event.operation) {
    case "addBlock": {
      const blocks = [...doc.blocks];
      if (!event.parentId) {
        blocks.splice(event.index ?? blocks.length, 0, event.block);
        return { ...doc, blocks };
      }
      return { ...doc, blocks: insertIntoParent(doc.blocks, event.parentId, event.index ?? Number.MAX_SAFE_INTEGER, event.block) };
    }
    case "updateBlock":
      return { ...doc, blocks: updateInTree(doc.blocks, event.blockId, event.block) };
    case "removeBlock":
      return { ...doc, blocks: removeFromTree(doc.blocks, event.blockId) };
    case "putAsset":
      return { ...doc, assets: { ...(doc.assets ?? {}), [event.key]: event.asset } };
    case "setCamera":
      return { ...doc, camera: event.camera };
    default:
      return doc;
  }
}

function insertIntoParent(blocks: readonly InkItem[], parentId: string, index: number, block: InkItem): InkItem[] {
  return blocks.map((node) => {
    if (node.kind !== "group") return node;
    if (node.id === parentId) {
      const children = [...node.children];
      children.splice(Math.min(index, children.length), 0, block);
      return { ...node, children };
    }
    return { ...node, children: insertIntoParent(node.children, parentId, index, block) };
  });
}

function updateInTree(blocks: readonly InkItem[], blockId: string, nextBlock: InkItem): InkItem[] {
  return blocks.map((block) => {
    if (block.id === blockId) return nextBlock;
    if (block.kind === "group") return { ...block, children: updateInTree(block.children, blockId, nextBlock) };
    return block;
  });
}

function removeFromTree(blocks: readonly InkItem[], blockId: string): InkItem[] {
  return blocks.filter((block) => block.id !== blockId).map((block) => (block.kind === "group" ? { ...block, children: removeFromTree(block.children, blockId) } : block));
}

function applyEventsLocal(doc: InkDocument, events: readonly InkCanvasEvent[]): InkDocument {
  return events.reduce((acc, event) => applyInkCanvasEventLocal(acc, event), doc);
}
//#endregion GeometryHelpers

//#region MathRenderer
export interface InkMathRenderer {
  render(tex: string, displayMode: boolean): string;
}

let inkMathRenderer: InkMathRenderer = {
  render(tex: string, displayMode: boolean) {
    return `<span class="ink-math-fallback">${displayMode ? `$$${tex}$$` : `$${tex}$`}</span>`;
  },
};

/** @emoji ∑ Sets the active ink math renderer adapter (defaults to a plain-text fallback until KaTeX loads). */
export function setInkMathRenderer(renderer: InkMathRenderer): void {
  inkMathRenderer = renderer;
}

async function ensureKatexMathRenderer(): Promise<void> {
  try {
    const katex = await import("katex");
    await import("katex/dist/katex.min.css");
    setInkMathRenderer({
      render(tex: string, displayMode: boolean) {
        return katex.default.renderToString(tex, { displayMode, throwOnError: false });
      },
    });
  } catch {
    /* fallback renderer stays active */
  }
}

if (typeof window !== "undefined") void ensureKatexMathRenderer();
//#endregion MathRenderer

//#region BlockViews
function inkTextRunStyle(run: InkTextRun): React.CSSProperties {
  return { fontWeight: run.bold ? "bold" : undefined, fontStyle: run.italic ? "italic" : undefined, textDecoration: run.underline ? "underline" : undefined };
}

function InkTextRunView({ run }: { readonly run: InkTextRun }) {
  if (run.link) {
    return (
      <a href={run.link} className="text-primary underline" style={inkTextRunStyle(run)} onPointerDown={(event) => event.stopPropagation()}>
        {run.text}
      </a>
    );
  }
  return <span style={inkTextRunStyle(run)}>{run.text}</span>;
}

function InkTextContentView({ block }: { readonly block: InkTextItem }) {
  return (
    <div className="text-foreground h-full w-full overflow-auto p-2 whitespace-pre-wrap" style={{ fontSize: block.fontSize, fontWeight: block.fontWeight, textAlign: block.align }}>
      {block.paragraphs.map((paragraph, paragraphIndex) => (
        <div key={paragraphIndex}>
          {paragraph.runs.map((run, runIndex) => (
            <InkTextRunView key={runIndex} run={run} />
          ))}
        </div>
      ))}
    </div>
  );
}

function InkItemView({
  block,
  assets,
  selected,
  hovered,
  hidden,
  onPointerDown,
}: {
  readonly block: InkItem;
  readonly assets?: Readonly<Record<string, InkImageAsset>>;
  readonly selected: boolean;
  readonly hovered: boolean;
  readonly hidden: boolean;
  readonly onPointerDown: (event: React.PointerEvent, blockId: string) => void;
}) {
  const groupLabel = useLabel("ui.host.blockGroup");
  if (!block.visible) return null;
  const bounds = inkItemBounds(block);
  const common = {
    "data-ink-block-id": block.id,
    className: cn("bg-background/90 absolute overflow-hidden rounded border shadow-sm", selected && "ring-primary ring-2", hovered && !selected && "ring-primary/60 ring-1", block.locked && "opacity-70", hidden && "pointer-events-none opacity-0"),
    style: {
      left: bounds.x,
      top: bounds.y,
      width: Math.max(8, bounds.width),
      height: Math.max(8, bounds.height),
      transform: block.rotation ? `rotate(${block.rotation}deg)` : undefined,
    },
    onPointerDown: (event: React.PointerEvent) => onPointerDown(event, block.id),
  };
  if (block.kind === "text")
    return (
      <div {...common}>
        <InkTextContentView block={block} />
      </div>
    );
  if (block.kind === "math") {
    const html = inkMathRenderer.render(block.tex, block.displayMode);
    return (
      <div {...common}>
        <div className="flex h-full w-full items-center justify-center p-2">
          <div className="ink-math" dangerouslySetInnerHTML={{ __html: html }} />
        </div>
      </div>
    );
  }
  if (block.kind === "table") {
    return (
      <div {...common}>
        <table className="h-full w-full border-collapse text-sm">
          <thead>
            <tr>
              {block.columns.map((column) => (
                <th key={column} className="border-border border px-2 py-1 text-left font-medium">
                  {column}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {block.rows.map((row, rowIndex) => (
              <tr key={rowIndex}>
                {row.map((cell, cellIndex) => (
                  <td key={cellIndex} className="border-border border px-2 py-1 align-top">
                    {cell.content}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  }
  if (block.kind === "image") {
    const asset = assets?.[block.imageKey];
    const src = asset ? inkImageAssetDataUrl(asset) : null;
    return (
      <div {...common}>
        {src ? <img src={src} alt={block.name} className="h-full w-full object-contain" draggable={false} /> : <div className="bg-muted text-muted-foreground flex h-full w-full items-center justify-center text-xs">{block.imageKey}</div>}
      </div>
    );
  }
  if (block.kind === "stroke") {
    if (block.points.length < 2) return null;
    const path = block.points.map((point, index) => `${index === 0 ? "M" : "L"} ${block.x + point[0]} ${block.y + point[1]}`).join(" ");
    const [r, g, b, a] = block.color;
    return (
      <svg className="pointer-events-none absolute inset-0 overflow-visible" style={{ width: "100%", height: "100%" }}>
        <path d={path} fill="none" stroke={`rgba(${Math.round(r * 255)},${Math.round(g * 255)},${Math.round(b * 255)},${a})`} strokeWidth={block.strokeWidth} strokeLinecap="round" strokeLinejoin="round" />
      </svg>
    );
  }
  if (block.kind === "group") {
    return (
      <div {...common}>
        <div className="text-muted-foreground p-1 text-xs">
          {groupLabel} · {block.children.length} children
        </div>
      </div>
    );
  }
  return null;
}

const INK_RESIZE_HANDLES: readonly InkResizeHandle[] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];
const INK_RESIZE_CURSOR: Record<InkResizeHandle, string> = { nw: "nwse-resize", n: "ns-resize", ne: "nesw-resize", e: "ew-resize", se: "nwse-resize", s: "ns-resize", sw: "nesw-resize", w: "ew-resize" };

function InkSelectionChrome({ camera, bounds, onResizePointerDown }: { readonly camera: InkCamera; readonly bounds: InkBounds; readonly onResizePointerDown: (handle: InkResizeHandle, event: React.PointerEvent) => void }) {
  const topLeft = worldToScreen(camera, bounds.x, bounds.y);
  const width = bounds.width * camera.zoom;
  const height = bounds.height * camera.zoom;
  return (
    <>
      <div className="border-primary pointer-events-none absolute z-20 border" style={{ left: topLeft.x, top: topLeft.y, width, height }} />
      {INK_RESIZE_HANDLES.map((handle) => {
        const left = handle.includes("w") ? topLeft.x - 4 : handle.includes("e") ? topLeft.x + width - 4 : topLeft.x + width / 2 - 4;
        const top = handle.includes("n") ? topLeft.y - 4 : handle.includes("s") ? topLeft.y + height - 4 : topLeft.y + height / 2 - 4;
        return <div key={handle} className="border-primary bg-background absolute z-30 h-2 w-2 rounded-sm border" style={{ left, top, cursor: INK_RESIZE_CURSOR[handle] }} onPointerDown={(event) => onResizePointerDown(handle, event)} />;
      })}
    </>
  );
}

function InkViewportGrid({ camera, spacing, subdivisions, opacity, color }: { readonly camera: InkCamera; readonly spacing: number; readonly subdivisions: number; readonly opacity: number; readonly color: string }) {
  const majorPx = spacing * camera.zoom;
  const minorPx = majorPx / Math.max(1, subdivisions);
  const offsetX = inkPositiveMod(camera.x, majorPx);
  const offsetY = inkPositiveMod(camera.y, majorPx);
  const patternId = `ink-viewport-grid-${spacing}-${subdivisions}`;
  const minorLines: React.ReactNode[] = [];
  for (let index = 1; index < subdivisions; index += 1) {
    const position = index * minorPx;
    minorLines.push(
      <line key={`v-${index}`} x1={position} y1={0} x2={position} y2={majorPx} stroke={color} strokeWidth={0.5} opacity={opacity * 0.55} />,
      <line key={`h-${index}`} x1={0} y1={position} x2={majorPx} y2={position} stroke={color} strokeWidth={0.5} opacity={opacity * 0.55} />,
    );
  }
  return (
    <svg className="pointer-events-none absolute inset-0 h-full w-full" aria-hidden>
      <defs>
        <pattern id={patternId} width={majorPx} height={majorPx} patternUnits="userSpaceOnUse" x={offsetX} y={offsetY}>
          {minorLines}
          <path d={`M ${majorPx} 0 L 0 0 0 ${majorPx}`} fill="none" stroke={color} strokeWidth={1} opacity={opacity} />
        </pattern>
      </defs>
      <rect width="100%" height="100%" fill={`url(#${patternId})`} />
    </svg>
  );
}
//#endregion BlockViews

//#region Overlays
function InkTextEditorOverlay({ block, screenBounds, onCommit, onCancel }: { readonly block: InkTextItem; readonly screenBounds: InkBounds; readonly onCommit: (paragraphs: readonly InkTextParagraph[]) => void; readonly onCancel: () => void }) {
  const editorRef = useRef<HTMLDivElement | null>(null);
  const linkLabel = useLabel("ui.ink.link");
  const linkUrlPromptLabel = useLabel("ui.ink.linkUrlPrompt");
  const applyCommand = (command: string, value?: string) => {
    editorRef.current?.focus();
    document.execCommand(command, false, value);
  };
  useEffect(() => {
    const editor = editorRef.current;
    if (!editor) return;
    editor.focus();
    const selection = window.getSelection();
    const range = document.createRange();
    range.selectNodeContents(editor);
    selection?.removeAllRanges();
    selection?.addRange(range);
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        onCancel();
      }
    };
    editor.addEventListener("keydown", onKeyDown);
    return () => editor.removeEventListener("keydown", onKeyDown);
  }, [onCancel]);
  return (
    <div className="absolute z-30" style={{ left: screenBounds.x, top: screenBounds.y, width: screenBounds.width, height: screenBounds.height }}>
      <div className="bg-background/95 mb-1 flex gap-1 rounded border p-1 shadow-sm">
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("bold");
          }}
        >
          B
        </button>
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs italic"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("italic");
          }}
        >
          I
        </button>
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs underline"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("underline");
          }}
        >
          U
        </button>
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs"
          onMouseDown={(event) => {
            event.preventDefault();
            const url = window.prompt(linkUrlPromptLabel);
            if (url) applyCommand("createLink", url);
          }}
        >
          {linkLabel}
        </button>
      </div>
      <div
        ref={editorRef}
        contentEditable
        suppressContentEditableWarning
        className="text-foreground bg-background h-[calc(100%-2rem)] w-full overflow-auto rounded border p-2 outline-none"
        style={{ fontSize: block.fontSize, fontWeight: block.fontWeight, textAlign: block.align }}
        dangerouslySetInnerHTML={{ __html: inkParagraphsToHtml(block.paragraphs) }}
        onBlur={() => {
          if (!editorRef.current) return;
          onCommit(inkHtmlToParagraphs(editorRef.current));
        }}
      />
    </div>
  );
}

function InkTableCellEditorOverlay({
  block,
  row,
  col,
  screenBounds,
  onCommit,
  onCancel,
}: {
  readonly block: InkTableItem;
  readonly row: number;
  readonly col: number;
  readonly screenBounds: InkBounds;
  readonly onCommit: (content: string, advance?: boolean) => void;
  readonly onCancel: () => void;
}) {
  const inputRef = useRef<HTMLInputElement | null>(null);
  useEffect(() => {
    inputRef.current?.focus();
    inputRef.current?.select();
  }, []);
  return (
    <input
      ref={inputRef}
      className="bg-background ring-primary absolute z-30 rounded border px-2 py-1 text-sm ring-2 outline-none"
      style={{ left: screenBounds.x, top: screenBounds.y, width: screenBounds.width, height: screenBounds.height }}
      defaultValue={block.rows[row]?.[col]?.content ?? ""}
      onKeyDown={(event) => {
        if (event.key === "Escape") {
          event.preventDefault();
          onCancel();
        }
        if (event.key === "Enter" || event.key === "Tab") {
          event.preventDefault();
          onCommit(event.currentTarget.value, true);
        }
      }}
      onBlur={(event) => onCommit(event.currentTarget.value)}
    />
  );
}
//#endregion Overlays

//#region DragState
type InkDragState =
  | { readonly kind: "pan"; readonly startX: number; readonly startY: number; readonly camera: InkCamera }
  | { readonly kind: "move"; readonly origins: Readonly<Record<string, { readonly x: number; readonly y: number }>>; readonly startX: number; readonly startY: number }
  | { readonly kind: "marquee"; readonly start: SelectionMarqueePoint }
  | { readonly kind: "stroke"; readonly blockId: string }
  | { readonly kind: "eraser"; readonly mode: "eraserStroke" | "eraserPoint" }
  | { readonly kind: "resize"; readonly handle: InkResizeHandle; readonly fromBounds: InkBounds; readonly startX: number; readonly startY: number; readonly selectedIds: readonly string[] };

type InkTextEditState = { readonly blockId: string; readonly created?: boolean };
type InkTableEditState = { readonly blockId: string; readonly row: number; readonly col: number };

const INK_MARQUEE_THRESHOLD_PX = 4;
//#endregion DragState

//#region InkCanvasHost
export function InkCanvasHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.inkCanvas;
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const contextMenuTitleLabel = useLabel("ui.surfaceContextMenu.ink");
  const rootRef = useRef<HTMLDivElement | null>(null);
  const gestureActiveRef = useRef(false);
  const rafRef = useRef<number | null>(null);
  const pendingLiveEventsRef = useRef<readonly InkCanvasEvent[] | null>(null);
  const [draftDoc, setDraftDoc] = useState<InkDocument | null>(null);
  const [dragState, setDragState] = useState<InkDragState | null>(null);
  const [marqueePoints, setMarqueePoints] = useState<readonly SelectionMarqueePoint[]>([]);
  const [textEdit, setTextEdit] = useState<InkTextEditState | null>(null);
  const [tableEdit, setTableEdit] = useState<InkTableEditState | null>(null);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number; readonly items: readonly ContextMenuItem[] } | null>(null);
  const emptySceneLabel = useLabel("ui.host.emptyScene");

  const sceneDoc = useMemo(() => parseInkScene(scene?.documentJson), [scene?.documentJson]);
  const doc = draftDoc ?? sceneDoc;
  const docRef = useRef(doc);
  docRef.current = doc;
  const selectedIds = useMemo(() => parseSelectionIds(scene?.selectionJson), [scene?.selectionJson]);
  const selectedSet = useMemo(() => new Set(selectedIds), [selectedIds]);
  const hoveredId = scene?.hoveredId ?? null;
  const isNavigator = scene?.viewMode === "navigator";
  const interactive = scene?.interactive ?? false;

  useEffect(() => {
    if (!gestureActiveRef.current) setDraftDoc(null);
  }, [scene?.documentJson]);

  useEffect(() => {
    if (!windowInstanceId) return;
    return registerIntroductionSurfaceResolver(windowElementId(windowInstanceId), {
      canvasPoint: (x, y) => {
        const root = rootRef.current;
        const document = docRef.current;
        if (!root || !document) return null;
        const rect = root.getBoundingClientRect();
        const screen = worldToScreen(document.camera, x, y);
        return { x: rect.left + screen.x, y: rect.top + screen.y, visible: true };
      },
      entity: (domain, entityId): IntroductionResolvedGeometry | null => {
        const root = rootRef.current;
        const document = docRef.current;
        if (!root || !document) return null;
        const rect = root.getBoundingClientRect();
        if (domain === "block") {
          const items = flattenInkItems(document.blocks);
          const block = entityId === "*" ? items[0] : findInkItem(document, entityId);
          if (!block) return null;
          const bounds = inkItemBounds(block);
          const topLeft = worldToScreen(document.camera, bounds.x, bounds.y);
          const bottomRight = worldToScreen(document.camera, bounds.x + bounds.width, bounds.y + bounds.height);
          return {
            point: { x: rect.left + (topLeft.x + bottomRight.x) / 2, y: rect.top + (topLeft.y + bottomRight.y) / 2 },
            rect: { x: rect.left + topLeft.x, y: rect.top + topLeft.y, width: bottomRight.x - topLeft.x, height: bottomRight.y - topLeft.y },
            visible: true,
          };
        }
        if (domain === "stroke") {
          const block = entityId === "*" ? flattenInkItems(document.blocks).find((item) => item.kind === "stroke") : findInkItem(document, entityId);
          if (!block || block.kind !== "stroke") return null;
          const polyline = block.points.map(([px, py]) => {
            const screen = worldToScreen(document.camera, block.x + px, block.y + py);
            return { x: rect.left + screen.x, y: rect.top + screen.y };
          });
          if (polyline.length === 0) return null;
          return { point: polyline[Math.floor(polyline.length / 2)], polyline, visible: true };
        }
        return null;
      },
    });
  }, [windowInstanceId]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      if (!node.controllerId) return;
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();

  const flushPendingLive = useCallback(() => {
    if (rafRef.current != null) {
      cancelAnimationFrame(rafRef.current);
      rafRef.current = null;
    }
    pendingLiveEventsRef.current = null;
  }, []);

  const beginGesture = useCallback(
    (events: readonly InkCanvasEvent[], selectIds?: readonly string[]) => {
      gestureActiveRef.current = true;
      setDraftDoc((current) => applyEventsLocal(current ?? sceneDoc ?? { schema: "ink.document", id: "empty", camera: { x: 0, y: 0, zoom: 1 }, blocks: [] }, events));
      dispatch(inkCanvasActions.applyEvents, { eventsJson: JSON.stringify(events), phase: "begin", ...(selectIds ? { selectIds: [...selectIds] } : {}) });
    },
    [dispatch, sceneDoc],
  );

  const liveGesture = useCallback(
    (events: readonly InkCanvasEvent[]) => {
      setDraftDoc((current) => (current ? applyEventsLocal(current, events) : current));
      pendingLiveEventsRef.current = events;
      if (rafRef.current == null) {
        rafRef.current = requestAnimationFrame(() => {
          rafRef.current = null;
          const pending = pendingLiveEventsRef.current;
          pendingLiveEventsRef.current = null;
          if (pending) dispatch(inkCanvasActions.applyEvents, { eventsJson: JSON.stringify(pending), phase: "live" });
        });
      }
    },
    [dispatch],
  );

  const commitGesture = useCallback(
    (events: readonly InkCanvasEvent[], selectIds?: readonly string[]) => {
      flushPendingLive();
      gestureActiveRef.current = false;
      dispatch(inkCanvasActions.applyEvents, { eventsJson: JSON.stringify(events), phase: "commit", ...(selectIds ? { selectIds: [...selectIds] } : {}) });
    },
    [dispatch, flushPendingLive],
  );

  const atomicGesture = useCallback(
    (events: readonly InkCanvasEvent[], selectIds?: readonly string[]) => {
      dispatch(inkCanvasActions.applyEvents, { eventsJson: JSON.stringify(events), phase: "atomic", ...(selectIds ? { selectIds: [...selectIds] } : {}) });
    },
    [dispatch],
  );

  const selectionBounds = useMemo(() => (doc ? inkSelectionBounds(doc.blocks, selectedIds) : null), [doc, selectedIds]);
  const utility = doc?.activeUtility ?? "selectDirect";
  const showResizeHandles = !isNavigator && (utility === "selectDirect" || utility === "selectMarquee") && Boolean(selectionBounds) && selectedIds.length > 0;

  const beginMove = useCallback(
    (event: React.PointerEvent, blockId: string) => {
      if (!rootRef.current || !doc) return;
      const block = findInkItem(doc, blockId);
      if (!block || block.locked) return;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const moveIds = selectedSet.has(blockId) ? selectedIds : [blockId];
      const origins: Record<string, { x: number; y: number }> = {};
      for (const id of moveIds) {
        const entry = findInkItem(doc, id);
        if (entry) origins[id] = { x: entry.x, y: entry.y };
      }
      setDragState({ kind: "move", origins, startX: screenX, startY: screenY });
    },
    [doc, selectedIds, selectedSet],
  );

  const handlePointerDown = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator || !interactive) return;
      rootRef.current.focus();
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      if (utility === "pan" || event.button === 1 || (utility === "selectDirect" && event.altKey)) {
        setDragState({ kind: "pan", startX: screenX, startY: screenY, camera });
        return;
      }
      if (utility === "eraserStroke" || utility === "eraserPoint") {
        setDragState({ kind: "eraser", mode: utility });
        const events = utility === "eraserStroke" ? eraseInkStrokeEventsAtPoint(doc, worldX, worldY) : eraseInkStrokePointEventsNearPoint(doc, worldX, worldY, doc.eraserRadius ?? 12);
        if (events.length) beginGesture(events);
        return;
      }
      if (utility === "selectMarquee") {
        setDragState({ kind: "marquee", start: { x: screenX, y: screenY } });
        setMarqueePoints([{ x: screenX, y: screenY }]);
        return;
      }
      if (utility === "pencil") {
        const block = createInkItemByKind("stroke", worldX, worldY);
        beginGesture([{ operation: "addBlock", block }], [block.id]);
        setDragState({ kind: "stroke", blockId: block.id });
        return;
      }
      if (utility === "text" || utility === "image" || utility === "table" || utility === "math") {
        const [placeX, placeY] = inkMaybeSnapWorldPoint(doc, worldX, worldY);
        const block = createInkItemByKind(utility, placeX, placeY);
        atomicGesture([{ operation: "addBlock", block }], [block.id]);
        if (utility === "text") setTextEdit({ blockId: block.id, created: true });
        return;
      }
      const hits = inkItemsAtPoint(doc.blocks, worldX, worldY);
      const top = hits[0];
      if (!top || top.locked) {
        if (utility === "selectDirect") dispatch(inkCanvasActions.setSelection, { ids: [] });
        return;
      }
      if (utility === "selectDirect") {
        const nextSelection = event.shiftKey ? [...new Set([...selectedIds, top.id])] : [top.id];
        dispatch(inkCanvasActions.setSelection, { ids: nextSelection });
        beginMove(event, top.id);
      }
    },
    [atomicGesture, beginGesture, beginMove, dispatch, doc, interactive, isNavigator, selectedIds, utility],
  );

  const handleBlockPointerDown = useCallback(
    (event: React.PointerEvent, blockId: string) => {
      event.stopPropagation();
      if (!rootRef.current || !doc || !interactive) return;
      const block = findInkItem(doc, blockId);
      if (!block || block.locked) return;
      const nextSelection = event.shiftKey ? [...new Set([...selectedIds, blockId])] : [blockId];
      dispatch(inkCanvasActions.setSelection, { ids: nextSelection });
      if (utility === "selectDirect" || utility === "selectMarquee") beginMove(event, blockId);
    },
    [beginMove, dispatch, doc, interactive, selectedIds, utility],
  );

  const handleResizePointerDown = useCallback(
    (handle: InkResizeHandle, event: React.PointerEvent) => {
      event.stopPropagation();
      if (!rootRef.current || !selectionBounds) return;
      const rect = rootRef.current.getBoundingClientRect();
      setDragState({ kind: "resize", handle, fromBounds: selectionBounds, startX: event.clientX - rect.left, startY: event.clientY - rect.top, selectedIds: [...selectedIds] });
    },
    [selectedIds, selectionBounds],
  );

  const handlePointerMove = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc) return;
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      if (!dragState) {
        if (!interactive) return;
        const hits = inkItemsAtPoint(doc.blocks, worldX, worldY);
        const top = hits[0] ?? null;
        dispatch(inkCanvasActions.setHover, { id: top?.id ?? null });
        return;
      }
      if (dragState.kind === "pan") {
        const nextCamera = { ...dragState.camera, x: dragState.camera.x + (screenX - dragState.startX), y: dragState.camera.y + (screenY - dragState.startY) };
        setDraftDoc((current) => ({ ...(current ?? doc), camera: nextCamera }));
        dispatch(inkCanvasActions.setCamera, { camera: nextCamera });
        return;
      }
      if (dragState.kind === "move") {
        const dx = (screenX - dragState.startX) / camera.zoom;
        const dy = (screenY - dragState.startY) / camera.zoom;
        const events: InkCanvasEvent[] = [];
        for (const [blockId, origin] of Object.entries(dragState.origins)) {
          const block = findInkItem(doc, blockId);
          if (!block) continue;
          events.push({ operation: "updateBlock", blockId, block: { ...block, x: origin.x + dx, y: origin.y + dy } });
        }
        if (events.length) liveGesture(events);
        return;
      }
      if (dragState.kind === "marquee") {
        setMarqueePoints([dragState.start, { x: screenX, y: screenY }]);
        return;
      }
      if (dragState.kind === "stroke") {
        const block = findInkItem(doc, dragState.blockId);
        if (!block || block.kind !== "stroke") return;
        const localX = worldX - block.x;
        const localY = worldY - block.y;
        liveGesture([{ operation: "updateBlock", blockId: block.id, block: { ...block, points: [...block.points, [localX, localY]] } }]);
        return;
      }
      if (dragState.kind === "eraser") {
        const events = dragState.mode === "eraserStroke" ? eraseInkStrokeEventsAtPoint(doc, worldX, worldY) : eraseInkStrokePointEventsNearPoint(doc, worldX, worldY, doc.eraserRadius ?? 12);
        if (events.length) liveGesture(events);
        return;
      }
      if (dragState.kind === "resize") {
        const dx = (screenX - dragState.startX) / camera.zoom;
        const dy = (screenY - dragState.startY) / camera.zoom;
        const toBounds = inkResizeBounds(dragState.fromBounds, dragState.handle, dx, dy);
        const events: InkCanvasEvent[] = [];
        for (const blockId of dragState.selectedIds) {
          const block = findInkItem(doc, blockId);
          if (!block) continue;
          events.push({ operation: "updateBlock", blockId, block: inkScaleItemWithinGroup(block, dragState.fromBounds, toBounds) });
        }
        if (events.length) liveGesture(events);
      }
    },
    [dispatch, doc, dragState, interactive, liveGesture],
  );

  const handlePointerUp = useCallback(() => {
    if (!doc) {
      setDragState(null);
      setMarqueePoints([]);
      return;
    }
    if (dragState?.kind === "move") {
      const events: InkCanvasEvent[] = [];
      for (const blockId of Object.keys(dragState.origins)) {
        const block = findInkItem(doc, blockId);
        if (!block) continue;
        if (doc.snapEnabled) {
          const spacing = doc.snapGridSpacing ?? 8;
          const [x, y] = inkSnapWorldPoint(block.x, block.y, spacing);
          events.push({ operation: "updateBlock", blockId, block: { ...block, x, y } });
        } else {
          events.push({ operation: "updateBlock", blockId, block });
        }
      }
      commitGesture(events);
    } else if (dragState?.kind === "stroke") {
      const block = findInkItem(doc, dragState.blockId);
      if (block) commitGesture([{ operation: "updateBlock", blockId: block.id, block }]);
      else commitGesture([]);
    } else if (dragState?.kind === "resize") {
      const events: InkCanvasEvent[] = [];
      for (const blockId of dragState.selectedIds) {
        const block = findInkItem(doc, blockId);
        if (block) events.push({ operation: "updateBlock", blockId, block });
      }
      commitGesture(events);
    } else if (dragState?.kind === "eraser") {
      commitGesture([]);
    } else if (dragState?.kind === "pan") {
      flushPendingLive();
      gestureActiveRef.current = false;
    }
    if (dragState?.kind === "marquee" && marqueePoints.length >= 2 && rootRef.current) {
      const screenRect = screenRectFromPoints(marqueePoints);
      if (screenRect) {
        const camera = doc.camera;
        const worldRect = { x: (screenRect.x - camera.x) / camera.zoom, y: (screenRect.y - camera.y) / camera.zoom, width: screenRect.width / camera.zoom, height: screenRect.height / camera.zoom };
        dispatch(inkCanvasActions.setSelection, { ids: inkItemsIntersectingRect(doc.blocks, worldRect) });
      }
    }
    setDragState(null);
    setMarqueePoints([]);
  }, [commitGesture, dispatch, doc, dragState, flushPendingLive, marqueePoints]);

  const handleWheel = useCallback(
    (event: React.WheelEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator) return;
      event.preventDefault();
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const zoomFactor = event.deltaY < 0 ? 1.08 : 0.92;
      const nextZoom = Math.min(8, Math.max(0.1, camera.zoom * zoomFactor));
      const worldX = (screenX - camera.x) / camera.zoom;
      const worldY = (screenY - camera.y) / camera.zoom;
      const nextCamera = { x: screenX - worldX * nextZoom, y: screenY - worldY * nextZoom, zoom: nextZoom };
      setDraftDoc((current) => ({ ...(current ?? doc), camera: nextCamera }));
      dispatch(inkCanvasActions.setCamera, { camera: nextCamera });
    },
    [dispatch, doc, isNavigator],
  );

  const handleDoubleClick = useCallback(
    (event: React.MouseEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator || !interactive) return;
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      const hits = inkItemsAtPoint(doc.blocks, worldX, worldY);
      const top = hits[0];
      if (top?.kind === "text" && !top.locked) {
        setTableEdit(null);
        setTextEdit({ blockId: top.id });
        dispatch(inkCanvasActions.setSelection, { ids: [top.id] });
        return;
      }
      if (top?.kind === "table" && !top.locked) {
        const cell = inkTableCellAtPoint(top, worldX - top.x, worldY - top.y);
        if (!cell) return;
        setTextEdit(null);
        setTableEdit({ blockId: top.id, row: cell.row, col: cell.col });
        dispatch(inkCanvasActions.setSelection, { ids: [top.id] });
        return;
      }
      if (top) return;
      const [placeX, placeY] = inkMaybeSnapWorldPoint(doc, worldX, worldY);
      const block = createInkItemByKind("text", placeX, placeY);
      atomicGesture([{ operation: "addBlock", block }], [block.id]);
      setTextEdit({ blockId: block.id, created: true });
    },
    [atomicGesture, dispatch, doc, interactive, isNavigator],
  );

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
      if (top && !selectedSet.has(top.id)) dispatch(inkCanvasActions.setSelection, { ids: selectionIds });
      const hits = hitItems.map((item) => ({ domain: "block", id: item.id }));
      void (async () => {
        const items = await openSurfaceContextMenu(
          requestContextMenu,
          {
            menu: { id: "inkCanvas" },
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
        setContextMenu({ x: event.clientX, y: event.clientY, items });
      })();
    },
    [dispatch, doc, mapContextMenu, node.surfaceId, requestContextMenu, selectedIds, selectedSet, shellContextMenuFallback, windowInstanceId],
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
        dispatch(inkCanvasActions.setSelection, { ids: [] });
      } else {
        atomicGesture([{ operation: "updateBlock", blockId, block: { ...block, paragraphs } }]);
      }
      setTextEdit(null);
    },
    [atomicGesture, dispatch, doc],
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
      className={cn("relative h-full w-full touch-none overflow-hidden outline-none", surfaceClass)}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerLeave={handlePointerUp}
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
