/** 📚️ The catalog of the archive: the manifest, one lazily loaded and validated document per category, and the rows the pages read.
 *
 * Each asset file is its own chunk, loaded when a page needs it: an asset page asks for its one asset ({@link CatalogStore.loadAsset}), a grid for its category. An asset with findings, or one the manifest plans but nobody
 * has authored, stays in its place as an unavailable row with the reason, and never stops the rest. Loading reports progress per asset, yields to the browser every eight assets and can be cancelled; a cancelled load leaves the
 * category idle so it can be retried. Once every category is loaded `validateManifest` checks the whole archive against every loaded document, including one with findings.
 *
 * The findings of an asset are an {@link AssetCheck}. A build ships them with the asset, so the row is settled at once: an asset with findings is unavailable. The development server computes them on a thread of its own
 * and streams them: the asset is offered immediately, its row says how far the check is (`check`), and findings that arrive stay beside the offered asset (`findings`), because the development server is where an author
 * judges the model the findings are about. A check whose reader left is withdrawn and asked again when a reader returns.
 *
 * @see ../🔎️search/🟦️.ts — `Row`
 * @see ../../../../🗂️catalog/🔣️.json — the manifest
 */
import { useEffect, useRef, useState, useSyncExternalStore } from "react";
import { CATEGORIES, SCALE_DENOMINATORS, totalPowerW, validateAsset, validateManifest } from "@teaching/staffage";
import type { Asset, CategoryEntry, Revision, CategoryId, Issue, JobControl, Manifest, ManifestEntry, Slug, Text } from "@teaching/staffage";
import type { StaffageFindingsLine, StaffageServedPayload } from "../../🌐️vite/🟦️.ts";
import manifestDocument from "../../../../🗂️catalog/🔣️.json";
import { namedPoses, type Row } from "../🔎️search/🟦️.ts";
import { unpackAsset } from "./🗜️payload/🟦️.ts";

/** 🗂️ The assets of one category folder, gathered from its one-file-per-asset documents. */
export type CategoryDocument = { readonly assets: readonly Asset[]; readonly revisions: Readonly<Record<Slug, Revision>> };

/** 🔌️ Where catalog documents come from; the app bundles them, tests inject their own. */
export interface CatalogSource {
  readonly manifest: (signal?: AbortSignal) => Promise<Manifest>;
  readonly category: (entry: CategoryEntry, signal?: AbortSignal) => Promise<CategoryDocument | null>;
  readonly asset?: (entry: CategoryEntry, id: Slug, signal?: AbortSignal) => Promise<{ readonly asset: Asset; readonly revision?: Revision } | null>;
}

const documents = import.meta.glob<StaffageServedPayload>("../../../../🗂️catalog/*/*/🔣️.json", { query: "?staffage-asset", import: "default" });
const stripSelectors = (path: string): string => path.replaceAll("️", "");
const documentsFor = (entry: CategoryEntry): string[] => {
  const wanted = `/${stripSelectors(entry.folder)}/`;
  return Object.keys(documents).filter((path) => stripSelectors(path).includes(wanted));
};

const decoded = new Map<string, Promise<{ asset: Asset; revision: Revision }>>();

async function readDocument(key: string, signal?: AbortSignal): Promise<{ asset: Asset; revision: Revision }> {
  signal?.throwIfAborted();
  let reading = decoded.get(key);
  if (reading === undefined) {
    reading = documents[key]!().then(decodeDocument);
    decoded.set(key, reading);
    reading.catch(() => decoded.get(key) === reading && decoded.delete(key));
  }
  const document = await reading;
  signal?.throwIfAborted();
  return document;
}

function decodeDocument(payload: StaffageServedPayload): { asset: Asset; revision: Revision } {
  const asset = unpackAsset(payload);
  verdicts.set(asset, payload.issues ?? payload.findings ?? validateAsset(asset));
  return { asset, revision: payload.revision as Revision };
}

const DOCUMENTS_AT_ONCE = 6;

