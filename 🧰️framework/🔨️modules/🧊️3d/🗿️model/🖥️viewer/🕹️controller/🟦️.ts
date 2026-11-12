/** 🕹️ `ModelViewer`: a framework-free controller that draws a `ModelDocument` on a `<canvas>` and owns no application state; it emits events instead.
 *
 * - Document: resolved, or procedural with `quality` (tessellated first); preparation reports progress and can be cancelled ({@link ModelViewer.cancelPreparation}), the event loop is never blocked for long.
 * - Look: pose, animation (play, pause, seek, rate, loop; `requestAnimationFrame` clock, suspended while the page is hidden or the canvas is off screen), variant, camera preset, orbit/pan/zoom.
 * - Input: pointer (drag orbit, shift/middle/right drag pan, wheel zoom), two-finger touch (pinch zoom + pan), full keyboard (arrows orbit, shift+arrows pan, `+`/`-` zoom, `Home` fit, `Space` play/pause, `1`..`5` presets), double click fits.
 * - Accessibility: the canvas is focusable with `aria-label`, `aria-roledescription` and `aria-describedby`; a visually hidden description and polite live region are inserted after it. Every string comes from the client (`strings`); there is no built-in language.
 * - Motion: `prefers-reduced-motion: reduce` disables autoplay (an explicit `play()` still works).
 * - Customisation: `background` option or the CSS custom property `--semio-viewer-background` on the canvas, lighting, control switches and speeds, backend factory.
 *
 * - Follow and root motion: `follow` (`off`, `bounds` or `{ node }`) keeps the orbit target on the animated bounds centre or a node, smoothed per frame with `followSmoothingS` seconds (default 0.25; 0 snaps); `rootMotion` `travel` (default) keeps the root translation of the clip, `in-place` removes its horizontal part. Both are specified in `../🎥️follow/🟦️.ts`; while following, the camera target is the followed point (panning has no lasting effect) and `getCamera()` returns the live camera whose target array is updated in place.
 *
 * Rendering is on demand: frames are drawn only when something changed or an animation plays.
 * @see ../🧬️contract/🟦️.ts */
import { CancelledError } from "../../⏱️operation/🟦️.ts";
import { tessellate } from "../../🔺️tessellation/🟦️.ts";
import { clamp } from "../../🧮️math/🟦️.ts";
import type { Animation, ModelDocument, Operation, Quality, Slug } from "../../🧬️schema/🟦️.ts";
import type { BackendStatus, Frame, RenderBackend, Scene, V4 } from "../🧬️contract/🟦️.ts";
import { cameraOf, fitCamera, orbit, pan, withPreset, zoom, type CameraPreset, type CameraState } from "../📷️camera/🟦️.ts";
import { createFollowState, type RootMotion, type ViewerFollow } from "../🎥️follow/🟦️.ts";
import { boundsOf, createFrame, frameOf, type Lighting } from "../🎞️frame/🟦️.ts";
import { pickAt, type Hit } from "../🎯️picking/🟦️.ts";
import { sceneOfAsync } from "../🧱️scene/🟦️.ts";
import { SoftwareBackend } from "../🖼️software/🟦️.ts";
import { createWebGl2Backend } from "../🔶️webgl2/🟦️.ts";
import { parseCssColor } from "./🎨️color/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

/** 🗣️ What happened, for the client to turn into a sentence in its own language. */
export type ViewerStatus =
  | { kind: "ready" }
  | { kind: "preset"; preset: CameraPreset }
  | { kind: "fit" }
  | { kind: "playing" }
  | { kind: "paused" }
  | { kind: "variant"; variant: Slug | null }
  | { kind: "animation"; animation: Slug | null }
  | { kind: "pose"; pose: Slug | null }
  | { kind: "orbit" | "pan" | "zoom" };

/** 🌍️ Client-supplied texts; the viewer has no default language. `status` may return an empty string to stay silent. */
export type ViewerStrings = { readonly label: string; readonly description: string; readonly roleDescription?: string; readonly status?: (status: ViewerStatus) => string };
/** 🎛️ Input switches and speeds. */
export type ViewerControls = { readonly orbit?: boolean; readonly pan?: boolean; readonly zoom?: boolean; readonly keyboard?: boolean; readonly rotateSpeed?: number; readonly panSpeed?: number; readonly zoomSpeed?: number; readonly keyboardStep?: number };
/** 🔌️ Creates the backend of a canvas. */
export type BackendFactory = (canvas: HTMLCanvasElement, onStatus: (status: BackendStatus) => void) => RenderBackend;

