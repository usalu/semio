/** 📚️ Category files load lazily, are validated asset by asset with progress, and a bad or missing asset never stops the rest.
 * @see ./🥒️.feature — the scenarios */
import { afterEach, describe, expect, it, vi } from "vitest";
import { revisionOfText, type Asset, type Issue, type Manifest, type ManifestEntry } from "@teaching/staffage";
import { testLevelBudgetMs } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import walkerDocument from "../../../../🧫️fixtures/🚶️reference-walker/🔣️.json";
import { bundledCatalog, CatalogStore, FINDINGS_AT_ONCE, streamedFindings, type AssetCheck, type CatalogSource, type CategoryDocument } from "../../🔨️modules/📚️catalog/🟦️.ts";

const walker = walkerDocument as unknown as Asset;
const entry = (id: string): ManifestEntry => ({ id, category: "humans", title: walker.title, heightM: 1.7, widthM: 0.5, powerW: 0, poses: [], clips: [], representations: { "500": "unavailable", "200": "unavailable", "100": "authored", "50": "unavailable" }, symbols: [], analytics: ["thermal"], exports: ["svg"], distinction: walker.summary, completion: "planned" });
const categoryEntry = (id: "humans" | "objects") => ({ id, emoji: "x", title: { en: id, de: id }, description: { en: id, de: id }, folder: id });
const manifest = { $schema: "", version: "semio.staffage/v2", categories: [categoryEntry("humans"), categoryEntry("objects")], collections: [], entries: [entry("good-one"), entry("bad-one"), entry("never-authored")] } as unknown as Manifest;
const document = (assets: Asset[]): CategoryDocument => ({ assets, revisions: Object.fromEntries(assets.map((asset) => [asset.id, revisionOfText(asset.id)])) });
const source = (reads: string[], gate?: Promise<void>): CatalogSource => ({
  manifest: async () => manifest,
  category: async (category, signal) => {
    reads.push(category.id);
    await gate;
    signal?.throwIfAborted();
    return category.id === "objects" ? null : document([{ ...walker, id: "good-one" }, { ...walker, id: "bad-one" }]);
  },
});
const reasons = (asset: Asset): readonly Issue[] => (asset.id === "bad-one" ? [{ path: "$.rig", code: "broken", message: { en: "Broken rig", de: "Defektes Modell" } }] : []);

