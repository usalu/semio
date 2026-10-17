/** 🫳️ The presentation interaction layer: pointer targets, background interaction, the slide reset control and the layer that hosts the interactive dispositions.
 *
 * @see ./🎛️interaction-hook/🟦️.ts — the per-slide interaction state hook
 * @see ./🗂️disposition-registry/🟦️.tsx — the measured disposition rects
 * @see ./🧲️interactive-disposition/🟦️.tsx — the interactive disposition and row band
 */

import type { DispositionPosition } from "@semio-tech/presentation";

// #region 🔌️Adapters
import { Icon, SelectionMarquee, useCallback, useEffect, useRef, useState, type FC, type RefObject } from "@semio-tech/ui-react";

// #endregion 🔌️Adapters
import { clientToSectionFraction, dispositionHasEphemeralLayout, dispositionPlacementContainer, marqueeSelectionRule, marqueeSelects, normalizeMarquee, POINTER_DRAG_THRESHOLD_PX, pointerNearSlideResetHotspot, revealDeckInOverview, suppressRevealOverviewSlideNavigation, usePresentationInteractionState } from "../🧮️interaction-model/🟦️.ts";
import { resolveEffectiveDispositionRect, useSlideDispositionRegistry } from "./🗂️disposition-registry/🟦️.tsx";

//#region 🔖️Interaction
function isDispositionPointerTarget(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) {
    return false;
  }
  if (target.closest(".presentation-target-ghost") || target.closest(".presentation-source-ghost")) {
    return false;
  }
  return Boolean(
    target.closest(".presentation-interactive-disposition") ||
    target.closest(".presentation-interactive-row-band") ||
    target.closest(".presentation-interactive-visual-row") ||
    target.closest(".presentation-interaction-handle") ||
    target.closest(".presentation-interaction-enlarge") ||
    target.closest(".presentation-pdf-page-nav__button") ||
    target.closest(".presentation-interaction-reset") ||
    target.closest(".presentation-interaction-slide-reset") ||
    target.closest(".presentation-interaction-slide-reset-host"),
  );
}

export function isRevealSlideBackgroundPointerTarget(target: EventTarget | null): boolean {
  return target instanceof Element && Boolean(target.closest(".slide-background.present, .slide-background-content"));
}

/** 🖱️ Marquee and deselect on slide background; mounted on the arrangement section capture phase. */
export function useSlideBackgroundInteraction({
  sectionRef,
  canvasPlacement,
  dispositionIds,
  declaredRects,
}: {
  readonly sectionRef: RefObject<HTMLElement | null>;
  readonly canvasPlacement: boolean;
  readonly dispositionIds: readonly string[];
  readonly declaredRects: ReadonlyMap<string, DispositionPosition | undefined>;
}): {
  readonly onPointerDownCapture: (event: React.PointerEvent) => void;
  readonly marquee: {
    readonly start: { readonly x: number; readonly y: number };
    readonly end: { readonly x: number; readonly y: number };
  } | null;
} {
  const interaction = usePresentationInteractionState();
  const registry = useSlideDispositionRegistry();
  const [marquee, setMarquee] = useState<{
    readonly start: { readonly x: number; readonly y: number };
    readonly end: { readonly x: number; readonly y: number };
  } | null>(null);

  const resolveRectForId = useCallback(
    (targetId: string): DispositionPosition | undefined => {
      return resolveEffectiveDispositionRect(targetId, declaredRects.get(targetId), interaction, registry);
    },
    [declaredRects, interaction, registry],
  );

  const onPointerDownCapture = useCallback(
    (event: React.PointerEvent): void => {
      if (event.button !== 0 || isDispositionPointerTarget(event.target)) {
        return;
      }
      const section = sectionRef.current;
      if (!section) {
        return;
      }
      const fractionContainer = dispositionPlacementContainer(section, canvasPlacement);
      const fraction = clientToSectionFraction(fractionContainer, event.clientX, event.clientY);
      const start = { x: fraction.x, y: fraction.y };
      const startedInOverview = revealDeckInOverview(section);
      let moved = false;

      const onMove = (moveEvent: PointerEvent): void => {
        if (!moved && Math.hypot(moveEvent.clientX - event.clientX, moveEvent.clientY - event.clientY) < POINTER_DRAG_THRESHOLD_PX) {
          return;
        }
        moved = true;
        const current = clientToSectionFraction(fractionContainer, moveEvent.clientX, moveEvent.clientY);
        setMarquee({ start, end: { x: current.x, y: current.y } });
      };

      const onUp = (upEvent: PointerEvent): void => {
        window.removeEventListener("pointermove", onMove);
        window.removeEventListener("pointerup", onUp);
        setMarquee(null);
        if (!moved) {
          if (!startedInOverview) {
            interaction.clearSelection();
          }
          return;
        }
        suppressRevealOverviewSlideNavigation(upEvent);
        const end = clientToSectionFraction(fractionContainer, upEvent.clientX, upEvent.clientY);
        const box = normalizeMarquee(start, end);
        const rule = marqueeSelectionRule(start, end);
        const hits: string[] = [];
        for (const targetId of dispositionIds) {
          const rect = resolveRectForId(targetId);
          if (rect && marqueeSelects(box, rect, rule)) {
            hits.push(targetId);
          }
        }
        interaction.selectIds(hits, upEvent.shiftKey);
      };

      window.addEventListener("pointermove", onMove);
      window.addEventListener("pointerup", onUp);
    },
    [canvasPlacement, declaredRects, dispositionIds, interaction, resolveRectForId, sectionRef],
  );

  return { onPointerDownCapture, marquee };
}