/** 🖥️ Construction options. `background` null reads `--semio-viewer-background` from the canvas style (white when absent). `fallback` draws with the software rasteriser into a 2D canvas when the backend cannot be created. */
export type ModelViewerOptions = {
  readonly canvas: HTMLCanvasElement;
  readonly strings: ViewerStrings;
  readonly backend?: BackendFactory;
  readonly fallback?: boolean;
  readonly background?: V4 | null;
  readonly lighting?: Lighting;
  readonly quality?: Quality;
  readonly autoplay?: boolean;
  readonly reducedMotion?: boolean;
  readonly controls?: ViewerControls;
  readonly preset?: CameraPreset;
  readonly fitMargin?: number;
  readonly maxPixelRatio?: number;
  readonly follow?: ViewerFollow;
  readonly followSmoothingS?: number;
  readonly rootMotion?: RootMotion;
  readonly rootNode?: Slug;
};

/** 📣️ Events by name. */
export type ViewerEvents = {
  progress: { fraction: number };
  ready: { scene: Scene };
  error: { error: Error };
  camera: { camera: CameraState };
  preset: { preset: CameraPreset | null };
  follow: { follow: ViewerFollow };
  rootMotion: { rootMotion: RootMotion };
  time: { animation: Slug; timeS: number };
  playing: { playing: boolean };
  pick: { hit: Hit | null; clientX: number; clientY: number };
  backend: { status: BackendStatus };
  issues: { issues: Frame["issues"] };
};

const PRESET_KEYS: Record<string, CameraPreset> = { "1": "elevation", "2": "front", "3": "plan", "4": "axonometric", "5": "perspective" };
const HIDDEN = "position:absolute;width:1px;height:1px;margin:-1px;padding:0;overflow:hidden;clip:rect(0 0 0 0);white-space:nowrap;border:0";
let counter = 0;

/** 🪟️ The viewer controller. Dispose it when done. */
export class ModelViewer {
  readonly canvas: HTMLCanvasElement;
  private readonly options: ModelViewerOptions;
  private readonly controls: Required<ViewerControls>;
  private readonly listeners = new Map<string, Set<(e: never) => void>>();
  private readonly cleanup: (() => void)[] = [];
  private backend: RenderBackend | null = null;
  private software = false;
  private scene: Scene | null = null;
  private frame: Frame | null = null;
  private camera: CameraState;
  private preset: CameraPreset | null;
  private poseId: Slug | null = null;
  private animation: Animation | null = null;
  private timeS = 0;
  private rate = 1;
  private loopOverride: boolean | null = null;
  private playing = false;
  private variant: Slug | null = null;
  private dirty = false;
  private frameRequest = 0;
  private lastNow = -1;
  private visible = true;
  private ratio = 1;
  private cssWidth = 1;
  private cssHeight = 1;
  private preparation: AbortController | null = null;
  private live: HTMLElement | null = null;
  private description: HTMLElement | null = null;
  private disposed = false;
  private reduced: boolean;
  private followMode: ViewerFollow;
  private motion: RootMotion;
  private readonly followState = createFollowState();
  private dtS = 0;
  private lastTarget: [number, number, number] = [NaN, NaN, NaN];

  constructor(options: ModelViewerOptions) {
    this.options = options;
    this.canvas = options.canvas;
    const c = options.controls ?? {};
    this.controls = { orbit: c.orbit ?? true, pan: c.pan ?? true, zoom: c.zoom ?? true, keyboard: c.keyboard ?? true, rotateSpeed: c.rotateSpeed ?? 1, panSpeed: c.panSpeed ?? 1, zoomSpeed: c.zoomSpeed ?? 1, keyboardStep: c.keyboardStep ?? 0.1 };
    this.followMode = options.follow ?? "off";
    this.motion = options.rootMotion ?? "travel";
    this.preset = options.preset ?? "perspective";
    this.camera = cameraOf(this.preset);
    const mq = typeof matchMedia === "function" ? matchMedia("(prefers-reduced-motion: reduce)") : null;
    this.reduced = options.reducedMotion ?? mq?.matches ?? false;
    if (mq && options.reducedMotion === undefined) {
      const onChange = () => {
        this.reduced = mq.matches;
        if (this.reduced && this.playing) this.pause();
      };
      mq.addEventListener?.("change", onChange);
      this.cleanup.push(() => mq.removeEventListener?.("change", onChange));
    }
    this.mountAccessibility();
    this.createBackend();
    this.observe();
    this.bindInput();
  }

