/** 📚️ Category files load lazily, are validated asset by asset with progress, and a bad or missing asset never stops the rest.
 * @see ./🥒️.feature — the scenarios */
import { describe, expect, it } from "vitest";
import type { Asset, CategoryDocument, Issue, Manifest, ManifestEntry } from "@teaching/staffage";
import walkerDocument from "../../../../🧫️fixtures/🚶️reference-walker/🔣️.json";
import { CatalogStore, type CatalogSource } from "../../🔨️modules/📚️catalog/🟦️.ts";

const walker = walkerDocument as unknown as Asset;
const entry = (id: string): ManifestEntry => ({ id, category: "humans", title: walker.title, heightM: 1.7, widthM: 0.5, poses: [], clips: [], analytics: ["thermal"], viewpoints: ["elevation"], exports: ["svg"], distinction: walker.summary, completion: "planned" });
const categoryEntry = (id: "humans" | "objects") => ({ id, emoji: "x", title: { en: id, de: id }, description: { en: id, de: id }, file: `${id}/🔣️.json` });
const manifest = { $schema: "", version: "semio.staffage/v1", categories: [categoryEntry("humans"), categoryEntry("objects")], collections: [], entries: [entry("good-one"), entry("bad-one"), entry("never-authored")] } as unknown as Manifest;
const document = (assets: Asset[]): CategoryDocument => ({ $schema: "", version: "semio.staffage/v1", category: "humans", assets }) as unknown as CategoryDocument;
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

  it("reports a failing file and a validator that throws without stopping", async () => {
    const failing = new CatalogStore({ manifest: async () => manifest, category: async () => { throw new Error("offline"); } }, reasons);
    await failing.loadCategory("humans");
    expect(failing.getSnapshot().categories.humans.state).toBe("failed");
    const throwing = new CatalogStore(source([]), () => { throw new Error("boom"); });
    await throwing.loadCategory("humans");
    expect(throwing.getSnapshot().rows.find((row) => row.entry.id === "good-one")!.issues![0]!.code).toBe("validation-threw");
  });
});
