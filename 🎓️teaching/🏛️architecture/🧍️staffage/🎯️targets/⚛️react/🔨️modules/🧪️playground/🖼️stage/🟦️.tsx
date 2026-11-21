/** 🖼️ The stage of the section and floor plan playgrounds: placed figures drawn through the shared evaluator at one true scale, grabbed by pointer or keyboard.
 *
 * The side and front elevation stand their figures on a ground line (metres ticked along it); the plan is the ground seen from above, `lengthM` by `depthM` with a metre grid and the axes through the origin, and a figure is grabbed by its footprint and dragged in both directions.
 * A figure plays on the shared timeline through its own track (`track`), so the section and the plan always show the instant the 3D playground shows. A little play (hop, pat) answers a drop unless motion is reduced or the reader switched it off.
 *
 * @see ../📐️geometry/🟦️.ts — positions and drags
 * @see ../../📍️placed/🟦️.tsx — the figure
 */
import { useEffect, useRef, useState, type KeyboardEvent as ReactKeyboardEvent, type PointerEvent as ReactPointerEvent, type ReactElement } from "react";
import { formatNumber } from "@teaching/staffage";
import type { Asset, PlanarViewpoint } from "@teaching/staffage";
import type { StaffageClock } from "../../⏱️clock/🟦️.ts";
import { useArchive } from "../../🧭️context/🟦️.tsx";
import type { Placement, Playground } from "../../💾️persistence/🟦️.ts";
import { PlacedFigure } from "../../📍️placed/🟦️.tsx";
import type { Row } from "../../🔎️search/🟦️.ts";
import { linePosition } from "../🧱️compose/🟦️.ts";
import { PAD_PX, dragPosition, extentM, hitBox } from "../📐️geometry/🟦️.ts";

const HOP_MS = 520;

/** 🎬️ A reaction to answer: which placement and a counter that makes the same placement react again. */
export type Reaction = { readonly id: string; readonly token: number };

/** 📐️ The stage in pixels. */
export type StageSizes = { readonly pxPerMm: number; readonly metrePx: number; readonly groundPx: number; readonly widthPx: number; readonly heightPx: number };

/** 🧭️ The turn of a placement in the plan: a mirror across its forward axis (a lateral flip), then the yaw about its ground point, positive yaw turning the forward direction to the left, which is up the page. */
export function planTurn(yawDeg: number, mirrored: boolean): string {
  return `rotate(${-yawDeg}deg)${mirrored ? " scaleY(-1)" : ""}`;
}