  /** 📡️ Subscribes to an event; returns the unsubscribe function. */
  on<K extends keyof ViewerEvents>(type: K, handler: (event: ViewerEvents[K]) => void): () => void {
    const set = this.listeners.get(type) ?? new Set();
    this.listeners.set(type, set);
    set.add(handler as (e: never) => void);
    return () => set.delete(handler as (e: never) => void);
  }

  private emit<K extends keyof ViewerEvents>(type: K, event: ViewerEvents[K]): void {
    for (const h of this.listeners.get(type) ?? []) (h as (e: ViewerEvents[K]) => void)(event);
  }

  private announce(status: ViewerStatus): void {
    const text = this.options.strings.status?.(status);
    if (text && this.live) this.live.textContent = text;
  }

  private mountAccessibility(): void {
    const c = this.canvas, s = this.options.strings;
    const id = `semio-viewer-${++counter}`;
    c.tabIndex = c.tabIndex >= 0 ? c.tabIndex : 0;
    c.setAttribute("role", "application");
    c.setAttribute("aria-label", s.label);
    c.setAttribute("aria-roledescription", s.roleDescription ?? s.label);
    c.style.touchAction = "none";
    const doc = c.ownerDocument;
    if (c.parentNode && doc) {
      const d = doc.createElement("div");
      d.id = `${id}-description`;
      d.style.cssText = HIDDEN;
      d.textContent = s.description;
      const l = doc.createElement("div");
      l.setAttribute("role", "status");
      l.setAttribute("aria-live", "polite");
      l.style.cssText = HIDDEN;
      c.parentNode.insertBefore(d, c.nextSibling);
      c.parentNode.insertBefore(l, d.nextSibling);
      c.setAttribute("aria-describedby", d.id);
      this.description = d;
      this.live = l;
      this.cleanup.push(() => {
        d.remove();
        l.remove();
      });
    }
  }

  private createBackend(): void {
    const status = (s: BackendStatus) => {
      this.emit("backend", { status: s });
      if (s === "restored") this.requestRender();
    };
    try {
      this.backend = (this.options.backend ?? createWebGl2Backend)(this.canvas, status);
    } catch (error) {
      if (this.options.fallback === false) {
        this.emit("error", { error: error as Error });
        return;
      }
      this.backend = new SoftwareBackend(1, 1);
      this.software = true;
      status("ready");
    }
  }

  private observe(): void {
    const c = this.canvas;
    const measure = () => {
      const r = c.getBoundingClientRect();
      this.cssWidth = Math.max(1, r.width || c.clientWidth || 1);
      this.cssHeight = Math.max(1, r.height || c.clientHeight || 1);
      this.ratio = clamp(globalThis.devicePixelRatio || 1, 1, this.options.maxPixelRatio ?? 3);
      this.backend?.resize(this.cssWidth, this.cssHeight, this.ratio);
      this.requestRender();
    };
    measure();
    if (typeof ResizeObserver === "function") {
      const ro = new ResizeObserver(measure);
      ro.observe(c);
      this.cleanup.push(() => ro.disconnect());
    }
    if (typeof IntersectionObserver === "function") {
      const io = new IntersectionObserver((entries) => {
        this.visible = entries.some((e) => e.isIntersecting);
        if (this.visible) this.requestRender();
      });
      io.observe(c);
      this.cleanup.push(() => io.disconnect());
    }
    const doc = c.ownerDocument;
    if (doc) {
      const onVisibility = () => {
        this.lastNow = -1;
        if (!doc.hidden) this.requestRender();
      };
      doc.addEventListener("visibilitychange", onVisibility);
      this.cleanup.push(() => doc.removeEventListener("visibilitychange", onVisibility));
    }
  }

  private get aspect(): number {
    return this.cssWidth / this.cssHeight;
  }

  private backgroundColor(): V4 {
    if (this.options.background) return this.options.background;
    const text = typeof getComputedStyle === "function" ? getComputedStyle(this.canvas).getPropertyValue("--semio-viewer-background") : "";
    return (text && parseCssColor(text)) || [1, 1, 1, 1];
  }

