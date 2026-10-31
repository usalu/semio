/** ⏱️ The deterministic shared time source of the web renderer: figures on one clock show one instant, frame for frame.
 *
 * Time advances by accumulating the elapsed time of a scheduler tick times `rate`, so a clock never drifts with frame pacing; `step` and `seek` land on exact frames `n / fps`.
 * `prefers-reduced-motion` starts the clock paused; pressing play is always honoured.
 *
 * @see ../🟦️.ts — the React hook over this class
 */

/** ⏲️ The frame source of a clock; the default drives `requestAnimationFrame` and `performance.now`. */
export type ClockScheduler = { readonly request: (tick: (nowMs: number) => void) => number; readonly cancel: (handle: number) => void };

/** 🎛️ How a clock is created. */
export type ClockOptions = { readonly fps?: number; readonly durationS?: number | null; readonly loop?: boolean; readonly rate?: number; readonly playing?: boolean; readonly scheduler?: ClockScheduler; readonly reducedMotion?: boolean };

/** 📸️ An immutable snapshot of a clock. Playback publishes a new object when the frame or the playing flag changes; `seek`, `step` and the transport controls publish at once. */
export type ClockState = { readonly timeS: number; readonly frame: number; readonly playing: boolean; readonly rate: number; readonly loop: boolean; readonly fps: number; readonly durationS: number | null };

const FRAME_EPSILON = 1e-9;

const browserScheduler: ClockScheduler = { request: (tick) => requestAnimationFrame(tick), cancel: (handle) => cancelAnimationFrame(handle) };

function prefersReducedMotion(): boolean {
  return typeof matchMedia === "function" && matchMedia("(prefers-reduced-motion: reduce)").matches;
}

/** ⏱️ A play/pause, rate, loop, seek and step time source with subscribers. */
export class StaffageClock {
  #state: ClockState;
  readonly #listeners = new Set<() => void>();
  readonly #scheduler: ClockScheduler;
  #handle: number | null = null;
  #lastMs: number | null = null;

  constructor(options: ClockOptions = {}) {
    const fps = options.fps ?? 30;
    this.#scheduler = options.scheduler ?? browserScheduler;
    const reduced = options.reducedMotion ?? prefersReducedMotion();
    this.#state = { timeS: 0, frame: 0, playing: false, rate: options.rate ?? 1, loop: options.loop ?? true, fps, durationS: options.durationS ?? null };
    if (options.playing ?? !reduced) this.play();
  }

  get state(): ClockState {
    return this.#state;
  }

  get timeS(): number {
    return this.#state.timeS;
  }

  subscribe = (listener: () => void): (() => void) => {
    this.#listeners.add(listener);
    return () => void this.#listeners.delete(listener);
  };

  getSnapshot = (): ClockState => this.#state;

  play(): void {
    if (this.#state.playing) return;
    this.#publish({ playing: true });
    this.#lastMs = null;
    this.#handle = this.#scheduler.request(this.#tick);
  }

  pause(): void {
    if (!this.#state.playing) return;
    if (this.#handle !== null) this.#scheduler.cancel(this.#handle);
    this.#handle = null;
    this.#lastMs = null;
    this.#publish({ playing: false });
  }

  toggle(): void {
    if (this.#state.playing) this.pause();
    else this.play();
  }

  setRate(rate: number): void {
    this.#publish({ rate });
  }

  setLoop(loop: boolean): void {
    this.#publish({ loop, ...this.#timeFields(this.#state.timeS, loop, this.#state.durationS) });
  }

  setDuration(durationS: number | null): void {
    this.#publish({ durationS, ...this.#timeFields(this.#state.timeS, this.#state.loop, durationS) });
  }

  seek(timeS: number): void {
    this.#publish(this.#timeFields(timeS, this.#state.loop, this.#state.durationS));
  }

  step(frames: number): void {
    const { fps, timeS } = this.#state;
    this.seek((Math.round(timeS * fps + FRAME_EPSILON) + frames) / fps);
  }

  advance(elapsedS: number): void {
    this.seek(this.#state.timeS + elapsedS * this.#state.rate);
  }

  dispose(): void {
    this.pause();
    this.#listeners.clear();
  }

  #tick = (nowMs: number): void => {
    const prevFrame = this.#state.frame;
    const prevPlaying = this.#state.playing;
    if (this.#lastMs !== null) {
      const elapsedS = (nowMs - this.#lastMs) / 1000;
      this.#state = { ...this.#state, ...this.#timeFields(this.#state.timeS + elapsedS * this.#state.rate, this.#state.loop, this.#state.durationS) };
    }
    this.#lastMs = nowMs;
    this.#handle = this.#state.playing ? this.#scheduler.request(this.#tick) : null;
    if (this.#state.frame !== prevFrame || this.#state.playing !== prevPlaying) for (const listener of [...this.#listeners]) listener();
  };

  #timeFields(timeS: number, loop: boolean, durationS: number | null): Pick<ClockState, "timeS" | "frame"> {
    const bounded = durationS === null || durationS <= 0 ? Math.max(0, timeS) : loop ? ((timeS % durationS) + durationS) % durationS : Math.min(Math.max(0, timeS), durationS);
    return { timeS: bounded, frame: Math.floor(bounded * this.#state.fps + FRAME_EPSILON) };
  }

  #publish(patch: Partial<ClockState>): void {
    this.#state = { ...this.#state, ...patch };
    for (const listener of [...this.#listeners]) listener();
  }
}
