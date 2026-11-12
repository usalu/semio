/** 🧪️ The playground composition puts every figure at its true paper offset; the event reducer keeps placements clamped and persisted.
 * Independent checks: jsdom's XML parser reads the document and svgelements (the pinned geometry oracle of the library tests) applies its nested viewports, so the bounding boxes show where the drawn rectangles really landed.
 * @see ./🥒️.feature — the scenarios */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { VIEWS, mmPerMetre } from "@teaching/staffage";
import { composeSvg, partOffsetMm, snap, type ComposedPart } from "../../🔨️modules/🧪️playground/🧱️compose/🟦️.ts";
import { ArchiveStore, INITIAL_STATE, memoryStorage, reduce, type Placement } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { playgroundFromParams, playgroundToParams } from "../../🔨️modules/🗺️routing/🟦️.ts";
import { svgShapeBoxes } from "../../../../🧪️tests/🧰️support/🐍️.ts";

const block = (x: number, w: number): string => `<svg xmlns="http://www.w3.org/2000/svg" width="20mm" height="40mm" viewBox="0 0 20 40"><rect x="${x}" y="0" width="${w}" height="40" fill="#000000"/></svg>`;
const part = (id: string, xM: number, markup: string, mirrored = false): ComposedPart => ({ id, asset: id, title: id, xM, mirrored, markup, widthMm: 20, heightMm: 40, originXMm: 10, originYMm: 40 });
const options = (scale: number) => ({ scale, view: "light-line", viewpoint: "elevation", lengthM: 10, background: "transparent" as const, language: "en" as const, title: "Playground" });

/** 📏️ The left and right edge of the one black rectangle of a composition in millimetres (svgelements reports CSS pixels of 1/96 inch). */
function blockSpan(svg: string): { readonly first: number; readonly last: number } {
  const directory = mkdtempSync(join(tmpdir(), "staffage-compose-"));
  try {
    const file = join(directory, "composition.svg");
    writeFileSync(file, svg);
    const drawn = svgShapeBoxes(file).filter((box) => box.fill.toLowerCase() === "#000000");
    expect(drawn).toHaveLength(1);
    const pxPerMm = 96 / 25.4;
    return { first: drawn[0]!.minX / pxPerMm, last: drawn[0]!.maxX / pxPerMm };
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

describe("🧪️ composition", () => {
  it("places the nested svg at margin + x·mm-per-metre − origin at 1:100 and 1:50", () => {
    for (const [scale, xM] of [[100, 2], [50, 3]] as const) {
      const doc = new DOMParser().parseFromString(composeSvg([part("a", xM, block(0, 20))], options(scale)), "image/svg+xml");
      expect(doc.querySelector("parsererror")).toBeNull();
      const nested = doc.querySelector("#placement-a > svg")!;
      const expected = partOffsetMm(part("a", xM, ""), scale, 8 + 40);
      expect(Number(nested.getAttribute("x"))).toBeCloseTo(8 + xM * (1000 / scale) - 10, 6);
      expect(Number(nested.getAttribute("x"))).toBeCloseTo(expected.xMm, 6);
      expect(Number(nested.getAttribute("y"))).toBeCloseTo(8, 6);
      expect(mmPerMetre({ denominator: scale })).toBeCloseTo(1000 / scale, 9);
      const root = doc.documentElement;
      expect(root.getAttribute("width")).toMatch(/mm$/u);
      expect(root.getAttribute("viewBox")).toBe(`0 0 ${2 * 8 + 10 * (1000 / scale)} ${root.getAttribute("height")!.replace("mm", "")}`);
    }
  });

  it("records scale, view and assets in title and metadata", () => {
    const doc = new DOMParser().parseFromString(composeSvg([part("a", 1, block(0, 20)), part("b", 4, block(0, 20))], options(100)), "image/svg+xml");
    expect(doc.querySelector("title")!.textContent).toContain("1:100");
    const meta = doc.getElementsByTagName("staffage")[0]!;
    expect(meta.getAttribute("scale")).toBe("1:100");
    expect(meta.getAttribute("assets")).toBe("a b");
    expect(meta.getAttribute("view")).toBe("light-line");
  });

  it("places the drawn rectangle (svgelements) at the computed millimetres, mirrored ones reflected about their ground point", () => {
    const lands = (svg: string, first: number, last: number): void => {
      const span = blockSpan(svg);
      expect(span.first).toBeCloseTo(first, 3);
      expect(span.last).toBeCloseTo(last, 3);
    };
    lands(composeSvg([part("a", 2, block(0, 5))], options(100)), 18, 23);
    lands(composeSvg([part("a", 2, block(0, 5), true)], options(100)), 33, 38);
    lands(composeSvg([part("a", 3, block(0, 20))], options(50)), 58, 78);
  });

  it("snaps positions to the grid", () => {
    expect(snap(1.26)).toBeCloseTo(1.25, 9);
    expect(snap(1.234, 0.01)).toBeCloseTo(1.23, 9);
  });
});

describe("🧪️ placement events", () => {
  const placement: Placement = { id: "p1", asset: "walking-commuter", xM: 3, zM: 0, yawDeg: 0, pick: null, offsetS: 0, mirrored: false };

  it("adds, moves, clamps, duplicates and removes", () => {
    let state = reduce(INITIAL_STATE, { type: "placement-added", placement });
    expect(state.playground.placements).toHaveLength(1);
    state = reduce(state, { type: "placement-changed", id: "p1", patch: { xM: 99 } });
    expect(state.playground.placements[0]!.xM).toBe(state.playground.lengthM);
    state = reduce(state, { type: "placement-changed", id: "p1", patch: { xM: -4 } });
    expect(state.playground.placements[0]!.xM).toBe(0);
    state = reduce(state, { type: "placement-duplicated", id: "p1", newId: "p2" });
    expect(state.playground.placements.map((item) => [item.id, item.xM])).toEqual([["p1", 0], ["p2", 0.6]]);
    state = reduce(state, { type: "placement-removed", id: "p1" });
    expect(state.playground.placements.map((item) => item.id)).toEqual(["p2"]);
  });

  it("persists across a reload and survives the address round trip", () => {
    const storage = memoryStorage();
    const first = new ArchiveStore(storage, VIEWS);
    first.dispatch({ type: "placement-added", placement: { ...placement, pick: { kind: "clip", clip: "walk" }, offsetS: 0.5, mirrored: true } });
    first.dispatch({ type: "playground-configured", patch: { scale: 50, view: "thermal" } });
    const second = new ArchiveStore(storage, VIEWS);
    expect(second.getSnapshot().playground).toEqual(first.getSnapshot().playground);
    const shared = playgroundFromParams(playgroundToParams(second.getSnapshot().playground))!;
    expect(shared.scale).toBe(50);
    expect(shared.view).toBe("thermal");
    expect(shared.placements[0]).toMatchObject({ asset: "walking-commuter", xM: 3, pick: { kind: "clip", clip: "walk" }, offsetS: 0.5, mirrored: true });
  });
});
