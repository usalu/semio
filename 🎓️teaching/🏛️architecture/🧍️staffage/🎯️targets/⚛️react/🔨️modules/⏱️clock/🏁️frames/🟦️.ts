/** 🏁️ One subscription per clock for all imperatively drawn figures: a clock tick steps only the figures that are visible and animated, with no React render, and a figure that turns visible catches up at once.
 *
 * @see ../🧱️core/🟦️.ts — the clock and its single `requestAnimationFrame` loop
 * @see ../../📍️placed/🟦️.tsx — the figure that draws through it
 */
import type { StaffageClock } from "../🧱️core/🟦️.ts";

/** 🎯️ A figure the frame driver steps. */
export type FrameTarget = { readonly element: Element; readonly animated: () => boolean; readonly step: (timeS: number) => void; readonly cancelPending?: () => void };

type Entry = { readonly target: FrameTarget; visible: boolean };

const timeOf = (clock: StaffageClock): number => clock.state.frame / clock.state.fps;

class Driver {
  readonly entries = new Set<Entry>();
  readonly byElement = new Map<Element, Entry>();
  #off: (() => void) | null = null;
  #observer: IntersectionObserver | null = null;
  readonly #pending = new Set<Entry>();
  #frame: number | null = null;
  #lastTime: number;

  constructor(readonly clock: StaffageClock) {
    this.#lastTime = timeOf(clock);
  }

  add(target: FrameTarget): () => void {
    const entry: Entry = { target, visible: true };
    this.entries.add(entry);
    this.byElement.set(target.element, entry);
    this.#visibility()?.observe(target.element);
    this.#off ??= this.clock.subscribe(this.#tick);
    return () => {
      this.entries.delete(entry);
      this.#pending.delete(entry);
      this.byElement.delete(target.element);
      this.#observer?.unobserve(target.element);
      if (this.entries.size === 0) {
        this.#off?.();
        this.#off = null;
        this.#cancelDrawing();
      }
    };
  }

  #visibility(): IntersectionObserver | null {
    if (this.#observer === null && typeof IntersectionObserver === "function")
      this.#observer = new IntersectionObserver((records) => {
        const latest = new Map<Element, boolean>();
        for (const record of records) latest.set(record.target, record.isIntersecting);
        for (const [element, visible] of latest) {
          const entry = this.byElement.get(element);
          if (entry === undefined || entry.visible === visible) continue;
          entry.visible = visible;
          if (!visible) {
            this.#pending.delete(entry);
            entry.target.cancelPending?.();
          }
          else if (entry.target.animated()) {
            if (this.#canDefer()) {
              this.#pending.add(entry);
              this.#scheduleDrawing();
            } else entry.target.step(timeOf(this.clock));
          }
        }
      });
    return this.#observer;
  }

  #canDefer(): boolean {
    return this.clock.state.playing && this.clock.drawBudgetMs > 0 && typeof requestAnimationFrame === "function";
  }

  #scheduleDrawing(): void {
    if (this.#frame === null && this.#pending.size > 0) this.#frame = requestAnimationFrame(this.#draw);
  }

  #cancelDrawing(): void {
    if (this.#frame !== null) cancelAnimationFrame(this.#frame);
    this.#frame = null;
    this.#pending.clear();
  }

  readonly #draw = (): void => {
    this.#frame = null;
    const deadline = performance.now() + this.clock.drawBudgetMs;
    const timeS = timeOf(this.clock);
    for (const entry of this.#pending) {
      this.#pending.delete(entry);
      if (entry.visible && entry.target.animated()) entry.target.step(timeS);
      if (performance.now() >= deadline) break;
    }
    this.#scheduleDrawing();
  };

  readonly #tick = (): void => {
    const timeS = timeOf(this.clock);
    const moved = timeS !== this.#lastTime;
    this.#lastTime = timeS;
    const deferred = this.#canDefer();
    if (!deferred) this.#cancelDrawing();
    if (!moved) {
      if (!this.clock.state.playing) for (const entry of this.entries) entry.target.cancelPending?.();
      return;
    }
    for (const entry of this.entries) {
      if (!entry.visible || !entry.target.animated()) continue;
      if (deferred) this.#pending.add(entry);
      else entry.target.step(timeS);
    }
    if (deferred) this.#scheduleDrawing();
  };
}

const drivers = new WeakMap<StaffageClock, Driver>();

/** 🔗️ Registers `target` on `clock`; returns the unregister function. */
export function driveFigure(clock: StaffageClock, target: FrameTarget): () => void {
  let driver = drivers.get(clock);
  if (driver === undefined) drivers.set(clock, (driver = new Driver(clock)));
  return driver.add(target);
}
