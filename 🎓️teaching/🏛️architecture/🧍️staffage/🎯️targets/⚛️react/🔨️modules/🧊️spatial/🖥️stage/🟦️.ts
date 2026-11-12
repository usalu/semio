/** 🖥️ `ModelStage`: the interactive 3D view of one asset as plain DOM — a toolbar, the owned WebGL2 `ModelViewer` on a canvas, a progress bar with cancel while the model is prepared, and the notes and alerts that say what is going on. The React figure and the `<staffage-figure>` element both mount this one class, so they behave and read the same.
 *
 * The stage follows its options: the representation is a variant of the model, the pose or clip and the scale decide which model is built (see {@link ../🟦️.ts}), the camera is a preset the reader can leave by orbiting, the environment and analytic settings colour the thermal and force variants. Time is never the stage's own: the shared {@link StaffageClock} drives the viewer (play, pause, scrub and rate stay the one transport of the 2D drawing), and Space on the canvas toggles that clock.
 * Where WebGL2 is missing or the context cannot be restored the viewer draws in software and says so; a model that cannot be shown is explained in an alert, never a blank box.
 *
 * Accessibility: native buttons with names and a visible focus ring, `aria-pressed` presets, a canvas with role, name, instructions and a translated text alternative of the state (asset, pose or clip, representation, scale, camera), polite status and live regions. Customisation: the custom properties `--staffage-model-background`, `--staffage-model-foreground`, `--staffage-model-border`, `--staffage-model-accent`, `--staffage-model-focus`, `--staffage-model-panel`, `--staffage-model-min-height` and `--staffage-model-target` (the minimum size of a control); the picture's own colour is read by the viewer, so give a colour as hex, `rgb()` or `rgba()`. Forced colours, increased contrast and reduced motion are honoured.
 *
 * @see ../🏭️prepare/🟦️.ts — the model pipeline
 * @see ../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🕹️controller/🟦️.ts — the viewer
 */
import { ModelViewer, PRESETS, type BackendFactory, type CameraPreset, type ViewerStatus } from "../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🟦️.ts";
import type { Asset, Language } from "@teaching/staffage";
import type { StaffageClock } from "../../⏱️clock/🧱️core/🟦️.ts";
import { translator, type Key } from "../../🌐️i18n/🟦️.ts";
import { VIEW_NAMES } from "../../🗣️labels/🟦️.ts";
import { PREPARE_PHASES, phaseStart, prepareModel, type PreparePhase } from "../🏭️prepare/🟦️.ts";
import { CAMERA_PRESETS, describeModel, modelKey, variantOf, type CameraPresetName, type SpatialRequest } from "../🟦️.ts";

/** 🧾️ Where the stage is: nothing yet, preparing the model, showing it, cancelled by the reader, failed, or refused because the asset has no 3D volume. */
export type StagePhase = "idle" | "preparing" | "ready" | "cancelled" | "error" | "unavailable";

/** 📤️ What a stage reports about itself. */
export type StageState = { readonly phase: StagePhase; readonly backend: "webgl2" | "software" | "none"; readonly fraction: number; readonly message: string };

/** ⚙️ What a stage shows and how it behaves. */
export type StageOptions = SpatialRequest & {
  readonly asset: Asset;
  readonly clock?: StaffageClock | null;
  readonly preset: CameraPresetName;
  readonly background?: string | null;
  readonly toolbar?: boolean;
  readonly reducedMotion?: boolean;
  readonly onPreset?: (preset: CameraPresetName) => void;
  readonly onState?: (state: StageState) => void;
  readonly backend?: BackendFactory;
  readonly settleMs?: number;
  readonly restoreMs?: number;
};

