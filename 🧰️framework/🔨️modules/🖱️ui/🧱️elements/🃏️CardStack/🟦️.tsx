import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { snapCardHeight } from "./bands.ts";

export { snapCardHeight } from "./bands.ts";

const CARD = "[data-overview-card]";
const OWNER = "[data-card-stack], [data-card-grid]";
const WINDOW = "[data-overview-card-stack]";
const SNAPPED = "min-block-size";

const put = (element: HTMLElement | null, value: string | null): void => { if (!element || element.style.getPropertyValue(SNAPPED) === (value ?? "")) return; if (value === null) element.style.removeProperty(SNAPPED); else element.style.setProperty(SNAPPED, value); };

/** 📐️ Snaps the heights of the cards of a stack or grid `container` to whole device pixels. Every card is one rectangle with its
 * chips overlaid inside it, so cards touch on one coincident line and need no lift, band or margin: the snapped heights make the
 * bottom line of one card and the top line of the next fall on the same device pixel, and the shared-border compositor paints
 * that line once. Keeps doing so while the cards or the container change.
 *
 * @see ./bands.ts */
export function useCardInterlock(container: HTMLElement | null): void {
  React.useLayoutEffect(() => {
    if (!container) return;
    const view = container.ownerDocument.defaultView;
    let frame = 0;
    let cards: HTMLElement[] | null = null;
    let settled = "";
    const observed = new Set<Element>();
    const measure = () => {
      frame = 0;
      const owned = cards ??= [...container.querySelectorAll<HTMLElement>(CARD)].filter(card => card.closest(OWNER) === container && card.parentElement?.closest(CARD) == null);
      const pixel = 1 / (view?.devicePixelRatio || 1);
      const signature = () => `${pixel}:${[container, ...owned].map(element => { const rect = element.getBoundingClientRect(); return `${Math.round(rect.width * 1000)}x${Math.round(rect.height * 1000)}`; }).join()}`;
      if (settled !== "" && signature() === settled) return;
      for (const card of owned) { put(card, null); put(card.querySelector<HTMLElement>(WINDOW), null); }
      const snap = (elements: readonly (HTMLElement | null)[]) => { const naturals = elements.map(element => element?.getBoundingClientRect().height ?? 0); elements.forEach((element, index) => { const snapped = snapCardHeight(naturals[index]!, pixel); if (element && snapped - naturals[index]! > 1e-3) put(element, `${snapped}px`); }); };
      snap(owned.map(card => card.querySelector<HTMLElement>(WINDOW)));
      snap(owned);
      settled = signature();
      const next = new Set<Element>([container, ...owned]);
      for (const element of observed) if (!next.has(element)) { resize?.unobserve(element); observed.delete(element); }
      for (const element of next) if (!observed.has(element)) { resize?.observe(element); observed.add(element); }
    };
    const schedule = () => {
      if (frame) return;
      if (view?.requestAnimationFrame) frame = view.requestAnimationFrame(measure);
      else measure();
    };
    const sizes = new WeakMap<Element, string>();
    const resize = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(entries => {
      let moved = false;
      for (const { target, contentRect } of entries) {
        const size = `${Math.round(contentRect.width * 1000)}x${Math.round(contentRect.height * 1000)}`;
        if (sizes.get(target) !== size) { moved = sizes.has(target) || moved; sizes.set(target, size); }
      }
      if (moved) schedule();
    });
    const relevant = (node: Node) => node.nodeType === 1 && (node as Element).namespaceURI === "http://www.w3.org/1999/xhtml" && ((node as Element).matches(CARD) || (node as Element).querySelector(CARD) !== null);
    const mutations = typeof MutationObserver === "undefined" ? null : new MutationObserver(records => {
      for (const record of records) {
        const target = record.target as Element;
        if (target.nodeType !== 1 || target.namespaceURI !== "http://www.w3.org/1999/xhtml" || target.closest(OWNER) !== container) continue;
        if (![...record.addedNodes, ...record.removedNodes].some(relevant)) continue;
        cards = null;
        settled = "";
        schedule();
        break;
      }
    });
    measure();
    mutations?.observe(container, { childList: true, subtree: true });
    view?.addEventListener("resize", schedule);
    return () => {
      if (frame) view?.cancelAnimationFrame(frame);
      resize?.disconnect();
      mutations?.disconnect();
      view?.removeEventListener("resize", schedule);
      for (const card of container.querySelectorAll<HTMLElement>(CARD)) { put(card, null); put(card.querySelector<HTMLElement>(WINDOW), null); }
    };
  }, [container]);
}

