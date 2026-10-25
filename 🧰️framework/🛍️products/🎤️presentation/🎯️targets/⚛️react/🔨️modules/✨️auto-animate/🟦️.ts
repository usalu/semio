/** ✨️ Reveal auto-animate integration: settled-state sync, many-to-one morph classes, ghost anchors, the auto-animate matcher, style-sheet patches and the hidden-slide preflight.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

import type { DispositionPosition } from "@semio-tech/presentation";
import { applyFigureCropCssVars, applyMorphFrameCssVars, figureCropBackgroundVarsTargetGhost, readPercentDispositionFrame } from "../📐️figure-frames/🟦️.ts";
import { isUsableMeasuredRect, measureElementRectInSection, slideLayoutBounds, tightElementBoundsRect } from "../🧮️interaction-model/🟦️.ts";

//#region 🔖️ArrangementSettled
/** 🔗️ True when two slide sections share the same reveal.js auto-animate run id. */
export function slidesShareAutoAnimateId(fromSlide: HTMLElement | null | undefined, toSlide: HTMLElement | null | undefined): boolean {
  const fromId = fromSlide?.getAttribute("data-auto-animate-id");
  const toId = toSlide?.getAttribute("data-auto-animate-id");
  return fromId !== null && fromId !== undefined && fromId === toId;
}

/** ⏳️ True while reveal.js auto-animate is pending or running on one section. */
export function isSectionAutoAnimating(sectionEl: HTMLElement): boolean {
  const state = sectionEl.getAttribute("data-auto-animate");
  return state === "pending" || state === "running";
}

/** ▶️ True while reveal.js auto-animate is measuring or running on this slide (or any slide in the deck). */
export function isRevealSlideAutoAnimating(sectionEl: HTMLElement): boolean {
  if (isSectionAutoAnimating(sectionEl)) {
    return true;
  }
  const deck = sectionEl.closest(".reveal");
  if (!(deck instanceof HTMLElement)) {
    return false;
  }
  return deck.querySelector('section[data-auto-animate="pending"], section[data-auto-animate="running"]') !== null;
}

/** 🔄️ Clears settled state on arrival so split tiles stay visible at rest; drops settled on slides no longer adjacent. */
export function syncArrangementSettledState(deckEl: HTMLElement, currentSlide: HTMLElement | null, previousSlide: HTMLElement | null): void {
  const settleSections = deckEl.querySelectorAll<HTMLElement>("section[data-settle-before-morph-to]");
  for (const section of settleSections) {
    if (section !== currentSlide && section !== previousSlide) {
      section.classList.remove("presentation-arrangement--settled");
    }
  }
  if (currentSlide?.hasAttribute("data-settle-before-morph-to")) {
    currentSlide.classList.remove("presentation-arrangement--settled");
  }
  if (currentSlide?.classList.contains("presentation-arrangement--settled") && currentSlide.getAttribute("data-auto-animate") !== "pending" && currentSlide.getAttribute("data-auto-animate") !== "running") {
    currentSlide.classList.remove("presentation-arrangement--settled");
  }
}

/** 📑️ Resolves the reveal.js slide section at stack indices `h` / `v`. */
export function resolveRevealSlideAt(deckEl: HTMLElement, indices: { readonly h: number; readonly v: number }): HTMLElement | null {
  const horizontal = deckEl.querySelectorAll<HTMLElement>(".slides > section")[indices.h];
  if (!horizontal) {
    return null;
  }
  const vertical = horizontal.querySelectorAll<HTMLElement>("section");
  return vertical[indices.v] ?? horizontal;
}

/** 📖️ Reads reveal.js `slidechanged` slide elements extended onto the event object. */
export function slideChangedEventSlides(event: Event): {
  readonly previousSlide: HTMLElement | undefined;
  readonly currentSlide: HTMLElement | undefined;
} {
  const slideEvent = event as Event & {
    readonly previousSlide?: HTMLElement;
    readonly currentSlide?: HTMLElement;
  };
  return {
    previousSlide: slideEvent.previousSlide,
    currentSlide: slideEvent.currentSlide,
  };
}