function Placed(props: { readonly placement: Placement; readonly row: Row | undefined; readonly xM: number; readonly zM: number; readonly selected: boolean; readonly reaction: Reaction | null; readonly playful: boolean; readonly pg: Playground; readonly viewpoint: PlanarViewpoint; readonly sizes: StageSizes; readonly clock: StaffageClock; readonly timelineS: number; readonly onTimeline: boolean; readonly interactive: boolean; readonly onSelect: () => void; readonly onKey: (event: ReactKeyboardEvent) => void; readonly onDown: (event: ReactPointerEvent) => void; readonly onMove: (event: ReactPointerEvent) => void; readonly onUp: (event: ReactPointerEvent) => void; readonly onCancel: (event: ReactPointerEvent) => void }): ReactElement {
  const { placement, row, xM, zM, selected, reaction, playful, pg, viewpoint, sizes, interactive } = props;
  const { language, t } = useArchive();
  const hop = useRef<HTMLDivElement>(null);
  const pat = useRef<HTMLSpanElement>(null);
  const place = linePosition(viewpoint, { xM, zM }, pg);
  const xPx = PAD_PX + place.horizontalM * sizes.metrePx;
  const groundPx = sizes.groundPx - place.liftM * sizes.metrePx;
  const order = Math.round(place.nearness * 100) + 10_000;
  const asset: Asset | undefined = row?.asset;
  const title = row?.entry.title[language] ?? placement.asset;
  const box = hitBox(viewpoint, asset?.dimensions, sizes.metrePx);
  useEffect(() => {
    if (reaction === null || reaction.id !== placement.id || !playful) return;
    hop.current?.animate?.([{ transform: "scale(1.06, 0.88) translateY(0)" }, { transform: "scale(0.97, 1.05) translateY(-6px)", offset: 0.45 }, { transform: "scale(1, 1) translateY(0)" }], { duration: HOP_MS, easing: "cubic-bezier(.3,.7,.4,1)" });
    pat.current?.animate?.([{ opacity: 0.55, transform: "scaleX(0.4)" }, { opacity: 0, transform: "scaleX(1.4)" }], { duration: HOP_MS * 1.4, easing: "ease-out" });
  }, [reaction, placement.id, playful]);
  return (
    <>
      <div ref={hop} className="pointer-events-none absolute inset-0" style={{ transformOrigin: `${xPx}px ${groundPx}px`, zIndex: order }} data-near={order}>
        {asset === undefined ? null : <div className="pointer-events-none absolute inset-0" data-plan-turn={viewpoint === "plan" ? "" : undefined} data-yaw-deg={viewpoint === "plan" ? placement.yawDeg : undefined} data-flipped={viewpoint === "plan" ? placement.mirrored : undefined} style={viewpoint === "plan" ? { transformOrigin: `${xPx}px ${groundPx}px`, transform: planTurn(placement.yawDeg, placement.mirrored) } : undefined}><PlacedFigure asset={asset} selection={placement.pick} clock={props.clock} offsetS={placement.offsetS} {...(props.onTimeline ? { track: { input: placement, timelineS: props.timelineS } } : {})} view={pg.view} viewpoint={viewpoint} scale={pg.scale} language={language} pxPerMm={sizes.pxPerMm} xPx={xPx} groundPx={groundPx} mirrored={placement.mirrored} label={title} /></div>}
        <span ref={pat} aria-hidden="true" className="absolute h-1.5 w-12 -translate-x-1/2 rounded-[50%] bg-current opacity-0" style={{ left: xPx, top: groundPx - 3 }} />
      </div>
      {interactive ? (
        <button
          type="button"
          aria-pressed={selected}
          aria-label={t(viewpoint === "plan" ? "playground.placementLabelPlan" : "playground.placementLabel", { title, x: formatNumber(xM, 2, language), z: formatNumber(zM, 2, language) })}
          data-placement={placement.id}
          data-x-m={xM}
          data-z-m={zM}
          onClick={props.onSelect}
          onKeyDown={props.onKey}
          onPointerDown={props.onDown}
          onPointerMove={props.onMove}
          onPointerUp={props.onUp}
          onPointerCancel={props.onCancel}
          className="absolute cursor-grab touch-none border-0 bg-transparent p-0 text-inherit active:cursor-grabbing focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-current focus-visible:outline-solid"
          style={{ left: xPx + box.left, top: groundPx + box.top, width: box.width, height: box.height, zIndex: order + 1 }}
        />
      ) : null}
    </>
  );
}

