// #region 🧲️Header
/** 🎡️ semio-tech play landing — introduction and every app as a live page of one {@link LayeredOverview}: the apps on a strip behind the
 * glass, one card per app above it. */
// #endregion 🧲️Header

import { mountUiRoot, useUiCallback as useCallback, useUiMemo as useMemo, useUiState as useState } from "@semio-tech/ui-react/runtime";
import {
  LayeredOverview,
  Navbar,
  ShellBrandLogo,
  UIIntroduction,
  bootstrapElementsSurfaceChromeDocument,
  capturePosterFromCanvases,
  centeredLastRowCells,
  initUiLocaleSync,
  nearSquareGrid,
  readStoredUiChromeAppearance,
  readStoredUiChromeLayout,
  readStoredUiDriver,
  registerUiTranslationBundles,
  UI_MOBILE_MEDIA_QUERY,
  useElementsSurfaceChrome,
  useLabel,
  useLabelFormatter,
  useMediaQuery,
  type LayeredCardState,
  type LayeredChromeState,
  type LayeredDirection,
  type LayeredPane,
} from "@semio-tech/ui-react";
import { createBrowserStoragePort, resolvePlaygroundBoot } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "@semio-tech/plugin-registry/catalog";
import { FrameworkOsShell, resolveShellLocks, resolveShellDefaults } from "@semio-tech/framework-renderer-react";
import { PUZZLE_BOARD_SESSION_FACTORIES } from "@semio-tech/puzzle-2d";
import { PlayCard } from "./⚛️play-card.tsx";
import { PLAY_LOCALE, PLAY_PANES, SEMIO_TECH_PLAY_INTRODUCTION, SEMIO_TECH_PLAY_LOGO_SVG, type PlayPaneSpec } from "./🪧️brand.ts";
import "./🎨️globals.css";

// 🎡️ Page-owning (single React root, no `ShellScope` of its own) — plain browser storage is correct;
// each pane's own `FrameworkOsShell` gets its own `ShellScope` (ephemeral brands → in-memory storage).
const playStorage = createBrowserStoragePort();

bootstrapElementsSurfaceChromeDocument(readStoredUiChromeAppearance(playStorage));
initUiLocaleSync(PLAY_LOCALE);

/** 📱️ Touch phones swipe through the same grid one app at a time instead of seeing every card at once. */
const PLAY_SWIPE_MEDIA_QUERY = `${UI_MOBILE_MEDIA_QUERY} and (hover: none) and (pointer: coarse)`;

//#region 🌐️PlayLandingLabels
/** 🌐️ The landing's own chrome strings. Play locks its shells to {@link PLAY_LOCALE}, but chrome
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
          neighbourUp: { label: { normal: "Go up to {{label}}", beginner: "Swipe down to reach {{label}} above" } },
          neighbourLeft: { label: { normal: "Go left to {{label}}", beginner: "Swipe right to reach {{label}} on the left" } },
          neighbourRight: { label: { normal: "Go right to {{label}}", beginner: "Swipe left to reach {{label}} on the right" } },
          neighbourDown: { label: { normal: "Go down to {{label}}", beginner: "Swipe up to reach {{label}} below" } },
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
          neighbourUp: { label: { normal: "Nach oben zu {{label}}", beginner: "Nach unten wischen, um {{label}} oben zu erreichen" } },
          neighbourLeft: { label: { normal: "Nach links zu {{label}}", beginner: "Nach rechts wischen, um {{label}} links zu erreichen" } },
          neighbourRight: { label: { normal: "Nach rechts zu {{label}}", beginner: "Nach links wischen, um {{label}} rechts zu erreichen" } },
          neighbourDown: { label: { normal: "Nach unten zu {{label}}", beginner: "Nach oben wischen, um {{label}} unten zu erreichen" } },
        },
      },
    },
  },
});
//#endregion 🌐️PlayLandingLabels

//#region 🎡️PlayPages
/** 📍️ Every app's cell: row-major with a short last row centred, so no empty cell trails off to one side. */
const PLAY_CELLS = Object.fromEntries(centeredLastRowCells(PLAY_PANES.length).map((cell, index) => [PLAY_PANES[index]!.id, cell]));

/** 🔢️ The card grid mirrors the strip: one card per cell. */
const PLAY_GRID = nearSquareGrid(PLAY_PANES.length);
const PLAY_OVERLAY_STYLE = { gridTemplateColumns: `repeat(${PLAY_GRID.columns}, minmax(0, 1fr))`, gridTemplateRows: `repeat(${PLAY_GRID.rows}, minmax(0, 1fr))` } as const;

/** 🧮️ Four shells live at most (the least recently touched pristine one goes to its poster first); the first warm boot waits 4 s so the
 * introduction never fights a wasm boot, every later one a 35 s plugin-load budget; a pristine shell untouched for two minutes is released. */
const PLAY_LIFECYCLE = { budget: 4, warmStartMs: 4_000, warmIntervalMs: 35_000, suspendIdleMs: 2 * 60_000 } as const;

/** 📏️ When swiping, the cards and the neighbour hints stay below the navbar, which lies over the top of the overview. */
const PLAY_INSETS = { top: "calc(var(--size-workbench) * 1.5)" } as const;

