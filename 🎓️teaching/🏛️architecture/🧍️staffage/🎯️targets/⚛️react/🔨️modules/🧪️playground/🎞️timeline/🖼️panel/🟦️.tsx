/** 🖼️ The timeline panel of the playgrounds: the one transport of the shared clock, and a row per placement whose window can be moved and resized.
 *
 * A row shows the window of its placement as a bar on the timeline (the repeats of its clip as ticks inside it) and a playhead for the clock. The bar's body is a slider for the start, its right edge a slider for the length (double-click or End on it: the window runs to the end of the timeline); both are native keyboard sliders
 * (Left and Right 0.1 s, Shift 1 s, Alt 0.01 s, Home and End) and drag with the pointer; a drag is one edit, committed when the pointer is released. Each row also pauses and resumes its placement on its own.
 *
 * @see ../🟦️.ts — the rules a window obeys
 * @see ../../../⏯️transport/🟦️.tsx — the transport
 */
import { useRef, useState, type KeyboardEvent as ReactKeyboardEvent, type PointerEvent as ReactPointerEvent, type ReactElement } from "react";
import { formatNumber } from "@teaching/staffage";
import type { Asset } from "@teaching/staffage";
import { useClock, type StaffageClock } from "../../../⏱️clock/🟦️.ts";
import { useArchive } from "../../../🧭️context/🟦️.tsx";
import { useSize } from "../../../🥞️levels/🟦️.tsx";
import { Transport } from "../../../⏯️transport/🟦️.tsx";
import { Bar, Button, CheckField, Cell, Column, NumberField, cn } from "../../../🪟️chrome/🟦️.tsx";
import type { Placement, Track } from "../../../💾️persistence/🟦️.ts";
import { clipDurationS, passS, readTrack, windowEndS } from "../🟦️.ts";

const LABEL_PX = 160;
const MIN_BAR_PX = 10;
const STEP_S = 0.05;

const roundTo = (value: number, step: number): number => Math.round(value / step) * step;

/** ⌨️ The change of a slider value a key asks for, or null: Left and Right 0.1 s (Shift 1 s, Alt 0.01 s), Home and End the ends. */
export function sliderStep(event: { readonly key: string; readonly shiftKey: boolean; readonly altKey: boolean }): { readonly deltaS: number } | { readonly to: "start" | "end" } | null {
  const size = event.altKey ? 0.01 : event.shiftKey ? 1 : 0.1;
  if (event.key === "ArrowLeft" || event.key === "ArrowDown") return { deltaS: -size };
  if (event.key === "ArrowRight" || event.key === "ArrowUp") return { deltaS: size };
  if (event.key === "Home") return { to: "start" };
  if (event.key === "End") return { to: "end" };
  return null;
}

type Drag = { readonly id: string; readonly kind: "start" | "length"; readonly originX: number; readonly originS: number; readonly startS: number; readonly durationS: number | null };

