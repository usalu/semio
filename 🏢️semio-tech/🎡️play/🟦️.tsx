// #region 🧲️Header
/** @emoji 🎡️ semio-tech play landing — introduction, every app as a live pane in one grid, glass card overlay. */
// #endregion 🧲️Header

import { createUiErrorBoundary, mountUiRoot, useUiCallback as useCallback, useUiEffect as useEffect, useUiMemo as useMemo, useUiRef as useRef, useUiState as useState, type UiNode } from "@semio-tech/ui-react/runtime";
import {
  Navbar,
  ShellBrandLogo,
  UIIntroduction,
  bootstrapElementsSurfaceChromeDocument,
  CanvasSkeleton,
  cn,
  loadingBorderClass,
  Icon,
  initUiLocaleSync,
  readStoredUiChromeAppearance,
  readStoredUiChromeLayout,
  readStoredUiDriver,
  registerUiTranslationBundles,
  uiDataLabel,
  UI_MOBILE_MEDIA_QUERY,
  useElementsSurfaceChrome,
  useLabel,
  useMediaQuery,
} from "@semio-tech/ui-react";
import { createBrowserStoragePort, resolvePlaygroundBoot } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "@semio-tech/plugin-registry/catalog";
import { FrameworkOsShell, resolveShellLocks, resolveShellDefaults } from "@semio-tech/framework-renderer-react";
import { PUZZLE_BOARD_SESSION_FACTORIES } from "@semio-tech/puzzle-js";
import { PlayCard } from "./⚛️play-card.tsx";
import { PLAY_LOCALE, PLAY_PANES, SEMIO_TECH_PLAY_INTRODUCTION, SEMIO_TECH_PLAY_LOGO_SVG, playGridDimensions, playNextWarmBootPane, playOccupiedColumnRange, playPaneGridCell, playPanesOverBudget, schedulePlayIdle, type PlayPaneSpec } from "./🪧️brand.ts";
import "./🎨️globals.css";

// 🎡️ Page-owning (single React root, no `ShellScope` of its own) — plain browser storage is correct;
// each pane's own `FrameworkOsShell` gets its own `ShellScope` (ephemeral brands → in-memory storage).
const playStorage = createBrowserStoragePort();

bootstrapElementsSurfaceChromeDocument(readStoredUiChromeAppearance(playStorage));
initUiLocaleSync(PLAY_LOCALE);

/** @emoji 📱️ Touch-first viewports use the vertical snap list even when wider than {@link UI_MOBILE_MEDIA_QUERY}. */
const PLAY_TOUCH_LIST_MEDIA_QUERY = `${UI_MOBILE_MEDIA_QUERY} and (hover: none) and (pointer: coarse)`;

//#region 🌐️PlayLandingLabels
/** @emoji 🌐️ The landing's own chrome strings. Play locks its shells to {@link PLAY_LOCALE}, but chrome
 * never carries a default language: every key is registered for English AND German, and the page reads
 * them through `useLabel` like any other shell — so the same landing serves a German lock unchanged. */
export const playLandingUiLabel = registerUiTranslationBundles({
  en: {
    translation: {
      play: {
        landing: {
          appCount: { label: { normal: "{{apps}} apps", beginner: "{{apps}} apps" } },
          overview: { label: { normal: "Overview", beginner: "Back to all apps" } },
          paneWaiting: { label: { normal: "{{label}} is waiting to start", beginner: "{{label}} is waiting to start" } },
          paneFailed: { label: { normal: "{{label}} could not be loaded.", beginner: "{{label}} could not be loaded." } },
          grid: { label: { normal: "Every semio app", beginner: "Every semio app" } },
        },
      },
    },
  },
  de: {
    translation: {
      play: {
        landing: {
          appCount: { label: { normal: "{{apps}} Apps", beginner: "{{apps}} Apps" } },
          overview: { label: { normal: "Übersicht", beginner: "Zurück zu allen Apps" } },
          paneWaiting: { label: { normal: "{{label}} wartet auf den Start", beginner: "{{label}} wartet auf den Start" } },
          paneFailed: { label: { normal: "{{label}} konnte nicht geladen werden.", beginner: "{{label}} konnte nicht geladen werden." } },
          grid: { label: { normal: "Alle semio Apps", beginner: "Alle semio Apps" } },
        },
      },
    },
  },
});
//#endregion 🌐️PlayLandingLabels

//#region 🎡️PlayGridGeometry
/** @emoji 🔢️ Columns and rows of the play grid; the strip spans `columns * 100vw` by `rows * 100vh`. */
const { columns: PLAY_GRID_COLUMNS, rows: PLAY_GRID_ROWS } = playGridDimensions(PLAY_PANES.length);

/** @emoji 📍️ Every pane's cell, row-major with the short trailing row centred — the single source both the
 * pane strip, the card overlay and every scroll offset read, so a centred pane still pins under its card. */
const PLAY_GRID_CELLS: readonly { readonly column: number; readonly row: number }[] = PLAY_PANES.map((_, paneIndex) => playPaneGridCell(paneIndex, PLAY_PANES.length));

