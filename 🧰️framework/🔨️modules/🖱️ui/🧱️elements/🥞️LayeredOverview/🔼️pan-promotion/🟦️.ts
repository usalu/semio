/** 🔼️ Layer promotion that lasts as long as a pan: the layered overview's strip is many viewports large, and a permanent `will-change` or `translate3d` on it keeps one layer of tens of megapixels for as long as the page lives. Every write of the strip's transform promotes it (`will-change: transform`, a 3D translation) and a moment after the last write the same translation is written in 2D and `will-change` is given up, so a resting strip belongs to its parent's layer and costs no layer memory.
 *
 * @see ../🔶️panes/🟦️.tsx — where the strip's transform is written, once per frame of a pan, a glide or a settle
 * @see https://developer.mozilla.org/en-US/docs/Web/CSS/will-change
 */

/** ⏲️ How long after the last write of a pan the strip is given up, in milliseconds: long enough to span the gap between two frames of a glide, short enough to free the layer soon after the strip rests. */
export const PAN_REST_MS = 250;

const TIMERS = new WeakMap<HTMLElement, ReturnType<typeof setTimeout>>();
const THREE_D = /^translate3d\(([^,()]+),\s*([^,()]+),\s*0(?:px)?\)$/;

/** 📐️ The 2D translation that moves an element exactly as the 3D translation `transform` does, `transform` itself when it is anything else. */
export function flatten(transform: string): string {
  const found = THREE_D.exec(transform);
  return found === null ? transform : `translate(${found[1]!.trim()}, ${found[2]!.trim()})`;
}

/** ✍️ Writes `transform` to `element` as one step of a pan, promoted until {@link PAN_REST_MS} after the last write; `null` lets go of an element (its timer is cleared and nothing more is written to it). */
export function writeTransform(element: HTMLElement, transform: string | null): void {
  const pending = TIMERS.get(element);
  if (pending !== undefined) clearTimeout(pending);
  TIMERS.delete(element);
  if (transform === null) return;
  element.style.willChange = "transform";
  element.style.transform = transform;
  TIMERS.set(
    element,
    setTimeout(() => {
      TIMERS.delete(element);
      element.style.transform = flatten(element.style.transform);
      element.style.willChange = "";
    }, PAN_REST_MS),
  );
}