  /** 📥️ Prepares and shows `document` (resolved, or procedural when `options.quality` is set). Progress goes to the `progress` event; a newer call or {@link cancelPreparation} cancels this one, which then resolves without change. Rejects on invalid documents. */
  async setDocument(document: ModelDocument, signal?: AbortSignal): Promise<void> {
    this.preparation?.abort();
    const controller = new AbortController();
    this.preparation = controller;
    signal?.addEventListener("abort", () => controller.abort(), { once: true });
    const procedural = document.solids.length > 0;
    const part = (from: number, to: number): Operation => ({ report: (f) => this.emit("progress", { fraction: from + (to - from) * f }), cancelled: () => controller.signal.aborted || this.disposed });
    try {
      let resolved = document;
      if (procedural) {
        if (!this.options.quality) throw new Error("procedural document needs the quality option");
        resolved = await tessellate(document, this.options.quality, part(0, 0.5));
      }
      const scene = await sceneOfAsync(resolved, part(procedural ? 0.5 : 0, 1));
      if (controller.signal.aborted || this.disposed) return;
      this.scene = scene;
      this.frame = createFrame(scene);
      this.poseId = null;
      this.animation = null;
      this.timeS = 0;
      this.playing = false;
      this.variant = null;
      this.followState.ready = false;
      this.backend?.load(scene);
      this.fit(false);
      this.emit("ready", { scene });
      this.announce({ kind: "ready" });
    } catch (error) {
      if (error instanceof CancelledError) return;
      this.emit("error", { error: error as Error });
      throw error;
    } finally {
      if (this.preparation === controller) this.preparation = null;
    }
  }

  /** 🛑️ Cancels a running preparation. */
  cancelPreparation(): void {
    this.preparation?.abort();
  }

  /** 🧍️ Shows a pose id (null: rest pose) and stops any animation. */
  setPose(id: Slug | null): void {
    this.requireScene();
    if (id !== null && !this.scene!.document.poses.some((p) => p.id === id)) throw new Error(`unknown pose ${id}`);
    this.pause();
    this.animation = null;
    this.followState.ready = false;
    this.poseId = id;
    this.announce({ kind: "pose", pose: id });
    this.requestRender();
  }

  /** 🎞️ Selects an animation (null: none), at `timeS`; it starts only when `options.autoplay` is set and motion is not reduced. */
  setAnimation(id: Slug | null, timeS = 0): void {
    const scene = this.requireScene();
    this.pause();
    this.poseId = null;
    this.followState.ready = false;
    this.animation = id === null ? null : (scene.document.animations.find((a) => a.id === id) ?? (() => { throw new Error(`unknown animation ${id}`); })());
    this.timeS = timeS;
    this.announce({ kind: "animation", animation: id });
    if (this.animation && this.options.autoplay && !this.reduced) this.play();
    this.requestRender();
  }

  /** ▶️ Starts the selected animation. */
  play(): void {
    if (!this.animation || this.playing) return;
    const a = this.animation;
    if (!(this.loopOverride ?? a.loop)) {
      if (this.rate >= 0 && this.timeS >= a.durationS) this.timeS = 0;
      else if (this.rate < 0 && this.timeS <= 0) this.timeS = a.durationS;
    }
    this.playing = true;
    this.lastNow = -1;
    this.emit("playing", { playing: true });
    this.announce({ kind: "playing" });
    this.requestRender();
  }

  /** ⏸️ Pauses the animation. */
  pause(): void {
    if (!this.playing) return;
    this.playing = false;
    this.emit("playing", { playing: false });
    this.announce({ kind: "paused" });
  }

  /** ⏩️ Jumps to `timeS` seconds. */
  seek(timeS: number): void {
    this.timeS = Math.max(0, timeS);
    this.followState.ready = false;
    if (this.animation) this.emit("time", { animation: this.animation.id, timeS: this.timeS });
    this.requestRender();
  }

  /** 🐢️ Playback rate multiplier; negative plays backwards. */
  setRate(rate: number): void {
    this.rate = rate;
  }

  /** 🔁️ Overrides the clip's loop flag (null restores it). */
  setLoop(loop: boolean | null): void {
    this.loopOverride = loop;
  }

