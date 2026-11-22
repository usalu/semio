/** 🖥️ `FlowStage`: the air flow of one figure as plain DOM laid over it — the overlay canvas, a progress bar with cancel while the flow develops, and the words that say what is shown, what was cancelled or what failed, with a retry. The drawing figure and the 3D stage both mount this one class, so the flow behaves and reads the same in every view.
 *
 * The stage follows its options: asset, pose or clip and environment decide the solve ({@link FlowSession}); layers, theme, range and scale decide the marks; the shared {@link StaffageClock} is the only time (a paused clock shows a still field, a missing clock the developed field of the first instant). The owner supplies the projection and calls {@link FlowStage.redraw} when it changed (camera, zoom, size).
 * Accessibility: the canvas is decoration (`aria-hidden`); a polite status states the field in words; the progress bar and the buttons are native elements with names. Reduced motion is the clock's concern: a paused clock moves nothing here.
 *
 * @see ../📡️session/🟦️.ts — the solver client
 * @see ../🖌️overlay/🟦️.ts — the renderer
 */
import { formatNumber } from "@teaching/staffage";
import type { Asset, DisplayRange, Environment, EvaluationQuery, FlowField, FlowLayer, Language } from "@teaching/staffage";
import type { StaffageClock } from "../../⏱️clock/🧱️core/🟦️.ts";
import { translator } from "../../🌐️i18n/🟦️.ts";
import { FlowOverlay, type FlowProjection } from "../🖌️overlay/🟦️.ts";
import { FlowSession, type FlowStatus } from "../📡️session/🟦️.ts";

/** ⚙️ What a flow stage shows. `projection` returns `null` while the owner cannot place world points (no camera yet). */
export type FlowStageOptions = {
  readonly asset: Asset;
  readonly query: EvaluationQuery;
  readonly environment: Environment;
  readonly clock: StaffageClock | null;
  readonly layers: readonly FlowLayer[];
  readonly theme: "light" | "dark";
  readonly range: DisplayRange;
  readonly scale: number;
  readonly language: Language;
  readonly projection: () => FlowProjection | null;
  readonly onFlow?: (flow: FlowField | null) => void;
  readonly onStatus?: (status: FlowStatus) => void;
};

const STYLE_ID = "semio-flow-style";
const CSS = `
.semio-flow{position:absolute;inset:0;pointer-events:none;overflow:hidden}
.semio-flow canvas{position:absolute;inset:0;inline-size:100%;block-size:100%}
.semio-flow-bar{position:absolute;inset-inline:0;inset-block-end:0;display:flex;flex-wrap:wrap;align-items:center;gap:.5em;padding:.25em .5em;font:inherit;font-size:.75rem;color:var(--staffage-flow-foreground,CanvasText);background:var(--staffage-flow-panel,color-mix(in srgb,Canvas 86%,transparent));pointer-events:auto}
.semio-flow-bar[hidden]{display:none}
.semio-flow-bar progress{flex:1 1 6rem;min-inline-size:4rem;block-size:.5rem;accent-color:var(--staffage-flow-accent,Highlight)}
.semio-flow-bar button{min-block-size:var(--staffage-flow-target,1.75rem);padding:.125em .625em;border:0;background:color-mix(in srgb,currentColor 14%,transparent);color:inherit;font:inherit;cursor:pointer}
.semio-flow-bar button:focus-visible{outline:2px solid var(--staffage-flow-focus,Highlight);outline-offset:1px}
.semio-flow-status{position:absolute;inline-size:1px;block-size:1px;margin:-1px;overflow:hidden;clip:rect(0 0 0 0);white-space:nowrap}
`;

const solveKey = (options: FlowStageOptions): string => JSON.stringify([options.asset.id, options.query.kind, options.query.kind === "pose" ? options.query.pose : options.query.kind === "clip" ? options.query.clip : null, options.environment]);

/** 🌊️ The air flow layer of one figure. */
export class FlowStage {
  readonly #root: HTMLElement;
  readonly #bar: HTMLElement;
  readonly #progress: HTMLProgressElement;
  readonly #text: HTMLElement;
  readonly #button: HTMLButtonElement;
  readonly #status: HTMLElement;
  readonly #overlay: FlowOverlay;
  readonly #session: FlowSession;
  #options: FlowStageOptions;
  #asset: Asset;
  #key = "";
  #unsubscribe: (() => void) | null = null;
  #request = 0;
  #startS = 0;
  #shown: FlowField | null = null;
  #disposed = false;

