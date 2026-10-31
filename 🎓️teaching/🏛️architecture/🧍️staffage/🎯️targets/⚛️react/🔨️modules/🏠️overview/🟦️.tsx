/** 🏠️ The editorial overview: the hero with a live frieze at true relative scale, a card per category with a live preview of its figures, and the collections, comparison and playground cards, all on one layered grid.
 *
 * The root grid is a {@link Level}: the hero card opens the whole archive, a category card opens that category's own grid of asset cards, and each deeper page opens while the one before stays visible and live behind the glass.
 *
 * @see ../🥞️levels/🟦️.tsx — the layered grid
 * @see ../📍️placed/🟦️.tsx — figures on a ground line
 */
import { useEffect, useId, useMemo, type ReactElement, type ReactNode } from "react";
import { CATEGORIES, CSS_PX_PER_MM } from "@teaching/staffage";
import type { Asset, CategoryId } from "@teaching/staffage";
import { APP_CONFIG } from "../⚙️config/🟦️.ts";
import { useArchive, useLocal, useRows } from "../🧭️context/🟦️.tsx";
import { ArchiveCard, CardButton, Icon, Notice, Tag, cn, type IconName } from "../🪟️chrome/🟦️.tsx";
import { Level, PointerProvider, tracks, useBreakpoint, useSize, type LevelPage, type PaneView } from "../🥞️levels/🟦️.tsx";
import { RowPreview, surfaceView, useDarkSurface } from "../🖼️previews/🟦️.tsx";
import { PlacedFigure } from "../📍️placed/🟦️.tsx";
import { defaultQuery, evaluateState } from "../🧮️pipeline/🟦️.ts";
import { formatAddress, openedChain, parentRoute, useAddress, type Route } from "../🗺️routing/🟦️.ts";
import { completionCounts, type Row } from "../🔎️search/🟦️.ts";
import { BrowsePage, CategoryPage } from "../🗃️browse/🟦️.tsx";
import { CollectionsPage } from "../⭐️collections/🟦️.tsx";
import { ComparePage } from "../⚖️compare/🟦️.tsx";
import { PlaygroundPage } from "../🧪️playground/🟦️.tsx";

/** 🗺️ The pages of the root grid, in cell order. */
export const ROOT_PAGES = ["all", ...CATEGORIES, "collections", "compare", "playground"] as const;

/** 🗺️ One of {@link ROOT_PAGES}. */
export type RootPage = (typeof ROOT_PAGES)[number];

const isCategory = (id: string): id is CategoryId => (CATEGORIES as readonly string[]).includes(id);

/** 🧭️ The route a root page opens. */
export function routeOfRootPage(id: string): Route {
  if (isCategory(id)) return { page: "category", category: id };
  if (id === "collections") return { page: "collections", collection: null };
  if (id === "compare" || id === "playground" || id === "all") return { page: id };
  return { page: "overview" };
}

/** 📏 The drawn width at 1:100, or the nominal width when that representation cannot be drawn. */
function drawnSpan(asset: Asset): { minX: number; maxX: number } {
  try {
    return evaluateState(asset, defaultQuery(asset, null), { kind: "scale", scale: { denominator: 100 } }).bounds;
  } catch {
    const half = Math.max(asset.dimensions.widthM, 0.1) / 2;
    return { minX: -half, maxX: half };
  }
}

const GAP_M = 0.7;
const LANE_CYCLES = 2;

type Item = { readonly row: Row; readonly x: number; readonly lane: number | null; readonly width: number };