/** 📦️ The catalog bundled with the site: the manifest inline, each asset file a chunk of its own, gathered per category folder; a category without asset files loads as `null`. */
export const bundledCatalog: CatalogSource = {
  manifest: async () => manifestDocument as unknown as Manifest,
  category: async (entry, signal) => {
    const keys = documentsFor(entry).sort();
    if (keys.length === 0) return null;
    const loaded: { asset: Asset; revision: Revision }[] = [];
    for (let from = 0; from < keys.length; from += DOCUMENTS_AT_ONCE) loaded.push(...(await Promise.all(keys.slice(from, from + DOCUMENTS_AT_ONCE).map((key) => readDocument(key, signal)))));
    return { assets: loaded.map(({ asset }) => asset), revisions: Object.fromEntries(loaded.map(({ asset, revision }) => [asset.id, revision])) };
  },
  asset: async (entry, id, signal) => {
    const key = documentsFor(entry).find((path) => path.split("/").at(-2)?.replace(/^[^a-z]+/u, "") === id);
    return key === undefined ? null : readDocument(key, signal);
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
const verdicts = new WeakMap<Asset, readonly Issue[] | string>();
const PROGRESS_STRIDE = 8;
const THREW: Issue = { path: "$", code: "validation-threw", message: tx("The check of this asset stopped before it finished.", "Die Prüfung dieses Assets brach vorzeitig ab.") };

/** 🩺️ How the findings of an asset are found: at once, or later while the asset is already offered. `control.signal` withdraws a later answer, `control.progress` hears how far its check is. */
export type AssetCheck = (asset: Asset, control?: JobControl) => readonly Issue[] | Promise<readonly Issue[]>;

/** 🚦️ How many findings streams are read at once, so the checks of a whole category never take the connections the assets themselves load over. */
export const FINDINGS_AT_ONCE = 2;

let streaming = 0;
const turns: (() => void)[] = [];

function takeTurn(signal?: AbortSignal): Promise<void> {
  return new Promise((resolve, reject) => {
    const leave = (): void => {
      turns.splice(turns.indexOf(turn), 1);
      reject(signal!.reason);
    };
    const turn = (): void => {
      signal?.removeEventListener("abort", leave);
      streaming += 1;
      resolve();
    };
    if (signal?.aborted) return reject(signal.reason);
    if (streaming < FINDINGS_AT_ONCE) return turn();
    turns.push(turn);
    signal?.addEventListener("abort", leave, { once: true });
  });
}

/** 📡️ Reads the findings the development server streams at `address`: every progress line is passed on, the findings line ends the stream; a `stale` line means the file changed after this module was served, so it has no findings of its own.
 * @see ../../🌐️vite/🟦️.ts — `StaffageFindingsLine` */
export async function streamedFindings(address: string, control: JobControl = {}): Promise<readonly Issue[]> {
  await takeTurn(control.signal);
  try {
    const response = await fetch(address, control.signal ? { signal: control.signal } : {});
    if (!response.ok || response.body === null) throw new Error(`The findings stream answered ${response.status}`);
    const reader = response.body.pipeThrough(new TextDecoderStream()).getReader();
    let rest = "";
    for (;;) {
      const { done, value } = await reader.read();
      const lines = (rest + (value ?? "")).split("\n");
      rest = lines.pop()!;
      for (const text of lines) {
        if (text === "") continue;
        const line = JSON.parse(text) as StaffageFindingsLine;
        if ("progress" in line) control.progress?.(...line.progress);
        else return "issues" in line ? line.issues : [];
      }
      if (done) throw new Error("The findings stream ended before its findings");
    }
  } finally {
    streaming -= 1;
    turns.shift()?.();
  }
}

/** 🩺️ The findings of a bundled asset: those its module carries, else those its module points to, else `validateAsset` here. */
export const checkBundledAsset: AssetCheck = (asset, control) => {
  const verdict = verdicts.get(asset) ?? validateAsset(asset);
  if (typeof verdict !== "string") {
    verdicts.set(asset, verdict);
    return verdict;
  }
  return streamedFindings(verdict, control).then((issues) => {
    verdicts.set(asset, issues);
    return issues;
  });
};

function entryOf(asset: Asset): ManifestEntry {
  return { id: asset.id, category: asset.category, title: asset.title, heightM: asset.dimensions.heightM, widthM: asset.dimensions.widthM, powerW: totalPowerW(asset), poses: namedPoses(asset).map((pose) => pose.id), clips: asset.clips.map((clip) => clip.id), representations: Object.fromEntries(SCALE_DENOMINATORS.map((denominator) => [`${denominator}`, asset.representations[`${denominator}`]!.status])) as ManifestEntry["representations"], symbols: asset.symbols.map((symbol) => symbol.id), analytics: [asset.thermal.regions.length > 0 ? "thermal" : null, asset.thermal.emitters.length > 0 ? "transport" : null, "structural"].filter((item): item is "thermal" | "transport" | "structural" => item !== null), exports: asset.exports, distinction: asset.summary, completion: asset.completion };
}

type Loaded = { readonly revisions: Readonly<Record<Slug, Revision>>; readonly assets: Map<Slug, Asset>; readonly allAssets: ReadonlyMap<Slug, Asset>; readonly invalid: Map<Slug, readonly Issue[]>; readonly known: ReadonlySet<Slug> };

/** 📶️ How far the check of an offered asset is; `total` is 0 until its first part is done. */
export type CheckProgress = NonNullable<Row["check"]>;

type Unchecked = { readonly asset: Asset; readonly category: CategoryId };

/** 🏪️ The catalog store: `loadManifest`, `loadCategory`, `loadAll`; read through {@link useCatalog}. */
export class CatalogStore {
  #snapshot: CatalogSnapshot = { manifest: null, manifestState: "idle", categories: initialCategories(), rows: [], manifestIssues: null };
  readonly #listeners = new Set<() => void>();
  readonly #loaded = new Map<CategoryId, Loaded>();
  readonly #individual = new Map<Slug, Row>();
  readonly #running = new Map<CategoryId, { readonly promise: Promise<void>; readonly controller: AbortController; interested: number }>();
  #manifestPromise: Promise<void> | null = null;
  readonly #source: CatalogSource;
  readonly #validate: AssetCheck;
  readonly #checks = new Map<Slug, CheckProgress>();
  readonly #unchecked = new Map<Slug, Unchecked>();
  readonly #findings = new WeakMap<Asset, readonly Issue[]>();

  constructor(source: CatalogSource, validate: AssetCheck = checkBundledAsset) {
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
    if (this.#snapshot.manifest === null) return;
    const current = this.#snapshot.categories[id];
    if (current.state === "ready") for (const [slug, unchecked] of [...this.#unchecked]) if (unchecked.category === id) this.#recheck(slug, control.signal);
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

  async loadAsset(entry: ManifestEntry, signal: AbortSignal): Promise<void> {
    await this.loadManifest(signal);
    if (signal.aborted || this.#snapshot.manifest === null) return;
    if (this.#loaded.has(entry.category)) return this.#recheck(entry.id, signal);
    const category = this.#snapshot.manifest?.categories.find(({ id }) => id === entry.category);
    if (category === undefined || this.#source.asset === undefined) {
      if (this.#snapshot.categories[entry.category].state === "failed") await this.retryCategory(entry.category, { signal });
      else await this.loadCategory(entry.category, { signal });
      return;
    }
    const previous = this.#individual.get(entry.id);
    if (previous !== undefined && previous.status !== "failed") return this.#recheck(entry.id, signal);
    let row: Row;
    try {
      const loaded = await this.#source.asset(category, entry.id, signal);
      if (signal.aborted) return;
      if (loaded === null) row = { entry, status: "missing" };
      else {
        const { asset, revision } = loaded;
        if (asset.id !== entry.id || asset.category !== entry.category) throw new Error("Asset document does not match the requested entry");
        const issues = this.#verdict(asset, entry.category, signal);
        row = issues === undefined || issues.length === 0 ? { entry, status: "ready", asset, revision } : { entry, status: "invalid", issues };
      }
    } catch {
      if (signal.aborted) return;
      row = { entry, status: "failed" };
    }
    if (this.#loaded.has(entry.category)) return;
    this.#individual.set(entry.id, row);
    this.#set({});
  }

  retryCategory(id: CategoryId, control: JobControl = {}): Promise<void> {
    this.#category(id, IDLE);
    return this.loadCategory(id, control);
  }

  async loadAll(control: JobControl = {}): Promise<void> {
    await this.loadManifest(control.signal);
    const categories = this.#snapshot.manifest?.categories ?? [];
    for (const [index, { id }] of categories.entries()) {
      if (control.signal?.aborted) return;
      await this.loadCategory(id, control.signal ? { signal: control.signal } : {});
      control.progress?.(index + 1, categories.length, tx("Categories loaded", "Kategorien geladen"));
    }
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
      if (document !== null && (!Array.isArray(document.assets) || document.revisions === null || typeof document.revisions !== "object")) return this.#category(id, { state: "failed", done: 0, total: 0, failure: tx("The category file is malformed: it needs a list of assets and a record of revisions.", "Die Kategoriedatei ist fehlerhaft: sie braucht eine Liste der Assets und eine Zuordnung der Revisionen.") });
      if (document === null) {
        this.#loaded.set(id, { revisions: {}, assets: new Map(), allAssets: new Map(), invalid: new Map(), known: new Set() });
        return this.#category(id, { state: "ready", done: planned, total: planned, absent: true });
      }
      const assets = new Map<Slug, Asset>();
      const allAssets = new Map<Slug, Asset>();
      const invalid = new Map<Slug, readonly Issue[]>();
      const total = document.assets.length;
      for (const [index, asset] of document.assets.entries()) {
        if (index % PROGRESS_STRIDE === 0) await yieldToBrowser();
        signal.throwIfAborted();
        allAssets.set(asset.id, asset);
        let issues: readonly Issue[] | undefined;
        try {
          issues = this.#verdict(asset, id, signal);
        } catch (error) {
          issues = [THREW];
        }
        if (issues === undefined || issues.length === 0) assets.set(asset.id, asset);
        else invalid.set(asset.id, issues);
        progress?.(index + 1, total, asset.title);
        if ((index + 1) % PROGRESS_STRIDE === 0) this.#category(id, { state: "loading", done: index + 1, total }, false);
      }
      this.#loaded.set(id, { revisions: document.revisions, assets, allAssets, invalid, known: new Set(manifest.entries.map((candidate) => candidate.id)) });
      for (const [key, row] of this.#individual) if (row.entry.category === id) this.#individual.delete(key);
      this.#category(id, { state: "ready", done: total, total });
    } catch (error) {
      if (isAbort(error) || signal.aborted) return this.#running.get(id)?.controller.signal === signal || !this.#running.has(id) ? this.#category(id, IDLE) : undefined;
      this.#category(id, { state: "failed", done: 0, total: 0, failure: tx("The category file could not be read. Check the connection and try again.", "Die Kategoriedatei konnte nicht gelesen werden. Prüfen Sie die Verbindung und versuchen Sie es erneut.") });
    }
  }

  #verdict(asset: Asset, category: CategoryId, signal: AbortSignal | undefined): readonly Issue[] | undefined {
    const check: { done: number; total: number; label?: Text } = { done: 0, total: 0 };
    const verdict = this.#validate(asset, {
      ...(signal === undefined ? {} : { signal }),
      progress: (done, total, label) => {
        if (this.#checks.get(asset.id) !== check) return;
        this.#checks.set(asset.id, Object.assign(check, { done, total, label }));
        this.#set({});
      },
    });
    if (!(verdict instanceof Promise)) return verdict;
    this.#checks.set(asset.id, check);
    this.#unchecked.delete(asset.id);
    void verdict.then((issues) => issues, () => (signal?.aborted ? null : [THREW])).then((issues) => {
      if (this.#checks.get(asset.id) !== check) return;
      this.#checks.delete(asset.id);
      if (issues === null) this.#unchecked.set(asset.id, { asset, category });
      else if (issues.length > 0) this.#findings.set(asset, issues);
      this.#set({});
    });
    return undefined;
  }

  #recheck(id: Slug, signal: AbortSignal | undefined): void {
    const unchecked = this.#unchecked.get(id);
    if (unchecked === undefined || signal?.aborted) return;
    this.#unchecked.delete(id);
    const issues = this.#verdict(unchecked.asset, unchecked.category, signal);
    if (issues !== undefined && issues.length > 0) this.#findings.set(unchecked.asset, issues);
    this.#set({});
  }

  #category(id: CategoryId, progress: CategoryProgress, rebuild = true): void {
    const categories = { ...this.#snapshot.categories, [id]: progress };
    if (!rebuild) {
      this.#snapshot = { ...this.#snapshot, categories };
      for (const listener of [...this.#listeners]) listener();
      return;
    }
    this.#set({ categories });
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
      const check = this.#checks.get(entry.id);
      const individual = this.#individual.get(entry.id);
      const late = (asset: Asset | undefined): Pick<Row, "check" | "findings"> => {
        const findings = asset === undefined ? undefined : this.#findings.get(asset);
        return { ...(check === undefined ? {} : { check: { ...check } }), ...(findings === undefined ? {} : { findings }) };
      };
      if (loaded === undefined) return individual === undefined ? (progress.state === "failed" ? { entry, status: "failed" } : { entry, status: "pending" }) : individual.status === "ready" ? { ...individual, ...late(individual.asset) } : individual;
      const asset = loaded.assets.get(entry.id);
      if (asset !== undefined) return { entry, status: "ready", asset, revision: loaded.revisions[entry.id], ...late(asset) };
      const issues = loaded.invalid.get(entry.id);
      return issues === undefined ? { entry, status: "missing" } : { entry, status: "invalid", issues };
    });
    for (const loaded of this.#loaded.values()) for (const asset of loaded.assets.values()) if (!loaded.known.has(asset.id)) rows.push({ entry: entryOf(asset), status: "ready", asset, revision: loaded.revisions[asset.id] });
    return rows;
  }

  #issues(snapshot: CatalogSnapshot): readonly Issue[] | null {
    if (snapshot.manifest === null || !snapshot.manifest.categories.every((category) => snapshot.categories[category.id].state === "ready")) return null;
    const authored: Asset[] = [];
    for (const loaded of this.#loaded.values()) for (const asset of loaded.allAssets.values()) authored.push(asset);
    return validateManifest(snapshot.manifest, authored);
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
    void (async () => {
      for (const id of scopeKey.split(",") as CategoryId[]) {
        if (run.signal.aborted) return;
        await store.loadCategory(id, { signal: run.signal });
      }
    })();
    return () => run.abort();
  }, [store, wanted, cancelled, scopeKey]);
  const planned = (id: CategoryId): number => snapshot.manifest?.entries.filter((entry) => entry.category === id).length ?? 0;
  const progress = ids.map((id) => snapshot.categories[id]);
  const total = ids.reduce((sum, id, index) => sum + (progress[index]!.total || planned(id)), 0);
  const done = progress.reduce((sum, state, index) => sum + (state.state === "ready" ? (state.total || planned(ids[index]!)) : state.done), 0);
  const busy = wanted && !cancelled && progress.some((state) => state.state === "idle" || state.state === "loading");
  return { busy, cancelled: wanted && cancelled && progress.some((state) => state.state === "idle" || state.state === "loading"), done, total, cancel: () => { setCancelled(true); controller.current?.abort(); }, retry: () => setCancelled(false) };
}
