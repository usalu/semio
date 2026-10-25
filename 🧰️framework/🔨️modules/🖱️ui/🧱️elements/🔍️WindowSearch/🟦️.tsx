/** 🔍 Owns the window search line, its completion model and the key routing that focuses it. */
import * as React from "react";
import { reactHostPort } from "../🔌️Ports/⚛️react/🟦️.ts";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { interactiveActiveFillClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
import { ChevronDownIcon, ChevronLeftIcon, ChevronRightIcon } from "../🔣️Icons/🟦️.tsx";
import { isInternalChromeControlId } from "../🚗️UiDriver/🟦️.tsx";
import { type UiLabel } from "../🎗️UiLabel/🟦️.tsx";
import { type UiTranslationKey } from "../📚️I18n/🟦️.tsx";
import { LevelProvider } from "../🌈️Surface/🟦️.tsx";
import { useFlow } from "../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { Command, CommandEmpty, CommandGroup, CommandItem, CommandList } from "../⌨️Command/🟦️.tsx";
import { Popover, PopoverAnchor, PopoverContent } from "../🗨️Popover/🟦️.tsx";
import { useLabel } from "../🏷️Label/🟦️.tsx";
import { Action } from "../⚡️ActionGroup/🟦️.tsx";
import { Input } from "../✏️Input/🟦️.tsx";

/** 🏷️ i18n keys for the window search pane (`ui.windowSearch.*` in {@link uiChromeTranslationBundles}; distinct from the `ui.search.*` command palette). */
export const UI_WINDOW_SEARCH = {
  title: "ui.windowSearch.title",
  action: "ui.windowSearch.action",
  actionActive: "ui.windowSearch.actionActive",
  suggestions: "ui.windowSearch.suggestions",
  noMatches: "ui.windowSearch.noMatches",
} as const satisfies Record<string, UiTranslationKey>;

/** 🔤️ Normalizes an action NAME for display and matching: no separators, PascalCase tokens (`set height` → `SetHeight`, `box` → `Box`), preserving decimal points inside numbers (`3.5` stays `3.5`, not `35`). Names only — the typed engagement line reaches the program verbatim (a grammar like `move <dx> <dy>` needs its spaces), so this never touches {@link SearchInput.onChange}/`onSubmit` payloads. */
export function normalizeEngagementActionText(text: string): string {
  const decimalMarker = "\u0001";
  const withProtectedDecimals = text.replace(/(\d)\.(\d)/g, `$1${decimalMarker}$2`);
  const words = withProtectedDecimals
    .replace(new RegExp(`[^a-zA-Z0-9${decimalMarker}]+`, "g"), " ")
    .trim()
    .split(/\s+/)
    .filter(Boolean)
    .flatMap((word) => word.split(/(?=[A-Z])/))
    .filter(Boolean)
    .map((word) => word.split(decimalMarker).join("."));
  return words.map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()).join("");
}

/** ⌨️ One typed action line for a window's {@link SearchSpec}. */
export interface SearchInput {
  id?: string;
  value?: string;
  placeholder?: UiLabel;
  onChange?: (value: string) => void;
  onSubmit?: (value: string) => void;
  /** 🔁️ Restarts the last finalized engagement when Space is pressed with an empty action. */
  onRepeatLast?: () => void;
  /** 🛑️ ⎋️ Cancels the active engagement session (Escape), e.g. abort interaction or clear action. */
  onAbort?: () => void;
  disabled?: boolean;
}

/** 📕️ One autocomplete row for {@link SearchSpec.possibles} (interaction, transition, …). */
export interface SearchPossible {
  id: string;
  label: string;
  detail?: string;
  onSelect?: () => void;
}

function searchPossibleRankScore(query: string, item: SearchPossible): number {
  const ql = normalizeEngagementActionText(query).toLowerCase();
  if (!ql) return -1;
  const label = normalizeEngagementActionText(item.label).toLowerCase();
  const detail = (item.detail ?? "").toLowerCase();
  const id = item.id.toLowerCase();
  if (label.startsWith(ql)) return 3000 - label.length;
  if (detail.startsWith(ql)) return 2000 - detail.length;
  if (id.startsWith(ql)) return 1000 - id.length;
  const haystack = `${label} ${detail} ${id}`;
  if (haystack.includes(ql)) return 500;
  return -1;
}