/** 🧍️ A frieze: figures side by side on one ground line at one scale, walkers travelling along their lane; metre ticks name the scale. */
export function Frieze(props: { readonly rows: readonly Row[]; readonly label: string; readonly maxPxPerM?: number; readonly ticks?: boolean; readonly travel?: boolean; readonly className?: string }): ReactElement {
  const { rows, label, maxPxPerM = 160, ticks = true, travel = false, className } = props;
  const { language, ambient, t } = useArchive();
  const dark = useDarkSurface();
  const [attach, size] = useSize({ width: 0, height: 0 });
  const items = useMemo(() => {
    let cursor = 0;
    return rows.flatMap((row): Item[] => {
      const asset = row.asset;
      if (asset === undefined) return [];
      const clip = asset.clips.find((candidate) => candidate.id === asset.presentation.defaultClip);
      const walks = travel && clip?.rootMotion === "plant" && clip.advanceM !== undefined;
      const { minX, maxX } = drawnSpan(asset);
      const width = Math.max(maxX - minX, 0.1);
      const lane = walks ? Math.max(clip!.advanceM! * LANE_CYCLES, width) : null;
      const item = { row, x: cursor + (lane === null ? width / 2 - (minX + maxX) / 2 : -minX), lane, width };
      cursor += (lane ?? width) + GAP_M;
      return [item];
    });
  }, [rows, travel]);
  const totalM = items.reduce((end, item, index) => Math.max(end, index === items.length - 1 ? cursorEnd(items) : 0), 0.5);
  const tallestM = Math.max(1, ...items.map((item) => item.row.asset!.dimensions.heightM));
  const margin = 12;
  const groundGap = ticks ? 22 : 8;
  const pxPerM = Math.max(4, Math.min((size.width - 2 * margin) / totalM, (size.height - groundGap - 12) / (tallestM * 1.08), maxPxPerM));
  const scale = 100;
  const pxPerMm = (pxPerM * scale) / 1000;
  const ground = size.height - groundGap;
  const offset = Math.max(margin, (size.width - totalM * pxPerM) / 2);
  const metres = Math.floor(totalM);
  return (
    <div ref={attach} role="group" aria-label={label} data-frieze="" className={cn("relative min-h-24 w-full flex-1 overflow-hidden", className)}>
      {size.width === 0 ? null : items.map((item, index) => (
        <PlacedFigure key={item.row.entry.id} asset={item.row.asset!} selection={null} clock={ambient} view={surfaceView(dark)} scale={scale} language={language} pxPerMm={pxPerMm} xPx={offset + item.x * pxPerM} groundPx={ground} travel={item.lane === null ? undefined : { laneM: item.lane, phaseM: (index * 0.9) % item.lane }} />
      ))}
      <div aria-hidden="true" className="absolute inset-x-0 border-t border-foreground/50" style={{ top: ground }} />
      {ticks ? (
        <div aria-hidden="true" className="absolute inset-x-0 font-mono text-[0.65rem] text-muted-foreground" style={{ top: ground + 2 }}>
          {Array.from({ length: metres + 1 }, (_, metre) => (
            <span key={metre} className="absolute -translate-x-1/2 border-l border-foreground/50 pl-px" style={{ left: offset + metre * pxPerM, height: metre % 5 === 0 ? 8 : 4 }}>
              {metre % 5 === 0 ? <span className="relative top-2 left-1">{metre === 0 ? "0" : `${metre} m`}</span> : null}
            </span>
          ))}
        </div>
      ) : null}
      <span className="sr-only">{t("hero.friezeScale", { px: Math.round(pxPerM / (CSS_PX_PER_MM / 1)) })}</span>
    </div>
  );
}

const cursorEnd = (items: readonly Item[]): number => items.reduce((end, item) => end + (item.lane ?? item.width) + GAP_M, 0) - GAP_M;

function useCategoryRows(category: CategoryId): readonly Row[] {
  const { rows } = useRows();
  return useMemo(() => rows.filter((row) => row.entry.category === category), [rows, category]);
}

const CATEGORY_ICONS: Readonly<Record<CategoryId, IconName>> = { humans: "user", vegetation: "sparkles", animals: "hand", furniture: "layout", objects: "box", vehicles: "wrench", machinery: "cpu", components: "building" };

