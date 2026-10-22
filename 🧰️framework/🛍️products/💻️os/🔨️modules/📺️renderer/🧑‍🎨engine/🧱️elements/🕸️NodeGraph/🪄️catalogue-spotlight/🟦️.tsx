/** 🪄️ The flow catalogue: drag payloads, items, groups and sections, ranking and scoring of suggestions, and the double-click spotlight.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | NodeGraph}, which re-exports every public name below so import specifiers stay unchanged.
 */

import * as React from "../../⚛️react-port/🟦️.ts";
import { useEffect, useMemo, useRef, useState } from "../../⚛️react-port/🟦️.ts";
import {
  borderNormalBottomClass,
  cn,
  floatingMenuItemClass,
  floatingMenuSurfaceClass,
  Input,
  useLabel
} from "@semio-tech/ui-react";

//#region 🔖️NodeGraphHost
//#region Parsing
export interface CatalogueAppDragPayload {
  readonly pluginId: string;
  readonly appId: string;
  readonly label?: string;
}

/** 🎯️ Parses a catalogue drag payload; returns null for non-catalogue-app payloads (garbage/legacy descriptors). */
export function parseCatalogueAppDragPayload(raw: string): CatalogueAppDragPayload | null {
  try {
    const parsed = JSON.parse(raw) as { readonly pluginId?: string; readonly appId?: string; readonly label?: string };
    if (!parsed.pluginId || !parsed.appId) return null;
    return { pluginId: parsed.pluginId, appId: parsed.appId, label: parsed.label };
  } catch {
    return null;
  }
}

/** 👻️ Builds the ghost widget descriptor shown while a catalogue app is dragged over the workflow. */
export function catalogueGhostDescriptorJson(payload: CatalogueAppDragPayload): string {
  return JSON.stringify({ kind: "neuron", neuronKind: payload.label ?? payload.appId });
}

//#region FlowCatalogueSpotlight
export type FlowCatalogueItem = {
  readonly kind: string;
  readonly neuronKind?: string;
  readonly action?: string;
  readonly format?: string;
  readonly name: string;
  readonly abbreviation: string;
  readonly icon: string;
  readonly summary: string;
};

export type FlowCatalogueGroup = {
  readonly id: string;
  readonly title: string;
  readonly items?: readonly FlowCatalogueItem[];
  readonly groups?: readonly FlowCatalogueGroup[];
};

export type FlowCatalogueSection = {
  readonly id: string;
  readonly title: string;
  readonly items?: readonly FlowCatalogueItem[];
  readonly groups?: readonly FlowCatalogueGroup[];
};

/** 🧩️ Builds an addWidget/setGhostWidget descriptor JSON from a catalogue row. */
export function flowCatalogueItemDescriptor(item: FlowCatalogueItem): string {
  const descriptor: Record<string, string> = { kind: item.kind };
  if (item.kind === "inputSlider") descriptor.label = item.name;
  if (item.neuronKind) descriptor.neuronKind = item.neuronKind;
  if (item.action) descriptor.action = item.action;
  if (item.format) descriptor.format = item.format;
  return JSON.stringify(descriptor);
}

function flattenFlowCatalogueItems(sections: readonly FlowCatalogueSection[]): FlowCatalogueItem[] {
  const out: FlowCatalogueItem[] = [];
  const walkGroup = (group: FlowCatalogueGroup) => {
    for (const item of group.items ?? []) out.push(item);
    for (const child of group.groups ?? []) walkGroup(child);
  };
  for (const section of sections) {
    for (const item of section.items ?? []) out.push(item);
    for (const group of section.groups ?? []) walkGroup(group);
  }
  return out;
}

function scoreFlowCatalogueItem(item: FlowCatalogueItem, query: string, sectionTitle?: string): number | null {
  if (!query) return item.kind === "neuron" ? 1 : 2;
  const q = query.toLowerCase();
  const name = item.name.toLowerCase();
  const neuron = (item.neuronKind ?? "").toLowerCase();
  const abbr = item.abbreviation.toLowerCase();
  const summary = item.summary.toLowerCase();
  const section = (sectionTitle ?? "").toLowerCase();
  if (name === q || neuron === q || abbr === q) return 0;
  if (name.startsWith(q) || neuron.startsWith(q) || abbr.startsWith(q) || section.startsWith(q)) return 1;
  if (name.includes(q) || neuron.includes(q) || abbr.includes(q) || summary.includes(q) || section.includes(q)) return 2;
  return null;
}

