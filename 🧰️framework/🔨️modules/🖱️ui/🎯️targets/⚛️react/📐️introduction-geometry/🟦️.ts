import { reactHostPort } from "../../../🧱️elements/🔌️Ports/🟦️.tsx";
import { useShellScopeOptional } from "../../../🧱️elements/🐚️ShellScope/🟦️.tsx";
import { elementIdSelector } from "../../../🧱️elements/🆔️ElementId/🟦️.tsx";
import type { IntroductionPlacement } from "@semio-tech/framework";
import { ephemeralMap } from "@semio-tech/framework/transient";

type IntroductionRect = { readonly top: number; readonly left: number; readonly width: number; readonly height: number };

/** @emoji 🎓️ Converts a viewport (getBoundingClientRect) box into coordinates local to `host`. */
export function introductionRectRelativeToHost(rect: IntroductionRect, hostRect: IntroductionRect): IntroductionRect {
  return { top: rect.top - hostRect.top, left: rect.left - hostRect.left, width: rect.width, height: rect.height };
}

/** @emoji 🎓️ Viewport-pixel point → host-local point for absolute overlays inside a transformed shell. */
export function introductionPointRelativeToHost(point: { readonly x: number; readonly y: number }, hostRect: IntroductionRect): { readonly x: number; readonly y: number } {
  return { x: point.x - hostRect.left, y: point.y - hostRect.top };
}

/** @emoji 🎓️ Live-tracks the union DOM rect of an introduction step's `introduce` element(s) (via
 * {@link elementIdSelector}), stamping every match `data-introduced="true"` (pulsing the introduced border, see
 * `framework/ui/styling/js/🎨️ui.css`) for as long as they stay attached — cleared on unmount/selector change. A kind-level
 * window id that aliases every open instance (Top + Perspective) therefore pulses and anchors against the
 * union of all matches, not only the first. Waits for the element(s) to mount (a folded panel/utility bar the
 * shell is in the middle of revealing) instead of failing closed; a `null` selector or a never-found element
 * both resolve to `null`, which the caller treats as a `Screen`-style full veil. A `MutationObserver` on the
 * document re-attempts attachment on every DOM change until at least one mounts, then keeps observing so
 * late-arriving aliases (extra window instances) join the pulse set. */
export function useIntroductionAnchorRect(selector: string | null): IntroductionRect | null {
  const [rect, setRect] = reactHostPort.useState<IntroductionRect | null>(null);
  // 🐚️ Falls back to `document`/`document.body` outside any shell (e.g. the mit-bestand demonstrator's
  // own standalone tour) — inside one, scopes the search+observe to that shell's own root so an id/alias
  // shared across two mounted shells only anchors within the one actually running this introduction.
  const shellScope = useShellScopeOptional();

  reactHostPort.useEffect(() => {
    if (!selector) {
      setRect(null);
      return;
    }
    const getSearchRoot = (): ParentNode | null => {
      if (shellScope) return shellScope.rootRef.current;
      return document;
    };
    const observeRoot: globalThis.Node = shellScope?.rootRef.current ?? document.body;
    let elements: Element[] = [];
    let resizeObserver: ResizeObserver | null = null;
    let reportedUnresolved = false;

    const unionRect = (matches: readonly Element[]): IntroductionRect | null => {
      if (matches.length === 0) return null;
      let top = Infinity;
      let left = Infinity;
      let right = -Infinity;
      let bottom = -Infinity;
      for (const match of matches) {
        const box = match.getBoundingClientRect();
        top = Math.min(top, box.top);
        left = Math.min(left, box.left);
        right = Math.max(right, box.right);
        bottom = Math.max(bottom, box.bottom);
      }
      return { top, left, width: right - left, height: bottom - top };
    };
    const measure = () => {
      const next = unionRect(elements);
      if (next) setRect(next);
    };
    const attach = () => {
      const searchRoot = getSearchRoot();
      if (!searchRoot) {
        if (elements.length > 0) {
          elements.forEach((el) => el.removeAttribute("data-introduced"));
          elements = [];
          resizeObserver?.disconnect();
          resizeObserver = null;
        }
        if (!reportedUnresolved) {
          reportedUnresolved = true;
          setRect(null);
        }
        return;
      }
      const candidates = [...searchRoot.querySelectorAll(selector)];
      if (candidates.length === 0) {
        if (elements.length > 0) {
          elements.forEach((el) => el.removeAttribute("data-introduced"));
          elements = [];
          resizeObserver?.disconnect();
          resizeObserver = null;
        }
        if (!reportedUnresolved) {
          reportedUnresolved = true;
          setRect(null);
        }
        return;
      }
      const same = candidates.length === elements.length && candidates.every((candidate, index) => candidate === elements[index]);
      if (same) return;
      const previous = new Set(elements);
      const next = new Set(candidates);
      previous.forEach((el) => {
        if (!next.has(el)) el.removeAttribute("data-introduced");
      });
      candidates.forEach((el) => el.setAttribute("data-introduced", "true"));
      elements = candidates;
      resizeObserver?.disconnect();
      resizeObserver = new ResizeObserver(measure);
      candidates.forEach((el) => resizeObserver?.observe(el));
      measure();
    };

    attach();
    const mutationObserver = new MutationObserver(attach);
    mutationObserver.observe(observeRoot, { childList: true, subtree: true });
    const onResize = () => measure();
    window.addEventListener("resize", onResize);
    return () => {
      elements.forEach((el) => el.removeAttribute("data-introduced"));
      resizeObserver?.disconnect();
      mutationObserver.disconnect();
      window.removeEventListener("resize", onResize);
      setRect(null);
    };
  }, [selector, shellScope]);

  return rect;
}

