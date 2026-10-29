/** 🧭️ What every page of the archive shares: the reader's language and translator, the catalog, the local store, the motion decision and the ambient clock.
 *
 * @see ../🌐️i18n/🟦️.ts — languages
 * @see ../📚️catalog/🟦️.ts — the catalog
 * @see ../💾️persistence/🟦️.ts — the local store
 */
import { createContext, useContext, useEffect, useMemo, useState, type ReactElement, type ReactNode } from "react";
import type { Language } from "@teaching/staffage";
import { StaffageClock } from "../⏱️clock/🟦️.ts";
import { type Translate } from "../🌐️i18n/🟦️.ts";
import { CatalogStore, useCatalog, type CatalogSnapshot } from "../📚️catalog/🟦️.ts";
import { ArchiveStore, useArchiveState, type ArchiveState } from "../💾️persistence/🟦️.ts";

/** 🧭️ The shared services of the app. */
export type Archive = {
  readonly language: Language;
  readonly t: Translate;
  readonly catalog: CatalogStore;
  readonly store: ArchiveStore;
  readonly reducedMotion: boolean;
  readonly ambient: StaffageClock;
};

const ArchiveContext = createContext<Archive | null>(null);

/** 🎞️ Whether the device asks for reduced motion. */
export function usePrefersReducedMotion(): boolean {
  const [reduced, setReduced] = useState(() => typeof matchMedia === "function" && matchMedia("(prefers-reduced-motion: reduce)").matches);
  useEffect(() => {
    if (typeof matchMedia !== "function") return;
    const query = matchMedia("(prefers-reduced-motion: reduce)");
    const update = (): void => setReduced(query.matches);
    query.addEventListener("change", update);
    return () => query.removeEventListener("change", update);
  }, []);
  return reduced;
}

/** 🎞️ The ambient clock rate of previews, frames per second. */
export const AMBIENT_FPS = 12;

/** 🧭️ Provides the shared services to a subtree. */
export function ArchiveProvider(props: { readonly language: Language; readonly t: Translate; readonly catalog: CatalogStore; readonly store: ArchiveStore; readonly children: ReactNode }): ReactElement {
  const { language, t, catalog, store, children } = props;
  const { preferences } = useArchiveState(store);
  const device = usePrefersReducedMotion();
  const reducedMotion = preferences.motion === "reduced" || (preferences.motion === "system" && device);
  const ambient = useMemo(() => new StaffageClock({ fps: AMBIENT_FPS, playing: false, reducedMotion: true }), []);
  useEffect(() => {
    if (reducedMotion) ambient.pause();
    else ambient.play();
    const onVisibility = (): void => (document.hidden ? ambient.pause() : reducedMotion ? undefined : ambient.play());
    document.addEventListener("visibilitychange", onVisibility);
    return () => document.removeEventListener("visibilitychange", onVisibility);
  }, [ambient, reducedMotion]);
  useEffect(() => () => ambient.dispose(), [ambient]);
  const value = useMemo<Archive>(() => ({ language, t, catalog, store, reducedMotion, ambient }), [language, t, catalog, store, reducedMotion, ambient]);
  return <ArchiveContext.Provider value={value}>{children}</ArchiveContext.Provider>;
}

/** 🪝️ The shared services. */
export function useArchive(): Archive {
  const value = useContext(ArchiveContext);
  if (value === null) throw new Error("useArchive needs an ArchiveProvider");
  return value;
}

/** 🪝️ The catalog snapshot. */
export function useRows(): CatalogSnapshot {
  return useCatalog(useArchive().catalog);
}

/** 🪝️ The persisted state. */
export function useLocal(): ArchiveState {
  return useArchiveState(useArchive().store);
}