/** 🔎️ Ranks catalogue items for the double-click spotlight (exact/prefix/substring; neurons first). */
export function flowRankCatalogueSuggestions(sections: readonly FlowCatalogueSection[], query: string): FlowCatalogueItem[] {
  const scored: { item: FlowCatalogueItem; score: number }[] = [];
  const walkGroup = (group: FlowCatalogueGroup, sectionTitle: string) => {
    for (const item of group.items ?? []) {
      const score = scoreFlowCatalogueItem(item, query.trim(), sectionTitle);
      if (score != null) scored.push({ item, score });
    }
    for (const child of group.groups ?? []) walkGroup(child, sectionTitle);
  };
  for (const section of sections) {
    for (const item of section.items ?? []) {
      const score = scoreFlowCatalogueItem(item, query.trim(), section.title);
      if (score != null) scored.push({ item, score });
    }
    for (const group of section.groups ?? []) walkGroup(group, section.title);
  }
  scored.sort((a, b) => {
      if (a.score !== b.score) return a.score - b.score;
      const aNeuron = a.item.kind === "neuron" ? 0 : 1;
      const bNeuron = b.item.kind === "neuron" ? 0 : 1;
      if (aNeuron !== bNeuron) return aNeuron - bNeuron;
      return a.item.name.localeCompare(b.item.name);
    });
  return scored.map((row) => row.item);
}

/** 🔍️ Scroll container classes for expanded flow spotlight suggestions. */
export function flowSpotlightSuggestionListScrollClass(expanded: boolean): string {
  return cn("min-h-0 overscroll-contain", expanded ? "overflow-y-auto max-h-[min(24rem,70vh)]" : "overflow-hidden");
}

export function parseFlowCatalogueSections(json: string | undefined | null): FlowCatalogueSection[] {
  if (!json) return [];
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? (parsed as FlowCatalogueSection[]) : [];
  } catch {
    return [];
  }
}

export type FlowSpotlightState = {
  readonly screen: { readonly x: number; readonly y: number };
  readonly world: { readonly x: number; readonly y: number };
};

