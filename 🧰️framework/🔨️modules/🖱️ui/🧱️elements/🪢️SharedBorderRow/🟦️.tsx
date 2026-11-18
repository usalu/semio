import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { useSharedBorderFrameRef } from "../🗂️WindowChrome/shared-border.ts";

/** 🪢️ Direction in which the cells of a {@link SharedBorderRow} follow each other. */
export type SharedBorderRowOrientation = "horizontal" | "vertical";

/** 🪢️ A physical side of a framed {@link SharedBorderRow}. */
export type SharedBorderRowEdge = "top" | "right" | "bottom" | "left";

/** 🪢️ Colour tone of a {@link SharedBorderRow} or cell: `destructive` paints the destructive border and surface and takes part in seam priority like any other state. */
export type SharedBorderRowTone = "default" | "destructive";

/** 🪢️ Native element a row or cell renders as: lists use `ul`/`ol` with `li` items. */
export type SharedBorderRowElement = "div" | "ul" | "ol" | "li" | "section" | "nav" | "form" | "fieldset" | "dl" | "header" | "aside" | "footer";

/** 🪢️ Props of {@link SharedBorderRow}. `framed={false}` makes the row a group only: an enclosing owner (a card, a window) paints the outer edges. `as` picks the element (`ul` for lists); `ref` is forwarded to it. */
export interface SharedBorderRowProps extends Omit<React.ComponentProps<"div">, "ref"> {
  readonly wrap?: boolean;
  readonly as?: SharedBorderRowElement;
  readonly ref?: React.Ref<HTMLElement>;
  readonly orientation?: SharedBorderRowOrientation;
  readonly framed?: boolean;
  readonly edges?: readonly SharedBorderRowEdge[];
  readonly tone?: SharedBorderRowTone;
}

const allEdges: readonly SharedBorderRowEdge[] = ["top", "right", "bottom", "left"];

/** 🧮️ Where one cell sits in a wrapped row: `end` closes its line (no divider to a next cell), `last` lies on the final line (no divider to a line below). */
export interface SharedBorderWrapPlacement {
  readonly end: boolean;
  readonly last: boolean;
}

/**
 * 🧮️ Pure line classification of a wrapped row: `tops` are the block offsets of the cells in document order; cells whose offsets differ by at most `tolerance` share a line.
 *
 * @see ./wrap.cases.json language-agnostic cases
 */
export function sharedBorderWrapPlacements(tops: readonly number[], tolerance = 1): readonly SharedBorderWrapPlacement[] {
  const bottom = Math.max(...tops);
  return tops.map((top, index) => ({ end: index === tops.length - 1 || Math.abs(tops[index + 1]! - top) > tolerance, last: Math.abs(bottom - top) <= tolerance }));
}

const wrapCellSelector = ":scope > :is(.shared-border-cell, [data-shared-border-cell], [data-shared-border-frame])";

/**
 * 🧮️ Keeps `data-wrap-end` / `data-wrap-last` on the cells of a wrapped row current while it resizes or its cells change, so wrapped cells share one hairline across lines instead of drawing doubled edges at the frame.
 * `row` holds the wrapping element; the hook is inactive (and leaves no attributes) while `enabled` is false.
 */
export function useSharedBorderWrap(row: React.RefObject<HTMLElement | null>, enabled: boolean): void {
  React.useLayoutEffect(() => {
    const element = row.current;
    if (!element || !enabled) return;
    let cells: HTMLElement[] = [];
    const resize = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(() => apply());
    const apply = () => {
      const next = [...element.querySelectorAll<HTMLElement>(wrapCellSelector)];
      if (next.length !== cells.length || next.some((cell, index) => cell !== cells[index])) {
        resize?.disconnect();
        resize?.observe(element);
        for (const cell of next) resize?.observe(cell);
        for (const cell of cells) if (!next.includes(cell)) { cell.removeAttribute("data-wrap-end"); cell.removeAttribute("data-wrap-last"); }
        cells = next;
      }
      const placements = cells.length ? sharedBorderWrapPlacements(cells.map(cell => cell.offsetTop)) : [];
      cells.forEach((cell, index) => {
        const { end, last } = placements[index]!;
        if (cell.hasAttribute("data-wrap-end") !== end) cell.toggleAttribute("data-wrap-end", end);
        if (cell.hasAttribute("data-wrap-last") !== last) cell.toggleAttribute("data-wrap-last", last);
      });
    };
    const mutation = typeof MutationObserver === "undefined" ? null : new MutationObserver(apply);
    mutation?.observe(element, { childList: true });
    apply();
    return () => {
      resize?.disconnect();
      mutation?.disconnect();
      for (const cell of cells) { cell.removeAttribute("data-wrap-end"); cell.removeAttribute("data-wrap-last"); }
    };
  }, [enabled, row]);
}