  /** 🎭️ Selects a variant id (null: the base materials). */
  setVariant(id: Slug | null): void {
    const scene = this.requireScene();
    if (id !== null && !scene.variants.has(id)) throw new Error(`unknown variant ${id}`);
    this.variant = id;
    this.announce({ kind: "variant", variant: id });
    this.requestRender();
  }

  /** 📷️ Moves the camera into a preset direction and frames the model. */
  setPreset(preset: CameraPreset): void {
    this.preset = preset;
    this.camera = withPreset(this.camera, preset);
    this.fit(false);
    this.emit("preset", { preset });
    this.announce({ kind: "preset", preset });
  }

  /** 🎯️ Frames the model's current bounds. */
  fit(announce = true): void {
    if (this.scene && this.frame) {
      const f = this.poseFrame();
      this.camera = fitCamera(this.camera, boundsOf(this.scene, f), this.aspect, this.options.fitMargin ?? 1.1);
    }
    if (announce) this.announce({ kind: "fit" });
    this.emit("camera", { camera: this.camera });
    this.requestRender();
  }

  getCamera(): CameraState {
    return this.camera;
  }

  setCamera(camera: CameraState): void {
    this.camera = camera;
    if (this.preset !== null) this.emit("preset", { preset: null });
    this.preset = null;
    this.emit("camera", { camera });
    this.requestRender();
  }

  /** 🧲️ What the orbit target follows (`off` by default); changing it snaps the target onto the new point and emits `follow`. */
  get follow(): ViewerFollow {
    return this.followMode;
  }

  /** 🚶️ Root motion mode of the playback (`travel` by default). */
  get rootMotion(): RootMotion {
    return this.motion;
  }

  /** 🎬️ Sets the follow mode; `off` leaves the camera looking at the last followed point. */
  setFollow(follow: ViewerFollow): void {
    if (this.followMode !== "off" && follow === "off") this.camera = { ...this.camera, target: [...this.camera.target] };
    this.followMode = follow;
    this.followState.ready = false;
    this.emit("follow", { follow });
    this.fit(false);
  }

  /** 🏃️ Sets the root motion mode (see `../🎥️follow/🟦️.ts`) and emits `rootMotion`. */
  setRootMotion(rootMotion: RootMotion): void {
    this.motion = rootMotion;
    this.followState.ready = false;
    this.emit("rootMotion", { rootMotion });
    this.requestRender();
  }

  /** 🔄️ Orbits by yaw/pitch deltas in radians. */
  orbitBy(dYaw: number, dPitch: number): void {
    if (this.controls.orbit) this.setCamera(orbit(this.camera, dYaw, dPitch));
  }

  /** ✋️ Pans by fractions of the visible height (`dx` right, `dy` up). */
  panBy(dx: number, dy: number): void {
    if (this.controls.pan) this.setCamera(pan(this.camera, dx, dy, this.aspect));
  }

  /** 🔎️ Zooms in by a factor above 1. */
  zoomBy(factor: number): void {
    if (this.controls.zoom) this.setCamera(zoom(this.camera, factor));
  }

  /** 🖱️ Picks the surface under a client position. */
  pickAt(clientX: number, clientY: number): Hit | null {
    if (!this.scene || !this.frame) return null;
    const r = this.canvas.getBoundingClientRect();
    const x = ((clientX - r.left) / r.width) * 2 - 1, y = 1 - ((clientY - r.top) / r.height) * 2;
    return pickAt(this.scene, this.poseFrame(), this.camera, this.aspect, x, y);
  }

  /** 🖼️ RGBA bytes of the current picture, top row first (thumbnails). */
  readPixels(): Uint8ClampedArray {
    this.render();
    return this.backend!.readPixels();
  }

  /** 🔧️ The kind of backend in use (`webgl2`, `software`). */
  get backendKind(): string {
    return this.backend?.kind ?? "none";
  }

  /** ⏱️ Current animation id and time. */
  get clock(): { animation: Slug | null; timeS: number; playing: boolean; rate: number } {
    return { animation: this.animation?.id ?? null, timeS: this.timeS, playing: this.playing, rate: this.rate };
  }

  private requireScene(): Scene {
    if (!this.scene) throw new Error("no document set");
    return this.scene;
  }