function paneColumn(paneIndex: number): number {
  return PLAY_GRID_CELLS[paneIndex]?.column ?? 0;
}

function paneRow(paneIndex: number): number {
  return PLAY_GRID_CELLS[paneIndex]?.row ?? 0;
}

function paneIndexById(id: string): number {
  return PLAY_PANES.findIndex((pane) => pane.id === id);
}

function paneIdFromLocationHash(): string | null {
  const raw = window.location.hash.replace(/^#/, "").trim();
  if (!raw) return null;
  return PLAY_PANES.some((pane) => pane.id === raw) ? raw : null;
}

/** @emoji 🧭️ Horizontal (vw) and vertical (vh) scroll offset into the grid. */
type ScrollOffset = { readonly x: number; readonly y: number };

/** @emoji 🧭️ Largest scroll offset that still keeps the last column and row flush with the viewport edge. */
const PLAY_MAX_SCROLL: ScrollOffset = { x: (PLAY_GRID_COLUMNS - 1) * 100, y: (PLAY_GRID_ROWS - 1) * 100 };

/** @emoji 🧭️ Keeps a free pan on occupied ground ({@link playOccupiedColumnRange}): the flanks beside a
 * short trailing row are never a viewport of their own, so the overview has no reachable empty cell. */
function clampScrollOffset(offset: ScrollOffset): ScrollOffset {
  const y = Math.min(PLAY_MAX_SCROLL.y, Math.max(0, offset.y));
  const { first, last } = playOccupiedColumnRange(y / 100, PLAY_PANES.length);
  return { x: Math.min(last * 100, Math.max(first * 100, offset.x)), y };
}

/** @emoji 🎞 Programmatic pane pin / focus glide duration — one rAF timeline owns the transform. */
const PLAY_SCROLL_GLIDE_MS = 500;

/** @emoji 🎞 Exponential follow factor while the cursor freely pans the overview. */
const PLAY_SCROLL_FOLLOW_LERP = 0.12;

/** @emoji 🎞 Settle epsilon (vw/vh) for the free-pan follow loop. */
const PLAY_SCROLL_FOLLOW_EPSILON = 0.01;

/** @emoji 🎞 Either free-pan follow (exponential) or a timed ease-in-out glide to a pane — mutually exclusive so the grid never fights itself. */
type ScrollDrive =
  | { readonly mode: "follow" }
  | { readonly mode: "glide"; readonly from: ScrollOffset; readonly to: ScrollOffset; readonly startedAt: number; readonly durationMs: number };

/** @emoji 🎞 Cubic ease-in-out for focus / hover pin glides. */
function easeInOutCubic(t: number): number {
  return t < 0.5 ? 4 * t * t * t : 1 - (-2 * t + 2) ** 3 / 2;
}

/** @emoji 🧭️ Scroll offset that brings the given pane fully into the viewport. */
function scrollOffsetForPaneIndex(paneIndex: number): ScrollOffset {
  return {
    x: Math.min(PLAY_MAX_SCROLL.x, Math.max(0, paneColumn(paneIndex) * 100)),
    y: Math.min(PLAY_MAX_SCROLL.y, Math.max(0, paneRow(paneIndex) * 100)),
  };
}

/** @emoji 🧭️ Linear blend of two scroll offsets. */
function lerpScrollOffset(from: ScrollOffset, to: ScrollOffset, t: number): ScrollOffset {
  return { x: from.x + (to.x - from.x) * t, y: from.y + (to.y - from.y) * t };
}

type PaneAxisBounds = { readonly start: number; readonly end: number; readonly visible: boolean };

/** @emoji 📐 Maps one axis of a grid cell into the current viewport after scrolling (percent of that axis). */
function paneAxisBounds(cellIndex: number, scrollPercent: number): PaneAxisBounds {
  const cellStart = cellIndex * 100 - scrollPercent;
  const start = Math.max(0, cellStart);
  const end = Math.min(100, cellStart + 100);
  return { start, end, visible: end > start };
}

type RectPx = { readonly top: number; readonly left: number; readonly width: number; readonly height: number };

/** @emoji 👁 Visible on-screen bounds of a grid pane — the region that stays untinted while its card is hovered. */
function playPaneRevealRect(paneIndex: number, scrollOffset: ScrollOffset): RectPx {
  const horizontal = paneAxisBounds(paneColumn(paneIndex), scrollOffset.x);
  const vertical = paneAxisBounds(paneRow(paneIndex), scrollOffset.y);
  if (!horizontal.visible || !vertical.visible) return { top: 0, left: 0, width: 0, height: 0 };
  const vw = window.innerWidth;
  const vh = window.innerHeight;
  const left = (horizontal.start / 100) * vw;
  const top = (vertical.start / 100) * vh;
  return { top, left, width: Math.max(0, (horizontal.end / 100) * vw - left), height: Math.max(0, (vertical.end / 100) * vh - top) };
}

/** @emoji 🪟️ Full-viewport veil pieces; optional rectangular cutout leaves the hovered app pane untinted. */
function playTintSegmentsPx(revealRect: RectPx | null): readonly RectPx[] {
  const vw = window.innerWidth;
  const vh = window.innerHeight;
  if (!revealRect) return [{ top: 0, left: 0, width: vw, height: vh }];
  const holeLeft = Math.max(0, revealRect.left);
  const holeTop = Math.max(0, revealRect.top);
  const holeRight = Math.min(vw, revealRect.left + revealRect.width);
  const holeBottom = Math.min(vh, revealRect.top + revealRect.height);
  if (holeRight <= holeLeft || holeBottom <= holeTop) return [{ top: 0, left: 0, width: vw, height: vh }];
  const segments: RectPx[] = [];
  if (holeTop > 0) segments.push({ top: 0, left: 0, width: vw, height: holeTop });
  if (holeBottom < vh) segments.push({ top: holeBottom, left: 0, width: vw, height: vh - holeBottom });
  if (holeLeft > 0) segments.push({ top: holeTop, left: 0, width: holeLeft, height: holeBottom - holeTop });
  if (holeRight < vw) segments.push({ top: holeTop, left: holeRight, width: vw - holeRight, height: holeBottom - holeTop });
  return segments;
}
//#endregion 🎡️PlayGridGeometry

//#region 🎡️PlayPaneLifecycle
/** @emoji 🧮️ At most this many shells stay live at once — every other booted pane is released to a poster. */
const PLAY_LIVE_PANE_BUDGET = 4;

/** @emoji ⏱️ How often idle live panes are re-checked for release. */
const PLAY_SUSPENSION_SWEEP_MS = 5_000;

/** @emoji ⏱️ A pristine pane nobody has looked at for this long is released even within budget. */
const PLAY_IDLE_SUSPEND_MS = 2 * 60_000;

/** @emoji ⏱️ Grace period before the background warm-boot queue starts, so the first paint and the
 * introduction are never fighting a wasm plugin boot for the main thread. */
const PLAY_WARM_BOOT_START_MS = 4_000;

/** @emoji ⏱️ Distance between two warm boots — one pane's 30-second plugin-load budget must be over
 * before the next shell starts competing with it. */
const PLAY_WARM_BOOT_INTERVAL_MS = 35_000;

/** @emoji 📋️ Warm-boot order IS grid order. */
const PLAY_PANE_IDS: readonly string[] = PLAY_PANES.map((pane) => pane.id);

/** @emoji 🖼️ Composites every canvas inside a pane's container into one offscreen 2D canvas and returns
 * it as a data URL — synchronously, before a `preserveDrawingBuffer: false` backbuffer is cleared. */
function capturePanePoster(container: HTMLElement): string | null {
  const canvases = container.querySelectorAll("canvas");
  if (canvases.length === 0) return null;
  const containerRect = container.getBoundingClientRect();
  if (containerRect.width <= 0 || containerRect.height <= 0) return null;
  const poster = document.createElement("canvas");
  poster.width = Math.round(containerRect.width);
  poster.height = Math.round(containerRect.height);
  const ctx = poster.getContext("2d");
  if (!ctx) return null;
  let drewSomething = false;
  canvases.forEach((canvas) => {
    if (canvas.width === 0 || canvas.height === 0) return;
    const rect = canvas.getBoundingClientRect();
    try {
      ctx.drawImage(canvas, rect.left - containerRect.left, rect.top - containerRect.top, rect.width, rect.height);
      drewSomething = true;
    } catch {
      /* tainted canvas or a lost GPU context — the placeholder fallback still works */
    }
  });
  if (!drewSomething) return null;
  try {
    return poster.toDataURL("image/png");
  } catch {
    return null;
  }
}

/** @emoji 🎡️ Boots panes on demand (hash, hover, focus, keyboard) plus a slow background warm-boot queue
 * ({@link playNextWarmBootPane}), and keeps at most {@link PLAY_LIVE_PANE_BUDGET} of them live: the least
 * recently touched PRISTINE pane is released to a poster first. A pane the user interacted with is never
 * released, because its document would be lost. A warm-booted pane carries no touch timestamp, so it sorts
 * ahead of every user-touched pane in the release order and never idles one of them out. */
function usePaneLifecycle(initialFocusId: string | null, focusedId: string | null): {
  readonly bootedIds: ReadonlySet<string>;
  readonly suspendedIds: ReadonlySet<string>;
  readonly liveCount: number;
  readonly postersById: ReadonlyMap<string, string>;
  readonly touch: (id: string) => void;
  readonly warm: (id: string) => void;
  readonly markDirty: (id: string) => void;
  readonly registerContainer: (id: string, el: HTMLDivElement | null) => void;
} {
  const [bootedIds, setBootedIds] = useState<ReadonlySet<string>>(() => new Set(initialFocusId ? [initialFocusId] : []));
  const [suspendedIds, setSuspendedIds] = useState<ReadonlySet<string>>(new Set());
  const [postersById, setPostersById] = useState<ReadonlyMap<string, string>>(new Map());
  const [dirtyIds, setDirtyIds] = useState<ReadonlySet<string>>(new Set());
  const containersRef = useRef<Map<string, HTMLDivElement>>(new Map());
  const touchedAtRef = useRef<Map<string, number>>(new Map(initialFocusId ? [[initialFocusId, Date.now()]] : []));

  const touch = useCallback((id: string) => {
    touchedAtRef.current.set(id, Date.now());
    setBootedIds((prev) => (prev.has(id) ? prev : new Set(prev).add(id)));
    setSuspendedIds((prev) => {
      if (!prev.has(id)) return prev;
      const next = new Set(prev);
      next.delete(id);
      return next;
    });
  }, []);

  /** @emoji 🐢️ Only the very first warm boot waits the short start grace; every later one waits a full interval. */
  const warmedRef = useRef(false);

  const warm = useCallback((id: string) => {
    warmedRef.current = true;
    setBootedIds((prev) => (prev.has(id) ? prev : new Set(prev).add(id)));
  }, []);

  const markDirty = useCallback((id: string) => {
    setDirtyIds((prev) => (prev.has(id) ? prev : new Set(prev).add(id)));
  }, []);

  const registerContainer = useCallback((id: string, el: HTMLDivElement | null) => {
    if (el) containersRef.current.set(id, el);
    else containersRef.current.delete(id);
  }, []);

  const suspend = useCallback((ids: readonly string[]) => {
    if (ids.length === 0) return;
    const posters = new Map<string, string>();
    for (const id of ids) {
      const container = containersRef.current.get(id);
      const poster = container ? capturePanePoster(container) : null;
      if (poster) posters.set(id, poster);
    }
    if (posters.size > 0) setPostersById((prev) => new Map([...prev, ...posters]));
    setSuspendedIds((prev) => new Set([...prev, ...ids]));
  }, []);

  const liveByRecency = useCallback(
    () => [...bootedIds].filter((id) => !suspendedIds.has(id)).sort((a, b) => (touchedAtRef.current.get(a) ?? 0) - (touchedAtRef.current.get(b) ?? 0)),
    [bootedIds, suspendedIds],
  );

  const liveCount = useMemo(() => [...bootedIds].filter((id) => !suspendedIds.has(id)).length, [bootedIds, suspendedIds]);

  useEffect(() => {
    suspend(playPanesOverBudget(liveByRecency(), dirtyIds, focusedId, PLAY_LIVE_PANE_BUDGET));
  }, [liveByRecency, dirtyIds, focusedId, suspend]);

  useEffect(() => {
    const step = playNextWarmBootPane(PLAY_PANE_IDS, bootedIds, liveCount, focusedId, PLAY_LIVE_PANE_BUDGET);
    if (step.kind !== "boot") return;
    return schedulePlayIdle(() => warm(step.id), warmedRef.current ? PLAY_WARM_BOOT_INTERVAL_MS : PLAY_WARM_BOOT_START_MS, window);
  }, [bootedIds, liveCount, focusedId, warm]);

  useEffect(() => {
    const sweep = () => {
      const now = Date.now();
      const idle = liveByRecency().filter((id) => id !== focusedId && !dirtyIds.has(id) && now - (touchedAtRef.current.get(id) ?? now) >= PLAY_IDLE_SUSPEND_MS);
      suspend(idle);
    };
    const interval = window.setInterval(sweep, PLAY_SUSPENSION_SWEEP_MS);
    return () => window.clearInterval(interval);
  }, [liveByRecency, dirtyIds, focusedId, suspend]);

  return { bootedIds, suspendedIds, liveCount, postersById, touch, warm, markDirty, registerContainer };
}
//#endregion 🎡️PlayPaneLifecycle

//#region 🛟️PaneErrorBoundary
/** @emoji 🛟️ One pane crashing must never take down the others or the landing chrome around them. */
type PaneErrorBoundaryProps = { readonly paneLabel: string; readonly failedLabel: string; readonly children: UiNode };
type PaneErrorBoundaryState = { readonly error: Error | null };

const PaneErrorBoundary = createUiErrorBoundary<PaneErrorBoundaryProps, PaneErrorBoundaryState>({
  initialState: { error: null },
  deriveState: (error) => ({ error }),
  didCatch: (props, error) => console.error(`Play pane "${props.paneLabel}" crashed`, error),
  render: (props, state) => {
    if (state.error) {
      return (
        <div data-play-pane-error="" role="alert" className="flex h-full w-full items-center justify-center bg-background p-double text-center text-sm text-muted-foreground">
          {props.failedLabel}
        </div>
      );
    }
    return props.children;
  },
});
//#endregion 🛟️PaneErrorBoundary

//#region 🎡️PlayPane
/** @emoji 🎡️ One grid cell: the logo placeholder (not booted), the live shell, or its poster once released —
 * `inert` while not focused so it never steals pointer, keyboard or focus from the overview. */
function PlayPane({
  pane,
  booted,
  focused,
  suspended,
  posterDataUrl,
  style,
  onDirty,
  onContainerElement,
}: {
  readonly pane: PlayPaneSpec;
  readonly booted: boolean;
  readonly focused: boolean;
  readonly suspended: boolean;
  readonly posterDataUrl: string | null;
  /** @emoji 📍️ Explicit grid placement — the trailing row is centred, so cells are never auto-flowed. */
  readonly style?: { readonly gridColumn: number; readonly gridRow: number };
  readonly onDirty: () => void;
  readonly onContainerElement: (id: string, el: HTMLDivElement | null) => void;
}) {
  const boot = useMemo(() => resolvePlaygroundBoot(PLUGIN_CATALOG, pane.variant), [pane.variant]);
  const locks = useMemo(() => resolveShellLocks(pane.brand.locks), [pane.brand]);
  const defaults = useMemo(() => resolveShellDefaults(pane.brand, undefined), [pane.brand]);
  const waitingLabel = useLabel(playLandingUiLabel("play.landing.paneWaiting"), { label: pane.label });
  const failedLabel = useLabel(playLandingUiLabel("play.landing.paneFailed"), { label: pane.label });
  const live = booted && !suspended;

  return (
    <div
      ref={(el) => onContainerElement(pane.id, el)}
      data-play-pane={pane.id}
      className="relative h-full w-full overflow-hidden bg-background"
      style={style}
      inert={!focused}
      onPointerDownCapture={onDirty}
      onKeyDownCapture={onDirty}
    >
      {live ? (
        <PaneErrorBoundary paneLabel={pane.label} failedLabel={failedLabel}>
          <FrameworkOsShell
            pluginFilter={pane.variant}
            plugins={boot.plugins}
            surfaceSessionFactories={PUZZLE_BOARD_SESSION_FACTORIES}
            appId={boot.defaultAppId}
            locks={locks}
            defaults={defaults}
            brand={pane.brand}
            shellId={pane.id}
            storageNamespace={pane.id}
            suppressAutoIntroduction={!focused}
          />
        </PaneErrorBoundary>
      ) : booted && suspended && posterDataUrl ? (
        <img src={posterDataUrl} alt="" className="h-full w-full object-cover" aria-hidden />
      ) : (
        <div className={cn("flex h-full w-full flex-col items-center justify-center gap-double bg-background", loadingBorderClass)} role="status" aria-busy={booted} aria-label={waitingLabel}>
          <Icon icon={pane.icon} size="large" className="text-foreground opacity-40" title={uiDataLabel(pane.label)} />
          <div className="h-full min-h-0 w-full max-w-4xl flex-1 p-double">
            <CanvasSkeleton label={waitingLabel} />
          </div>
        </div>
      )}
    </div>
  );
}
//#endregion 🎡️PlayPane

//#region 🎡️PlayLanding
function PlayLanding() {
  const viewportMobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const touchListMode = useMediaQuery(PLAY_TOUCH_LIST_MEDIA_QUERY);

  const surfaceChrome = useMemo(() => {
    const device: "mobile" | "tablet" | "desktop" = viewportMobile ? "mobile" : readStoredUiChromeLayout(playStorage) === "tablet" ? "tablet" : "desktop";
    return { appearance: readStoredUiChromeAppearance(playStorage), device, driver: readStoredUiDriver(playStorage) };
  }, [viewportMobile]);
  useElementsSurfaceChrome(surfaceChrome);

  const initialFocusId = useMemo(() => paneIdFromLocationHash(), []);
  const [introductionStep, setIntroductionStep] = useState(0);
  const [showIntroduction, setShowIntroduction] = useState(!initialFocusId);
  const [focusedId, setFocusedId] = useState<string | null>(initialFocusId);
  const { bootedIds, suspendedIds, liveCount, postersById, touch, warm, markDirty, registerContainer } = usePaneLifecycle(initialFocusId, focusedId);
  const appCountLabel = useLabel(playLandingUiLabel("play.landing.appCount"), { apps: PLAY_PANES.length });
  const overviewLabel = useLabel(playLandingUiLabel("play.landing.overview"));
  const gridLabel = useLabel(playLandingUiLabel("play.landing.grid"));
  const [hoveredPaneId, setHoveredPaneId] = useState<string | null>(null);
  const [revealRect, setRevealRect] = useState<RectPx | null>(null);
  const hoveredPaneIdRef = useRef<string | null>(null);
  const scrollTargetRef = useRef<ScrollOffset>(initialFocusId ? scrollOffsetForPaneIndex(paneIndexById(initialFocusId)) : { x: 0, y: 0 });
  const scrollCurrentRef = useRef<ScrollOffset>(scrollTargetRef.current);
  const scrollDriveRef = useRef<ScrollDrive>({ mode: "follow" });
  /** @emoji 🎞 Bumped on every drive change so a stale follow `setScrollOffset` cannot paint after a glide has taken ownership. */
  const scrollEpochRef = useRef(0);
  const [scrollOffset, setScrollOffset] = useState<ScrollOffset>(scrollTargetRef.current);
  const listScrollRef = useRef<HTMLDivElement | null>(null);
  const listProgressRafRef = useRef<number | null>(null);
  const [listProgress, setListProgress] = useState(0);
  const [listScrollLocked, setListScrollLocked] = useState(Boolean(initialFocusId));
  const scrollLoopRunningRef = useRef(false);
  const ensureScrollLoopRef = useRef<() => void>(() => {});

  const commitScrollOffset = useCallback((epoch: number, next: ScrollOffset) => {
    scrollCurrentRef.current = next;
    setScrollOffset((prev) => {
      if (scrollEpochRef.current !== epoch) return prev;
      return prev.x === next.x && prev.y === next.y ? prev : next;
    });
  }, []);

  const applyPaneScroll = useCallback(
    (paneIndex: number) => {
      const offset = scrollOffsetForPaneIndex(paneIndex);
      const from = scrollCurrentRef.current;
      scrollTargetRef.current = offset;
      scrollEpochRef.current += 1;
      const epoch = scrollEpochRef.current;
      if (Math.abs(offset.x - from.x) < PLAY_SCROLL_FOLLOW_EPSILON && Math.abs(offset.y - from.y) < PLAY_SCROLL_FOLLOW_EPSILON) {
        scrollDriveRef.current = { mode: "follow" };
        commitScrollOffset(epoch, offset);
        return;
      }
      scrollDriveRef.current = { mode: "glide", from: { x: from.x, y: from.y }, to: offset, startedAt: performance.now(), durationMs: PLAY_SCROLL_GLIDE_MS };
      commitScrollOffset(epoch, from);
      ensureScrollLoopRef.current();
    },
    [commitScrollOffset],
  );

  const scrollListToPaneIndex = useCallback((paneIndex: number) => {
    const el = listScrollRef.current;
    if (!el) return;
    const height = el.clientHeight;
    if (height <= 0) return;
    el.scrollTop = paneIndex * height;
    setListProgress(paneIndex);
  }, []);

  const focusPane = useCallback(
    (id: string) => {
      touch(id);
      setFocusedId(id);
      setShowIntroduction(false);
      hoveredPaneIdRef.current = null;
      setHoveredPaneId(null);
      setRevealRect(null);
      const paneIndex = paneIndexById(id);
      if (touchListMode) {
        scrollListToPaneIndex(paneIndex);
        setListScrollLocked(true);
      } else {
        applyPaneScroll(paneIndex);
      }
      window.history.replaceState(null, "", `#${id}`);
    },
    [touch, applyPaneScroll, touchListMode, scrollListToPaneIndex],
  );

  const returnToOverview = useCallback(() => {
    const previousFocus = focusedId;
    setFocusedId(null);
    if (touchListMode) {
      setListScrollLocked(false);
      if (previousFocus) {
        const paneIndex = paneIndexById(previousFocus);
        if (paneIndex >= 0) requestAnimationFrame(() => scrollListToPaneIndex(paneIndex));
      }
    }
    window.history.replaceState(null, "", window.location.pathname + window.location.search);
  }, [touchListMode, focusedId, scrollListToPaneIndex]);

  useEffect(() => {
    const onHashChange = () => {
      const paneId = paneIdFromLocationHash();
      if (paneId) focusPane(paneId);
      else returnToOverview();
    };
    window.addEventListener("hashchange", onHashChange);
    return () => window.removeEventListener("hashchange", onHashChange);
  }, [focusPane, returnToOverview]);

  useEffect(() => {
    if (!focusedId) return;
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") returnToOverview();
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [focusedId, returnToOverview]);

  useEffect(() => {
    if (!touchListMode || !initialFocusId) return;
    const paneIndex = paneIndexById(initialFocusId);
    if (paneIndex < 0) return;
    requestAnimationFrame(() => scrollListToPaneIndex(paneIndex));
  }, [touchListMode, initialFocusId, scrollListToPaneIndex]);

  const handleListScroll = useCallback(() => {
    const el = listScrollRef.current;
    if (!el || listScrollLocked) return;
    if (listProgressRafRef.current != null) return;
    listProgressRafRef.current = requestAnimationFrame(() => {
      listProgressRafRef.current = null;
      const height = el.clientHeight;
      if (height > 0) setListProgress(el.scrollTop / height);
    });
  }, [listScrollLocked]);

  // 📱️ Snap-list scrolling warms the pane the user is about to land on as well as the one under the thumb,
  // but only while a slot is free — the preload must never cost a pane the user already touched.
  useEffect(() => {
    if (!touchListMode || focusedId) return;
    const current = Math.round(listProgress);
    const pane = PLAY_PANES[current];
    if (pane) touch(pane.id);
    const next = PLAY_PANES[current + 1];
    if (next && liveCount < PLAY_LIVE_PANE_BUDGET) warm(next.id);
  }, [touchListMode, listProgress, focusedId, touch, warm, liveCount]);

  const refreshRevealRect = useCallback((paneId: string | null, offset: ScrollOffset) => {
    const paneIndex = paneId ? paneIndexById(paneId) : -1;
    setRevealRect(paneIndex < 0 ? null : playPaneRevealRect(paneIndex, offset));
  }, []);

  const tintSegments = useMemo(() => playTintSegmentsPx(revealRect), [revealRect]);

  useEffect(() => {
    if (touchListMode) return;
    const onResize = () => {
      if (hoveredPaneId) refreshRevealRect(hoveredPaneId, scrollCurrentRef.current);
      else setRevealRect(null);
    };
    window.addEventListener("resize", onResize);
    return () => window.removeEventListener("resize", onResize);
  }, [touchListMode, hoveredPaneId, refreshRevealRect]);

  // 🖱️ Mouse-follow panning only makes sense in overview — a focused pane owns the mouse.
  useEffect(() => {
    if (touchListMode || focusedId) return;
    const onMove = (event: MouseEvent) => {
      if (hoveredPaneIdRef.current) return;
      if (scrollDriveRef.current.mode !== "follow") scrollEpochRef.current += 1;
      scrollDriveRef.current = { mode: "follow" };
      scrollTargetRef.current = clampScrollOffset({ x: (event.clientX / window.innerWidth) * PLAY_MAX_SCROLL.x, y: (event.clientY / window.innerHeight) * PLAY_MAX_SCROLL.y });
      ensureScrollLoopRef.current();
    };
    window.addEventListener("mousemove", onMove, { passive: true });
    return () => window.removeEventListener("mousemove", onMove);
  }, [touchListMode, focusedId]);

  useEffect(() => {
    if (touchListMode) return;
    let frame = 0;
    const tick = () => {
      const now = performance.now();
      const epoch = scrollEpochRef.current;
      const drive = scrollDriveRef.current;
      if (drive.mode === "glide") {
        const t = Math.min(1, Math.max(0, (now - drive.startedAt) / drive.durationMs));
        const next = lerpScrollOffset(drive.from, drive.to, easeInOutCubic(t));
        commitScrollOffset(epoch, next);
        if (hoveredPaneIdRef.current) refreshRevealRect(hoveredPaneIdRef.current, next);
        if (t >= 1) {
          scrollTargetRef.current = drive.to;
          scrollDriveRef.current = { mode: "follow" };
          scrollLoopRunningRef.current = false;
          return;
        }
        frame = requestAnimationFrame(tick);
        return;
      }
      const current = scrollCurrentRef.current;
      const target = scrollTargetRef.current;
      if (Math.abs(target.x - current.x) < PLAY_SCROLL_FOLLOW_EPSILON && Math.abs(target.y - current.y) < PLAY_SCROLL_FOLLOW_EPSILON) {
        if (current.x !== target.x || current.y !== target.y) {
          commitScrollOffset(epoch, target);
          if (hoveredPaneIdRef.current) refreshRevealRect(hoveredPaneIdRef.current, target);
        }
        scrollLoopRunningRef.current = false;
        return;
      }
      const next = lerpScrollOffset(current, target, PLAY_SCROLL_FOLLOW_LERP);
      if (scrollDriveRef.current.mode === "glide" || scrollEpochRef.current !== epoch) {
        frame = requestAnimationFrame(tick);
        return;
      }
      commitScrollOffset(epoch, next);
      if (hoveredPaneIdRef.current) refreshRevealRect(hoveredPaneIdRef.current, next);
      frame = requestAnimationFrame(tick);
    };
    ensureScrollLoopRef.current = () => {
      if (scrollLoopRunningRef.current) return;
      scrollLoopRunningRef.current = true;
      frame = requestAnimationFrame(tick);
    };
    ensureScrollLoopRef.current();
    return () => {
      scrollLoopRunningRef.current = false;
      cancelAnimationFrame(frame);
    };
  }, [touchListMode, refreshRevealRect, commitScrollOffset]);

  const revealPane = useCallback(
    (pane: PlayPaneSpec, paneIndex: number) => {
      hoveredPaneIdRef.current = pane.id;
      setHoveredPaneId(pane.id);
      touch(pane.id);
      applyPaneScroll(paneIndex);
      refreshRevealRect(pane.id, scrollOffsetForPaneIndex(paneIndex));
    },
    [touch, applyPaneScroll, refreshRevealRect],
  );

  const concealPane = useCallback((pane: PlayPaneSpec) => {
    if (hoveredPaneIdRef.current !== pane.id) return;
    hoveredPaneIdRef.current = null;
    setHoveredPaneId(null);
    setRevealRect(null);
  }, []);

  const overviewChrome = (
    <>
      {showIntroduction && (
        <UIIntroduction introduction={SEMIO_TECH_PLAY_INTRODUCTION} stepIndex={introductionStep} completedInteractionIndices={[]} onStepIndexChange={setIntroductionStep} onDismiss={() => setShowIntroduction(false)} />
      )}

      {!hoveredPaneId && (
        <Navbar
          items={[
            {
              key: "logoAndTitle",
              centered: true,
              content: (
                <div className="flex min-w-0 shrink-0 items-center gap-single">
                  <ShellBrandLogo svg={SEMIO_TECH_PLAY_LOGO_SVG} className="size-workbench shrink-0" />
                  <span data-slot="app-name" className="px-single text-sm font-semibold text-foreground">
                    semio Play
                  </span>
                  <span data-slot="play-app-count" className="text-xs text-muted-foreground">
                    {appCountLabel}
                  </span>
                </div>
              ),
            },
          ]}
          showFullscreenToggle={false}
          className="pointer-events-none absolute inset-x-0 top-0 z-40 bg-transparent"
        />
      )}
    </>
  );

  const overviewReturnButton = focusedId ? (
    <button
      type="button"
      onClick={returnToOverview}
      data-play-overview-button=""
      aria-label={overviewLabel}
      className="ui-glass absolute right-double top-double z-40 inline-flex items-center gap-single rounded-md border border-border-normal px-single py-half text-sm font-medium text-foreground shadow-md outline-none transition-colors hover:border-border-emphasized focus-visible:ring-2 focus-visible:ring-ring"
    >
      <Icon icon="layout-grid" size="small" />
      {overviewLabel}
    </button>
  ) : null;

  if (touchListMode) {
    return (
      <div className="relative h-full w-full overflow-hidden bg-background text-foreground">
        <div
          ref={listScrollRef}
          data-play-list-scroll=""
          aria-label={gridLabel}
          onScroll={handleListScroll}
          className={cn("flex w-full flex-col overscroll-y-contain", listScrollLocked ? "overflow-hidden" : "snap-y snap-mandatory overflow-y-auto")}
          style={{ height: "100dvh" }}
        >
          {PLAY_PANES.map((pane) => (
            <section key={pane.id} className="relative w-full shrink-0 snap-start overflow-hidden" style={{ height: "100dvh", minHeight: "100dvh" }}>
              <PlayPane
                pane={pane}
                booted={bootedIds.has(pane.id)}
                focused={focusedId === pane.id}
                suspended={suspendedIds.has(pane.id)}
                posterDataUrl={postersById.get(pane.id) ?? null}
                onDirty={() => markDirty(pane.id)}
                onContainerElement={registerContainer}
              />
              {!focusedId && (
                <>
                  <div className="pointer-events-none absolute inset-0 z-30">
                    <div className="ui-veil absolute inset-0" />
                  </div>
                  <div className="pointer-events-none absolute inset-0 z-[31] flex items-center justify-center px-double pb-[5.5rem]">
                    <PlayCard pane={pane} onClick={() => focusPane(pane.id)} />
                  </div>
                </>
              )}
            </section>
          ))}
        </div>

        {!focusedId && overviewChrome}

        {overviewReturnButton}
      </div>
    );
  }

  return (
    <div className="relative h-full w-full overflow-hidden bg-background text-foreground">
      <div
        className="grid"
        style={{
          gridTemplateColumns: `repeat(${PLAY_GRID_COLUMNS}, 100vw)`,
          gridTemplateRows: `repeat(${PLAY_GRID_ROWS}, 100vh)`,
          width: `${PLAY_GRID_COLUMNS * 100}vw`,
          height: `${PLAY_GRID_ROWS * 100}vh`,
          transform: `translate(-${scrollOffset.x}vw, -${scrollOffset.y}vh)`,
        }}
      >
        {PLAY_PANES.map((pane, paneIndex) => (
          <PlayPane
            key={pane.id}
            pane={pane}
            booted={bootedIds.has(pane.id)}
            focused={focusedId === pane.id}
            suspended={suspendedIds.has(pane.id)}
            posterDataUrl={postersById.get(pane.id) ?? null}
            style={{ gridColumn: paneColumn(paneIndex) + 1, gridRow: paneRow(paneIndex) + 1 }}
            onDirty={() => markDirty(pane.id)}
            onContainerElement={registerContainer}
          />
        ))}
      </div>

      {!focusedId && (
        <>
          <div className="pointer-events-none absolute inset-0 z-30">
            {tintSegments.map((segment, index) => (
              <div key={`tint-${index}-${segment.top}-${segment.left}`} className="ui-veil absolute" style={{ top: segment.top, left: segment.left, width: segment.width, height: segment.height }} />
            ))}
          </div>

          <div
            data-play-overview=""
            role="navigation"
            aria-label={gridLabel}
            className="pointer-events-none absolute inset-0 z-[31] grid items-center pb-double pt-[calc(var(--size-workbench)*1.5)]"
            style={{ gridTemplateColumns: `repeat(${PLAY_GRID_COLUMNS}, minmax(0, 1fr))`, gridTemplateRows: `repeat(${PLAY_GRID_ROWS}, minmax(0, 1fr))` }}
          >
            {PLAY_PANES.map((pane, paneIndex) => (
              <div key={pane.id} className="flex min-w-0 justify-center px-single" style={{ gridColumn: paneColumn(paneIndex) + 1, gridRow: paneRow(paneIndex) + 1 }}>
                <PlayCard
                  pane={pane}
                  lifted={hoveredPaneId === pane.id}
                  onClick={() => focusPane(pane.id)}
                  onMouseEnter={() => revealPane(pane, paneIndex)}
                  onMouseLeave={() => concealPane(pane)}
                  onFocus={() => revealPane(pane, paneIndex)}
                />
              </div>
            ))}
          </div>

          {overviewChrome}
        </>
      )}

      {overviewReturnButton}
    </div>
  );
}
//#endregion 🎡️PlayLanding

mountUiRoot(document.getElementById("root")!, <PlayLanding />);
