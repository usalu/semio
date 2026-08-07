// #region 🧲️Header
// 💻️ framework/ui/elements/🫀️core/🌈️Surface/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ephemeralBox, ephemeralSet } from "@semio-tech/framework-core";
import * as React from "react";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { glassClass, surfaceClass, veilClass } from "../🏷️ClassNames/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🎈️Surface
export type Level = "base" | "window" | "pane" | "panel" | "dialog" | "menu";

/** @emoji 📚️ Every {@link Level}, ordered base..menu (Storybook/tests). */
export const LEVELS: readonly Level[] = ["base", "window", "pane", "panel", "dialog", "menu"] as const;

const LevelContext = reactHostPort.createContext<Level>("base");

/** @emoji 🎈️ Sets the current UI depth level for descendant chrome. */
export const LevelProvider: React.FC<{
  readonly level: Level;
  readonly children: React.ReactNode;
}> = ({ level, children }) => <LevelContext.Provider value={level}>{children}</LevelContext.Provider>;

/** @emoji 🪝️ Returns the nearest {@link LevelProvider} level. */
export function useLevel(): Level {
  return reactHostPort.useContext(LevelContext);
}

/** @emoji 🎨️ Tailwind z-index class for a {@link Level}. */
export function getLevelZClass(level: Level): string {
  switch (level) {
    case "window":
      return "z-window";
    case "pane":
      return "z-pane";
    case "panel":
      return "z-panel";
    case "dialog":
      return "z-dialog";
    case "menu":
      return "z-menu";
    default:
      return "z-base";
  }
}

/** @emoji 🎨️ Opaque per-level fill — background-color only, no blur (see `[data-level]` cascade in 🎨️ui.css). */

/** @emoji 🎨️ Whether a base-floor chrome row (navbar/footer/canvas/mode-body) must paint its own
 * {@link surfaceClass}, or stay transparent so Layout's one continuous base surface shows through.
 * Nested same-level paints are the "navbar ≠ canvas ≠ footer" bug class — one base floor, one fill. */
/** @emoji 🎨️ Fullscreen scrim; host element must carry `data-level="dialog"` for correct tint. */

/** @emoji 🪟️ Which fill a painted surface uses — maps 1:1 to {@link surfaceClass}/{@link glassClass}/{@link veilClass}. */
export type SurfaceFill = "surface" | "glass" | "veil";

/** @emoji 🎨️ Literal-safe fill lookup for a {@link SurfaceFill} (Tailwind's static scanner only finds complete literal class strings, never a `${}`-built name). */
export function surfaceFillClass(fill: SurfaceFill): string {
  switch (fill) {
    case "glass":
      return glassClass;
    case "veil":
      return veilClass;
    default:
      return surfaceClass;
  }
}

/** @emoji 🪟️ The nearest painted surface's level + fill, or `"none"` for a level root that intentionally defers painting to a descendant. */
export interface SurfaceScopeValue {
  readonly level: Level;
  readonly fill: SurfaceFill | "none";
}

const SurfaceContext = reactHostPort.createContext<SurfaceScopeValue | null>(null);

/** @emoji 🪟️ Opens a {@link LevelProvider} and records the level's fill for descendants — the "you
 * are already inside a painted surface" signal that {@link Surface} uses to warn on accidental
 * double-painting. Prefer {@link Surface} for a DOM-backed level root; use `SurfaceScope` directly
 * only when the level root isn't a plain `<div>` (e.g. `WindowChrome`, which stamps `data-level`
 * on its own stack element). */
export const SurfaceScope: React.FC<{
  readonly level: Level;
  readonly fill?: SurfaceFill | "none";
  readonly children: React.ReactNode;
}> = ({ level, fill = "none", children }) => (
  <LevelProvider level={level}>
    <SurfaceContext.Provider value={{ level, fill }}>{children}</SurfaceContext.Provider>
  </LevelProvider>
);