/** @emoji 🎓️ Elevates the chrome unit containing each of `ids` above the single fullscreen introduction
 * veil, by stamping `data-introduction-elevated` (see `framework/ui/styling/js/🎨️ui.css`) on its nearest
 * `[data-slot="mode-dock-stack"]` (full window silhouette: tabs + gap + controls + body) or else its
 * nearest `[data-elevation-root]` ancestor — the panel/window/navbar/footer that owns a real, root-level
 * stacking context (a deeply-nested target such as a tree row can't be raised on its own; CSS stacking
 * contexts trap it inside its panel/window regardless of its own z-index). An id with no enclosing
 * chrome unit elevates itself directly, gaining `position: relative` as a fallback if it was
 * `static` (restored on cleanup). Every DOM match of an id is elevated — a window-kind id aliased onto
 * every open instance (Top + Perspective via `data-element-alias`) therefore raises all of them, not
 * only the first `querySelector` hit. Waits for each id to mount (a folded utility bar/panel the shell is
 * still revealing) via the same `MutationObserver` re-attach pattern as `useIntroductionAnchorRect`, and
 * returns the ids that currently resolved so the caller can tell "still waiting" from "revealed" per id
 * (e.g. to decide whether the veil should block pointer events yet). All stamps and fallback positioning
 * are undone on id-list change and unmount. */
export function useIntroductionElevation(ids: readonly string[]): ReadonlySet<string> {
  const [resolved, setResolved] = reactHostPort.useState<ReadonlySet<string>>(() => new Set());
  // 🐚️ See `useIntroductionAnchorRect`'s doc for why this is scoped to a shell's own root when one exists.
  const shellScope = useShellScopeOptional();

  reactHostPort.useEffect(() => {
    if (ids.length === 0) {
      setResolved(new Set());
      return;
    }
    const getSearchRoot = (): ParentNode | null => {
      if (shellScope) return shellScope.rootRef.current;
      return document;
    };
    const observeRoot: globalThis.Node = shellScope?.rootRef.current ?? document.body;
    let stampedRoots = new Set<Element>();
    const positionedByUs = new Set<Element>();

    const resolve = () => {
      const searchRoot = getSearchRoot();
      if (!searchRoot) return;
      const nextResolved = new Set<string>();
      const wantedRoots = new Set<Element>();
      for (const id of ids) {
        const targets = searchRoot.querySelectorAll(elementIdSelector(id));
        if (targets.length === 0) continue;
        nextResolved.add(id);
        targets.forEach((target) => wantedRoots.add(target.closest('[data-slot="mode-dock-stack"]') ?? target.closest("[data-elevation-root]") ?? target));
      }
      stampedRoots.forEach((root) => {
        if (wantedRoots.has(root)) return;
        root.removeAttribute("data-introduction-elevated");
        if (positionedByUs.has(root)) {
          (root as HTMLElement).style.position = "";
          positionedByUs.delete(root);
        }
      });
      wantedRoots.forEach((root) => {
        root.setAttribute("data-introduction-elevated", "true");
        if (!positionedByUs.has(root) && getComputedStyle(root as Element).position === "static") {
          (root as HTMLElement).style.position = "relative";
          positionedByUs.add(root);
        }
      });
      stampedRoots = wantedRoots;
      setResolved((prev) => (prev.size === nextResolved.size && [...nextResolved].every((id) => prev.has(id)) ? prev : nextResolved));
    };

    resolve();
    const mutationObserver = new MutationObserver(resolve);
    mutationObserver.observe(observeRoot, { childList: true, subtree: true });
    return () => {
      mutationObserver.disconnect();
      stampedRoots.forEach((root) => {
        root.removeAttribute("data-introduction-elevated");
        if (positionedByUs.has(root)) (root as HTMLElement).style.position = "";
      });
      setResolved(new Set());
    };
  }, [ids, shellScope]);

  return resolved;
}

