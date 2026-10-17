/** 🎛️ The presentation interaction hook: per-slide selection, transforms and enlarge flags, cleared only through reset controls. */
import type { DispositionPosition } from "@semio-tech/presentation";
import { useCallback, useMemo, useState } from "@semio-tech/ui-react";
import { dispositionPositionChanged, type DispositionTransform, type PresentationInteractionState } from "../../🧮️interaction-model/🟦️.ts";

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
//#endregion 🔖️Interaction