const STYLE = `
.semio-stage{position:relative;display:flex;flex-direction:column;gap:.5rem;box-sizing:border-box;inline-size:100%;block-size:100%;min-block-size:var(--staffage-model-min-height,18rem);color:var(--staffage-model-foreground,inherit);font:inherit;font-size:.8125rem}
.semio-stage *{box-sizing:border-box}
.semio-stage-bar{display:flex;flex-wrap:wrap;gap:.375rem;align-items:center}
.semio-stage-group{display:inline-flex;flex-wrap:wrap;gap:0;border:1px solid var(--staffage-model-border,currentColor)}
.semio-stage button{min-block-size:var(--staffage-model-target,1.75rem);min-inline-size:var(--staffage-model-target,1.75rem);padding:.125rem .625rem;border:1px solid var(--staffage-model-border,currentColor);background:var(--staffage-model-panel,transparent);color:inherit;font:inherit;cursor:pointer}
.semio-stage-group button{border:0;border-inline-start:1px solid var(--staffage-model-border,currentColor)}
.semio-stage-group button:first-child{border-inline-start:0}
.semio-stage button:hover{background:color-mix(in srgb,var(--staffage-model-accent,currentColor) 18%,transparent)}
.semio-stage button[aria-pressed="true"]{background:var(--staffage-model-accent,Highlight);color:var(--staffage-model-accent-foreground,HighlightText);font-weight:600}
.semio-stage button:focus-visible,.semio-stage canvas:focus-visible{outline:2px solid var(--staffage-model-focus,var(--staffage-model-accent,Highlight));outline-offset:2px}
.semio-stage-view{position:relative;flex:1 1 auto;min-block-size:12rem;border:1px solid var(--staffage-model-border,currentColor);background:var(--staffage-model-background,transparent)}
.semio-stage[data-ground="transparent"] .semio-stage-view{background-color:#fff;background-image:conic-gradient(#c9c9c9 25%,transparent 0 50%,#c9c9c9 0 75%,transparent 0);background-size:16px 16px}
.semio-stage canvas{display:block;inline-size:100%;block-size:100%;min-block-size:12rem;outline:none;--semio-viewer-background:var(--staffage-model-background,#f7f3e3)}
.semio-stage-progress{display:flex;flex-wrap:wrap;align-items:center;gap:.5rem;padding:.375rem;border:1px solid var(--staffage-model-border,currentColor);background:var(--staffage-model-panel,transparent)}
.semio-stage-progress progress{flex:1 1 8rem;min-inline-size:6rem;block-size:.5rem;accent-color:var(--staffage-model-accent,Highlight)}
.semio-stage-notice{margin:0;padding:.375rem .5rem;border:1px dashed var(--staffage-model-border,currentColor)}
.semio-stage-notice b{display:block}
.semio-stage-note{margin:0;font-size:.75rem;opacity:.85}
.semio-stage [hidden]{display:none!important}
.semio-stage .semio-sr{position:absolute;inline-size:1px;block-size:1px;margin:-1px;padding:0;overflow:hidden;clip:rect(0 0 0 0);white-space:nowrap;border:0}
@media (prefers-contrast:more){.semio-stage button,.semio-stage-view,.semio-stage-group{border-width:2px}}
@media (prefers-reduced-motion:reduce){.semio-stage *{transition:none!important;animation:none!important}}
@media (forced-colors:active){.semio-stage button[aria-pressed="true"]{forced-color-adjust:none;background:Highlight;color:HighlightText}.semio-stage-view,.semio-stage-group,.semio-stage button{border-color:CanvasText}.semio-stage canvas:focus-visible,.semio-stage button:focus-visible{outline-color:Highlight}}
`;

const ZOOM_STEP = 1.25;
let counter = 0;
const adopted = new WeakMap<Document | ShadowRoot, CSSStyleSheet>();

/** 🎨️ Gives the stage its style: one constructable stylesheet adopted by the document or shadow root the stage lives in (a Content-Security-Policy that forbids inline `<style>` elements, like the site's release policy, does not block it); where the platform has none, a `<style>` element in the stage. */
function styled(host: HTMLElement, root: HTMLElement): void {
  const scope = host.getRootNode();
  const container = scope instanceof ShadowRoot || scope instanceof Document ? scope : null;
  if (container !== null && "adoptedStyleSheets" in container && typeof CSSStyleSheet === "function" && "replaceSync" in CSSStyleSheet.prototype) {
    let sheet = adopted.get(container);
    if (sheet === undefined) {
      sheet = new CSSStyleSheet();
      sheet.replaceSync(STYLE);
      adopted.set(container, sheet);
    }
    if (!container.adoptedStyleSheets.includes(sheet)) container.adoptedStyleSheets = [...container.adoptedStyleSheets, sheet];
    return;
  }
  const style = root.ownerDocument.createElement("style");
  style.textContent = STYLE;
  root.prepend(style);
}