/** @emoji 🪝️ Returns the nearest {@link SurfaceScope}/{@link Surface} value, or `null` outside any. */
export function useSurface(): SurfaceScopeValue | null {
  return reactHostPort.useContext(SurfaceContext);
}

export interface SurfaceProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "className"> {
  readonly level: Level;
  readonly fill?: SurfaceFill | "none";
  readonly className?: string;
}

/** @emoji 🪟️ The one reusable "level root" primitive: stamps `data-level`, paints exactly one
 * {@link SurfaceFill}, and opens a {@link SurfaceScope} for its children. This is the enforcement
 * mechanism for "one level = one appearance" — a component that needs a new painted surface uses
 * `Surface` (or `SurfaceScope` + the fill class, for `WindowChrome`-style non-div roots) instead of
 * hand-composing `data-level` + a fill class, so nested same-level surfaces are structurally
 * visible via {@link useSurface} rather than silently drifting. Dev-only: warns when nested inside
 * an ancestor already painting the same level (that ancestor already painted — the "3 different
 * backgrounds in one dialog" bug class). */
export const Surface = reactHostPort.forwardRef<HTMLDivElement, SurfaceProps>(({ level, fill = "surface", className, children, ...rest }, ref) => {
  const parent = useSurface();
  if (process.env.NODE_ENV !== "production" && parent && parent.level === level && parent.fill !== "none" && fill !== "none") {
    console.warn(`Surface: nested "${level}" surface painted inside an ancestor Surface already painting "${level}" — one level must render one appearance. Pass fill="none" on the inner Surface, or remove it and let the ancestor's fill show through.`);
  }
  return (
    <div ref={ref} data-level={level} className={cn(fill === "none" ? undefined : surfaceFillClass(fill), className)} {...rest}>
      <SurfaceScope level={level} fill={fill}>
        {children}
      </SurfaceScope>
    </div>
  );
});
Surface.displayName = "Surface";

const surfaceActiveRoots = ephemeralSet<HTMLElement>("framework.modules.ui.elements.core.Surface.component.tsx.surfaceActiveRoots");
const surfaceActiveRoot = ephemeralBox<HTMLElement | null>("framework.modules.ui.elements.core.Surface.component.tsx.surfaceActiveRoot", null);
const surfaceActiveSubscribers = ephemeralSet<() => void>("framework.modules.ui.elements.core.Surface.component.tsx.surfaceActiveSubscribers");
const surfaceActiveListenersInstalled = ephemeralBox("framework.modules.ui.elements.core.Surface.component.tsx.surfaceActiveListenersInstalled", false);

/** @emoji 🎯️ Drops introduction stamps on an activated surface so the pulse cannot outrank the active stroke. */
function clearIntroducedStamps(root: HTMLElement): void {
  if (root.getAttribute("data-introduced") === "true") root.removeAttribute("data-introduced");
  root.querySelectorAll('[data-introduced="true"]').forEach((el) => el.removeAttribute("data-introduced"));
}

export function setSurfaceActiveRoot(next: HTMLElement | null): void {
  if (surfaceActiveRoot.current === next) return;
  if (next) clearIntroducedStamps(next);
  surfaceActiveRoot.current = next;
  surfaceActiveSubscribers.forEach((notify) => notify());
}

/** @emoji 🎯️ Silhouette gaps are holes onto the canvas, unless an explicit chrome chip is nested inside them. */
function isSurfaceActiveBackgroundTarget(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) return false;
  const gap = target.closest<HTMLElement>('[data-window-silhouette-gap]');
  if (!gap) return false;
  const chip = target.closest<HTMLElement>('[data-window-silhouette-chip]');
  return !chip || !gap.contains(chip);
}

