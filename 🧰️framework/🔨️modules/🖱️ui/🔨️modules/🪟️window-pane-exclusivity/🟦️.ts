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
