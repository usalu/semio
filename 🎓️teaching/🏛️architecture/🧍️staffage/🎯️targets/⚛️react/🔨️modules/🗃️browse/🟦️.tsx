/** 🗃️ Browsing the archive: asset cards, the search/filter/sort toolbar, the page of every asset and the page of one category with its own nested grid.
 *
 * Every filter lives in the address (`#/all?q=oak&cat=vegetation&sort=height`), so a result list can be shared. Filters that read asset data (tags, summaries, mass, root motion, completion) load the categories they need with progress and a cancel button;
 * titles are searched at once from the manifest.
 *
 * @see ../🔎️search/🟦️.ts — the pure search, filter and sort
 * @see ../🥞️levels/🟦️.tsx — the nested grid of a category
 */
import { useId, useMemo, useState, type ReactElement, type ReactNode } from "react";
import { CATEGORIES, COMPLETION, formatNumber } from "@teaching/staffage";
import type { CategoryId, Completion } from "@teaching/staffage";
import { useArchive, useLocal, useRows } from "../🧭️context/🟦️.tsx";
import { ArchiveCard, Button, CheckField, Notice, NumberField, PageFrame, ProgressNotice, SelectField, StarIcon, Tag, TextField, cn } from "../🪟️chrome/🟦️.tsx";
import { Level, rowMajorCells, tracks, useBreakpoint, useSize, type LevelPage, type PaneView } from "../🥞️levels/🟦️.tsx";
import { CatalogProblem, RowPreview } from "../🖼️previews/🟦️.tsx";
import { navigate, recallParams, rememberParams, useAddress, type Route } from "../🗺️routing/🟦️.ts";
import { ANALYTICS, NO_FILTERS, SORT_KEYS, activeFilterCount, filterAndSort, filtersFromParams, filtersToParams, needsAssets, rowCompletion, rowHeightM, rowMassKg, type Analytic, type Filters, type Row, type SortKey } from "../🔎️search/🟦️.ts";
import { useAssetLoading } from "../📚️catalog/🟦️.ts";
import { AssetPage } from "../🔬️detail/🟦️.tsx";

const NO_PARAMS = new URLSearchParams();

/** 🔗️ The route and address query of the asset page of a row. */
export const assetRoute = (row: Row): Route => ({ page: "asset", category: row.entry.category, asset: row.entry.id });

/** ⭐️ The favorite toggle of an asset: a real button whose pressed state and label say what it does. */
export function FavoriteButton(props: { readonly asset: string; readonly title: string; readonly className?: string }): ReactElement {
  const { t } = useArchive();
  const { store } = useArchive();
  const { favorites } = useLocal();
  const on = favorites.includes(props.asset);
  return (
    <Button variant="quiet" aria-pressed={on} aria-label={t(on ? "favorite.remove" : "favorite.add", { title: props.title })} title={t(on ? "favorite.remove" : "favorite.add", { title: props.title })} className={cn("px-single", props.className)} onClick={() => store.dispatch(on ? { type: "favorite-removed", asset: props.asset } : { type: "favorite-added", asset: props.asset })}>
      <StarIcon filled={on} />
    </Button>
  );
}

/** 🏷️ Dimension chips of a row with units: height and, once loaded, mass. */
export function RowFacts(props: { readonly row: Row }): ReactElement {
  const { language, t } = useArchive();
  const mass = rowMassKg(props.row);
  return (
    <>
      <Tag title={t("fact.height")}>{formatNumber(rowHeightM(props.row), 2, language)} m</Tag>
      {mass === null ? null : <Tag title={t("fact.mass")}>{formatNumber(mass, mass < 10 ? 1 : 0, language)} kg</Tag>}
    </>
  );
}

/** 🃏️ The card of one asset: live preview, title, facts and the favorite toggle; it opens the asset page. */
export function AssetCard(props: { readonly row: Row; readonly card?: { readonly revealed: boolean; readonly open: () => void }; readonly className?: string; readonly previewClassName?: string }): ReactElement {
  const { row, card, className, previewClassName } = props;
  const { language, t } = useArchive();
  const id = useId();
  const title = row.entry.title[language];
  const open = card?.open ?? (() => navigate(assetRoute(row)));
  const unavailable = row.status === "invalid" || row.status === "missing" || row.status === "failed";
  return (
    <ArchiveCard
      id={id}
      card={`asset:${row.entry.id}`}
      href={`#/${row.entry.category}/${row.entry.id}`}
      onOpen={open}
      revealed={card?.revealed}
      icon={<span aria-hidden="true" className="inline-block size-2 shrink-0 bg-foreground" />}
      title={title}
      footerLeft={<RowFacts row={row} />}
      footerRight={
        <>
          {unavailable ? <Tag>{t(row.status === "invalid" ? "state.invalid.short" : row.status === "failed" ? "state.failed.short" : "state.missing.short")}</Tag> : null}
          <FavoriteButton asset={row.entry.id} title={title} />
        </>
      }
      className={cn("h-full", className)}
    >
      <div className={cn("min-h-0 flex-1", previewClassName)}>
        <RowPreview row={row} solid compact />
      </div>
    </ArchiveCard>
  );
}

