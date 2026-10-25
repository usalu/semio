import { isElementId } from "../../🧱️elements/🆔️ElementId/🟦️.tsx";

/** 🎉️ Default lifetime of a transient celebration stamp — two burst cycles of --celebrate-border-duration. */
export const CELEBRATE_STAMP_DURATION_MS = 2400;

/** 🎯️ Imperatively stamps `data-celebrated="true"` on `target` for `durationMs`, then removes it —
 * the transient counterpart of an authored `state: "celebrating"`. Unmanaged by React (like the
 * introduction engine's `data-introduced` stamp) so re-renders can't clobber it. Returns a cancel
 * function that un-stamps immediately. */
export function celebrateElement(target: Element, durationMs = CELEBRATE_STAMP_DURATION_MS): () => void {
  target.setAttribute("data-celebrated", "true");
  const clear = () => target.removeAttribute("data-celebrated");
  const timer = window.setTimeout(clear, durationMs);
  return () => {
    window.clearTimeout(timer);
    clear();
  };
}

/** 🔎️ Imperatively stamps `data-celebrated="true"` on every match of `selector` for `durationMs`,
 * then removes it — the selector form of {@link celebrateElement}. Returns a cancel that un-stamps
 * every match immediately. `root` scopes the search (e.g. a shell's own root) — omitted, searches the
 * whole document as before; matters because element ids/aliases are not guaranteed unique across
 * several mounted shells, and an unscoped search can stamp another shell's element by mistake. */
export function celebrateElements(selector: string, durationMs = CELEBRATE_STAMP_DURATION_MS, root: ParentNode = document): () => void {
  const cancels = [...root.querySelectorAll(selector)].map((el) => celebrateElement(el, durationMs));
  return () => cancels.forEach((cancel) => cancel());
}

/** 🎊️ Imperatively stamps `data-celebrated="true"` on every mounted UI element id (and every
 * element carrying a valid `data-element-alias`) for `durationMs` — the tour-finale counterpart of
 * {@link celebrateElements}. Skips the introduction chrome itself (`ui.introduction.*`) so the
 * dismiss unmount doesn't race the stamp. Returns a cancel that un-stamps every match immediately.
 * `root` scopes the search — see {@link celebrateElements}'s doc for why this matters across shells. */
export function celebrateAllElements(durationMs = CELEBRATE_STAMP_DURATION_MS, root: ParentNode = document): () => void {
  const targets = new Set<Element>();
  for (const el of root.querySelectorAll("[id]")) {
    if (isElementId(el.id) && !el.id.startsWith("ui.introduction.")) targets.add(el);
  }
  for (const el of root.querySelectorAll("[data-element-alias]")) {
    const aliases = (el.getAttribute("data-element-alias") ?? "").split(/\s+/).filter(Boolean);
    if (aliases.some((alias) => isElementId(alias) && !alias.startsWith("ui.introduction."))) targets.add(el);
  }
  const cancels = [...targets].map((el) => celebrateElement(el, durationMs));
  return () => cancels.forEach((cancel) => cancel());
}
