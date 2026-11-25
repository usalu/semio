import { describe, expect, it } from "vitest";
import type * as React from "react";
import { CATALOGUE_DRAG_MIME, catalogueTreeDragController, isTreeReorderDragEvent, resolveTreeDropPosition } from "../🧲️drag-drop/🟦️.tsx";
import { treeDropBand } from "../../../🔨️modules/🫳️pointer-drag/🟦️.tsx";
import vectors from "./🧫️fixtures/🧲️drag-drop/🔣️.json";

const DEFAULT_MIME = "application/vnd.code.tree.item";
const ROW_REORDER_MIME = "application/x-semio-tree-row-reorder";

describe("resolveTreeDropPosition over the shared vectors", () => {
  it("answers before, inside or after exactly as the Rust twin", () => {
    for (const vector of vectors.positions) {
      const event = { clientY: vector.y, currentTarget: { getBoundingClientRect: () => ({ top: vector.top, height: vector.height }) } } as unknown as React.DragEvent<HTMLElement>;
      expect(resolveTreeDropPosition(event), vector.name).toBe(vector.position);
    }
  });
});

describe("the active band over the shared vectors", () => {
  it("is the band the React places take for every previewed position", () => {
    for (const vector of vectors.previews) {
      if (vector.band === null) continue;
      const [x, y, w, h] = vector.frame as [number, number, number, number];
      const band = treeDropBand({ left: x, top: y, right: x + w, bottom: y + h }, vector.position as "before" | "after" | "inside");
      expect([band.left, band.top, band.right - band.left, band.bottom - band.top], vector.name).toEqual(vector.band);
    }
  });
});

describe("isTreeReorderDragEvent over the shared vectors", () => {
  it("is true only for the plain kind (default MIME, no foreign application MIME, no row-reorder MIME)", () => {
    for (const vector of vectors.kinds) {
      const types = [...(vector.defaultMime ? [DEFAULT_MIME] : []), ...(vector.rowReorder ? [ROW_REORDER_MIME] : []), ...(vector.foreign ? ["application/x-semio-catalogue-item"] : [])];
      const react = isTreeReorderDragEvent({ dataTransfer: { types } } as unknown as React.DragEvent);
      expect(react && !vector.rowReorder, JSON.stringify(vector)).toBe(vector.kind === "plain");
    }
  });
});

describe("the catalogue palette drag over the shared vectors", () => {
  it("names the same MIME and starts only for a non-blank payload", () => {
    expect(CATALOGUE_DRAG_MIME).toBe(vectors.palette.mime);
    const read = catalogueTreeDragController().pointerPaletteDrag!.readEncodedDragPayload;
    for (const vector of vectors.palette.payloads) {
      const data: Record<string, string> = vector.payload === null ? {} : { [CATALOGUE_DRAG_MIME]: vector.payload };
      expect(read(data) !== undefined, JSON.stringify(vector)).toBe(vector.starts);
    }
  });

  it("publishes the payload while the pointer drag runs and clears it on end or cancel", async () => {
    const { getActiveCatalogueDragPayload, getActiveCataloguePointerDragData } = await import("../🧲️drag-drop/🟦️.tsx");
    const controller = catalogueTreeDragController();
    controller.pointerPaletteDrag!.begin("{\"id\":\"a\"}", [CATALOGUE_DRAG_MIME]);
    expect(getActiveCatalogueDragPayload()).toBe("{\"id\":\"a\"}");
    expect(getActiveCataloguePointerDragData()).toEqual({ payload: "{\"id\":\"a\"}", types: [CATALOGUE_DRAG_MIME] });
    controller.pointerPaletteDrag!.cancel();
    expect(getActiveCatalogueDragPayload()).toBeNull();
    expect(getActiveCataloguePointerDragData()).toBeNull();
  });
});