/** 🗂️ The card of a category: its description, how many assets it holds and a live frieze of the first of them. */
function CategoryCard(props: { readonly category: CategoryId; readonly card: { readonly revealed: boolean; readonly open: () => void }; readonly compact: boolean }): ReactElement {
  const { category, card, compact } = props;
  const { t, language, catalog } = useArchive();
  const { manifest, categories } = useRows();
  const rows = useCategoryRows(category);
  const id = useId();
  useEffect(() => {
    if (compact && !card.revealed) return;
    const controller = new AbortController();
    const index = CATEGORIES.indexOf(category);
    const timer = setTimeout(() => void catalog.loadCategory(category, { signal: controller.signal }), compact ? 1200 + index * 800 : 400 + index * 350);
    return () => {
      clearTimeout(timer);
      controller.abort();
    };
  }, [catalog, category, compact, card.revealed]);
  const entry = manifest?.categories.find((candidate) => candidate.id === category);
  const progress = categories[category];
  const ready = rows.filter((row) => row.status === "ready");
  const preview = useMemo(() => {
    const heights = ready.map((row) => row.asset!.dimensions.heightM).sort((a, b) => a - b);
    const cap = Math.max(4, 2.5 * (heights[Math.floor(heights.length / 2)] ?? 0));
    return ready.filter((row) => row.asset!.dimensions.heightM <= cap).slice(0, compact ? 2 : 4);
  }, [ready.length, rows, compact]);
  return (
    <ArchiveCard
      id={id}
      card={`category:${category}`}
      href={formatAddress({ page: "category", category })}
      onOpen={card.open}
      revealed={card.revealed}
      icon={<Icon icon={CATEGORY_ICONS[category]} size="small" className="shrink-0 text-muted-foreground" />}
      title={entry?.title[language] ?? category}
      footerLeft={<span className="px-single text-xs font-medium text-muted-foreground">{t("category.count", completionCounts(rows))}</span>}
      footerRight={<CardButton icon="chevron-right" onClick={card.open}>{t("card.explore")}</CardButton>}
      className="h-full"
    >
      {compact ? null : <p className="m-0 line-clamp-2 text-xs leading-normal text-muted-foreground">{entry?.description[language]}</p>}
      <div className="min-h-0 flex-1">
        {progress.state === "idle" ? (
          <div className="flex h-full min-h-12 items-center">
            <button type="button" onClick={() => void catalog.loadCategory(category)} className="text-xs text-primary">{t("category.load")}</button>
          </div>
        ) : progress.state === "failed" ? (
          <Notice kind="error" compact title={t("state.categoryFailed.title")}>{progress.failure?.[language]}</Notice>
        ) : preview.length === 0 ? (
          progress.state === "loading" ? <div role="status" aria-busy="true" aria-label={t("state.loading.title")} className="h-full min-h-12 animate-pulse bg-muted/60 motion-reduce:animate-none" /> : <Notice kind="unavailable" compact title={t("state.missing.title")}>{t("state.categoryEmpty.body")}</Notice>
        ) : (
          <Frieze rows={preview} label={t("category.previewLabel", { category: entry?.title[language] ?? category })} ticks={false} className="h-full" maxPxPerM={90} />
        )}
      </div>
    </ArchiveCard>
  );
}

/** 🌟️ The hero card: what the archive is, the live frieze and the way into the whole archive. */
function HeroCard(props: { readonly card: { readonly revealed: boolean; readonly open: () => void } }): ReactElement {
  const { card } = props;
  const { t, catalog } = useArchive();
  const { rows, manifest, manifestState } = useRows();
  const id = useId();
  const featured = useMemo(() => manifest?.collections.find((collection) => collection.id === APP_CONFIG.featuredCollection)?.assets ?? [], [manifest]);
  const frieze = useMemo(() => featured.flatMap((asset) => rows.filter((row) => row.entry.id === asset && row.status === "ready")), [featured, rows]);
  const pending = featured.some((asset) => rows.some((row) => row.entry.id === asset && row.status === "pending"));
  const needed = useMemo(() => [...new Set(featured.flatMap((asset) => rows.filter((row) => row.entry.id === asset && row.status === "pending").map((row) => row.entry.category)))], [featured, rows]);
  useFriezeLoad(needed.join(","), needed, catalog);
  return (
    <ArchiveCard
      id={id}
      card="hero"
      href={formatAddress({ page: "all" })}
      onOpen={card.open}
      revealed={card.revealed}
      headingLevel={2}
      icon={<Icon icon="scaling" size="small" className="shrink-0 text-muted-foreground" />}
      title={t("hero.title")}
      footerLeft={<span className="px-single text-xs font-medium text-muted-foreground">{t("hero.count", completionCounts(rows))}</span>}
      footerRight={<CardButton icon="chevron-right" onClick={card.open}>{t("hero.browse")}</CardButton>}
      className="h-full"
    >
      <p className="m-0 max-w-[75ch] text-xs leading-normal text-muted-foreground">{t("hero.lede")}</p>
      <div className="min-h-0 flex-1">
        {frieze.length === 0 ? (
          manifestState === "failed" ? <Notice kind="error" title={t("state.manifestFailed.title")}>{t("state.manifestFailed.body")}</Notice> : pending || manifestState !== "ready" ? <div role="status" aria-busy="true" aria-label={t("state.loading.title")} className="h-full min-h-24 animate-pulse bg-muted/60 motion-reduce:animate-none" /> : <Notice kind="unavailable" title={t("state.missing.title")}>{t("hero.friezeEmpty")}</Notice>
        ) : (
          <Frieze rows={frieze} label={t("hero.friezeLabel")} className="h-full" travel />
        )}
      </div>
    </ArchiveCard>
  );
}

function useFriezeLoad(key: string, needed: readonly CategoryId[], catalog: ReturnType<typeof useArchive>["catalog"]): void {
  useEffect(() => {
    if (key === "") return;
    const controller = new AbortController();
    for (const category of needed) void catalog.loadCategory(category, { signal: controller.signal });
    return () => controller.abort();
  }, [key, catalog]);
}

