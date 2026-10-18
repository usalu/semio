import type { IntroductionDefinition, IntroductionLogo, IntroductionStepDefinition, IntroductionDemonstration } from "@semio-tech/framework";
import { reactHostPort } from "../../../🧱️elements/🔌️Ports/🟦️.tsx";
import * as React from "react";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { useShellScopeOptional, useIsActiveShellRoot, NULL_SHELL_ROOT_REF } from "../../../🧱️elements/🐚️ShellScope/🟦️.tsx";
import { useUiTerminology, resolveUiLocalizedText } from "../../../🔨️modules/🗣️ui-terminology/🟦️.tsx";
import { detectShellLocale, uiI18n } from "../🌐️i18n/🟦️.ts";
import { elementIdSelector } from "../../../🧱️elements/🆔️ElementId/🟦️.tsx";
import { useSurfaceActive } from "../../../🧱️elements/🌈️Surface/🟦️.tsx";
import { availableViewportHeightPx } from "../../../📱️device/🟦️.ts";
import { useLabel } from "../../../🧱️elements/🏷️Label/🟦️.tsx";
import { usePointerDrag } from "../../../🧱️elements/🖼️Panel/🟦️.tsx";
import { useControlKeybinding } from "../../../🔨️modules/🕹️control-keybinding-context/🟦️.tsx";
import { veilClass } from "../../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { WindowChrome, windowChromeTitleChipClass } from "../../../🧱️elements/🗂️WindowChrome/🟦️.tsx";
import { DragHandle } from "../../../🧱️elements/🧱️DragHandle/🟦️.tsx";
import { CloseIcon, Icon } from "../../../🧱️elements/🔣️Icons/🟦️.tsx";
import { Button } from "../../../🧱️elements/🔘️Button/🟦️.tsx";
import { createPortal } from "react-dom";
import { type IntroductionInfoBoxPosition, clampIntroductionInfoBoxPosition, introductionRectRelativeToHost, resolveIntroductionPlacement, splitIntroductionBodyParagraphs, useIntroductionAnchorRect, useIntroductionElevation } from "../📐️introduction-geometry/🟦️.ts";
import { INTRODUCTION_DEMO_NEXT_BUTTON_DEMONSTRATION, IntroductionDemonstrationOverlay } from "../🎬️introduction-demo/🟦️.tsx";
import { celebrateElement } from "../🧬️element-state/🟦️.ts";

export type UIIntroductionProps = {
  readonly introduction: IntroductionDefinition;
  readonly stepIndex: number;
  /** ✅️ Indices into the active step's `interactions` that are done — drives the checklist rows. */
  readonly completedInteractionIndices?: readonly number[];
  readonly onStepIndexChange: (index: number) => void;
  readonly onDismiss: (completed: boolean) => void;
};

/** @emoji 📐️ One row of an introduction step's {@link IntroductionLogo}s, all sharing one computed height
 * so the row fills its full width edge-to-edge with no logo dominating over another — the height isn't
 * guessed: it's solved from the row's measured width and each logo's own natural aspect ratio (`width /
 * sum-of-aspect-ratios`, accounting for the `gap-double` between logos), so it's exactly right for
 * whatever logos a step declares, never distorting any of them. */