/** 🎡️ One app's live shell on its own storage namespace; its own tour only while it is opened. */
function PlayShell({ pane, opened }: { readonly pane: PlayPaneSpec; readonly opened: boolean }) {
  const boot = useMemo(() => resolvePlaygroundBoot(PLUGIN_CATALOG, pane.variant), [pane.variant]);
  const locks = useMemo(() => resolveShellLocks(pane.brand.locks), [pane.brand]);
  const defaults = useMemo(() => resolveShellDefaults(pane.brand, undefined), [pane.brand]);
  return (
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
      suppressAutoIntroduction={!opened}
    />
  );
}

const playPaneShortLabel = (pane: (typeof PLAY_PANES)[number]): string => pane.brand.shortWindowTitle?.replace(/^semio · /u, "") ?? pane.label;

const PLAY_PAGES: readonly LayeredPane[] = PLAY_PANES.map((pane) => ({ id: pane.id, label: pane.label, shortLabel: playPaneShortLabel(pane), icon: pane.icon, capturePoster: capturePosterFromCanvases, render: ({ opened }) => <PlayShell pane={pane} opened={opened} /> }));
const PLAY_SPECS = new Map(PLAY_PANES.map((pane) => [pane.id, pane]));

/** 🃏️ An app's card, in its own cell of the card grid (when swiping, the overview centres it on its page itself). */
function renderPlayCard(page: LayeredPane, state: LayeredCardState) {
  const card = <PlayCard pane={PLAY_SPECS.get(page.id)!} lifted={state.revealed} onClick={state.open} />;
  if (state.mode === "swipe") return card;
  const cell = PLAY_CELLS[page.id]!;
  return (
    <div className="flex min-w-0 justify-center px-single" style={{ gridColumn: cell.column + 1, gridRow: cell.row + 1 }}>
      {card}
    </div>
  );
}
//#endregion 🎡️PlayPages

//#region 🎡️PlayLanding
function PlayLanding() {
  const viewportMobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const swipe = useMediaQuery(PLAY_SWIPE_MEDIA_QUERY);
  const surfaceChrome = useMemo(() => {
    const device: "mobile" | "tablet" | "desktop" = viewportMobile ? "mobile" : readStoredUiChromeLayout(playStorage) === "tablet" ? "tablet" : "desktop";
    return { appearance: readStoredUiChromeAppearance(playStorage), device, driver: readStoredUiDriver(playStorage) };
  }, [viewportMobile]);
  useElementsSurfaceChrome(surfaceChrome);

  const [introductionStep, setIntroductionStep] = useState(0);
  const [introductionDismissed, setIntroductionDismissed] = useState(false);
  const appCountLabel = useLabel(playLandingUiLabel("play.landing.appCount"), { apps: PLAY_PANES.length });
  const overviewLabel = useLabel(playLandingUiLabel("play.landing.overview"));
  const gridLabel = useLabel(playLandingUiLabel("play.landing.grid"));
  const waitingLabel = useLabelFormatter(playLandingUiLabel("play.landing.paneWaiting"));
  const failedLabel = useLabelFormatter(playLandingUiLabel("play.landing.paneFailed"));
  const up = useLabelFormatter(playLandingUiLabel("play.landing.neighbourUp"));
  const left = useLabelFormatter(playLandingUiLabel("play.landing.neighbourLeft"));
  const right = useLabelFormatter(playLandingUiLabel("play.landing.neighbourRight"));
  const down = useLabelFormatter(playLandingUiLabel("play.landing.neighbourDown"));
  const labels = useMemo(
    () => ({
      grid: gridLabel,
      overview: overviewLabel,
      waiting: (page: LayeredPane) => waitingLabel({ label: page.label }),
      failed: (page: LayeredPane) => failedLabel({ label: page.label }),
      neighbour: (page: LayeredPane, direction: LayeredDirection) => ({ up, left, right, down })[direction]({ label: page.label }),
    }),
    [gridLabel, overviewLabel, waitingLabel, failedLabel, up, left, right, down],
  );
  const dismissIntroduction = useCallback((id: string | null) => id !== null && setIntroductionDismissed(true), []);

  const renderChrome = ({ revealed, opened }: LayeredChromeState) =>
    opened ? null : (
      <>
        {introductionDismissed ? null : (
          <UIIntroduction introduction={SEMIO_TECH_PLAY_INTRODUCTION} stepIndex={introductionStep} completedInteractionIndices={[]} onStepIndexChange={setIntroductionStep} onDismiss={() => setIntroductionDismissed(true)} />
        )}
        {revealed ? null : (
          <div className="pointer-events-none absolute inset-x-0 top-0 z-40">
            <Navbar
              label="semio Play"
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
              className="bg-transparent"
            />
          </div>
        )}
      </>
    );

  return (
    <LayeredOverview
      panes={PLAY_PAGES}
      cells={PLAY_CELLS}
      renderCard={renderPlayCard}
      overlayClassName="grid items-center pb-double pt-[calc(var(--size-workbench)*1.5)]"
      overlayStyle={PLAY_OVERLAY_STYLE}
      renderChrome={renderChrome}
      mode={swipe ? "swipe" : "strip"}
      insets={PLAY_INSETS}
      lifecycle={PLAY_LIFECYCLE}
      labels={labels}
      onOpenedIdChange={dismissIntroduction}
    />
  );
}
//#endregion 🎡️PlayLanding

mountUiRoot(document.getElementById("root")!, <PlayLanding />);
