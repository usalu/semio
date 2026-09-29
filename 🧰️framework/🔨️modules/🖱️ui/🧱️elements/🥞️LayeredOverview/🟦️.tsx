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
  LAYERED_VIEW,
  cellOffset,
  clampOffset,
  coverPlacement,
  followStep,
  glideOffset,
  glideRect,
  inWindow,
  nextWarmBoot,
  panesOverBudget,
  panesToRelease,
  pointerOffset,
  resolveLifecycle,
  restRect,
  scheduleIdle,
  stripGrid,
  stripTransform,
  trackTemplate,
  veilClip,
  veilClipPath,
  veilForRect,
  viewRect,
  warmDelay,
  windowAround,
  type LayeredCell,
  type LayeredGrid,
  type LayeredLifecycle,
  type LayeredOffset,
  type LayeredRect,
  type LayeredTracks,
  type LayeredVeil,
  type LayeredWindow,
} from "../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import { Icon, type IconName } from "../🔣️Icons/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🧬️LayeredOverviewContracts
/** 🧭️ `strip`: every pane on one panorama under one glass with the card grid above; `list`: one snap section per pane, each under its own glass (touch phones). */
export type LayeredMode = "strip" | "list";

/** 🔍️ What the strip shows at rest: `panorama` — one container-sized page, panned by the pointer (play, the demonstrator); `grid` — every page at
 * once, each scaled without distortion to cover its track cell (`gridTracks`) exactly behind its card, all live; a reveal zooms its page to full
 * size. */
export type LayeredRest = "panorama" | "grid";

/** 🥞️ One backdrop page, the real page its card opens: `id` is its hash route, `data-layered-pane` and key, `label` names the opened page and its
 * placeholder, `render` is called only while the pane is mounted, `poster` is shown while it is not, and `capturePoster` (for example
 * {@link capturePosterFromCanvases}) turns the mounted page into a poster right before it is released. */
export interface LayeredPane {
  readonly id: string;
  readonly label: string;
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

/** 🌐️ The element's own words, always the app's (no default language): the card group, the Overview button, a waiting and a failed page. */
export interface LayeredLabels {
  readonly grid: string;
  readonly overview: string;
  readonly waiting: (pane: LayeredPane) => string;
  readonly failed: (pane: LayeredPane) => string;
}

/** 🥞️ Props of {@link LayeredOverview}. `cells` places every pane on the strip for the current breakpoint; the card grid is the app's CSS
 * (`overlayClassName`/`overlayStyle`, each card wrapped in a `display: contents` host; the overlay carries `--layered-columns`/`--layered-rows`,
 * the track lists of `gridTracks`); `openedId` makes the open page controlled. */
export interface LayeredOverviewProps {
  readonly panes: readonly LayeredPane[];
  readonly cells: Readonly<Record<string, LayeredCell>>;
  readonly renderCard: (pane: LayeredPane, state: LayeredCardState) => React.ReactNode;
  readonly overlayClassName?: string;
  readonly overlayStyle?: React.CSSProperties;
  readonly renderChrome?: (state: LayeredChromeState) => React.ReactNode;
  readonly mode?: LayeredMode;
  readonly rest?: LayeredRest;
  readonly gridTracks?: LayeredTracks;
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

type LayeredDrive =
  | { readonly kind: "follow" }
  | { readonly kind: "glide"; readonly from: LayeredOffset; readonly to: LayeredOffset; readonly startedAt: number }
  | { readonly kind: "zoom"; readonly from: LayeredRect; readonly to: LayeredRect; readonly startedAt: number };
const FOLLOW: LayeredDrive = { kind: "follow" };
const GRID_PANE_STYLE: React.CSSProperties = { position: "absolute", inset: 0, transformOrigin: "0 0", willChange: "transform" };

/** 🧮️ How panes start: a paced `warm` queue (panorama), `all` at once (the grid rest shows every page), or on `demand` only (list). */
type LayeredStart = "warm" | "all" | "demand";

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

/** 🧮️ Mounts, releases and posters of the panes: boots on reveal/open/list scroll, a warm queue in cell order, a live `budget` (least recently
 * touched pristine pane first), time-based release, posters from `capturePoster`. Dirty, opened and revealed panes are never released; the most
 * recently opened pane (initially the first) is never released by time while the overview shows. */
function usePaneLifecycle(panes: readonly LayeredPane[], ids: readonly string[], opened: string | null, revealed: string | null, lifecycle: LayeredLifecycle, start: LayeredStart) {
  const initial = start === "all" ? ids : opened ? [opened] : [];
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
    if (start === "all") for (const id of ids) if (!state.current.live.has(id)) boot(id);
  }, [start, ids, boot]);

