/** 🔎️ Search, filters and sorting over catalog rows, with accent folding in both languages.
 * @see ./🥒️.feature — the scenarios */
import { describe, expect, it } from "vitest";
import type { Asset, ManifestEntry } from "@teaching/staffage";
import walkerDocument from "../../../../🧫️fixtures/🚶️reference-walker/🔣️.json";
import { NO_FILTERS, filterAndSort, hasRootMotion, matches, needsAssets, normalize, rowCompletion, type Row } from "../../🔨️modules/🔎️search/🟦️.ts";

const walker = walkerDocument as unknown as Asset;
const entry = (id: string, category: ManifestEntry["category"], en: string, de: string, heightM: number, analytics: ManifestEntry["analytics"] = ["thermal", "structural"]): ManifestEntry => ({ id, category, title: { en, de }, heightM, widthM: 1, poses: [], clips: [], representations: { "500": "unavailable", "200": "unavailable", "100": "authored", "50": "unavailable" }, symbols: [], analytics, viewpoints: ["elevation"], exports: ["svg"], distinction: { en: `${en} distinction`, de: `${de} Unterschied` }, completion: "planned" });
const rows: readonly Row[] = [
  { entry: entry("english-oak", "vegetation", "English Oak", "Stieleiche", 18), status: "pending" },
  { entry: entry("park-bench", "objects", "Park Bench", "Parkbank", 0.85, ["structural"]), status: "pending" },
  { entry: entry("walking-commuter", "humans", "Walking Commuter", "Pendler zu Fuß", 1.78), status: "ready", asset: { ...walker, id: "walking-commuter", tags: ["walker", "commuter"], summary: { en: "An adult walking to work", de: "Ein Erwachsener auf dem Weg zur Arbeit" }, completion: "authored", dimensions: { ...walker.dimensions, massKg: 75 } } },
  { entry: entry("tower-crane", "vehicles", "Tower Crane", "Turmdrehkran", 40), status: "missing" },
];
const ids = (list: readonly Row[]): string[] => list.map((row) => row.entry.id);

describe("search", () => {
  it("finds titles in both languages without accents or case", () => {
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, q: "pendler zu FUSS" }, "en"))).toEqual(["walking-commuter"]);
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, q: "Stieleiche" }, "de"))).toEqual(["english-oak"]);
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, q: "turm kran" }, "en"))).toEqual(["tower-crane"]);
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, q: "turm eiche" }, "en"))).toEqual([]);
  });

  it("finds tags and summaries once the asset is loaded", () => {
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, q: "commuter" }, "en"))).toEqual(["walking-commuter"]);
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, q: "arbeit" }, "en"))).toEqual(["walking-commuter"]);
  });

  it("normalises like Unicode canonical decomposition", () => {
    expect(normalize("Äpfel Straße Crème")).toBe("apfel strasse creme");
    expect(normalize("Äpfel")).toBe("Äpfel".normalize("NFD").replace(/[̀-ͯ]/gu, "").toLowerCase());
  });
});

describe("filters", () => {
  it("narrows by category, analytics, height and completion", () => {
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, categories: ["objects", "humans"] }, "en"))).toEqual(["park-bench", "walking-commuter"]);
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, analytics: ["thermal"] }, "en"))).toEqual(["english-oak", "tower-crane", "walking-commuter"]);
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, heightMinM: 1, heightMaxM: 20 }, "en"))).toEqual(["english-oak", "walking-commuter"]);
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, completion: ["authored"] }, "en"))).toEqual(["walking-commuter"]);
  });

  it("filters by root motion from the loaded clips only", () => {
    expect(hasRootMotion(rows[2]!.asset)).toBe(walker.clips.some((clip) => clip.rootMotion === "plant"));
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, rootMotion: true }, "en"))).toEqual(walker.clips.some((clip) => clip.rootMotion === "plant") ? ["walking-commuter"] : []);
  });

  it("uses the asset's own completion once loaded", () => {
    expect(rowCompletion(rows[2]!)).toBe("authored");
    expect(rowCompletion(rows[0]!)).toBe("planned");
  });

  it("says when filters need loaded assets", () => {
    expect(needsAssets(NO_FILTERS)).toBe(false);
    expect(needsAssets({ ...NO_FILTERS, q: "x" })).toBe(true);
    expect(needsAssets({ ...NO_FILTERS, sort: "mass" })).toBe(true);
    expect(matches(rows[0]!, NO_FILTERS)).toBe(true);
  });
});

describe("sorting", () => {
  it("orders by name in the reader's language", () => {
    expect(ids(filterAndSort(rows, NO_FILTERS, "en"))).toEqual(["english-oak", "park-bench", "tower-crane", "walking-commuter"]);
    expect(ids(filterAndSort(rows, NO_FILTERS, "de"))).toEqual(["tower-crane", "walking-commuter", "park-bench", "english-oak"].sort((a, b) => rows.find((r) => r.entry.id === a)!.entry.title.de.localeCompare(rows.find((r) => r.entry.id === b)!.entry.title.de, "de")));
  });

  it("orders by height in both directions", () => {
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, sort: "height" }, "en"))).toEqual(["park-bench", "walking-commuter", "english-oak", "tower-crane"]);
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, sort: "height", descending: true }, "en"))).toEqual(["tower-crane", "english-oak", "walking-commuter", "park-bench"]);
  });

  it("puts rows without a mass last in either direction", () => {
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, sort: "mass" }, "en"))[0]).toBe("walking-commuter");
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, sort: "mass", descending: true }, "en"))[0]).toBe("walking-commuter");
  });

  it("orders by category in archive order", () => {
    expect(ids(filterAndSort(rows, { ...NO_FILTERS, sort: "category" }, "en"))).toEqual(["walking-commuter", "english-oak", "park-bench", "tower-crane"]);
  });
});
