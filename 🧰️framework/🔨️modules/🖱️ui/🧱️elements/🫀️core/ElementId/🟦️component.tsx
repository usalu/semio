// #region 🧲️Header
// 💻️ framework/ui/elements/core/ElementId/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
// 🚧️W3-interim: reactHostPort still lives in the ui-react barrel (🔌️Adapters region) — rewire once
// that region gets its own 🧱️elements/🫀️core/Adapters/ file.
import { reactHostPort } from "../../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

//#region 🆔️ElementId
/** 🆔️ Renderer-agnostic UI element id grammar: dot-separated camelCase segments, each starting with a
 * lowercase letter — e.g. `framework.window.main.action.addLayer`. Mirrors `is_element_id` in
 * `framework/core/rs/lib.rs` byte-for-byte; this id is the single integration key across i18n, tooltips,
 * hotkeys, command origin tracking, tutorials, E2E selectors, and introduction anchors. */
export const ELEMENT_ID_PATTERN = /^[a-z][a-zA-Z0-9]*(\.[a-z][a-zA-Z0-9]*)*$/;

/** 🆔️ Whether `id` matches {@link ELEMENT_ID_PATTERN}. */
export function isElementId(id: string): boolean {
  return ELEMENT_ID_PATTERN.test(id);
}

/** 🆔️ Normalizes arbitrary input (a domain object's own id, a free-text label) into a single camelCase
 * element-id segment — mirrors `element_id_segment` in `framework/core/rs/lib.rs` byte-for-byte.
 * Idempotent on input that is already a valid segment. Prefer a real semantic key first, this second,
 * and a numeric index only as a last resort (see {@link childElementId}). */
export function elementIdSegment(raw: string): string {
  let segment = "";
  let capitalizeNext = false;
  for (const ch of raw) {
    if (ch === "-" || ch === "_" || ch === " " || ch === ".") {
      capitalizeNext = true;
      continue;
    }
    if (!/[a-zA-Z0-9]/.test(ch)) continue;
    if (segment.length === 0) {
      segment += ch.toLowerCase();
    } else if (capitalizeNext) {
      segment += ch.toUpperCase();
      capitalizeNext = false;
    } else {
      segment += ch;
    }
  }
  return segment;
}

/** 🆔️ Derives a child element id by suffixing `parent` with one or more segments, each normalized
 * through {@link elementIdSegment} — the hierarchical mechanism every composite component uses to name
 * its parts instead of a context/registry: `childElementId("ui.chat", "send")` → `"ui.chat.send"`. */
export function childElementId(parent: string, ...segments: (string | number)[]): string {
  return segments.reduce<string>((id, segment) => `${id}.${elementIdSegment(String(segment))}`, parent);
}

/** 🆔️ Dev-only console warning when `id` violates {@link ELEMENT_ID_PATTERN} — called by base components
 * so a malformed id surfaces immediately at the call site instead of silently breaking i18n/tooltip/
 * introduction resolution downstream. No-operation in production builds. */
export function assertElementId(id: string, componentName: string): void {
  if (process.env.NODE_ENV === "production") return;
  if (!isElementId(id)) console.error(`${componentName} received id "${id}" which does not match the UI element id grammar (dot-separated camelCase, e.g. "framework.window.main.action.addLayer")`);
}

/** 🆔️ CSS selector matching the element whose DOM `id` is `id`, OR any element carrying `id` as a
 * secondary logical id via the space-separated `data-element-alias` attribute — the single logical-id →
 * element resolver every consumer (introductions, tests, tutorials) should use instead of hand-rolling
 * `[id=...]`/`[data-*=...]` selectors. */
export function elementIdSelector(id: string): string {
  return `[id="${id}"], [data-element-alias~="${id}"]`;
}

/** 🆔️ Adds `id` as a token to `element`'s space-separated `data-element-alias` attribute (idempotent). */
function addElementAlias(element: Element, id: string): void {
  const existing = element.getAttribute("data-element-alias");
  const tokens = existing ? existing.split(" ").filter(Boolean) : [];
  if (tokens.includes(id)) return;
  tokens.push(id);
  element.setAttribute("data-element-alias", tokens.join(" "));
}

/** 🆔️ Removes `id` from `element`'s `data-element-alias` attribute, dropping the attribute entirely once empty. */
function removeElementAlias(element: Element, id: string): void {
  const existing = element.getAttribute("data-element-alias");
  if (!existing) return;
  const tokens = existing.split(" ").filter((token) => token && token !== id);
  if (tokens.length === 0) element.removeAttribute("data-element-alias");
  else element.setAttribute("data-element-alias", tokens.join(" "));
}

/** 🆔️ Stamps `alias` (a logical element id) onto the first draggable tree row inside `containerRef`, in
 * document order — the generic mechanism behind teaching catalogue drag-and-drop without any component
 * hardcoding "first draggable row" semantics: the Panel/MobilePanel decide the alias value (their own tab
 * id + `.firstDraggable`), this hook just keeps it stamped on whichever row is first as the tree changes.
 * Re-scans on `data-draggable` mutations (not `data-element-alias`, which would self-trigger the observer). */
export function useFirstDraggableElementAlias(containerRef: React.RefObject<HTMLElement | null>, alias: string | null): void {
  reactHostPort.useEffect(() => {
    const container = containerRef.current;
    if (!container || !alias) return;
    let current: Element | null = null;
    const restamp = () => {
      const next = container.querySelector('[data-slot="tree-item-row"][data-draggable="true"]');
      if (next === current) return;
      if (current) removeElementAlias(current, alias);
      current = next;
      if (current) addElementAlias(current, alias);
    };
    restamp();
    const observer = new MutationObserver((mutations) => {
      if (mutations.some((mutation) => mutation.type === "childList" || mutation.attributeName === "data-draggable")) restamp();
    });
    observer.observe(container, { childList: true, subtree: true, attributes: true, attributeFilter: ["data-draggable"] });
    return () => {
      observer.disconnect();
      if (current) removeElementAlias(current, alias);
    };
  }, [containerRef, alias]);
}
//#endregion 🆔️ElementId
