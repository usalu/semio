/** 🌳️ InkCanvasHost document operations: the text and html conversion of paragraphs, the clipboard payload, the item cloning and creation by kind and the local application of canvas events to the item tree. */
import { hostLabel } from "../../✏️TextEditor/🖍️buffer-and-edit-helpers/🟦️.tsx";
import type { InkCanvasEvent, InkDocument, InkImageAsset, InkItem, InkItemKind, InkTextParagraph, InkTextRun } from "../🧾️ink-model/🟦️.ts";
import { createInkHostId } from "../📐️ink-geometry/🟦️.ts";

//#region 🔖️InkCanvasHost
//#region GeometryHelpers
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

/** 🖊️ Local pure application of the generic ink-apply-events operation vocabulary — mirrors the note plugin's event-apply function for optimistic in-gesture rendering. */
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

export function applyEventsLocal(doc: InkDocument, events: readonly InkCanvasEvent[]): InkDocument {
  return events.reduce((acc, event) => applyInkCanvasEventLocal(acc, event), doc);
}
//#endregion GeometryHelpers
//#endregion 🔖️InkCanvasHost
