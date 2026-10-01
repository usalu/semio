// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🔎️ShellSearch/component.tsx
/** 🔎️ `🔎️ShellSearch` — the OS shell's two fuzzy-search command surfaces: `UISearch` (the global
 * command palette over an arbitrary `UISearchItem[]`) and `UIFind`/`UIFindProvider` (an in-document
 * find-in-content surface a window registers `UIFindItem[]` into via `useUIFind`). Both share one
 * owned fuzzy-ranked grouped-results layout on top of `CommandDialog`.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { createContext, useCallback, useContext, useMemo, useRef, useState, type ReactNode } from "react";
import { CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, rankFuzzyItems, useLabel, useShellScopeOptional, type FuzzySearchResult } from "@semio-tech/ui-react";
// #endregion 🔌️Adapters

//#region 🔖️ui-search-find

//#region UISearch
export type UISearchItem = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly icon?: ReactNode;
  readonly category?: string;
  readonly onSelect: () => void;
};

export function UISearch({
  items,
  open,
  onOpenChange,
  placeholder,
  emptyMessage,
}: {
  readonly items: readonly UISearchItem[];
  readonly open: boolean;
  readonly onOpenChange: (open: boolean) => void;
  readonly placeholder?: string;
  readonly emptyMessage?: string;
}) {
  const scope = useShellScopeOptional();
  const title = useLabel("ui.search.title");
  const description = useLabel("ui.search.description");
  const localizedPlaceholder = useLabel("ui.search.placeholder");
  const localizedEmpty = useLabel("ui.search.empty");
  const [query, setQuery] = useState("");
  const results = useMemo(
    () =>
      rankFuzzyItems(items, query, {
        fields: [
          { read: (item) => item.label, weight: 2 },
          { read: (item) => item.description, weight: 1 },
          { read: (item) => item.category, weight: 0.5 },
        ],
        threshold: 0.4,
        limit: 20,
      }),
    [items, query],
  );
  const grouped = useMemo(() => {
    const groups: Record<string, FuzzySearchResult<UISearchItem>[]> = {};
    for (const result of results) {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    }
    return groups;
  }, [results]);
  const handleSelect = useCallback(
    (item: UISearchItem) => {
      onOpenChange(false);
      setQuery("");
      item.onSelect();
    },
    [onOpenChange],
  );

  return (
    <CommandDialog title={title} description={description} isolationRoot={scope && !scope.ownsPage ? scope.rootRef.current : undefined} open={open} onOpenChange={onOpenChange} shouldFilter={false}>
      <CommandInput id={scope ? `${scope.shellId}.ui.search.input` : "ui.search.input"} placeholder={placeholder ?? localizedPlaceholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage ?? localizedEmpty}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem key={`${result.item.id}-${idx}`} data-command-item-id={result.item.id} value={`${result.item.label} ${result.item.description ?? ""} ${result.item.category ?? ""}`.trim()} onSelect={() => handleSelect(result.item)}>
                <div className="flex items-center gap-single">
                  {result.item.icon}
                  <div className="flex flex-col">
                    <span>{result.item.label}</span>
                    {result.item.description ? <span className="text-xs text-muted-foreground">{result.item.description}</span> : null}
                  </div>
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
    </CommandDialog>
  );
}
//#endregion UISearch

//#region UIFind
export type UIFindItem = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly category?: string;
};

export type UIFindContextValue = {
  readonly findItems: readonly UIFindItem[];
  readonly setFindItems: (items: readonly UIFindItem[]) => void;
  readonly setOnFindItem: (callback: ((itemId: string) => void) | undefined) => void;
  readonly triggerFindItem: (itemId: string) => void;
};

const UIFindContext = createContext<UIFindContextValue | null>(null);

function areFindItemsShallowEqual(previousItems: readonly UIFindItem[], nextItems: readonly UIFindItem[]): boolean {
  if (previousItems === nextItems) return true;
  if (previousItems.length !== nextItems.length) return false;
  for (let index = 0; index < nextItems.length; index += 1) {
    const previous = previousItems[index];
    const next = nextItems[index];
    if (!previous || !next || previous.id !== next.id || previous.label !== next.label || previous.description !== next.description || previous.category !== next.category) {
      return false;
    }
  }
  return true;
}

export function UIFindProvider({ children }: { readonly children: ReactNode }) {
  const [findItems, setFindItemsState] = useState<readonly UIFindItem[]>([]);
  const onFindItemCallbackRef = useRef<((itemId: string) => void) | undefined>(undefined);
  const setFindItems = useCallback((items: readonly UIFindItem[]) => {
    setFindItemsState((previousItems) => (areFindItemsShallowEqual(previousItems, items) ? previousItems : items));
  }, []);
  const setOnFindItem = useCallback((callback: ((itemId: string) => void) | undefined) => {
    onFindItemCallbackRef.current = callback;
  }, []);
  const triggerFindItem = useCallback((itemId: string) => {
    onFindItemCallbackRef.current?.(itemId);
  }, []);
  const contextValue = useMemo(() => ({ findItems, setFindItems, setOnFindItem, triggerFindItem }), [findItems, setFindItems, setOnFindItem, triggerFindItem]);
  return <UIFindContext.Provider value={contextValue}>{children}</UIFindContext.Provider>;
}

export function useUIFind(): UIFindContextValue {
  const context = useContext(UIFindContext);
  if (!context) throw new Error("useUIFind must be used within UIFindProvider");
  return context;
}

export function useUIFindSafe(): UIFindContextValue | null {
  return useContext(UIFindContext);
}

export function UIFind({
  open,
  onOpenChange,
  placeholder,
  emptyMessage,
}: {
  readonly open: boolean;
  readonly onOpenChange: (open: boolean) => void;
  readonly placeholder?: string;
  readonly emptyMessage?: string;
}) {
  const scope = useShellScopeOptional();
  const title = useLabel("ui.find.title");
  const description = useLabel("ui.find.description");
  const localizedPlaceholder = useLabel("ui.find.placeholder");
  const localizedEmpty = useLabel("ui.find.empty");
  const [query, setQuery] = useState("");
  const findContext = useContext(UIFindContext);
  const findItems = findContext?.findItems ?? [];
  const triggerFindItem = findContext?.triggerFindItem;
  const results = useMemo(
    () =>
      rankFuzzyItems(findItems, query, {
        fields: [
          { read: (item) => item.label, weight: 2 },
          { read: (item) => item.description, weight: 1 },
          { read: (item) => item.category, weight: 0.5 },
        ],
        threshold: 0.4,
        limit: 20,
      }),
    [findItems, query],
  );
  const grouped = useMemo(() => {
    const groups: Record<string, FuzzySearchResult<UIFindItem>[]> = {};
    for (const result of results) {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    }
    return groups;
  }, [results]);
  const handleSelect = useCallback(
    (item: UIFindItem) => {
      onOpenChange(false);
      setQuery("");
      triggerFindItem?.(item.id);
    },
    [onOpenChange, triggerFindItem],
  );

  if (!findContext) return null;

  return (
    <CommandDialog title={title} description={description} isolationRoot={scope && !scope.ownsPage ? scope.rootRef.current : undefined} open={open} onOpenChange={onOpenChange} shouldFilter={false}>
      <CommandInput id={scope ? `${scope.shellId}.ui.find.input` : "ui.find.input"} placeholder={placeholder ?? localizedPlaceholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage ?? localizedEmpty}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem key={`${result.item.id}-${idx}`} value={`${result.item.label} ${result.item.description ?? ""} ${result.item.category ?? ""}`.trim()} onSelect={() => handleSelect(result.item)}>
                <div className="flex flex-col">
                  <span>{result.item.label}</span>
                  {result.item.description ? <span className="text-xs text-muted-foreground">{result.item.description}</span> : null}
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
    </CommandDialog>
  );
}
//#endregion UIFind
//#endregion 🔖️ui-search-find