/** 🧹️ Strips reveal.js FLIP `transform`/`transition` only; never `left`/`top`/`width`/`height` (React owns those on morph frames). */
export function clearRevealAutoAnimateInlineLayout(deckEl: HTMLElement): void {
  const flipProps = ["transform", "transition"] as const;
  const selectors = ["[data-auto-animate-target]"];
  for (const selector of selectors) {
    for (const element of deckEl.querySelectorAll<HTMLElement>(selector)) {
      for (const prop of flipProps) {
        element.style.removeProperty(prop);
      }
    }
  }
}

/** ✅️ Clears reveal `pending`/`running` on slides so morph-source/into rest CSS applies after FLIP completes. */
export function finalizeRevealAutoAnimateRestState(deckEl: HTMLElement): void {
  for (const slide of deckEl.querySelectorAll<HTMLElement>('section[data-auto-animate="running"], section[data-auto-animate="pending"]')) {
    slide.setAttribute("data-auto-animate", "");
  }
  clearManyToOneMorphArrangementClass(deckEl);
  const presentSlide = deckEl.querySelector<HTMLElement>("section.present");
  const introFlowPresent = presentSlide !== null && isIntroFlowSlide(presentSlide);
  if (!introFlowPresent) {
    clearRevealAutoAnimateInlineLayout(deckEl);
    for (const element of deckEl.querySelectorAll<HTMLElement>("[data-auto-animate-target]")) {
      delete element.dataset.autoAnimateTarget;
    }
  }
  if (presentSlide?.classList.contains("presentation-arrangement--settled") && presentSlide.getAttribute("data-auto-animate") !== "pending" && presentSlide.getAttribute("data-auto-animate") !== "running") {
    presentSlide.classList.remove("presentation-arrangement--settled");
  }
}

/** 🔀️ Slide class while auto-animating a `data-settle-before-morph-to` many-to-one run (focus tiles → label ghosts). */
export const PRESENTATION_MANY_TO_ONE_MORPH_CLASS = "presentation-arrangement--many-to-one-morph";

/** 📊️ True when `fromSlide` settles into `toSlide` via `data-settle-before-morph-to` (arrangement id on `title`). */
export function isManyToOneMorphTransition(fromSlide: HTMLElement, toSlide: HTMLElement): boolean {
  const settleBefore = fromSlide.getAttribute("data-settle-before-morph-to");
  if (!settleBefore) {
    return false;
  }
  const toId = toSlide.getAttribute("title");
  if (!toId) {
    return false;
  }
  return settleBefore.split(",").some((entry) => entry.trim() === toId);
}