function IntroductionLogoRow({ logos }: { readonly logos: readonly IntroductionLogo[] }) {
  const rowRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [rowWidth, setRowWidth] = reactHostPort.useState(0);
  const [gapPx, setGapPx] = reactHostPort.useState(0);
  const [aspectRatios, setAspectRatios] = reactHostPort.useState<readonly (number | null)[]>(() => logos.map(() => null));

  reactHostPort.useEffect(() => {
    const el = rowRef.current;
    if (!el) return;
    const measure = () => {
      setRowWidth(el.getBoundingClientRect().width);
      setGapPx(parseFloat(getComputedStyle(el).columnGap) || 0);
    };
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(el);
    return () => observer.disconnect();
  }, []);

  const onLogoLoad = reactHostPort.useCallback((index: number, event: React.SyntheticEvent<HTMLImageElement>) => {
    const img = event.currentTarget;
    if (!img.naturalWidth || !img.naturalHeight) return;
    setAspectRatios((prev) => (prev[index] != null ? prev : prev.map((ratio, i) => (i === index ? img.naturalWidth / img.naturalHeight : ratio))));
  }, []);

  const aspectRatioSum = aspectRatios.reduce<number | null>((sum, ratio) => (sum == null || ratio == null ? null : sum + ratio), 0);
  const height = rowWidth > 0 && aspectRatioSum != null && aspectRatioSum > 0 ? (rowWidth - gapPx * (logos.length - 1)) / aspectRatioSum : undefined;

  return (
    <div ref={rowRef} className="flex w-full items-center gap-double" style={{ visibility: height == null ? "hidden" : "visible" }}>
      {logos.map((logo, index) => {
        const style = height != null ? { height } : { height: 0, width: 0 };
        return (
          <span key={index} className="inline-flex items-center">
            {logo.href ? (
              <a href={logo.href} target="_blank" rel="noopener noreferrer" className="inline-flex items-center transition-opacity hover:opacity-80">
                <img src={logo.src} alt={logo.alt} onLoad={(event) => onLogoLoad(index, event)} className={cn("w-auto object-contain", logo.darkSrc && "dark:hidden")} style={style} />
                {logo.darkSrc && <img src={logo.darkSrc} alt={logo.alt} onLoad={(event) => onLogoLoad(index, event)} className="hidden w-auto object-contain dark:block" style={style} />}
              </a>
            ) : (
              <>
                <img src={logo.src} alt={logo.alt} onLoad={(event) => onLogoLoad(index, event)} className={cn("w-auto object-contain", logo.darkSrc && "dark:hidden")} style={style} />
                {logo.darkSrc && <img src={logo.darkSrc} alt={logo.alt} onLoad={(event) => onLogoLoad(index, event)} className="hidden w-auto object-contain dark:block" style={style} />}
              </>
            )}
          </span>
        );
      })}
    </div>
  );
}

/** @emoji 🎓️ Full-screen first-run walkthrough: a single fullscreen glass veil covers the screen, the
 * current step's `introduce`/`show` elements elevate above it (see `useIntroductionElevation`) and stay
 * crisp and interactive — `introduce` additionally pulses the introduced border on the precise element —
 * and an info box explains it (header {@link DragHandle} between title and step count repositions the box
 * for the current step; the box silhouette starts with the introduced highlight + thickness pulse, then
 * follows the shared surface-active lifecycle — click activates (primary stroke), click outside returns
 * to normal and never re-enters the introduced pulse for this step; each blank-line body paragraph
 * emphasizes only while the pointer is on that paragraph — see `[data-slot="introduction-body-paragraph"]:hover`
 * in `🎨️ui.css`).
 * Renders the declarative `IntroductionDefinition`/`IntroductionStepDefinition` contract. Every step
 * plays a ghost-cursor `IntroductionDemonstrationOverlay`: the step's own declared `demonstration` if
 * it has one, otherwise — for a purely informational step whose only way forward is the Next/Done
 * button — an automatic "click Next" demonstration (see `INTRODUCTION_DEMO_NEXT_BUTTON_DEMONSTRATION`),
 * so authors never have to spell that one out per step. */
