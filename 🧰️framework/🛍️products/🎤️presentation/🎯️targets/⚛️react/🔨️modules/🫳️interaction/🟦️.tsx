/** 🫳️ The presentation interaction hook, the slide disposition registry, interactive dispositions and row bands, background interaction, reset and the interaction layer.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

import type { DispositionPosition } from "@semio-tech/presentation";
// #region 🔌️Adapters
import {
  Icon,
  SelectionMarquee,
  createContext,
  useCallback,
  useContext,
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type FC,
  type ReactNode,
  type RefObject
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { isRevealMorphCompanionOnly, type RevealResolvedDisposition } from "../🔀️reveal-morph/🟦️.ts";
import { isRevealSlideAutoAnimating, isSectionAutoAnimating } from "../✨️auto-animate/🟦️.ts";
import {
  MorphAnchorOnWrapperContext,
  PresentationDispositionEnlargeContext,
  PresentationFigureCropFrameContext,
  PresentationInteractiveDispositionIdContext,
  PresentationSlideEpochContext
} from "../🏷️morph-context/🟦️.ts";
import { morphFrameCssVars } from "../📐️figure-frames/🟦️.ts";
import { MorphDispositionView } from "../🎭️morph-views/🟦️.tsx";
import {
  clientToSectionFraction,
  DISPOSITION_RESIZE_HANDLES,
  dispositionFrameElement,
  dispositionHasEphemeralLayout,
  dispositionPlacementContainer,
  dispositionPositionChanged,
  type DispositionResizeHandle,
  type DispositionTransform,
  flowDispositionManipulationRect,
  flowDispositionOffsetStyle,
  flowDispositionReserveStyle,
  flowDragTransformElement,
  flowPixelOffsetToSectionRect,
  flowPointerDeltaToLocal,
  groupBoundingRect,
  interactiveDispositionChromeStyle,
  interactiveDispositionContentScale,
  interactiveDispositionContentScaleStyle,
  isFlowPixelOffsetTransform,
  isUsableMeasuredRect,
  marqueeSelectionRule,
  marqueeSelects,
  measureDispositionBoundsInContainer,
  measureDispositionBoundsInSection,
  measureElementRectInSection,
  normalizeMarquee,
  POINTER_DRAG_THRESHOLD_PX,
  pointerNearSlideResetHotspot,
  type PresentationInteractionState,
  resizeDispositionRect,
  resolveDispositionDragGroupIds,
  revealDeckInOverview,
  scaleRectWithinGroup,
  SLIDE_INTERACTIVE_ENLARGE_FRAME,
  suppressRevealOverviewSlideNavigation,
  transformFrameStyle,
  translateDispositionRect,
  usePresentationInteractionState
} from "../🧮️interaction-model/🟦️.ts";

//#region 🔖️Interaction
/** 🖱️ Per-slide selection, transforms, and enlarge flags; cleared only via reset controls. */
export function usePresentationInteraction(): PresentationInteractionState {
  const [selectedIds, setSelectedIds] = useState<ReadonlySet<string>>(() => new Set());
  const [transforms, setTransforms] = useState<ReadonlyMap<string, DispositionTransform>>(() => new Map());
  const [enlargedIds, setEnlargedIds] = useState<ReadonlySet<string>>(() => new Set());
  const [enlargeStashById, setEnlargeStashById] = useState<ReadonlyMap<string, DispositionPosition>>(() => new Map());
  const [pdfPageById, setPdfPageById] = useState<ReadonlyMap<string, number>>(() => new Map());

  const isSelected = useCallback((id: string) => selectedIds.has(id), [selectedIds]);

  const isEnlarged = useCallback((id: string) => enlargedIds.has(id), [enlargedIds]);

  const hasPdfPageOverride = useCallback((id: string) => pdfPageById.has(id), [pdfPageById]);

  const getPdfPage = useCallback((id: string, defaultPage: number) => pdfPageById.get(id) ?? defaultPage, [pdfPageById]);

  const setPdfPage = useCallback((id: string, page: number, defaultPage: number) => {
    setPdfPageById((previous) => {
      const had = previous.has(id);
      const same = page === defaultPage;
      if (!had && same) {
        return previous;
      }
      if (had && same) {
        const next = new Map(previous);
        next.delete(id);
        return next;
      }
      const next = new Map(previous);
      next.set(id, page);
      return next;
    });
  }, []);

  const getTransform = useCallback((id: string) => transforms.get(id), [transforms]);

  const getEnlargeStash = useCallback((id: string) => enlargeStashById.get(id), [enlargeStashById]);

  const stashEnlargeRect = useCallback((id: string, rect: DispositionPosition) => {
    setEnlargeStashById((previous) => {
      const existing = previous.get(id);
      if (existing && !dispositionPositionChanged(existing, rect)) {
        return previous;
      }
      const next = new Map(previous);
      next.set(id, rect);
      return next;
    });
  }, []);

  const clearEnlargeStash = useCallback((id: string) => {
    setEnlargeStashById((previous) => {
      if (!previous.has(id)) {
        return previous;
      }
      const next = new Map(previous);
      next.delete(id);
      return next;
    });
  }, []);

  const setTransform = useCallback((id: string, rect: DispositionTransform) => {
    setTransforms((previous) => {
      const next = new Map(previous);
      next.set(id, rect);
      return next;
    });
  }, []);

  const setTransformsBatch = useCallback((updates: ReadonlyMap<string, DispositionTransform>) => {
    setTransforms((previous) => {
      const next = new Map(previous);
      for (const [id, rect] of updates) {
        next.set(id, rect);
      }
      return next;
    });
  }, []);

  const selectIds = useCallback((ids: readonly string[], additive: boolean) => {
    setSelectedIds((previous) => {
      if (!additive) {
        return new Set(ids);
      }
      const next = new Set(previous);
      for (const id of ids) {
        if (next.has(id)) {
          next.delete(id);
        } else {
          next.add(id);
        }
      }
      return next;
    });
  }, []);

  const clearSelection = useCallback(() => {
    setSelectedIds(new Set());
  }, []);

  const toggleEnlarge = useCallback((id: string) => {
    setEnlargedIds((previous) => {
      const next = new Set(previous);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  }, []);

  const clearTransform = useCallback((id: string) => {
    setTransforms((previous) => {
      if (!previous.has(id)) {
        return previous;
      }
      const next = new Map(previous);
      next.delete(id);
      return next;
    });
  }, []);

  const resetSlide = useCallback((dispositionIds: readonly string[]) => {
    const idSet = new Set(dispositionIds);
    setSelectedIds((previous) => {
      const next = new Set(previous);
      for (const id of idSet) {
        next.delete(id);
      }
      return next;
    });
    setTransforms((previous) => {
      let changed = false;
      for (const id of idSet) {
        if (previous.has(id)) {
          changed = true;
          break;
        }
      }
      if (!changed) {
        return previous;
      }
      const next = new Map(previous);
      for (const id of idSet) {
        next.delete(id);
      }
      return next;
    });
    setEnlargedIds((previous) => {
      const next = new Set(previous);
      for (const id of idSet) {
        next.delete(id);
      }
      return next;
    });
    setEnlargeStashById((previous) => {
      let changed = false;
      for (const id of idSet) {
        if (previous.has(id)) {
          changed = true;
          break;
        }
      }
      if (!changed) {
        return previous;
      }
      const next = new Map(previous);
      for (const id of idSet) {
        next.delete(id);
      }
      return next;
    });
    setPdfPageById((previous) => {
      let changed = false;
      for (const id of idSet) {
        if (previous.has(id)) {
          changed = true;
          break;
        }
      }
      if (!changed) {
        return previous;
      }
      const next = new Map(previous);
      for (const id of idSet) {
        next.delete(id);
      }
      return next;
    });
  }, []);

  const clearEphemeralLayout = useCallback(() => {
    setSelectedIds(new Set());
    setTransforms(new Map());
    setEnlargedIds(new Set());
    setEnlargeStashById(new Map());
    setPdfPageById(new Map());
  }, []);

  return useMemo(
    () => ({
      selectedIds,
      transforms,
      enlargedIds,
      enlargeStashById,
      pdfPageById,
      isSelected,
      isEnlarged,
      hasPdfPageOverride,
      getTransform,
      getEnlargeStash,
      getPdfPage,
      setTransform,
      setTransforms: setTransformsBatch,
      setPdfPage,
      stashEnlargeRect,
      clearEnlargeStash,
      selectIds,
      clearSelection,
      toggleEnlarge,
      clearTransform,
      resetSlide,
      clearEphemeralLayout,
    }),
    [
      selectedIds,
      transforms,
      enlargedIds,
      enlargeStashById,
      pdfPageById,
      isSelected,
      isEnlarged,
      hasPdfPageOverride,
      getTransform,
      getEnlargeStash,
      getPdfPage,
      setTransform,
      setTransformsBatch,
      setPdfPage,
      stashEnlargeRect,
      clearEnlargeStash,
      selectIds,
      clearSelection,
      toggleEnlarge,
      clearTransform,
      resetSlide,
      clearEphemeralLayout,
    ],
  );
}

interface SlideDispositionRegistry {
  readonly registerRect: (id: string, rect: DispositionPosition | null) => void;
  readonly getRect: (id: string) => DispositionPosition | undefined;
  readonly listEntries: () => readonly { readonly id: string; readonly rect: DispositionPosition }[];
}

const SlideDispositionRegistryContext = createContext<SlideDispositionRegistry | null>(null);

function useSlideDispositionRegistry(): SlideDispositionRegistry {
  const value = useContext(SlideDispositionRegistryContext);
  if (!value) {
    throw new Error("Slide disposition registry requires SlideDispositionRegistryContext.");
  }
  return value;
}

export function SlideDispositionRegistryProvider({ children }: { readonly children: ReactNode }): ReactNode {
  const measuredRectsRef = useRef<Map<string, DispositionPosition>>(new Map());
  const [, bump] = useState(0);

  const registerRect = useCallback((id: string, rect: DispositionPosition | null) => {
    const map = measuredRectsRef.current;
    if (rect === null) {
      if (map.delete(id)) {
        bump((value) => value + 1);
      }
      return;
    }
    const existing = map.get(id);
    if (existing && Math.abs(existing.x - rect.x) < 1e-5 && Math.abs(existing.y - rect.y) < 1e-5 && Math.abs(existing.width - rect.width) < 1e-5 && Math.abs(existing.height - rect.height) < 1e-5) {
      return;
    }
    map.set(id, rect);
    bump((value) => value + 1);
  }, []);

  const getRect = useCallback((id: string) => measuredRectsRef.current.get(id), []);

  const listEntries = useCallback(() => {
    return [...measuredRectsRef.current.entries()].map(([id, rect]) => ({ id, rect }));
  }, []);

  const registry = useMemo(
    () => ({
      registerRect,
      getRect,
      listEntries,
    }),
    [registerRect, getRect, listEntries],
  );

  return <SlideDispositionRegistryContext.Provider value={registry}>{children}</SlideDispositionRegistryContext.Provider>;
}

function resolveEffectiveDispositionRect(id: string, declared: DispositionPosition | undefined, interaction: PresentationInteractionState, registry: SlideDispositionRegistry): DispositionPosition | undefined {
  const transform = interaction.getTransform(id);
  if (transform) {
    return transform;
  }
  if (declared) {
    return declared;
  }
  return registry.getRect(id);
}

export const InteractiveRowBand: FC<{
  readonly id: string;
  readonly frame: DispositionPosition;
  readonly tileIds: readonly string[];
}> = ({ id, frame, tileIds }) => {
  const interaction = usePresentationInteractionState();
  const rowSelected = tileIds.length > 0 && tileIds.every((tileId) => interaction.isSelected(tileId));
  const onPointerDown = (event: React.PointerEvent): void => {
    if (event.button !== 0) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    interaction.selectIds(tileIds, event.shiftKey);
  };
  return (
    <div
      className={["presentation-interactive-row-band", rowSelected ? "presentation-interactive-row-band--selected" : undefined].filter(Boolean).join(" ")}
      data-row-band={id}
      style={transformFrameStyle(frame)}
      onPointerDown={onPointerDown}
      aria-hidden
    />
  );
};

export const InteractiveDisposition: FC<{
  readonly id: string;
  readonly disposition: RevealResolvedDisposition;
  readonly declaredRect: DispositionPosition | undefined;
  readonly sectionDeclaredRect: DispositionPosition | undefined;
  readonly allDeclaredRects: ReadonlyMap<string, DispositionPosition | undefined>;
  readonly sectionRef: RefObject<HTMLElement | null>;
  readonly revealMorphId?: string;
  readonly rowBandId?: string;
}> = ({ id, disposition, declaredRect, sectionDeclaredRect, allDeclaredRects, sectionRef, revealMorphId, rowBandId }) => {
  const slideEpoch = useContext(PresentationSlideEpochContext);
  const interaction = usePresentationInteractionState();
  const registry = useSlideDispositionRegistry();
  const rootRef = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);
  const selected = interaction.isSelected(id);
  const interactionRect = sectionDeclaredRect ?? declaredRect;
  const flowLayout = interactionRect === undefined;
  const transform = interaction.getTransform(id);
  const transformed = transform !== undefined;
  const measuredNatural = registry.getRect(id);
  const flowPixelOffset = transformed && flowLayout && transform !== undefined && isFlowPixelOffsetTransform(transform, measuredNatural);
  const flowSectionFrame = transformed && flowLayout && !flowPixelOffset;
  const enlarged = interaction.isEnlarged(id);
  const morphGhost = isRevealMorphCompanionOnly(disposition);
  const canvasFramed = declaredRect !== undefined && !enlarged;
  const canvasPlacement = interactionRect !== undefined;
  const effectiveRect = resolveEffectiveDispositionRect(id, interactionRect, interaction, registry);
  const canvasAnchorRect = interactionRect;
  const canvasLiveTransform = canvasFramed && transform && canvasAnchorRect ? transform : undefined;
  const canvasDragActive = Boolean(canvasLiveTransform && canvasAnchorRect && !enlarged && dispositionPositionChanged(canvasAnchorRect, canvasLiveTransform));
  const pinned = !enlarged && ((transformed && !flowLayout && !canvasFramed) || flowSectionFrame || (canvasFramed && canvasDragActive && !flowPixelOffset));
  const [gesturing, setGesturing] = useState(false);
  const [flowReservePx, setFlowReservePx] = useState<{ readonly width: number; readonly height: number } | null>(null);
  const useFlowInkFrame = flowLayout && !flowSectionFrame;
  const [inkInWrapper, setInkInWrapper] = useState<DispositionPosition | null>(null);
  const [enlargedFlowContentScale, setEnlargedFlowContentScale] = useState<number | null>(null);
  const displayDisposition =
    enlarged && disposition.position !== undefined
      ? disposition.embodiment.kind === "figure" && disposition.embodiment.crop !== undefined
        ? { ...disposition, position: SLIDE_INTERACTIVE_ENLARGE_FRAME }
        : { ...disposition, position: undefined }
      : disposition;

  const measureDispositionRect = useCallback((): DispositionPosition | null => {
    const section = sectionRef.current;
    const root = rootRef.current;
    if (!root || !section) {
      return null;
    }
    const container = dispositionPlacementContainer(section, canvasPlacement);
    const measured = canvasPlacement ? measureElementRectInSection(dispositionFrameElement(root), container) : measureDispositionBoundsInSection(root, section);
    if (!measured || !isUsableMeasuredRect(measured)) {
      return null;
    }
    return measured;
  }, [canvasPlacement, sectionRef]);

  useLayoutEffect(() => {
    if (!transform) {
      setFlowReservePx(null);
    }
  }, [transform]);

  useLayoutEffect(() => {
    if (interactionRect) {
      registry.registerRect(id, null);
      return;
    }
    if (transform) {
      return;
    }
    const section = sectionRef.current;
    if (!section || !section.classList.contains("present") || isRevealSlideAutoAnimating(section)) {
      registry.registerRect(id, null);
      return;
    }
    const measured = measureDispositionRect();
    registry.registerRect(id, measured);
  }, [id, interactionRect, transform, registry, sectionRef, slideEpoch, measureDispositionRect]);

  useLayoutEffect(() => {
    if (!useFlowInkFrame || !(selected || gesturing || flowPixelOffset)) {
      setInkInWrapper(null);
      return;
    }
    if (gesturing) {
      return;
    }
    const section = sectionRef.current;
    if (section && isRevealSlideAutoAnimating(section)) {
      return;
    }
    const root = rootRef.current;
    if (!root) {
      setInkInWrapper(null);
      return;
    }
    setInkInWrapper(measureDispositionBoundsInContainer(root, root));
  }, [useFlowInkFrame, selected, gesturing, flowPixelOffset, transform, slideEpoch, disposition, sectionRef]);

  useLayoutEffect(() => {
    if (!enlarged || !flowLayout) {
      setEnlargedFlowContentScale(null);
      return;
    }
    const section = sectionRef.current;
    const root = rootRef.current;
    if (!section || !root || !section.classList.contains("present")) {
      setEnlargedFlowContentScale(null);
      return;
    }
    const baseline = measureDispositionBoundsInSection(root, section);
    if (!baseline) {
      setEnlargedFlowContentScale(null);
      return;
    }
    setEnlargedFlowContentScale(interactiveDispositionContentScale(SLIDE_INTERACTIVE_ENLARGE_FRAME, baseline));
  }, [enlarged, flowLayout, slideEpoch, sectionRef]);

  const ensureRectForManipulation = useCallback(
    (kind: "move" | "resize"): DispositionPosition | null => {
      const section = sectionRef.current;
      const existing = interaction.getTransform(id);
      const measuredNatural = registry.getRect(id) ?? measureDispositionRect();
      if (interactionRect) {
        return existing ?? interactionRect;
      }
      if (kind === "resize") {
        if (!measuredNatural) {
          return null;
        }
        registry.registerRect(id, measuredNatural);
        if (existing && section && isFlowPixelOffsetTransform(existing, measuredNatural)) {
          return flowPixelOffsetToSectionRect(measuredNatural, existing, section, flowDragTransformElement(section, rootRef.current, contentRef.current));
        }
        return existing ?? measuredNatural;
      }
      if (existing) {
        return existing;
      }
      if (!measuredNatural) {
        return null;
      }
      registry.registerRect(id, measuredNatural);
      return flowDispositionManipulationRect(measuredNatural, undefined);
    },
    [id, interactionRect, interaction, registry, measureDispositionRect, sectionRef],
  );

  const attachPointerGesture = useCallback(
    (
      origin: {
        readonly pointerId: number;
        readonly clientX: number;
        readonly clientY: number;
        readonly captureEl: HTMLElement | null;
      },
      mode: "move" | DispositionResizeHandle,
      initialRect: DispositionPosition,
      requireDragThreshold: boolean,
      dragMemberIds: readonly string[],
    ): void => {
      const section = sectionRef.current;
      if (!section) {
        return;
      }
      const fractionContainer = dispositionPlacementContainer(section, canvasPlacement);
      const gesturePlacementBounds = fractionContainer.getBoundingClientRect();
      const pointerId = origin.pointerId;
      const startClient = { x: origin.clientX, y: origin.clientY };
      const groupIds = dragMemberIds.length > 1 ? [...dragMemberIds] : [id];
      const startRects = new Map<string, DispositionPosition>();
      for (const memberId of groupIds) {
        const memberDeclared = allDeclaredRects.get(memberId);
        const natural = memberId === id ? initialRect : registry.getRect(memberId);
        let rect: DispositionPosition | undefined = interaction.getTransform(memberId);
        if (!rect) {
          if (flowLayout && natural) {
            rect = flowDispositionManipulationRect(natural, undefined);
          } else if (natural) {
            rect = memberDeclared !== undefined ? natural : flowDispositionManipulationRect(natural, undefined);
          } else {
            rect = memberDeclared;
          }
        } else if (flowLayout && natural) {
          rect = isFlowPixelOffsetTransform(rect, natural) ? rect : flowDispositionManipulationRect(natural, undefined);
        }
        if (rect && (flowLayout || memberDeclared !== undefined || isUsableMeasuredRect(rect))) {
          startRects.set(memberId, rect);
        }
      }
      if (startRects.size === 0) {
        startRects.set(id, initialRect);
      }
      const startGroup = mode !== "move" && groupIds.length > 1 ? groupBoundingRect([...startRects.values()]) : null;
      let dragging = !requireDragThreshold;
      setGesturing(true);
      const flowTransformEl = flowDragTransformElement(section, rootRef.current, contentRef.current);
      if (flowLayout) {
        const root = rootRef.current;
        if (root && root.offsetWidth > 0 && root.offsetHeight > 0) {
          setFlowReservePx({ width: root.offsetWidth, height: root.offsetHeight });
        }
      }

      const onMove = (moveEvent: PointerEvent): void => {
        if (moveEvent.pointerId !== pointerId) {
          return;
        }
        if (!dragging) {
          if (Math.hypot(moveEvent.clientX - startClient.x, moveEvent.clientY - startClient.y) < POINTER_DRAG_THRESHOLD_PX) {
            return;
          }
          dragging = true;
        }
        moveEvent.preventDefault();
        const updates = new Map<string, DispositionTransform>();
        if (mode === "move") {
          for (const [memberId, rect] of startRects) {
            if (flowLayout) {
              const { dx, dy } = flowPointerDeltaToLocal(flowTransformEl, startClient.x, startClient.y, moveEvent.clientX, moveEvent.clientY);
              updates.set(memberId, {
                x: rect.x + dx,
                y: rect.y + dy,
                width: rect.width,
                height: rect.height,
              });
            } else {
              const container = dispositionPlacementContainer(section, allDeclaredRects.get(memberId) !== undefined);
              const bounds = container === fractionContainer && gesturePlacementBounds.width > 0 ? gesturePlacementBounds : container.getBoundingClientRect();
              const dx = bounds.width > 0 ? (moveEvent.clientX - startClient.x) / bounds.width : 0;
              const dy = bounds.height > 0 ? (moveEvent.clientY - startClient.y) / bounds.height : 0;
              updates.set(memberId, translateDispositionRect(rect, dx, dy));
            }
          }
        } else {
          const bounds = gesturePlacementBounds.width > 0 ? gesturePlacementBounds : fractionContainer.getBoundingClientRect();
          const dx = bounds.width > 0 ? (moveEvent.clientX - startClient.x) / bounds.width : 0;
          const dy = bounds.height > 0 ? (moveEvent.clientY - startClient.y) / bounds.height : 0;
          if (startGroup && groupIds.length > 1) {
            const resizedGroup = resizeDispositionRect(startGroup, mode, dx, dy);
            for (const [memberId, rect] of startRects) {
              updates.set(memberId, scaleRectWithinGroup(rect, startGroup, resizedGroup));
            }
          } else {
            const startRect = startRects.get(id) ?? initialRect;
            updates.set(id, resizeDispositionRect(startRect, mode, dx, dy));
          }
        }
        interaction.setTransforms(updates);
      };

      const onUp = (upEvent: PointerEvent): void => {
        if (upEvent.pointerId !== pointerId) {
          return;
        }
        if (dragging) {
          suppressRevealOverviewSlideNavigation(upEvent);
        }
        setGesturing(false);
        window.removeEventListener("pointermove", onMove);
        window.removeEventListener("pointerup", onUp);
        window.removeEventListener("pointercancel", onUp);
        try {
          origin.captureEl?.releasePointerCapture?.(pointerId);
        } catch {
          // jsdom may not support pointer capture
        }
      };

      try {
        origin.captureEl?.setPointerCapture?.(pointerId);
      } catch {
        // jsdom may not support pointer capture
      }
      window.addEventListener("pointermove", onMove);
      window.addEventListener("pointerup", onUp);
      window.addEventListener("pointercancel", onUp);
    },
    [id, flowLayout, canvasPlacement, allDeclaredRects, interaction, registry, sectionRef],
  );

  const seedCanvasTransform = useCallback((): DispositionPosition | null => {
    if (!interactionRect) {
      return null;
    }
    const existing = interaction.getTransform(id);
    if (existing) {
      return existing;
    }
    interaction.setTransform(id, interactionRect);
    return interactionRect;
  }, [id, interactionRect, interaction]);

  const onPointerDown = (event: React.PointerEvent): void => {
    if (event.button !== 0) {
      return;
    }
    const target = event.target instanceof Element ? event.target : null;
    if (target?.closest(".presentation-interaction-handle")) {
      return;
    }
    if (target?.closest(".presentation-interaction-enlarge")) {
      return;
    }
    if (target?.closest(".presentation-interaction-reset")) {
      return;
    }
    if (target?.closest(".presentation-pdf-page-nav")) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    const additive = event.shiftKey;
    const dragMemberIds = resolveDispositionDragGroupIds(id, selected, additive, interaction.selectedIds);
    if (!selected) {
      interaction.selectIds([id], additive);
    } else if (additive) {
      interaction.selectIds([id], true);
    }
    seedCanvasTransform();
    const rect = ensureRectForManipulation("move");
    if (!rect) {
      return;
    }
    attachPointerGesture(
      {
        pointerId: event.pointerId,
        clientX: event.clientX,
        clientY: event.clientY,
        captureEl: rootRef.current,
      },
      "move",
      rect,
      true,
      dragMemberIds,
    );
  };

  const onHandlePointerDown = (handle: DispositionResizeHandle) => (event: React.PointerEvent) => {
    if (event.button !== 0) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    const dragMemberIds = resolveDispositionDragGroupIds(id, selected, false, interaction.selectedIds);
    if (!selected) {
      interaction.selectIds([id], false);
    }
    seedCanvasTransform();
    const rect = ensureRectForManipulation("resize");
    if (!rect) {
      return;
    }
    attachPointerGesture(
      {
        pointerId: event.pointerId,
        clientX: event.clientX,
        clientY: event.clientY,
        captureEl: event.currentTarget as HTMLElement,
      },
      handle,
      rect,
      false,
      dragMemberIds,
    );
  };

  const onEnlargeClick = (event: React.MouseEvent): void => {
    event.preventDefault();
    event.stopPropagation();
    if (!selected) {
      interaction.selectIds([id], false);
    }
    if (enlarged) {
      const stashed = interaction.getEnlargeStash(id);
      interaction.toggleEnlarge(id);
      interaction.clearEnlargeStash(id);
      if (stashed) {
        if (interactionRect && !dispositionPositionChanged(interactionRect, stashed)) {
          interaction.clearTransform(id);
        } else {
          interaction.setTransform(id, stashed);
        }
      }
      return;
    }
    const preEnlargeRect = effectiveRect ?? interactionRect;
    if (preEnlargeRect) {
      interaction.stashEnlargeRect(id, preEnlargeRect);
    }
    interaction.toggleEnlarge(id);
  };

  const onResetClick = (event: React.MouseEvent): void => {
    event.preventDefault();
    event.stopPropagation();
    if (!selected) {
      interaction.selectIds([id], false);
    }
    if (enlarged) {
      const stashed = interaction.getEnlargeStash(id);
      interaction.toggleEnlarge(id);
      interaction.clearEnlargeStash(id);
      if (stashed && interactionRect && !dispositionPositionChanged(interactionRect, stashed)) {
        interaction.clearTransform(id);
      } else if (stashed) {
        interaction.setTransform(id, stashed);
      } else {
        interaction.clearTransform(id);
      }
      return;
    }
    interaction.clearTransform(id);
  };

  const morphCropFrom = disposition.revealMorphCompanion === "target" && disposition.revealMorphFromFrame !== undefined && disposition.position !== undefined;
  const morphCropTo = disposition.position !== undefined && (disposition.revealMorphFromMorphToFrame !== undefined || disposition.revealMorphToFrame !== undefined);
  const wrapperClass = [
    "presentation-interactive-disposition",
    `presentation-interactive-disposition--kind-${disposition.embodiment.kind}`,
    selected ? "presentation-interactive-disposition--selected" : undefined,
    flowPixelOffset ? "presentation-interactive-disposition--offset" : undefined,
    pinned ? "presentation-interactive-disposition--pinned" : undefined,
    canvasFramed ? "presentation-interactive-disposition--canvas-framed" : undefined,
    gesturing ? "presentation-interactive-disposition--gesturing" : undefined,
    enlarged ? "presentation-interactive-disposition--enlarged" : undefined,
    disposition.revealMorphCompanion === "target" ? "presentation-target-ghost" : undefined,
    disposition.revealMorphCompanion === "source" ? "presentation-source-ghost" : undefined,
    morphCropFrom ? "presentation-morph-crop-from" : undefined,
    morphCropTo ? "presentation-morph-crop-to" : undefined,
    (disposition.morphFrom?.length ?? 0) > 0 ? "presentation-morph-target" : undefined,
    (disposition.morphTo?.length ?? 0) > 0 ? "presentation-morph-one" : undefined,
  ]
    .filter(Boolean)
    .join(" ");

  const flowInkActive = useFlowInkFrame && (selected || gesturing || flowPixelOffset);
  const wrapperFrame: CSSProperties = flowInkActive ? { position: "relative" } : {};
  if (flowReservePx) {
    Object.assign(wrapperFrame, flowDispositionReserveStyle(flowReservePx));
  }
  const flowDragOffsetStyle = flowPixelOffset && transform ? flowDispositionOffsetStyle(transform) : undefined;
  if (flowDragOffsetStyle) {
    Object.assign(wrapperFrame, flowDragOffsetStyle);
  }
  if (canvasFramed && canvasAnchorRect) {
    // 🔀️ The wrapper owns the reveal `data-id` morph anchor; placing it on the live ephemeral
    // rect (drag/resize) makes reveal.js auto-animate capture the modified frame as the morph
    // `from`, so morphs start from the current disposition including ephemeral modifications.
    // 👻️ Target ghosts keep label layout; live source frames are synced from slide 8 DOM before morph.
    const morphAnchorRect = disposition.revealMorphCompanion === "target" ? canvasAnchorRect : disposition.revealMorphCompanion === "source" ? canvasAnchorRect : (canvasLiveTransform ?? canvasAnchorRect);
    Object.assign(wrapperFrame, transformFrameStyle(morphAnchorRect));
    if (morphCropFrom && disposition.revealMorphFromFrame !== undefined && disposition.position !== undefined) {
      for (const [key, value] of Object.entries(morphFrameCssVars(disposition.revealMorphFromFrame, disposition.position))) {
        if (typeof value === "string") {
          (wrapperFrame as Record<string, string>)[key] = value;
        }
      }
    }
  } else if (transformed && transform && !flowPixelOffset) {
    Object.assign(wrapperFrame, transformFrameStyle(transform));
  }
  if (morphGhost) {
    Object.assign(wrapperFrame, { pointerEvents: "none", zIndex: 0 });
  }
  const wrapperStyle: CSSProperties | undefined = enlarged ? transformFrameStyle(SLIDE_INTERACTIVE_ENLARGE_FRAME) : Object.keys(wrapperFrame).length > 0 ? wrapperFrame : undefined;
  const resizeContentBaseline = measuredNatural ?? interactionRect;
  const resizeContentScale = !enlarged && !canvasFramed && !flowPixelOffset && transform && resizeContentBaseline ? interactiveDispositionContentScale(transform, resizeContentBaseline) : null;
  const contentScale = enlargedFlowContentScale ?? resizeContentScale;
  const contentStyle: CSSProperties | undefined = contentScale !== null ? interactiveDispositionContentScaleStyle(contentScale) : undefined;
  const hasContentStyle = contentStyle !== undefined;
  const chromeLayoutRect = flowSectionFrame && transform ? transform : (canvasLiveTransform ?? effectiveRect);
  const sectionAnimating = sectionRef.current !== null && isSectionAutoAnimating(sectionRef.current);
  const flowInkChromeRect = useFlowInkFrame ? inkInWrapper : undefined;
  const chromeStyle: CSSProperties | undefined = interactiveDispositionChromeStyle({
    selected: (selected || gesturing) && !sectionAnimating,
    effectiveRect: flowInkChromeRect ?? chromeLayoutRect,
    canvasFramed,
    enlarged,
  });
  const showControls = !morphGhost && (selected || gesturing || enlarged) && Boolean(enlarged || selected || gesturing || (useFlowInkFrame ? inkInWrapper : (chromeLayoutRect ?? effectiveRect)));
  const showHandles = showControls && !enlarged;
  const canResetPosition = enlarged || dispositionHasEphemeralLayout(transform, interactionRect, measuredNatural, flowLayout);

  return (
    <div
      ref={rootRef}
      data-disposition-id={id}
      {...(revealMorphId ? { "data-id": revealMorphId } : {})}
      {...(rowBandId ? { "data-row-band": rowBandId } : {})}
      className={wrapperClass}
      style={wrapperStyle}
      onPointerDown={morphGhost ? undefined : onPointerDown}
      aria-hidden={morphGhost ? true : undefined}
    >
      <div ref={contentRef} className="presentation-interactive-disposition__content" style={hasContentStyle ? contentStyle : undefined}>
        <PresentationInteractiveDispositionIdContext.Provider value={id}>
          <PresentationDispositionEnlargeContext.Provider value={enlarged}>
            <MorphAnchorOnWrapperContext.Provider value={Boolean(revealMorphId && declaredRect !== undefined)}>
              <PresentationFigureCropFrameContext.Provider value={canvasLiveTransform ?? displayDisposition.position}>
                <MorphDispositionView disposition={displayDisposition} />
              </PresentationFigureCropFrameContext.Provider>
            </MorphAnchorOnWrapperContext.Provider>
          </PresentationDispositionEnlargeContext.Provider>
        </PresentationInteractiveDispositionIdContext.Provider>
        {showHandles ? (
          <div className="presentation-interactive-disposition__chrome" style={chromeStyle} aria-hidden>
            {DISPOSITION_RESIZE_HANDLES.map((handle) => (
              <div key={handle} className={`presentation-interaction-handle presentation-interaction-handle--${handle}`} onPointerDown={onHandlePointerDown(handle)} />
            ))}
          </div>
        ) : null}
      </div>
      {showControls ? (
        <div className="presentation-interaction-actions">
          {canResetPosition ? (
            <button type="button" className="presentation-interaction-reset" title="Reset position" onClick={onResetClick}>
              <Icon icon="rotate-ccw" size="small" />
            </button>
          ) : null}
          <button type="button" className="presentation-interaction-enlarge" title={enlarged ? "Exit enlarge" : "Enlarge"} aria-pressed={enlarged} onClick={onEnlargeClick}>
            <Icon icon="maximize-2" size="small" />
          </button>
        </div>
      ) : null}
    </div>
  );
};

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