/** 🗑️ Clears {@link PRESENTATION_MANY_TO_ONE_MORPH_CLASS} from all slides in the deck. */
export function clearManyToOneMorphArrangementClass(deckEl: HTMLElement): void {
  for (const slide of deckEl.querySelectorAll<HTMLElement>(`section.${PRESENTATION_MANY_TO_ONE_MORPH_CLASS}`)) {
    slide.classList.remove(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
  }
}

/** ❎️ Marks the active many-to-one morph run on `fromSlide` and `toSlide` only. */
export function syncManyToOneMorphArrangementClass(deckEl: HTMLElement, fromSlide: HTMLElement | null, toSlide: HTMLElement | null): void {
  clearManyToOneMorphArrangementClass(deckEl);
  if (fromSlide && toSlide && isManyToOneMorphTransition(fromSlide, toSlide)) {
    fromSlide.classList.add(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
    toSlide.classList.add(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
  }
}

/** 🎞️ Prepares settle + many-to-one frame/crop morph before reveal auto-animate measures FLIP. */
export function prepareArrangementBeforeAutoAnimate(fromSlide: HTMLElement, toSlide: HTMLElement): void {
  const deckEl = fromSlide.closest(".reveal");
  if (deckEl instanceof HTMLElement) {
    syncManyToOneMorphArrangementClass(deckEl, fromSlide, toSlide);
  }
  if (!isManyToOneMorphTransition(fromSlide, toSlide)) {
    return;
  }
  fromSlide.classList.add("presentation-arrangement--settled");
  void fromSlide.offsetHeight;
  void toSlide.offsetHeight;
  syncManyToOneGhostMorphFramesFromDom(fromSlide, toSlide);
}

export type AutoAnimateMatcherHost = {
  findAutoAnimateMatches: (
    pairs: { from: HTMLElement; to: HTMLElement; options?: Record<string, unknown> }[],
    fromScope: HTMLElement,
    toScope: HTMLElement,
    selector: string,
    serializer: (node: HTMLElement) => string,
    animationOptions?: Record<string, unknown>,
  ) => void;
};

export function elementIsTargetGhostAnchor(element: HTMLElement): boolean {
  return element.classList.contains("presentation-target-ghost") || element.closest(".presentation-target-ghost") !== null;
}

export function elementIsSourceGhostAnchor(element: HTMLElement): boolean {
  return element.classList.contains("presentation-source-ghost") || element.closest(".presentation-source-ghost") !== null;
}

function elementIsMorphOneAnchor(element: HTMLElement): boolean {
  return element.classList.contains("presentation-morph-one") || element.closest(".presentation-morph-one") !== null;
}

function elementIsLabelMorphSource(element: HTMLElement): boolean {
  return element.classList.contains("presentation-affiliation-morph-source");
}

/** 🥢️ True when this node may be a reveal.js auto-animate pair endpoint (intro wrapper or canvas-framed tile). */
export function isRevealAutoAnimatePairSource(element: HTMLElement): boolean {
  if (!element.hasAttribute("data-id")) {
    return false;
  }
  if (element.classList.contains("presentation-interactive-disposition")) {
    return element.classList.contains("presentation-interactive-disposition--canvas-framed");
  }
  if (element.closest(".presentation-interactive-disposition") !== null) {
    return element.matches("h1, h2, h3, h4, h5, h6, p, img, video");
  }
  return true;
}

/** 📏️ Slide-local ink box for reveal.js `measure` (avoids viewport `getBoundingClientRect` fly-in when `center: true`). */
export function revealInkMeasureForAutoAnimate(element: HTMLElement): {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
} {
  const slide = element.closest("section.presentation-arrangement--interactive, section[data-auto-animate]");
  if (!(slide instanceof HTMLElement)) {
    const rect = element.getBoundingClientRect();
    return { x: rect.left, y: rect.top, width: rect.width, height: rect.height };
  }
  const slideBounds = slideLayoutBounds(slide);
  const ink = tightElementBoundsRect(element) ?? element.getBoundingClientRect();
  return {
    x: ink.left - slideBounds.left,
    y: ink.top - slideBounds.top,
    width: ink.width,
    height: ink.height,
  };
}

function isIntroFlowSlide(slide: HTMLElement): boolean {
  return slide.classList.contains("presentation-arrangement--intro") && !slide.classList.contains("presentation-arrangement--positioned");
}

/** 📄️ reveal.js auto-animate options for canvas morph text (translate only). */
export function revealTextAutoAnimatePairOptions(_from: HTMLElement, _fromSlide: HTMLElement, _toSlide: HTMLElement): Record<string, unknown> {
  return {
    scale: false,
    measure: revealInkMeasureForAutoAnimate,
  };
}

function elementIsFigureMorphSlot(element: HTMLElement): boolean {
  return element.classList.contains("presentation-morph-slot--figure") || element.closest(".presentation-morph-slot--figure") !== null || element.querySelector(".presentation-morph-slot--figure") !== null;
}

export function elementIsInteractiveFigureDisposition(element: HTMLElement): boolean {
  return element.classList.contains("presentation-interactive-disposition") && elementIsFigureMorphSlot(element) && !elementIsSourceGhostAnchor(element) && !elementIsTargetGhostAnchor(element);
}

/** 🎯️ Picks the `to` morph anchor for one `data-id` (focus tile → label target ghost, catalogue source → focus tile). */
export function resolveMorphAutoAnimateTo(fromElement: HTMLElement, toSlide: HTMLElement): HTMLElement | null {
  const id = fromElement.getAttribute("data-id");
  if (!id) {
    return null;
  }
  const escapedId = id.replace(/\\/g, "\\\\").replace(/"/g, '\\"');
  let candidates = [...toSlide.querySelectorAll<HTMLElement>(`[data-id="${escapedId}"]`)];
  if (candidates.length === 0) {
    const baseId = id.replace(/--\d+$/, "");
    if (baseId !== id) {
      candidates = [...toSlide.querySelectorAll<HTMLElement>(`[data-id="${baseId.replace(/\\/g, "\\\\").replace(/"/g, '\\"')}"]`)];
    } else {
      candidates = [...toSlide.querySelectorAll<HTMLElement>(`[data-id="${escapedId}--0"]`)];
    }
  }
  if (candidates.length === 0) {
    return null;
  }
  if (elementIsInteractiveFigureDisposition(fromElement)) {
    const targetGhost = candidates.find((candidate) => candidate.classList.contains("presentation-interactive-disposition") && elementIsTargetGhostAnchor(candidate));
    if (targetGhost) {
      return targetGhost;
    }
  }
  if (elementIsSourceGhostAnchor(fromElement)) {
    const focusTile = candidates.find((candidate) => elementIsInteractiveFigureDisposition(candidate));
    if (focusTile) {
      return focusTile;
    }
  }
  const visible = candidates.find((candidate) => !elementIsLabelMorphSource(candidate));
  if (visible) {
    return visible;
  }
  return candidates.find((candidate) => !elementIsSourceGhostAnchor(candidate) || (elementIsTargetGhostAnchor(candidate) && candidate.classList.contains("presentation-interactive-disposition"))) ?? null;
}

/** 🪄️ reveal.js auto-animate matcher: intro uses stock `data-id` pairing; catalogue uses ghost-aware pairing. */
export function presentationAutoAnimateMatcher(this: AutoAnimateMatcherHost, fromSlide: HTMLElement, toSlide: HTMLElement): { from: HTMLElement; to: HTMLElement; options?: Record<string, unknown> }[] {
  if (isIntroFlowSlide(fromSlide) && isIntroFlowSlide(toSlide)) {
    const pairs: { from: HTMLElement; to: HTMLElement; options?: Record<string, unknown> }[] = [];
    this.findAutoAnimateMatches(pairs, fromSlide, toSlide, "[data-id]", (node) => {
      return `${node.nodeName}:::${node.getAttribute("data-id")}`;
    });
    const reserved: HTMLElement[] = [];
    return pairs.filter((pair) => {
      if (reserved.includes(pair.to)) {
        return false;
      }
      reserved.push(pair.to);
      return true;
    });
  }

  const pairs: { from: HTMLElement; to: HTMLElement; options?: Record<string, unknown> }[] = [];
  for (const fromElement of fromSlide.querySelectorAll<HTMLElement>("[data-id]")) {
    if (!isRevealAutoAnimatePairSource(fromElement)) {
      continue;
    }
    const toElement = resolveMorphAutoAnimateTo(fromElement, toSlide);
    if (toElement && isRevealAutoAnimatePairSource(toElement)) {
      const isTextLeaf = fromElement.matches("h1, h2, h3, h4, h5, h6, p");
      const options = isTextLeaf ? revealTextAutoAnimatePairOptions(fromElement, fromSlide, toSlide) : undefined;
      pairs.push({ from: fromElement, to: toElement, ...(options ? { options } : {}) });
    }
  }
  const reserved: HTMLElement[] = [];
  return pairs.filter((pair) => {
    if (reserved.includes(pair.to)) {
      return false;
    }
    if (elementIsFigureMorphSlot(pair.from) && pair.to.closest(".presentation-morph-target") !== null && !elementIsTargetGhostAnchor(pair.to)) {
      return false;
    }
    if (elementIsTargetGhostAnchor(pair.to) && !elementIsFigureMorphSlot(pair.from)) {
      return false;
    }
    if (elementIsSourceGhostAnchor(pair.to)) {
      return false;
    }
    if (elementIsMorphOneAnchor(pair.from)) {
      return false;
    }
    if (elementIsSourceGhostAnchor(pair.from) && !elementIsInteractiveFigureDisposition(pair.to)) {
      return false;
    }
    if (elementIsFigureMorphSlot(pair.from) && elementIsMorphOneAnchor(pair.to)) {
      return false;
    }
    if (elementIsLabelMorphSource(pair.from) && !elementIsLabelMorphSource(pair.to)) {
      return false;
    }
    reserved.push(pair.to);
    return true;
  });
}

/** 📐️ Rewrites reveal FLIP `scale(sx, sy)` to `scale(max(sx,sy))` so figure tiles zoom instead of squashing during auto-animate. */
export function patchAutoAnimateUniformScale(css: string): string {
  return css.replace(/scale\(([\d.]+),\s*([\d.]+)\)/g, (_match, scaleX: string, scaleY: string) => {
    const sx = Number.parseFloat(scaleX);
    const sy = Number.parseFloat(scaleY);
    if (!Number.isFinite(sx) || !Number.isFinite(sy)) {
      return `scale(${scaleX}, ${scaleY})`;
    }
    return `scale(${Math.max(sx, sy)})`;
  });
}

/** 👻️ Figure ghost visibility during reveal auto-animate (FLIP only; no intro text keyframes). */
export function presentationMorphGhostAutoAnimateCss(durationSeconds: number): string {
  const duration = `${durationSeconds}s`;
  return `
.reveal .slides section[data-auto-animate="pending"] .presentation-target-ghost[data-auto-animate-target] {
	opacity: 1 !important;
	visibility: visible !important;
	animation: none !important;
}
.reveal .slides section[data-auto-animate="running"] .presentation-target-ghost[data-auto-animate-target] {
	visibility: visible !important;
	animation: presentation-target-ghost-fade-out ${duration} ease forwards !important;
}
.reveal .slides section.presentation-arrangement--many-to-one-morph[data-auto-animate="pending"] .presentation-target-ghost.presentation-interactive-disposition--canvas-framed {
	left: var(--presentation-morph-frame-left) !important;
	top: var(--presentation-morph-frame-top) !important;
	width: var(--presentation-morph-frame-width) !important;
	height: var(--presentation-morph-frame-height) !important;
	transform: none !important;
	transition: none !important;
}
.reveal .slides section.presentation-arrangement--many-to-one-morph[data-auto-animate="running"] .presentation-target-ghost.presentation-interactive-disposition--canvas-framed[data-auto-animate-target] {
	visibility: visible !important;
	transform: none !important;
	transition: none !important;
	animation: presentation-target-ghost-frame ${duration} ease forwards,
		presentation-target-ghost-fade-out ${duration} ease forwards !important;
}
.reveal .slides section[data-auto-animate="pending"] .presentation-morph-one[data-auto-animate-target],
.reveal .slides section[data-auto-animate="running"] .presentation-morph-one[data-auto-animate-target] {
	animation: presentation-morph-one-fade-out ${duration} ease forwards !important;
}
`;
}

/** 🩹️ Patches reveal auto-animate sheet: uniform scale for catalogue tiles only; intro text keeps native FLIP. */
export function patchPresentationAutoAnimateStyleSheet(sheet: { innerHTML: string }, durationSeconds: number, options?: { readonly manyToOneMorph?: boolean; readonly introFlowMorph?: boolean }): void {
  let css = sheet.innerHTML;
  if (options?.manyToOneMorph !== true && options?.introFlowMorph !== true) {
    css = patchAutoAnimateUniformScale(css);
  }
  sheet.innerHTML = css + presentationMorphGhostAutoAnimateCss(durationSeconds);
}

/** 💊️ Applies reveal auto-animate sheet fixes for the current slide pair. */
export function patchPresentationAutoAnimateRunStyleSheet(sheet: { innerHTML: string }, durationSeconds: number, fromSlide: HTMLElement | undefined, toSlide: HTMLElement | undefined): void {
  if (fromSlide !== undefined && toSlide !== undefined && isIntroFlowSlide(fromSlide) && isIntroFlowSlide(toSlide)) {
    return;
  }
  patchPresentationAutoAnimateStyleSheet(sheet, durationSeconds, {
    manyToOneMorph: fromSlide !== undefined && toSlide !== undefined && isManyToOneMorphTransition(fromSlide, toSlide),
  });
}

export interface PresentationAutoAnimateRunSlides {
  readonly fromSlide?: HTMLElement;
  readonly toSlide?: HTMLElement;
}

/** 🔎️ Resolves an auto-animate slide pair when reveal.js omits it from the `autoanimate` event. */
export function resolvePresentationAutoAnimateRunSlides(explicit: PresentationAutoAnimateRunSlides, pending: PresentationAutoAnimateRunSlides | undefined): PresentationAutoAnimateRunSlides {
  if (explicit.fromSlide !== undefined && explicit.toSlide !== undefined) {
    return explicit;
  }
  if (pending?.fromSlide !== undefined && pending.toSlide !== undefined) {
    return pending;
  }
  return {
    fromSlide: explicit.fromSlide ?? pending?.fromSlide,
    toSlide: explicit.toSlide ?? pending?.toSlide,
  };
}
//#endregion 🔖️ArrangementSettled

//#region 🔖️HiddenPreflight
/**
 * 👁️ Lets reveal.js own slide visibility by relaxing Tailwind preflight's `[hidden]` reset.
 *
 * `@semio-tech/ui-react` surface chrome ships Tailwind v4 preflight, whose layered
 * `[hidden]{display:none!important}` outranks reveal.js's inline `display:block` on the off-screen
 * slides it briefly un-hides to measure auto-animate `from`/`to` rects. The collapsed measurement
 * makes morph elements fly in from the deck origin instead of morphing in place. Dropping only the
 * `display` declaration restores the standard, non-important UA `[hidden]{display:none}` (so ordinary
 * hidden elements stay hidden) while reveal's inline `display` again wins for slides — giving native
 * reveal.js auto-animate exactly like {@link https://revealjs.com/auto-animate/}.
 */
export function relaxHiddenPreflight(): void {
  if (typeof document === "undefined") {
    return;
  }
  const visit = (rules: CSSRuleList): void => {
    for (const rule of Array.from(rules)) {
      const styleRule = rule as CSSStyleRule;
      if (typeof styleRule.selectorText === "string" && styleRule.selectorText.includes("[hidden]") && styleRule.style?.getPropertyValue("display") === "none") {
        styleRule.style.removeProperty("display");
      }
      const grouping = rule as CSSGroupingRule;
      if (grouping.cssRules) {
        visit(grouping.cssRules);
      }
    }
  };
  const adopted = (document as unknown as { adoptedStyleSheets?: CSSStyleSheet[] }).adoptedStyleSheets ?? [];
  for (const sheet of [...Array.from(document.styleSheets), ...adopted]) {
    try {
      visit(sheet.cssRules);
    } catch {}
  }
}
//#endregion 🔖️HiddenPreflight

//#region 🔖️MorphView
/** 🀄️ Copies live focus-tile frames from slide 8 onto label-slide target ghosts before many-to-one FLIP. */
export function syncManyToOneGhostMorphFramesFromDom(fromSlide: HTMLElement, toSlide: HTMLElement): void {
  if (!isManyToOneMorphTransition(fromSlide, toSlide)) {
    return;
  }
  for (const ghost of toSlide.querySelectorAll<HTMLElement>(".presentation-target-ghost.presentation-interactive-disposition--canvas-framed")) {
    const morphId = ghost.getAttribute("data-id");
    if (!morphId) {
      continue;
    }
    const escapedId = morphId.replace(/\\/g, "\\\\").replace(/"/g, '\\"');
    const sourceEl = fromSlide.querySelector<HTMLElement>(`.presentation-interactive-disposition--canvas-framed[data-id="${escapedId}"]`);
    if (!sourceEl) {
      continue;
    }
    const sourceFrame = measureElementRectInSection(sourceEl, fromSlide) ?? readPercentDispositionFrame(sourceEl);
    if (!sourceFrame || !isUsableMeasuredRect(sourceFrame)) {
      continue;
    }
    const labelFrame = readPercentDispositionFrame(ghost) ?? measureElementRectInSection(ghost, toSlide);
    if (!labelFrame || !isUsableMeasuredRect(labelFrame)) {
      continue;
    }
    applyMorphFrameCssVars(ghost, sourceFrame, labelFrame);
    const slot = ghost.querySelector<HTMLElement>(".presentation-morph-slot--figure");
    const cropRaw = slot?.dataset.presentationMorphCrop;
    if (slot && cropRaw) {
      try {
        const crop = JSON.parse(cropRaw) as DispositionPosition;
        applyFigureCropCssVars(slot, figureCropBackgroundVarsTargetGhost({ kind: "figure", id: "", src: "", crop }, crop, sourceFrame, labelFrame));
      } catch {}
    }
    void ghost.offsetHeight;
  }
}
//#endregion 🔖️MorphView
