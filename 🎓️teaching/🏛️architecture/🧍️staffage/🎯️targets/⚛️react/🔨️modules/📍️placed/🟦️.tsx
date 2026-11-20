/** 📍️ A figure placed on a ground line at a true world position: the compositional primitive of the frieze, the category previews and the playground.
 *
 * The scene's own origin (where world (0, 0) lies on the paper) is used to put the figure's ground point exactly at (`xM`, 0) of a line, so figures of different heights and
 * different frames of a gait stay on one ground at one scale, and nothing jitters when the drawn extent changes from frame to frame. One zoom maps paper millimetres to screen pixels
 * for every figure of a composition, which keeps relative scale exact whatever the drawing scale chosen for line weights.
 *
 * @see ../🧮️pipeline/🟦️.ts — `frameOf`, `inlineSvg`
 * @see ../⏱️clock/🟦️.ts — the clock figures follow
 */
import { useLayoutEffect, useMemo, useRef, useState, type CSSProperties, type ReactElement } from "react";
import type { Asset, DisplayRange, EvaluationQuery, Language, View, PlanarViewpoint } from "@teaching/staffage";
import { type StaffageClock } from "../⏱️clock/🟦️.ts";
import { driveFigure } from "../⏱️clock/🏁️frames/🟦️.ts";
import { writeSvgMarkup } from "../🧮️pipeline/🧬️svg/🟦️.ts";
import { defaultQuery, depictionOptions } from "../🧮️pipeline/🟦️.ts";
import { drawFigure, type Drawing } from "../🧮️pipeline/🏁️drawing/🟦️.ts";
import { retainDrawing } from "../🧮️pipeline/🏁️drawing/queue.ts";
import type { Selection } from "../💾️persistence/🟦️.ts";
import { VIEWPOINT_PHRASES, resolveViewpoint, viewpointWord } from "../🔭️viewpoints/🟦️.ts";
import { clipDurationS, playable, readTrack, type TrackInput } from "../🧪️playground/🎞️timeline/🟦️.ts";

/** 🎯️ The evaluation query of a placement: a pose, a clip at the clock time plus an offset, or the asset's default clip. */
export function placementQuery(asset: Asset, selection: Selection | null, timeS: number | null, offsetS = 0): EvaluationQuery {
  if (selection?.kind === "pose") return { kind: "pose", pose: selection.pose };
  const clip = selection?.kind === "clip" ? selection.clip : asset.presentation.defaultClip;
  return timeS === null && selection === null ? defaultQuery(asset, null) : { kind: "clip", clip, timeS: (timeS ?? 0) + offsetS };
}

/** 🛣️ The extra distance a travelling clip has covered over whole cycles, so a walker keeps going instead of snapping back each loop. */
export function travelledM(asset: Asset, query: EvaluationQuery): number {
  if (query.kind !== "clip") return 0;
  const clip = asset.clips.find((candidate) => candidate.id === query.clip);
  if (clip === undefined || clip.rootMotion !== "plant" || clip.advanceM === undefined || clip.durationS <= 0) return 0;
  return Math.floor(query.timeS / clip.durationS) * clip.advanceM;
}

/** 📤️ The props of {@link PlacedFigure}. */
export type PlacedFigureProps = {
  readonly asset: Asset;
  readonly selection: Selection | null;
  readonly clock: StaffageClock;
  readonly offsetS?: number;
  readonly track?: { readonly input: TrackInput; readonly timelineS: number };
  readonly view: View;
  readonly viewpoint: PlanarViewpoint;
  readonly scale: number;
  readonly language: Language;
  readonly pxPerMm: number;
  readonly xPx: number;
  readonly groundPx: number;
  readonly mirrored?: boolean;
  readonly travel?: { readonly laneM: number; readonly phaseM?: number };
  readonly range?: DisplayRange;
  readonly theme?: "light" | "dark";
  readonly className?: string;
  readonly label?: string;
  readonly style?: CSSProperties;
};

type Inputs = Pick<PlacedFigureProps, "asset" | "selection" | "offsetS" | "track" | "pxPerMm" | "xPx" | "groundPx" | "travel"> & { readonly mirrored: boolean; readonly options: ReturnType<typeof depictionOptions> };
type Painted = { markup: string; left: number; top: number; width: number; height: number; opacity: number; origin: string };

function queryOf({ asset, selection, track, offsetS }: Inputs, timeS: number): EvaluationQuery {
  return track === undefined ? placementQuery(asset, selection, timeS, offsetS) : placementQuery(asset, playable(asset, selection), readTrack(track.input, clipDurationS(asset, selection), timeS, track.timelineS).clipTimeS);
}

function place({ asset, pxPerMm, xPx, groundPx, travel }: Inputs, query: EvaluationQuery, drawing: Drawing): Painted {
  const lane = travel === undefined ? null : (travelledM(asset, query) + (travel.phaseM ?? 0)) % travel.laneM;
  const shiftPx = (lane === null ? -drawing.rootM : lane) * pxPerMm * drawing.mmPerMetre;
  const fade = lane === null || travel === undefined ? 1 : Math.min(1, lane / 0.4, (travel.laneM - lane) / 0.4);
  return { markup: drawing.markup, left: xPx + shiftPx - drawing.origin.xMm * pxPerMm, top: groundPx - drawing.origin.yMm * pxPerMm, width: drawing.widthMm * pxPerMm, height: drawing.heightMm * pxPerMm, opacity: fade, origin: `${drawing.origin.xMm * pxPerMm}px 0` };
}