/** 🔎️ Filters {@link SearchPossible} rows by label, detail, and id for the window search action line. */
export function filterSearchPossibles(query: string, items: readonly SearchPossible[]): SearchPossible[] {
  const trimmed = normalizeEngagementActionText(query).toLowerCase();
  if (!trimmed) return [...items];
  return items
    .map((item) => ({ item, score: searchPossibleRankScore(query, item) }))
    .filter((row) => row.score >= 0)
    .sort((a, b) => b.score - a.score)
    .map((row) => row.item);
}

/** 🔖️ Inline completion segments for one {@link SearchPossible} using label casing for the matched name prefix. */
export interface SearchInlineCompletion {
  readonly prefix: string;
  readonly suffix: string;
}

/** ↩️ Returns PascalCase inline completion when query prefix-matches the possible's name, detail, or id. */
export function searchInlineCompletion(query: string, item: SearchPossible | undefined): SearchInlineCompletion | null {
  if (!query.trim() || !item) return null;
  const q = query;
  const ql = q.toLowerCase();
  const label = normalizeEngagementActionText(item.label);
  let best: SearchInlineCompletion | null = null;
  const consider = (matched: boolean) => {
    if (!matched || !label.toLowerCase().startsWith(ql)) return;
    const prefix = label.slice(0, q.length);
    const suffix = label.slice(q.length);
    if (!suffix.length) return;
    if (!best || suffix.length > best.suffix.length) best = { prefix, suffix };
  };
  consider(label.toLowerCase().startsWith(ql));
  consider(Boolean(item.detail?.toLowerCase().startsWith(ql)));
  consider(item.id.toLowerCase().startsWith(ql));
  return best;
}

/** ⏮️ First non-empty inline completion across ranked {@link SearchPossible} matches. */
export function searchActiveInlineCompletion(query: string, matches: readonly SearchPossible[], index: number): SearchInlineCompletion | null {
  if (!query.trim() || !matches.length) return null;
  const order = [matches[Math.min(index, matches.length - 1)]!, ...matches];
  const seen = new Set<SearchPossible>();
  for (const item of order) {
    if (seen.has(item)) continue;
    seen.add(item);
    const completion = searchInlineCompletion(query, item);
    if (completion) return completion;
  }
  return null;
}

/** 🖍️ Renders a possible name with the query prefix emphasized using label casing (e.g. **B**ox). */
export function searchHighlightedLabel(label: string, query: string, detail?: string): React.ReactNode {
  const displayLabel = normalizeEngagementActionText(label);
  const trimmed = normalizeEngagementActionText(query);
  if (!trimmed) return displayLabel;
  const ql = trimmed.toLowerCase();
  const ll = displayLabel.toLowerCase();
  let start = ll.startsWith(ql) ? 0 : -1;
  if (start < 0 && detail?.toLowerCase().startsWith(ql)) start = ll.indexOf(ql) >= 0 ? ll.indexOf(ql) : ll.indexOf(ql[0] ?? "");
  if (start < 0) return displayLabel;
  const end = start + trimmed.length;
  return (
    <>
      {start > 0 ? <span>{displayLabel.slice(0, start)}</span> : null}
      <span className="font-semibold text-foreground">{displayLabel.slice(start, end)}</span>
      <span>{displayLabel.slice(end)}</span>
    </>
  );
}

/** 🚫️ React props that disable native browser affordances on editable UI controls. */