/** 📚️ The collections card: the curated collections and the reader's favorites with a live preview of them. */
function CollectionsCard(props: { readonly card: { readonly revealed: boolean; readonly open: () => void } }): ReactElement {
  const { card } = props;
  const { t, language } = useArchive();
  const { manifest, rows } = useRows();
  const { favorites, collections } = useLocal();
  const id = useId();
  const favorite = useMemo(() => favorites.flatMap((asset) => rows.filter((row) => row.entry.id === asset && row.status === "ready")).slice(0, 4), [favorites, rows]);
  return (
    <ArchiveCard id={id} card="collections" href={formatAddress({ page: "collections", collection: null })} onOpen={card.open} revealed={card.revealed} icon={<Icon icon="folder" size="small" className="shrink-0 text-muted-foreground" />} title={t("collections.title")} footerLeft={<span className="px-single text-xs font-medium text-muted-foreground">{t("collections.count", { count: (manifest?.collections.length ?? 0) + collections.length })}</span>} footerRight={<CardButton icon="chevron-right" onClick={card.open}>{t("card.explore")}</CardButton>} className="h-full">
      <ul className="m-0 flex list-none flex-wrap gap-single p-0">
        {(manifest?.collections ?? []).slice(0, 5).map((collection) => (
          <li key={collection.id}><Tag>{collection.title[language]}</Tag></li>
        ))}
      </ul>
      <div className="min-h-0 flex-1">
        {favorite.length > 0 ? <Frieze rows={favorite} label={t("collections.favoritesLabel")} ticks={false} className="h-full" maxPxPerM={90} /> : <p className="m-0 text-xs text-muted-foreground">{t("collections.noFavorites")}</p>}
      </div>
    </ArchiveCard>
  );
}

/** ⚖️ The comparison card: one asset side by side in two views, on one clock. */
function CompareCard(props: { readonly card: { readonly revealed: boolean; readonly open: () => void } }): ReactElement {
  const { card } = props;
  const { t } = useArchive();
  const { rows, manifest } = useRows();
  const dark = useDarkSurface();
  const id = useId();
  const featured = manifest?.collections.find((collection) => collection.id === APP_CONFIG.featuredCollection)?.assets ?? [];
  const row = featured.flatMap((asset) => rows.filter((candidate) => candidate.status === "ready" && candidate.entry.id === asset))[0] ?? rows.find((candidate) => candidate.status === "ready");
  return (
    <ArchiveCard id={id} card="compare" href={formatAddress({ page: "compare" })} onOpen={card.open} revealed={card.revealed} icon={<Icon icon="combine" size="small" className="shrink-0 text-muted-foreground" />} title={t("compare.title")} footerRight={<CardButton icon="chevron-right" onClick={card.open}>{t("card.explore")}</CardButton>} className="h-full">
      <div className="grid min-h-0 flex-1 grid-cols-2 gap-double">
        {row === undefined ? (rows.some((candidate) => candidate.status === "pending") ? <div role="status" aria-busy="true" aria-label={t("state.loading.title")} className="col-span-2 h-full min-h-12 animate-pulse bg-muted/60 motion-reduce:animate-none" /> : <p className="col-span-2 m-0 text-xs text-muted-foreground">{t("compare.lede")}</p>) : (
          <>
            <RowPreview row={row} view={surfaceView(dark)} compact {...(row.asset === undefined ? {} : { query: { kind: "pose", pose: row.asset.presentation.defaultPose } })} />
            <RowPreview row={row} view="thermal" compact {...(row.asset === undefined ? {} : { query: { kind: "pose", pose: row.asset.presentation.defaultPose } })} />
          </>
        )}
      </div>
    </ArchiveCard>
  );
}

/** 🧪️ The playground card: the reader's own composition, or the invitation to start one. */
function PlaygroundCard(props: { readonly card: { readonly revealed: boolean; readonly open: () => void } }): ReactElement {
  const { card } = props;
  const { t } = useArchive();
  const { rows } = useRows();
  const { playground } = useLocal();
  const id = useId();
  const placed = useMemo(() => playground.placements.flatMap((placement) => rows.filter((row) => row.entry.id === placement.asset && row.status === "ready")), [playground.placements, rows]);
  return (
    <ArchiveCard id={id} card="playground" href={formatAddress({ page: "playground" })} onOpen={card.open} revealed={card.revealed} icon={<Icon icon="move" size="small" className="shrink-0 text-muted-foreground" />} title={t("playground.title")} footerLeft={<span className="px-single text-xs font-medium text-muted-foreground">{t("playground.count", { count: playground.placements.length })}</span>} footerRight={<CardButton icon="chevron-right" onClick={card.open}>{t("card.explore")}</CardButton>} className="h-full">
      <div className="min-h-0 flex-1">
        {placed.length > 0 ? <Frieze rows={placed.slice(0, 5)} label={t("playground.previewLabel")} ticks={false} className="h-full" maxPxPerM={90} /> : <p className="m-0 text-xs leading-normal text-muted-foreground">{t("playground.emptyInvite")}</p>}
      </div>
    </ArchiveCard>
  );
}

