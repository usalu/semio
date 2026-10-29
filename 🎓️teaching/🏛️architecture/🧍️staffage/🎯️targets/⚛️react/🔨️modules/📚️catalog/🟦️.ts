/** 📚️ The catalog of the archive: the manifest, one lazily loaded and validated document per category, and the rows the pages read.
 *
 * Each category file is its own chunk, loaded when a page needs it. Every asset passes `validateAsset` before it is offered; one that fails, or one the manifest plans but nobody has authored, stays in its place as an
 * unavailable row with the reason, and never stops the rest. Loading reports progress per asset, yields to the browser between assets and can be cancelled; a cancelled load leaves the category idle so it can be retried.
 * Once every category is loaded `validateManifest` checks the whole archive.
 *
 * @see ../🔎️search/🟦️.ts — `Row`
 * @see ../../../../🗂️catalog/🔣️.json — the manifest
 */
import { useEffect, useRef, useState, useSyncExternalStore } from "react";
import { CATEGORIES, validateAsset, validateManifest } from "@teaching/staffage";
import type { Asset, CategoryDocument, CategoryEntry, CategoryId, Issue, JobControl, Manifest, ManifestEntry, Slug, Text } from "@teaching/staffage";
import manifestDocument from "../../../../🗂️catalog/🔣️.json";
import { namedPoses, type Row } from "../🔎️search/🟦️.ts";

/** 🔌️ Where catalog documents come from; the app bundles them, tests inject their own. */
export interface CatalogSource {
  readonly manifest: (signal?: AbortSignal) => Promise<Manifest>;
  readonly category: (entry: CategoryEntry, signal?: AbortSignal) => Promise<CategoryDocument | null>;
}

const documents = import.meta.glob<CategoryDocument>("../../../../🗂️catalog/*/🔣️.json", { import: "default" });
const stripSelectors = (path: string): string => path.replaceAll("️", "");

/** 📦️ The catalog bundled with the site: the manifest inline, each category file a chunk of its own; a category whose file does not exist yet loads as `null`. */
export const bundledCatalog: CatalogSource = {
  manifest: async () => manifestDocument as unknown as Manifest,
  category: async (entry, signal) => {
    signal?.throwIfAborted();
    const wanted = stripSelectors(entry.file);
    const key = Object.keys(documents).find((path) => stripSelectors(path).endsWith(`/${wanted}`));
    return key === undefined ? null : await documents[key]!();
  },
};

/** 🔄️ How far a category has come. */
export type CategoryProgress = { readonly state: "idle" | "loading" | "ready" | "failed"; readonly done: number; readonly total: number; readonly failure?: Text; readonly absent?: boolean };

/** 📸️ What the pages read. */
export type CatalogSnapshot = {
  readonly manifest: Manifest | null;
  readonly manifestState: "idle" | "loading" | "ready" | "failed";
  readonly categories: Readonly<Record<CategoryId, CategoryProgress>>;
  readonly rows: readonly Row[];
  readonly manifestIssues: readonly Issue[] | null;
};

const IDLE: CategoryProgress = { state: "idle", done: 0, total: 0 };
const initialCategories = (): Record<CategoryId, CategoryProgress> => Object.fromEntries(CATEGORIES.map((id) => [id, IDLE])) as Record<CategoryId, CategoryProgress>;
const tx = (en: string, de: string): Text => ({ en, de });
const isAbort = (error: unknown): boolean => error instanceof DOMException && error.name === "AbortError";

const yieldToBrowser = (): Promise<void> => new Promise((resolve) => setTimeout(resolve, 0));

function entryOf(asset: Asset): ManifestEntry {
  return { id: asset.id, category: asset.category, title: asset.title, heightM: asset.dimensions.heightM, widthM: asset.dimensions.widthM, poses: namedPoses(asset).map((pose) => pose.id), clips: asset.clips.map((clip) => clip.id), analytics: [asset.thermal.regions.length > 0 ? "thermal" : null, "structural"].filter((item): item is "thermal" | "structural" => item !== null), viewpoints: asset.presentation.viewpoints, exports: asset.exports, distinction: asset.summary, completion: asset.completion };
}

type Loaded = { readonly assets: ReadonlyMap<Slug, Asset>; readonly invalid: ReadonlyMap<Slug, readonly Issue[]>; readonly extra: readonly Asset[] };

/** 🏪️ The catalog store: `loadManifest`, `loadCategory`, `loadAll`; read through {@link useCatalog}. */
export class CatalogStore {
  #snapshot: CatalogSnapshot = { manifest: null, manifestState: "idle", categories: initialCategories(), rows: [], manifestIssues: null };
  readonly #listeners = new Set<() => void>();
  readonly #loaded = new Map<CategoryId, Loaded>();
  readonly #running = new Map<CategoryId, { readonly promise: Promise<void>; readonly controller: AbortController; interested: number }>();
  #manifestPromise: Promise<void> | null = null;
  readonly #source: CatalogSource;
  readonly #validate: (asset: Asset) => readonly Issue[];

