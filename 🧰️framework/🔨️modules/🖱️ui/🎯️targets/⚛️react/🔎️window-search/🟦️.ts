import { normalizeEngagementActionText, type SearchPossible, searchInlineCompletion, searchActiveInlineCompletion } from "@semio-tech/ui/elements/WindowSearch";

/** 🏷️ Default English copy for the window search pane (matches `ui.windowSearch.*` en bundle) — for standalone REPL surfaces that build a {@link SearchSpec} outside {@link Search}'s own `useLabel` resolution. */
export const WINDOW_SEARCH_USER = {
  actionPlaceholder: "Action",
  actionPlaceholderActive: "Action or value",
  suggestionsAria: "Suggestions",
  noMatches: "No matches",
} as const;

/** ⚖️ True when two engagement action tokens match after {@link normalizeEngagementActionText} (case-insensitive). */
export function engagementActionTokenEquals(a: string, b: string): boolean {
  return normalizeEngagementActionText(a).toLowerCase() === normalizeEngagementActionText(b).toLowerCase();
}

/** 🎯️ Resolves a pointer event target to an element for search suggestion hit-testing. */
function searchSuggestionPointerTarget(event: Pick<PointerEvent, "target">): Element | null {
  const target = event.target;
  if (target instanceof Element) return target;
  if (target instanceof Text) return target.parentElement;
  return null;
}

/** 🔎️ True when a pointer event targets a search suggestion action row. */
export function isSearchSuggestionActionTarget(event: Pick<PointerEvent, "target">): boolean {
  return Boolean(searchSuggestionPointerTarget(event)?.closest('[data-slot="command-item"]'));
}

/** ⌨️ Inline completion suffix for one {@link SearchPossible} (longest prefix match on label, detail, or id). */
export function searchCompletionSuffix(query: string, item: SearchPossible | undefined): string {
  return searchInlineCompletion(query, item)?.suffix ?? "";
}

/** ⏭️ First non-empty inline completion suffix across ranked {@link SearchPossible} matches. */
export function searchActiveCompletionSuffix(query: string, matches: readonly SearchPossible[], index: number): string {
  return searchActiveInlineCompletion(query, matches, index)?.suffix ?? "";
}
