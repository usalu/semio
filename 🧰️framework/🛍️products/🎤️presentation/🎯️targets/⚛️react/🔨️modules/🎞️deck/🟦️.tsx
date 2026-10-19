/** 🎞️ The reveal-backed presentation deck component and the mount and unmount entry points.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import { ephemeralBox } from "@semio-tech/framework";
import type { Presentation } from "@semio-tech/presentation";
import { expandThoughtSlides } from "@semio-tech/presentation";
import {
  applyElementsSurfaceChrome,
  useCallback,
  useEffect,
  useRef,
  useState,
  type FC,
  createRoot,
  type Root
} from "@semio-tech/ui-react";
import Reveal from "reveal.js";
// #endregion 🔌️Adapters
import {
  DEFAULT_SURFACE_CHROME,
  type PresentationMountOptions,
  readPresentationSlideIndicesFromUrl,
  syncPresentationSlideSizeVars,
  syncPresentationSlideUrl,
  syncRevealBackgroundKind
} from "../🎚️deck-chrome/🟦️.ts";
import {
  finalizeRevealAutoAnimateRestState,
  patchPresentationAutoAnimateRunStyleSheet,
  prepareArrangementBeforeAutoAnimate,
  presentationAutoAnimateMatcher,
  type PresentationAutoAnimateRunSlides,
  relaxHiddenPreflight,
  resolvePresentationAutoAnimateRunSlides,
  resolveRevealSlideAt,
  slideChangedEventSlides,
  slidesShareAutoAnimateId,
  syncArrangementSettledState
} from "../✨️auto-animate/🟦️.ts";
import { parsePresentationSlideCssSize, PresentationSlideAspectContext, PresentationSlideEpochContext } from "../🏷️morph-context/🟦️.ts";
import { ArrangementSection, PresentationInteractionProvider, PresentationSlideInteractionBridge } from "../🧭️arrangement-section/🟦️.tsx";

//#region 🔖️PresentationDeck
/** 🎞️ Maps a {@link Presentation} to reveal.js DOM. */
export const PresentationDeck: FC<{
  readonly presentation: Presentation;
  readonly options?: PresentationMountOptions;
}> = ({ presentation, options }) => {
  const deckDivRef = useRef<HTMLDivElement>(null);
  const deckRef = useRef<Reveal.Api | null>(null);
  const previousSlideRef = useRef<HTMLElement | null>(null);
  const [slideEpoch, setSlideEpoch] = useState(0);
  const [slideAspect, setSlideAspect] = useState<number | undefined>(undefined);
  const syncSlideAspect = useCallback((deckEl: HTMLElement): void => {
    const { width, height } = parsePresentationSlideCssSize(deckEl);
    setSlideAspect(width / height);
  }, []);

  useEffect(() => {
    const deckEl = deckDivRef.current;
    if (!deckEl || deckRef.current) {
      return;
    }
    const slideUrlEnabled = options?.hash !== false;
    const revealOptions: Reveal.Options = {
      transition: options?.transition ?? "fade",
      autoAnimate: true,
      autoAnimateMatcher: presentationAutoAnimateMatcher,
      autoAnimateUnmatched: true,
      center: true,
    };
    if (options?.slideNumber === true) {
      revealOptions.slideNumber = true;
    }
    if (options?.width ?? presentation.width) {
      revealOptions.width = options?.width ?? presentation.width;
    }
    if (options?.height ?? presentation.height) {
      revealOptions.height = options?.height ?? presentation.height;
    }
    relaxHiddenPreflight();
    const deck = new Reveal(deckEl, revealOptions);
    deckRef.current = deck;
    const syncSlideUrl = (): void => {
      if (!slideUrlEnabled) {
        return;
      }
      syncPresentationSlideUrl(presentation, deck.getIndices());
    };
    const tryPlayVideo = (video: HTMLVideoElement): void => {
      try {
        const playResult = video.play();
        if (playResult !== undefined) {
          void playResult.catch(() => undefined);
        }
      } catch {}
    };
    const syncPresentSlideMedia = (): void => {
      for (const video of deckEl.querySelectorAll<HTMLVideoElement>("video.presentation-media-video")) {
        const section = video.closest("section");
        const isPresent = section?.classList.contains("present") === true;
        if (isPresent) {
          tryPlayVideo(video);
        } else {
          video.pause();
        }
      }
    };
    const onResize = (): void => {
      syncPresentationSlideSizeVars(deckEl, deck);
      syncSlideAspect(deckEl);
    };
    let autoAnimateFinalizeTimer: ReturnType<typeof setTimeout> | undefined;
    let pendingAutoAnimateRunSlides: PresentationAutoAnimateRunSlides | undefined;
    const scheduleFinalizeAutoAnimateRest = (): void => {
      const configuredDuration = deck.getConfig().autoAnimateDuration;
      const durationSeconds = typeof configuredDuration === "number" ? configuredDuration : 1;
      if (autoAnimateFinalizeTimer !== undefined) {
        clearTimeout(autoAnimateFinalizeTimer);
      }
      autoAnimateFinalizeTimer = setTimeout(
        () => {
          finalizeRevealAutoAnimateRestState(deckEl);
          autoAnimateFinalizeTimer = undefined;
        },
        durationSeconds * 1000 + 80,
      );
    };
    const onBeforeSlideChange = (event: Event): void => {
      if (autoAnimateFinalizeTimer !== undefined) {
        clearTimeout(autoAnimateFinalizeTimer);
        autoAnimateFinalizeTimer = undefined;
      }
      relaxHiddenPreflight();
      const slideEvent = event as Event & { readonly indexh?: number; readonly indexv?: number };
      const fromSlide = deck.getCurrentSlide() as HTMLElement | null;
      if (!fromSlide || slideEvent.indexh === undefined || slideEvent.indexv === undefined) {
        return;
      }
      const toSlide = resolveRevealSlideAt(deckEl, { h: slideEvent.indexh, v: slideEvent.indexv });
      if (!toSlide) {
        return;
      }
      pendingAutoAnimateRunSlides = { fromSlide, toSlide };
      const fromAnimateId = fromSlide.getAttribute("data-auto-animate-id");
      const toAnimateId = toSlide.getAttribute("data-auto-animate-id");
      if (!fromAnimateId || fromAnimateId !== toAnimateId) {
        finalizeRevealAutoAnimateRestState(deckEl);
      }
      prepareArrangementBeforeAutoAnimate(fromSlide, toSlide);
    };
    const onSlideChanged = (event: Event): void => {
      relaxHiddenPreflight();
      syncRevealBackgroundKind(deckEl);
      syncPresentationSlideSizeVars(deckEl, deck);
      syncSlideAspect(deckEl);
      syncPresentSlideMedia();
      syncSlideUrl();
      const { previousSlide: eventPrevious, currentSlide: eventCurrent } = slideChangedEventSlides(event);
      const currentSlide = eventCurrent ?? (deck.getCurrentSlide() as HTMLElement | null);
      const previousSlide = eventPrevious ?? previousSlideRef.current;
      if (previousSlide && currentSlide) {
        prepareArrangementBeforeAutoAnimate(previousSlide, currentSlide);
      }
      syncArrangementSettledState(deckEl, currentSlide, previousSlide);
      previousSlideRef.current = currentSlide;
      if (!slidesShareAutoAnimateId(previousSlide, currentSlide)) {
        setSlideEpoch((epoch) => epoch + 1);
      }
      scheduleFinalizeAutoAnimateRest();
    };
    const onWindowHashChange = (): void => {
      if (!slideUrlEnabled) {
        return;
      }
      const indices = readPresentationSlideIndicesFromUrl();
      if (!indices) {
        return;
      }
      const current = deck.getIndices();
      if (current.h !== indices.h || current.v !== indices.v) {
        void deck.slide(indices.h, indices.v);
      }
    };
    const onAutoAnimate = (event: Event): void => {
      const animateEvent = event as Event & {
        readonly data?: { readonly fromSlide?: HTMLElement; readonly toSlide?: HTMLElement };
        readonly sheet?: { innerHTML: string };
      };
      const fromSlide = animateEvent.data?.fromSlide;
      const toSlide = animateEvent.data?.toSlide;
      const runSlides = resolvePresentationAutoAnimateRunSlides({ fromSlide, toSlide }, pendingAutoAnimateRunSlides);
      if (runSlides.fromSlide && runSlides.toSlide) {
        prepareArrangementBeforeAutoAnimate(runSlides.fromSlide, runSlides.toSlide);
      }
      const sheet = animateEvent.sheet;
      if (sheet && typeof sheet.innerHTML === "string") {
        const configuredDuration = deck.getConfig().autoAnimateDuration;
        const durationSeconds = typeof configuredDuration === "number" ? configuredDuration : 1;
        patchPresentationAutoAnimateRunStyleSheet(sheet, durationSeconds, runSlides.fromSlide, runSlides.toSlide);
      }
      scheduleFinalizeAutoAnimateRest();
    };
    void deck.initialize().then(() => {
      relaxHiddenPreflight();
      syncRevealBackgroundKind(deckEl);
      syncPresentationSlideSizeVars(deckEl, deck);
      syncSlideAspect(deckEl);
      syncPresentSlideMedia();
      if (slideUrlEnabled) {
        const indices = readPresentationSlideIndicesFromUrl();
        const afterSlideSync = (): void => {
          syncSlideUrl();
          const currentSlide = deck.getCurrentSlide();
          syncArrangementSettledState(deckEl, currentSlide, previousSlideRef.current);
          previousSlideRef.current = currentSlide;
          setSlideEpoch((epoch) => epoch + 1);
        };
        if (indices) {
          const current = deck.getIndices();
          if (current.h !== indices.h || current.v !== indices.v) {
            const slideResult = deck.slide(indices.h, indices.v) as Promise<void> | undefined;
            if (slideResult && typeof slideResult.then === "function") {
              void slideResult.then(() => {
                afterSlideSync();
                finalizeRevealAutoAnimateRestState(deckEl);
                scheduleFinalizeAutoAnimateRest();
              });
            } else {
              afterSlideSync();
              finalizeRevealAutoAnimateRestState(deckEl);
              scheduleFinalizeAutoAnimateRest();
            }
          } else {
            afterSlideSync();
            finalizeRevealAutoAnimateRestState(deckEl);
            scheduleFinalizeAutoAnimateRest();
          }
        } else {
          afterSlideSync();
          finalizeRevealAutoAnimateRestState(deckEl);
          scheduleFinalizeAutoAnimateRest();
        }
        window.addEventListener("hashchange", onWindowHashChange);
      } else {
        const currentSlide = deck.getCurrentSlide();
        syncArrangementSettledState(deckEl, currentSlide, previousSlideRef.current);
        previousSlideRef.current = currentSlide;
        setSlideEpoch((epoch) => epoch + 1);
        finalizeRevealAutoAnimateRestState(deckEl);
        scheduleFinalizeAutoAnimateRest();
      }
      options?.onRevealReady?.(deck);
      deck.on("beforeslidechange", onBeforeSlideChange);
      deck.on("slidechanged", onSlideChanged);
      deck.on("resize", onResize);
      deck.on("autoanimate", onAutoAnimate);
    });
    return () => {
      if (autoAnimateFinalizeTimer !== undefined) {
        clearTimeout(autoAnimateFinalizeTimer);
      }
      window.removeEventListener("hashchange", onWindowHashChange);
      deck.off("beforeslidechange", onBeforeSlideChange);
      deck.off("slidechanged", onSlideChanged);
      deck.off("resize", onResize);
      deck.off("autoanimate", onAutoAnimate);
      try {
        deck.destroy();
      } catch {}
      deckRef.current = null;
    };
  }, []);

  return (
    <PresentationSlideEpochContext.Provider value={slideEpoch}>
      <PresentationSlideAspectContext.Provider value={slideAspect}>
        <PresentationInteractionProvider>
          <PresentationSlideInteractionBridge deckRef={deckRef} />
          <div className="reveal" ref={deckDivRef} style={{ width: "100vw", height: "100vh" }}>
            <div className="slides">
              {presentation.chapters.flatMap((chapter) =>
                chapter.sequences.map((sequence) => (
                  <section key={`${chapter.id}-${sequence.id}`}>
                    {sequence.thoughts.flatMap((thought) =>
                      expandThoughtSlides(thought).map((renderSlide) => (
                        <ArrangementSection key={`${chapter.id}-${sequence.id}-${thought.id}-${renderSlide.id}`} presentation={presentation} chapter={chapter} sequence={sequence} thought={thought} renderSlide={renderSlide} />
                      )),
                    )}
                  </section>
                )),
              )}
            </div>
          </div>
        </PresentationInteractionProvider>
      </PresentationSlideAspectContext.Provider>
    </PresentationSlideEpochContext.Provider>
  );
};
//#endregion 🔖️PresentationDeck

