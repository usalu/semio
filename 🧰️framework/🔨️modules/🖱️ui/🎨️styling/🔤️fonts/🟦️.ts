import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const moduleRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));
const stylingOwnerRoot = resolve(moduleRoot, "..");
const tokensPath = join(stylingOwnerRoot, "🔣️.json");
export const ELEMENTS_ASSETS_ROOT = resolve(stylingOwnerRoot, "../../🖼️assets");
const elementsAssetsRoot = ELEMENTS_ASSETS_ROOT;
const fontCatalogPath = join(elementsAssetsRoot, "🔤️fonts/📇️catalog.json");
const GOOGLE_FONTS_UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

export interface FontCatalog {
  readonly $schema: "./🧬️schema/🔣️.json";
  readonly version: 1;
  readonly encodings: Readonly<Record<"ttf" | "woff" | "woff2", string>>;
  readonly weights: Readonly<Record<string, number>>;
  readonly families: readonly { readonly directory: string; readonly family: string; readonly weights: readonly string[]; readonly subsets: readonly { readonly directory: string; readonly sourceSubset: string | null }[] }[];
}

export interface FontSource {
  readonly path: string;
  readonly family: string;
  readonly subset: string | null;
  readonly weight: number;
  readonly format: "ttf" | "woff" | "woff2";
}