  constructor(source: CatalogSource, validate: (asset: Asset) => readonly Issue[] = validateAsset) {
    this.#source = source;
    this.#validate = validate;
  }

  getSnapshot = (): CatalogSnapshot => this.#snapshot;

  subscribe = (listener: () => void): (() => void) => {
    this.#listeners.add(listener);
    return () => void this.#listeners.delete(listener);
  };

  asset(id: Slug): Asset | undefined {
    return this.#snapshot.rows.find((row) => row.entry.id === id)?.asset;
  }

  loadManifest(signal?: AbortSignal): Promise<void> {
    if (this.#snapshot.manifest !== null) return Promise.resolve();
    this.#manifestPromise ??= (async () => {
      this.#set({ manifestState: "loading" });
      try {
        const manifest = await this.#source.manifest(signal);
        this.#set({ manifest, manifestState: "ready" });
      } catch (error) {
        this.#manifestPromise = null;
        this.#set({ manifestState: isAbort(error) ? "idle" : "failed" });
      }
    })();
    return this.#manifestPromise;
  }

  async loadCategory(id: CategoryId, control: JobControl = {}): Promise<void> {
    await this.loadManifest(control.signal);
    const current = this.#snapshot.categories[id];
    if (current.state === "ready" || current.state === "failed") return;
    let run = this.#running.get(id);
    if (run === undefined || run.controller.signal.aborted) {
      const controller = new AbortController();
      run = { controller, interested: 0, promise: this.#run(id, controller.signal, control.progress) };
      this.#running.set(id, run);
      const mine = run;
      void run.promise.finally(() => this.#running.get(id) === mine && this.#running.delete(id));
    }
    run.interested += 1;
    const started = run;
    const release = (): void => {
      started.interested -= 1;
      if (started.interested <= 0) started.controller.abort();
    };
    if (control.signal?.aborted) release();
    else control.signal?.addEventListener("abort", release, { once: true });
    await started.promise;
  }

  retryCategory(id: CategoryId, control: JobControl = {}): Promise<void> {
    this.#category(id, IDLE);
    return this.loadCategory(id, control);
  }

  async loadAll(control: JobControl = {}): Promise<void> {
    await this.loadManifest(control.signal);
    const ids = this.#snapshot.manifest!.categories.map((category) => category.id);
    let done = 0;
    await Promise.all(
      ids.map((id) =>
        this.loadCategory(id, { ...(control.signal === undefined ? {} : { signal: control.signal }) }).then(() => {
          done += 1;
          control.progress?.(done, ids.length, tx("Categories loaded", "Kategorien geladen"));
        }),
      ),
    );
  }

  async #run(id: CategoryId, signal: AbortSignal, progress: JobControl["progress"]): Promise<void> {
    const manifest = this.#snapshot.manifest!;
    const entry = manifest.categories.find((category) => category.id === id);
    const planned = manifest.entries.filter((candidate) => candidate.category === id).length;
    if (entry === undefined) return this.#category(id, { state: "failed", done: 0, total: 0, failure: tx(`Category "${id}" is not in the manifest.`, `Kategorie "${id}" steht nicht im Manifest.`) });
    this.#category(id, { state: "loading", done: 0, total: planned });
    try {
      const document = await this.#source.category(entry, signal);
      signal.throwIfAborted();
      if (document === null) {
        this.#loaded.set(id, { assets: new Map(), invalid: new Map(), extra: [] });
        return this.#category(id, { state: "ready", done: planned, total: planned, absent: true });
      }
      const assets = new Map<Slug, Asset>();
      const invalid = new Map<Slug, readonly Issue[]>();
      const total = document.assets.length;
      for (const [index, asset] of document.assets.entries()) {
        await yieldToBrowser();
        signal.throwIfAborted();
        let issues: readonly Issue[];
        try {
          issues = this.#validate(asset);
        } catch (error) {
          issues = [{ path: "$", code: "validation-threw", message: tx("The check of this asset stopped before it finished.", "Die Prüfung dieses Assets brach vorzeitig ab.") }];
        }
        if (issues.length === 0) assets.set(asset.id, asset);
        else invalid.set(asset.id, issues);
        progress?.(index + 1, total, asset.title);
        this.#category(id, { state: "loading", done: index + 1, total });
      }
      const known = new Set(manifest.entries.map((candidate) => candidate.id));
      this.#loaded.set(id, { assets, invalid, extra: [...assets.values()].filter((asset) => !known.has(asset.id)) });
      this.#category(id, { state: "ready", done: total, total });
    } catch (error) {
      if (isAbort(error) || signal.aborted) return this.#running.get(id)?.controller.signal === signal || !this.#running.has(id) ? this.#category(id, IDLE) : undefined;
      this.#category(id, { state: "failed", done: 0, total: 0, failure: tx("The category file could not be read. Check the connection and try again.", "Die Kategoriedatei konnte nicht gelesen werden. Prüfen Sie die Verbindung und versuchen Sie es erneut.") });
    }
  }

  #category(id: CategoryId, progress: CategoryProgress): void {
    this.#set({ categories: { ...this.#snapshot.categories, [id]: progress } });
  }

  #set(patch: Partial<CatalogSnapshot>): void {
    const next = { ...this.#snapshot, ...patch };
    this.#snapshot = { ...next, rows: this.#rows(next), manifestIssues: this.#issues(next) };
    for (const listener of [...this.#listeners]) listener();
  }

  #rows(snapshot: CatalogSnapshot): readonly Row[] {
    if (snapshot.manifest === null) return [];
    const rows: Row[] = snapshot.manifest.entries.map((entry): Row => {
      const loaded = this.#loaded.get(entry.category);
      const progress = snapshot.categories[entry.category];
      if (loaded === undefined) return progress.state === "failed" ? { entry, status: "failed" } : { entry, status: "pending" };
      const asset = loaded.assets.get(entry.id);
      if (asset !== undefined) return { entry, status: "ready", asset };
      const issues = loaded.invalid.get(entry.id);
      return issues === undefined ? { entry, status: "missing" } : { entry, status: "invalid", issues };
    });
    for (const loaded of this.#loaded.values()) for (const asset of loaded.extra) rows.push({ entry: entryOf(asset), status: "ready", asset });
    return rows;
  }

  #issues(snapshot: CatalogSnapshot): readonly Issue[] | null {
    if (snapshot.manifest === null || !snapshot.manifest.categories.every((category) => snapshot.categories[category.id].state === "ready")) return null;
    return validateManifest(snapshot.manifest, snapshot.rows.flatMap((row) => (row.asset === undefined ? [] : [row.asset])));
  }
}