describe("catalog store", () => {
  it("loads nothing until a category is asked for, then validates every asset with progress", async () => {
    const reads: string[] = [];
    const store = new CatalogStore(source(reads), reasons);
    await store.loadManifest();
    expect(reads).toEqual([]);
    expect(store.getSnapshot().rows.map((row) => row.status)).toEqual(["pending", "pending", "pending"]);
    const progress: number[] = [];
    await store.loadCategory("humans", { progress: (done) => progress.push(done) });
    expect(reads).toEqual(["humans"]);
    expect(progress).toEqual([1, 2]);
    const byId = Object.fromEntries(store.getSnapshot().rows.map((row) => [row.entry.id, row]));
    expect(byId["good-one"]!.status).toBe("ready");
    expect(byId["bad-one"]!.status).toBe("invalid");
    expect(byId["bad-one"]!.issues![0]!.message.de).toBe("Defektes Modell");
    expect(byId["never-authored"]!.status).toBe("missing");
  });

  it("loads a category once however often it is asked", async () => {
    const reads: string[] = [];
    const store = new CatalogStore(source(reads), reasons);
    await Promise.all([store.loadCategory("humans"), store.loadCategory("humans")]);
    await store.loadCategory("humans");
    expect(reads).toEqual(["humans"]);
  });

  it("treats a category file that does not exist as absent, not as a failure", async () => {
    const store = new CatalogStore(source([]), reasons);
    await store.loadCategory("objects");
    expect(store.getSnapshot().categories.objects).toMatchObject({ state: "ready", absent: true });
  });

  it("fails a category file without revisions in both languages instead of staying in loading", async () => {
    const store = new CatalogStore({ manifest: async () => manifest, category: async () => ({ assets: [walker] }) as unknown as CategoryDocument }, reasons);
    await store.loadCategory("humans");
    const humans = store.getSnapshot().categories.humans;
    expect(humans.state).toBe("failed");
    expect(humans.failure!.en).toMatch(/malformed/u);
    expect(humans.failure!.de).toMatch(/fehlerhaft/u);
  });

  it("returns a cancelled category to idle so it can be retried", async () => {
    let open!: () => void;
    const gate = new Promise<void>((resolve) => (open = resolve));
    const store = new CatalogStore(source([], gate), reasons);
    const controller = new AbortController();
    const pending = store.loadCategory("humans", { signal: controller.signal });
    await new Promise((resolve) => setTimeout(resolve, 10));
    controller.abort();
    open();
    await pending;
    expect(store.getSnapshot().categories.humans.state).toBe("idle");
    await store.loadCategory("humans");
    expect(store.getSnapshot().categories.humans.state).toBe("ready");
  });

  it("still shows an invalid asset document to the manifest check", async () => {
    const store = new CatalogStore(source([]), reasons);
    await store.loadAll();
    const missing = (store.getSnapshot().manifestIssues ?? []).filter((issue) => issue.code === "manifest-asset-missing").map((issue) => issue.message.en);
    expect(missing).toEqual(['Manifest entry "never-authored" has no asset document.']);
  });

  it("asks for nothing and settles when the manifest cannot be read", async () => {
    const category = vi.fn<CatalogSource["category"]>(async () => null);
    const store = new CatalogStore({ manifest: async () => { throw new Error("offline"); }, category });
    await store.loadAsset(entry("good-one"), new AbortController().signal);
    await store.loadCategory("humans");
    expect(store.getSnapshot().manifestState).toBe("failed");
    expect(category).not.toHaveBeenCalled();
  });

  it("decodes only the requested digital scale and retains its identity", async () => {
    const store = new CatalogStore(bundledCatalog, () => []);
    const entry = (await bundledCatalog.manifest()).entries.find(({ id }) => id === "city-bicycle")!;
    await store.loadAsset(entry, new AbortController().signal);
    const asset = store.asset(entry.id)!;
    expect(Object.getOwnPropertyDescriptor(asset.volumes, "50")?.get).toBeTypeOf("function");
    const print = asset.representations["100"];
    expect(print.status).toBe("authored");
    if (print.status !== "authored") throw new Error("Expected the authored paper drawing");
    expect(Object.getOwnPropertyDescriptor(print, "geometry")?.get).toBeTypeOf("function");
    const drawing = print.geometry;
    expect(drawing).toBe(print.geometry);
    expect(Object.keys(drawing).length).toBeGreaterThan(0);
    const volume = asset.volumes["50"];
    expect(volume.status).toBe("authored");
    expect(asset.volumes["50"]).toBe(volume);
    expect(Object.getOwnPropertyDescriptor(asset.volumes, "500")?.get).toBeTypeOf("function");
  });

  it("loads the authored archive as one ready row per manifest entry with an empty manifest check", async () => {
    const store = new CatalogStore(bundledCatalog);
    const total = (await bundledCatalog.manifest()).entries.length;
    await store.loadAll();
    const snapshot = store.getSnapshot();
    const counts = snapshot.rows.reduce<Record<string, number>>((tally, row) => ({ ...tally, [row.status]: (tally[row.status] ?? 0) + 1 }), {});
    expect(snapshot.rows).toHaveLength(total);
    const invalid = snapshot.rows.filter(row => row.status !== "ready").map(row => [row.entry.id, row.issues]);
    expect(counts, JSON.stringify(invalid)).toEqual({ ready: total });
    expect(snapshot.manifestIssues).toEqual([]);
    for (const id of ["cantilever-balcony", "glulam-beam", "steel-column"]) expect(snapshot.rows.find((row) => row.entry.id === id)?.status).toBe("ready");
  }, testLevelBudgetMs("quick"));

  it("reports a failing file and a validator that throws without stopping", async () => {
    const failing = new CatalogStore({ manifest: async () => manifest, category: async () => { throw new Error("offline"); } }, reasons);
    await failing.loadCategory("humans");
    expect(failing.getSnapshot().categories.humans.state).toBe("failed");
    const throwing = new CatalogStore(source([]), () => { throw new Error("boom"); });
    await throwing.loadCategory("humans");
    expect(throwing.getSnapshot().rows.find((row) => row.entry.id === "good-one")!.issues![0]!.code).toBe("validation-threw");
  });
});

