/** 🚦️ Which previews of the overview grids are live: a preview follows the ambient clock only while its card is inside the viewport or within one row of it, or while its card is hovered or holds the focus, and only as long as all live previews together stay within {@link PREVIEW_LIVE_BUDGET_POINTS}. Every other preview keeps the still figure it drew once and holds no drawing lease.
 *
 * A live preview replaces its whole drawing on every frame of the ambient clock, so the memory of a page follows what its live previews weigh, not how many there are: the bound is in evaluated points, the unit of `STATE_BUDGET_POINTS`.
 * Seats are served in this order: hovered or focused, inside the viewport, within one row of it; inside one of these a preview that is live stays live, and the earlier seat goes first. The first seat is live whatever it weighs.
 *
 * @see ../🟦️.tsx — the preview of a catalog row
 * @see ../../📍️placed/🟦️.tsx — the figure of a frieze
 * @see ../../🧮️pipeline/🟦️.ts — `STATE_BUDGET_POINTS`, `statePoints`
 * @see ../../⏱️clock/🏁️frames/🟦️.ts — the frame driver of a live figure
 */
import type { Asset, DepictionOptions, EvaluationQuery } from "@teaching/staffage";
import { driveFigure } from "../../⏱️clock/🏁️frames/🟦️.ts";
import type { StaffageClock } from "../../⏱️clock/🧱️core/🟦️.ts";
import { drawFigure, type Drawing } from "../../🧮️pipeline/🏁️drawing/🟦️.ts";
import { retainDrawing, type DrawingLease } from "../../🧮️pipeline/🏁️drawing/queue.ts";

/** ⚖️ The evaluated side-elevation points all live previews may weigh together. */
export const PREVIEW_LIVE_BUDGET_POINTS = 8_000;

/** 🪑️ A preview the live set decides about: its element and how it is told to go live or to rest. */
export type PreviewTarget = { readonly element: Element; readonly live: (live: boolean) => void };

/** 🎟️ The seat of a preview: `weigh` states what its drawing weighs (a preview is not live before it was weighed), `release` gives the seat back. */
export type PreviewSeat = { readonly weigh: (points: number) => void; readonly release: () => void };

type Seat = { readonly target: PreviewTarget; readonly order: number; points: number | null; inside: boolean; near: boolean; urged: boolean; live: boolean };

const CARD = "[data-card]";
const ROW_STEP_PX = 64;

/** 🚦️ The live set of one document. */
export class PreviewLiveSet {
  readonly #seats = new Map<Element, Seat>();
  readonly #margins = new Map<number, IntersectionObserver>();
  #inside: IntersectionObserver | null = null;
  #order = 0;
  #hovered: Element | null = null;
  #focused: Element | null = null;
  #settling = false;
  #unsettled = false;
  #listening: Document | null = null;

  constructor(readonly budget: number = PREVIEW_LIVE_BUDGET_POINTS) {}

