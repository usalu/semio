import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { WINDOW_SILHOUETTE_PATH_INSET } from "../🔲️WindowSilhouette/🟦️.tsx";
import { resolveCardBands, snapCardHeight, type CardBandCard, type CardBandSpan } from "./bands.ts";

export { resolveCardBands, type CardBand, type CardBandCard, type CardBandSpan } from "./bands.ts";

const CARD = "[data-overview-card]";
const OWNER = "[data-card-stack], [data-card-grid]";
const WINDOW = "[data-overview-card-stack]";
const CAP = "[data-slot='window-chrome-cap']";
const FOOTER = "[data-slot='window-chrome-footer']";
const CHIP = "[data-window-silhouette-chip]";
const CHIPS = `:scope > ${CHIP}`;
const STRUCTURE = `${CARD}, ${WINDOW}, ${CAP}, ${FOOTER}, ${CHIP}`;
const LIFT = "--card-stack-lift";
const CAP_BAND = "--card-stack-cap";
const FOOTER_BAND = "--card-stack-footer";
const REACH = 1 + 2 * WINDOW_SILHOUETTE_PATH_INSET;
const SETTLE_PASSES = 4;

const spans = (row: Element | null): readonly CardBandSpan[] => [...(row?.querySelectorAll(CHIPS) ?? [])].map(chip => { const rect = chip.getBoundingClientRect(); return { left: rect.left, right: rect.right }; }).filter(span => span.right > span.left);
const height = (row: Element | null): number => row?.getBoundingClientRect().height ?? 0;
const put = (element: HTMLElement | null, name: string, value: string | null): void => { if (!element || element.style.getPropertyValue(name) === (value ?? "")) return; if (value === null) element.style.removeProperty(name); else element.style.setProperty(name, value); };
const lift = (target: HTMLElement, by: number): void => { by = Math.round(by * 1000) / 1000; put(target, LIFT, `${by}px`); put(target, "margin-block-start", by > 0 ? `${-by}px` : null); };
const overlap = (a: DOMRect, b: DOMRect): number => Math.min(a.right, b.right) - Math.max(a.left, b.left);

/** 📐️ Interlocks the cards of a stack or grid `container`: each card whose top lies on the bottom of another card (the one
 * above it in its stack or column) is lifted onto that card's bottom line, so its cap row coincides with the footer row of the
 * card above (one shared band as high as the taller of the two) unless their chips would collide, in which case the rows follow
 * each other. Measures the rows at their natural height, writes `--card-stack-lift` on the card (on its grid cell in a grid) and
 * `--card-stack-cap|footer` on the cards, and keeps doing so while the cards, their chips or the container change. A lift moves the
 * tracks of a grid (fr rows give the freed room to their neighbours), so the measured seams are re-read after the lifts are written and
 * corrected until every overlap equals the band (a few passes), and all heights are snapped together before the first read.
 *
 * @see ./bands.ts */