//#region 🔖️Mount
const mountedRoot = ephemeralBox<Root | null>("framework.products.presentation.targets.react.mountedRoot", null);
const surfaceChromeCleanup = ephemeralBox<(() => void) | null>("framework.products.presentation.targets.react.surfaceChromeCleanup", null);

/** 🚀️ Mounts a declarative presentation into a DOM root via React + reveal.js (eg-ice-25 reveal wiring). */
export function mountPresentation(rootEl: HTMLElement, presentation: Presentation, options?: PresentationMountOptions): void {
  surfaceChromeCleanup.current?.();
  surfaceChromeCleanup.current = null;
  const chrome = options?.surfaceChrome;
  if (chrome !== false) {
    surfaceChromeCleanup.current = applyElementsSurfaceChrome(chrome ?? DEFAULT_SURFACE_CHROME);
  }
  mountedRoot.current?.unmount();
  mountedRoot.current = createRoot(rootEl);
  mountedRoot.current.render(<PresentationDeck presentation={presentation} options={options} />);
}

/** 🧹️ Unmounts a presentation previously mounted with {@link mountPresentation}. */
export function unmountPresentation(): void {
  mountedRoot.current?.unmount();
  mountedRoot.current = null;
  surfaceChromeCleanup.current?.();
  surfaceChromeCleanup.current = null;
}
//#endregion 🔖️Mount