const COLUMNS = { desktop: 3, tablet: 2, mobile: 1 } as const;
const ROW_MIN_PX = 150;
const ROW_WEIGHTS = { desktop: [1.5, 1, 1, 0.9], tablet: [1.4, 1, 1, 1, 1, 1], mobile: [1] } as const;

type Placement = { readonly column: number; readonly row: number; readonly span: number };

function placementOf(id: string, columns: number): Placement {
  if (id === "all") return { column: 1, row: 1, span: columns };
  const index = ROOT_PAGES.indexOf(id as RootPage) - 1;
  return { column: (index % columns) + 1, row: 2 + Math.floor(index / columns), span: 1 };
}

function RootPageContent(props: { readonly id: string; readonly view: PaneView }): ReactElement {
  const { id, view } = props;
  if (isCategory(id)) return <CategoryPage category={id} view={view} />;
  switch (id) {
    case "collections":
      return <CollectionsPage view={view} />;
    case "compare":
      return <ComparePage view={view} />;
    case "playground":
      return <PlaygroundPage view={view} />;
    default:
      return <BrowsePage view={view} />;
  }
}

/** 🌳️ The root grid: its opened page is derived from the address, and under it pages open their own nested grids. */
export function RootLevel(): ReactElement {
  const { t, language } = useArchive();
  const { manifest } = useRows();
  const { route, navigate } = useAddress();
  const breakpoint = useBreakpoint();
  const [attach, size] = useSize({ width: 1280, height: 800 });
  const chain = openedChain(route);
  const depth = route.page === "category" || route.page === "asset" ? 1 : 0;
  const columns = COLUMNS[breakpoint];
  const weights = ROW_WEIGHTS[breakpoint];
  const mode = breakpoint === "mobile" ? "swipe" : "strip";
  const compact = size.height < 760;
  const labels = useMemo(() => Object.fromEntries([["all", t("all.title")], ["collections", t("collections.title")], ["compare", t("compare.title")], ["playground", t("playground.title")], ...(manifest?.categories ?? []).map((category) => [category.id, category.title[language]])]) as Readonly<Record<string, string>>, [t, language, manifest]);
  const pages: readonly LevelPage[] = useMemo(() => ROOT_PAGES.map((id): LevelPage => ({ id, label: labels[id] ?? id, render: (view) => <RootPageContent id={id} view={view} /> })), [labels]);
  const cells = useMemo(() => Object.fromEntries(ROOT_PAGES.map((id) => { const p = placementOf(id, columns); return [id, { column: p.column - 1, row: p.row - 1 }]; })), [columns]);
  const renderCard = (page: LevelPage, card: { readonly revealed: boolean; readonly opened: boolean; readonly open: () => void }): ReactNode => {
    const content: ReactNode = isCategory(page.id) ? <CategoryCard category={page.id} card={card} compact={compact || mode === "swipe" ? compact : false} /> : page.id === "all" ? <HeroCard card={card} /> : page.id === "collections" ? <CollectionsCard card={card} /> : page.id === "compare" ? <CompareCard card={card} /> : <PlaygroundCard card={card} />;
    const place = placementOf(page.id, columns);
    return mode === "strip" ? (
      <div className="staffage-cell" style={{ gridColumn: `${place.column} / span ${place.span}`, gridRow: place.row }}>
        <div className="staffage-card">{content}</div>
      </div>
    ) : (
      <div className="staffage-entry staffage-card">{content}</div>
    );
  };
  return (
    <div ref={attach} className="relative h-full min-h-0 w-full">
      <PointerProvider>
        <Level label={t("overview.label")} pages={pages} cells={cells} openedId={chain[0] ?? null} interactive={depth === 0} mode={mode} onOpen={(id) => navigate(id === null ? { page: "overview" } : routeOfRootPage(id))} renderCard={renderCard} columns={tracks(Array.from({ length: columns }, () => 1))} rows={tracks(weights, ROW_MIN_PX)} budget={breakpoint === "mobile" ? 1 : 5} />
      </PointerProvider>
    </div>
  );
}

/** ⬆️ Where Escape and the back button lead from a route. */
export const upFrom = parentRoute;