const presetOfCamera = (camera: { yaw: number; pitch: number; projection: string }): CameraPresetName | null =>
  CAMERA_PRESETS.find((name) => Math.abs(PRESETS[name].yaw - camera.yaw) < 1e-6 && Math.abs(PRESETS[name].pitch - camera.pitch) < 1e-6 && PRESETS[name].projection === camera.projection) ?? null;

const isAbort = (error: unknown): boolean => (error instanceof DOMException && error.name === "AbortError") || (error instanceof Error && error.name === "AbortError");

/** 🌟️ The stage. Construct it over an empty host element, call {@link ModelStage.update} when options change and {@link ModelStage.dispose} when done. */
export class ModelStage {
  readonly #host: HTMLElement;
  readonly #root: HTMLElement;
  readonly #view: HTMLElement;
  readonly #bar: HTMLElement;
  readonly #presets: HTMLElement;
  readonly #progress: HTMLElement;
  readonly #bar2: HTMLProgressElement;
  readonly #progressText: HTMLElement;
  readonly #notice: HTMLElement;
  readonly #note: HTMLElement;
  readonly #description: HTMLElement;
  readonly #id = `semio-stage-${++counter}`;
  #options: StageOptions;
  #viewer: ModelViewer | null = null;
  #canvas: HTMLCanvasElement | null = null;
  #unsubscribe: (() => void) | null = null;
  #clock: StaffageClock | null = null;
  #job: AbortController | null = null;
  #timer: ReturnType<typeof setTimeout> | null = null;
  #lostTimer: ReturnType<typeof setTimeout> | null = null;
  #software = false;
  #loaded = "";
  #requested = "";
  #noticeKey = "";
  #phase: StagePhase = "idle";
  #fraction = 0;
  #message = "";
  #wanted: CameraPresetName | null = null;
  #shown: CameraPresetName | null = null;
  #variant = "";
  #selection = "";
  #disposed = false;
  #ready = false;

