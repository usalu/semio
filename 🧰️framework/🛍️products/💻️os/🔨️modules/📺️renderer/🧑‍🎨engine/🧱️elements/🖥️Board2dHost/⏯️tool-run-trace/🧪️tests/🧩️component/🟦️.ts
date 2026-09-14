/** @emoji 🧪️ Laws of the board-2d trace footprints against the language-neutral board-2d lane contract (the Rust
 * `board2d_tool_run_trace_shapes` pins the same rows), and of the layer paint over the contract's placement lane: every
 * placement record fills its kind's footprint path once. */
import { describe, expect, it } from "vitest";
import contract from "../../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json";
import { ToolRunTraceRecordStore } from "../../../../🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx";
import { paintToolRunTrace2d, type ToolRunTrace2dContext } from "../../../../📐️Canvas2dHost/⏯️tool-run-trace/🟦️.tsx";
import { BOARD2D_TOOL_RUN_TRACE_KIND_SIZE, board2dToolRunTracePathForShape, board2dToolRunTraceShapes } from "../../🟦️.tsx";

type PathCall = { readonly op: "rect" | "arc"; readonly args: readonly number[] };

function recordingPath(calls: PathCall[]): Path2D {
  return { rect: (...args: number[]) => calls.push({ op: "rect", args }), arc: (...args: number[]) => calls.push({ op: "arc", args }) } as unknown as Path2D;
}

describe("board-2d tool run trace footprints", () => {
  const shapes = contract.traceShapes;

  it("reads the contract's footprints from the kind catalogs and none from a malformed catalog", () => {
    expect(BOARD2D_TOOL_RUN_TRACE_KIND_SIZE).toBe(shapes.kindSize);
    expect(board2dToolRunTraceShapes(shapes.glyphCatalogsJson)).toEqual(shapes.shapes);
    for (const malformed of shapes.malformed) expect(board2dToolRunTraceShapes(malformed)).toEqual([]);
  });

  it("builds one centered footprint path per kind and fills it once per placement record of the contract lane", () => {
    const calls: PathCall[] = [];
    const pathForShape = board2dToolRunTracePathForShape(board2dToolRunTraceShapes(shapes.glyphCatalogsJson), () => recordingPath(calls));
    expect(pathForShape(0)).toBe(pathForShape(0));
    expect(pathForShape(shapes.shapes.length)).toBeNull();
    expect(calls).toEqual([{ op: "arc", args: [0, 0, 48, 0, Math.PI * 2] }]);
    pathForShape(1);
    expect(calls[1]).toEqual({ op: "rect", args: [-24, -24, 48, 48] });
    const store = new ToolRunTraceRecordStore();
    store.applyLane(shapes.placementLane);
    expect({ run: Number(store.cursor?.run), generation: store.cursor?.generation, page: store.cursor?.page }).toEqual({ run: shapes.placementLaneRecords.run, generation: shapes.placementLaneRecords.generation, page: shapes.placementLaneRecords.page });
    let fills = 0;
    let strokes = 0;
    const ctx = { save() {}, restore() {}, setTransform() {}, fill: () => (fills += 1), stroke: () => (strokes += 1), fillStyle: "", strokeStyle: "", globalAlpha: 1, lineWidth: 1 } as unknown as ToolRunTrace2dContext;
    const palette = { fill: { testing: "#111111", success: "#222222", warning: "#333333", danger: "#444444" }, highlight: "#555555" };
    paintToolRunTrace2d(ctx, store, { x: 0, y: 0, zoom: 1 }, { width: 100, height: 100, pixelRatio: 1 }, pathForShape, palette);
    expect(fills).toBe(shapes.placementLaneRecords.placements);
    expect(strokes).toBe(1);
  });
});