/** 🖼️ The stage of one planar viewpoint. `onDrop` commits a drag as one edit. */
export function PlanarStage(props: {
  readonly pg: Playground;
  readonly viewpoint: PlanarViewpoint;
  readonly byId: ReadonlyMap<string, Row>;
  readonly sizes: StageSizes;
  readonly interactive: boolean;
  readonly selected: string | null;
  readonly reaction: Reaction | null;
  readonly playful: boolean;
  readonly clock: StaffageClock;
  readonly timelineS: number;
  readonly onSelect: (id: string) => void;
  readonly onKey: (placement: Placement) => (event: ReactKeyboardEvent) => void;
  readonly onDrop: (placement: Placement, position: { readonly xM?: number; readonly zM?: number }) => void;
}): ReactElement {
  const { pg, viewpoint, byId, sizes, interactive, selected, reaction, playful } = props;
  const { t, language } = useArchive();
  const [drag, setDrag] = useState<{ readonly id: string; readonly xM?: number; readonly zM?: number } | null>(null);
  const gesture = useRef<{ readonly id: string; readonly pointerId: number; readonly startX: number; readonly startY: number; readonly xM: number; readonly zM: number; moved: boolean } | null>(null);
  const { acrossM, downM } = extentM(viewpoint, pg);
  const down = (placement: Placement) => (event: ReactPointerEvent): void => {
    if (event.button !== 0 || gesture.current !== null) return;
    props.onSelect(placement.id);
    event.currentTarget.setPointerCapture?.(event.pointerId);
    gesture.current = { id: placement.id, pointerId: event.pointerId, startX: event.clientX, startY: event.clientY, xM: placement.xM, zM: placement.zM, moved: false };
  };
  const move = (event: ReactPointerEvent): void => {
    const g = gesture.current;
    if (g === null || g.pointerId !== event.pointerId) return;
    g.moved ||= Math.abs(event.clientX - g.startX) > 2 || Math.abs(event.clientY - g.startY) > 2;
    if (g.moved) setDrag({ id: g.id, ...dragPosition(viewpoint, g, { x: event.clientX - g.startX, y: event.clientY - g.startY }, sizes.metrePx, pg) });
  };
  const cancel = (event: ReactPointerEvent): void => {
    if (gesture.current === null || gesture.current.pointerId !== event.pointerId) return;
    gesture.current = null;
    setDrag(null);
    if (event.currentTarget.hasPointerCapture?.(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId);
  };
  const up = (event: ReactPointerEvent): void => {
    const g = gesture.current;
    if (g === null || g.pointerId !== event.pointerId) return;
    cancel(event);
    if (!g.moved) return;
    const result = dragPosition(viewpoint, g, { x: event.clientX - g.startX, y: event.clientY - g.startY }, sizes.metrePx, pg);
    const placement = pg.placements.find((candidate) => candidate.id === g.id);
    if (placement === undefined) return;
    const changed = (result.xM !== undefined && result.xM !== placement.xM) || (result.zM !== undefined && result.zM !== placement.zM);
    if (changed) props.onDrop(placement, { ...(result.xM === undefined ? {} : { xM: result.xM }), ...(result.zM === undefined ? {} : { zM: result.zM }) });
  };
  const planGrid = viewpoint === "plan" ? { backgroundImage: "linear-gradient(to right, currentColor 1px, transparent 1px), linear-gradient(to bottom, currentColor 1px, transparent 1px)", backgroundSize: `${sizes.metrePx}px ${sizes.metrePx}px`, backgroundPosition: `${PAD_PX}px ${PAD_PX}px` } : {};
  return (
    <div role="group" aria-label={t(viewpoint === "plan" ? "playground.stagePlanLabel" : "playground.stageLabel", { count: pg.placements.length, scale: pg.scale })} className="relative" style={{ width: sizes.widthPx, height: sizes.heightPx }} data-metre-px={sizes.metrePx} data-viewpoint={viewpoint}>
      {viewpoint === "plan" ? (
        <>
          <div aria-hidden="true" className="pointer-events-none absolute opacity-15" style={{ left: PAD_PX, top: PAD_PX, width: acrossM * sizes.metrePx, height: downM * sizes.metrePx, ...planGrid }} />
          <div aria-hidden="true" className="absolute border border-current opacity-70" style={{ left: PAD_PX, top: PAD_PX, width: acrossM * sizes.metrePx, height: downM * sizes.metrePx }} />
          <div aria-hidden="true" className="absolute border-t border-dashed border-current opacity-60" style={{ left: PAD_PX, width: acrossM * sizes.metrePx, top: sizes.groundPx }} data-axis="x" />
          <div aria-hidden="true" className="absolute w-24 text-right font-mono text-[0.65rem] opacity-80" style={{ left: PAD_PX + acrossM * sizes.metrePx - 98, top: sizes.groundPx + 2 }}>{t("playground.axisAhead")}</div>
          <div aria-hidden="true" className="absolute font-mono text-[0.65rem] opacity-80" style={{ left: PAD_PX + 2, top: PAD_PX + 2 }}>{t("playground.axisLeft")}</div>
        </>
      ) : (
        <div aria-hidden="true" className="absolute border-t border-current opacity-70" style={{ left: PAD_PX, width: acrossM * sizes.metrePx, top: sizes.groundPx }} />
      )}
      <div aria-hidden="true" className="absolute font-mono text-[0.65rem] opacity-80" style={{ left: PAD_PX, top: viewpoint === "plan" ? PAD_PX + downM * sizes.metrePx + 2 : sizes.groundPx + 2 }}>
        {Array.from({ length: Math.floor(acrossM) + 1 }, (_, metre) => (
          <span key={metre} className="absolute -translate-x-1/2 border-l border-current pl-px" style={{ left: metre * sizes.metrePx, height: metre % 5 === 0 ? 10 : 5 }}>
            {metre % 5 === 0 ? <span className="relative left-1 top-2.5 whitespace-nowrap">{formatNumber(viewpoint === "front" ? metre - acrossM / 2 : metre, 0, language)} m</span> : null}
          </span>
        ))}
      </div>
      {pg.placements.map((placement) => (
        <Placed key={placement.id} placement={placement} row={byId.get(placement.asset)} xM={drag?.id === placement.id && drag.xM !== undefined ? drag.xM : placement.xM} zM={drag?.id === placement.id && drag.zM !== undefined ? drag.zM : placement.zM} selected={selected === placement.id} reaction={reaction} playful={playful} pg={pg} viewpoint={viewpoint} sizes={sizes} clock={props.clock} timelineS={props.timelineS} onTimeline={interactive} interactive={interactive} onSelect={() => props.onSelect(placement.id)} onKey={props.onKey(placement)} onDown={down(placement)} onMove={move} onUp={up} onCancel={cancel} />
      ))}
    </div>
  );
}
