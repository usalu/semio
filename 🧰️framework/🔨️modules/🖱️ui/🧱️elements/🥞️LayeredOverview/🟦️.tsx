// #region 🧲️Header
// 💻️ framework/ui/elements/🥞️LayeredOverview/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { loadingBorderClass } from "../../🔨️modules/🌀️status-border-presentation/🟦️.ts";
import {
  LAYERED_WHEEL_QUIET_MS,
  LAYERED_WHEEL_STEP_PX,
  cellOffset,
  clampOffset,
  flingStep,
  followStep,
  glideOffset,
  inWindow,
  nearestCell,
  neighbourWarmBoot,
  nextWarmBoot,
  panesOverBudget,
  panesToRelease,
  pointerOffset,
  resolveLifecycle,
  restingCell,
  scheduleIdle,
  settleOffset,
  stripGrid,
  stripTransform,
  swipeAxis,
  swipeDragOffset,
  swipeNeighbours,
  swipeStep,
  swipeTarget,
  swipeWrapTarget,
  veilClip,
  veilClipPath,
  warmDelay,
  windowAround,
  type LayeredAxis,
  type LayeredCell,
  type LayeredDirection,
  type LayeredGrid,
  type LayeredLanding,
  type LayeredLifecycle,
  type LayeredOffset,
  type LayeredVeil,
  type LayeredWindow,
} from "../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import { Icon, type IconName } from "../🔣️Icons/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🧬️LayeredOverviewContracts
/** 🧭️ How the one strip under the one glass is read. `strip`: the app's card grid stays put above it and the mouse pans the pages behind
 * (desktop, tablet); `swipe`: the view is one cell of the same grid, each card rides on its own page, and a swipe along either axis — or the
 * wheel — carries the strip to the neighbouring cell, across the edges too: the grid wraps like an endless canvas (touch phones, windows too
 * small for the card grid). */
export type LayeredMode = "strip" | "swipe";

/** 🥞️ One backdrop page, the real page its card opens: `id` is its hash route, `data-layered-pane` and key, `label` names the opened page and its
 * placeholder, `render` is called only while the pane is mounted, `poster` is shown while it is not, and `capturePoster` (for example
 * {@link capturePosterFromCanvases}) turns the mounted page into a poster right before it is released. */
export interface LayeredPane {
  readonly id: string;
  readonly label: string;
  /** 📱️ What swipe hints and other tight chrome show instead of {@link label} when space is scarce; defaults to `label`. */
  readonly shortLabel?: string;
  readonly icon?: IconName;
  readonly render: (state: LayeredPaneState) => React.ReactNode;
  readonly poster?: string | null;
  readonly capturePoster?: (container: HTMLElement) => string | null;
}

/** 🥞️ What a mounted page knows: `opened` (full size and interactive, otherwise inert backdrop), `revealed` (its card is hovered or focused, so it
 * shows clear) and `dirty` (the page reports user work, so it is never released). */
export interface LayeredPaneState {
  readonly opened: boolean;
  readonly revealed: boolean;
  readonly dirty: () => void;
}

/** 🃏️ What a card knows: the mode, whether its page is revealed (lift the card) or opened, and `open` (hash route plus glide). */
export interface LayeredCardState {
  readonly mode: LayeredMode;
  readonly revealed: boolean;
  readonly opened: boolean;
  readonly open: () => void;
}

/** 🧢️ What the app's chrome above the element knows, so it can hide while a page is revealed or opened. */
export interface LayeredChromeState {
  readonly revealed: boolean;
  readonly opened: boolean;
}

/** 🌐️ The element's own words, always the app's (no default language): the card group, the Overview button, a waiting and a failed page, and
 * the hint of a page one swipe away in `direction` (its accessible name, such as "Go right to Physics"; the hint shows the page's label). An
 * app whose own chrome leads back to the overview names no `overview`, and the element shows no button of its own over the opened page. */
export interface LayeredLabels {
  readonly grid: string;
  readonly overview?: string;
  readonly waiting: (pane: LayeredPane) => string;
  readonly failed: (pane: LayeredPane) => string;
  readonly neighbour: (pane: LayeredPane, direction: LayeredDirection) => string;
}

export type { LayeredDirection };

/** 📏️ How far the app's own chrome covers the `top` and the `bottom` of the overview, as CSS lengths (`0px` where not given). */
export interface LayeredInsets {
  readonly top?: string;
  readonly bottom?: string;
}

/** 🥞️ Props of {@link LayeredOverview}. `cells` places every pane on the strip for the current breakpoint; in `strip` mode the card grid is the
 * app's CSS (`overlayClassName`/`overlayStyle`, each card wrapped in a `display: contents` host), in `swipe` mode the element centres each card
 * in its page's cell itself, inside `insets`, and the hints of its neighbours stand inside them too; `pan` lets the mouse pan the strip under the glass while it is between the cards; `reducedMotion` says whether
 * pan, glide and settle give way to stillness — `"never"` (the default), `"auto"` (when the device asks for it) or `"always"`; `openedId`
 * makes the open page controlled. */
export interface LayeredOverviewProps {
  readonly panes: readonly LayeredPane[];
  readonly cells: Readonly<Record<string, LayeredCell>>;
  readonly renderCard: (pane: LayeredPane, state: LayeredCardState) => React.ReactNode;
  readonly overlayClassName?: string;
  readonly overlayStyle?: React.CSSProperties;
  readonly renderChrome?: (state: LayeredChromeState) => React.ReactNode;
  readonly mode?: LayeredMode;
  readonly insets?: LayeredInsets;
  readonly pan?: "pointer" | "none";
  readonly routing?: "hash" | "none";
  readonly openedId?: string | null;
  readonly onOpenedIdChange?: (id: string | null) => void;
  readonly onRevealedIdChange?: (id: string | null) => void;
  readonly lifecycle?: Partial<LayeredLifecycle>;
  readonly windowing?: { readonly radius: number };
  readonly reducedMotion?: "auto" | "always" | "never";
  readonly labels: LayeredLabels;
}
// #endregion 🧬️LayeredOverviewContracts

// #region 🖼️LayeredPoster
/** 🖼️ Composites every `<canvas>` inside `container` into one PNG data URL, synchronously (before a `preserveDrawingBuffer: false` back buffer is
 * cleared); `null` when the container holds no drawable canvas, so a DOM-only page never gets a poster. */
export function capturePosterFromCanvases(container: HTMLElement): string | null {
  const canvases = [...container.querySelectorAll("canvas")].filter((canvas) => canvas.width > 0 && canvas.height > 0);
  const box = container.getBoundingClientRect();
  if (canvases.length === 0 || box.width <= 0 || box.height <= 0) return null;
  const poster = document.createElement("canvas");
  poster.width = Math.round(box.width);
  poster.height = Math.round(box.height);
  const context = poster.getContext("2d");
  if (!context) return null;
  const drawn = canvases.filter((canvas) => {
    const rect = canvas.getBoundingClientRect();
    try {
      context.drawImage(canvas, rect.left - box.left, rect.top - box.top, rect.width, rect.height);
      return true;
    } catch {
      return false;
    }
  });
  if (drawn.length === 0) return null;
  try {
    return poster.toDataURL("image/png");
  } catch {
    return null;
  }
}
// #endregion 🖼️LayeredPoster

// #region 🧮️LayeredPaneLifecycle
const REDUCED_MOTION_QUERY = "(prefers-reduced-motion: reduce)";
const FOCUSABLE = 'a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