export const UIIntroduction: React.FC<UIIntroductionProps> = ({ introduction, stepIndex, completedInteractionIndices, onStepIndexChange, onDismiss }) => {
  const introductionShellScope = useShellScopeOptional();
  const { terminology } = useUiTerminology();
  const locale = detectShellLocale(introductionShellScope?.i18n.resolvedLanguage ?? uiI18n.resolvedLanguage);
  const step: IntroductionStepDefinition | undefined = introduction.steps[stepIndex];
  const introduceSelector = step?.introduce ? elementIdSelector(step.introduce) : null;
  const introduceRect = useIntroductionAnchorRect(introduceSelector);
  const elevationIds = reactHostPort.useMemo((): readonly string[] => [...(step?.introduce ? [step.introduce] : []), ...(step?.show ?? [])], [step]);
  const elevated = useIntroductionElevation(elevationIds);
  // 🎬️ A step that declares no `demonstrations` still gets one automatically when Next/Done is its only
  // way forward — see `INTRODUCTION_DEMO_NEXT_BUTTON_DEMONSTRATION`. Memoized on `step` (not recomputed
  // fresh every render) so `IntroductionDemonstrationOverlay`'s effect — keyed on this array — doesn't
  // restart the demonstration loop on every unrelated re-render (viewport resize, box measure, …).
  const effectiveDemonstrations = reactHostPort.useMemo((): readonly IntroductionDemonstration[] => {
    if (!step) return [];
    // 🎬️ Hand-written brands may omit `demonstrations` (Rust `#[serde(default)]`); treat missing as empty.
    const demonstrations = step.demonstrations ?? [];
    if (demonstrations.length === 0) return (step.interactions ?? []).length === 0 ? [INTRODUCTION_DEMO_NEXT_BUTTON_DEMONSTRATION] : [];
    const interactions = step.interactions ?? [];
    if (interactions.length === 0) return demonstrations;
    const completed = completedInteractionIndices ?? [];
    return demonstrations.filter((_, index) => index >= interactions.length || !completed.includes(index));
  }, [step, completedInteractionIndices]);
  const [viewport, setViewport] = reactHostPort.useState(() => ({ width: window.innerWidth, height: window.innerHeight }));
  const [boxSize, setBoxSize] = reactHostPort.useState({ width: 320, height: 160 });
  const boxRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [surfaceActive, surfaceActiveProps] = useSurfaceActive(boxRef);
  // 🎯️ Once the step chrome is activated the introduced pulse is gone for this step — active while
  // selected, then normal after a background click (same lifecycle as panels/panes/windows).
  const [introductionActivated, setIntroductionActivated] = reactHostPort.useState(false);
  // 🫳️ User-dragged absolute position — `null` keeps auto/`placement` until the top-center handle moves the box;
  // reset on every step so each card starts at its authored placement again.
  const [dragPosition, setDragPosition] = reactHostPort.useState<IntroductionInfoBoxPosition | null>(null);
  const [dragging, setDragging] = reactHostPort.useState(false);
  const dragPositionRef = reactHostPort.useRef(dragPosition);
  dragPositionRef.current = dragPosition;
  const dragLayoutRef = reactHostPort.useRef({ placement: { top: 0, left: 0 }, boxSize, viewport });
  const dragSessionRef = reactHostPort.useRef<{ startTop: number; startLeft: number; pointerX: number; pointerY: number } | null>(null);

  reactHostPort.useEffect(() => {
    if (surfaceActive) setIntroductionActivated(true);
  }, [surfaceActive]);

  const shellScope = useShellScopeOptional();
  const shellActive = useIsActiveShellRoot(shellScope?.rootRef ?? NULL_SHELL_ROOT_REF);
  const overlayHost = shellScope?.portalLayerRef.current ?? null;
  const overlayPositioned = Boolean(overlayHost);

  const measureOverlayViewport = reactHostPort.useCallback(() => {
    if (overlayHost) {
      setViewport({ width: overlayHost.clientWidth, height: overlayHost.clientHeight });
      return;
    }
    const visual = window.visualViewport;
    setViewport({ width: visual && visual.width > 0 ? visual.width : window.innerWidth, height: availableViewportHeightPx({ innerHeight: window.innerHeight, visualHeight: visual?.height }) });
  }, [overlayHost]);

  reactHostPort.useEffect(() => {
    // Stamp the shell root (or documentElement for the landing-page tour outside any ShellScope) so
    // elevated-chrome CSS stays scoped to the introducing shell instead of every co-hosted shell.
    const root = shellScope?.rootRef.current ?? document.documentElement;
    root.setAttribute("data-introduction-active", "true");
    return () => root.removeAttribute("data-introduction-active");
  }, [shellScope]);

  reactHostPort.useEffect(() => {
    measureOverlayViewport();
    const onResize = () => measureOverlayViewport();
    window.addEventListener("resize", onResize);
    const visual = window.visualViewport;
    visual?.addEventListener("resize", onResize);
    let observer: ResizeObserver | null = null;
    if (overlayHost && typeof ResizeObserver !== "undefined") {
      observer = new ResizeObserver(measureOverlayViewport);
      observer.observe(overlayHost);
    }
    return () => {
      window.removeEventListener("resize", onResize);
      visual?.removeEventListener("resize", onResize);
      observer?.disconnect();
    };
  }, [measureOverlayViewport, overlayHost]);

  reactHostPort.useEffect(() => {
    const el = boxRef.current;
    if (!el) return;
    const measure = () => setBoxSize({ width: el.offsetWidth, height: el.offsetHeight });
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(el);
    return () => observer.disconnect();
  }, [stepIndex]);

  reactHostPort.useEffect(() => {
    setDragPosition(null);
    setDragging(false);
    dragSessionRef.current = null;
    setIntroductionActivated(false);
  }, [stepIndex]);

  const skipLabel = useLabel("introduction.skip");
  const backLabel = useLabel("introduction.back");
  const nextLabel = useLabel("introduction.next");
  const doneLabel = useLabel("introduction.done");

  const isLast = stepIndex >= introduction.steps.length - 1;
  const skip = reactHostPort.useCallback(() => onDismiss(false), [onDismiss]);
  const next = reactHostPort.useCallback(() => {
    if (isLast) onDismiss(true);
    else onStepIndexChange(stepIndex + 1);
  }, [isLast, onDismiss, onStepIndexChange, stepIndex]);
  const back = reactHostPort.useCallback(() => onStepIndexChange(Math.max(0, stepIndex - 1)), [onStepIndexChange, stepIndex]);
  const interactions = step?.interactions ?? [];
  const advanceByButton = interactions.length === 0;

  // 🎉️ Celebrates a checklist row's own label text (conic-gradient ring, see `[data-celebrated="true"]`
  // in `🎨️ui.css`) the instant its interaction flips from pending to done — on top of whatever app element
  // `interaction.celebrate`/`step.introduce` celebrates, so the completed line item itself visibly glows.
  const interactionLabelRefs = reactHostPort.useRef<Map<number, HTMLSpanElement>>(new Map());
  const prevCompletedRef = reactHostPort.useRef<{ stepIndex: number; indices: readonly number[] }>({ stepIndex, indices: [] });
  reactHostPort.useEffect(() => {
    const prev = prevCompletedRef.current;
    const current = completedInteractionIndices ?? [];
    if (prev.stepIndex === stepIndex) {
      for (const index of current) {
        if (!prev.indices.includes(index)) {
          const label = interactionLabelRefs.current.get(index);
          if (label) celebrateElement(label);
        }
      }
    }
    prevCompletedRef.current = { stepIndex, indices: current };
  }, [stepIndex, completedInteractionIndices]);

  const dragPointerProps = usePointerDrag<HTMLSpanElement>({
    onStart: (event) => {
      event.preventDefault();
      event.stopPropagation();
      const start = dragPositionRef.current ?? dragLayoutRef.current.placement;
      dragSessionRef.current = { startTop: start.top, startLeft: start.left, pointerX: event.clientX, pointerY: event.clientY };
      setDragging(true);
    },
    onMove: (event) => {
      const session = dragSessionRef.current;
      if (!session) return;
      const { boxSize: size, viewport: vp } = dragLayoutRef.current;
      setDragPosition(
        clampIntroductionInfoBoxPosition(
          {
            top: session.startTop + (event.clientY - session.pointerY),
            left: session.startLeft + (event.clientX - session.pointerX),
          },
          size,
          vp,
          0,
        ),
      );
    },
    onEnd: () => {
      dragSessionRef.current = null;
      setDragging(false);
    },
    onCancel: () => {
      dragSessionRef.current = null;
      setDragging(false);
    },
  });

  useControlKeybinding("ui.introduction.skip", skip, { enableOnFormTags: true, enabled: shellActive }, [skip, shellActive]);
  useControlKeybinding("ui.introduction.next", () => advanceByButton && next(), { enableOnFormTags: true, enabled: shellActive && advanceByButton }, [advanceByButton, next, shellActive]);
  useControlKeybinding("ui.introduction.back", back, { enableOnFormTags: true, enabled: shellActive }, [back, shellActive]);

  if (!step) return null;

  const stepTitle = resolveUiLocalizedText(step.title, terminology, locale);
  const bodyParagraphs = splitIntroductionBodyParagraphs(resolveUiLocalizedText(step.body, terminology, locale));
  // When the overlay is absolutely positioned inside the shell portal layer (multi-shell hosts that
  // CSS-transform their shells, e.g. the demonstrator grid), placement must use host-local coords —
  // getBoundingClientRect is viewport-space and would otherwise land the box on the wrong pane.
  const hostRect = overlayHost?.getBoundingClientRect() ?? null;
  const placementAnchor = introduceRect && hostRect ? introductionRectRelativeToHost(introduceRect, hostRect) : introduceRect;
  const boxPosition = resolveIntroductionPlacement(step.placement, placementAnchor, boxSize, viewport);
  dragLayoutRef.current = { placement: boxPosition, boxSize, viewport };
  const position = dragPosition ?? boxPosition;
  const overlayPositionClass = overlayPositioned ? "absolute" : "fixed";
  // 🎓️ A targeted element that hasn't mounted yet (a folded utility bar/panel the shell is still revealing)
  // must not trap the user behind an opaque-to-clicks veil — only screen-style steps (`introduce == null`)
  // and steps whose target did resolve block pointer events; an unresolved `introduce` lets clicks through
  // so the user can reveal it themselves. A resolved `show` target also counts (e.g. a drop window while
  // the drag source mounts).
  const veilBlocksPointer = step.introduce == null || elevated.has(step.introduce) || (step.show ?? []).some((id) => elevated.has(id));

  const overlay = (
    <>
      {/* 🧯️ A blocking veil owns every pointer in the app, so pressing it MUST end the tour — otherwise a
          Skip press that never lands (covered, animating, off-viewport) leaves the whole application inert
          with no way out but the keyboard. Keyboard parity is `ui.introduction.skip` ("escape", bound
          above); the scrim itself stays `aria-hidden` so no assistive technology announces a bare div. */}
      <div
        data-slot="introduction-veil"
        data-level="dialog"
        aria-hidden="true"
        onPointerDown={veilBlocksPointer ? skip : undefined}
        className={cn(veilClass, "z-tutorial inset-0", overlayPositionClass, veilBlocksPointer ? "pointer-events-auto cursor-pointer" : "pointer-events-none")}
      />
      <WindowChrome
        ref={boxRef}
        stackSlot="introduction-info-box"
        stackDataAttrs={dragging ? { "data-dragging": "true" } : undefined}
        stackBindProps={surfaceActiveProps}
        active={surfaceActive}
        borderKind={introductionActivated ? undefined : "introduced"}
        level="dialog"
        className={cn("pointer-events-auto z-tutorial w-fit max-w-[calc(100vw-2rem)] bg-transparent", overlayPositionClass)}
        style={{ top: position.top, left: position.left, zIndex: "calc(var(--z-tutorial) + 2)" }}
        titleChips={
          <div data-hover-scope data-slot="introduction-info-box-chip" className={cn(windowChromeTitleChipClass, "!h-auto max-w-none shrink overflow-visible whitespace-normal")}>
            <span data-slot="introduction-info-box-title" className="min-w-0 flex-1 break-words whitespace-normal text-sm leading-normal font-medium">
              {stepTitle}
            </span>
            <div data-slot="introduction-info-box-drag" className="flex shrink-0 items-center justify-center">
              <DragHandle {...dragPointerProps} emphasized={dragging} />
            </div>
          </div>
        }
        close={{
          id: "ui.introduction.skip",
          slot: "introduction-close",
          icon: <CloseIcon className="size-small" />,
          label: skipLabel,
          onClick: skip,
        }}
        body={
          <div data-slot="introduction-info-box-content" className="max-w-sm">
            {bodyParagraphs.length > 0 && (
              <div data-slot="introduction-body" className="mb-double flex flex-col gap-double">
                {bodyParagraphs.map((paragraph, index) => (
                  <p key={index} data-slot="introduction-body-paragraph" className="whitespace-pre-line text-xs text-muted-foreground">
                    {paragraph}
                  </p>
                ))}
              </div>
            )}
            {step.logos && step.logos.length > 0 && (
              <div className="mb-double flex flex-col gap-double">
                {[step.logos.slice(0, -1), step.logos.slice(-1)]
                  .filter((row) => row.length > 0)
                  .map((row, rowIndex) => (
                    <IntroductionLogoRow key={rowIndex} logos={row} />
                  ))}
              </div>
            )}
            {interactions.length > 0 && (
              <ul data-slot="introduction-interactions" className="mb-double flex flex-col gap-half text-xs">
                {interactions.map((interaction, index) => {
                  const done = completedInteractionIndices?.includes(index) ?? false;
                  return (
                    <li key={index} data-completed={done || undefined} className={cn("flex items-center gap-single", done ? "text-foreground" : "text-muted-foreground")}>
                      <Icon icon={done ? "check" : "circle"} size="small" />
                      {step.ordered && <span data-slot="introduction-interaction-index">{index + 1}.</span>}
                      <span
                        ref={(el) => {
                          if (el) interactionLabelRefs.current.set(index, el);
                          else interactionLabelRefs.current.delete(index);
                        }}
                        data-slot="introduction-interaction-label"
                        className="inline-flex items-center"
                      >
                        {interaction.label}
                      </span>
                    </li>
                  );
                })}
              </ul>
            )}
          </div>
        }
        footerCenterChips={
          <div data-slot="introduction-step-chip" className={windowChromeTitleChipClass}>
            <span className="px-single text-xs text-muted-foreground transition-colors group-hover:text-emphasized">
              {stepIndex + 1} / {introduction.steps.length}
            </span>
          </div>
        }
        footerLeftChips={stepIndex > 0 ? <Button id="ui.introduction.back" variant="ghost" icon="chevron-left" text={backLabel} onClick={back} /> : undefined}
        footerRightChips={advanceByButton ? <Button id="ui.introduction.next" icon={isLast ? "check" : "chevron-right"} text={isLast ? doneLabel : nextLabel} onClick={next} /> : undefined}
        bodyClassName="p-double shadow-lg"
      />
      {effectiveDemonstrations.length > 0 && <IntroductionDemonstrationOverlay key={step.id} demonstrations={effectiveDemonstrations} />}
    </>
  );

  if (overlayHost && typeof document !== "undefined") {
    return createPortal(overlay, overlayHost);
  }
  return overlay;
};