  React.useEffect(() => {
    if (start !== "warm") return;
    const step = nextWarmBoot(ids, booted, live.size, opened, lifecycle.budget);
    if (step.kind !== "boot") return;
    return scheduleIdle(() => warm(step.id), warmDelay(warmed.current, lifecycle), window);
  }, [start, ids, booted, live.size, opened, lifecycle.budget, lifecycle.warmStartMs, lifecycle.warmIntervalMs, warm]);

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

// #region 🥞️LayeredOverview
/** 🥞️ A layered landing: every page on a strip of container-sized cells, ONE `ui-veil` glass above it whose `clip-path` punches a hole over the
 * revealed page, and the app's card grid above the glass. It fills its positioned parent. Hovering (mouse) or keyboard-focusing a card glides the
 * strip to that page (500 ms, cubic) and shows it clear; leaving or blurring the card restores the glass and the pointer pan (lerp 0.12). Opening a
 * card (or `#id`) shows its page full size as a focused region; Escape or the Overview button return focus to the card. With `rest="grid"` every
 * page is live and visible at rest, each covering its track cell behind its card, and a reveal zooms its page to full size instead of panning.
 * Transforms and veil are written imperatively in percent (no render per frame, no viewport reads), reduced motion snaps, and only pages near the
 * view keep placeholders.
 *
 * @see ../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts — the pure geometry and lifecycle policy
 * @see ../🃏️OverviewCard/🟦️.tsx — the card the apps render
 * @see https://www.w3.org/WAI/ARIA/apg/practices/landmark-regions/ — the opened page is a labelled region */
export function LayeredOverview(props: LayeredOverviewProps): React.ReactElement {
  const { panes, cells, renderCard, labels, mode = "strip", pan = "pointer", routing = "hash", reducedMotion = "auto" } = props;
  const radius = props.windowing?.radius ?? 1;
  const lifecycle = resolveLifecycle(props.lifecycle);
  const freshIds = panes.map((pane) => pane.id);
  const ids = useStableByKey(freshIds, freshIds.join("\n"));
  const freshCells = ids.map((id) => requireCell(cells, id));
  const cellList = useStableByKey(freshCells, freshCells.map((cell) => `${cell.column}:${cell.row}`).join(" "));
  const grid = React.useMemo<LayeredGrid>(() => stripGrid(cellList), [cellList]);
  const gridRest = mode === "strip" && props.rest === "grid";
  const freshTracks = props.gridTracks ?? {};
  const tracks = useStableByKey(freshTracks, JSON.stringify(freshTracks));
  const rests = React.useMemo(() => cellList.map((cell) => restRect(cell, grid, tracks)), [cellList, grid, tracks]);
  const reduced = useReducedMotion(reducedMotion);
  const controlled = props.openedId !== undefined;
  const [deepLink] = React.useState<string | null>(() => (routing === "hash" ? hashPaneId(ids) : null));
  const [ownOpened, setOwnOpened] = React.useState<string | null>(controlled ? null : deepLink);
  const wanted = controlled ? (props.openedId ?? null) : ownOpened;
  const opened = wanted !== null && ids.includes(wanted) ? wanted : null;
  const [revealedId, setRevealedId] = React.useState<string | null>(null);
  const { booted, live, posters, touch, warm, markDirty, registerContainer, container } = usePaneLifecycle(panes, ids, opened, revealedId, lifecycle, mode === "list" ? "demand" : gridRest ? "all" : "warm");

  const initial = opened ? cellOffset(requireCell(cells, opened)) : cellList[0] ? cellOffset(cellList[0]) : { x: 0, y: 0 };
  const rootRef = React.useRef<HTMLDivElement | null>(null);
  const stripRef = React.useRef<HTMLDivElement | null>(null);
  const veilRef = React.useRef<HTMLDivElement | null>(null);
  const listRef = React.useRef<HTMLDivElement | null>(null);
  const cardElements = React.useRef(new Map<string, HTMLElement>());
  const offsetRef = React.useRef<LayeredOffset>(initial);
  const targetRef = React.useRef<LayeredOffset>(initial);
  const cameraRef = React.useRef<LayeredRect>(opened && gridRest ? rests[ids.indexOf(opened)]! : LAYERED_VIEW);
  const driveRef = React.useRef<LayeredDrive>(FOLLOW);
  const frameRef = React.useRef(0);
  const epochRef = React.useRef(0);
  const runningRef = React.useRef(false);
  const revealedRef = React.useRef<string | null>(null);
  const openedRef = React.useRef(opened);
  const previousOpened = React.useRef(opened);
  const listFrame = React.useRef<number | null>(null);
  const [windowBox, setWindowBox] = React.useState<LayeredWindow>(() => windowAround(initial, radius));
  const windowRef = React.useRef(windowBox);
  const [listIndex, setListIndex] = React.useState(() => Math.max(0, opened ? ids.indexOf(opened) : 0));
  const latest = React.useRef({ props, cells, cellList, grid, rests, gridRest, reduced, radius, mode, controlled, ids });
  latest.current = { props, cells, cellList, grid, rests, gridRest, reduced, radius, mode, controlled, ids };

  const paint = React.useCallback((offset: LayeredOffset) => {
    offsetRef.current = offset;
    const { grid: strip, cells: placed, radius: reach } = latest.current;
    if (stripRef.current) stripRef.current.style.transform = stripTransform(offset, strip);
    if (veilRef.current) applyVeil(veilRef.current, veilClip(revealedRef.current === null ? null : (placed[revealedRef.current] ?? null), offset));
    const next = windowAround(offset, reach);
    if (!sameWindow(next, windowRef.current)) {
      windowRef.current = next;
      setWindowBox(next);
    }
  }, []);

  const paintCamera = React.useCallback(
    (camera: LayeredRect) => {
      cameraRef.current = camera;
      const { ids: all, rests: resting } = latest.current;
      all.forEach((id, index) => {
        const element = container(id);
        if (!element) return;
        const placement = coverPlacement(viewRect(resting[index]!, camera));
        element.style.transform = placement.transform;
        element.style.clipPath = placement.clipPath;
      });
      const shown = revealedRef.current === null ? -1 : all.indexOf(revealedRef.current);
      if (veilRef.current) applyVeil(veilRef.current, veilForRect(shown < 0 ? null : viewRect(resting[shown]!, camera)));
    },
    [container],
  );

  const repaint = React.useCallback(() => (latest.current.gridRest ? paintCamera(cameraRef.current) : paint(offsetRef.current)), [paint, paintCamera]);

  const stop = React.useCallback(() => {
    epochRef.current += 1;
    cancelAnimationFrame(frameRef.current);
    runningRef.current = false;
  }, []);

  const run = React.useCallback(() => {
    stop();
    runningRef.current = true;
    const epoch = epochRef.current;
    const advance = (drive: LayeredDrive, now: number): boolean => {
      if (drive.kind === "zoom") {
        const step = glideRect(drive.from, drive.to, now - drive.startedAt);
        paintCamera(step.rect);
        return step.done;
      }
      if (drive.kind === "glide") {
        const step = glideOffset(drive.from, drive.to, now - drive.startedAt);
        paint(step.offset);
        return step.done;
      }
      const step = followStep(offsetRef.current, targetRef.current);
      paint(step.offset);
      return step.settled;
    };
    const tick = () => {
      if (epoch !== epochRef.current) return;
      if (advance(driveRef.current, performance.now())) {
        driveRef.current = FOLLOW;
        runningRef.current = false;
        return;
      }
      frameRef.current = requestAnimationFrame(tick);
    };
    frameRef.current = requestAnimationFrame(tick);
  }, [paint, paintCamera, stop]);

  const zoomTo = React.useCallback(
    (to: LayeredRect) => {
      const from = cameraRef.current;
      if (latest.current.reduced || (from.x === to.x && from.y === to.y && from.width === to.width && from.height === to.height)) {
        stop();
        driveRef.current = FOLLOW;
        paintCamera(to);
        return;
      }
      driveRef.current = { kind: "zoom", from, to, startedAt: performance.now() };
      run();
    },
    [paintCamera, run, stop],
  );

  const glideTo = React.useCallback(
    (to: LayeredOffset) => {
      targetRef.current = to;
      const from = offsetRef.current;
      if (latest.current.reduced || (from.x === to.x && from.y === to.y)) {
        stop();
        driveRef.current = FOLLOW;
        paint(to);
        return;
      }
      driveRef.current = { kind: "glide", from, to, startedAt: performance.now() };
      run();
    },
    [paint, run, stop],
  );

  const follow = React.useCallback(
    (to: LayeredOffset) => {
      targetRef.current = to;
      if (driveRef.current.kind === "follow" && runningRef.current) return;
      driveRef.current = FOLLOW;
      run();
    },
    [run],
  );

  const focusView = React.useCallback(
    (id: string | null) => {
      const { gridRest: zooming, rests: resting, ids: all, cells: placed } = latest.current;
      if (zooming) zoomTo(id === null ? LAYERED_VIEW : resting[all.indexOf(id)]!);
      else if (id !== null) glideTo(cellOffset(requireCell(placed, id)));
    },
    [glideTo, zoomTo],
  );

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
    driveRef.current = FOLLOW;
    if (gridRest) {
      paintCamera(focus === null ? LAYERED_VIEW : rests[latest.current.ids.indexOf(focus)]!);
      return;
    }
    const next = focus === null ? clampOffset(offsetRef.current, cellList) : cellOffset(requireCell(latest.current.cells, focus));
    targetRef.current = next;
    paint(next);
  }, [cellList, rests, gridRest, paint, paintCamera, stop]);

  const reveal = React.useCallback(
    (id: string) => {
      if (openedRef.current !== null || revealedRef.current === id) return;
      revealedRef.current = id;
      setRevealedId(id);
      latest.current.props.onRevealedIdChange?.(id);
      touch(id);
      if (latest.current.mode !== "strip") return;
      focusView(id);
      repaint();
    },
    [focusView, repaint, touch],
  );

  const conceal = React.useCallback(
    (id: string) => {
      if (revealedRef.current !== id) return;
      revealedRef.current = null;
      setRevealedId(null);
      latest.current.props.onRevealedIdChange?.(null);
      repaint();
      if (latest.current.gridRest && openedRef.current === null) focusView(null);
    },
    [focusView, repaint],
  );

  const requestOpen = React.useCallback((id: string | null) => {
    if (id === openedRef.current) return;
    if (!latest.current.controlled) setOwnOpened(id);
    latest.current.props.onOpenedIdChange?.(id);
  }, []);
  const close = React.useCallback(() => requestOpen(null), [requestOpen]);

  const scrollListTo = React.useCallback((index: number) => {
    const list = listRef.current;
    if (!list || list.clientHeight <= 0) return;
    list.scrollTop = index * list.clientHeight;
    setListIndex(index);
  }, []);

  React.useEffect(() => {
    if (deepLink !== null && (!latest.current.controlled || deepLink !== openedRef.current)) latest.current.props.onOpenedIdChange?.(deepLink);
  }, [deepLink]);

  React.useLayoutEffect(() => {
    if (mode === "list" && openedRef.current !== null) scrollListTo(latest.current.ids.indexOf(openedRef.current));
  }, [mode, scrollListTo]);

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
      if (mode === "strip") focusView(opened);
      else scrollListTo(latest.current.ids.indexOf(opened));
      const region = container(opened);
      if (region && !region.contains(document.activeElement)) region.focus({ preventScroll: true });
      return;
    }
    if (previous === null) return;
    if (mode === "list") scrollListTo(latest.current.ids.indexOf(previous));
    else if (latest.current.gridRest) focusView(null);
    const active = document.activeElement;
    if (active && active !== document.body && rootRef.current && !rootRef.current.contains(active)) return;
    cardElements.current.get(previous)?.querySelector<HTMLElement>(FOCUSABLE)?.focus({ preventScroll: true });
  }, [opened, mode, routing, deepLink, container, focusView, scrollListTo]);

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
    if (mode !== "strip" || gridRest || pan !== "pointer" || opened !== null || reduced) return;
    const onMove = (event: PointerEvent) => {
      if (event.pointerType !== "mouse" || revealedRef.current !== null) return;
      const box = rootRef.current?.getBoundingClientRect();
      if (!box || box.width <= 0 || box.height <= 0) return;
      const { grid: strip, cellList: placed } = latest.current;
      follow(clampOffset(pointerOffset((event.clientX - box.left) / box.width, (event.clientY - box.top) / box.height, strip), placed));
    };
    window.addEventListener("pointermove", onMove, { passive: true });
    return () => window.removeEventListener("pointermove", onMove);
  }, [mode, gridRest, pan, opened, reduced, follow]);

  const onListScroll = React.useCallback(() => {
    if (openedRef.current !== null || listFrame.current !== null) return;
    listFrame.current = requestAnimationFrame(() => {
      listFrame.current = null;
      const list = listRef.current;
      if (list && list.clientHeight > 0) setListIndex(Math.round(list.scrollTop / list.clientHeight));
    });
  }, []);

  React.useEffect(() => {
    if (mode !== "list" || opened !== null) return;
    const current = ids[listIndex];
    const next = ids[listIndex + 1];
    if (current !== undefined) touch(current);
    if (next !== undefined && live.size < lifecycle.budget) warm(next);
  }, [mode, opened, listIndex, ids, live.size, lifecycle.budget, touch, warm]);

  const paneStyles = React.useMemo(() => new Map(ids.map((id, index) => [id, { gridColumn: cellList[index]!.column + 1, gridRow: cellList[index]!.row + 1 }])), [ids, cellList]);
  const paneView = (pane: LayeredPane, index: number) => (
    <LayeredPaneView
      key={pane.id}
      pane={pane}
      style={gridRest ? GRID_PANE_STYLE : mode === "strip" ? paneStyles.get(pane.id) : undefined}
      opened={opened === pane.id}
      revealed={revealedId === pane.id}
      live={live.has(pane.id)}
      shown={gridRest || (mode === "strip" ? inWindow(cellList[index]!, windowBox) : Math.abs(index - listIndex) <= radius)}
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
    opened === null ? null : (
      <button
        type="button"
        data-layered-overview-button=""
        data-level="dialog"
        onClick={close}
        className="ui-glass absolute right-double top-double z-40 inline-flex items-center gap-single border border-border-normal px-single py-half text-sm font-medium text-foreground outline-none transition-colors hover:border-border-emphasized focus-visible:ring-2 focus-visible:ring-ring motion-reduce:transition-none"
      >
        <Icon icon="layout-grid" size="small" />
        {labels.overview}
      </button>
    );

  if (mode === "list") {
    return (
      <div ref={rootRef} data-layered-overview="" data-mode="list" className="relative h-full w-full overflow-hidden bg-background text-foreground">
        <div ref={listRef} data-layered-list="" role="group" aria-label={labels.grid} onScroll={onListScroll} className={cn("flex h-full w-full flex-col overscroll-y-contain", opened === null ? "snap-y snap-mandatory overflow-y-auto" : "overflow-hidden")}>
          {panes.map((pane, index) => (
            <section key={pane.id} data-layered-section={pane.id} className="relative h-full w-full shrink-0 snap-start overflow-hidden">
              {paneView(pane, index)}
              {opened === null ? (
                <>
                  {Math.abs(index - listIndex) <= radius ? (
                    <div data-layered-veil="" data-level="dialog" data-veil={revealedId === pane.id ? "clear" : "whole"} className="ui-veil pointer-events-none absolute inset-0 z-30" style={revealedId === pane.id ? { visibility: "hidden" } : undefined} />
                  ) : null}
                  <div className="pointer-events-none absolute inset-0 z-[31] flex items-center justify-center px-double pb-[5.5rem]">{cardHost(pane)}</div>
                </>
              ) : null}
            </section>
          ))}
        </div>
        {chrome}
        {overviewButton}
      </div>
    );
  }

  const overlayStyle = { "--layered-columns": trackTemplate(tracks.columns, grid.columns), "--layered-rows": trackTemplate(tracks.rows, grid.rows), ...props.overlayStyle } as React.CSSProperties;
  return (
    <div ref={rootRef} data-layered-overview="" data-mode="strip" data-rest={gridRest ? "grid" : "panorama"} className="relative h-full w-full overflow-clip bg-background text-foreground">
      {gridRest ? (
        <div key="grid" ref={stripCallback} data-layered-strip="" className="relative h-full w-full" style={{ contain: "layout paint" }}>
          {panes.map(paneView)}
        </div>
      ) : (
        <div
          key="panorama"
          ref={stripCallback}
          data-layered-strip=""
          className="grid will-change-transform"
          style={{ width: `${grid.columns * 100}%`, height: `${grid.rows * 100}%`, gridTemplateColumns: `repeat(${grid.columns}, minmax(0, 1fr))`, gridTemplateRows: `repeat(${grid.rows}, minmax(0, 1fr))`, contain: "layout paint" }}
        >
          {panes.map(paneView)}
        </div>
      )}
      {opened === null ? (
        <>
          <div ref={veilCallback} data-layered-veil="" data-level="dialog" className="ui-veil pointer-events-none absolute inset-0 z-30" />
          <div data-layered-overlay="" role="group" aria-label={labels.grid} className={cn("pointer-events-none absolute inset-0 z-[31]", props.overlayClassName)} style={overlayStyle}>
            {panes.map(cardHost)}
          </div>
        </>
      ) : null}
      {chrome}
      {overviewButton}
    </div>
  );
}
// #endregion 🥞️LayeredOverview