/** 🏹️ True when the event target should receive typed characters (skip engagement routing and global REPL capture). */
export function isUiTypingTarget(t: EventTarget | null): boolean {
  if (!(t instanceof HTMLElement)) return false;
  if (t instanceof HTMLTextAreaElement || t instanceof HTMLSelectElement) return true;
  if (t.isContentEditable) return true;
  if (t.closest('[role="textbox"]')) return true;
  if (t instanceof HTMLInputElement) {
    const kind = (t.type || "text").toLowerCase();
    return kind !== "button" && kind !== "checkbox" && kind !== "radio" && kind !== "file" && kind !== "range" && kind !== "color";
  }
  if (t.closest('[data-slot="input-root"], [data-slot="textarea-root"], [data-collapsed="true"], [data-slot="command-input"], [data-slot="select-trigger"], [data-slot="select-content"]')) {
    return true;
  }
  return Boolean(t.closest('[data-slot="search"] input, [data-slot="search"] textarea'));
}

/** 🪟️ True when the event target is already the active window search action field. */
export function isWindowSearchTypingTarget(t: EventTarget | null): boolean {
  if (!(t instanceof HTMLElement)) return false;
  return Boolean(t.closest('[data-slot="window"][data-active="true"] [data-slot="search"][data-active="true"] [data-slot="input"], [data-slot="window"][data-active="true"] [data-slot="search"][data-active="true"] textarea'));
}

/** 🔐️ True when printable keys should route to the active window search action (skip other text fields). */
export function shouldRouteKeysToWindowSearch(t: EventTarget | null): boolean {
  if (isWindowSearchTypingTarget(t)) return false;
  const searchField = queryWindowSearchInput(true) ?? queryWindowSearchInput(false);
  const active = document.activeElement;
  if (searchField && (active === searchField || searchField.contains(active))) return false;
  if (active instanceof HTMLElement && isUiTypingTarget(active) && !active.closest('[data-slot="search"]')) return false;
  if (isUiTypingTarget(t)) return false;
  return true;
}

/** 📥️ Returns the window search action input, optionally requiring {@link SearchProps.active}. */
export function queryWindowSearchInput(activeOnly = false): HTMLInputElement | null {
  const searchActive = activeOnly ? '[data-active="true"]' : "";
  return document.querySelector<HTMLInputElement>(`[data-slot="window"][data-active="true"] [data-slot="search"]${searchActive} [data-slot="input"]`);
}

/** 🎤️ Focuses the action input in the active window search overlay, if present. */
export function focusActiveSearchInput(): boolean {
  const active = document.activeElement;
  if (active instanceof HTMLElement && active.closest('[data-slot="engagement-control"]')) return false;
  const field = queryWindowSearchInput(true) ?? queryWindowSearchInput(false);
  if (!field || field.disabled) return false;
  field.focus({ preventScroll: true });
  return true;
}

/** ✍️ One local edit of a controlled search line: the text the user typed, the published value
 * that stood when the edit began, and every line this edit already dispatched through `onChange` (a
 * program that echoes the line back republishes exactly those). */
export interface SearchLineEdit {
  readonly text: string;
  readonly base: string;
  readonly sent?: readonly string[];
}

/** 🧾️ How many dispatched lines one edit remembers for echo detection — a slow program may still be
 * echoing the first keystrokes of a long verb when the last ones are typed. */
export const SEARCH_LINE_EDIT_SENT_LIMIT = 64;

/**
 * 📃️ Which line a controlled window search field shows. The published value is a program's
 * property, and a program that stores the line without republishing it (or that answers a full round trip
 * later) used to make the field unwritable: every keystroke dispatched `onChange` and the field snapped
 * straight back to the stale `value`, so every submit carried an empty line. The local edit therefore
 * LEADS, and the published value only wins once it moved away from what stood when the edit began — which
 * is exactly the case where the program authored the line itself (a completed submit, an abort, a
 * program-side rewrite) and the user's draft is stale instead. A program that ECHOES each keystroke
 * (the cad engagement input republishes `engagement-input` per `onChange`, one guest round trip behind)
 * publishes lines this edit itself sent; those are not authorship, so a published value the edit
 * already dispatched keeps the typed text — otherwise "PlaceColumn" typed at 40 ms/key collapsed to "Pn".
 */