  private poseFrame(): Frame {
    const a = this.animation;
    const wraps = a ? (this.loopOverride ?? a.loop) : false;
    const pose = a ? { animation: a.id, timeS: !wraps && a.loop ? Math.min(this.timeS, a.durationS * (1 - 1e-12)) : this.timeS } : this.poseId;
    const frame = frameOf(this.scene!, { pose, variant: this.variant, camera: this.camera, aspect: this.aspect, lighting: this.options.lighting, rootMotion: this.motion, rootNode: this.options.rootNode, follow: { mode: this.followMode, state: this.followState, dtS: this.dtS, smoothingS: this.options.followSmoothingS ?? 0.25 } }, this.frame!);
    if (this.followMode !== "off" && this.followState.ready) {
      const t = this.followState.target;
      if (this.camera.target !== t) this.camera = { ...this.camera, target: t };
      if (t[0] !== this.lastTarget[0] || t[1] !== this.lastTarget[1] || t[2] !== this.lastTarget[2]) {
        this.lastTarget[0] = t[0]; this.lastTarget[1] = t[1]; this.lastTarget[2] = t[2];
        this.emit("camera", { camera: this.camera });
      }
    }
    return frame;
  }

  /** 🖌️ Draws one frame now. */
  render(): void {
    if (!this.scene || !this.frame || !this.backend || this.disposed) return;
    const ratio = clamp(globalThis.devicePixelRatio || 1, 1, this.options.maxPixelRatio ?? 3);
    if (ratio !== this.ratio) {
      this.ratio = ratio;
      this.backend.resize(this.cssWidth, this.cssHeight, ratio);
    }
    const f = this.poseFrame();
    if (f.issues.length > 0) this.emit("issues", { issues: f.issues });
    this.backend.render(f, { background: this.backgroundColor() });
    if (this.software && this.backend instanceof SoftwareBackend && typeof ImageData === "function") this.backend.present(this.canvas as never, ImageData as never);
    this.dirty = false;
  }

  /** 🔔️ Schedules a frame. */
  requestRender(): void {
    this.dirty = true;
    if (this.frameRequest || this.disposed || typeof requestAnimationFrame !== "function") return;
    this.frameRequest = requestAnimationFrame((now) => this.tick(now));
  }

  private tick(now: number): void {
    this.frameRequest = 0;
    if (this.disposed) return;
    const hidden = this.canvas.ownerDocument?.hidden ?? false;
    if (hidden || !this.visible) {
      this.lastNow = -1;
      return;
    }
    if (this.playing && this.animation) {
      const dt = this.lastNow >= 0 ? (now - this.lastNow) / 1000 : 0;
      this.lastNow = now;
      const a = this.animation;
      const loop = this.loopOverride ?? a.loop;
      let t = this.timeS + dt * this.rate;
      if (loop) t = ((t % a.durationS) + a.durationS) % a.durationS;
      else if ((this.rate >= 0 && t >= a.durationS) || (this.rate < 0 && t <= 0)) {
        t = clamp(t, 0, a.durationS);
        this.playing = false;
        this.emit("playing", { playing: false });
        this.announce({ kind: "paused" });
      }
      this.timeS = t;
      this.dtS = Math.abs(dt * this.rate);
      this.emit("time", { animation: a.id, timeS: t });
      this.dirty = true;
    }
    if (this.dirty) this.render();
    this.dtS = 0;
    if (this.playing) this.frameRequest = requestAnimationFrame((n) => this.tick(n));
  }