const COMPLETION_KEYS = { planned: "completion.planned", authored: "completion.authored", verified: "completion.verified" } as const satisfies Record<Completion, string>;

function toggle<T>(list: readonly T[], item: T): readonly T[] {
  return list.includes(item) ? list.filter((candidate) => candidate !== item) : [...list, item];
}

/** 🎚️ The search, filter and sort controls; every change goes through `onChange`. */
export function FilterToolbar(props: { readonly filters: Filters; readonly onChange: (filters: Filters) => void; readonly fixedCategory?: CategoryId; readonly shown: number; readonly total: number; readonly compact?: boolean; readonly disabled?: boolean }): ReactElement {
  const { filters, onChange, fixedCategory, shown, total, compact, disabled } = props;
  const { t, language } = useArchive();
  const { manifest } = useRows();
  const [open, setOpen] = useState(false);
  const panel = useId();
  const count = activeFilterCount(filters) - (fixedCategory === undefined || filters.categories.length === 0 ? 0 : 1);
  const patch = (next: Partial<Filters>): void => onChange({ ...filters, ...next });
  return (
    <form role="search" aria-label={t("filters.label")} onSubmit={(event) => event.preventDefault()} className="flex flex-col gap-double" inert={disabled}>
      <div className="flex flex-wrap items-end gap-double">
        <TextField type="search" className="min-w-48 flex-1" label={t("filters.search")} placeholder={t("filters.searchPlaceholder")} value={filters.q} onChange={(q) => patch({ q })} />
        <SelectField label={t("filters.sort")} value={filters.sort} onChange={(sort: SortKey) => patch({ sort })} options={SORT_KEYS.map((key) => ({ value: key, label: t(`sort.${key}`) }))} className="w-40" />
        <Button aria-pressed={filters.descending} onClick={() => patch({ descending: !filters.descending })} icon={filters.descending ? "arrow-down" : "arrow-up"}>
          {t(filters.descending ? "sort.descending" : "sort.ascending")}
        </Button>
        <Button aria-expanded={open} aria-controls={panel} onClick={() => setOpen(!open)} icon="sliders-horizontal">
          {t("filters.toggle")}
          {count > 0 ? <Tag className="ml-single">{count}</Tag> : null}
        </Button>
        {count > 0 || filters.q !== "" ? (
          <Button variant="quiet" icon="x" onClick={() => onChange({ ...NO_FILTERS, sort: filters.sort, descending: filters.descending, categories: fixedCategory === undefined ? [] : [fixedCategory] })}>
            {t("filters.clear")}
          </Button>
        ) : null}
        <p role="status" aria-live="polite" className="m-0 ml-auto font-mono text-xs text-muted-foreground">
          {t("filters.results", { shown, total })}
        </p>
      </div>
      {open ? (
        <div id={panel} className={cn("grid gap-double border border-border bg-panel p-double", compact ? "sm:grid-cols-2 xl:grid-cols-4" : "sm:grid-cols-2 xl:grid-cols-4")}>
          {fixedCategory === undefined ? (
            <fieldset className="m-0 flex min-w-0 flex-col gap-single border-0 p-0">
              <legend className="mb-single p-0 text-[0.65rem] uppercase tracking-wide text-muted-foreground">{t("filters.category")}</legend>
              {CATEGORIES.map((category) => (
                <CheckField key={category} label={manifest?.categories.find((entry) => entry.id === category)?.title[language] ?? category} checked={filters.categories.includes(category)} onChange={() => patch({ categories: toggle(filters.categories, category) })} />
              ))}
            </fieldset>
          ) : null}
          <fieldset className="m-0 flex min-w-0 flex-col gap-single border-0 p-0">
            <legend className="mb-single p-0 text-[0.65rem] uppercase tracking-wide text-muted-foreground">{t("filters.analytics")}</legend>
            {ANALYTICS.map((analytic: Analytic) => (
              <CheckField key={analytic} label={t(`analytic.${analytic}`)} checked={filters.analytics.includes(analytic)} onChange={() => patch({ analytics: toggle(filters.analytics, analytic) })} />
            ))}
            <CheckField label={t("filters.rootMotion")} checked={filters.rootMotion} onChange={(rootMotion) => patch({ rootMotion })} />
          </fieldset>
          <fieldset className="m-0 flex min-w-0 flex-col gap-single border-0 p-0">
            <legend className="mb-single p-0 text-[0.65rem] uppercase tracking-wide text-muted-foreground">{t("filters.height")}</legend>
            <div className="flex gap-double">
              <NumberField label={t("filters.heightMin")} unit="m" min={0} step={0.1} value={filters.heightMinM} onChange={(heightMinM) => patch({ heightMinM })} />
              <NumberField label={t("filters.heightMax")} unit="m" min={0} step={0.1} value={filters.heightMaxM} onChange={(heightMaxM) => patch({ heightMaxM })} />
            </div>
          </fieldset>
          <fieldset className="m-0 flex min-w-0 flex-col gap-single border-0 p-0">
            <legend className="mb-single p-0 text-[0.65rem] uppercase tracking-wide text-muted-foreground">{t("filters.completion")}</legend>
            {COMPLETION.map((completion) => (
              <CheckField key={completion} label={t(COMPLETION_KEYS[completion])} checked={filters.completion.includes(completion)} onChange={() => patch({ completion: toggle(filters.completion, completion) })} />
            ))}
          </fieldset>
        </div>
      ) : null}
    </form>
  );
}

