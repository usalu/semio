/** 🧬️ Puts inline SVG markup into an element by cloning a parsed template instead of re-parsing the string: a looping clip shows the same few markups over and over, so after the first cycle a frame costs one `cloneNode` and one `replaceChildren`.
 *
 * @see ../🟦️.ts — `inlineSvg`, the memoised markup
 */
import { useLayoutEffect, type RefObject } from "react";

const CAPACITY = 1024;
const templates = new Map<string, DocumentFragment>();

function templateOf(document: Document, markup: string): DocumentFragment {
  let fragment = templates.get(markup);
  if (fragment !== undefined) {
    templates.delete(markup);
    templates.set(markup, fragment);
    return fragment;
  }
  const template = document.createElement("template");
  template.innerHTML = markup;
  fragment = template.content;
  templates.set(markup, fragment);
  if (templates.size > CAPACITY) templates.delete(templates.keys().next().value as string);
  return fragment;
}

const written = new WeakMap<Element, string>();

/** ✍️ Makes the children of `element` equal to `markup`; writes only when the element does not show it yet. */
export function writeSvgMarkup(element: Element, markup: string): void {
  if (written.get(element) === markup) return;
  written.set(element, markup);
  element.replaceChildren(templateOf(element.ownerDocument, markup).cloneNode(true));
}

/** 🪝 Keeps the children of `ref` equal to `markup`. */
export function useSvgMarkup(ref: RefObject<HTMLElement | null>, markup: string): void {
  useLayoutEffect(() => {
    if (ref.current !== null) writeSvgMarkup(ref.current, markup);
  });
}
