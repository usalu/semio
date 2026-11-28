/** 🪟️ Narrow windows show one chrome pane body at a time: the chips stay a row of tabs, panes never overlap. */
export type WindowPaneId = "engagement" | "search" | "measures" | "utility";

export type WindowPaneOpen = Readonly<Record<WindowPaneId, boolean>>;

export const windowPanePriority: readonly WindowPaneId[] = ["engagement", "search", "measures", "utility"];

/** 📏️ A window body narrower than this cannot hold two pane bodies side by side. */
export const windowPaneExclusiveMaxWidthPx = 480;

/** 🔎️ The pane bodies that are actually shown: everything open, or in an exclusive (narrow) window only the pane opened last, falling back to the highest priority open pane. */
export function resolveWindowPaneExpansion(open: WindowPaneOpen, last: WindowPaneId | null, exclusive: boolean): WindowPaneOpen {
  if (!exclusive) return open;
  const shown = last && open[last] ? last : windowPanePriority.find((id) => open[id]);
  return { engagement: shown === "engagement", search: shown === "search", measures: shown === "measures", utility: shown === "utility" };
}

/** 🕳️ The quick-action group hangs under the Actions chip and would be covered by an open top-row pane body in an exclusive window. */
export function windowQuickActionsCovered(expanded: WindowPaneOpen, exclusive: boolean): boolean {
  return exclusive && (expanded.search || expanded.measures);
}

/** 📐️ The horizontal span one open pane covers, in pixels from the left edge of the window body. */
export type WindowPaneSpanV1 = { readonly leftPx: number; readonly rightPx: number };

/** 🧱️ The padding that keeps a window's viewport beside its open panes. */
export type WindowViewportInsetV1 = { readonly leftPx: number; readonly rightPx: number };

/** 🔭️ A viewport narrower than this is not worth keeping beside the panes; they overlay it instead. */
export const windowViewportMinWidthPx = 160;

/** 🔭️ A pane or panel only claims the viewport's side when it covers at least this share of the window body's height; shorter ones overlay it. */
export const windowViewportCoverMinShare = 0.5;

/** 🔭️ Where a window's viewport keeps rendering while panes are open: the widest strip of the body that no pane covers, or no inset when that strip is narrower than {@link windowViewportMinWidthPx}. */
export function windowViewportInsetV1(bodyWidthPx: number, panes: readonly WindowPaneSpanV1[], minViewportPx: number = windowViewportMinWidthPx): WindowViewportInsetV1 {
  const none: WindowViewportInsetV1 = { leftPx: 0, rightPx: 0 };
  const covered = panes
    .map((pane) => ({ from: Math.max(0, pane.leftPx), to: Math.min(bodyWidthPx, pane.rightPx) }))
    .filter((span) => span.to > span.from)
    .sort((a, b) => a.from - b.from);
  if (covered.length === 0) return none;
  let best = { from: 0, to: 0 };
  let cursor = 0;
  for (const span of covered) {
    if (span.from - cursor > best.to - best.from) best = { from: cursor, to: span.from };
    cursor = Math.max(cursor, span.to);
  }
  if (bodyWidthPx - cursor > best.to - best.from) best = { from: cursor, to: bodyWidthPx };
  return best.to - best.from < minViewportPx ? none : { leftPx: Math.round(best.from), rightPx: Math.round(bodyWidthPx - best.to) };
}

/** 📦️ A rectangle in viewport pixels. */
export type WindowRectV1 = { readonly left: number; readonly right: number; readonly top: number; readonly bottom: number };

/** 🛬️ How far the shell docks that lie over the lower half of a window body reach up from its bottom edge: top-anchored panes stop that much higher, so their last rows and footers are never covered. Docks that miss the body, or reach into its upper half (they claim the viewport's side instead, {@link windowViewportCoverMinShare}), reserve nothing. */
export function windowBottomDockReservePx(body: WindowRectV1, docks: readonly WindowRectV1[]): number {
  const middle = body.top + (body.bottom - body.top) * windowViewportCoverMinShare;
  return Math.round(docks.reduce((reserve, dock) => dock.right > body.left && dock.left < body.right && dock.bottom > dock.top && dock.top >= middle && dock.top < body.bottom ? Math.max(reserve, body.bottom - dock.top) : reserve, 0));
}