  private bindInput(): void {
    const c = this.canvas;
    const pointers = new Map<number, { x: number; y: number }>();
    let moved = 0, mode: "orbit" | "pan" = "orbit", pinch = 0;
    const listen = <K extends keyof HTMLElementEventMap>(type: K, fn: (e: HTMLElementEventMap[K]) => void, opts?: AddEventListenerOptions) => {
      c.addEventListener(type, fn as EventListener, opts);
      this.cleanup.push(() => c.removeEventListener(type, fn as EventListener, opts));
    };
    const spread = () => {
      const [a, b] = [...pointers.values()];
      return portable.hypot(a!.x - b!.x, a!.y - b!.y);
    };
    listen("pointerdown", (e) => {
      c.setPointerCapture?.(e.pointerId);
      pointers.set(e.pointerId, { x: e.clientX, y: e.clientY });
      moved = 0;
      mode = e.button === 1 || e.button === 2 || e.shiftKey ? "pan" : "orbit";
      if (pointers.size === 2) pinch = spread();
      c.focus?.({ preventScroll: true });
    });
    listen("pointermove", (e) => {
      const p = pointers.get(e.pointerId);
      if (!p) return;
      const dx = e.clientX - p.x, dy = e.clientY - p.y;
      p.x = e.clientX;
      p.y = e.clientY;
      moved += Math.abs(dx) + Math.abs(dy);
      const h = this.cssHeight;
      if (pointers.size >= 2) {
        const s = spread();
        if (pinch > 0 && s > 0) this.zoomBy(portable.pow(s / pinch, this.controls.zoomSpeed));
        pinch = s;
        this.panBy((-dx / pointers.size / h) * this.controls.panSpeed, (dy / pointers.size / h) * this.controls.panSpeed);
        this.announce({ kind: "pan" });
      } else if (mode === "pan") this.panBy((-dx / h) * this.controls.panSpeed, (dy / h) * this.controls.panSpeed);
      else this.orbitBy((-dx / h) * Math.PI * this.controls.rotateSpeed, (dy / h) * Math.PI * this.controls.rotateSpeed);
    });
    const release = (e: PointerEvent) => {
      if (!pointers.delete(e.pointerId)) return;
      pinch = 0;
      if (pointers.size === 0 && moved < 4 && e.type === "pointerup") this.emit("pick", { hit: this.pickAt(e.clientX, e.clientY), clientX: e.clientX, clientY: e.clientY });
    };
    listen("pointerup", release);
    listen("pointercancel", release);
    listen("wheel", (e) => {
      if (!this.controls.zoom) return;
      e.preventDefault();
      this.zoomBy(portable.exp(-e.deltaY * 0.0015 * this.controls.zoomSpeed * (e.deltaMode === 1 ? 16 : 1)));
    }, { passive: false });
    listen("dblclick", () => this.fit());
    listen("contextmenu", (e) => {
      if (this.controls.pan) e.preventDefault();
    });
    listen("keydown", (e) => {
      if (!this.controls.keyboard || e.ctrlKey || e.metaKey || e.altKey) return;
      const k = this.controls.keyboardStep;
      const step = (name: "orbit" | "pan" | "zoom", run: () => void) => {
        e.preventDefault();
        run();
        this.announce({ kind: name });
      };
      switch (e.key) {
        case "ArrowLeft": return e.shiftKey ? step("pan", () => this.panBy(-k * this.controls.panSpeed, 0)) : step("orbit", () => this.orbitBy(-k * Math.PI * this.controls.rotateSpeed, 0));
        case "ArrowRight": return e.shiftKey ? step("pan", () => this.panBy(k * this.controls.panSpeed, 0)) : step("orbit", () => this.orbitBy(k * Math.PI * this.controls.rotateSpeed, 0));
        case "ArrowUp": return e.shiftKey ? step("pan", () => this.panBy(0, k * this.controls.panSpeed)) : step("orbit", () => this.orbitBy(0, k * Math.PI * this.controls.rotateSpeed));
        case "ArrowDown": return e.shiftKey ? step("pan", () => this.panBy(0, -k * this.controls.panSpeed)) : step("orbit", () => this.orbitBy(0, -k * Math.PI * this.controls.rotateSpeed));
        case "+": case "=": case "PageUp": return step("zoom", () => this.zoomBy(1 + k * this.controls.zoomSpeed * 2));
        case "-": case "_": case "PageDown": return step("zoom", () => this.zoomBy(1 / (1 + k * this.controls.zoomSpeed * 2)));
        case "Home": case "0": e.preventDefault(); return this.fit();
        case " ": case "Spacebar":
          if (!this.animation) return;
          e.preventDefault();
          return this.playing ? this.pause() : this.play();
        default:
          if (PRESET_KEYS[e.key]) {
            e.preventDefault();
            this.setPreset(PRESET_KEYS[e.key]!);
          }
      }
    });
  }

  /** 🧹️ Releases the backend, listeners and inserted elements. The canvas stays. */
  dispose(): void {
    this.disposed = true;
    this.preparation?.abort();
    if (this.frameRequest && typeof cancelAnimationFrame === "function") cancelAnimationFrame(this.frameRequest);
    for (const f of this.cleanup) f();
    this.cleanup.length = 0;
    this.backend?.dispose();
    this.backend = null;
    this.listeners.clear();
  }
}