describe("findings that arrive later", () => {
  const finding: Issue = { path: "$.rig", code: "broken", message: { en: "Broken rig", de: "Defektes Rig" } };
  const single: CatalogSource = { manifest: async () => manifest, category: async () => null, asset: async (_category, id) => (id === "never-authored" ? null : { asset: { ...walker, id, category: "humans" } }) };
  const later = (): { readonly check: AssetCheck; readonly asked: { readonly asset: Asset; readonly control: NonNullable<Parameters<AssetCheck>[1]>; readonly answer: (issues: readonly Issue[]) => void; readonly fail: (error: unknown) => void }[] } => {
    const asked: ReturnType<typeof later>["asked"][number][] = [];
    return { asked, check: (asset, control) => new Promise((resolve, reject) => asked.push({ asset, control: control!, answer: resolve, fail: reject })) };
  };
  const row = (store: CatalogStore, id: string) => store.getSnapshot().rows.find((candidate) => candidate.entry.id === id)!;
  const settled = (): Promise<void> => new Promise((resolve) => setTimeout(resolve, 0));

  it("offers the asset at once, reports the check and keeps the asset beside the findings that arrive", async () => {
    const { check, asked } = later();
    const store = new CatalogStore(single, check);
    await store.loadAsset(entry("good-one"), new AbortController().signal);
    await store.loadAsset(entry("bad-one"), new AbortController().signal);
    expect(row(store, "good-one")).toMatchObject({ status: "ready", check: { done: 0, total: 0 } });
    expect(row(store, "good-one").asset?.id).toBe("good-one");
    asked[1]!.control.progress!(3, 6, { en: "Symbols checked", de: "Symbole geprüft" });
    expect(row(store, "bad-one").check).toEqual({ done: 3, total: 6, label: { en: "Symbols checked", de: "Symbole geprüft" } });
    asked[0]!.answer([]);
    asked[1]!.answer([finding]);
    await settled();
    expect(row(store, "good-one").status).toBe("ready");
    expect(row(store, "good-one").check).toBeUndefined();
    expect(row(store, "good-one").findings).toBeUndefined();
    expect(row(store, "bad-one")).toMatchObject({ status: "ready", findings: [finding] });
    expect(row(store, "bad-one").asset?.id).toBe("bad-one");
    expect(row(store, "bad-one").check).toBeUndefined();
  });

  it("keeps an asset whose check was withdrawn ready and asks again when a reader returns", async () => {
    const { check, asked } = later();
    const store = new CatalogStore(single, check);
    const reader = new AbortController();
    await store.loadAsset(entry("bad-one"), reader.signal);
    expect(asked[0]!.control.signal).toBe(reader.signal);
    reader.abort();
    asked[0]!.fail(new DOMException("left", "AbortError"));
    await settled();
    expect(row(store, "bad-one")).toMatchObject({ status: "ready" });
    expect(row(store, "bad-one").check).toBeUndefined();
    await store.loadAsset(entry("bad-one"), new AbortController().signal);
    expect(asked).toHaveLength(2);
    expect(asked[1]!.asset).toBe(asked[0]!.asset);
    expect(row(store, "bad-one").check).toEqual({ done: 0, total: 0 });
    asked[1]!.answer([finding]);
    await settled();
    expect(row(store, "bad-one")).toMatchObject({ status: "ready", findings: [finding] });
  });

  it("reports a check that fails for another reason as a finding in both languages", async () => {
    const { check, asked } = later();
    const store = new CatalogStore(single, check);
    await store.loadAsset(entry("good-one"), new AbortController().signal);
    asked[0]!.fail(new Error("offline"));
    await settled();
    expect(row(store, "good-one").status).toBe("ready");
    expect(row(store, "good-one").findings![0]).toMatchObject({ code: "validation-threw", message: { en: expect.stringMatching(/stopped/u), de: expect.stringMatching(/brach/u) } });
  });

  it("makes a category ready before its findings and settles each asset when they arrive", async () => {
    const { check, asked } = later();
    const store = new CatalogStore(source([]), check);
    await store.loadCategory("humans");
    expect(store.getSnapshot().categories.humans.state).toBe("ready");
    expect([row(store, "good-one").status, row(store, "bad-one").status, row(store, "never-authored").status]).toEqual(["ready", "ready", "missing"]);
    asked.find(({ asset }) => asset.id === "bad-one")!.answer([finding]);
    asked.find(({ asset }) => asset.id === "good-one")!.answer([]);
    await settled();
    expect(row(store, "bad-one")).toMatchObject({ status: "ready", findings: [finding] });
    expect(row(store, "bad-one").asset?.id).toBe("bad-one");
    expect(row(store, "good-one").findings).toBeUndefined();
    expect(row(store, "good-one").status).toBe("ready");
    expect(row(store, "good-one").check).toBeUndefined();
  });
});