export function useCardInterlock(container: HTMLElement | null): void {
  React.useLayoutEffect(() => {
    if (!container) return;
    const view = container.ownerDocument.defaultView;
    let frame = 0;
    const observed = new Set<Element>();
    let settled = "";
    let cards: HTMLElement[] | null = null;
    const targetOf = (card: HTMLElement): HTMLElement => card.closest<HTMLElement>("[data-card-grid-cell]") ?? card;
    const measure = () => {
      frame = 0;
      const owned = cards ??= [...container.querySelectorAll<HTMLElement>(CARD)].filter(card => card.closest(OWNER) === container && card.parentElement?.closest(CARD) == null);
      const next = new Set<Element>([container, ...owned]);
      const pixel = 1 / (view?.devicePixelRatio || 1);
      const rows = owned.map(card => { const window = card.querySelector(WINDOW); return { cap: window?.querySelector(`:scope > ${CAP}`) ?? null, footer: window?.querySelector(`:scope > ${FOOTER}`) ?? null }; });
      for (const row of rows) for (const element of [row.cap, row.footer]) for (const chip of element?.querySelectorAll(CHIPS) ?? []) next.add(chip);
      const signature = () => {
        const root = container.getBoundingClientRect();
        const parts: number[] = [root.width, root.height];
        for (const card of owned) {
          const rect = card.getBoundingClientRect();
          parts.push(rect.left - root.left, rect.top - root.top, rect.width, rect.height);
        }
        for (const row of rows) for (const element of [row.cap, row.footer]) {
          parts.push(height(element));
          for (const chip of element?.querySelectorAll(CHIPS) ?? []) {
            const rect = chip.getBoundingClientRect();
            parts.push(rect.left - root.left, rect.width, rect.height);
          }
        }
        return `${pixel}:${parts.map(part => Math.round(part * 1000)).join()}`;
      };
      const sync = () => {
        for (const element of observed) if (!next.has(element)) { resize?.unobserve(element); observed.delete(element); }
        for (const element of next) if (!observed.has(element)) { resize?.observe(element); observed.add(element); }
      };
      if (settled !== "" && signature() === settled) return sync();
      for (const card of owned) { lift(targetOf(card), 0); put(card, CAP_BAND, "0px"); put(card, FOOTER_BAND, "0px"); put(card, "min-block-size", null); put(card.querySelector<HTMLElement>(WINDOW), "min-block-size", null); }
      const snap = (elements: readonly (HTMLElement | null)[]) => { const naturals = elements.map(element => element?.getBoundingClientRect().height ?? 0); elements.forEach((element, index) => { const snapped = snapCardHeight(naturals[index]!, pixel); if (element && snapped - naturals[index]! > 1e-3) put(element, "min-block-size", `${snapped}px`); }); };
      snap(owned.map(card => card.querySelector<HTMLElement>(WINDOW)));
      snap(owned);
      const measured: CardBandCard[] = rows.map(row => ({ cap: height(row.cap), footer: height(row.footer), capChips: spans(row.cap), footerChips: spans(row.footer) }));
      const rects = owned.map(card => card.getBoundingClientRect());
      const seams: { index: number; above: number; target: number; by: number; from: readonly [number, number] }[] = [];
      owned.forEach((card, index) => {
        let above = -1, best = 0;
        rects.forEach((rect, other) => { const shared = overlap(rects[index]!, rect); if (other !== index && Math.abs(rect.bottom - rects[index]!.top) <= REACH && shared > best) { above = other; best = shared; } });
        if (above < 0) return;
        const band = resolveCardBands([measured[above]!, measured[index]!], WINDOW_SILHOUETTE_PATH_INSET)[0]!;
        const by = band.lift - (rects[above]!.bottom - rects[index]!.top);
        seams.push({ index, above, target: band.lift, by, from: [0, by] });
        lift(targetOf(card), by);
        put(card, CAP_BAND, `${band.cap}px`);
        put(owned[above]!, FOOTER_BAND, `${band.footer}px`);
      });
      for (let pass = 0; pass < SETTLE_PASSES && seams.length; pass++) {
        const placed = owned.map(card => card.getBoundingClientRect());
        let moved = false;
        for (const seam of seams) {
          const error = seam.target - (placed[seam.above]!.bottom - placed[seam.index]!.top);
          if (Math.abs(error) <= 5e-3) continue;
          const slope = seam.by === seam.from[0] ? 0 : (error - seam.from[1]) / (seam.by - seam.from[0]);
          const by = slope < 0 ? seam.by - error / slope : seam.by + error;
          seam.from = [seam.by, error];
          seam.by = by;
          lift(targetOf(owned[seam.index]!), by);
          moved = true;
        }
        if (!moved) break;
      }
      settled = signature();
      sync();
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
    const relevant = (node: Node) => node.nodeType === 1 && (node as Element).namespaceURI === "http://www.w3.org/1999/xhtml" && ((node as Element).matches(STRUCTURE) || (node as Element).querySelector(STRUCTURE) !== null);
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
      for (const card of container.querySelectorAll<HTMLElement>(CARD)) { targetOf(card).style.removeProperty(LIFT); targetOf(card).style.removeProperty("margin-block-start"); card.style.removeProperty(CAP_BAND); card.style.removeProperty(FOOTER_BAND); card.style.removeProperty("min-block-size"); card.querySelector<HTMLElement>(WINDOW)?.style.removeProperty("min-block-size"); }
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

/** 🧱️ A column of cards that interlock: no gap and no page margin between them, the container touches whatever lies above
 * and below it, the first card's cap row starts on the container's top edge and the last card ends on its bottom edge.
 * The next card's cap row starts on the previous card's bottom line, sharing one band with the previous card's footer chips
 * (cap chip left, footer chip right; if they would collide the band holds both rows one after the other, see
 * {@link useCardInterlock}), and the shared-border compositor paints each coincident line once. Domain-neutral: it knows
 * only `OverviewCard`s (`data-overview-card`), whatever else it holds lies in the column as ordinary children.
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
 * Cells are {@link CardGridCell}s or any element marked `data-card-grid-cell`. Cards of consecutive rows interlock like
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
