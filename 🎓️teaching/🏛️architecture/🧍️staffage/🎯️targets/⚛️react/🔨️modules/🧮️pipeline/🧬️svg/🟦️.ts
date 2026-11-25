/** 🧬️ Puts inline SVG markup into an element by cloning a parsed template instead of re-parsing the string: a looping clip shows the same few markups over and over, so after the first cycle a frame costs one `cloneNode` and one `replaceChildren`.
 *
 * The kept templates are bounded by what they weigh ({@link SVG_TEMPLATE_BUDGET_CHARACTERS}), least recently shown first out: a live figure of a heavy asset shows a different markup of thousands of nodes on every frame, and a bound by count alone lets the detached fragments grow to gigabytes.
 *
 * @see ../🟦️.ts — `inlineSvg`, the memoised markup
 */
import { useLayoutEffect, type RefObject } from "react";

/** ⚖️ The markup characters all kept templates may weigh together; a markup heavier than this is shown and not kept. Measured on the desktop overview of the 375-asset catalog: a node costs about 24 markup characters, and the looping clips of its seven live figures are about 3.5 to 10 million characters, so a budget below that parses every frame again. Eight million characters are about 330 000 detached nodes. */
export const SVG_TEMPLATE_BUDGET_CHARACTERS = 8_000_000;

const templates = new Map<string, DocumentFragment>();
let characters = 0;

/** 📊️ How many templates are kept and what they weigh in markup characters. */
export const svgTemplateLoad = (): { readonly templates: number; readonly characters: number } => ({ templates: templates.size, characters });

function templateOf(document: Document, markup: string): DocumentFragment {
  const kept = templates.get(markup);
  if (kept !== undefined) {
    templates.delete(markup);
    templates.set(markup, kept);
    return kept;
  }
  const template = document.createElement("template");
  template.innerHTML = markup;
  if (markup.length > SVG_TEMPLATE_BUDGET_CHARACTERS) return template.content;
  templates.set(markup, template.content);
  characters += markup.length;
  for (const oldest of templates.keys()) {
    if (characters <= SVG_TEMPLATE_BUDGET_CHARACTERS) break;
    templates.delete(oldest);
    characters -= oldest.length;
  }
  return template.content;
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
