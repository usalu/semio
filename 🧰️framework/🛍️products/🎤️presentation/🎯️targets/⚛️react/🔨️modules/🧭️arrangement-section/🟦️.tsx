/** 🧭️ The arrangement section and its surface, animate scene embeds and the interaction provider and bridge.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

import type {
  DispositionPosition,
  Presentation,
  RenderSlide,
  Chapter,
  Sequence,
  Thought
} from "@semio-tech/presentation";
import { isIntroArrangementId, resolutionScopeForArrangement } from "@semio-tech/presentation";
// #region 🔌️Adapters
import {
  useEffect,
  useMemo,
  useRef,
  type FC,
  type ReactNode,
  type RefObject
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import Reveal from "reveal.js";
import { centerRevealResolvedArrangement, isRevealMorphCompanionOnly, resolveRevealArrangement } from "../🔀️reveal-morph/🟦️.ts";
import { buildInteractiveSlideLayout, PresentationInteractionContext, usePresentationInteractionState } from "../🧮️interaction-model/🟦️.ts";
import {
  InteractionLayer,
  InteractiveDisposition,
  InteractiveRowBand,
  isRevealSlideBackgroundPointerTarget,
  SlideDispositionRegistryProvider,
  SlideInteractionReset,
  usePresentationInteraction,
  useSlideBackgroundInteraction
} from "../🫳️interaction/🟦️.tsx";

//#region 🔖️ArrangementSection
function resolveSceneClipSrc(sceneHash: string): string {
  return `scenes/${sceneHash}/scene.mp4`;
}

const AnimateSceneEmbed: FC<{ readonly sceneHash: string }> = ({ sceneHash }) => (
  <div className="presentation-animate-scene" data-scene-hash={sceneHash}>
    <video className="presentation-media-video presentation-animate-scene__video" src={resolveSceneClipSrc(sceneHash)} controls playsInline muted loop preload="metadata" />
    <canvas className="presentation-animate-scene__canvas" aria-hidden />
  </div>
);

export const ArrangementSection: FC<{
  readonly presentation: Presentation;
  readonly chapter: Chapter;
  readonly sequence: Sequence;
  readonly thought: Thought;
  readonly renderSlide: RenderSlide;
}> = ({ presentation, chapter, sequence, thought, renderSlide }) => {
  const sectionRef = useRef<HTMLElement>(null);
  const scope = useMemo(() => resolutionScopeForArrangement(presentation, chapter, sequence, thought, renderSlide.arrangement), [presentation, chapter, sequence, thought, renderSlide.arrangement]);
  const slideContext = useMemo(() => {
    const index = thought.slides.findIndex((slide) => slide.arrangement.id === renderSlide.id);
    if (index < 0) {
      return {};
    }
    return {
      previousSlide: index > 0 ? thought.slides[index - 1] : undefined,
      nextSlide: index < thought.slides.length - 1 ? thought.slides[index + 1] : undefined,
    };
  }, [thought.slides, renderSlide.id]);
  const resolved = useMemo(() => resolveRevealArrangement(scope, renderSlide.arrangement, slideContext), [scope, renderSlide.arrangement, slideContext]);
  const morph = renderSlide.autoAnimateId !== undefined;
  const positioned = resolved.some((disposition) => disposition.position !== undefined);
  const layoutResolved = positioned && !morph ? centerRevealResolvedArrangement(resolved) : resolved;
  const interactiveLayout = useMemo(() => buildInteractiveSlideLayout(renderSlide.id, layoutResolved, morph), [layoutResolved, morph, renderSlide.id]);
  const declaredRects = useMemo(() => {
    const map = new Map<string, DispositionPosition | undefined>();
    for (const entry of interactiveLayout.placements) {
      map.set(entry.id, entry.sectionRect ?? entry.declaredRect);
    }
    return map;
  }, [interactiveLayout.placements]);
  const interactiveDispositionIds = useMemo(() => interactiveLayout.placements.filter((entry) => !isRevealMorphCompanionOnly(entry.disposition)).map((entry) => entry.id), [interactiveLayout.placements]);
  const sceneHash = renderSlide.metadata?.sceneHash ?? renderSlide.arrangement.metadata?.sceneHash;
  const placements = (
    <>
      {sceneHash ? <AnimateSceneEmbed sceneHash={sceneHash} /> : null}
      {interactiveLayout.rowBands.map((band) => (
        <InteractiveRowBand key={band.id} id={band.id} frame={band.frame} tileIds={band.tileIds} />
      ))}
      {interactiveLayout.placements.map((entry) => (
        <InteractiveDisposition
          key={entry.id}
          id={entry.id}
          disposition={entry.disposition}
          declaredRect={entry.declaredRect}
          sectionDeclaredRect={entry.sectionRect}
          allDeclaredRects={declaredRects}
          sectionRef={sectionRef}
          revealMorphId={entry.revealMorphId}
          rowBandId={entry.rowBandId}
        />
      ))}
    </>
  );
  return (
    <SlideDispositionRegistryProvider>
      <ArrangementSectionSurface
        sectionRef={sectionRef}
        morph={morph}
        autoAnimateId={renderSlide.autoAnimateId}
        settleBeforeMorphTo={renderSlide.arrangement.settleBeforeMorphTo}
        slideId={renderSlide.id}
        positioned={positioned}
        dispositionIds={interactiveDispositionIds}
        declaredRects={declaredRects}
        placements={placements}
      />
    </SlideDispositionRegistryProvider>
  );
};

const ArrangementSectionSurface: FC<{
  readonly sectionRef: RefObject<HTMLElement | null>;
  readonly morph: boolean;
  readonly autoAnimateId: string | undefined;
  readonly settleBeforeMorphTo: readonly string[] | undefined;
  readonly slideId: string;
  readonly positioned: boolean;
  readonly dispositionIds: readonly string[];
  readonly declaredRects: ReadonlyMap<string, DispositionPosition | undefined>;
  readonly placements: ReactNode;
}> = ({ sectionRef, morph, autoAnimateId, settleBeforeMorphTo, slideId, positioned, dispositionIds, declaredRects, placements }) => {
  const backgroundInteraction = useSlideBackgroundInteraction({
    sectionRef,
    canvasPlacement: positioned,
    dispositionIds,
    declaredRects,
  });
  return (
    <section
      ref={sectionRef}
      onPointerDownCapture={backgroundInteraction.onPointerDownCapture}
      {...(morph ? { "data-auto-animate": "", "data-auto-animate-id": autoAnimateId } : {})}
      {...(settleBeforeMorphTo?.length ? { "data-settle-before-morph-to": settleBeforeMorphTo.join(",") } : {})}
      title={slideId}
      className={["presentation-arrangement--interactive", positioned ? "presentation-arrangement--positioned" : undefined, isIntroArrangementId(slideId) ? "presentation-arrangement--intro" : undefined].filter(Boolean).join(" ")}
    >
      {positioned ? (
        <div className="presentation-arrangement-surface">
          <InteractionLayer marquee={backgroundInteraction.marquee} />
          <div className="presentation-arrangement-canvas">{placements}</div>
          <SlideInteractionReset sectionRef={sectionRef} dispositionIds={dispositionIds} declaredRects={declaredRects} />
        </div>
      ) : (
        <>
          <InteractionLayer marquee={backgroundInteraction.marquee} />
          {placements}
          <SlideInteractionReset sectionRef={sectionRef} dispositionIds={dispositionIds} declaredRects={declaredRects} />
        </>
      )}
    </section>
  );
};
//#endregion 🔖️ArrangementSection

//#region 🔖️PresentationInteractionProvider
/** 🖱️ Clears selection and ephemeral layout when reveal.js changes the active slide. */
export const PresentationSlideInteractionBridge: FC<{
  readonly deckRef: RefObject<Reveal.Api | null>;
}> = ({ deckRef }) => {
  const interaction = usePresentationInteractionState();

  useEffect(() => {
    const attach = (deck: Reveal.Api): (() => void) => {
      const onSlideChanged = (): void => {
        interaction.clearEphemeralLayout();
      };
      const deckEl = deck.getRevealElement();
      const onPointerDown = (event: PointerEvent): void => {
        if (event.button === 0 && deckEl && !deckEl.classList.contains("overview") && isRevealSlideBackgroundPointerTarget(event.target)) {
          interaction.clearSelection();
        }
      };
      deck.on("slidechanged", onSlideChanged);
      deckEl?.addEventListener("pointerdown", onPointerDown, { capture: true });
      return () => {
        deck.off("slidechanged", onSlideChanged);
        deckEl?.removeEventListener("pointerdown", onPointerDown, { capture: true });
      };
    };

    const deck = deckRef.current;
    if (deck) {
      return attach(deck);
    }

    let detach: (() => void) | undefined;
    const poll = window.setInterval(() => {
      const ready = deckRef.current;
      if (!ready) {
        return;
      }
      window.clearInterval(poll);
      detach = attach(ready);
    }, 50);
    return () => {
      window.clearInterval(poll);
      detach?.();
    };
  }, [deckRef, interaction]);

  return null;
};

export const PresentationInteractionProvider: FC<{
  readonly children: ReactNode;
}> = ({ children }) => {
  const interaction = usePresentationInteraction();
  return <PresentationInteractionContext.Provider value={interaction}>{children}</PresentationInteractionContext.Provider>;
};
//#endregion 🔖️PresentationInteractionProvider
