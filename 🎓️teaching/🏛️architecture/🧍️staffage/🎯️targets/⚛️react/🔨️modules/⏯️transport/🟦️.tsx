/** ⏯️ The transport of a clock: play and pause, frame stepping, a scrubber, rate and loop, with the time as text; every control is a native, labelled element.
 *
 * Keyboard: Space toggles play, ← and → step one frame (Shift: ten), Home goes to the start. The scrubber announces its value as seconds and frame.
 *
 * @see ../⏱️clock/🟦️.ts — `StaffageClock` and `useClock`
 */
import type { KeyboardEvent, ReactElement } from "react";
import { formatNumber } from "@teaching/staffage";
import { useClock, type StaffageClock } from "../⏱️clock/🟦️.ts";
import { useArchive } from "../🧭️context/🟦️.tsx";
import { Button, CheckField, SelectField, cn } from "../🪟️chrome/🟦️.tsx";

/** 🐢️ The offered playback rates. */
export const RATES = [0.1, 0.25, 0.5, 1, 2] as const;

/** ⏯️ Play, step, scrub, rate and loop for one clock. `onUserTime` reports a time the reader set (scrub or step), so the address can follow. */
export function Transport(props: { readonly clock: StaffageClock; readonly label: string; readonly onUserTime?: (timeS: number) => void; readonly onRate?: (rate: number) => void; readonly onLoop?: (loop: boolean) => void; readonly className?: string; readonly compact?: boolean }): ReactElement {
  const { clock, label, onUserTime, onRate, onLoop, className, compact } = props;
  const { t, language } = useArchive();
  const state = useClock(clock)!;
  const duration = state.durationS ?? 0;
  const seek = (timeS: number): void => {
    clock.seek(timeS);
    onUserTime?.(clock.timeS);
  };
  const step = (frames: number): void => {
    clock.pause();
    clock.step(frames);
    onUserTime?.(clock.timeS);
  };
  const onKey = (event: KeyboardEvent): void => {
    if (event.target instanceof Element && event.target.closest("select, input[type=number], input[type=text]") !== null) return;
    if (event.key === " " && event.target instanceof Element && event.target.closest("button, input[type=checkbox]") === null) {
      event.preventDefault();
      clock.toggle();
    } else if (event.key === "ArrowLeft" && event.target instanceof Element && event.target.closest("input[type=range]") === null) step(event.shiftKey ? -10 : -1);
    else if (event.key === "ArrowRight" && event.target instanceof Element && event.target.closest("input[type=range]") === null) step(event.shiftKey ? 10 : 1);
    else if (event.key === "Home" && event.target instanceof Element && event.target.closest("input[type=range]") === null) seek(0);
  };
  const time = `${formatNumber(state.timeS, 2, language)} s`;
  const text = t("transport.time", { time, frame: state.frame });
  return (
    <div role="group" aria-label={label} onKeyDown={onKey} className={cn("flex min-w-0 flex-col gap-single border border-border bg-panel p-double", className)}>
      <div className="flex flex-wrap items-center gap-single">
        <Button variant="primary" icon={state.playing ? "pause" : "play"} onClick={() => clock.toggle()}>
          {t(state.playing ? "transport.pause" : "transport.play")}
        </Button>
        <Button icon="skip-back" aria-label={t("transport.stepBack")} title={t("transport.stepBack")} onClick={() => step(-1)} />
        <Button icon="skip-forward" aria-label={t("transport.stepForward")} title={t("transport.stepForward")} onClick={() => step(1)} />
        <Button icon="rotate-ccw" aria-label={t("transport.toStart")} title={t("transport.toStart")} onClick={() => seek(0)} />
        <output aria-live="off" className="ml-auto font-mono text-xs text-foreground">
          {text}
        </output>
        <span role="status" aria-live="polite" className="sr-only">
          {state.playing ? "" : text}
        </span>
      </div>
      <input type="range" aria-label={t("transport.scrub")} aria-valuetext={text} min={0} max={Math.max(duration, 0.001)} step={1 / state.fps} value={Math.min(state.timeS, duration)} disabled={duration <= 0} onChange={(event) => seek(Number(event.target.value))} className="min-h-6 w-full accent-[var(--accent)]" />
      {compact ? null : (
        <div className="flex flex-wrap items-end gap-double">
          <SelectField label={t("transport.rate")} value={String(state.rate)} onChange={(rate) => { clock.setRate(Number(rate)); onRate?.(Number(rate)); }} options={RATES.map((rate) => ({ value: String(rate), label: `${formatNumber(rate, rate < 1 ? 2 : 0, language)}×` }))} className="w-28" />
          <CheckField label={t("transport.loop")} checked={state.loop} onChange={(loop) => { clock.setLoop(loop); onLoop?.(loop); }} />
          <span className="font-mono text-[0.65rem] text-muted-foreground">{t("transport.fps", { fps: state.fps })}</span>
        </div>
      )}
    </div>
  );
}
