// #region 🧲️Header
// 💻️ framework/ui/elements/🫀️core/🌈️Surface/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ephemeralBox, ephemeralSet } from "@semio-tech/framework-transient";
import * as React from "react";
import { reactHostPort } from "../🔌️Ports/⚛️react/🟦️.ts";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { glassClass, surfaceClass, veilClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🎈️Surface
export type Level = "base" | "window" | "pane" | "panel" | "dialog" | "menu";

/** 📚️ Every {@link Level}, ordered base..menu (Storybook/tests). */
export const LEVELS: readonly Level[] = ["base", "window", "pane", "panel", "dialog", "menu"] as const;

/** 🪜️ The surface immediately above its owner, capped at the menu level. */
export function surfaceLevelAbove(level: Level): Level {
  return LEVELS[Math.min(LEVELS.indexOf(level) + 1, LEVELS.length - 1)]!;
}

const LevelContext = reactHostPort.createContext<Level>("base");

/** 🎈️ Sets the current UI depth level for descendant chrome. */
export const LevelProvider: React.FC<{
  readonly level: Level;
  readonly children: React.ReactNode;
}> = ({ level, children }) => <LevelContext.Provider value={level}>{children}</LevelContext.Provider>;

/** 🪝️ Returns the nearest {@link LevelProvider} level. */
export function useLevel(): Level {
  return reactHostPort.useContext(LevelContext);
}

/** 🪜️ Tailwind z-index class for a {@link Level}. */
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

/** 🪟️ Which fill a painted surface uses — maps 1:1 to {@link surfaceClass}/{@link glassClass}/{@link veilClass}. */
export type SurfaceFill = "surface" | "glass" | "veil";

/** 🔎️ Literal-safe fill lookup for a {@link SurfaceFill} (Tailwind's static scanner only finds complete literal class strings, never a `${}`-built name). */
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

/** 🏷️ The nearest painted surface's level + fill, or `"none"` for a level root that intentionally defers painting to a descendant. */
export interface SurfaceScopeValue {
  readonly level: Level;
  readonly fill: SurfaceFill | "none";
}

const SurfaceContext = reactHostPort.createContext<SurfaceScopeValue | null>(null);

/** 🔓️ Records the level and fill owned by a frame, overlay or other non-div surface root. */
export const SurfaceScope: React.FC<{
  readonly level: Level;
  readonly fill?: SurfaceFill | "none";
  readonly children: React.ReactNode;
}> = ({ level, fill = "none", children }) => (
  <LevelProvider level={level}>
    <SurfaceContext.Provider value={{ level, fill }}>{children}</SurfaceContext.Provider>
  </LevelProvider>
);

/** 🔗️ Returns the nearest {@link SurfaceScope}/{@link Surface} value, or `null` outside any. */
export function useSurface(): SurfaceScopeValue | null {
  return reactHostPort.useContext(SurfaceContext);
}

export interface SurfaceProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "className"> {
  readonly level?: Level | "chip";
  readonly fill?: SurfaceFill | "none";
  readonly overlay?: boolean;
  readonly className?: string;
}

/** 🏠️ Shares a painted owner with in-flow descendants. Independent overlays can paint at the same level. */
export const Surface = reactHostPort.forwardRef<HTMLDivElement, SurfaceProps>(({ level, fill = level === "chip" ? "glass" : "surface", overlay = false, className, children, ...rest }, ref) => {
  const parent = useSurface();
  const ambient = useLevel();
  const resolved = level === "chip" ? surfaceLevelAbove(ambient) : level ?? ambient;
  const sharesOwner = level !== "chip" && !overlay && parent?.level === resolved && parent.fill !== "none";
  return (
    <div ref={ref} data-level={resolved} data-glass-chip={level === "chip" ? "" : undefined} className={cn(sharesOwner || fill === "none" ? "bg-transparent" : surfaceFillClass(fill), className)} {...rest}>
      <SurfaceScope level={resolved} fill={sharesOwner ? parent.fill : fill}>
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

/** 🧹️ Drops introduction stamps on an activated surface so the pulse cannot outrank the active stroke. */
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

function resolveSurfaceActiveRoot(target: EventTarget | null): HTMLElement | null {
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
    setSurfaceActiveRoot(resolveSurfaceActiveRoot(event.target));
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

/** 🔚️ True when this surface root was the last panel, pane, window stack, or introduction step to receive pointer or keyboard focus. */
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
      queueMicrotask(() => {
        if (surfaceActiveRoot.current === element && !surfaceActiveRoots.has(element)) setSurfaceActiveRoot(null);
      });
    };
  });
  const bind = reactHostPort.useMemo<SurfaceActiveBindProps>(
    () => ({
      onPointerDownCapture: () => {
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