const shown = new WeakMap<HTMLElement, Painted & { mirrored: boolean }>();

function paint(element: HTMLElement, inputs: Inputs, query: EvaluationQuery, drawing: Drawing): void {
  const next = place(inputs, query, drawing);
  const last = shown.get(element);
  shown.set(element, { ...next, mirrored: inputs.mirrored });
  writeSvgMarkup(element, next.markup);
  const { style } = element;
  if (last?.left !== next.left) style.left = `${next.left}px`;
  if (last?.top !== next.top) style.top = `${next.top}px`;
  if (last?.width !== next.width) style.width = `${next.width}px`;
  if (last?.height !== next.height) style.height = `${next.height}px`;
  if (last?.opacity !== next.opacity) style.opacity = `${next.opacity}`;
  if (last?.origin !== next.origin) style.transformOrigin = next.origin;
  if (last?.mirrored !== inputs.mirrored) style.transform = inputs.mirrored ? "scaleX(-1)" : "";
}

/** 📦️ Draws one figure with its ground point at (`xPx`, `groundPx`) of the positioned parent. React renders the element once per change of its props; the clock moves it through the frame driver without a render: markup, position and fade are written straight to the element, and only while it is visible. */
export function PlacedFigure(props: PlacedFigureProps): ReactElement | null {
  const { asset, selection, clock, offsetS, track, view, viewpoint, scale, language, pxPerMm, xPx, groundPx, mirrored: mirroredRequest = false, travel, className, label, style } = props;
  const mirrored = mirroredRequest && viewpoint === "elevation";
  const ref = useRef<HTMLDivElement | null>(null);
  const resolution = useMemo(() => resolveViewpoint(asset, viewpoint, scale, language), [asset, viewpoint, scale, language]);
  const options = useMemo(() => {
    const base = depictionOptions({ view, viewpoint, scale: { denominator: scale }, language, transparent: true, annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: false }, ...(props.range === undefined ? {} : { range: props.range }) });
    return props.theme === undefined ? base : { ...base, theme: props.theme };
  }, [view, viewpoint, scale, language, props.range, props.theme]);
  const inputs: Inputs = { asset, selection, offsetS, track, pxPerMm, xPx, groundPx, travel, mirrored, options };
  const latest = useRef(inputs);
  const held = useRef<number | null>(null);
  const update = useRef<{ readonly asset: Asset; readonly options: Inputs["options"]; readonly clock: StaffageClock; readonly redraw: () => void } | null>(null);
  const [failure, setFailure] = useState<Error | null>(null);
  const available = resolution.available;
  useLayoutEffect(() => {
    latest.current = inputs;
    const current = update.current;
    if (current?.asset === asset && current.options === options && current.clock === clock) current.redraw();
  });
  useLayoutEffect(() => {
    const element = ref.current;
    if (element === null) return;
    let last: { readonly query: EvaluationQuery; readonly drawing: Drawing } | null = null;
    const receive = (drawing: Drawing, query: EvaluationQuery) => {
      last = { drawing, query };
      paint(element, latest.current, query, drawing);
    };
    const lease = clock.drawBudgetMs > 0 ? retainDrawing(asset, options, true, receive, setFailure) : null;
    const step = (timeS: number) => {
      held.current = timeS;
      const query = queryOf(latest.current, timeS);
      if (lease) lease.request(query);
      else receive(drawFigure(asset, options, query, true), query);
    };
    const redraw = () => {
      if (last) paint(element, latest.current, last.query, last.drawing);
      step(held.current ?? clock.state.frame / clock.state.fps);
    };
    update.current = { asset, options, clock, redraw };
    step(clock.state.frame / clock.state.fps);
    const stop = driveFigure(clock, { element, animated: () => latest.current.selection?.kind !== "pose", step, ...(lease ? { cancelPending: lease.cancelPending } : {}) });
    return () => { stop(); lease?.release(); update.current = null; };
  }, [asset, options, clock, available]);
  if (failure !== null) throw failure;
  if (!resolution.available)
    return (
      <div role="img" aria-label={`${label ?? asset.title[language]}: ${viewpointWord("viewpoints.unavailable", language, { view: VIEWPOINT_PHRASES[viewpoint][language] })}. ${resolution.message[language]}`} title={resolution.message[language]} data-placed={asset.id} data-viewpoint={viewpoint} data-placed-unavailable={resolution.reason} className={className} style={{ position: "absolute", left: xPx - 6, top: groundPx - 24, width: 12, height: 24, border: "1px dashed currentColor", fontSize: 10, textAlign: "center", pointerEvents: "none", ...style }}>⊘</div>
    );
  return <div ref={ref} role="img" aria-label={label ?? asset.title[language]} className={className} data-placed={asset.id} data-viewpoint={viewpoint} style={{ position: "absolute", pointerEvents: "none", ...style }} />;
}