/** 🔎️ Filters read from the address of the page that is open, and the way to change them (replacing the history entry). */
export function useAddressFilters(route: Route, active: boolean, fixed?: CategoryId): { readonly filters: Filters; readonly set: (filters: Filters) => void } {
  const address = useAddress();
  const filters = active ? filtersFromParams(address.params) : NO_FILTERS;
  const effective = useMemo(() => (fixed === undefined ? filters : { ...filters, categories: [fixed] }), [filters.q, filters.categories.join(), filters.analytics.join(), filters.heightMinM, filters.heightMaxM, filters.rootMotion, filters.completion.join(), filters.sort, filters.descending, fixed]);
  return {
    filters: effective,
    set: (next) => {
      const params = filtersToParams(fixed === undefined ? next : { ...next, categories: [] });
      rememberParams(route, params);
      navigate(route, params, true);
    },
  };
}

function LoadingNotice(props: { readonly loading: ReturnType<typeof useAssetLoading> }): ReactNode {
  const { t } = useArchive();
  const { loading } = props;
  if (loading.busy) return <ProgressNotice label={t("state.loadingAssets")} done={loading.done} total={loading.total} cancelLabel={t("action.cancel")} onCancel={loading.cancel} />;
  if (loading.cancelled)
    return (
      <Notice kind="info" compact title={t("state.cancelled.title")} actions={<Button onClick={loading.retry} icon="rotate-cw">{t("action.retry")}</Button>}>
        {t("state.cancelled.body")}
      </Notice>
    );
  return null;
}

/** 🗂️ The page of every asset: toolbar and a grid of cards. */
export function BrowsePage(props: { readonly view: PaneView }): ReactElement {
  const { view } = props;
  const { t, language, catalog } = useArchive();
  const { rows, manifestIssues, manifest } = useRows();
  const route: Route = { page: "all" };
  const { filters, set } = useAddressFilters(route, view.opened);
  const loading = useAssetLoading(catalog, view.opened || view.revealed, "all", JSON.stringify(filters.q));
  const shown = useMemo(() => filterAndSort(rows, filters, language), [rows, filters, language]);
  return (
    <PageFrame page="all" wide eyebrow={t("all.eyebrow")} title={t("all.title")} lede={t("all.lede")}>
      <FilterToolbar filters={filters} onChange={set} shown={shown.length} total={rows.length} disabled={!view.opened} />
      <LoadingNotice loading={loading} />
      {manifestIssues !== null && manifestIssues.length > 0 ? (
        <Notice kind="info" compact title={t("state.manifestIssues.title", { count: manifestIssues.length })}>
          {manifestIssues.slice(0, 3).map((issue, index) => <p key={index} className="m-0">{issue.message[language]}</p>)}
        </Notice>
      ) : null}
      {manifest === null ? <CatalogProblem><Notice kind="loading" title={t("state.loading.title")} /></CatalogProblem> : shown.length === 0 ? (
        <Notice kind="empty" title={t("state.noResults.title")} actions={<Button onClick={() => set(NO_FILTERS)} icon="x">{t("filters.clear")}</Button>}>
          {t("state.noResults.body")}
        </Notice>
      ) : (
        <ul className="m-0 grid list-none gap-double p-0" style={{ gridTemplateColumns: "repeat(auto-fill, minmax(min(100%, 16rem), 1fr))" }}>
          {shown.map((row) => (
            <li key={row.entry.id} className="h-72">
              <AssetCard row={row} />
            </li>
          ))}
        </ul>
      )}
    </PageFrame>
  );
}