  /** 📊️ How many previews are seated, how many are live and what the live ones weigh. */
  get load(): { readonly seats: number; readonly live: number; readonly points: number } {
    let live = 0, points = 0;
    for (const seat of this.#seats.values()) if (seat.live) { live += 1; points += seat.points ?? 0; }
    return { seats: this.#seats.size, live, points };
  }

  seat(target: PreviewTarget): PreviewSeat {
    const observed = typeof IntersectionObserver === "function";
    const seat: Seat = { target, order: this.#order++, points: null, inside: !observed, near: !observed, urged: false, live: false };
    const { element } = target;
    this.#seats.set(element, seat);
    this.#listen(element.ownerDocument);
    seat.urged = this.#urges(element);
    const margin = observed ? this.#margin(element) : null;
    if (observed) (this.#inside ??= new IntersectionObserver((records) => this.#seen(records, "inside"))).observe(element);
    margin?.observe(element);
    return {
      weigh: (points) => {
        if (seat.points === points || !this.#seats.has(element)) return;
        seat.points = points;
        this.#settle();
      },
      release: () => {
        if (!this.#seats.delete(element)) return;
        this.#inside?.unobserve(element);
        margin?.unobserve(element);
        seat.live = false;
        if (this.#seats.size === 0) this.#unlisten();
        this.#settle();
      },
    };
  }

  #margin(element: Element): IntersectionObserver {
    const row = Math.ceil((element.closest(CARD) ?? element).getBoundingClientRect().height / ROW_STEP_PX) * ROW_STEP_PX;
    let observer = this.#margins.get(row);
    if (observer === undefined) {
      const reach = `${row}px 0px`;
      this.#margins.set(row, (observer = new IntersectionObserver((records) => this.#seen(records, "near"), { rootMargin: reach, scrollMargin: reach } as IntersectionObserverInit)));
    }
    return observer;
  }

  #seen(records: readonly IntersectionObserverEntry[], field: "inside" | "near"): void {
    let changed = false;
    for (const record of records) {
      const seat = this.#seats.get(record.target);
      if (seat === undefined || seat[field] === record.isIntersecting) continue;
      seat[field] = record.isIntersecting;
      changed = true;
    }
    if (changed) this.#settle();
  }

  #urges(element: Element): boolean {
    return (this.#hovered?.contains(element) ?? false) || (this.#focused?.contains(element) ?? false);
  }

  readonly #point = (event: Event): void => this.#urge("hover", event.target);
  readonly #leave = (event: Event): void => { if ((event as PointerEvent).relatedTarget === null) this.#urge("hover", null); };
  readonly #focus = (event: Event): void => this.#urge("focus", event.target);
  readonly #blur = (event: Event): void => this.#urge("focus", (event as FocusEvent).relatedTarget);

  #urge(kind: "hover" | "focus", target: EventTarget | null): void {
    const card = typeof Element === "function" && target instanceof Element ? target.closest(CARD) : null;
    if (kind === "hover") {
      if (this.#hovered === card) return;
      this.#hovered = card;
    } else {
      if (this.#focused === card) return;
      this.#focused = card;
    }
    let changed = false;
    for (const seat of this.#seats.values()) {
      const urged = this.#urges(seat.target.element);
      if (seat.urged === urged) continue;
      seat.urged = urged;
      changed = true;
    }
    if (changed) this.#settle();
  }

  #listen(document: Document): void {
    if (this.#listening !== null) return;
    this.#listening = document;
    document.addEventListener("pointerover", this.#point, { passive: true });
    document.addEventListener("pointerout", this.#leave, { passive: true });
    document.addEventListener("focusin", this.#focus);
    document.addEventListener("focusout", this.#blur);
    const active = document.activeElement;
    this.#focused = active === null ? null : active.closest(CARD);
  }

  #unlisten(): void {
    this.#inside?.disconnect();
    this.#inside = null;
    for (const observer of this.#margins.values()) observer.disconnect();
    this.#margins.clear();
    const document = this.#listening;
    if (document === null) return;
    this.#listening = null;
    this.#hovered = this.#focused = null;
    document.removeEventListener("pointerover", this.#point);
    document.removeEventListener("pointerout", this.#leave);
    document.removeEventListener("focusin", this.#focus);
    document.removeEventListener("focusout", this.#blur);
  }

  #settle(): void {
    this.#unsettled = true;
    if (this.#settling) return;
    this.#settling = true;
    try {
      while (this.#unsettled) {
        this.#unsettled = false;
        const rank = (seat: Seat): number => (seat.urged ? 3 : seat.inside ? 2 : 1);
        const wanted = [...this.#seats.values()].filter((seat) => seat.points !== null && (seat.urged || seat.inside || seat.near)).sort((a, b) => rank(b) - rank(a) || Number(b.live) - Number(a.live) || a.order - b.order);
        const chosen = new Set<Seat>();
        let points = 0;
        for (const seat of wanted) {
          if (chosen.size > 0 && points + seat.points! > this.budget) continue;
          chosen.add(seat);
          points += seat.points!;
        }
        const resting = [...this.#seats.values()].filter((seat) => seat.live && !chosen.has(seat));
        const waking = [...chosen].filter((seat) => !seat.live);
        for (const seat of resting) { seat.live = false; seat.target.live(false); }
        for (const seat of waking) { if (!this.#seats.has(seat.target.element)) continue; seat.live = true; seat.target.live(true); }
      }
    } finally {
      this.#settling = false;
    }
  }
}

const sets = new WeakMap<Document, PreviewLiveSet>();

/** 🚦️ The live set of the document of `element`. */
export function previewLiveSet(document: Document): PreviewLiveSet {
  let set = sets.get(document);
  if (set === undefined) sets.set(document, (set = new PreviewLiveSet()));
  return set;
}

/** 🎚️ How a drawn figure follows its clock: `always` on every frame (a stage, a viewer), `budgeted` only while the live set of its document seats it as live (a preview of an overview grid), `still` never (a held pose). */
export type Liveness = "always" | "budgeted" | "still";

/** 🎬️ What a figure gives to be drawn: where it draws, the instant its clock asks for, and how a finished drawing is shown. */
export type FigureRun = {
  readonly element: HTMLElement;
  readonly clock: StaffageClock;
  readonly asset: Asset;
  readonly options: DepictionOptions;
  readonly wrapLoops: boolean;
  readonly liveness: Liveness;
  readonly time: () => number;
  readonly query: (timeS: number) => EvaluationQuery;
  readonly animated: () => boolean;
  readonly paint: (drawing: Drawing, query: EvaluationQuery) => void;
  readonly fail: (error: Error) => void;
};

/** 🎬️ Draws a figure now and then as its {@link Liveness} says: a live figure holds a drawing lease and is stepped by the frame driver of its clock, a resting one keeps the last drawing it showed and gives its lease back, so the drawing worker holds the assets of live figures only. `redraw` asks again for the instant last shown (its inputs changed), `stop` ends the run. The element carries `data-preview-state` (`live` or `still`) and, once drawn, `data-preview-weight` in points unless it is `always` live. */
export function runFigure(run: FigureRun): { readonly redraw: () => void; readonly stop: () => void } {
  const { element, clock, asset, options, wrapLoops, liveness, time, query, animated, paint, fail } = run;
  let lease: DrawingLease | null = null;
  let live = liveness === "always";
  let off: (() => void) | null = null;
  let held: number | null = null;
  let asked = "";
  let drawn = false;
  let stopped = false;
  const mark = (): void => { if (liveness !== "always") element.dataset.previewState = live ? "live" : "still"; };
  const rest = (): void => { lease?.release(); lease = null; };
  const receive = (drawing: Drawing, at: EvaluationQuery): void => {
    if (stopped) return;
    drawn = true;
    paint(drawing, at);
    if (liveness === "always") return;
    element.dataset.previewWeight = String(drawing.points);
    seat?.weigh(drawing.points);
    if (!live) rest();
  };
  const step = (timeS: number): void => {
    held = timeS;
    const at = query(timeS);
    const key = JSON.stringify(at);
    if (key === asked) return;
    asked = key;
    try {
      if (clock.drawBudgetMs > 0) lease ??= retainDrawing(asset, options, wrapLoops, receive, fail);
      if (lease) lease.request(at);
      else receive(drawFigure(asset, options, at, wrapLoops), at);
    } catch (error) {
      fail(error instanceof Error ? error : new Error(String(error)));
    }
  };
  const drive = (): void => { off ??= driveFigure(clock, { element, animated, step: () => step(time()), cancelPending: () => { asked = ""; lease?.cancelPending(); } }); };
  const seat = liveness !== "budgeted" ? null : previewLiveSet(element.ownerDocument).seat({ element, live: (next) => {
    if (stopped || live === next) return;
    live = next;
    mark();
    if (next) {
      asked = "";
      drive();
      step(time());
    } else {
      off?.();
      off = null;
      if (drawn) rest();
    }
  } });
  mark();
  if (live) drive();
  step(time());
  return {
    redraw: () => { if (!stopped) step(held ?? time()); },
    stop: () => {
      stopped = true;
      off?.();
      off = null;
      seat?.release();
      rest();
    },
  };
}
