/** 📍️ A figure placed on a ground line at a true world position: the compositional primitive of the frieze, the category previews and the playground.
 *
 * The scene's own origin (where world (0, 0) lies on the paper) is used to put the figure's ground point exactly at (`xM`, 0) of a line, so figures of different heights and
 * different frames of a gait stay on one ground at one scale, and nothing jitters when the drawn extent changes from frame to frame. One zoom maps paper millimetres to screen pixels
 * for every figure of a composition, which keeps relative scale exact whatever the drawing scale chosen for line weights.
 *
 * @see ../🧮️pipeline/🟦️.ts — `frameOf`, `inlineSvg`
 * @see ../⏱️clock/🟦️.ts — the clock figures follow
 */
import { useMemo, type CSSProperties, type ReactElement } from "react";
import type { Asset, DisplayRange, EvaluationQuery, Language, View } from "@teaching/staffage";
import { useClock, type StaffageClock } from "../⏱️clock/🟦️.ts";
import { defaultQuery, depictionOptions, evaluateState, inlineSvg, sceneOf } from "../🧮️pipeline/🟦️.ts";
import type { Selection } from "../💾️persistence/🟦️.ts";

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

/** 📍️ The props of {@link PlacedFigure}. */
export type PlacedFigureProps = {
  readonly asset: Asset;
  readonly selection: Selection | null;
  readonly clock: StaffageClock;
  readonly offsetS?: number;
  readonly view: View;
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

/** 📍️ Draws one figure with its ground point at (`xPx`, `groundPx`) of the positioned parent. */
export function PlacedFigure(props: PlacedFigureProps): ReactElement {
  const { asset, selection, clock, offsetS = 0, view, scale, language, pxPerMm, xPx, groundPx, mirrored = false, travel, className, label, style } = props;
  const time = useClock(clock);
  const timeS = time === null ? null : time.frame / time.fps;
  const query = useMemo(() => placementQuery(asset, selection, timeS, offsetS), [asset, selection, timeS, offsetS]);
  const { scene, rootM } = useMemo(() => {
    const options = depictionOptions({ view, scale: { denominator: scale }, language, transparent: true, annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: false }, ...(props.range === undefined ? {} : { range: props.range }) });
    const state = evaluateState(asset, query);
    return { scene: sceneOf(state, props.theme === undefined ? options : { ...options, theme: props.theme }), rootM: state.bones[asset.rig.root]?.origin.x ?? 0 };
  }, [asset, query, view, scale, language, props.range, props.theme]);
  const markup = useMemo(() => inlineSvg(scene), [scene]);
  const lane = travel === undefined ? null : (travelledM(asset, query) + (travel.phaseM ?? 0)) % travel.laneM;
  const shiftPx = (lane === null ? -rootM : lane) * pxPerMm * scene.metadata.mmPerMetre;
  const width = scene.widthMm * pxPerMm;
  const left = xPx + shiftPx - scene.metadata.origin.xMm * pxPerMm;
  const fade = lane === null || travel === undefined ? 1 : Math.min(1, lane / 0.4, (travel.laneM - lane) / 0.4);
  return (
    <div
      role="img"
      aria-label={label ?? asset.title[language]}
      className={className}
      data-placed={asset.id}
      style={{ position: "absolute", left, top: groundPx - scene.metadata.origin.yMm * pxPerMm, width, height: scene.heightMm * pxPerMm, opacity: fade, transform: mirrored ? "scaleX(-1)" : undefined, transformOrigin: `${scene.metadata.origin.xMm * pxPerMm}px 0`, pointerEvents: "none", ...style }}
      dangerouslySetInnerHTML={{ __html: markup }}
    />
  );
}