const CATEGORY_COLUMNS = { desktop: 4, tablet: 3, mobile: 1 } as const;
const CATEGORY_ROW_MIN_PX = 190;

/** 🗂️ The page of one category: its filters and a nested grid of asset cards; an asset opens its page while the grid stays live behind it. */
export function CategoryPage(props: { readonly category: CategoryId; readonly view: PaneView }): ReactElement {
  const { category, view } = props;
  const { t, language, catalog } = useArchive();
  const { rows, manifest } = useRows();
  const { route, navigate: go } = useAddress();
  const breakpoint = useBreakpoint();
  const [attach] = useSize({ width: 1280, height: 640 });
  const here: Route = { page: "category", category };
  const inside = view.opened && (route.page === "category" || route.page === "asset") && route.category === category;
  const { filters, set } = useAddressFilters(here, inside, category);
  const loading = useAssetLoading(catalog, true, [category], category);
  const own = useMemo(() => rows.filter((row) => row.entry.category === category), [rows, category]);
  const opened = inside && route.page === "asset" ? route.asset : null;
  const filtered = useMemo(() => filterAndSort(own, filters, language), [own, filters, language]);
  const withOpened = useMemo(() => (opened === null || filtered.some((row) => row.entry.id === opened) ? filtered : [...filtered, ...own.filter((row) => row.entry.id === opened)]), [filtered, own, opened]);
  const entry = manifest?.categories.find((candidate) => candidate.id === category);
  const columns = CATEGORY_COLUMNS[breakpoint];
  const rowCount = Math.max(1, Math.ceil(withOpened.length / columns));
  const mode = breakpoint === "mobile" ? "swipe" : "strip";
  const pages: readonly LevelPage[] = useMemo(() => withOpened.map((row): LevelPage => ({ id: row.entry.id, label: row.entry.title[language], render: (pane) => <AssetPage row={row} view={pane} /> })), [withOpened, language]);
  const cells = useMemo(() => rowMajorCells(pages.map((page) => page.id), columns), [pages, columns]);
  const byId = useMemo(() => new Map(withOpened.map((row) => [row.entry.id, row])), [withOpened]);
  const index = (id: string): number => pages.findIndex((page) => page.id === id);
  return (
    <div data-page={`category:${category}`} className="relative flex h-full min-h-0 w-full flex-col bg-background text-foreground">
      <div className={cn("flex flex-col gap-double border-b border-border p-double sm:px-4", opened !== null && "hidden")}>
        <div className="flex flex-wrap items-baseline justify-between gap-double">
          <h1 tabIndex={-1} className="m-0 font-serif text-3xl leading-tight outline-none">
            {entry?.title[language] ?? category}
          </h1>
          <p className="m-0 max-w-[70ch] text-xs leading-normal text-muted-foreground">{entry?.description[language]}</p>
        </div>
        <FilterToolbar filters={filters} onChange={set} fixedCategory={category} shown={filtered.length} total={own.length} compact disabled={!inside} />
        <LoadingNotice loading={loading} />
      </div>
      <div ref={attach} className="relative min-h-0 flex-1">
        {pages.length === 0 ? (
          <div className="p-double">
            {own.length === 0 ? <CatalogProblem category={category}><Notice kind="loading" title={t("state.loading.title")} /></CatalogProblem> : (
              <Notice kind="empty" title={t("state.noResults.title")} actions={<Button onClick={() => set({ ...NO_FILTERS, categories: [category] })} icon="x">{t("filters.clear")}</Button>}>
                {t("state.noResults.body")}
              </Notice>
            )}
          </div>
        ) : (
          <Level
            label={entry?.title[language] ?? category}
            pages={pages}
            cells={cells}
            openedId={opened}
            interactive={inside}
            mode={mode}
            onOpen={(id) => go(id === null ? here : assetRoute(byId.get(id)!), id === null ? recallParams(here) : undefined)}
            onEscape={() => go({ page: "overview" })}
            renderCard={(page, card) => {
              const row = byId.get(page.id)!;
              const place = index(page.id);
              const body = <AssetCard row={row} card={card} />;
              return mode === "strip" ? (
                <div className="staffage-cell" style={{ gridColumn: (place % columns) + 1, gridRow: Math.floor(place / columns) + 1 }}>
                  <div className="staffage-card">{body}</div>
                </div>
              ) : (
                <div className="staffage-entry staffage-card">{body}</div>
              );
            }}
            columns={tracks(Array.from({ length: columns }, () => 1))}
            rows={tracks(Array.from({ length: rowCount }, () => 1), CATEGORY_ROW_MIN_PX)}
            budget={breakpoint === "mobile" ? 1 : 4}
          />
        )}
      </div>
    </div>
  );
}
