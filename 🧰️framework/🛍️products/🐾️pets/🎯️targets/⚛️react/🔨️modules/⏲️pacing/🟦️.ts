/** ⏲️ The clock of the pet layer: a fixed-step accumulator that turns animation-frame timestamps into whole simulation ticks and sleeps exactly as deep as the stage allows.
 *
 * The stage says how often it needs to be advanced (`Frame.rate`) and, when nothing moves, at which tick something
 * will (`Frame.wake`). The pacer follows: every animation frame at rate 64, every other one at rate 32 (once two ticks
 * are due), every fourth one at rate 16 (sleepers that only breathe), no animation frame at all at rate 0 — then a
 * single timer waits for the wake tick. A hidden document runs neither frames nor timers, and the time spent hidden is
 * never simulated. Time, frames and timers come in through seams; a caller that scales them makes the stage's time
 * pass faster or slower than the wall clock (the layer's `tempo`).
 *
 * @see ../🫧️layer/🟦️.tsx — the only caller in production
 * @see https://gafferongames.com/post/fix_your_timestep/ — the accumulator
 * @see https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame
 * @see https://developer.chrome.com/blog/timer-throttling-in-chrome-88 — why a hidden page must not keep timers
 */

//#region 🔌️Adapters
import { TICKS_PER_SECOND, type Frame } from "@semio-tech/pets";
//#endregion 🔌️Adapters

//#region 🔖️Constants
/** 🥁️ The length of one simulation tick in milliseconds (15.625, an exact binary fraction). */
export const TICK_MILLISECONDS = 1000 / TICKS_PER_SECOND;

/** 🧢️ The most ticks one animation frame of a running stage may carry; time beyond that is dropped, never caught up. */
export const FRAME_TICKS = 8;
//#endregion 🔖️Constants

//#region 🔖️Pacer
/** 🎼️ What the stage asks of its clock after a step: its tick, how many ticks per second its motion needs and the tick of its next change when nothing moves. */
export type Pace = Pick<Frame, "tick" | "rate" | "wake">;

/** 🪡️ The environment a pacer runs on — the time in milliseconds, animation frames and timers; the browser's own when absent. */
export interface PacerSeams {
  readonly now: () => number;
  readonly requestFrame: (callback: (time: number) => void) => number;
  readonly cancelFrame: (handle: number) => void;
  readonly setTimer: (callback: () => void, milliseconds: number) => unknown;
  readonly clearTimer: (handle: unknown) => void;
}

/** 🎛️ The handle of a running pacer: `wake` announces that something happened (a step follows with the next animation
 * frame, however deep the sleep), `hide` cancels every frame and timer for as long as the document is hidden, `show`
 * restarts the clock without catching up, `stop` ends the pacer for good. */
export interface Pacer {
  readonly wake: () => void;
  readonly hide: () => void;
  readonly show: () => void;
  readonly stop: () => void;
}

/** 🕰️ Creates the clock of a stage; it stays idle until the first {@link Pacer.wake}.
 *
 * `step(ticks)` is called inside an animation frame with the whole ticks that passed since the previous step and
 * answers with the pace of the stage. While the stage runs (rate above 0) a step carries at most {@link FRAME_TICKS}
 * ticks and happens once `64 ÷ rate` ticks are due — or at once after a `wake`. At rate 0 no frame is requested; when
 * the pace names a wake tick, one timer covers the distance, and the step that ends the sleep carries every tick slept
 * (a stage that sleeps jumps over them).
 */
export function createPacer(step: (ticks: number) => Pace, seams: Partial<PacerSeams> = {}): Pacer {
  const now = seams.now ?? ((): number => performance.now());
  const requestFrame = seams.requestFrame ?? ((callback: (time: number) => void): number => requestAnimationFrame(callback));
  const cancelFrame = seams.cancelFrame ?? ((handle: number): void => cancelAnimationFrame(handle));
  const setTimer = seams.setTimer ?? ((callback: () => void, milliseconds: number): unknown => setTimeout(callback, milliseconds));
  const clearTimer = seams.clearTimer ?? ((handle: unknown): void => clearTimeout(handle as ReturnType<typeof setTimeout>));
  let frame: number | null = null;
  let timer: { readonly handle: unknown } | null = null;
  let last = now();
  let owed = 0;
  let rate: Pace["rate"] = 0;
  let alarm = false;
  let woken = false;
  let hidden = false;
  let stopped = false;

  const disarm = (): void => {
    if (timer !== null) clearTimer(timer.handle);
    timer = null;
  };
  const request = (): void => {
    if (frame === null) frame = requestFrame(tick);
  };
  const wake = (): void => {
    if (stopped) return;
    woken = true;
    if (!hidden) request();
  };
  const ring = (): void => {
    timer = null;
    wake();
  };
  const tick = (time: number): void => {
    frame = null;
    if (hidden || stopped) return;
    owed += Math.max(0, time - last);
    last = Math.max(last, time);
    const due = Math.floor(owed / TICK_MILLISECONDS);
    if (rate > 0 && !woken && due < TICKS_PER_SECOND / rate) return request();
    owed -= due * TICK_MILLISECONDS;
    woken = false;
    disarm();
    const pace = step(rate > 0 ? Math.min(due, FRAME_TICKS) : due);
    rate = pace.rate;
    alarm = pace.wake !== null;
    if (hidden || stopped) return;
    if (rate > 0 || woken) request();
    else if (pace.wake !== null) timer = { handle: setTimer(ring, Math.max(0, (pace.wake - pace.tick) * TICK_MILLISECONDS - owed)) };
  };
  const halt = (): void => {
    if (frame !== null) cancelFrame(frame);
    frame = null;
    disarm();
  };
  return {
    wake,
    hide: (): void => {
      hidden = true;
      halt();
    },
    show: (): void => {
      if (!hidden || stopped) return;
      hidden = false;
      last = now();
      owed = 0;
      if (rate > 0 || woken || alarm) request();
    },
    stop: (): void => {
      stopped = true;
      halt();
    },
  };
}
//#endregion 🔖️Pacer

//#region 🔖️Visibility
/** 🪟️ What the visibility watch needs of a window; the layer passes `window`, a test may pass a fake. */
export type PacingHost = Pick<Window, "document" | "addEventListener" | "removeEventListener">;

/** 🙈️ Whether nobody can see the document right now. */
export function documentHidden(view: PacingHost): boolean {
  return view.document.visibilityState === "hidden";
}

/** 👁️ Calls `change` with whether the document is hidden whenever that may have changed (`visibilitychange`, `pagehide`,
 * `pageshow`) and returns the function that ends the watch. A page on its way out counts as hidden. */
export function watchVisibility(view: PacingHost, change: (hidden: boolean) => void): () => void {
  const page = view.document;
  const seen = (): void => change(documentHidden(view));
  const gone = (): void => change(true);
  const passive = { passive: true } as const;
  page.addEventListener("visibilitychange", seen, passive);
  view.addEventListener("pagehide", gone, passive);
  view.addEventListener("pageshow", seen, passive);
  return () => {
    page.removeEventListener("visibilitychange", seen);
    view.removeEventListener("pagehide", gone);
    view.removeEventListener("pageshow", seen);
  };
}
//#endregion 🔖️Visibility