/** @emoji 🎯️ Treats the visual cutout as canvas even when an app's absolute canvas extends beneath it and becomes the DOM pointer target. */
export function isSurfaceActiveBackgroundPointer(event: { readonly target: EventTarget | null; readonly clientX?: number; readonly clientY?: number }): boolean {
  if (isSurfaceActiveBackgroundTarget(event.target)) return true;
  if (typeof document === "undefined" || typeof event.clientX !== "number" || typeof event.clientY !== "number") return false;
  const target = event.target instanceof Element ? event.target : null;
  const stack = target?.closest<HTMLElement>('[data-slot="mode-dock-stack"]');
  const scope: Document | HTMLElement = stack ?? document;
  for (const gap of scope.querySelectorAll<HTMLElement>('[data-window-silhouette-gap]')) {
    if (stack && gap.closest('[data-slot="mode-dock-stack"]') !== stack) continue;
    const rect = gap.getBoundingClientRect();
    if (rect.width <= 0 || rect.height <= 0) continue;
    if (event.clientX >= rect.left && event.clientX <= rect.right && event.clientY >= rect.top && event.clientY <= rect.bottom) return true;
  }
  return false;
}

function resolveSurfaceActiveRoot(target: EventTarget | null): HTMLElement | null {
  if (isSurfaceActiveBackgroundTarget(target)) return null;
  if (!(target instanceof Element)) return null;
  let node: HTMLElement | null = target instanceof HTMLElement ? target : target.parentElement;
  let match: HTMLElement | null = null;
  while (node instanceof HTMLElement) {
    if (surfaceActiveRoots.has(node)) match = node;
    node = node.parentElement;
  }
  return match;
}

function installSurfaceActiveDocumentListeners(): void {
  if (surfaceActiveListenersInstalled.current || typeof document === "undefined") return;
  surfaceActiveListenersInstalled.current = true;
  const onPointerDown = (event: Event): void => {
    setSurfaceActiveRoot(isSurfaceActiveBackgroundPointer(event) ? null : resolveSurfaceActiveRoot(event.target));
  };
  const onFocusIn = (event: Event): void => {
    setSurfaceActiveRoot(resolveSurfaceActiveRoot(event.target));
  };
  document.addEventListener("pointerdown", onPointerDown);
  document.addEventListener("focusin", onFocusIn);
}

export interface SurfaceActiveBindProps {
  readonly onPointerDownCapture: (event: React.PointerEvent) => void;
  readonly onFocusCapture: (event: React.FocusEvent) => void;
}

/** @emoji 🎯️ True when this surface root was the last panel, pane, window stack, or introduction step to receive pointer or keyboard focus. */
export function useSurfaceActive(ref: React.RefObject<HTMLElement | null>): readonly [boolean, SurfaceActiveBindProps] {
  const [, bump] = reactHostPort.useState(0);
  reactHostPort.useLayoutEffect(() => {
    installSurfaceActiveDocumentListeners();
    const element = ref.current;
    if (!element) return;
    surfaceActiveRoots.add(element);
    const notify = (): void => bump((value) => value + 1);
    surfaceActiveSubscribers.add(notify);
    return () => {
      surfaceActiveRoots.delete(element);
      surfaceActiveSubscribers.delete(notify);
      // 🎯️ Effect re-runs every commit (ref may attach late). Defer the clear so a same-commit
      // re-register can reclaim the root before we drop the active stroke.
      queueMicrotask(() => {
        if (surfaceActiveRoot.current === element && !surfaceActiveRoots.has(element)) setSurfaceActiveRoot(null);
      });
    };
  });
  const bind = reactHostPort.useMemo<SurfaceActiveBindProps>(
    () => ({
      onPointerDownCapture: (event: React.PointerEvent) => {
        if (isSurfaceActiveBackgroundPointer(event)) {
          setSurfaceActiveRoot(null);
          return;
        }
        const root = ref.current;
        if (root) setSurfaceActiveRoot(root);
      },
      onFocusCapture: (event: React.FocusEvent) => {
        const root = ref.current;
        if (root && event.target instanceof Element && root.contains(event.target)) setSurfaceActiveRoot(root);
      },
    }),
    [ref],
  );
  return [ref.current !== null && surfaceActiveRoot.current === ref.current, bind] as const;
}
// #endregion 🎈️Surface
