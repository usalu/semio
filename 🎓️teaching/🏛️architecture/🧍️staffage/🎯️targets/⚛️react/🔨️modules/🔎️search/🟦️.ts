/** 🔎️ Search, filters and sorting of the archive: pure functions over catalog rows, and their shareable URL parameters.
 *
 * Titles (both languages) are searched from the manifest alone; tags, summaries, mass, root motion and the authoritative completion need the loaded assets ({@link needsAssets}), which the app then loads with progress and cancellation.
 * Matching is accent-insensitive and tokenised: every word the reader types must occur in the searched text.
 *
 * @see ../🗺️routing/🟦️.ts — where these parameters live in the address
 * @see https://unicode.org/reports/tr15/ — normalisation behind accent folding
 */
import { CATEGORIES, COMPLETION } from "@teaching/staffage";
import type { Asset, CategoryId, Completion, Issue, Language, ManifestEntry } from "@teaching/staffage";

/** 🧮️ The analyses an asset can offer. */
export const ANALYTICS = ["thermal", "structural"] as const;

/** 🧮️ One of {@link ANALYTICS}. */
export type Analytic = (typeof ANALYTICS)[number];

/** ↕️ The sort keys. */
export const SORT_KEYS = ["name", "height", "mass", "category"] as const;

/** ↕️ One of {@link SORT_KEYS}. */
export type SortKey = (typeof SORT_KEYS)[number];

/** 🎚️ Everything the reader can narrow and order the archive by. */
export type Filters = {
  readonly q: string;
  readonly categories: readonly CategoryId[];
  readonly analytics: readonly Analytic[];
  readonly heightMinM: number | null;
  readonly heightMaxM: number | null;
  readonly rootMotion: boolean;
  readonly completion: readonly Completion[];
  readonly sort: SortKey;
  readonly descending: boolean;
};

/** 🧹️ No narrowing, ordered by name. */
export const NO_FILTERS: Filters = { q: "", categories: [], analytics: [], heightMinM: null, heightMaxM: null, rootMotion: false, completion: [], sort: "name", descending: false };

/** 🗂️ One catalog row: the manifest entry, and the asset once its category is loaded and valid. `pending`: its category is not loaded yet; `missing`: loaded, but the asset is not authored; `failed`: its category file could not be read; `invalid`: authored, but it fails validation (`issues` say why). */
export type Row = { readonly entry: ManifestEntry; readonly status: "ready" | "pending" | "invalid" | "missing" | "failed"; readonly asset?: Asset; readonly issues?: readonly Issue[] };

function list<T extends string>(value: string | null, allowed: readonly T[]): readonly T[] {
  return (value ?? "")
    .split(",")
    .filter((item): item is T => (allowed as readonly string[]).includes(item))
    .filter((item, index, all) => all.indexOf(item) === index);
}

function metres(text: string): number | null {
  const value = text === "" ? Number.NaN : Number(text);
  return Number.isFinite(value) && value >= 0 ? value : null;
}

/** 🔗️ The filters of an address query; unknown values are dropped. */
export function filtersFromParams(params: URLSearchParams): Filters {
  const [low = "", high = ""] = (params.get("h") ?? "").split("-");
  const sort = params.get("sort");
  return {
    q: (params.get("q") ?? "").slice(0, 120),
    categories: list(params.get("cat"), CATEGORIES),
    analytics: list(params.get("an"), ANALYTICS),
    heightMinM: metres(low),
    heightMaxM: metres(high),
    rootMotion: params.get("root") === "1",
    completion: list(params.get("done"), COMPLETION),
    sort: (SORT_KEYS as readonly string[]).includes(sort ?? "") ? (sort as SortKey) : "name",
    descending: params.get("dir") === "desc",
  };
}

/** 🔗️ Writes the non-default filters into an address query, in a stable order. */
export function filtersToParams(filters: Filters, into: URLSearchParams = new URLSearchParams()): URLSearchParams {
  if (filters.q !== "") into.set("q", filters.q);
  if (filters.categories.length > 0) into.set("cat", filters.categories.join(","));
  if (filters.analytics.length > 0) into.set("an", filters.analytics.join(","));
  if (filters.heightMinM !== null || filters.heightMaxM !== null) into.set("h", `${filters.heightMinM ?? ""}-${filters.heightMaxM ?? ""}`);
  if (filters.rootMotion) into.set("root", "1");
  if (filters.completion.length > 0) into.set("done", filters.completion.join(","));
  if (filters.sort !== "name") into.set("sort", filters.sort);
  if (filters.descending) into.set("dir", "desc");
  return into;
}

