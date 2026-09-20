/**
 * 📋 React host oracle for the language-neutral Ink clipboard contract.
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createElement } from "react";
import Ajv2020 from "ajv/dist/2020.js";
import { afterEach, describe, expect, it } from "vitest";
import { act, cleanup, fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import {
  InkCanvasHost,
  cloneInkItemsWithOffset,
  inkClipboardPayload,
  inkItemsFromClipboardPayload,
  type InkItem,
} from "../../🧱️elements/🖋️InkCanvasHost/🟦️.tsx";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/🖋️ink-clipboard/🔣️.json"), "utf8")) as any;
const schema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🖋️ink-clipboard/🔣️.json"), "utf8")) as any;

type ClipboardDataStub = {
  readonly items: readonly unknown[];
  readonly getData: (type: string) => string;
  readonly setData: (type: string, value: string) => void;
};

function mountedHost(selectedIds: readonly string[] = [], activeUtility?: string) {
  const actions: any[] = [];
  const node = {
    type: "componentScene",
    surfaceId: "ink.clipboard.fixture",
    controllerId: "ink-clipboard-controller",
    componentKind: "ink-canvas",
    inkCanvas: {
      selectionJson: JSON.stringify(selectedIds),
      activeUtility: "selectDirect",
      viewMode: "edit",
      interactive: true,
      documentJson: JSON.stringify(activeUtility == null ? fixture.document : { ...fixture.document, activeUtility }),
    },
  };
  const rendered = render(createElement(InkCanvasHost, { node, onAction: (action: unknown) => actions.push(action) } as any));
  const root = rendered.container.querySelector('[data-surface-id="ink.clipboard.fixture"]') as HTMLDivElement;
  Object.defineProperty(root, "getBoundingClientRect", {
    value: () => ({ x: 0, y: 0, left: 0, top: 0, right: fixture.viewport.width, bottom: fixture.viewport.height, width: fixture.viewport.width, height: fixture.viewport.height, toJSON: () => ({}) }),
  });
  return { actions, rendered, root };
}

function dispatchClipboard(target: Element, type: "copy" | "paste", clipboardData: ClipboardDataStub): Event {
  const event = new Event(type, { bubbles: true, cancelable: true });
  Object.defineProperty(event, "clipboardData", { value: clipboardData });
  act(() => void target.dispatchEvent(event));
  return event;
}

function textClipboard(text: string, writes: Array<readonly [string, string]> = []): ClipboardDataStub {
  return {
    items: [],
    getData: (type) => (type === "text/plain" ? text : ""),
    setData: (type, value) => writes.push([type, value]),
  };
}

function inkActions(actions: readonly any[]): any[] {
  return actions.filter((action) => action.action === "inkApplyEvents");
}

function actionEvents(action: any): any[] {
  return JSON.parse(action.args.eventsJson);
}

function itemIds(items: readonly InkItem[]): string[] {
  const ids: string[] = [];
  const visit = (item: InkItem) => {
    ids.push(item.id);
    if (item.kind === "group") item.children.forEach(visit);
  };
  items.forEach(visit);
  return ids;
}

function itemNames(items: readonly InkItem[]): string[] {
  const names: string[] = [];
  const visit = (item: InkItem) => {
    names.push(item.name);
    if (item.kind === "group") item.children.forEach(visit);
  };
  items.forEach(visit);
  return names;
}

describe("InkCanvas clipboard", () => {
  afterEach(() => cleanup());

  it("validates the shared bounded clipboard grammar", () => {
    const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("copies selected blocks in selection order and omits document assets", () => {
    const { root } = mountedHost(fixture.copy.selectedIds);
    const writes: Array<readonly [string, string]> = [];
    const event = dispatchClipboard(root, "copy", textClipboard("", writes));
    expect(event.defaultPrevented).toBe(true);
    expect(writes).toHaveLength(1);
    expect(writes[0]?.[0]).toBe("text/plain");
    const copied = JSON.parse(writes[0]![1]);
    expect(copied.schema).toBe(fixture.payloadSchema);
    expect(copied.blocks.map((block: any) => block.id)).toEqual(fixture.copy.expectedBlockIds);
    expect(copied).not.toHaveProperty("assets");
    expect(copied.blocks).toEqual(fixture.copy.expectedBlockIds.map((id: string) => fixture.document.blocks.find((block: any) => block.id === id)));

    const empty = mountedHost([]);
    const emptyWrites: Array<readonly [string, string]> = [];
    const emptyEvent = dispatchClipboard(empty.root, "copy", textClipboard("", emptyWrites));
    expect(emptyEvent.defaultPrevented).toBe(false);
    expect(emptyWrites).toEqual([]);
  });

  it("round-trips the payload and recursively re-identifies pasted blocks at the snapped center", () => {
    const source = fixture.paste.blocks.payload.blocks as InkItem[];
    const payload = inkClipboardPayload(source);
    expect(inkItemsFromClipboardPayload(payload)).toEqual(source);
    for (const invalid of fixture.paste.invalidPayloads) expect(inkItemsFromClipboardPayload(invalid)).toBeNull();

    const clones = cloneInkItemsWithOffset(source, fixture.paste.target.x, fixture.paste.target.y);
    expect(clones.map(({ x, y }) => ({ x, y }))).toEqual(fixture.paste.blocks.expectedTopLevelPoints);
    expect(itemNames(clones)).toEqual(itemNames(source));
    expect(new Set(itemIds(clones)).size).toBe(itemIds(clones).length);
    for (const id of itemIds(source)) expect(itemIds(clones)).not.toContain(id);

    const { actions, root } = mountedHost();
    const event = dispatchClipboard(root, "paste", textClipboard(payload));
    expect(event.defaultPrevented).toBe(true);
    const action = inkActions(actions)[0];
    const events = actionEvents(action);
    expect(action.args.phase).toBe("atomic");
    expect(events.map((entry: any) => entry.operation)).toEqual(["addBlock", "addBlock"]);
    const pasted = events.map((entry: any) => entry.block) as InkItem[];
    expect(pasted.map(({ x, y }) => ({ x, y }))).toEqual(fixture.paste.blocks.expectedTopLevelPoints);
    expect(itemNames(pasted)).toEqual(itemNames(source));
    expect(action.args.selectIds).toEqual(pasted.map((block) => block.id));
    for (const id of itemIds(source)) expect(itemIds(pasted)).not.toContain(id);
  });

  it("classifies SVG before plain text and emits the matching atomic events", () => {
    const plain = mountedHost();
    dispatchClipboard(plain.root, "paste", textClipboard(fixture.paste.plainText.clipboard));
    const plainAction = inkActions(plain.actions)[0];
    const plainBlock = actionEvents(plainAction)[0].block;
    expect(plainAction.args.phase).toBe("atomic");
    expect(plainBlock).toEqual(expect.objectContaining({ kind: "text", ...fixture.paste.plainText.point, ...fixture.paste.plainText.size }));
    expect(plainBlock.paragraphs.map((paragraph: any) => paragraph.runs[0].text)).toEqual(fixture.paste.plainText.paragraphs);
    expect(plainAction.args.selectIds).toEqual([plainBlock.id]);
    plain.rendered.unmount();

    const svg = mountedHost();
    dispatchClipboard(svg.root, "paste", textClipboard(fixture.paste.svg.clipboard));
    const svgAction = inkActions(svg.actions)[0];
    const svgEvents = actionEvents(svgAction);
    expect(svgEvents.map((entry: any) => entry.operation)).toEqual(["putAsset", "addBlock"]);
    expect(svgEvents[0].asset).toEqual({ mime: fixture.paste.svg.mime, data: fixture.paste.svg.trimmed });
    expect(svgEvents[1].block).toEqual(expect.objectContaining({ kind: "image", ...fixture.paste.svg.point, ...fixture.paste.svg.size, imageKey: svgEvents[0].key }));
    expect(svgAction.args.selectIds).toEqual([svgEvents[1].block.id]);
  });

  it("gives an image clipboard item priority and publishes its data URL with one image block", async () => {
    const { actions, root } = mountedHost();
    const bytes = Uint8Array.from(Buffer.from(fixture.paste.raster.base64, "base64"));
    const file = new File([bytes], "pixel.png", { type: fixture.paste.raster.mime });
    const data: ClipboardDataStub = {
      items: [{ kind: "file", type: fixture.paste.raster.mime, getAsFile: () => file }],
      getData: () => JSON.stringify(fixture.paste.blocks.payload),
      setData: () => undefined,
    };
    const event = dispatchClipboard(root, "paste", data);
    expect(event.defaultPrevented).toBe(true);
    await waitFor(() => expect(inkActions(actions)).toHaveLength(1));
    const action = inkActions(actions)[0];
    const events = actionEvents(action);
    expect(events.map((entry: any) => entry.operation)).toEqual(["putAsset", "addBlock"]);
    expect(events[0].asset).toEqual({ mime: fixture.paste.raster.mime, data: fixture.paste.raster.dataUrl });
    expect(events[1].block).toEqual(expect.objectContaining({ kind: "image", ...fixture.paste.raster.point, ...fixture.paste.raster.size, imageKey: events[0].key }));
    expect(action.args.selectIds).toEqual([events[1].block.id]);
  });

  it("leaves copy and paste to the active contenteditable editor", () => {
    const { actions, root } = mountedHost(["text-a"]);
    fireEvent.doubleClick(root, { clientX: 184, clientY: 32 });
    const editor = root.querySelector('[contenteditable="true"]') as HTMLDivElement;
    expect(editor).not.toBeNull();
    const writes: Array<readonly [string, string]> = [];
    const copy = dispatchClipboard(editor, "copy", textClipboard("", writes));
    const paste = dispatchClipboard(editor, "paste", textClipboard(fixture.paste.plainText.clipboard));
    expect(copy.defaultPrevented).toBe(false);
    expect(paste.defaultPrevented).toBe(false);
    expect(writes).toEqual([]);
    expect(inkActions(actions)).toEqual([]);
  });

  it("retires a cancelled gesture without forging an up and accepts the next down", () => {
    const { actions, root } = mountedHost([], "pencil");
    fireEvent.pointerDown(root, { button: 0, clientX: 32, clientY: 40, pointerId: 7 });
    fireEvent.pointerMove(root, { buttons: 1, clientX: 48, clientY: 52, pointerId: 7 });
    fireEvent.pointerCancel(root, { pointerId: 7 });
    expect(inkActions(actions).map((action) => action.args.phase)).not.toContain("commit");

    fireEvent.pointerDown(root, { button: 0, clientX: 72, clientY: 80, pointerId: 8 });
    fireEvent.pointerUp(root, { button: 0, clientX: 72, clientY: 80, pointerId: 8 });
    expect(inkActions(actions).map((action) => action.args.phase)).toContain("commit");
  });
});