export function searchControlledLineV1(published: string, edit: SearchLineEdit | null): string {
  if (!edit) return published;
  if (published === edit.base) return edit.text;
  return edit.sent?.includes(published) ? edit.text : published;
}

/** ✅️ True when Space/Enter should pick the active filtered {@link SearchPossible} instead of submitting the raw draft. */
export function shouldActivateSearchPossibleOnConfirm(draft: string, showPossiblesList: boolean, filteredCount: number): boolean {
  if (!filteredCount) return false;
  return showPossiblesList || Boolean(draft.trim());
}

/**
 * 🧲️ ␣️ Whether Space CONFIRMS the line inside the action field instead of typing a separator.
 * An empty line confirms (repeat-last when idle, step submit during a session) and a live engagement
 * session keeps the step-value grammar where each Space commits the typed number. A typed line with no
 * session is prose the program tokenizes itself (`move 50 25`), so Space belongs in the text.
 */
export function searchSpaceConfirmsLine(draft: string, sessionActive: boolean): boolean {
  return !draft.trim() || sessionActive;
}

/** 📱️ ␣️ Applies Space on a window search action line (step submit vs repeat-last when idle). */
export function applySearchSpaceAction(input: SearchInput, draft: string, sessionActive: boolean): boolean {
  if (input.disabled) return false;
  if (!draft.trim()) {
    if (sessionActive) {
      if (!input.onSubmit) return false;
      input.onSubmit(draft.trim());
      return true;
    }
    if (!input.onRepeatLast) return false;
    input.onRepeatLast();
    return true;
  }
  if (!input.onSubmit) return false;
  input.onSubmit(draft.trim());
  return true;
}

/** 🌌️ Routes Space outside the search field: idle empty → {@link SearchInput.onRepeatLast}; session or typed draft → {@link SearchInput.onSubmit}. */
export function routeWindowSearchSpace(search: SearchSpec | undefined, event: Pick<KeyboardEvent, "key" | "ctrlKey" | "metaKey" | "altKey" | "defaultPrevented" | "isComposing" | "target">): boolean {
  const input = search?.input;
  if (!input || event.defaultPrevented || event.isComposing) return false;
  if (event.key !== " " || event.ctrlKey || event.metaKey || event.altKey) return false;
  if (!shouldRouteKeysToWindowSearch(event.target)) return false;
  const field = queryWindowSearchInput(true) ?? queryWindowSearchInput(false);
  const draft = input.value ?? field?.value ?? "";
  return applySearchSpaceAction(input, draft, Boolean(search?.sessionActive));
}

/** 🗺️ Routes a printable key to the active window search action when focus is elsewhere in the window. */
export function routeWindowSearchKeydown(search: SearchSpec | undefined, event: Pick<KeyboardEvent, "key" | "ctrlKey" | "metaKey" | "altKey" | "defaultPrevented" | "isComposing" | "target">): boolean {
  const input = search?.input;
  if (!input || input.disabled || event.defaultPrevented || event.isComposing) return false;
  if (!shouldRouteKeysToWindowSearch(event.target)) return false;
  if (event.key === " ") return false;
  const printable = event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey;
  if (!printable) return false;
  const field = queryWindowSearchInput(true) ?? queryWindowSearchInput(false);
  const next = `${input.value ?? field?.value ?? ""}${event.key}`;
  input.onChange?.(next);
  return true;
}

/** 📈️ ⎋️ Routes Escape to {@link SearchInput.onAbort} when window search chrome is active (skips other typing targets). */
export function routeWindowSearchEscape(search: SearchSpec | undefined, event: Pick<KeyboardEvent, "key" | "defaultPrevented" | "isComposing" | "target">, zone: { readonly chromeVisible: boolean; readonly actionActive: boolean }): boolean {
  if (event.key !== "Escape" || event.defaultPrevented || event.isComposing) return false;
  const onAbort = search?.input?.onAbort;
  if (!onAbort) return false;
  if (!search?.sessionActive && !zone.chromeVisible && !zone.actionActive) return false;
  if (isUiTypingTarget(event.target) && !isWindowSearchTypingTarget(event.target)) return false;
  const focused = document.activeElement;
  if (focused instanceof HTMLElement && isUiTypingTarget(focused) && !focused.closest('[data-slot="search"]')) return false;
  onAbort();
  return true;
}

