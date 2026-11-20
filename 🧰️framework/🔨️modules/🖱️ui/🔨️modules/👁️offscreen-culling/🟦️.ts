/** 👁️ Off-screen culling: one shared `IntersectionObserver` per root marks what lies a viewport or more away from the view with `data-offscreen`, and the stylesheet (`🎨️styling/🖌️ui/🎨️.css`) drops the blur, the `will-change` layers and the heavy shadows under that mark, so a page of many glass windows holds compositor memory only for the windows that can be seen, and every app gets it.
 *
 * The margin is a whole viewport on every side, so a surface is unmarked long before it enters the view and nothing pops in. A root's margin is the only margin its clipped children get (an ancestor that clips cuts a target before the margin counts), so every root has an observer of its own and every element of that root shares it. Nothing is marked before the observer has spoken; where there is no observer everything is shown.
 *
 * @see ../../🧱️elements/🥞️LayeredOverview/🟦️.tsx — the first caller: the pages of its strip
 * @see https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/rootMargin
 */

import * as React from "react";

/** 🏷️ The attribute of whatever is away from the view. */
export const OFFSCREEN_ATTRIBUTE = "data-offscreen";

/** 📏️ How far beyond the root an element still counts as near: a viewport on every side. */
export const OFFSCREEN_MARGIN = "100% 100% 100% 100%";

interface Watch {
  readonly observer: IntersectionObserver;
  readonly counts: Map<Element, number>;
}

const WATCHES = new WeakMap<Element, Watch>();
const DOCUMENT_WATCH: { current: Watch | null } = { current: null };

function mark(entries: readonly IntersectionObserverEntry[]): void {
  for (const entry of entries) {
    if (entry.isIntersecting) entry.target.removeAttribute(OFFSCREEN_ATTRIBUTE);
    else entry.target.setAttribute(OFFSCREEN_ATTRIBUTE, "");
  }
}

/** 👀️ Watches `element` against `root` (the viewport when absent) and returns the function that ends the watch, unmarks the element and, when it was the last of its root, disconnects the root's observer. An element watched twice stays watched until both end. */
export function observeOffscreen(element: Element, root: Element | null = null): () => void {
  if (typeof IntersectionObserver !== "function") return () => {};
  let watch = root === null ? DOCUMENT_WATCH.current : (WATCHES.get(root) ?? null);
  if (watch === null) {
    watch = { observer: new IntersectionObserver(mark, { root, rootMargin: OFFSCREEN_MARGIN, threshold: 0 }), counts: new Map() };
    if (root === null) DOCUMENT_WATCH.current = watch;
    else WATCHES.set(root, watch);
  }
  const held = watch;
  const known = held.counts.get(element) ?? 0;
  held.counts.set(element, known + 1);
  if (known === 0) held.observer.observe(element);
  let ended = false;
  return () => {
    if (ended) return;
    ended = true;
    const left = (held.counts.get(element) ?? 1) - 1;
    if (left > 0) return void held.counts.set(element, left);
    held.counts.delete(element);
    held.observer.unobserve(element);
    element.removeAttribute(OFFSCREEN_ATTRIBUTE);
    if (held.counts.size > 0) return;
    held.observer.disconnect();
    if (root === null) DOCUMENT_WATCH.current = null;
    else WATCHES.delete(root);
  };
}

/** 🪝️ A ref callback that watches the element it is given for as long as it stays mounted, against the root `rootOf` names for that element (the viewport when it names none or is absent); `rootOf` must be stable. */
export function useOffscreen(rootOf?: (element: HTMLElement) => Element | null): React.RefCallback<HTMLElement> {
  const stop = React.useRef<(() => void) | null>(null);
  React.useEffect(() => () => stop.current?.(), []);
  return React.useCallback(
    (element: HTMLElement | null) => {
      stop.current?.();
      stop.current = element === null ? null : observeOffscreen(element, rootOf?.(element) ?? null);
    },
    [rootOf],
  );
}
