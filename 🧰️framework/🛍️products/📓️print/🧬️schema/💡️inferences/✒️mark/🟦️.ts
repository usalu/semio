/** ✒️ Marks: the TypeScript twin of `semio-viz-mark`. A recording path context and the curve contract that the nineteen true curve interpolators of `\SemioVizPath` and the area-sized symbol glyphs of `\SemioVizSymbol` implement — every one of them numerically identical to its d3-shape counterpart.
 * @see ../../../🖋️latex/semio-viz-mark.sty
 * @see ./🪜️curves-linear-and-step/🟦️.ts — linear and step curves
 * @see ./🌀️curves-basis-and-bundle/🟦️.ts — basis and bundle curves
 * @see ./🎢️curves-cardinal-and-catmull-rom/🟦️.ts — cardinal and Catmull–Rom curves
 * @see ./🏔️curves-monotone-natural-and-bump/🟦️.ts — monotone, natural and bump curves
 * @see ./💠️curve-registry-and-symbols/🟦️.ts — the curve registry and the symbol glyphs
 */

//#region 🔖️PathContext
/** 🖊️ One recorded path command; the argument order is the canvas order d3 emits. */
export type VizPathCommand =
  | { readonly op: "moveTo"; readonly args: readonly [number, number] }
  | { readonly op: "lineTo"; readonly args: readonly [number, number] }
  | { readonly op: "quadraticCurveTo"; readonly args: readonly [number, number, number, number] }
  | { readonly op: "bezierCurveTo"; readonly args: readonly [number, number, number, number, number, number] }
  | { readonly op: "arc"; readonly args: readonly [number, number, number, number, number, number] }
  | { readonly op: "arcTo"; readonly args: readonly [number, number, number, number, number] }
  | { readonly op: "rect"; readonly args: readonly [number, number, number, number] }
  | { readonly op: "closePath"; readonly args: readonly [] };

/** 🖊️ The canvas-shaped sink every curve and shape generator writes into. */
export type VizPathContext = {
  moveTo(x: number, y: number): void;
  lineTo(x: number, y: number): void;
  quadraticCurveTo(cx: number, cy: number, x: number, y: number): void;
  bezierCurveTo(cx1: number, cy1: number, cx2: number, cy2: number, x: number, y: number): void;
  arc(x: number, y: number, r: number, a0: number, a1: number, counterclockwise?: boolean): void;
  arcTo(x1: number, y1: number, x2: number, y2: number, r: number): void;
  rect(x: number, y: number, w: number, h: number): void;
  closePath(): void;
};

/** 🖊️ A path context that records every command, so geometry is comparable without rasterising. */
export type VizPathRecorder = VizPathContext & { readonly commands: VizPathCommand[]; toString(): string };

function formatNumber(value: number): string {
  return Number.isInteger(value) ? String(value) : String(Math.round(value * 1e6) / 1e6);
}

/** 🖊️ Creates a recording path context. */
export function vizPathRecorder(): VizPathRecorder {
  const commands: VizPathCommand[] = [];
  const recorder: VizPathRecorder = {
    commands,
    moveTo: (x, y) => void commands.push({ op: "moveTo", args: [x, y] }),
    lineTo: (x, y) => void commands.push({ op: "lineTo", args: [x, y] }),
    quadraticCurveTo: (cx, cy, x, y) => void commands.push({ op: "quadraticCurveTo", args: [cx, cy, x, y] }),
    bezierCurveTo: (cx1, cy1, cx2, cy2, x, y) => void commands.push({ op: "bezierCurveTo", args: [cx1, cy1, cx2, cy2, x, y] }),
    arc: (x, y, r, a0, a1, counterclockwise = false) => void commands.push({ op: "arc", args: [x, y, r, a0, a1, counterclockwise ? 1 : 0] }),
    arcTo: (x1, y1, x2, y2, r) => void commands.push({ op: "arcTo", args: [x1, y1, x2, y2, r] }),
    rect: (x, y, w, h) => void commands.push({ op: "rect", args: [x, y, w, h] }),
    closePath: () => void commands.push({ op: "closePath", args: [] }),
    toString: () => commands.map((command) => `${command.op}(${command.args.map(formatNumber).join(",")})`).join(" "),
  };
  return recorder;
}

//#endregion 🔖️PathContext
//#region 🔖️CurveContract
/** ✒️ A curve interpolator: the state machine `line` and `area` drive point by point. */
export type VizCurve = {
  areaStart(): void;
  areaEnd(): void;
  lineStart(): void;
  lineEnd(): void;
  point(x: number, y: number): void;
};

/** ✒️ A curve factory bound to a path context, exactly d3's `curve(context)` shape. */
export type VizCurveFactory = (context: VizPathContext) => VizCurve;

/** ✒️ The curve callback that does nothing, for the area start and end of curves without areas. */
export function vizCurveNoop(): void {}
//#endregion 🔖️CurveContract