/** 🔝️ Floating top-middle window search payload: typed action input and autocomplete possibles. */
export interface SearchSpec {
  /** 🎯️ Ongoing engagement: chrome stays visible; action input accepts step values. */
  sessionActive?: boolean;
  input?: SearchInput;
  possibles?: SearchPossible[];
}

export interface SearchProps extends SearchSpec {
  className?: string;
  /** 🎬️ When true, focuses the action input whenever this search pane belongs to the globally active window. */
  active?: boolean;
}

/** 👑️ Floating top-middle window search pane: action input with optional right chevron for possibles. */
const Search: React.FC<SearchProps> = ({ sessionActive = false, input, possibles, className = "", active = false }) => {
  const { inline } = useFlow();
  const actionPlaceholderLabel = useLabel(UI_WINDOW_SEARCH.action);
  const actionActivePlaceholderLabel = useLabel(UI_WINDOW_SEARCH.actionActive);
  const suggestionsAriaLabel = useLabel(UI_WINDOW_SEARCH.suggestions);
  const noMatchesLabel = useLabel(UI_WINDOW_SEARCH.noMatches);
  const [uncontrolledDraft, setUncontrolledDraft] = reactHostPort.useState("");
  const [controlledEdit, setControlledEdit] = reactHostPort.useState<SearchLineEdit | null>(null);
  const isControlledInput = !!input?.onChange;
  const publishedLine = input?.value ?? "";
  const draft = isControlledInput ? searchControlledLineV1(publishedLine, controlledEdit) : uncontrolledDraft;
  const actionPlaceholder = input?.placeholder ?? (sessionActive ? actionActivePlaceholderLabel : actionPlaceholderLabel);
  const [possiblesExpanded, setPossiblesExpanded] = reactHostPort.useState(false);
  const [activePossibleIndex, setActivePossibleIndex] = reactHostPort.useState(0);
  const searchRef = reactHostPort.useRef<HTMLDivElement>(null);
  const filteredPossibles = reactHostPort.useMemo(() => filterSearchPossibles(draft, possibles ?? []), [draft, possibles]);

  reactHostPort.useEffect(() => {
    setActivePossibleIndex((index) => (filteredPossibles.length ? Math.min(index, filteredPossibles.length - 1) : 0));
  }, [filteredPossibles.length, draft]);

  reactHostPort.useEffect(() => {
    setPossiblesExpanded(false);
  }, [possibles]);

  const hasInput = !!input;
  const hasPossibles = !!possibles?.length;
  const showPossiblesList = hasPossibles && possiblesExpanded && filteredPossibles.length > 0;
  const inlineCompletion = reactHostPort.useMemo(() => (showPossiblesList ? null : searchActiveInlineCompletion(draft, filteredPossibles, activePossibleIndex)), [activePossibleIndex, draft, filteredPossibles, showPossiblesList]);

  /** 📝️ The typed line reaches the program VERBATIM: an app whose grammar carries arguments
   * (`move 50 25`, `fill 12`) owns its own tokenization, and a name-token grammar normalizes in its
   * own `onChange` (the cad repl PascalCases there). Squashing separators here made every argument
   * collapse into the verb (`move 50 25` → `Move5025`). */
  const applyDraft = reactHostPort.useCallback(
    (value: string) => {
      if (!isControlledInput) {
        setUncontrolledDraft(value);
        return;
      }
      setControlledEdit((previous) => ({ text: value, base: previous?.base ?? publishedLine, sent: [...(previous?.sent ?? []).slice(-(SEARCH_LINE_EDIT_SENT_LIMIT - 1)), value] }));
      input?.onChange?.(value);
    },
    [input, isControlledInput, publishedLine],
  );
  /** 🧹️ Hands the line back to the program on confirm or abort: the published value leads again from
   * here, so a command line the program empties (`engagement_submit` clears `engagement_input`) clears,
   * and a field the program keeps a standing value in keeps it. Never writes a value of its own — that
   * would leave the field empty forever against a program that publishes one. */
  const releaseDraft = reactHostPort.useCallback(() => {
    setControlledEdit(null);
    setUncontrolledDraft("");
  }, []);

  const pickingPossibleIdRef = reactHostPort.useRef<string | null>(null);
  const selectPossible = reactHostPort.useCallback(
    (item: SearchPossible) => {
      if (pickingPossibleIdRef.current === item.id) return;
      pickingPossibleIdRef.current = item.id;
      item.onSelect?.();
      applyDraft("");
      setPossiblesExpanded(false);
      setActivePossibleIndex(0);
      queueMicrotask(() => {
        if (pickingPossibleIdRef.current === item.id) pickingPossibleIdRef.current = null;
      });
    },
    [applyDraft],
  );

  const activatePossible = reactHostPort.useCallback((): boolean => {
    if (!filteredPossibles.length) return false;
    selectPossible(filteredPossibles[activePossibleIndex] ?? filteredPossibles[0]!);
    return true;
  }, [activePossibleIndex, filteredPossibles, selectPossible]);

  const wasActiveRef = reactHostPort.useRef(false);
  reactHostPort.useEffect(() => {
    const becameActive = active && !wasActiveRef.current;
    wasActiveRef.current = active;
    if (!becameActive || !hasInput || input?.disabled) return;
    const focused = document.activeElement;
    if (focused instanceof HTMLElement && isUiTypingTarget(focused) && !focused.closest('[data-slot="search"]')) return;
    const field = searchRef.current?.querySelector<HTMLInputElement>('[data-slot="input"]');
    field?.focus({ preventScroll: true });
  }, [active, hasInput, input?.disabled, input?.id]);

  if (!hasInput) return null;

  return (
    <LevelProvider level="pane">
      <div
        ref={searchRef}
        data-slot="search"
        data-active={active ? "true" : undefined}
        data-session-active={sessionActive ? "true" : undefined}
        data-possibles-open={showPossiblesList ? "true" : undefined}
        className={cn("pointer-events-auto flex w-full min-w-0 max-w-full flex-col gap-half", sessionActive && "bg-active-base ring-accent/35 rounded-sm ring-1 shadow-sm", className)}
      >
        <Popover
          open={showPossiblesList}
          onOpenChange={(open) => {
            if (!open) setPossiblesExpanded(false);
          }}
        >
          <PopoverAnchor asChild>
            <div data-slot="search-row" className="flex w-full min-w-0 items-stretch gap-half">
              <div data-slot="search-input" className="relative grid min-w-0 flex-1 [&_[data-slot=input-root]]:col-start-1 [&_[data-slot=input-root]]:row-start-1 [&_[data-slot=input-root]]:min-w-0">
                <Input
                  id={!input!.id || input!.id === "search-input" || isInternalChromeControlId(input!.id) ? UI_WINDOW_SEARCH.action : input!.id}
                  className="relative z-[1] min-w-0 flex-1 bg-transparent"
                  value={draft}
                  tabIndex={active ? 0 : -1}
                  onChange={(event) => applyDraft(event.target.value)}
                  onKeyDown={(event) => {
                    if (event.key === "Escape") {
                      if (showPossiblesList) {
                        event.preventDefault();
                        setPossiblesExpanded(false);
                        return;
                      }
                      if (input!.onAbort) {
                        event.preventDefault();
                        event.stopPropagation();
                        input!.onAbort();
                        releaseDraft();
                      }
                      return;
                    }
                    if (event.key === "Tab" && !showPossiblesList && inlineCompletion) {
                      event.preventDefault();
                      applyDraft(inlineCompletion.prefix + inlineCompletion.suffix);
                      return;
                    }
                    if (event.key === "ArrowDown" && filteredPossibles.length) {
                      event.preventDefault();
                      setActivePossibleIndex((index) => (index + 1) % filteredPossibles.length);
                      return;
                    }
                    if (event.key === "ArrowUp" && filteredPossibles.length) {
                      event.preventDefault();
                      setActivePossibleIndex((index) => (index - 1 + filteredPossibles.length) % filteredPossibles.length);
                      return;
                    }
                    if (event.key === " " && !event.ctrlKey && !event.metaKey && !event.altKey) {
                      if (shouldActivateSearchPossibleOnConfirm(draft, showPossiblesList, filteredPossibles.length) && activatePossible()) {
                        event.preventDefault();
                        return;
                      }
                      if (!searchSpaceConfirmsLine(draft, sessionActive)) return;
                      event.preventDefault();
                      if (applySearchSpaceAction(input!, draft, sessionActive)) releaseDraft();
                      return;
                    }
                    if (event.key === "Enter") {
                      event.preventDefault();
                      if (shouldActivateSearchPossibleOnConfirm(draft, showPossiblesList, filteredPossibles.length) && activatePossible()) return;
                      input!.onSubmit?.(draft.trim());
                      releaseDraft();
                    }
                  }}
                  placeholder={actionPlaceholder}
                  disabled={input!.disabled}
                  aria-label={actionPlaceholder}
                />
                {inlineCompletion ? (
                  <div aria-hidden data-slot="search-inline-completion" className="text-foreground pointer-events-none col-start-1 row-start-1 flex h-medium min-w-0 items-center overflow-hidden p-single text-sm md:text-sm">
                    <span className="relative inline-flex min-w-0 truncate">
                      <span className="truncate text-transparent">{draft}</span>
                      <span className="absolute inset-0 truncate font-semibold text-foreground">{inlineCompletion.prefix}</span>
                    </span>
                    <span data-slot="search-inline-suffix" className="truncate text-muted-foreground">
                      {inlineCompletion.suffix}
                    </span>
                  </div>
                ) : null}
              </div>
              {hasPossibles ? (
                <Action
                  id={UI_WINDOW_SEARCH.suggestions}
                  aria-expanded={possiblesExpanded}
                  aria-label={suggestionsAriaLabel}
                  data-slot="search-possibles-toggle"
                  icon={possiblesExpanded ? <ChevronDownIcon className="size-small" /> : inline === "rtl" ? <ChevronLeftIcon className="size-small" /> : <ChevronRightIcon className="size-small" />}
                  onClick={() => setPossiblesExpanded((open) => !open)}
                />
              ) : null}
            </div>
          </PopoverAnchor>
          {hasPossibles ? (
            <PopoverContent data-slot="search-autocomplete" className="w-[min(100vw-1rem,28rem)] p-0" align="end" onOpenAutoFocus={(event) => event.preventDefault()}>
              <Command shouldFilter={false}>
                <CommandList>
                  {filteredPossibles.length ? (
                    <CommandGroup>
                      {filteredPossibles.map((item, index) => (
                        <CommandItem
                          key={item.id}
                          value={item.id}
                          data-active={index === activePossibleIndex ? "true" : undefined}
                          className={cn(index === activePossibleIndex && interactiveActiveFillClass)}
                          onPointerDown={(event) => {
                            event.preventDefault();
                            event.stopPropagation();
                            selectPossible(item);
                          }}
                          onMouseEnter={() => setActivePossibleIndex(index)}
                          onSelect={() => selectPossible(item)}
                        >
                          <span className="truncate">{searchHighlightedLabel(item.label, draft, item.detail)}</span>
                          {item.detail ? <span className="ms-auto truncate text-xs text-muted-foreground">{item.detail}</span> : null}
                        </CommandItem>
                      ))}
                    </CommandGroup>
                  ) : (
                    <CommandEmpty>{noMatchesLabel}</CommandEmpty>
                  )}
                </CommandList>
              </Command>
            </PopoverContent>
          ) : null}
        </Popover>
      </div>
    </LevelProvider>
  );
};

export { Search };
