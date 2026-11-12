/** 🧩️ `defineModelViewerElement`: registers a custom element that hosts a {@link ModelViewer} in a shadow root. The client must set `strings` (there is no default language) before connecting, then `model`.
 *
 * Attributes `preset`, `variant`, `animation` (id), `playing` (boolean), `follow` (`off`, `bounds` or `node:<node id>`) and `root-motion` (`travel` or `in-place`) mirror the controller. The style sheet is a constructable one adopted by the shadow root (so a Content-Security-Policy without `style-src 'unsafe-inline'` does not block it), with a `<style>` element only where adoption is unsupported. CSS custom properties on the element: `--semio-viewer-background` (picture and host background), `--semio-viewer-focus` (focus ring colour), `--semio-viewer-min-height`.
 * Events of the controller are re-dispatched as `CustomEvent`s named `semio-<event>` (`semio-ready`, `semio-progress`, `semio-camera`, `semio-time`, `semio-playing`, `semio-pick`, `semio-error`, `semio-backend`, `semio-issues`, `semio-preset`, `semio-follow`, `semio-rootMotion`) with the payload in `detail`.
 * @see ../🕹️controller/🟦️.ts */
import type { ModelDocument } from "../../🧬️schema/🟦️.ts";
import type { CameraPreset } from "../📷️camera/🟦️.ts";
import type { RootMotion, ViewerFollow } from "../🎥️follow/🟦️.ts";
import { ModelViewer, type ModelViewerOptions, type ViewerEvents, type ViewerStrings } from "../🕹️controller/🟦️.ts";

const STYLE = `:host{display:block;position:relative;background:var(--semio-viewer-background,transparent);min-height:var(--semio-viewer-min-height,12rem)}canvas{display:block;width:100%;height:100%;outline:none}canvas:focus-visible{outline:2px solid var(--semio-viewer-focus,currentColor);outline-offset:-2px}`;
const EVENTS: (keyof ViewerEvents)[] = ["ready", "progress", "camera", "time", "playing", "pick", "error", "backend", "issues", "preset", "follow", "rootMotion"];

/** 🏷️ Registers `tagName` (once) and returns its class. */
export function defineModelViewerElement(tagName = "semio-model-viewer", registry: CustomElementRegistry = customElements): CustomElementConstructor {
  const existing = registry.get(tagName);
  if (existing) return existing;
  class ModelViewerElement extends HTMLElement {
    static observedAttributes = ["preset", "variant", "animation", "playing", "follow", "root-motion"];
    strings: ViewerStrings | null = null;
    options: Omit<ModelViewerOptions, "canvas" | "strings"> = {};
    viewer: ModelViewer | null = null;
    private pending: ModelDocument | null = null;
    private readonly canvas: HTMLCanvasElement;

    constructor() {
      super();
      const root = this.attachShadow({ mode: "open" });
      this.canvas = document.createElement("canvas");
      if (typeof CSSStyleSheet === "function" && "adoptedStyleSheets" in root) {
        try {
          const sheet = new CSSStyleSheet();
          sheet.replaceSync(STYLE);
          root.adoptedStyleSheets = [sheet];
          root.append(this.canvas);
          return;
        } catch {
          root.adoptedStyleSheets = [];
        }
      }
      const style = document.createElement("style");
      style.textContent = STYLE;
      root.append(style, this.canvas);
    }

    set model(document: ModelDocument | null) {
      this.pending = document;
      if (document && this.viewer) void this.viewer.setDocument(document).catch(() => undefined);
    }

    get model(): ModelDocument | null {
      return this.pending;
    }

    connectedCallback(): void {
      if (this.viewer) return;
      if (!this.strings) throw new Error("semio-model-viewer: set `strings` before connecting; the viewer has no default language");
      const viewer = new ModelViewer({ ...this.options, canvas: this.canvas, strings: this.strings });
      this.viewer = viewer;
      viewer.on("ready", () => this.sync());
      for (const name of EVENTS) viewer.on(name, (detail) => this.dispatchEvent(new CustomEvent(`semio-${name}`, { detail })));
      if (this.pending) this.model = this.pending;
    }

    disconnectedCallback(): void {
      this.viewer?.dispose();
      this.viewer = null;
    }

    attributeChangedCallback(): void {
      this.sync();
    }

    private sync(): void {
      const v = this.viewer;
      if (!v) return;
      try {
        const preset = this.getAttribute("preset");
        if (preset) v.setPreset(preset as CameraPreset);
        if (this.hasAttribute("variant")) v.setVariant(this.getAttribute("variant") || null);
        const follow = this.getAttribute("follow");
        const wanted: ViewerFollow = !follow || follow === "off" ? "off" : follow === "bounds" ? "bounds" : { node: follow.replace(/^node:/u, "") };
        if (JSON.stringify(wanted) !== JSON.stringify(v.follow)) v.setFollow(wanted);
        const motion = this.getAttribute("root-motion");
        if (motion && motion !== v.rootMotion) v.setRootMotion(motion as RootMotion);
        const animation = this.getAttribute("animation");
        if (animation && v.clock.animation !== animation) v.setAnimation(animation);
        if (animation) this.hasAttribute("playing") ? v.play() : v.pause();
      } catch {
        return;
      }
    }
  }
  registry.define(tagName, ModelViewerElement);
  return ModelViewerElement;
}