/** 🎞️ The timeline panel. `onChange` is one edit of one placement (`text` is what the status line says); `onTimeline` sets the reader's timeline length or `null` for the longest window. */
export function TimelinePanel(props: {
  readonly placements: readonly Placement[];
  readonly assets: ReadonlyMap<string, Asset | undefined>;
  readonly titleOf: (asset: string) => string;
  readonly timelineS: number;
  readonly chosenS: number | null;
  readonly clock: StaffageClock;
  readonly selected: string | null;
  readonly onSelect: (id: string) => void;
  readonly onChange: (id: string, patch: Partial<Track>, text: string) => void;
  readonly onTimeline: (timelineS: number | null) => void;
}): ReactElement {
  const { placements, assets, titleOf, timelineS, chosenS, clock, selected, onSelect, onChange, onTimeline } = props;
  const { t, language } = useArchive();
  const state = useClock(clock)!;
  const [attach, size] = useSize({ width: 800, height: 0 });
  const [drag, setDrag] = useState<Drag | null>(null);
  const trackPx = Math.max(120, size.width - LABEL_PX);
  const pxPerS = trackPx / timelineS;
  const seconds = (value: number): string => `${formatNumber(value, 2, language)} s`;
  const ticks = (() => {
    const step = timelineS > 60 ? 10 : timelineS > 20 ? 5 : timelineS > 8 ? 2 : 1;
    return Array.from({ length: Math.floor(timelineS / step) + 1 }, (_, index) => index * step);
  })();
  const gesture = useRef<Drag | null>(null);
  const begin = (placement: Placement, kind: Drag["kind"], event: ReactPointerEvent, windowS: number): void => {
    onSelect(placement.id);
    event.currentTarget.setPointerCapture?.(event.pointerId);
    gesture.current = { id: placement.id, kind, originX: event.clientX, originS: kind === "start" ? placement.startS : windowS, startS: placement.startS, durationS: placement.durationS };
  };
  const move = (event: ReactPointerEvent): void => {
    const g = gesture.current;
    if (g === null) return;
    const deltaS = (event.clientX - g.originX) / pxPerS;
    if (Math.abs(event.clientX - g.originX) < 2 && drag === null) return;
    setDrag(g.kind === "start" ? { ...g, startS: Math.max(0, roundTo(g.originS + deltaS, STEP_S)) } : { ...g, durationS: Math.max(0.1, roundTo(g.originS + deltaS, STEP_S)) });
  };
  const end = (event: ReactPointerEvent): void => {
    const g = gesture.current;
    gesture.current = null;
    event.currentTarget.releasePointerCapture?.(event.pointerId);
    const result = drag;
    setDrag(null);
    if (g === null || result === null) return;
    if (g.kind === "start") onChange(g.id, { startS: result.startS }, t("playground.timeline.said.start", { title: titleOf(placements.find((placement) => placement.id === g.id)?.asset ?? g.id), start: seconds(result.startS) }));
    else if (result.durationS !== null) onChange(g.id, { durationS: result.durationS }, t("playground.timeline.said.length", { title: titleOf(placements.find((placement) => placement.id === g.id)?.asset ?? g.id), length: seconds(result.durationS) }));
  };
  const keyed = (placement: Placement, kind: Drag["kind"], windowS: number) => (event: ReactKeyboardEvent): void => {
    const step = sliderStep(event);
    if (step === null) return;
    event.preventDefault();
    event.stopPropagation();
    const title = titleOf(placement.asset);
    if (kind === "start") {
      const startS = Math.max(0, "deltaS" in step ? roundTo(placement.startS + step.deltaS, 0.01) : step.to === "start" ? 0 : Math.max(0, timelineS - windowS));
      onChange(placement.id, { startS }, t("playground.timeline.said.start", { title, start: seconds(startS) }));
    } else if ("deltaS" in step) {
      const durationS = Math.max(0.1, roundTo(windowS + step.deltaS, 0.01));
      onChange(placement.id, { durationS }, t("playground.timeline.said.length", { title, length: seconds(durationS) }));
    } else if (step.to === "end") onChange(placement.id, { durationS: null }, t("playground.timeline.said.toEnd", { title }));
    else onChange(placement.id, { durationS: 0.1 }, t("playground.timeline.said.length", { title, length: seconds(0.1) }));
  };
  return (
    <Column as="section" aria-label={t("playground.timeline.title")} className="bg-panel" data-timeline="">
      <Transport clock={clock} label={t("playground.timeline.transport")} />
      <Bar below="tablet">
        <CheckField label={t("playground.timeline.auto")} checked={chosenS === null} onChange={(auto) => onTimeline(auto ? null : timelineS)} />
        <NumberField label={t("playground.timeline.length")} unit="s" min={1} max={3600} step={0.5} value={chosenS ?? timelineS} onChange={(value) => value !== null && value >= 1 && onTimeline(value)} className="lg:max-w-48" />
      </Bar>
      <div ref={attach} className="relative overflow-hidden" data-timeline-rows="">
        <div className="flex items-end border-t border-current/20 text-[0.65rem] text-muted-foreground" aria-hidden="true" data-border-probe-art="">
          <span style={{ width: LABEL_PX }} className="shrink-0 px-single" />
          <div className="relative h-5 flex-1" style={{ width: trackPx }}>
            {ticks.map((tick) => (
              <span key={tick} className="absolute bottom-0 border-l border-current/50 pl-px font-mono" style={{ left: tick * pxPerS }}>{tick}s</span>
            ))}
          </div>
        </div>
        {placements.length === 0 ? <Cell className="py-single"><span className="text-xs text-muted-foreground">{t("playground.timeline.empty")}</span></Cell> : null}
        {placements.map((placement) => {
          const asset = assets.get(placement.asset);
          const title = titleOf(placement.asset);
          const clipS = asset === undefined ? 0 : clipDurationS(asset, placement.pick);
          const shown = drag?.id === placement.id ? { ...placement, startS: drag.startS, durationS: drag.durationS } : placement;
          const endS = windowEndS(shown, timelineS);
          const windowS = Math.max(endS - shown.startS, 0.1);
          const reading = asset === undefined ? null : readTrack(placement, clipS, state.timeS, timelineS);
          const pass = passS(clipS, placement) * pxPerS;
          const widthPx = Math.max(windowS * pxPerS, MIN_BAR_PX);
          const picked = selected === placement.id;
          return (
            <div key={placement.id} className={cn("flex items-center border-t border-current/20", picked && "bg-muted/40")} data-timeline-row={placement.id} data-phase={reading?.phase ?? "unavailable"}>
              <div className="flex shrink-0 items-center gap-single px-single" style={{ width: LABEL_PX }}>
                <Button variant="quiet" icon={placement.holdS === null ? "pause" : "play"} disabled={clipS <= 0 || reading === null} aria-label={t(placement.holdS === null ? "playground.asset.pauseOne" : "playground.asset.playOne", { title })} title={t(placement.holdS === null ? "playground.asset.pauseOne" : "playground.asset.playOne", { title })} aria-pressed={placement.holdS !== null} onClick={() => reading !== null && onChange(placement.id, { holdS: placement.holdS === null ? reading.cycleS : null }, t(placement.holdS === null ? "playground.asset.said.paused" : "playground.asset.said.playing", { title }))} />
                <button type="button" aria-pressed={picked} onClick={() => onSelect(placement.id)} className="min-w-0 flex-1 truncate border-0 bg-transparent p-0 text-left text-xs text-foreground">{title}</button>
              </div>
              <div className="relative h-8 flex-1" style={{ width: trackPx }}>
                <div
                  role="slider"
                  tabIndex={0}
                  aria-label={t("playground.timeline.startOf", { title })}
                  aria-valuemin={0}
                  aria-valuemax={Math.max(timelineS, shown.startS)}
                  aria-valuenow={Number(shown.startS.toFixed(2))}
                  aria-valuetext={seconds(shown.startS)}
                  data-bar={placement.id}
                  data-start-s={shown.startS}
                  data-end-s={endS}
                  onFocus={() => onSelect(placement.id)}
                  onKeyDown={keyed(placement, "start", windowS)}
                  onPointerDown={(event) => begin(placement, "start", event, windowS)}
                  onPointerMove={move}
                  onPointerUp={end}
                  onPointerCancel={end}
                  className={cn("absolute inset-y-0 cursor-grab touch-none border-l border-current bg-[var(--accent)]/30 active:cursor-grabbing", picked && "bg-[var(--accent)]/55")}
                  style={{ left: shown.startS * pxPerS, width: widthPx, backgroundImage: pass > 4 ? `repeating-linear-gradient(90deg, transparent 0, transparent ${pass - 1}px, currentColor ${pass - 1}px, currentColor ${pass}px)` : undefined }}
                >
                </div>
                <div
                  role="slider"
                  tabIndex={0}
                  aria-label={t("playground.timeline.lengthOf", { title })}
                  aria-valuemin={0.1}
                  aria-valuemax={Math.max(timelineS, windowS)}
                  aria-valuenow={Number(windowS.toFixed(2))}
                  aria-valuetext={shown.durationS === null ? t("playground.timeline.toEnd", { length: seconds(windowS) }) : seconds(windowS)}
                  data-bar-end={placement.id}
                  onFocus={() => onSelect(placement.id)}
                  onKeyDown={keyed(placement, "length", windowS)}
                  onDoubleClick={() => onChange(placement.id, { durationS: null }, t("playground.timeline.said.toEnd", { title }))}
                  onPointerDown={(event) => begin(placement, "length", event, windowS)}
                  onPointerMove={move}
                  onPointerUp={end}
                  onPointerCancel={end}
                  className="absolute inset-y-0 w-2 cursor-ew-resize touch-none bg-current"
                  style={{ left: shown.startS * pxPerS + widthPx - 8 }}
                />
              </div>
            </div>
          );
        })}
        <div aria-hidden="true" data-playhead="" className="pointer-events-none absolute bottom-0 top-0 w-px bg-[var(--accent)]" style={{ left: LABEL_PX + state.timeS * pxPerS }} />
      </div>
    </Column>
  );
}
