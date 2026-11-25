/** 🖥️ `FlowStage`: the air flow of one figure as plain DOM laid over it — the overlay canvas, a progress bar with cancel while the flow develops, and the words that say what is shown, what was cancelled or what failed, with a retry. The drawing figure and the 3D stage both mount this one class, so the flow behaves and reads the same in every view.
 *
 * The stage follows its options: asset, pose or clip, environment, the conditions of the air and whether the parcel paths are a layer decide the solve ({@link FlowSession}; changing one of them starts it again); layers, theme, range, scale and the mark detail decide the marks (changing one of them only redraws); the shared {@link StaffageClock} is the only time (a paused clock shows a still field, a missing clock the developed field of the first instant). The owner supplies the projection and calls {@link FlowStage.redraw} when it changed (camera, zoom, size).
 * The stage lives where it is mounted: inside a shadow root its style goes into that root, and {@link FlowStage.mount} moves it to another parent when the owner rewrote its own markup.
 * Accessibility: the canvas is decoration (`aria-hidden`); a polite status states the field and its wind in words; the progress bar and the buttons are native elements with names. Reduced motion is the clock's concern: a paused clock moves nothing here.
 *
 * @see ../📡️session/🟦️.ts — the solver client
 * @see ../🖌️overlay/🟦️.ts — the renderer
 */
import { CAMERAS, axonometricCamera, evaluate, formatNumber, moveFlow, project } from "@teaching/staffage";
import type { Asset, DisplayRange, DrawingScene, Environment, EvaluationQuery, FlowConditions, FlowField, FlowLayer, FlowMarkDetail, Language, Viewpoint } from "@teaching/staffage";
import type { StaffageClock } from "../../⏱️clock/🧱️core/🟦️.ts";
import { translator } from "../../🌐️i18n/🟦️.ts";
import type { SceneFrame } from "../../🧮️pipeline/🟦️.ts";
import { flowPeriodS, windSector } from "../🟦️.ts";
import { FlowOverlay, type FlowProjection } from "../🖌️overlay/🟦️.ts";
import { FlowSession, type FlowStatus } from "../📡️session/🟦️.ts";