/** 📇️ Admits the source-first font catalog without interpreting handpicked filenames. */
export function parseFontCatalog(value: unknown): FontCatalog {
  const record = (input: unknown, keys: readonly string[]): Record<string, unknown> => {
    if (!input || typeof input !== "object" || Array.isArray(input) || Object.keys(input).length !== keys.length || !keys.every(key => Object.hasOwn(input, key))) throw new Error("Invalid font catalog record");
    return input as Record<string, unknown>;
  };
  const directory = (input: unknown): input is string => typeof input === "string" && /^\p{Extended_Pictographic}\uFE0F[a-z]+(?:-[a-z]+)*$/u.test(input);
  const unique = (items: readonly unknown[]): boolean => new Set(items).size === items.length;
  const root = record(value, ["$schema", "version", "encodings", "weights", "families"]);
  if (root.$schema !== "./🧬️schema/🔣️.json" || root.version !== 1) throw new Error("Unknown font catalog version");
  const encodings = record(root.encodings, ["ttf", "woff", "woff2"]);
  if (encodings.ttf !== "🔤️outline.ttf" || encodings.woff !== "🌐️web.woff" || encodings.woff2 !== "🗜️compressed.woff2") throw new Error("Invalid font encoding identities");
  if (!root.weights || typeof root.weights !== "object" || Array.isArray(root.weights)) throw new Error("Invalid font weights");
  const weights = Object.entries(root.weights);
  if (!weights.length || !weights.every(([name, weight]) => directory(name) && typeof weight === "number" && Number.isInteger(weight) && weight >= 100 && weight <= 900 && weight % 100 === 0) || !unique(weights.map(([, weight]) => weight))) throw new Error("Invalid font weight identities");
  if (!Array.isArray(root.families) || !root.families.length) throw new Error("Missing font families");
  const families = root.families.map(input => {
    const family = record(input, ["directory", "family", "weights", "subsets"]);
    if (!directory(family.directory) || typeof family.family !== "string" || !/^[A-Za-z]+(?: [A-Za-z]+)*$/.test(family.family)) throw new Error("Invalid font family identity");
    if (!Array.isArray(family.weights) || !family.weights.length || !unique(family.weights) || !family.weights.every(name => directory(name) && Object.hasOwn(root.weights as object, name))) throw new Error("Unknown family weight");
    if (!Array.isArray(family.subsets) || !family.subsets.length) throw new Error("Missing font subsets");
    const subsets = family.subsets.map(input => {
      const subset = record(input, ["directory", "sourceSubset"]);
      if (!directory(subset.directory) || (subset.sourceSubset !== null && (typeof subset.sourceSubset !== "string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(subset.sourceSubset)))) throw new Error("Invalid font subset identity");
      return subset;
    });
    if (!unique(subsets.map(subset => subset.directory)) || !unique(subsets.map(subset => subset.sourceSubset))) throw new Error("Duplicate font subset");
    return family;
  });
  if (!unique(families.map(family => family.directory)) || !unique(families.map(family => family.family))) throw new Error("Duplicate font family");
  return value as FontCatalog;
}

/** 🔗️ Binds exact catalog-relative file identities to declared provider metadata. */
export function fontCatalogSources(catalog: FontCatalog): readonly FontSource[] {
  return catalog.families.flatMap(family => family.subsets.flatMap(subset => family.weights.flatMap(weight => Object.entries(catalog.encodings).map(([format, filename]) => ({ path: ["🔤️fonts", family.directory, subset.directory, weight, filename].join("/"), family: family.family, subset: subset.sourceSubset, weight: catalog.weights[weight]!, format: format as FontSource["format"] })))));
}

/** 🔎️ Requires an exact catalog-owned font path, with no basename fallback. */
export function resolveFontSource(path: string, catalog: FontCatalog): FontSource {
  const source = fontCatalogSources(catalog).find(source => source.path === path);
  if (!source) throw new Error(`No catalog font identity for ${path}`);
  return source;
}

export function loadFontCatalog(): FontCatalog { return parseFontCatalog(JSON.parse(readFileSync(fontCatalogPath, "utf8"))); }


function googleFontsCssUrl(family: string, weight: number): string {
  const query = family.trim().replaceAll(" ", "+");
  return `https://fonts.googleapis.com/css2?family=${query}:wght@${weight}&display=swap`;
}

export function parseGoogleFontWoff2Map(css: string): Map<string, string> {
  const map = new Map<string, string>();
  let subset: string | undefined;
  for (const line of css.split("\n")) {
    const comment = line.match(/^\s*\/\*\s*([^*]+?)\s*\*\/\s*$/);
    if (comment) {
      subset = comment[1]!.trim().toLowerCase();
      continue;
    }
    const urlMatch = line.match(/url\((https:[^)]+\.woff2)\)/);
    if (!urlMatch) {
      continue;
    }
    const url = urlMatch[1]!;
    if (subset) {
      map.set(subset, url);
      subset = undefined;
      continue;
    }
    const indexMatch = url.match(/\.(\d+)\.woff2/);
    if (indexMatch) {
      map.set(indexMatch[1]!, url);
    }
  }
  return map;
}

export function resolveFontFaceUrl(source: FontSource, woff2ByKey: ReadonlyMap<string, string>): string | undefined {
  return source.format === "woff2" && source.subset !== null ? woff2ByKey.get(source.subset) : undefined;
}

/** ⬇️ Acquires only missing catalog-declared WOFF2 subsets, preserving every existing vendored file. */
export async function fetchElementsFonts(options: { readonly signal?: AbortSignal; readonly onProgress?: (completed: number, total: number, path: string) => void } = {}): Promise<void> {
  const tokens = JSON.parse(readFileSync(tokensPath, "utf8")) as { readonly fontFaces: readonly { readonly family: string; readonly src: string }[] };
  const catalog = loadFontCatalog();
  const sources = tokens.fontFaces.map(face => resolveFontSource(face.src, catalog));
  const missing = sources.filter(source => !existsSync(join(elementsAssetsRoot, source.path)));
  for (const source of missing) if (source.subset === null || source.format !== "woff2") throw new Error(`Missing vendored-only font ${source.path}`);
  const cssByFamilyWeight = new Map<string, Map<string, string>>();
  let wrote = 0;
  for (const source of missing) {
    options.signal?.throwIfAborted();
    const key = `${source.family}:${source.weight}`;
    if (!cssByFamilyWeight.has(key)) {
      const res = await fetch(googleFontsCssUrl(source.family, source.weight), { headers: { "User-Agent": GOOGLE_FONTS_UA }, signal: options.signal });
      if (!res.ok) throw new Error(`Google Fonts CSS failed for ${key}: ${res.status}`);
      cssByFamilyWeight.set(key, parseGoogleFontWoff2Map(await res.text()));
    }
    const remoteUrl = resolveFontFaceUrl(source, cssByFamilyWeight.get(key)!);
    if (!remoteUrl) throw new Error(`Provider has no exact subset ${source.subset} for ${source.path}`);
    const dest = join(elementsAssetsRoot, source.path);
    mkdirSync(dirname(dest), { recursive: true });
    if (existsSync(dest)) continue;
    const fileRes = await fetch(remoteUrl, { signal: options.signal });
    if (!fileRes.ok) {
      throw new Error(`Font download failed for ${source.path}: ${fileRes.status}`);
    }
    const bytes = new Uint8Array(await fileRes.arrayBuffer());
    if (bytes.length < 4 || bytes[0] !== 0x77 || bytes[1] !== 0x4f || bytes[2] !== 0x46 || bytes[3] !== 0x32) {
      throw new Error(`Downloaded bytes for ${source.path} are not woff2 (got ${bytes.length} bytes)`);
    }
    options.signal?.throwIfAborted();
    writeFileSync(dest, bytes, { flag: "wx" });
    wrote += 1;
    options.onProgress?.(wrote, missing.length, source.path);
  }
  console.log(`framework/ui/styling: catalog fonts ready under framework assets (${wrote} downloaded, ${sources.length} total)`);
}