  constructor(parent: HTMLElement, options: FlowStageOptions) {
    const doc = parent.ownerDocument;
    if (doc.getElementById(STYLE_ID) === null) {
      const style = doc.createElement("style");
      style.id = STYLE_ID;
      style.textContent = CSS;
      doc.head.append(style);
    }
    this.#options = options;
    this.#asset = options.asset;
    this.#root = doc.createElement("div");
    this.#root.className = "semio-flow";
    this.#root.setAttribute("data-flow", "idle");
    const canvas = doc.createElement("canvas");
    this.#bar = doc.createElement("div");
    this.#bar.className = "semio-flow-bar";
    this.#bar.setAttribute("role", "group");
    this.#progress = doc.createElement("progress");
    this.#progress.max = 1;
    this.#text = doc.createElement("span");
    this.#button = doc.createElement("button");
    this.#button.type = "button";
    this.#button.addEventListener("click", () => (this.#session.status.phase === "developing" ? this.#session.cancel() : this.#session.retry()));
    this.#status = doc.createElement("span");
    this.#status.className = "semio-flow-status";
    this.#status.setAttribute("role", "status");
    this.#status.setAttribute("aria-live", "polite");
    this.#bar.append(this.#progress, this.#text, this.#button);
    this.#root.append(canvas, this.#bar, this.#status);
    parent.append(this.#root);
    this.#overlay = new FlowOverlay(canvas);
    this.#session = new FlowSession(() => this.#changed());
    this.#bind(options.clock);
    this.#open();
  }

  /** 🧾️ The state of the solve. */
  get status(): FlowStatus {
    return this.#session.status;
  }

  /** 🔄️ Applies changed options: a new solve when asset, pose or clip or environment changed, otherwise only a redraw. */
  update(patch: Partial<FlowStageOptions>): void {
    if (this.#disposed) return;
    const before = this.#options;
    this.#options = { ...before, ...patch };
    if (this.#options.clock !== before.clock) this.#bind(this.#options.clock);
    if (solveKey(this.#options) !== this.#key || this.#options.asset !== this.#asset) this.#open();
    else {
      this.#words();
      this.redraw();
    }
  }

  /** 🖌️ Draws the current instant on the next animation frame. */
  redraw(): void {
    if (this.#disposed || this.#request !== 0) return;
    const run = (): void => {
      this.#request = 0;
      this.#draw();
    };
    this.#request = typeof requestAnimationFrame === "function" ? requestAnimationFrame(run) : (setTimeout(run, 16) as unknown as number);
  }

  /** 🧹️ Stops the solve and removes the layer. */
  dispose(): void {
    this.#disposed = true;
    this.#unsubscribe?.();
    this.#unsubscribe = null;
    this.#session.close();
    this.#root.remove();
  }

  #clockS(): number {
    const clock = this.#options.clock;
    return clock === null ? 0 : clock.state.frame / clock.state.fps;
  }

  #bind(clock: StaffageClock | null): void {
    this.#unsubscribe?.();
    this.#unsubscribe = clock?.subscribe(() => this.#tick()) ?? null;
  }

  #open(): void {
    const options = this.#options;
    this.#key = solveKey(options);
    this.#asset = options.asset;
    this.#shown = null;
    this.#startS = this.#clockS();
    options.onFlow?.(null);
    this.#session.open(options.asset, options.query, options.environment);
    this.#tick();
  }

  #tick(): void {
    if (this.#disposed) return;
    const clock = this.#options.clock;
    this.#session.frame(this.#clockS(), clock !== null && clock.state.loop ? clock.state.durationS : null);
    this.redraw();
  }

  #changed(): void {
    if (this.#disposed) return;
    const flow = this.#session.flow;
    if (flow !== this.#shown) {
      const first = this.#shown === null;
      this.#shown = flow;
      if (first || flow === null) this.#options.onFlow?.(flow);
    }
    this.#options.onStatus?.(this.#session.status);
    this.#words();
    this.redraw();
  }

  #words(): void {
    const { phase, fraction, message } = this.#session.status;
    const { language } = this.#options;
    const t = translator(language);
    this.#root.setAttribute("data-flow", phase);
    this.#root.lang = language;
    this.#bar.hidden = phase === "ready" || phase === "idle";
    this.#bar.setAttribute("aria-label", t("flow.layers"));
    this.#progress.hidden = phase !== "developing";
    this.#progress.value = fraction;
    this.#progress.setAttribute("aria-label", t("flow.progress"));
    this.#text.textContent = phase === "developing" ? `${t("flow.progress")} · ${t("flow.progressValue", { percent: Math.round(fraction * 100) })}` : phase === "cancelled" ? t("flow.cancelled") : phase === "error" ? `${t("flow.error")} ${message}` : "";
    this.#button.textContent = phase === "developing" ? t("flow.cancel") : t("flow.retry");
    this.#button.setAttribute("data-action", phase === "developing" ? "cancel" : "retry");
    const flow = this.#session.flow;
    const words = phase !== "ready" || flow === null ? this.#text.textContent ?? "" : flow.streams.length === 0 && flow.puffs.length === 0 ? t("flow.still") : t("flow.ready", { streams: flow.streams.length, air: formatNumber(flow.ambient.airTempC, 1, language), rh: Math.round(flow.ambient.relativeHumidity * 100), dew: formatNumber(flow.ambient.dewPointC, 1, language) });
    if (this.#status.textContent !== words) this.#status.textContent = words;
  }

  #draw(): void {
    if (this.#disposed) return;
    const options = this.#options;
    const projection = options.projection();
    const visible = this.#session.status.phase === "ready" || (this.#session.status.phase === "developing" && this.#session.flow !== null);
    if (projection === null) return;
    this.#overlay.draw({ flow: visible ? this.#session.flow : null, timeS: this.#startS + this.#session.elapsedS, layers: options.layers, theme: options.theme, range: options.range, denominator: options.scale, projection });
  }
}