/** ⚙️ What a flow stage shows. `projection` returns `null` while the owner cannot place world points (no camera yet). */
export type FlowStageOptions = {
  readonly asset: Asset;
  readonly query: EvaluationQuery;
  readonly environment: Environment;
  readonly conditions?: FlowConditions;
  readonly detail?: FlowMarkDetail;
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
const adopted = new WeakMap<Document | ShadowRoot, CSSStyleSheet>();

/** 🎨️ Gives the air flow layer in `parent` its style where `parent` is styled from, the document or its shadow root: one constructable style sheet adopted there (a Content-Security-Policy that forbids inline `<style>` elements, like the site's release policy, does not block it); where the platform has none, one `<style>` element.
 * @see ../../🧊️spatial/🖥️stage/🟦️.ts — the 3D stage is styled the same way */
export function styleFlowLayer(parent: HTMLElement): void {
  const doc = parent.ownerDocument;
  const scope = parent.getRootNode();
  const shadow = typeof ShadowRoot === "function" && scope instanceof ShadowRoot ? scope : null;
  const container: Document | ShadowRoot = shadow ?? doc;
  if ("adoptedStyleSheets" in container && Array.isArray(container.adoptedStyleSheets) && typeof CSSStyleSheet === "function" && "replaceSync" in CSSStyleSheet.prototype) {
    let sheet = adopted.get(container);
    if (sheet === undefined) {
      sheet = new CSSStyleSheet();
      sheet.replaceSync(CSS);
      adopted.set(container, sheet);
    }
    if (!container.adoptedStyleSheets.includes(sheet)) container.adoptedStyleSheets = [...container.adoptedStyleSheets, sheet];
    return;
  }
  const sheets: ParentNode = shadow ?? doc.head;
  if (sheets.querySelector(`#${STYLE_ID}`) !== null) return;
  const style = doc.createElement("style");
  style.id = STYLE_ID;
  style.textContent = CSS;
  sheets.append(style);
}
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

const solveKey = (options: FlowStageOptions): string => JSON.stringify([options.asset.id, options.query.kind, options.query.kind === "pose" ? options.query.pose : options.query.kind === "clip" ? options.query.clip : null, options.environment, options.conditions ?? {}, options.layers.includes("pathlines")]);

/** 🗺️ How the world lands on a drawing that fills `host`: the paper mapping of `scene` in `viewpoint`, through the paper window the figure shows (the whole scene when none), fitted and centred as the inline SVG is, seen from the eye of the viewpoint (so the marks behind the body are hidden as on paper). `null` while the host has no size. */
export function paperProjection(host: { readonly clientWidth: number; readonly clientHeight: number } | null, scene: DrawingScene, window: SceneFrame | undefined, viewpoint: Viewpoint): FlowProjection | null {
  if (host === null || host.clientWidth === 0 || host.clientHeight === 0) return null;
  const { origin, mmPerMetre: k } = scene.metadata;
  const view = window === undefined ? { x: 0, y: 0, w: scene.widthMm, h: scene.heightMm } : { x: origin.xMm + window.minX, y: origin.yMm + window.minY, w: window.maxX - window.minX, h: window.maxY - window.minY };
  const pxPerMm = Math.min(host.clientWidth / view.w, host.clientHeight / view.h);
  const left = (host.clientWidth - view.w * pxPerMm) / 2;
  const top = (host.clientHeight - view.h * pxPerMm) / 2;
  const camera = viewpoint === "axonometric" ? axonometricCamera() : undefined;
  const looking = camera === undefined ? CAMERAS[viewpoint as Exclude<Viewpoint, "axonometric">].looking : camera.looking;
  return {
    pxPerMm,
    eye: { toward: { x: -looking.x, y: -looking.y, z: -looking.z } },
    project: (at, out) => {
      const q = project(viewpoint, at, camera);
      out[0] = left + (origin.xMm + q.x * k - view.x) * pxPerMm;
      out[1] = top + (origin.yMm - q.y * k - view.y) * pxPerMm;
      return true;
    },
  };
}

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
  #stride: { readonly x: number; readonly y: number; readonly z: number } | null = null;
  readonly #moved = new WeakMap<FlowField, FlowField>();

  constructor(parent: HTMLElement, options: FlowStageOptions) {
    const doc = parent.ownerDocument;
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
    this.mount(parent);
    this.#overlay = new FlowOverlay(canvas);
    this.#session = new FlowSession(() => this.#changed());
    this.#bind(options.clock);
    this.#open();
  }

  /** 🧾️ The state of the solve. */
  get status(): FlowStatus {
    return this.#session.status;
  }

  /** 🌊️ The latest field, `null` until the first one arrived. */
  get flow(): FlowField | null {
    return this.#session.flow;
  }

  /** 📌️ Puts the layer into `parent` (again): the owner calls it after it replaced the element the layer stood in. The style sheet of the layer goes where `parent` is styled from, the document or its shadow root. */
  mount(parent: HTMLElement): void {
    styleFlowLayer(parent);
    if (this.#root.parentNode !== parent) parent.append(this.#root);
    this.redraw();
  }

  /** 🔄️ Applies changed options: a new solve when asset, pose or clip, environment or conditions changed, otherwise only a redraw. */
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
    this.#stride = null;
    this.#startS = this.#clockS();
    options.onFlow?.(null);
    this.#session.open(options.asset, options.query, options.environment, options.conditions, options.layers.includes("pathlines"));
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
    const wind = phase !== "ready" || flow === null ? "" : flow.ambient.windMS > 0 ? t("flow.windWords", { direction: t(`wind.towards.${windSector(flow.ambient.windDirectionDeg)}`), bearing: formatNumber(flow.ambient.windDirectionDeg, 0, language), speed: formatNumber(flow.ambient.windMS, 1, language) }) : t("flow.windStill");
    this.#root.setAttribute("data-wind", flow === null ? "" : `${flow.ambient.windMS} ${flow.ambient.windDirectionDeg}`);
    const spoken = wind === "" ? words : `${words} ${wind}`;
    if (this.#status.textContent !== spoken) this.#status.textContent = spoken;
  }

  /** 🚶️ The field around the figure the clock shows: a loop in which the figure travels (a walk cycle) is solved at a later round of the same phase, so the field is moved back by the whole rounds the rig root travelled since (`moveFlow`); the stride of one round is evaluated once per solve. */
  #around(flow: FlowField | null): FlowField | null {
    if (flow === null) return null;
    const { asset, query } = this.#options;
    const periodS = flowPeriodS(asset, query);
    if (query.kind !== "clip" || periodS <= 0) return flow;
    const kept = this.#moved.get(flow);
    if (kept !== undefined) return kept;
    if (this.#stride === null) {
      const from = evaluate(asset, { ...query, timeS: 0 }).bones[asset.rig.root]?.frame.o;
      const to = evaluate(asset, { ...query, timeS: periodS }).bones[asset.rig.root]?.frame.o;
      this.#stride = from === undefined || to === undefined ? { x: 0, y: 0, z: 0 } : { x: to.x - from.x, y: to.y - from.y, z: to.z - from.z };
    }
    const rounds = Math.floor(flow.timeS / periodS + 1e-9);
    const moved = moveFlow(flow, { x: -rounds * this.#stride.x, y: -rounds * this.#stride.y, z: -rounds * this.#stride.z });
    this.#moved.set(flow, moved);
    return moved;
  }

  #draw(): void {
    if (this.#disposed) return;
    const options = this.#options;
    const projection = options.projection();
    const visible = this.#session.status.phase === "ready" || (this.#session.status.phase === "developing" && this.#session.flow !== null);
    if (projection === null) return;
    this.#overlay.draw({ flow: visible ? this.#around(this.#session.flow) : null, timeS: this.#startS + this.#session.elapsedS, layers: options.layers, theme: options.theme, range: options.range, denominator: options.scale, ...(options.detail === undefined ? {} : { detail: options.detail }), projection });
  }
}
