/** 🎚️ The markdown compiler adapter box, deck mount options, slide-url synchronisation, default surface chrome and the reveal chrome variable sync.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import { ephemeralBox } from "@semio-tech/framework";
// #endregion 🔌️Adapters
import type { Presentation } from "@semio-tech/presentation";
import { formatPresentationUrlHash, parsePresentationSlideHash, presentationLanguage, presentationSlideAt } from "@semio-tech/presentation";
import { DEFAULT_UI_DRIVER, type ElementsSurfaceChromeInput } from "@semio-tech/ui-react";
import Reveal from "reveal.js";
import { compileOwnedMarkdownToHtml } from "../📝️markdown-html-compiler/🟦️.ts";

// #region 🔖️Markdown
/** 📝️ Compiles markdown source into an HTML fragment. */
export interface MarkdownHtmlCompiler {
  compile(markdown: string): Promise<string>;
}

const defaultMarkdownHtmlCompiler: MarkdownHtmlCompiler = {
  compile: compileOwnedMarkdownToHtml,
};

const markdownHtmlCompiler = ephemeralBox<MarkdownHtmlCompiler>("framework.products.presentation.targets.react.markdownHtmlCompiler", defaultMarkdownHtmlCompiler);

/** 🔌️ Replaces the markdown HTML compiler (tests or alternate renderers). */
export function setMarkdownHtmlCompiler(compiler: MarkdownHtmlCompiler): void {
  markdownHtmlCompiler.current = compiler;
}

/** 📝️ Compiles markdown through the active {@link MarkdownHtmlCompiler}. */
export function compileMarkdownToHtml(markdown: string): Promise<string> {
  return markdownHtmlCompiler.current.compile(markdown);
}
// #endregion 🔖️Markdown

//#region 🔖️MountOptions
/** ⚙️ Reveal.js and @semio-tech/ui-react surface chrome options for {@link mountPresentation}. */
export interface PresentationMountOptions {
  readonly surfaceChrome?: ElementsSurfaceChromeInput | false;
  readonly transition?: "fade" | "slide" | "convex" | "concave" | "zoom" | "none";
  /** 🔗️ Sync slide position to the URL hash; defaults to true. */
  readonly hash?: boolean;
  readonly slideNumber?: boolean;
  readonly width?: number;
  readonly height?: number;
  /** 🎞️ Called once reveal.js finished initializing (tests and tooling). */
  readonly onRevealReady?: (api: Reveal.Api) => void;
}

/** 🔗️ Writes reveal.js hash with localized bookmark params after the path; bookmark params are ignored for navigation. */
export function syncPresentationSlideUrl(presentation: Presentation, indices: { readonly h: number; readonly v: number }): void {
  if (typeof window === "undefined") {
    return;
  }
  const slide = presentationSlideAt(presentation, indices);
  if (!slide) {
    return;
  }
  const url = new URL(window.location.href);
  url.search = "";
  url.hash = formatPresentationUrlHash(indices, slide, presentationLanguage(presentation));
  history.replaceState(null, "", url);
}

/** 🔗️ Reads reveal.js slide indices from the URL hash; trailing bookmark query params are ignored. */
export function readPresentationSlideIndicesFromUrl(hash: string = typeof window !== "undefined" ? window.location.hash : ""): { readonly h: number; readonly v: number } | null {
  return parsePresentationSlideHash(hash);
}
//#endregion 🔖️MountOptions

export const DEFAULT_SURFACE_CHROME: ElementsSurfaceChromeInput = {
  appearance: "system",
  device: "desktop",
  driver: DEFAULT_UI_DRIVER,
};

//#region 🔖️RevealChrome
/** 📐️ Writes reveal slide dimensions as CSS variables for positioned arrangement canvases. */
export function syncPresentationSlideSizeVars(deckEl: HTMLElement | null, deck: Reveal.Api | null): void {
  if (!deckEl || !deck) {
    return;
  }
  const size = deck.getComputedSlideSize();
  deckEl.style.setProperty("--presentation-slide-width", `${size.width}px`);
  deckEl.style.setProperty("--presentation-slide-height", `${size.height}px`);
  syncPresentationAutoAnimateDurationVar(deckEl, deck);
}

/** ⏱️ Syncs reveal auto-animate duration for morph ghost/target opacity fades. */
export function syncPresentationAutoAnimateDurationVar(deckEl: HTMLElement, deck: Reveal.Api | null): void {
  const durationSeconds = typeof deck?.getConfig().autoAnimateDuration === "number" ? deck.getConfig().autoAnimateDuration : 1;
  deckEl.style.setProperty("--presentation-auto-animate-duration", `${durationSeconds}s`);
}

/** 🌓️ Align reveal `has-dark-background` with `html.dark` from system chrome. */
export function syncRevealBackgroundKind(deckEl: HTMLElement | null): void {
  if (!deckEl || typeof document === "undefined") {
    return;
  }
  const dark = document.documentElement.classList.contains("dark");
  deckEl.classList.toggle("has-dark-background", dark);
  deckEl.classList.toggle("has-light-background", !dark);
}
//#endregion 🔖️RevealChrome
