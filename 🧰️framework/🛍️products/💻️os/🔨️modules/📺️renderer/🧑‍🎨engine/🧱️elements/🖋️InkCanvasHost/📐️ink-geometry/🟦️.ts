/** 📐️ InkCanvasHost geometry: the host id counter, the world snapping, the screen and world conversion, the item and selection bounds, the resize and scale math, the hit tests and the stroke erasure. */
import type { InkBounds, InkCamera, InkCanvasEvent, InkDocument, InkItem, InkResizeHandle, InkStrokeItem, InkTableItem, Vec2 } from "../🧾️ink-model/🟦️.ts";

//#region 🔖️InkCanvasHost
//#region GeometryHelpers
let inkHostIdCounter = 0;

/** 🆔️ Host-generated ids only need to be unique client-side (Rust re-derives its own on the next round-trip). */
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

export function inkMaybeSnapWorldPoint(doc: InkDocument, x: number, y: number): Vec2 {
  if (!doc.snapEnabled) return [x, y];
  return inkSnapWorldPoint(x, y, doc.snapGridSpacing ?? 8);
}

export function screenToWorld(camera: InkCamera, screenX: number, screenY: number): Vec2 {
  return [(screenX - camera.x) / camera.zoom, (screenY - camera.y) / camera.zoom];
}

export function worldToScreen(camera: InkCamera, worldX: number, worldY: number): { readonly x: number; readonly y: number } {
  return { x: worldX * camera.zoom + camera.x, y: worldY * camera.zoom + camera.y };
}

/** 👕️ The ink camera (`screen = world · zoom + offset`) as the shared canvas presence camera (`canvasPointToScreen`:
 * `screen = (world − centre) · zoom + size / 2`) — the world point at the viewport centre — so a peer's cursor and viewport
 * land on the same ink position whatever either camera (row 3.x, C13: the ink host published and painted no presence).
 * @see ../../../../../../../🔨️modules/📡️replication/👕️peer-overlay/🟦️.ts */
export function inkPresenceCanvasV1(camera: InkCamera, size: readonly [number, number]): InkCamera {
  return { x: (size[0] / 2 - camera.x) / camera.zoom, y: (size[1] / 2 - camera.y) / camera.zoom, zoom: camera.zoom };
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

/** 🧹️ Whole-stroke eraser: returns removeBlock events for every ink stroke under the point. */
export function eraseInkStrokeEventsAtPoint(doc: InkDocument, x: number, y: number, threshold = 8): readonly InkCanvasEvent[] {
  const hits = flattenInkItems(doc.blocks).filter((block): block is InkStrokeItem => block.kind === "stroke" && inkHitsPoint(block, x, y, threshold));
  return hits.map((block) => ({ operation: "removeBlock", blockId: block.id }));
}

/** ✂️ Splits an ink stroke into surviving point-runs after removing points within `radius` of (x, y). */
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

/** ✂️ Point-eraser events: removeBlock for the original stroke, addBlock for each surviving fragment (skipped if untouched). */
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
//#endregion GeometryHelpers
//#endregion 🔖️InkCanvasHost