  constructor(host: HTMLElement, options: StageOptions) {
    this.#host = host;
    this.#options = options;
    const doc = host.ownerDocument;
    const make = <K extends keyof HTMLElementTagNameMap>(tag: K, className: string, parent: HTMLElement): HTMLElementTagNameMap[K] => {
      const element = doc.createElement(tag);
      element.className = className;
      parent.append(element);
      return element;
    };
    this.#root = doc.createElement("div");
    this.#root.className = "semio-stage";
    this.#root.setAttribute("part", "stage");
    this.#root.setAttribute("data-model-stage", "");
    this.#bar = make("div", "semio-stage-bar", this.#root);
    this.#bar.setAttribute("part", "toolbar");
    this.#bar.setAttribute("role", "group");
    this.#presets = make("div", "semio-stage-group", this.#bar);
    this.#presets.setAttribute("role", "group");
    this.#view = make("div", "semio-stage-view", this.#root);
    this.#progress = make("div", "semio-stage-progress", this.#root);
    this.#bar2 = make("progress", "", this.#progress);
    this.#bar2.max = 100;
    this.#progressText = make("span", "", this.#progress);
    this.#notice = make("div", "semio-stage-notice", this.#root);
    this.#note = make("p", "semio-stage-note", this.#root);
    this.#note.setAttribute("role", "status");
    this.#description = make("div", "semio-sr", this.#root);
    this.#description.id = `${this.#id}-state`;
    this.#buildBar();
    host.replaceChildren(this.#root);
    styled(host, this.#root);
    this.#bindClock(options.clock ?? null);
    this.#mountViewer();
    this.#refresh();
    this.#requested = modelKey(options.asset, options);
    this.#schedule(0);
  }

  /** 🧭️ The state of the stage. */
  get state(): StageState {
    return { phase: this.#phase, backend: this.#viewer === null ? "none" : this.#viewer.backendKind === "webgl2" ? "webgl2" : "software", fraction: this.#fraction, message: this.#message };
  }

  /** 👁️ The viewer, once mounted (tests and hosts that want its events). */
  get viewer(): ModelViewer | null {
    return this.#viewer;
  }

  /** 🔄️ Applies changed options: the representation and the camera at once, a new model after a short settle (so dragging a slider does not build a model per tick). */
  update(patch: Partial<StageOptions>): void {
    if (this.#disposed) return;
    const before = this.#options;
    const next = { ...before, ...patch } as StageOptions;
    this.#options = next;
    if (next.clock !== before.clock) this.#bindClock(next.clock ?? null);
    if (next.language !== before.language) {
      this.#buildBar();
      this.#mountViewer();
      this.#loaded = "";
      this.#requested = "";
    }
    const key = modelKey(next.asset, next);
    if (key !== this.#requested) {
      this.#requested = key;
      this.#schedule(this.#loaded === "" ? 0 : (next.settleMs ?? 200));
    }
    this.#applyLook();
    this.#refresh();
  }

  /** 🛑️ Cancels a running preparation; the stage says so and offers a retry. */
  cancel(): void {
    if (this.#phase !== "preparing") return;
    const job = this.#job;
    this.#job = null;
    job?.abort();
    this.#viewer?.cancelPreparation();
    this.#ready = this.#loaded !== "";
    this.#set("cancelled", 0, "");
  }

  /** 🔁️ Prepares the model again after a cancellation or a failure. */
  retry(): void {
    this.#schedule(0);
  }

  /** 🧹️ Stops everything and empties the host. */
  dispose(): void {
    this.#disposed = true;
    this.#job?.abort();
    this.#job = null;
    if (this.#timer !== null) clearTimeout(this.#timer);
    if (this.#lostTimer !== null) clearTimeout(this.#lostTimer);
    this.#unsubscribe?.();
    this.#unsubscribe = null;
    this.#viewer?.dispose();
    this.#viewer = null;
    this.#host.replaceChildren();
  }

  get #t() {
    const t = translator(this.#options.language);
    return (key: Key, params?: Record<string, string | number>): string => t(key, params);
  }

  #buildBar(): void {
    const t = this.#t;
    const doc = this.#host.ownerDocument;
    this.#root.setAttribute("lang", this.#options.language);
    this.#bar.setAttribute("aria-label", t("model.toolbar"));
    this.#presets.setAttribute("aria-label", t("model.camera"));
    this.#presets.replaceChildren(
      ...CAMERA_PRESETS.map((name) => {
        const button = doc.createElement("button");
        button.type = "button";
        button.textContent = t(`model.camera.${name}` as Key);
        button.setAttribute("data-preset", name);
        button.setAttribute("aria-pressed", String(this.#shown === name));
        button.addEventListener("click", () => this.#choose(name));
        return button;
      }),
    );
    for (const old of this.#bar.querySelectorAll("[data-action]")) old.remove();
    const action = (name: "zoom-out" | "zoom-in" | "reset", label: string, text: string, run: () => void): void => {
      const button = doc.createElement("button");
      button.type = "button";
      button.setAttribute("data-action", name);
      button.setAttribute("aria-label", label);
      button.title = label;
      button.textContent = text;
      button.addEventListener("click", run);
      this.#bar.append(button);
    };
    action("zoom-out", t("model.zoomOut"), "−", () => this.#viewer?.zoomBy(1 / ZOOM_STEP));
    action("zoom-in", t("model.zoomIn"), "+", () => this.#viewer?.zoomBy(ZOOM_STEP));
    action("reset", t("model.reset"), "⟲", () => this.#reset());
    const cancel = this.#progress.querySelector("button");
    cancel?.remove();
    const button = doc.createElement("button");
    button.type = "button";
    button.setAttribute("data-action", "cancel");
    button.textContent = t("model.cancel");
    button.addEventListener("click", () => this.cancel());
    this.#progress.append(button);
    this.#bar2.setAttribute("aria-label", t("model.progress"));
    this.#noticeKey = "";
  }

  #choose(name: CameraPresetName): void {
    this.#wanted = name;
    this.#applyPreset(name);
    this.#options.onPreset?.(name);
  }

  #reset(): void {
    const preset = this.#wanted ?? this.#options.preset;
    this.#viewer?.setPreset(preset as CameraPreset);
    this.#shown = preset;
    this.#markPresets();
  }

  #applyPreset(name: CameraPresetName): void {
    if (this.#viewer === null || !this.#ready) return;
    this.#viewer.setPreset(name as CameraPreset);
    this.#shown = name;
    this.#markPresets();
  }

  #markPresets(): void {
    for (const button of this.#presets.querySelectorAll("button")) button.setAttribute("aria-pressed", String(button.getAttribute("data-preset") === this.#shown));
  }

  #bindClock(clock: StaffageClock | null): void {
    this.#unsubscribe?.();
    this.#unsubscribe = null;
    this.#clock = clock;
    this.#unsubscribe = clock?.subscribe(() => this.#tick()) ?? null;
  }

  #timeS(): number {
    const clock = this.#clock;
    if (clock !== null) return clock.state.frame / clock.state.fps;
    return this.#options.query.kind === "clip" ? this.#options.query.timeS : 0;
  }

  #tick(): void {
    if (this.#disposed || !this.#ready || this.#viewer === null || this.#options.query.kind !== "clip") return;
    this.#viewer.seek(this.#timeS());
    if (this.#clock?.state.playing !== true) this.#refresh();
  }

  #mountViewer(): void {
    const doc = this.#host.ownerDocument;
    this.#viewer?.dispose();
    this.#view.replaceChildren();
    this.#ready = false;
    const canvas = doc.createElement("canvas");
    this.#canvas = canvas;
    this.#view.append(canvas);
    const t = this.#t;
    const options = this.#options;
    const name = (key: "camera" | "view" | "clip" | "pose", id: string): string =>
      key === "camera" ? t(`model.camera.${id}` as Key) : key === "view" ? (VIEW_NAMES[id as keyof typeof VIEW_NAMES]?.[options.language] ?? id) : key === "clip" ? (this.#options.asset.clips.find((clip) => clip.id === id)?.title[options.language] ?? id) : (this.#options.asset.poses.find((pose) => pose.id === id)?.title[options.language] ?? id);
    const status = (s: ViewerStatus): string => {
      switch (s.kind) {
        case "ready": return t("model.status.ready");
        case "preset": return t("model.status.preset", { camera: name("camera", s.preset) });
        case "fit": return t("model.status.fit");
        case "playing": return t("model.status.playing");
        case "paused": return t("model.status.paused");
        case "variant": return s.variant === null ? "" : t("model.status.variant", { representation: name("view", s.variant) });
        case "animation": return s.animation === null ? "" : t("model.status.animation", { clip: name("clip", s.animation) });
        case "pose": return s.pose === null ? "" : t("model.status.pose", { pose: name("pose", s.pose) });
        case "orbit": return t("model.status.orbit");
        case "pan": return t("model.status.pan");
        case "zoom": return t("model.status.zoom");
      }
    };
    if (options.background === undefined) canvas.style.removeProperty("--semio-viewer-background");
    const viewer = new ModelViewer({
      canvas,
      strings: { label: t("model.label", { title: options.asset.title[options.language] }), roleDescription: t("model.roleDescription"), description: t("model.instructions"), status },
      ...(this.#software ? { backend: () => { throw new Error("software"); } } : options.backend === undefined ? {} : { backend: options.backend }),
      fallback: true,
      background: null,
      autoplay: false,
      preset: this.#shown ?? options.preset,
      ...(options.reducedMotion === undefined ? {} : { reducedMotion: options.reducedMotion }),
    });
    this.#viewer = viewer;
    this.#applyBackground();
    viewer.on("camera", ({ camera }) => {
      if (!this.#ready) return;
      const found = presetOfCamera(camera);
      this.#shown = found;
      this.#markPresets();
      if (found !== null && found !== this.#wanted) {
        this.#wanted = found;
        this.#options.onPreset?.(found);
      }
      this.#refresh();
    });
    viewer.on("backend", ({ status: state }) => this.#backend(state));
    viewer.on("issues", () => this.#note.setAttribute("data-issues", ""));
    viewer.on("error", ({ error }) => this.#fail(error));
    viewer.on("ready", () => (this.#ready = true));
    this.#view.addEventListener(
      "keydown",
      (event) => {
        if (event.key !== " " || this.#clock === null || this.#options.query.kind !== "clip") return;
        event.preventDefault();
        event.stopPropagation();
        this.#clock.toggle();
      },
      true,
    );
    canvas.setAttribute("aria-describedby", `${canvas.getAttribute("aria-describedby") ?? ""} ${this.#description.id}`.trim());
    this.#refresh();
  }

  #backend(status: "ready" | "lost" | "restored"): void {
    const t = this.#t;
    if (this.#lostTimer !== null) clearTimeout(this.#lostTimer);
    this.#lostTimer = null;
    if (status === "lost") {
      this.#note.textContent = t("model.contextLost");
      this.#lostTimer = setTimeout(() => this.#toSoftware(), this.#options.restoreMs ?? 4000);
    } else if (status === "restored") this.#note.textContent = t("model.contextRestored");
    else this.#refresh();
  }

  #toSoftware(): void {
    if (this.#disposed || this.#software) return;
    this.#software = true;
    this.#mountViewer();
    this.#note.textContent = this.#t("model.contextSwitched");
    this.#loaded = "";
    this.#schedule(0);
  }

  #schedule(delayMs: number): void {
    if (this.#timer !== null) clearTimeout(this.#timer);
    this.#timer = setTimeout(() => {
      this.#timer = null;
      void this.#prepare();
    }, delayMs);
  }

  async #prepare(): Promise<void> {
    if (this.#disposed || this.#viewer === null) return;
    const options = this.#options;
    const asset = options.asset;
    const t = this.#t;
    if (asset.volume === undefined) return this.#set("unavailable", 0, t("model.noVolume"));
    if (options.query.kind === "transition") return this.#set("unavailable", 0, t("model.noVolume"));
    const key = modelKey(asset, options);
    if (key === this.#loaded && this.#phase === "ready") return;
    this.#job?.abort();
    this.#viewer.cancelPreparation();
    const job = new AbortController();
    this.#job = job;
    this.#ready = false;
    this.#set("preparing", 0, "");
    try {
      const document = await prepareModel(asset, options, { signal: job.signal, progress: (fraction, phase) => this.#advance(job, fraction, phase) });
      if (this.#job !== job) return;
      const viewer = this.#viewer;
      if (viewer === null) return;
      let scene = false;
      const stop = viewer.on("progress", ({ fraction }) => this.#advance(job, phaseStart("scene") + 0.15 * fraction, "scene"));
      const ready = viewer.on("ready", () => (scene = true));
      try {
        await viewer.setDocument(document, job.signal);
      } finally {
        stop();
        ready();
      }
      if (this.#job !== job || !scene) return;
      this.#job = null;
      this.#loaded = key;
      this.#variant = "";
      this.#selection = "";
      this.#wanted = options.preset;
      this.#shown = null;
      this.#set("ready", 1, "");
      this.#applyLook(true);
      this.#refresh();
    } catch (error) {
      if (this.#job !== job) return;
      this.#job = null;
      this.#ready = this.#loaded !== "";
      if (isAbort(error)) return this.#set("cancelled", 0, "");
      this.#fail(error);
    }
  }

  #advance(job: AbortController, fraction: number, phase: PreparePhase): void {
    if (this.#job !== job) return;
    this.#set("preparing", fraction, this.#t(`model.phase.${phase}` as Key));
  }

  #fail(error: unknown): void {
    this.#viewer?.cancelPreparation();
    this.#set("error", 0, error instanceof Error ? error.message : String(error));
  }

  #applyBackground(): void {
    const canvas = this.#canvas;
    if (canvas === null) return;
    const { background } = this.#options;
    if (background === undefined || background === null) canvas.style.removeProperty("--semio-viewer-background");
    else canvas.style.setProperty("--semio-viewer-background", background);
    this.#root.setAttribute("data-ground", background === "transparent" ? "transparent" : "solid");
    this.#viewer?.requestRender();
  }

  #applyLook(force = false): void {
    const viewer = this.#viewer;
    const options = this.#options;
    this.#applyBackground();
    if (viewer === null || !this.#ready) return;
    try {
      const variant = variantOf(options.view);
      if (force || variant !== this.#variant) {
        viewer.setVariant(variant);
        this.#variant = variant;
      }
      const query = options.query;
      const selection = query.kind === "pose" ? `pose:${query.pose}` : query.kind === "clip" ? `clip:${query.clip}` : "";
      if (force || selection !== this.#selection) {
        if (query.kind === "pose") viewer.setPose(query.pose);
        else if (query.kind === "clip") {
          viewer.setAnimation(query.clip, this.#timeS());
          viewer.setLoop(this.#clock?.state.loop ?? null);
        }
        this.#selection = selection;
      }
      if (query.kind === "clip") viewer.seek(this.#timeS());
      if (force || options.preset !== this.#wanted) {
        this.#wanted = options.preset;
        if (options.preset !== this.#shown) this.#applyPreset(options.preset);
      }
    } catch (error) {
      this.#fail(error);
    }
  }

  #set(phase: StagePhase, fraction: number, message: string): void {
    this.#phase = phase;
    this.#fraction = fraction;
    this.#message = message;
    this.#refresh();
    this.#options.onState?.(this.state);
  }

  #refresh(): void {
    if (this.#disposed) return;
    const t = this.#t;
    const phase = this.#phase;
    const options = this.#options;
    this.#root.setAttribute("data-state", phase);
    this.#root.setAttribute("data-backend", this.state.backend);
    this.#root.setAttribute("data-asset", options.asset.id);
    this.#root.setAttribute("data-view", options.view);
    this.#root.setAttribute("data-preset", this.#shown ?? "free");
    this.#progress.hidden = phase !== "preparing";
    if (phase === "preparing") {
      const percent = Math.round(this.#fraction * 100);
      this.#bar2.value = percent;
      this.#progressText.textContent = `${this.#message === "" ? t("model.progress") : this.#message} · ${t("model.progressValue", { percent })}`;
    }
    this.#bar.hidden = options.toolbar === false;
    this.#notice.hidden = phase === "preparing" || phase === "ready" || phase === "idle";
    const noticeKey = `${phase}|${this.#message}|${options.language}`;
    if (noticeKey !== this.#noticeKey) {
      this.#noticeKey = noticeKey;
      this.#notice.replaceChildren();
      if (!this.#notice.hidden) {
        const doc = this.#host.ownerDocument;
        const box = doc.createElement("div");
        const title = doc.createElement("b");
        const body = doc.createElement("span");
        box.setAttribute("role", phase === "error" ? "alert" : "status");
        title.textContent = phase === "cancelled" ? t("model.cancelled") : phase === "unavailable" ? t("model.noVolume") : t("model.error");
        body.textContent = phase === "error" ? t("model.errorBody", { message: this.#message }) : "";
        box.append(title, body);
        this.#notice.append(box);
        if (phase !== "unavailable") {
          const button = doc.createElement("button");
          button.type = "button";
          button.setAttribute("data-action", "retry");
          button.textContent = t("model.retry");
          button.addEventListener("click", () => this.retry());
          this.#notice.append(button);
        }
      }
    }
    this.#view.hidden = phase === "unavailable";
    this.#note.hidden = false;
    const software = this.state.backend === "software" && phase !== "idle";
    if (this.#viewer !== null && this.#note.textContent !== t("model.contextLost") && this.#note.textContent !== t("model.contextSwitched")) this.#note.textContent = software ? t("model.backend.software") : "";
    this.#note.hidden = this.#note.textContent === "";
    this.#description.textContent = describeModel(options.asset, { ...options, timeS: this.#timeS() }, this.#shown, options.language);
    this.#canvas?.setAttribute("aria-label", t("model.label", { title: options.asset.title[options.language] }));
  }
}