export const SlideInteractionReset: FC<{
  readonly sectionRef: RefObject<HTMLElement | null>;
  readonly dispositionIds: readonly string[];
  readonly declaredRects: ReadonlyMap<string, DispositionPosition | undefined>;
}> = ({ sectionRef, dispositionIds, declaredRects }) => {
  const interaction = usePresentationInteractionState();
  const hostRef = useRef<HTMLDivElement>(null);
  const modified = dispositionIds.some((id) => {
    if (interaction.isEnlarged(id)) {
      return true;
    }
    if (interaction.hasPdfPageOverride(id)) {
      return true;
    }
    const transform = interaction.getTransform(id);
    const declared = declaredRects.get(id);
    return dispositionHasEphemeralLayout(transform, declared, undefined, declared === undefined);
  });

  useEffect(() => {
    if (!modified) {
      return;
    }
    const section = sectionRef.current;
    const host = hostRef.current;
    if (!section || !host) {
      return;
    }
    const syncNear = (clientX: number, clientY: number): void => {
      host.classList.toggle("presentation-interaction-slide-reset-host--near", pointerNearSlideResetHotspot(section, clientX, clientY));
    };
    const onMove = (event: PointerEvent): void => {
      syncNear(event.clientX, event.clientY);
    };
    const onLeave = (): void => {
      host.classList.remove("presentation-interaction-slide-reset-host--near");
    };
    window.addEventListener("pointermove", onMove, { passive: true });
    section.addEventListener("pointerleave", onLeave);
    return () => {
      window.removeEventListener("pointermove", onMove);
      section.removeEventListener("pointerleave", onLeave);
      host.classList.remove("presentation-interaction-slide-reset-host--near");
    };
  }, [declaredRects, modified, sectionRef]);

  if (!modified) {
    return null;
  }

  return (
    <div ref={hostRef} className="presentation-interaction-slide-reset-host">
      <button
        type="button"
        className="presentation-interaction-slide-reset"
        title="Reset slide"
        onClick={(event) => {
          event.preventDefault();
          event.stopPropagation();
          interaction.resetSlide(dispositionIds);
        }}
      >
        <Icon icon="rotate-ccw" size="small" />
      </button>
    </div>
  );
};

export const InteractionLayer: FC<{
  readonly marquee: {
    readonly start: { readonly x: number; readonly y: number };
    readonly end: { readonly x: number; readonly y: number };
  } | null;
}> = ({ marquee }) => {
  const marqueeStyle: { readonly left: string; readonly top: string; readonly width: string; readonly height: string } | undefined = marquee
    ? (() => {
        const box = normalizeMarquee(marquee.start, marquee.end);
        return {
          left: `${box.x * 100}%`,
          top: `${box.y * 100}%`,
          width: `${box.width * 100}%`,
          height: `${box.height * 100}%`,
        };
      })()
    : undefined;

  const marqueeRule = marquee === null ? null : marqueeSelectionRule(marquee.start, marquee.end);

  return (
    <div className="presentation-interaction-layer" aria-hidden>
      {marquee && marqueeStyle ? (
        <SelectionMarquee
          coverage={marqueeRule === "crossing" ? "partial" : "full"}
          shape="rect"
          rect={{
            x: marqueeStyle.left,
            y: marqueeStyle.top,
            width: marqueeStyle.width,
            height: marqueeStyle.height,
          }}
        />
      ) : null}
    </div>
  );
};
//#endregion 🔖️Interaction