/**
 * 🪢️ One bordered row (or column) of cells: the replacement for ad-hoc `rounded border px-* py-*` boxes.
 *
 * The row is the only owner of its outer edges, painted by the shared-border compositor. Its direct children are cells:
 * framed controls (`Button`, `Input`, `Textarea`, `Select` trigger, `Toggle`, `ToggleGroup`, `ActionGroup`) lose their own
 * frame, share the row's edges, stretch across the row and keep one divider to their neighbour; plain content goes into a
 * {@link SharedBorderRowCell}, a label into a {@link SharedBorderRowLabel}, a native `select`/`input`/`textarea`/label pair into a
 * {@link SharedBorderRowField}. A scroller belongs INSIDE the row (in a cell), never around it: an `overflow` ancestor clips the painted edges.
 * `wrap` lets a horizontal row break into lines whose cells stretch to fill each line and share one hairline across lines (see {@link useSharedBorderWrap}).
 *
 * @see ../🗂️WindowChrome/shared-border.ts `isSharedBorderCellFrame`
 * @see ../../🎨️styling/🖌️ui/🎨️.css `data-shared-border-cells`
 */
export function SharedBorderRow({ as: TagName = "div", orientation = "horizontal", framed = true, edges = allEdges, tone = "default", wrap = false, className, ref, children, ...props }: SharedBorderRowProps) {
  const frameRef = useSharedBorderFrameRef<HTMLElement>(ref);
  const rowRef = React.useRef<HTMLElement | null>(null);
  useSharedBorderWrap(rowRef, wrap);
  const setRef = React.useCallback((node: HTMLElement | null) => {
    rowRef.current = node;
    if (framed) return frameRef(node);
    if (typeof ref === "function") return ref(node);
    if (ref) ref.current = node;
  }, [frameRef, framed, ref]);
  const Tag = TagName as "div";
  return (
    <Tag
      ref={setRef as React.Ref<HTMLDivElement>}
      data-slot="shared-border-row"
      data-shared-border-cells=""
      data-shared-border-wrap={wrap ? "" : undefined}
      data-shared-border-frame={framed ? "" : undefined}
      data-shared-border-group={framed ? undefined : ""}
      data-shared-border-edges={framed ? edges.join(" ") : undefined}
      data-orientation={orientation}
      data-tone={tone === "default" ? undefined : tone}
      className={cn("relative flex min-w-0", orientation === "vertical" ? "flex-col items-stretch" : "items-stretch", wrap && orientation === "horizontal" && "flex-wrap", className)}
      {...props}
    >
      {children}
    </Tag>
  );
}

/** 🪢️ Props of {@link SharedBorderRowCell}: `as` picks the element (`li` inside a `ul` row), `ref` is forwarded to it. */
export interface SharedBorderRowCellProps extends Omit<React.ComponentProps<"div">, "ref"> {
  readonly as?: SharedBorderRowElement;
  readonly ref?: React.Ref<HTMLElement>;
  readonly tone?: SharedBorderRowTone;
}

/** 🪢️ Plain (non-control) content of a {@link SharedBorderRow}: a cell with the standard cell padding and a single divider to its neighbour. */
export function SharedBorderRowCell({ as: TagName = "div", tone = "default", className, ref, ...props }: SharedBorderRowCellProps) {
  const Tag = TagName as "div";
  return <Tag ref={ref as React.Ref<HTMLDivElement>} data-slot="shared-border-row-cell" data-shared-border-cell="" data-tone={tone === "default" ? undefined : tone} className={cn("flex min-w-0 items-center px-single py-1", className)} {...props} />;
}

/** 🪢️ Props of {@link SharedBorderRowLabel}: `width="fixed"` gives every label of a form the same column (`--shared-border-label-width`, default 40%), `auto` sizes to the text. */
export interface SharedBorderRowLabelProps extends Omit<React.ComponentProps<"label">, "ref"> {
  readonly width?: "fixed" | "auto";
  readonly ref?: React.Ref<HTMLLabelElement>;
}

/** 🪢️ Label cell of a {@link SharedBorderRow}: a real `<label>` in a cell so the label/control pair shares one hairline. */
export function SharedBorderRowLabel({ width = "fixed", className, children, ...props }: SharedBorderRowLabelProps) {
  return (
    <div data-slot="shared-border-row-label" data-shared-border-cell="" data-label-width={width} className={cn("flex min-w-0 items-center px-single py-1", width === "fixed" ? "w-[var(--shared-border-label-width,40%)] shrink-0" : "shrink-0")}>
      <label className={cn("truncate text-xs text-muted-foreground", className)} {...props}>{children}</label>
    </div>
  );
}

/** 🪢️ Props of {@link SharedBorderRowField}: a `<label>` cell with optional label text around one native control. */
export interface SharedBorderRowFieldProps extends Omit<React.ComponentProps<"label">, "ref"> {
  readonly label?: React.ReactNode;
  readonly ref?: React.Ref<HTMLLabelElement>;
}

/**
 * 🪢️ A native field cell: `<label>` wrapping label text and one native `select`, `input` (any type, incl. `color`), `textarea` or `contentEditable`
 * surface. The control loses its own box (border, radius, background) and fills the cell; the cell carries the divider.
 */
export function SharedBorderRowField({ label, className, children, ...props }: SharedBorderRowFieldProps) {
  return (
    <label data-slot="shared-border-row-field" data-shared-border-cell="" data-shared-border-field="" className={cn("flex min-w-0 flex-1 items-center gap-single px-single py-1 text-xs", className)} {...props}>
      {label === undefined ? null : <span className="shrink-0">{label}</span>}
      {children}
    </label>
  );
}

export { useSharedBorderFrameRef };