/** 🃏️ Props of {@link CardStack}. */
export interface CardStackProps extends Omit<React.HTMLAttributes<HTMLElement>, "children"> {
  readonly as?: "div" | "main" | "section" | "ul" | "ol";
  /** 📏️ Widest the column grows (any CSS length); it is centred when the container is wider. Unbounded by default. */
  readonly maxWidth?: string;
  readonly children?: React.ReactNode;
}

/** 🧱️ A column of abutting cards: no gap and no page margin between them, the container touches whatever lies above
 * and below it, the first card starts on the container's top edge and the last card ends on its bottom edge. Each card is one
 * rectangle with its title and footer chips overlaid inside it, so two cards meet on one coincident line that the shared-border
 * compositor paints once (see {@link useCardInterlock}). Domain-neutral: it knows only `OverviewCard`s (`data-overview-card`),
 * whatever else it holds lies in the column as ordinary children.
 *
 * @see ../🃏️OverviewCard/🟦️.tsx
 * @see ../🗂️WindowChrome/shared-border.ts — one owner per coincident segment */
export const CardStack = React.forwardRef<HTMLElement, CardStackProps>(function CardStack({ as: Tag = "div", maxWidth, className, style, children, ...rest }, forwarded) {
  const [inner, setInner] = React.useState<HTMLElement | null>(null);
  const ref = React.useCallback((node: HTMLElement | null) => {
    setInner(node);
    if (typeof forwarded === "function") forwarded(node);
    else if (forwarded) forwarded.current = node;
  }, [forwarded]);
  useCardInterlock(inner);
  const Element = Tag as React.ElementType;
  return (
    <Element ref={ref} data-slot="card-stack" data-card-stack="" className={cn("relative mx-auto flex w-full min-w-0 flex-col", className)} style={{ maxWidth, ...style }} {...rest}>
      {children}
    </Element>
  );
});

/** 🧮️ Props of {@link CardGrid}. */
export interface CardGridProps extends Omit<React.HTMLAttributes<HTMLElement>, "children"> {
  readonly as?: "div" | "main" | "section" | "ul" | "ol";
  /** 🔳️ `grid-template-columns` (default: as many columns of at least `minColumn` as fit). */
  readonly columns?: string;
  /** 🔳️ `grid-template-rows` (default: as high as the tallest card of the row). */
  readonly rows?: string;
  /** 📏️ Narrowest a default column gets (any CSS length). */
  readonly minColumn?: string;
  readonly children?: React.ReactNode;
}

/** 🔳️ A grid of cards that tile edge to edge: no gap, no padding, every cell as large as its track and its card filling the
 * cell, so adjacent cards share one border line (the compositor paints it once) and text spacing lies inside the cards.
 * Cells are {@link CardGridCell}s or any element marked `data-card-grid-cell`. Cards of consecutive rows abut like
 * those of a {@link CardStack} (see {@link useCardInterlock}).
 *
 * @see ../🃏️OverviewCard/🟦️.tsx */
export const CardGrid = React.forwardRef<HTMLElement, CardGridProps>(function CardGrid({ as: Tag = "div", columns, rows, minColumn, className, style, children, ...rest }, forwarded) {
  const [inner, setInner] = React.useState<HTMLElement | null>(null);
  const ref = React.useCallback((node: HTMLElement | null) => {
    setInner(node);
    if (typeof forwarded === "function") forwarded(node);
    else if (forwarded) forwarded.current = node;
  }, [forwarded]);
  useCardInterlock(inner);
  const Element = Tag as React.ElementType;
  const vars = { "--card-grid-columns": columns, "--card-grid-rows": rows, "--card-grid-min-column": minColumn } as React.CSSProperties;
  return (
    <Element ref={ref} data-slot="card-grid" data-card-grid="" className={cn("relative", className)} style={{ ...vars, ...style }} {...rest}>
      {children}
    </Element>
  );
});

/** 🔲️ Props of {@link CardGridCell}. */
export interface CardGridCellProps extends React.HTMLAttributes<HTMLElement> {
  readonly as?: "div" | "li" | "section";
  /** 📍️ Zero-based column and row; omitted, the cell flows. */
  readonly column?: number;
  readonly row?: number;
}

/** 🔲️ One cell of a {@link CardGrid}: its single child (a card) is stretched over the whole cell. */
export const CardGridCell = React.forwardRef<HTMLElement, CardGridCellProps>(function CardGridCell({ as: Tag = "div", column, row, className, style, children, ...rest }, ref) {
  const Element = Tag as React.ElementType;
  return (
    <Element ref={ref} data-slot="card-grid-cell" data-card-grid-cell="" className={className} style={{ gridColumn: column === undefined ? undefined : column + 1, gridRow: row === undefined ? undefined : row + 1, ...style }} {...rest}>
      {children}
    </Element>
  );
});