/** 🔦️ Inline catalogue search opened by double-clicking empty flow canvas; hover/top match drives highlighted ghost preview. */
export function FlowSpotlight({
  state,
  sections,
  onPreview,
  onCommit,
  onClose,
}: {
  readonly state: FlowSpotlightState;
  readonly sections: readonly FlowCatalogueSection[];
  readonly onPreview: (item: FlowCatalogueItem | null) => void;
  readonly onCommit: (item: FlowCatalogueItem) => void;
  readonly onClose: () => void;
}) {
  const inputId = React.useId();
  const rootRef = useRef<HTMLDivElement>(null);
  const activeItemRef = useRef<HTMLButtonElement | null>(null);
  const [query, setQuery] = useState("");
  const [expanded, setExpanded] = useState(false);
  const [activeIndex, setActiveIndex] = useState(0);
  const [previewArmed, setPreviewArmed] = useState(false);
  const suggestions = useMemo(() => flowRankCatalogueSuggestions(sections, query), [query, sections]);
  const visible = expanded ? suggestions : suggestions.slice(0, 1);
  const hasMore = suggestions.length > 1;
  const activeItem = suggestions[activeIndex] ?? null;
  const shouldPreview = query.trim().length > 0 || previewArmed;
  const typeToAddLabel = useLabel("ui.flowSpotlight.typeToAdd");
  const collapseSuggestionsLabel = useLabel("ui.flowSpotlight.collapseSuggestions");
  const showAllSuggestionsLabel = useLabel("ui.flowSpotlight.showAllSuggestions");
  const noMatchesLabel = useLabel("ui.windowSearch.noMatches");

  useEffect(() => {
    setActiveIndex(0);
    setExpanded(false);
  }, [query]);

  useEffect(() => {
    onPreview(shouldPreview ? activeItem : null);
  }, [activeItem, onPreview, shouldPreview]);

  useEffect(() => {
    if (!expanded) return;
    activeItemRef.current?.scrollIntoView({ block: "nearest" });
  }, [activeIndex, expanded]);

  useEffect(() => {
    const onPointerDown = (event: PointerEvent) => {
      const root = rootRef.current;
      if (root?.contains(event.target as globalThis.Node)) return;
      onClose();
    };
    window.addEventListener("pointerdown", onPointerDown, true);
    return () => window.removeEventListener("pointerdown", onPointerDown, true);
  }, [onClose]);

  return (
    <div
      ref={rootRef}
      className={cn("pointer-events-auto absolute z-60 flex min-h-0 w-layout-floating-menu-sm flex-col overflow-hidden", floatingMenuSurfaceClass)}
      data-level="menu"
      style={{ left: state.screen.x, top: state.screen.y }}
      onPointerDown={(event) => event.stopPropagation()}
      onDoubleClick={(event) => event.stopPropagation()}
      onWheel={(event) => event.stopPropagation()}
    >
      <div className={cn("flex shrink-0 items-center gap-single px-single py-half", borderNormalBottomClass)}>
        <Input
          id={`flow-spotlight-${inputId}`}
          aria-label={typeToAddLabel}
          autoFocus
          value={query}
          placeholder={typeToAddLabel}
          className="min-w-0 flex-1 border-0 bg-transparent p-0 text-xs shadow-none focus-visible:ring-0"
          onChange={(event) => setQuery(event.target.value)}
          onKeyDown={(event) => {
            if (event.key === "Escape") {
              event.preventDefault();
              onClose();
              return;
            }
            if (event.key === "ArrowDown") {
              event.preventDefault();
              if (suggestions.length === 0) return;
              setPreviewArmed(true);
              setActiveIndex((index) => Math.min(index + 1, suggestions.length - 1));
              if (!expanded && hasMore) setExpanded(true);
              return;
            }
            if (event.key === "ArrowUp") {
              event.preventDefault();
              if (suggestions.length === 0) return;
              setPreviewArmed(true);
              setActiveIndex((index) => Math.max(index - 1, 0));
              return;
            }
            if (event.key === "Enter") {
              event.preventDefault();
              if (activeItem) onCommit(activeItem);
            }
          }}
        />
        {hasMore ? (
          <button
            type="button"
            aria-label={expanded ? collapseSuggestionsLabel : showAllSuggestionsLabel}
            className="text-muted-foreground hover:bg-muted/40 hover:text-foreground shrink-0 rounded px-half text-2xs"
            onClick={() => setExpanded((value) => !value)}
          >
            {expanded ? "▴️" : "▾️"}
          </button>
        ) : null}
      </div>
      <div
        className={flowSpotlightSuggestionListScrollClass(expanded && hasMore)}
        role="listbox"
        onWheel={(event) => event.stopPropagation()}
      >
        {visible.length === 0 ? (
          <div className="text-muted-foreground px-single py-half text-2xs">{noMatchesLabel}</div>
        ) : (
          visible.map((item, index) => {
            const globalIndex = expanded ? index : 0;
            const active = globalIndex === activeIndex && shouldPreview;
            const key = `${item.kind}:${item.neuronKind ?? item.action ?? item.format ?? item.name}`;
            return (
              <button
                key={key}
                ref={active ? activeItemRef : undefined}
                type="button"
                role="option"
                aria-selected={active}
                className={cn(floatingMenuItemClass, active && "bg-active-base text-emphasized")}
                onPointerEnter={() => {
                  setPreviewArmed(true);
                  setActiveIndex(globalIndex);
                }}
                onPointerDown={(event) => {
                  event.preventDefault();
                  event.stopPropagation();
                  onCommit(item);
                }}
              >
                <span className="truncate">{item.name}</span>
                {item.neuronKind ? <span className="text-muted-foreground truncate text-2xs">{item.neuronKind}</span> : null}
              </button>
            );
          })
        )}
      </div>
    </div>
  );
}
//#endregion FlowCatalogueSpotlight
//#endregion Parsing
//#endregion 🔖️NodeGraphHost