/** 🪝️ Subscribes a component to the catalog. */
export function useCatalog(store: CatalogStore): CatalogSnapshot {
  return useSyncExternalStore(store.subscribe, store.getSnapshot, store.getSnapshot);
}

/** 🏷️ The category entry of an id. */
export const categoryOf = (snapshot: CatalogSnapshot, id: CategoryId): CategoryEntry | undefined => snapshot.manifest?.categories.find((category) => category.id === id);

/** 📶️ How far the loading of the categories a view needs has come. */
export type AssetLoading = { readonly busy: boolean; readonly cancelled: boolean; readonly done: number; readonly total: number; readonly cancel: () => void; readonly retry: () => void };

/** 📥️ Loads the categories in `scope` (all by default) while `wanted`, with progress; `cancel` stops it and stays stopped until `retry` or until `resetKey` changes. */
export function useAssetLoading(store: CatalogStore, wanted: boolean, scope: readonly CategoryId[] | "all", resetKey = ""): AssetLoading {
  const snapshot = useCatalog(store);
  const [cancelled, setCancelled] = useState(false);
  const controller = useRef<AbortController | null>(null);
  const ids = scope === "all" ? (snapshot.manifest?.categories.map((category) => category.id) ?? []) : scope;
  const scopeKey = ids.join(",");
  useEffect(() => setCancelled(false), [resetKey]);
  useEffect(() => {
    if (!wanted || cancelled || scopeKey === "") return;
    const run = new AbortController();
    controller.current = run;
    for (const id of scopeKey.split(",") as CategoryId[]) void store.loadCategory(id, { signal: run.signal });
    return () => run.abort();
  }, [store, wanted, cancelled, scopeKey]);
  const planned = (id: CategoryId): number => snapshot.manifest?.entries.filter((entry) => entry.category === id).length ?? 0;
  const progress = ids.map((id) => snapshot.categories[id]);
  const total = ids.reduce((sum, id, index) => sum + (progress[index]!.total || planned(id)), 0);
  const done = progress.reduce((sum, state, index) => sum + (state.state === "ready" ? (state.total || planned(ids[index]!)) : state.done), 0);
  const busy = wanted && !cancelled && progress.some((state) => state.state === "idle" || state.state === "loading");
  return { busy, cancelled: wanted && cancelled && progress.some((state) => state.state === "idle" || state.state === "loading"), done, total, cancel: () => { setCancelled(true); controller.current?.abort(); }, retry: () => setCancelled(false) };
}
