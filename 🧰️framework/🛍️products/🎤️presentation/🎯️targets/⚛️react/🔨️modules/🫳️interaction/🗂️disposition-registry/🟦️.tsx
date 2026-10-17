/** 🗂️ The slide disposition registry: the measured natural rects of dispositions, its context, provider and the effective rect of a disposition. */
import type { DispositionPosition } from "@semio-tech/presentation";
import { createContext, useCallback, useContext, useMemo, useRef, useState, type ReactNode } from "@semio-tech/ui-react";
import { type PresentationInteractionState } from "../../🧮️interaction-model/🟦️.ts";

//#region 🔖️Interaction
interface SlideDispositionRegistry {
  readonly registerRect: (id: string, rect: DispositionPosition | null) => void;
  readonly getRect: (id: string) => DispositionPosition | undefined;
  readonly listEntries: () => readonly { readonly id: string; readonly rect: DispositionPosition }[];
}

const SlideDispositionRegistryContext = createContext<SlideDispositionRegistry | null>(null);

export function useSlideDispositionRegistry(): SlideDispositionRegistry {
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

export function resolveEffectiveDispositionRect(id: string, declared: DispositionPosition | undefined, interaction: PresentationInteractionState, registry: SlideDispositionRegistry): DispositionPosition | undefined {
  const transform = interaction.getTransform(id);
  if (transform) {
    return transform;
  }
  if (declared) {
    return declared;
  }
  return registry.getRect(id);
}
//#endregion 🔖️Interaction