export type IntroductionInfoBoxPosition = { readonly top: number; readonly left: number };

export const INTRODUCTION_INFO_BOX_GAP_PX = 16;

/** @emoji 📝️ Splits an introduction step body into visual paragraphs on blank lines so each
 * paragraph can emphasize independently on hover. */
export function splitIntroductionBodyParagraphs(body: string): readonly string[] {
  return body
    .split(/\n\n+/)
    .map((paragraph) => paragraph.trim())
    .filter((paragraph) => paragraph.length > 0);
}

/** @emoji 🧲️ Clamps an introduction info box to the viewport. Authored placement uses a visual inset;
 * direct manipulation uses zero so every outer edge can meet the corresponding viewport border. */
export function clampIntroductionInfoBoxPosition(
  position: IntroductionInfoBoxPosition,
  boxSize: { readonly width: number; readonly height: number },
  viewport: { readonly width: number; readonly height: number },
  inset: number,
): IntroductionInfoBoxPosition {
  return {
    top: Math.min(Math.max(position.top, inset), Math.max(inset, viewport.height - boxSize.height - inset)),
    left: Math.min(Math.max(position.left, inset), Math.max(inset, viewport.width - boxSize.width - inset)),
  };
}

/** @emoji 🎓️ Where the info box sits relative to its anchor. `auto` picks the side with the most free
 * viewport space; `center` (and any anchor-less step) centers the box in the viewport.
 *
 * 🧲️ EVERY branch clamps, the centered one included: centering a box that is wider or taller than the
 * measured viewport puts its top-left OUTSIDE the viewport, and the box carries the tour's only
 * dismissal controls. Measured on a 1440×900 boot of the puzzle3d welcome step: box at
 * `[-80, -78, 422, 108]`, its `ui.introduction.skip` control at `[287, -78, 55, 31]` — centre above the
 * top edge, `document.elementFromPoint` `null`, so neither a user nor an actionability-checked click can
 * reach Skip or Next while the step's veil holds `pointer-events: auto` over the whole application
 * (26/09/02/PUZZLE-3D-END-TO-END wave B49 §1; B47 §5.1 saw the same scrim eat every press). */
export function resolveIntroductionPlacement(
  placement: IntroductionPlacement,
  anchorRect: IntroductionRect | null,
  boxSize: { readonly width: number; readonly height: number },
  viewport: { readonly width: number; readonly height: number },
): IntroductionInfoBoxPosition {
  const gap = INTRODUCTION_INFO_BOX_GAP_PX;
  const clamp = (position: IntroductionInfoBoxPosition) => clampIntroductionInfoBoxPosition(position, boxSize, viewport, gap);
  const centered = clamp({ top: (viewport.height - boxSize.height) / 2, left: (viewport.width - boxSize.width) / 2 });
  if (!anchorRect || placement === "center") return centered;

  const space = {
    top: anchorRect.top,
    bottom: viewport.height - (anchorRect.top + anchorRect.height),
    left: anchorRect.left,
    right: viewport.width - (anchorRect.left + anchorRect.width),
  } as const;
  const side = placement === "auto" ? (Object.keys(space) as (keyof typeof space)[]).sort((a, b) => space[b] - space[a])[0] : placement;

  switch (side) {
    case "top":
      return clamp({ top: anchorRect.top - boxSize.height - gap, left: anchorRect.left + anchorRect.width / 2 - boxSize.width / 2 });
    case "bottom":
      return clamp({ top: anchorRect.top + anchorRect.height + gap, left: anchorRect.left + anchorRect.width / 2 - boxSize.width / 2 });
    case "left":
      return clamp({ top: anchorRect.top + anchorRect.height / 2 - boxSize.height / 2, left: anchorRect.left - boxSize.width - gap });
    case "right":
      return clamp({ top: anchorRect.top + anchorRect.height / 2 - boxSize.height / 2, left: anchorRect.left + anchorRect.width + gap });
    default:
      return centered;
  }
}