describe("the findings stream", () => {
  afterEach(() => vi.unstubAllGlobals());
  const stream = (chunks: readonly string[]): Response => new Response(new ReadableStream<Uint8Array>({ start: (controller) => { for (const chunk of chunks) controller.enqueue(new TextEncoder().encode(chunk)); controller.close(); } }));
  const finding: Issue = { path: "$", code: "broken", message: { en: "Broken", de: "Defekt" } };

  it("hears every progress line in order and returns the findings, however the lines are cut into chunks", async () => {
    const lines = `${JSON.stringify({ progress: [1, 6, { en: "Rig", de: "Rig" }] })}\n${JSON.stringify({ progress: [2, 6, { en: "Drawings", de: "Zeichnungen" }] })}\n${JSON.stringify({ issues: [finding] })}\n`;
    vi.stubGlobal("fetch", vi.fn(async () => stream([lines.slice(0, 17), lines.slice(17, 90), lines.slice(90)])));
    const heard: string[] = [];
    expect(await streamedFindings("/@staffage-findings/abc", { progress: (done, total, label) => heard.push(`${done}/${total} ${label.de}`) })).toEqual([finding]);
    expect(heard).toEqual(["1/6 Rig", "2/6 Zeichnungen"]);
  });

  it("has no findings for a stale revision and fails when the stream ends without findings", async () => {
    vi.stubGlobal("fetch", vi.fn(async () => stream([`${JSON.stringify({ stale: true })}\n`])));
    expect(await streamedFindings("/@staffage-findings/abc")).toEqual([]);
    vi.stubGlobal("fetch", vi.fn(async () => stream([`${JSON.stringify({ progress: [1, 6, { en: "Rig", de: "Rig" }] })}\n`])));
    await expect(streamedFindings("/@staffage-findings/abc")).rejects.toThrow(/ended before/u);
  });

  it("reads at most two streams at once and gives a waiting reader who leaves no turn", async () => {
    const open: (() => void)[] = [];
    const fetched = vi.fn((_address: string) => new Promise<Response>((resolve) => open.push(() => resolve(stream([`${JSON.stringify({ issues: [] })}\n`])))));
    vi.stubGlobal("fetch", fetched);
    const leaving = new AbortController();
    const reads = [streamedFindings("/a"), streamedFindings("/b"), streamedFindings("/c", { signal: leaving.signal }), streamedFindings("/d")];
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(fetched.mock.calls.map(([address]) => address)).toEqual(["/a", "/b"]);
    expect(fetched).toHaveBeenCalledTimes(FINDINGS_AT_ONCE);
    leaving.abort();
    await expect(reads[2]).rejects.toMatchObject({ name: "AbortError" });
    open[0]!();
    await reads[0];
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(fetched.mock.calls.map(([address]) => address)).toEqual(["/a", "/b", "/d"]);
    open[1]!();
    open[2]!();
    expect(await Promise.all([reads[1], reads[3]])).toEqual([[], []]);
  });
});