type LayeredDrive = { readonly kind: "follow" } | { readonly kind: "glide" | "settle"; readonly from: LayeredOffset; readonly to: LayeredOffset; readonly startedAt: number };
const FOLLOW: LayeredDrive = { kind: "follow" };

/** 🐢️ Whether the user asks for reduced motion, unless the app forces it on or off. */
function useReducedMotion(setting: "auto" | "always" | "never"): boolean {
  const [prefers, setPrefers] = React.useState(() => typeof window !== "undefined" && typeof window.matchMedia === "function" && window.matchMedia(REDUCED_MOTION_QUERY).matches);
  React.useEffect(() => {
    if (setting !== "auto" || typeof window.matchMedia !== "function") return;
    const media = window.matchMedia(REDUCED_MOTION_QUERY);
    const sync = () => setPrefers(media.matches);
    sync();
    media.addEventListener("change", sync);
    return () => media.removeEventListener("change", sync);
  }, [setting]);
  return setting === "always" || (setting === "auto" && prefers);
}

/** 📍️ The strip cell of pane `id`; every pane must have one. */
function requireCell(cells: Readonly<Record<string, LayeredCell>>, id: string): LayeredCell {
  const cell = cells[id];
  if (!cell) throw new Error(`LayeredOverview: pane "${id}" has no cell`);
  return cell;
}