/** @emoji 🏷️ What a surface resolver returns for `IntroductionPoint.Entity`/`Curve`/`Domain`: a viewport
 * pixel anchor plus whatever richer geometry the surface can offer — `rect` for offset-within-bounds and
 * domain-to-track mapping, `polyline` (viewport px) for arc-length curve targeting, `domain` for mapping
 * a value onto `rect`. `visible: false` means "found but not currently resolvable" (off-camera, hidden,
 * zero-length geometry) — callers treat that exactly like `null`, never as an error. */
export type IntroductionResolvedGeometry = {
  readonly point: { readonly x: number; readonly y: number };
  readonly rect?: { readonly x: number; readonly y: number; readonly width: number; readonly height: number };
  readonly polyline?: readonly { readonly x: number; readonly y: number }[];
  readonly domain?: { readonly min: number; readonly max: number; readonly axis: "x" | "y" };
  readonly visible: boolean;
};

/** @emoji 🧭️ A surface's live-camera/live-entity capabilities for demonstration targeting, registered per
 * window element id by the surface that owns the data (a react-three-fiber `useThree` bridge, a WASM
 * canvas session, a DOM-rendered host). Every method may be omitted — a surface implements only the point
 * kinds it can genuinely resolve. All three are called every animation frame; implementations must be
 * cheap (cache parsed JSON keyed by the source string's identity, never re-parse a whole fixture per
 * frame). */
export type IntroductionSurfaceResolver = {
  /** 🧊️ 3D world position → viewport pixel through the surface's live camera. */
  readonly scenePoint?: (position: readonly [number, number, number]) => { readonly x: number; readonly y: number; readonly visible: boolean } | null;
  /** 🗺️ 2D world (camera x/y/zoom) position → viewport pixel. */
  readonly canvasPoint?: (x: number, y: number) => { readonly x: number; readonly y: number; readonly visible: boolean } | null;
  /** 🏷️ A live entity by domain + id (`"*"` = any representative) → its resolved geometry. */
  readonly entity?: (domain: string, entity: string) => IntroductionResolvedGeometry | null;
};

export const introductionSurfaceResolvers = ephemeralMap<string, IntroductionSurfaceResolver>("framework.modules.ui.packages.typescript.targets.react.index.tsx.introductionSurfaceResolvers");

/** @emoji 🧭️ Registers the demonstration-targeting resolver for the surface shown by window element
 * `windowId` — call from the window's own host component (has the live camera/session/DOM refs),
 * unregister on unmount via the returned disposer. Multiple concurrently open instances of the same
 * window kind (split panes) last-write-wins; acceptable for a single demonstration target. */
export function registerIntroductionSurfaceResolver(windowId: string, resolver: IntroductionSurfaceResolver): () => void {
  introductionSurfaceResolvers.set(windowId, resolver);
  return () => {
    if (introductionSurfaceResolvers.get(windowId) === resolver) introductionSurfaceResolvers.delete(windowId);
  };
}

/** @emoji 🧊️ Pure NDC (`[-1, 1]`, y-up) → viewport-pixel conversion shared by every 3D scene resolver. */
export function ndcToViewportPoint(ndc: { readonly x: number; readonly y: number }, rect: { readonly left: number; readonly top: number; readonly width: number; readonly height: number }): { readonly x: number; readonly y: number } {
  return { x: rect.left + ((ndc.x + 1) / 2) * rect.width, y: rect.top + ((1 - ndc.y) / 2) * rect.height };
}
