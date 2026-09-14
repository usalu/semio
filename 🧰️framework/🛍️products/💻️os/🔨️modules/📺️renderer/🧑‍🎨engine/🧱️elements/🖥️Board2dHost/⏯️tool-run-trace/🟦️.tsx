// #region 🧲️Header
/** @emoji 🎲️ The board-2d twin of `📐️Canvas2dHost/⏯️tool-run-trace`: how a board draws the placement2d subjects of its
 * `Board2dScene.toolRunTrace` lane. A subject's `shape` indexes `glyphCatalogsJson.nodeKinds` — the order a board fill
 * run captures its kinds in — and its footprint is `kindSize × scale`, a square for shape `rectangle`, else a circle.
 * Pinned by `🖱️ui/🎬️scene/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json` `traceShapes`; the wgpu twin is
 * `board2d_tool_run_trace_shapes` (`♾️infinite/🌍️world/⏯️tool-run-trace`).
 * @see ../../📐️Canvas2dHost/⏯️tool-run-trace/🟦️.tsx */
// #endregion 🧲️Header

//#region 🔷️Shapes
/** 📏️ World-unit edge of a scale-1 kind footprint. */
export const BOARD2D_TOOL_RUN_TRACE_KIND_SIZE = 96;

/** 🔷️ One kind's footprint in world units, centered on the subject position. */
export type Board2dToolRunTraceShape = { readonly kind: "circle"; readonly radius: number } | { readonly kind: "rectangle"; readonly width: number; readonly height: number };

/** 🔷️ The footprint of every `nodeKinds` row, in row order; a malformed catalog document has none. */
export function board2dToolRunTraceShapes(glyphCatalogsJson: string): readonly Board2dToolRunTraceShape[] {
  let catalogs: unknown;
  try {
    catalogs = JSON.parse(glyphCatalogsJson);
  } catch {
    return [];
  }
  const rows = typeof catalogs === "object" && catalogs !== null ? (catalogs as { readonly nodeKinds?: unknown }).nodeKinds : undefined;
  if (!Array.isArray(rows)) return [];
  return rows.map((row: { readonly shape?: unknown; readonly scale?: unknown }) => {
    const scale = typeof row?.scale === "number" && Number.isFinite(row.scale) && row.scale > 0 ? row.scale : 1;
    const size = BOARD2D_TOOL_RUN_TRACE_KIND_SIZE * scale;
    return row?.shape === "rectangle" ? { kind: "rectangle", width: size, height: size } : { kind: "circle", radius: size * 0.5 };
  });
}

/** 🖊️ `pathForShape` for `ToolRunTrace2dLayer`: one cached `Path2D` per footprint, built lazily. */
export function board2dToolRunTracePathForShape(shapes: readonly Board2dToolRunTraceShape[], createPath: () => Path2D = () => new Path2D()): (shape: number) => Path2D | null {
  const paths = new Map<number, Path2D>();
  return (index) => {
    const cached = paths.get(index);
    if (cached) return cached;
    const shape = shapes[index];
    if (!shape) return null;
    const path = createPath();
    if (shape.kind === "rectangle") path.rect(-shape.width / 2, -shape.height / 2, shape.width, shape.height);
    else path.arc(0, 0, shape.radius, 0, Math.PI * 2);
    paths.set(index, path);
    return path;
  };
}
//#endregion 🔷️Shapes