/** 🔢️ How many narrowing filters (not the query and not the order) are on. */
export function activeFilterCount(filters: Filters): number {
  return (filters.categories.length > 0 ? 1 : 0) + (filters.analytics.length > 0 ? 1 : 0) + (filters.heightMinM !== null || filters.heightMaxM !== null ? 1 : 0) + (filters.rootMotion ? 1 : 0) + (filters.completion.length > 0 ? 1 : 0);
}

/** 📥️ Whether the filters read what only loaded assets carry. */
export function needsAssets(filters: Filters): boolean {
  return filters.q.trim() !== "" || filters.rootMotion || filters.completion.length > 0 || filters.sort === "mass";
}

/** 🔤️ Lower-case, accent-folded text; `ß` becomes `ss`. */
export function normalize(text: string): string {
  return text.normalize("NFD").replace(/\p{M}/gu, "").replace(/ß/gu, "ss").toLowerCase();
}

/** 🧾️ Everything searchable of a row: both titles, tags, both summaries, the distinction. */
export function searchText(row: Row): string {
  const { entry, asset } = row;
  return normalize([entry.title.en, entry.title.de, entry.category, entry.id.replaceAll("-", " "), entry.distinction.en, entry.distinction.de, ...(asset === undefined ? [] : [asset.title.en, asset.title.de, asset.summary.en, asset.summary.de, ...asset.tags])].join(" "));
}

/** 📏️ The nominal height of a row in metres. */
export const rowHeightM = (row: Row): number => row.asset?.dimensions.heightM ?? row.entry.heightM;

/** ⚖️ The nominal mass of a row in kilograms, once loaded. */
export const rowMassKg = (row: Row): number | null => row.asset?.dimensions.massKg ?? null;

/** ✅️ The completion of a row: the asset's own once loaded, else the plan's. */
export const rowCompletion = (row: Row): Completion => row.asset?.completion ?? row.entry.completion;

/** 🏃️ Whether an asset has a clip that solves its root motion. */
export const hasRootMotion = (asset: Asset | undefined): boolean => asset?.clips.some((clip) => clip.rootMotion === "plant") ?? false;

/** 🧮️ Whether a row satisfies every narrowing filter and the query. */
export function matches(row: Row, filters: Filters): boolean {
  const { entry } = row;
  if (filters.categories.length > 0 && !filters.categories.includes(entry.category)) return false;
  if (filters.analytics.length > 0 && !filters.analytics.every((analytic) => entry.analytics.includes(analytic))) return false;
  const height = rowHeightM(row);
  if (filters.heightMinM !== null && height < filters.heightMinM) return false;
  if (filters.heightMaxM !== null && height > filters.heightMaxM) return false;
  if (filters.rootMotion && !hasRootMotion(row.asset)) return false;
  if (filters.completion.length > 0 && !filters.completion.includes(rowCompletion(row))) return false;
  const words = normalize(filters.q).split(/\s+/u).filter(Boolean);
  if (words.length === 0) return true;
  const haystack = searchText(row);
  return words.every((word) => haystack.includes(word));
}

/** ↕️ The rows that match, ordered; rows without the sort value (mass before load) go last in either direction. */
export function filterAndSort(rows: readonly Row[], filters: Filters, language: Language): readonly Row[] {
  const direction = filters.descending ? -1 : 1;
  const byName = (a: Row, b: Row): number => a.entry.title[language].localeCompare(b.entry.title[language], language);
  const value = (row: Row): number | null => (filters.sort === "height" ? rowHeightM(row) : filters.sort === "mass" ? rowMassKg(row) : filters.sort === "category" ? CATEGORIES.indexOf(row.entry.category) : null);
  return rows.filter((row) => matches(row, filters)).sort((a, b) => {
    const [x, y] = [value(a), value(b)];
    if (filters.sort === "name") return direction * byName(a, b);
    if (x === null || y === null) return x === y ? byName(a, b) : x === null ? 1 : -1;
    return x === y ? byName(a, b) : direction * (x - y);
  });
}

/** 🧍️ The poses a reader may pick and count: the named ones; in-between key poses of clips (`key`) stay evaluable but are never listed. */
export const namedPoses = (asset: Asset): readonly Asset["poses"][number][] => asset.poses.filter((pose) => !(pose as { readonly key?: boolean }).key);

/** 🔢️ How many of the rows are authored (or verified) and how many are still only planned. */
export function completionCounts(rows: readonly Row[]): { readonly authored: number; readonly planned: number } {
  const authored = rows.filter((row) => rowCompletion(row) !== "planned").length;
  return { authored, planned: rows.length - authored };
}