/** 🔗️ The pane a location hash names, if any. */
function hashPaneId(ids: readonly string[]): string | null {
  const raw = window.location.hash.replace(/^#/, "");
  return ids.includes(raw) ? raw : null;
}

/** 🔗️ Mirrors the opened pane into the hash with `replaceState` (no history entries); a hash naming no pane, such as a skip link, is left alone. */
function syncHash(opened: string | null, ids: readonly string[]): void {
  const current = window.location.hash.replace(/^#/, "");
  if (opened === null ? !ids.includes(current) : current === opened) return;
  window.history.replaceState(window.history.state, "", opened === null ? window.location.pathname + window.location.search : `#${opened}`);
}

/** ⌨️ Whether focus landed by keyboard (`:focus-visible`); a browser that cannot answer counts as keyboard. */
function focusVisible(target: EventTarget): boolean {
  if (!(target instanceof Element)) return true;
  try {
    return target.matches(":focus-visible");
  } catch {
    return true;
  }
}

/** 🕳️ Writes a veil onto the glass element. */
function applyVeil(element: HTMLElement, veil: LayeredVeil): void {
  element.style.clipPath = veilClipPath(veil);
  element.style.visibility = veil.kind === "clear" ? "hidden" : "visible";
  element.dataset.veil = veil.kind;
}

/** 🪟️ Whether two windows cover the same cells. */
function sameWindow(a: LayeredWindow, b: LayeredWindow): boolean {
  return a.firstColumn === b.firstColumn && a.lastColumn === b.lastColumn && a.firstRow === b.firstRow && a.lastRow === b.lastRow;
}

/** 🧷️ The first value seen for a key: equal ids and cells keep one identity across renders, so an app re-rendering with fresh arrays never
 * restarts a glide or a warm-boot timer. */
function useStableByKey<T>(value: T, key: string): T {
  const kept = React.useRef({ key, value });
  if (kept.current.key !== key) kept.current = { key, value };
  return kept.current.value;
}

/** 🧮️ Mounts, releases and posters of the panes: boots on reveal/open/swipe, a warm queue in cell order (while `warming`: the strip mode, never
 * the swipe mode, which warms the neighbours of its cell instead; a pace of zero boots at once instead of waiting for an idle moment), a live `budget` (least recently touched pristine pane first), time-based release, posters from `capturePoster`. Dirty, opened and
 * revealed panes are never released; the most recently opened pane (initially the first) is never released by time while the overview shows. */
function usePaneLifecycle(panes: readonly LayeredPane[], ids: readonly string[], opened: string | null, revealed: string | null, lifecycle: LayeredLifecycle, warming: boolean) {
  const initial = opened ? [opened] : [];
  const [booted, setBooted] = React.useState<ReadonlySet<string>>(() => new Set(initial));
  const [suspended, setSuspended] = React.useState<ReadonlySet<string>>(() => new Set());
  const [dirty, setDirty] = React.useState<ReadonlySet<string>>(() => new Set());
  const [posters, setPosters] = React.useState<ReadonlyMap<string, string>>(() => new Map());
  const live = React.useMemo<ReadonlySet<string>>(() => new Set([...booted].filter((id) => !suspended.has(id))), [booted, suspended]);
  const mountedAt = React.useRef(new Map<string, number>(initial.map((id) => [id, Date.now()])));
  const touchedAt = React.useRef(new Map<string, number>(opened ? [[opened, Date.now()]] : []));
  const containers = React.useRef(new Map<string, HTMLElement>());
  const lastOpened = React.useRef<string | null>(opened ?? panes[0]?.id ?? null);
  const previousOpened = React.useRef(opened);
  const warmed = React.useRef(false);
  const state = React.useRef({ panes, live, dirty, opened, revealed, lifecycle });
  state.current = { panes, live, dirty, opened, revealed, lifecycle };

  const boot = React.useCallback((id: string) => {
    if (!state.current.live.has(id)) mountedAt.current.set(id, Date.now());
    setBooted((previous) => (previous.has(id) ? previous : new Set(previous).add(id)));
    setSuspended((previous) => (previous.has(id) ? new Set([...previous].filter((other) => other !== id)) : previous));
  }, []);
  const touch = React.useCallback(
    (id: string) => {
      touchedAt.current.set(id, Date.now());
      boot(id);
    },
    [boot],
  );
  const warm = React.useCallback(
    (id: string) => {
      warmed.current = true;
      boot(id);
    },
    [boot],
  );
  const markDirty = React.useCallback((id: string) => setDirty((previous) => (previous.has(id) ? previous : new Set(previous).add(id))), []);
  const registerContainer = React.useCallback((id: string, element: HTMLElement | null) => {
    if (element) containers.current.set(id, element);
    else containers.current.delete(id);
  }, []);
  const container = React.useCallback((id: string) => containers.current.get(id) ?? null, []);
  const keep = React.useCallback((byTime: boolean): ReadonlySet<string> => {
    const { dirty: kept, opened: open, revealed: shown } = state.current;
    return new Set([...kept, ...[open, shown, byTime && open === null ? lastOpened.current : null].filter((id): id is string => id !== null)]);
  }, []);
  const release = React.useCallback((released: readonly string[]) => {
    if (released.length === 0) return;
    const captured = new Map<string, string>();
    for (const id of released) {
      const element = containers.current.get(id);
      const poster = element ? state.current.panes.find((pane) => pane.id === id)?.capturePoster?.(element) : null;
      if (poster) captured.set(id, poster);
    }
    if (captured.size > 0) setPosters((previous) => new Map([...previous, ...captured]));
    setSuspended((previous) => new Set([...previous, ...released]));
  }, []);
  const byRecency = React.useCallback(
    (set: ReadonlySet<string>) => [...set].sort((a, b) => (touchedAt.current.get(a) ?? -Infinity) - (touchedAt.current.get(b) ?? -Infinity) || (mountedAt.current.get(a) ?? 0) - (mountedAt.current.get(b) ?? 0)),
    [],
  );

  React.useEffect(() => {
    const previous = previousOpened.current;
    previousOpened.current = opened;
    if (previous !== null && previous !== opened) touchedAt.current.set(previous, Date.now());
    if (opened === null) return;
    lastOpened.current = opened;
    touch(opened);
  }, [opened, touch]);

  React.useEffect(() => {
    release(panesOverBudget(byRecency(live), keep(false), lifecycle.budget));
  }, [live, dirty, opened, revealed, lifecycle.budget, byRecency, keep, release]);

  React.useEffect(() => {
    if (!warming) return;
    const step = nextWarmBoot(ids, booted, live.size, opened, lifecycle.budget);
    if (step.kind !== "boot") return;
    const delay = warmDelay(warmed.current, lifecycle);
    if (delay <= 0) return warm(step.id);
    return scheduleIdle(() => warm(step.id), delay, window);
  }, [warming, ids, booted, live.size, opened, lifecycle.budget, lifecycle.warmStartMs, lifecycle.warmIntervalMs, warm]);

  const timed = [lifecycle.suspendIdleMs, lifecycle.suspendOffscreenMs, lifecycle.suspendHiddenMs].some(Number.isFinite);
  React.useEffect(() => {
    if (!timed) return;
    const timer = window.setInterval(() => {
      const { live: mounted, opened: open, lifecycle: policy } = state.current;
      const since = (id: string) => Math.max(touchedAt.current.get(id) ?? 0, mountedAt.current.get(id) ?? 0);
      release(panesToRelease([...mounted].map((id) => ({ id, since: since(id) })), keep(true), Date.now(), { opened: open !== null, hidden: document.hidden }, policy));
    }, lifecycle.sweepMs);
    return () => window.clearInterval(timer);
  }, [timed, lifecycle.sweepMs, keep, release]);

  return { booted, live, posters, touch, warm, markDirty, registerContainer, container };
}
// #endregion 🧮️LayeredPaneLifecycle

// #region 🥞️LayeredPaneView
/** 🛟️ One crashing page never takes down the others or the landing around them. */
class LayeredPaneBoundary extends React.Component<{ readonly failed: string; readonly children: React.ReactNode }, { readonly crashed: boolean }> {
  override state = { crashed: false };
  static getDerivedStateFromError(): { readonly crashed: boolean } {
    return { crashed: true };
  }
  override render(): React.ReactNode {
    return this.state.crashed ? (
      <div role="alert" data-layered-pane-error="" className="flex h-full w-full items-center justify-center bg-background p-double text-center text-sm text-muted-foreground">
        {this.props.failed}
      </div>
    ) : (
      this.props.children
    );
  }
}

/** 📖️ Renders a page as a child of its boundary, so a throwing page is caught there. */
function LayeredPage({ render, state }: { readonly render: LayeredPane["render"]; readonly state: LayeredPaneState }): React.ReactNode {
  return render(state);
}

/** ⏳️ What a pane shows before it first mounts: its glyph at 40 % and a pulsing page behind a loading border. */
function LayeredPanePlaceholder({ icon, busy, label }: { readonly icon?: IconName; readonly busy: boolean; readonly label: string }): React.ReactElement {
  return (
    <div role="status" aria-busy={busy} aria-label={label} className={cn("flex h-full w-full flex-col items-center justify-center gap-double bg-background p-double", loadingBorderClass)}>
      {icon ? <Icon icon={icon} size="large" className="text-foreground opacity-40" /> : null}
      <div aria-hidden className="min-h-0 w-full max-w-4xl flex-1 animate-pulse bg-muted-foreground/10 motion-reduce:animate-none" />
    </div>
  );
}

interface LayeredPaneViewProps {
  readonly pane: LayeredPane;
  readonly style?: React.CSSProperties;
  readonly opened: boolean;
  readonly revealed: boolean;
  readonly live: boolean;
  readonly shown: boolean;
  readonly busy: boolean;
  readonly poster: string | null;
  readonly waiting: string;
  readonly failed: string;
  readonly onContainer: (id: string, element: HTMLElement | null) => void;
  readonly onDirty: (id: string) => void;
}

/** 🥞️ One pane: its live page, else its poster, else its placeholder (the last two only within the window). Inert and hidden from assistive
 * technology unless opened; opened, it is a labelled, focusable region. Pointer or key input inside marks it dirty. */
const LayeredPaneView = React.memo(function LayeredPaneView(props: LayeredPaneViewProps): React.ReactElement {
  const { pane, opened, revealed, onContainer, onDirty } = props;
  const dirty = React.useCallback(() => onDirty(pane.id), [onDirty, pane.id]);
  const ref = React.useCallback((element: HTMLDivElement | null) => onContainer(pane.id, element), [onContainer, pane.id]);
  const state = React.useMemo<LayeredPaneState>(() => ({ opened, revealed, dirty }), [opened, revealed, dirty]);
  return (
    <div
      ref={ref}
      data-layered-pane={pane.id}
      data-opened={opened ? "" : undefined}
      role={opened ? "region" : undefined}
      aria-label={opened ? pane.label : undefined}
      aria-hidden={opened ? undefined : true}
      tabIndex={opened ? -1 : undefined}
      inert={!opened}
      className="relative h-full w-full overflow-hidden bg-background outline-none"
      style={{ contain: "layout paint", ...props.style }}
      onPointerDownCapture={dirty}
      onKeyDownCapture={dirty}
    >
      {props.live ? (
        <LayeredPaneBoundary failed={props.failed}>
          <LayeredPage render={pane.render} state={state} />
        </LayeredPaneBoundary>
      ) : !props.shown ? null : props.poster ? (
        <img src={props.poster} alt="" aria-hidden className="h-full w-full object-cover" />
      ) : (
        <LayeredPanePlaceholder icon={pane.icon} busy={props.busy} label={props.waiting} />
      )}
    </div>
  );
});

/** 🃏️ The `display: contents` host of one card: pointer enter (mouse) or keyboard focus reveals its page, pointer leave or focus leaving the card
 * conceals it; the card itself is the app's. Memoized, so a reveal re-renders only the two cards whose state changed. */
const LayeredCardHost = React.memo(function LayeredCardHost(props: {
  readonly pane: LayeredPane;
  readonly mode: LayeredMode;
  readonly revealed: boolean;
  readonly onOpen: (id: string) => void;
  readonly onReveal: (id: string) => void;
  readonly onConceal: (id: string) => void;
  readonly onElement: (id: string, element: HTMLElement | null) => void;
  readonly renderCard: LayeredOverviewProps["renderCard"];
}): React.ReactElement {
  const { pane, onOpen, onElement, onReveal, onConceal } = props;
  const open = React.useCallback(() => onOpen(pane.id), [onOpen, pane.id]);
  const ref = React.useCallback((element: HTMLDivElement | null) => onElement(pane.id, element), [onElement, pane.id]);
  return (
    <div
      ref={ref}
      data-layered-card={pane.id}
      data-revealed={props.revealed ? "" : undefined}
      className="contents"
      onPointerEnter={(event) => event.pointerType === "mouse" && onReveal(pane.id)}
      onPointerLeave={(event) => event.pointerType === "mouse" && onConceal(pane.id)}
      onFocus={(event) => focusVisible(event.target) && onReveal(pane.id)}
      onBlur={(event) => !event.currentTarget.contains(event.relatedTarget as Node | null) && onConceal(pane.id)}
    >
      {props.renderCard(pane, { mode: props.mode, revealed: props.revealed, opened: false, open })}
    </div>
  );
});
// #endregion 🥞️LayeredPaneView

// #region 🧭️LayeredNeighbourHints
const HINT_ICONS: { readonly [D in LayeredDirection]: IconName } = { up: "arrow-up", left: "arrow-left", right: "arrow-right", down: "arrow-down" };
const HINT_PLACES: { readonly [D in LayeredDirection]: string } = {
  up: "left-1/2 -translate-x-1/2 max-w-[calc(100%-6rem)] min-h-[28px] flex-row px-single",
  down: "left-1/2 -translate-x-1/2 max-w-[calc(100%-6rem)] min-h-[28px] flex-row px-single",
  left: "top-1/2 -translate-y-1/2 max-h-[45%] w-[28px] flex-col py-single",
  right: "top-1/2 -translate-y-1/2 max-h-[45%] w-[28px] flex-col py-single",
};
const HINT_TEXT: { readonly [D in LayeredDirection]: string } = {
  up: "min-w-0 truncate",
  down: "min-w-0 truncate",
  left: "min-h-0 overflow-hidden text-ellipsis whitespace-nowrap [writing-mode:vertical-rl] rotate-180",
  right: "min-h-0 overflow-hidden text-ellipsis whitespace-nowrap [writing-mode:vertical-rl]",
};

/** 📏️ Where the hint of `direction` stands: inside the app's chrome at the top and the bottom, in the room the cells keep free at the sides. */
function hintPlace(direction: LayeredDirection, insets: Required<LayeredInsets>): React.CSSProperties {
  switch (direction) {
    case "up":
      return { top: `calc(${insets.top} + var(--spacing-single))` };
    case "down":
      return { bottom: `calc(${insets.bottom} + var(--spacing-single))` };
    case "left":
      return { left: "4px" };
    case "right":
      return { right: "4px" };
  }
}

/** 🧭️ One page one swipe away from the resting page and the direction it lies in. */
interface LayeredNeighbour extends LayeredLanding {
  readonly direction: LayeredDirection;
  readonly pane: LayeredPane;
}

/** 🧭️ The hints of the swiped overview: at each edge of the view behind which a page lies, that page's label with an arrow — the page a swipe
 * the other way brings in — at the top and bottom edges across, at the sides along them. Each hint is a button that goes there. While the strip
 * moves the hints fade out (and leave the focus order), so they always name the neighbours of the page at rest.
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pointer-gestures.html — every swipe also has a single-pointer way */
const LayeredNeighbourHints = React.memo(function LayeredNeighbourHints(props: {
  readonly neighbours: readonly LayeredNeighbour[];
  readonly moving: boolean;
  readonly insets: Required<LayeredInsets>;
  readonly label: LayeredLabels["neighbour"];
  readonly onGo: (neighbour: LayeredNeighbour) => void;
}): React.ReactElement {
  return (
    <>
      {props.neighbours.map((neighbour) => (
        <button
          key={neighbour.direction}
          type="button"
          data-layered-neighbour={neighbour.direction}
          data-pane={neighbour.pane.id}
          data-level="dialog"
          aria-label={props.label(neighbour.pane, neighbour.direction)}
          aria-hidden={props.moving || undefined}
          tabIndex={props.moving ? -1 : undefined}
          onClick={() => props.onGo(neighbour)}
          className={cn(
            "ui-glass absolute z-[32] inline-flex items-center justify-center gap-[4px] border border-border-normal text-xs font-medium text-foreground outline-none transition-opacity duration-150 hover:border-border-emphasized focus-visible:ring-2 focus-visible:ring-ring",
            HINT_PLACES[neighbour.direction],
            props.moving && "pointer-events-none opacity-0",
          )}
          style={hintPlace(neighbour.direction, props.insets)}
        >
          <Icon icon={HINT_ICONS[neighbour.direction]} size="small" className="shrink-0" />
          <span className={HINT_TEXT[neighbour.direction]}>{neighbour.pane.shortLabel ?? neighbour.pane.label}</span>
        </button>
      ))}
    </>
  );
});
// #endregion 🧭️LayeredNeighbourHints

// #region 👆️LayeredSwipe
/** 👆️ What the swipe gestures move: the strip's offset now, the occupied cells, a drag that follows the finger, a settle onto a cell, a stop. */
interface LayeredSwipeDrive {
  readonly offset: () => LayeredOffset;
  readonly cells: () => readonly LayeredCell[];
  readonly drag: (offset: LayeredOffset) => void;
  readonly aim: (from: LayeredCell, axis: LayeredAxis, step: number) => LayeredLanding | null;
  readonly settle: (to: LayeredOffset) => void;
  readonly finish: () => void;
  readonly stop: () => void;
}

/** 🍩️ The page that comes in across an edge of the wrapping strip: shifted from its `cell` to the `slot` beside the view until the strip rests. */
interface LayeredGhost extends LayeredLanding {
  readonly id: string;
}

/** 👆️ One touch on the overview: where it went down, the cell the strip left, the axis it locked to and, when it scrolls a tall card instead,
 * that card's scroller and its position at the lock. */
interface LayeredTouch {
  readonly id: number;
  readonly x: number;
  readonly y: number;
  readonly target: EventTarget | null;
  readonly samples: { t: number; x: number; y: number }[];
  start: LayeredOffset;
  from: LayeredCell;
  axis: LayeredAxis | null;
  scroller: HTMLElement | null;
  scrolled: number;
  aimed: number;
}

const VELOCITY_WINDOW_MS = 100;
const CLICK_AFTER_SWIPE_MS = 400;
const along = (axis: LayeredAxis, x: number, y: number): number => (axis === "x" ? x : y);
const scrollPosition = (element: HTMLElement, axis: LayeredAxis): number => (axis === "x" ? element.scrollLeft : element.scrollTop);
const scrollAlong = (element: HTMLElement, axis: LayeredAxis, value: number): void => {
  if (axis === "x") element.scrollLeft = value;
  else element.scrollTop = value;
};

/** 📜️ The nearest element from `target` up to (not including) `root` that scrolls along `axis` and still can toward `direction` (+1 forward,
 * −1 back): a tall card takes the gesture before the strip does, as a nested scroller does natively. */
function scrollerToward(target: EventTarget | null, root: HTMLElement, axis: LayeredAxis, direction: number): HTMLElement | null {
  for (let element = target instanceof Element ? target : null; element !== null && element !== root; element = element.parentElement) {
    if (!(element instanceof HTMLElement)) continue;
    const style = getComputedStyle(element);
    if (!/auto|scroll/u.test(axis === "x" ? style.overflowX : style.overflowY)) continue;
    const [position, size, view] = axis === "x" ? [element.scrollLeft, element.scrollWidth, element.clientWidth] : [element.scrollTop, element.scrollHeight, element.clientHeight];
    if (direction > 0 ? position + view < size - 1 : position > 0) return element;
  }
  return null;
}

/** 🧲️ Captures `pointerId` on `element` where the browser can: an environment without pointer capture is left alone, and a pointer that
 * is no longer active cannot be captured. */
function capturePointer(element: HTMLElement, pointerId: number): void {
  if (element.hasPointerCapture?.(pointerId) !== false) return;
  try {
    element.setPointerCapture(pointerId);
  } catch (error) {
    if (!(error instanceof DOMException)) throw error;
  }
}

/** 🏎️ A touch's speed along `axis` over its last {@link VELOCITY_WINDOW_MS} (px/ms). */
function touchVelocity(touch: LayeredTouch, axis: LayeredAxis): number {
  const last = touch.samples[touch.samples.length - 1]!;
  const first = touch.samples.find((sample) => last.t - sample.t <= VELOCITY_WINDOW_MS) ?? last;
  const elapsed = last.t - first.t;
  return elapsed > 0 ? (along(axis, last.x, last.y) - along(axis, first.x, first.y)) / elapsed : 0;
}

/** 👆️ Swiping the strip while `enabled`: a touch or pen that travels past the slop locks to one axis and carries the strip along it toward the
 * page that comes in on that side — across an edge the page from the far side, shifted beside the view ({@link swipeWrapTarget}); a rubber band
 * only where the row or column holds no other page — and its release settles there or back ({@link swipeStep}); a touch that starts on a tall card
 * scrolls the card while it can and flings it on release; once a touch is a swipe the root captures its pointer, so the rest of the gesture
 * reaches it even when the element it started on re-renders away; a second finger (never the primary pointer) hands the gesture to the
 * browser's pinch zoom, while a new primary touch always starts a new gesture. The wheel steps one cell per gesture along its longer axis
 * (Shift turns it sideways) once no card under it can scroll that way. The click a swipe would end in is swallowed. Mouse drags are left
 * alone. The root, every card's cell and everything inside a cell (at zero specificity, so a card's own
 * `touch-action` wins) carry `touch-action: pinch-zoom`: a browser reads `touch-action` only up to the nearest scroll container, and a cell
 * that may scroll is one, as is any scrolling part of a card, so without its own the browser would take the swipe — every scroller inside a
 * card is scrolled here instead.
 * @see https://w3c.github.io/pointerevents/#determining-supported-direct-manipulation-behavior */
function useSwipe(rootRef: React.RefObject<HTMLDivElement | null>, enabled: boolean, drive: React.RefObject<LayeredSwipeDrive>): void {
  React.useEffect(() => {
    const root = rootRef.current;
    if (!enabled || root === null) return;
    let touch: LayeredTouch | null = null;
    let fling = 0;
    let swallowUntil = -Infinity;
    const wheel = { at: -Infinity, sum: 0, spent: false };
    const size = (axis: LayeredAxis): number => Math.max(1, axis === "x" ? root.clientWidth : root.clientHeight);
    const residual = (current: LayeredTouch, axis: LayeredAxis): number => along(axis, current.start.x - current.from.column, current.start.y - current.from.row);
    const flingScroller = (element: HTMLElement, axis: LayeredAxis, velocity: number) => {
      let speed = velocity;
      let last = performance.now();
      const tick = (now: number) => {
        const step = flingStep(speed, now - last);
        last = now;
        const before = scrollPosition(element, axis);
        scrollAlong(element, axis, before + step.distance);
        speed = step.velocity;
        if (!step.done && scrollPosition(element, axis) !== before) fling = requestAnimationFrame(tick);
      };
      fling = requestAnimationFrame(tick);
    };
    const onDown = (event: PointerEvent) => {
      if (event.pointerType === "mouse") return;
      if (touch !== null) {
        const held = touch;
        touch = null;
        if (held.axis !== null && held.scroller === null) drive.current.settle(cellOffset(held.from));
        if (!event.isPrimary) return;
      }
      cancelAnimationFrame(fling);
      drive.current.finish();
      const start = drive.current.offset();
      touch = { id: event.pointerId, x: event.clientX, y: event.clientY, target: event.target, samples: [{ t: event.timeStamp, x: event.clientX, y: event.clientY }], start, from: nearestCell(start, drive.current.cells()), axis: null, scroller: null, scrolled: 0, aimed: 0 };
    };
    const onMove = (event: PointerEvent) => {
      const current = touch;
      if (current === null || event.pointerId !== current.id) return;
      current.samples.push({ t: event.timeStamp, x: event.clientX, y: event.clientY });
      if (current.samples.length > 32) current.samples.splice(0, current.samples.length - 32);
      const dx = event.clientX - current.x;
      const dy = event.clientY - current.y;
      if (current.axis === null) {
        const axis = swipeAxis(dx, dy);
        if (axis === null) return;
        current.axis = axis;
        capturePointer(root, event.pointerId);
        current.scroller = scrollerToward(current.target, root, axis, -Math.sign(along(axis, dx, dy)));
        if (current.scroller !== null) current.scrolled = scrollPosition(current.scroller, axis);
        else {
          drive.current.stop();
          current.start = drive.current.offset();
          current.from = nearestCell(current.start, drive.current.cells());
        }
      }
      const moved = along(current.axis, dx, dy);
      if (current.scroller !== null) {
        scrollAlong(current.scroller, current.axis, current.scrolled - moved);
        return;
      }
      const shift = residual(current, current.axis) - moved / size(current.axis);
      if (Math.sign(shift) !== 0 && Math.sign(shift) !== current.aimed) {
        current.aimed = Math.sign(shift);
        drive.current.aim(current.from, current.axis, current.aimed);
      }
      drive.current.drag(swipeDragOffset(current.from, current.axis, shift, drive.current.cells()));
    };
    const onEnd = (event: PointerEvent) => {
      const current = touch;
      if (current === null || event.pointerId !== current.id) return;
      touch = null;
      if (current.axis === null) return;
      const cancelled = event.type === "pointercancel";
      if (!cancelled) swallowUntil = performance.now() + CLICK_AFTER_SWIPE_MS;
      const velocity = touchVelocity(current, current.axis);
      if (current.scroller !== null) {
        if (!cancelled) flingScroller(current.scroller, current.axis, -velocity);
        return;
      }
      const view = size(current.axis);
      const moved = along(current.axis, event.clientX - current.x, event.clientY - current.y) - residual(current, current.axis) * view;
      const step = cancelled ? 0 : swipeStep(moved, velocity, view);
      const landing = step === 0 ? null : drive.current.aim(current.from, current.axis, step);
      drive.current.settle(landing?.slot ?? cellOffset(current.from));
    };
    const onClick = (event: MouseEvent) => {
      if (performance.now() > swallowUntil) return;
      swallowUntil = -Infinity;
      event.preventDefault();
      event.stopPropagation();
    };
    const onWheel = (event: WheelEvent) => {
      if (event.ctrlKey) return;
      const scale = event.deltaMode === 1 ? 16 : event.deltaMode === 2 ? size("y") : 1;
      const sideways = event.shiftKey && event.deltaX === 0;
      const axis: LayeredAxis = sideways || Math.abs(event.deltaX) > Math.abs(event.deltaY) ? "x" : "y";
      const delta = (sideways ? event.deltaY : along(axis, event.deltaX, event.deltaY)) * scale;
      if (delta === 0 || scrollerToward(event.target, root, axis, Math.sign(delta)) !== null) return;
      event.preventDefault();
      if (event.timeStamp - wheel.at > LAYERED_WHEEL_QUIET_MS) Object.assign(wheel, { sum: 0, spent: false });
      wheel.at = event.timeStamp;
      if (wheel.spent) return;
      wheel.sum += delta;
      if (Math.abs(wheel.sum) < LAYERED_WHEEL_STEP_PX) return;
      wheel.spent = true;
      drive.current.finish();
      const landing = drive.current.aim(nearestCell(drive.current.offset(), drive.current.cells()), axis, Math.sign(wheel.sum));
      if (landing !== null) drive.current.settle(landing.slot);
    };
    root.addEventListener("pointerdown", onDown);
    root.addEventListener("pointermove", onMove);
    root.addEventListener("pointerup", onEnd);
    root.addEventListener("pointercancel", onEnd);
    root.addEventListener("click", onClick, true);
    root.addEventListener("wheel", onWheel, { passive: false });
    return () => {
      cancelAnimationFrame(fling);
      root.removeEventListener("pointerdown", onDown);
      root.removeEventListener("pointermove", onMove);
      root.removeEventListener("pointerup", onEnd);
      root.removeEventListener("pointercancel", onEnd);
      root.removeEventListener("click", onClick, true);
      root.removeEventListener("wheel", onWheel);
    };
  }, [rootRef, enabled, drive]);
}
// #endregion 👆️LayeredSwipe

// #region 🥞️LayeredOverview
/** 🥞️ A layered landing: every page on a strip of container-sized cells, ONE `ui-veil` glass above it whose `clip-path` punches a hole over the
 * revealed page, and the cards above the glass — in `strip` mode the app's card grid, fixed; in `swipe` mode a card layer of the strip's own
 * grid that moves with the strip, each card centred on its page and scrolling inside its cell when taller. It fills its positioned parent.
 * Hovering (mouse) or keyboard-focusing a card glides the strip to that page (500 ms, cubic) and shows it clear; leaving or blurring the card
 * restores the glass and the pointer pan (12 % of the gap per 60 Hz frame, by elapsed time). In `swipe` mode a touch carries the strip along
 * the axis it takes first and settles it on the neighbouring cell (320 ms, cubic ease-out); past an edge the index wraps while the motion goes
 * on — the far page is shifted into the slot beside the view, and once the strip rests there the shift and the offset are undone in the same
 * frame — and the panes one swipe away are warmed. Opening a
 * card (or `#id`) shows its page full size as a focused region; Escape or the Overview button — the element's own, or the app's chrome closing the
 * controlled page — return focus to the card unless it lies in that chrome.
 * Transforms and veil are written imperatively in percent (no render per frame, no viewport reads, a rotated phone stays on its cell), and only
 * pages near the view keep placeholders. The pan, the glide and the settle are how the landing is read, so they run whatever the device says
 * about motion (`reducedMotion` `"never"`, the default): a Remote Desktop session reports reduced motion for everyone in it. An app that wants
 * the device's request honoured passes `"auto"`, one that wants stillness `"always"` — then nothing pans and a page is simply there.
 *
 * @see ../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts — the pure geometry and lifecycle policy
 * @see ../🃏️OverviewCard/🟦️.tsx — the card the apps render
 * @see https://www.w3.org/WAI/ARIA/apg/practices/landmark-regions/ — the opened page is a labelled region */
export function LayeredOverview(props: LayeredOverviewProps): React.ReactElement {
  const { panes, cells, renderCard, labels, mode = "strip", pan = "pointer", routing = "hash", reducedMotion = "never" } = props;
  const radius = props.windowing?.radius ?? 1;
  const lifecycle = resolveLifecycle(props.lifecycle);
  const freshIds = panes.map((pane) => pane.id);
  const ids = useStableByKey(freshIds, freshIds.join("\n"));
  const freshCells = ids.map((id) => requireCell(cells, id));
  const cellList = useStableByKey(freshCells, freshCells.map((cell) => `${cell.column}:${cell.row}`).join(" "));
  const grid = React.useMemo<LayeredGrid>(() => stripGrid(cellList), [cellList]);
  const reduced = useReducedMotion(reducedMotion);
  const panning = mode === "strip" && pan === "pointer" && !reduced;
  const controlled = props.openedId !== undefined;
  const [deepLink] = React.useState<string | null>(() => (routing === "hash" ? hashPaneId(ids) : null));
  const [ownOpened, setOwnOpened] = React.useState<string | null>(controlled ? null : deepLink);
  const wanted = controlled ? (props.openedId ?? null) : ownOpened;
  const opened = wanted !== null && ids.includes(wanted) ? wanted : null;
  const [revealedId, setRevealedId] = React.useState<string | null>(null);
  const { booted, live, posters, touch, warm, markDirty, registerContainer, container } = usePaneLifecycle(panes, ids, opened, revealedId, lifecycle, mode === "strip");

  const swiping = mode === "swipe";
  const initial = opened ? cellOffset(requireCell(cells, opened)) : cellList[0] ? cellOffset(cellList[0]) : { x: 0, y: 0 };
  const paneAt = React.useMemo(() => new Map(cellList.map((cell, index) => [`${cell.column}:${cell.row}`, ids[index]!])), [cellList, ids]);
  const rootRef = React.useRef<HTMLDivElement | null>(null);
  const stripRef = React.useRef<HTMLDivElement | null>(null);
  const veilRef = React.useRef<HTMLDivElement | null>(null);
  const cardStripRef = React.useRef<HTMLDivElement | null>(null);
  const cardElements = React.useRef(new Map<string, HTMLElement>());
  const offsetRef = React.useRef<LayeredOffset>(initial);
  const targetRef = React.useRef<LayeredOffset>(initial);
  const driveRef = React.useRef<LayeredDrive>(FOLLOW);
  const frameRef = React.useRef(0);
  const epochRef = React.useRef(0);
  const runningRef = React.useRef(false);
  const revealedRef = React.useRef<string | null>(null);
  const openedRef = React.useRef(opened);
  const previousOpened = React.useRef(opened);
  const [windowBox, setWindowBox] = React.useState<LayeredWindow>(() => windowAround(initial, radius));
  const windowRef = React.useRef(windowBox);
  const [resting, setResting] = React.useState<string | null>(() => paneAt.get(`${initial.x}:${initial.y}`) ?? null);
  const restingRef = React.useRef(resting);
  const latest = React.useRef({ props, cells, cellList, grid, reduced, radius, mode, controlled, ids, paneAt });
  latest.current = { props, cells, cellList, grid, reduced, radius, mode, controlled, ids, paneAt };

  const paint = React.useCallback((offset: LayeredOffset) => {
    offsetRef.current = offset;
    const { grid: strip, cells: placed, radius: reach, paneAt: at } = latest.current;
    const transform = stripTransform(offset, strip);
    if (stripRef.current) stripRef.current.style.transform = transform;
    if (cardStripRef.current) cardStripRef.current.style.transform = transform;
    if (veilRef.current) applyVeil(veilRef.current, veilClip(revealedRef.current === null ? null : (placed[revealedRef.current] ?? null), offset));
    const next = windowAround(offset, reach);
    if (!sameWindow(next, windowRef.current)) {
      windowRef.current = next;
      setWindowBox(next);
    }
    const cell = restingCell(offset);
    const still = cell === null ? null : (at.get(`${cell.column}:${cell.row}`) ?? null);
    if (still !== restingRef.current) {
      restingRef.current = still;
      setResting(still);
    }
  }, []);

  const repaint = React.useCallback(() => paint(offsetRef.current), [paint]);

  const stop = React.useCallback(() => {
    epochRef.current += 1;
    cancelAnimationFrame(frameRef.current);
    runningRef.current = false;
  }, []);

  const ghostRef = React.useRef<LayeredGhost | null>(null);
  const cellElements = React.useRef(new Map<string, HTMLElement>());
  const cellRefs = React.useMemo(
    () =>
      new Map(
        ids.map((id) => [
          id,
          (element: HTMLDivElement | null) => {
            if (element) cellElements.current.set(id, element);
            else cellElements.current.delete(id);
          },
        ]),
      ),
    [ids],
  );

  const aim = React.useCallback(
    (landing: LayeredLanding | null) => {
      const ghost = landing === null || (landing.slot.x === landing.cell.column && landing.slot.y === landing.cell.row) ? null : { id: latest.current.paneAt.get(`${landing.cell.column}:${landing.cell.row}`)!, ...landing };
      const previous = ghostRef.current;
      if (previous !== null && previous.id !== ghost?.id) for (const element of [container(previous.id), cellElements.current.get(previous.id)]) if (element) element.style.transform = "";
      ghostRef.current = ghost;
      if (ghost === null) return;
      const transform = `translate(${(ghost.slot.x - ghost.cell.column) * 100}%, ${(ghost.slot.y - ghost.cell.row) * 100}%)`;
      for (const element of [container(ghost.id), cellElements.current.get(ghost.id)]) if (element) element.style.transform = transform;
    },
    [container],
  );

  const settleGhost = React.useCallback(() => {
    const ghost = ghostRef.current;
    if (ghost === null) return;
    const at = offsetRef.current;
    aim(null);
    if (at.x !== ghost.slot.x || at.y !== ghost.slot.y) return;
    targetRef.current = cellOffset(ghost.cell);
    paint(targetRef.current);
  }, [aim, paint]);

  const finishGhost = React.useCallback(() => {
    if (ghostRef.current === null) return;
    if (runningRef.current) {
      stop();
      driveRef.current = FOLLOW;
      paint(targetRef.current);
    }
    settleGhost();
  }, [paint, settleGhost, stop]);

  const run = React.useCallback(() => {
    stop();
    runningRef.current = true;
    const epoch = epochRef.current;
    let painted = performance.now();
    const advance = (drive: LayeredDrive, now: number): boolean => {
      const elapsed = now - painted;
      painted = now;
      if (drive.kind !== "follow") {
        const step = (drive.kind === "glide" ? glideOffset : settleOffset)(drive.from, drive.to, now - drive.startedAt);
        paint(step.offset);
        return step.done;
      }
      const step = followStep(offsetRef.current, targetRef.current, elapsed);
      paint(step.offset);
      return step.settled;
    };
    const tick = () => {
      if (epoch !== epochRef.current) return;
      if (advance(driveRef.current, performance.now())) {
        driveRef.current = FOLLOW;
        runningRef.current = false;
        settleGhost();
        return;
      }
      frameRef.current = requestAnimationFrame(tick);
    };
    frameRef.current = requestAnimationFrame(tick);
  }, [paint, settleGhost, stop]);

  const moveTo = React.useCallback(
    (to: LayeredOffset, kind: "glide" | "settle") => {
      targetRef.current = to;
      const from = offsetRef.current;
      if (latest.current.reduced || (from.x === to.x && from.y === to.y)) {
        stop();
        driveRef.current = FOLLOW;
        paint(to);
        settleGhost();
        return;
      }
      driveRef.current = { kind, from, to, startedAt: performance.now() };
      run();
    },
    [paint, run, settleGhost, stop],
  );
  const glideTo = React.useCallback((to: LayeredOffset) => moveTo(to, "glide"), [moveTo]);

  const follow = React.useCallback(
    (to: LayeredOffset) => {
      targetRef.current = to;
      if (driveRef.current.kind === "follow" && runningRef.current) return;
      driveRef.current = FOLLOW;
      run();
    },
    [run],
  );

  const focusView = React.useCallback((id: string) => glideTo(cellOffset(requireCell(latest.current.cells, id))), [glideTo]);

  const swipeDrive = React.useRef<LayeredSwipeDrive>({
    offset: () => offsetRef.current,
    cells: () => latest.current.cellList,
    drag: (offset) => {
      stop();
      driveRef.current = FOLLOW;
      targetRef.current = offset;
      paint(offset);
    },
    aim: (from, axis, step) => {
      const landing = swipeWrapTarget(from, axis, step, latest.current.cellList);
      aim(landing);
      return landing;
    },
    settle: (to) => moveTo(to, "settle"),
    finish: finishGhost,
    stop,
  });
  useSwipe(rootRef, swiping && opened === null, swipeDrive);

  React.useEffect(() => stop, [stop]);

  const stripCallback = React.useCallback(
    (element: HTMLDivElement | null) => {
      stripRef.current = element;
      if (element) repaint();
    },
    [repaint],
  );
  const veilCallback = React.useCallback(
    (element: HTMLDivElement | null) => {
      veilRef.current = element;
      if (element) repaint();
    },
    [repaint],
  );
  const cardStripCallback = React.useCallback(
    (element: HTMLDivElement | null) => {
      cardStripRef.current = element;
      if (element) repaint();
    },
    [repaint],
  );
  const cardCallback = React.useCallback((id: string, element: HTMLElement | null) => {
    if (element) cardElements.current.set(id, element);
    else cardElements.current.delete(id);
  }, []);

  const firstCells = React.useRef(true);
  React.useLayoutEffect(() => {
    if (firstCells.current) {
      firstCells.current = false;
      return;
    }
    const focus = openedRef.current ?? revealedRef.current;
    stop();
    aim(null);
    driveRef.current = FOLLOW;
    const next = focus === null ? clampOffset(offsetRef.current, cellList) : cellOffset(requireCell(latest.current.cells, focus));
    targetRef.current = next;
    paint(next);
  }, [aim, cellList, paint, stop]);

  const reveal = React.useCallback(
    (id: string) => {
      if (openedRef.current !== null || revealedRef.current === id) return;
      revealedRef.current = id;
      setRevealedId(id);
      latest.current.props.onRevealedIdChange?.(id);
      touch(id);
      finishGhost();
      focusView(id);
      repaint();
    },
    [finishGhost, focusView, repaint, touch],
  );

  const conceal = React.useCallback(
    (id: string) => {
      if (revealedRef.current !== id) return;
      revealedRef.current = null;
      setRevealedId(null);
      latest.current.props.onRevealedIdChange?.(null);
      repaint();
    },
    [repaint],
  );

  const requestOpen = React.useCallback((id: string | null) => {
    if (id === openedRef.current) return;
    if (!latest.current.controlled) setOwnOpened(id);
    latest.current.props.onOpenedIdChange?.(id);
  }, []);
  const close = React.useCallback(() => requestOpen(null), [requestOpen]);

  React.useEffect(() => {
    if (deepLink !== null && (!latest.current.controlled || deepLink !== openedRef.current)) latest.current.props.onOpenedIdChange?.(deepLink);
  }, [deepLink]);

  React.useLayoutEffect(() => {
    const previous = previousOpened.current;
    previousOpened.current = opened;
    openedRef.current = opened;
    if (routing === "hash" && (previous !== opened || deepLink === null)) syncHash(opened, latest.current.ids);
    if (previous === opened) return;
    if (revealedRef.current !== null) {
      revealedRef.current = null;
      setRevealedId(null);
      latest.current.props.onRevealedIdChange?.(null);
    }
    if (opened !== null) {
      finishGhost();
      focusView(opened);
      const region = container(opened);
      if (region && !region.contains(document.activeElement)) region.focus({ preventScroll: true });
      return;
    }
    if (previous === null) return;
    const active = document.activeElement;
    if (active && active !== document.body && rootRef.current && !rootRef.current.contains(active)) return;
    cardElements.current.get(previous)?.querySelector<HTMLElement>(FOCUSABLE)?.focus({ preventScroll: true });
  }, [opened, routing, deepLink, container, finishGhost, focusView]);

  React.useEffect(() => {
    if (routing !== "hash") return;
    const onHash = () => {
      const raw = window.location.hash.replace(/^#/, "");
      if (latest.current.ids.includes(raw)) requestOpen(raw);
      else if (raw === "") requestOpen(null);
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  }, [routing, requestOpen]);

  React.useEffect(() => {
    if (opened === null) return;
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape" && !event.defaultPrevented) requestOpen(null);
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [opened, requestOpen]);

  React.useEffect(() => {
    if (!panning) return;
    const onMove = (event: PointerEvent) => {
      if (event.pointerType !== "mouse" || openedRef.current !== null || revealedRef.current !== null) return;
      const box = rootRef.current?.getBoundingClientRect();
      if (!box || box.width <= 0 || box.height <= 0) return;
      const { grid: strip, cellList: placed } = latest.current;
      follow(clampOffset(pointerOffset((event.clientX - box.left) / box.width, (event.clientY - box.top) / box.height, strip), placed));
    };
    window.addEventListener("pointermove", onMove, { passive: true });
    return () => window.removeEventListener("pointermove", onMove);
  }, [panning, follow]);

  React.useEffect(() => {
    if (swiping && opened === null && resting !== null) touch(resting);
  }, [swiping, opened, resting, touch]);

  React.useEffect(() => {
    if (!swiping || resting === null) return;
    const step = neighbourWarmBoot(ids, cellList, requireCell(latest.current.cells, resting), live, opened, lifecycle.budget);
    if (step.kind === "boot") warm(step.id);
  }, [swiping, resting, ids, cellList, live, opened, lifecycle.budget, warm]);

  const paneStyles = React.useMemo(() => new Map(ids.map((id, index) => [id, { gridColumn: cellList[index]!.column + 1, gridRow: cellList[index]!.row + 1 }])), [ids, cellList]);
  const insetTop = props.insets?.top ?? "0px";
  const insetBottom = props.insets?.bottom ?? "0px";
  const insets = React.useMemo(() => ({ top: insetTop, bottom: insetBottom }), [insetTop, insetBottom]);
  const cellStyles = React.useMemo(() => new Map([...paneStyles].map(([id, style]) => [id, { ...style, touchAction: "pinch-zoom", paddingTop: `calc(${insets.top} + 44px)`, paddingBottom: `calc(${insets.bottom} + 44px)` }])), [paneStyles, insets]);
  const restedOn = React.useRef(resting);
  if (resting !== null) restedOn.current = resting;
  const hintFrom = restedOn.current;
  const neighbours = React.useMemo<readonly LayeredNeighbour[]>(
    () => (hintFrom === null ? [] : swipeNeighbours(requireCell(cells, hintFrom), cellList).map(({ direction, cell, slot }) => ({ direction, cell, slot, pane: panes[ids.indexOf(paneAt.get(`${cell.column}:${cell.row}`)!)]! }))),
    [hintFrom, cells, cellList, panes, ids, paneAt],
  );
  const goTo = React.useCallback(
    (neighbour: LayeredNeighbour) => {
      finishGhost();
      aim(neighbour);
      moveTo(neighbour.slot, "settle");
    },
    [aim, finishGhost, moveTo],
  );
  const nearby = React.useMemo(() => new Set(neighbours.map(({ pane }) => pane.id)), [neighbours]);
  const stripStyle: React.CSSProperties = { width: `${grid.columns * 100}%`, height: `${grid.rows * 100}%`, gridTemplateColumns: `repeat(${grid.columns}, minmax(0, 1fr))`, gridTemplateRows: `repeat(${grid.rows}, minmax(0, 1fr))`, contain: swiping ? "layout" : "layout paint" };
  const paneView = (pane: LayeredPane, index: number) => (
    <LayeredPaneView
      key={pane.id}
      pane={pane}
      style={paneStyles.get(pane.id)}
      opened={opened === pane.id}
      revealed={revealedId === pane.id}
      live={live.has(pane.id)}
      shown={inWindow(cellList[index]!, windowBox) || (swiping && nearby.has(pane.id))}
      busy={booted.has(pane.id)}
      poster={posters.get(pane.id) ?? pane.poster ?? null}
      waiting={labels.waiting(pane)}
      failed={labels.failed(pane)}
      onContainer={registerContainer}
      onDirty={markDirty}
    />
  );
  const cardHost = (pane: LayeredPane) => (
    <LayeredCardHost key={pane.id} pane={pane} mode={mode} revealed={revealedId === pane.id} onOpen={requestOpen} onReveal={reveal} onConceal={conceal} onElement={cardCallback} renderCard={renderCard} />
  );
  const chrome = props.renderChrome?.({ revealed: revealedId !== null, opened: opened !== null });
  const overviewButton =
    opened === null || labels.overview === undefined ? null : (
      <button
        type="button"
        data-layered-overview-button=""
        data-level="dialog"
        aria-keyshortcuts="Escape"
        onClick={close}
        className="ui-glass absolute right-double top-double z-40 inline-flex items-center gap-single border border-border-normal px-single py-half text-sm font-medium text-foreground outline-none transition-colors hover:border-border-emphasized focus-visible:ring-2 focus-visible:ring-ring motion-reduce:transition-none"
      >
        <Icon icon="layout-grid" size="small" />
        {labels.overview}
      </button>
    );

  return (
    <div
      ref={rootRef}
      data-layered-overview=""
      data-mode={mode}
      data-pan={panning ? "pointer" : "none"}
      className="relative h-full w-full overflow-clip bg-background text-foreground"
      style={swiping && opened === null ? { touchAction: "pinch-zoom" } : undefined}
    >
      {overviewButton}
      <div ref={stripCallback} data-layered-strip="" className="grid will-change-transform" style={stripStyle}>
        {panes.map(paneView)}
      </div>
      {opened === null ? (
        <>
          <div ref={veilCallback} data-layered-veil="" data-level="dialog" className="ui-veil pointer-events-none absolute inset-0 z-30" />
          {swiping ? (
            <>
              <div ref={cardStripCallback} data-layered-overlay="" role="group" aria-label={labels.grid} className="pointer-events-none absolute left-0 top-0 z-[31] grid will-change-transform" style={stripStyle}>
                {panes.map((pane) => (
                  <div key={pane.id} ref={cellRefs.get(pane.id)} data-layered-cell={pane.id} className="pointer-events-none flex min-h-0 min-w-0 items-center-safe justify-center overflow-y-auto px-[36px] [:where(&)_*]:touch-pinch-zoom" style={cellStyles.get(pane.id)}>
                    {cardHost(pane)}
                  </div>
                ))}
              </div>
              <LayeredNeighbourHints neighbours={neighbours} moving={resting === null} insets={insets} label={labels.neighbour} onGo={goTo} />
            </>
          ) : (
            <div data-layered-overlay="" role="group" aria-label={labels.grid} className={cn("pointer-events-none absolute inset-0 z-[31]", props.overlayClassName)} style={props.overlayStyle}>
              {panes.map(cardHost)}
            </div>
          )}
        </>
      ) : null}
      {chrome}
    </div>
  );
}
// #endregion 🥞️LayeredOverview
