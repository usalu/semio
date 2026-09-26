import { mkdirSync, readFileSync, readdirSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { SEMIO_ASSET_DIRECTORY } from "../../../../../🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🌐️delivery/🟦️.ts";

/** 📦 One CDN deployment of play. Each page stays under {@link PLAY_PAGE_BUDGET_BYTES}. */
export const PLAY_PAGE_BUDGET_BYTES = 1_000_000_000;

/** 🗺️ Satellite directories published on their own host so the app page stays small. */
export const PLAY_PAGE_FAMILIES = [
  { id: "map", directories: ["osm", "vt", "dem"] },
  { id: "media", directories: ["mesh", "cad-assets", "infinite-assets", SEMIO_ASSET_DIRECTORY] },
] as const;

export type PlayDirectoryEntry = { readonly name: string; readonly bytes: number };

export type PlayPublishedPage = {
  readonly name: string;
  readonly host: string;
  readonly origin: string;
  readonly directories: readonly string[];
  readonly bytes: number;
};

/** 🌐️ Asset-only pages are served under `assets.semio-tech.com`. The app page keeps its own host. */
export const PLAY_ASSET_PAGE_SUFFIX = "assets.semio-tech.com";

/** 🌐️ `play.semio-tech.com` for the app, `modules.assets.semio-tech.com` for an asset page. */
export function playPageHost(name: string, apex: string): string {
  return name === "play" ? apex : `${name}.${PLAY_ASSET_PAGE_SUFFIX}`;
}

/** 🧭️ Route prefix to absolute origin for every satellite directory. */
export function playPageOrigins(apex: string): Record<string, string> {
  const origins: Record<string, string> = {};
  for (const family of PLAY_PAGE_FAMILIES) {
    const origin = `https://${playPageHost(family.id, apex)}`;
    for (const directory of family.directories) origins[`/${directory}`] = origin;
  }
  return origins;
}

/** 📄 Packs a built site into CDN pages. A page at or above the budget fails the build. */
export function assignPlayPages(entries: readonly PlayDirectoryEntry[], apex: string, budget = PLAY_PAGE_BUDGET_BYTES): { readonly pages: readonly PlayPublishedPage[]; readonly origins: Readonly<Record<string, string>> } {
  const bytesOf = new Map(entries.map((entry) => [entry.name, entry.bytes]));
  const satellite = new Set<string>(PLAY_PAGE_FAMILIES.flatMap((family) => family.directories));
  const origins = playPageOrigins(apex);
  const playDirectories = entries.filter((entry) => !satellite.has(entry.name)).map((entry) => entry.name);
  const pages: PlayPublishedPage[] = [
    page("play", playDirectories, bytesOf, apex),
    ...PLAY_PAGE_FAMILIES.map((family) => page(family.id, family.directories.filter((name) => bytesOf.has(name)), bytesOf, apex)),
  ].filter((entry) => entry.name === "play" || entry.directories.length > 0);
  for (const entry of pages) {
    if (entry.bytes >= budget) throw new Error(`Play page ${entry.name} is ${entry.bytes} bytes, at or above the ${budget} byte CDN page limit`);
  }
  return { pages, origins };
}

function page(name: string, directories: readonly string[], bytesOf: ReadonlyMap<string, number>, apex: string): PlayPublishedPage {
  const host = playPageHost(name, apex);
  return { name, host, origin: `https://${host}`, directories, bytes: directories.reduce((sum, directory) => sum + (bytesOf.get(directory) ?? 0), 0) };
}

function directoryBytes(directory: string): number {
  let bytes = 0;
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) bytes += directoryBytes(path);
    else if (entry.isFile()) bytes += statSync(path).size;
  }
  return bytes;
}

/** 📄 Moves a monolithic `dist/site` into one folder per CDN page and deletes the monolith. */

export function publishPlayPages(siteDir: string, pagesDir: string, apex: string): readonly PlayPublishedPage[] {
  const entries: PlayDirectoryEntry[] = readdirSync(siteDir, { withFileTypes: true })
    .filter((entry) => entry.name !== ".DS_Store")
    .map((entry) => ({ name: entry.name, bytes: entry.isDirectory() ? directoryBytes(join(siteDir, entry.name)) : statSync(join(siteDir, entry.name)).size }));
  const { pages, origins } = assignPlayPages(entries, apex);
  rmSync(pagesDir, { recursive: true, force: true });
  for (const pageEntry of pages) {
    const destination = join(pagesDir, pageEntry.name);
    mkdirSync(destination, { recursive: true });
    for (const name of pageEntry.directories) renameSync(join(siteDir, name), join(destination, name));
    writeFileSync(join(destination, "CNAME"), `${pageEntry.host}\n`);
    writeFileSync(join(destination, ".nojekyll"), "");
    if (pageEntry.name !== "play") {
      writeFileSync(join(destination, "_headers"), "/*\n  Access-Control-Allow-Origin: *\n");
    }
  }
  const playPage = pages.find((entry) => entry.name === "play");
  if (playPage) {
    const destination = join(pagesDir, playPage.name);
    rewritePublishedStyleUrls(destination, origins);
    installShardWorkerAssetFetch(destination, origins);
  }
  rmSync(siteDir, { recursive: true, force: true });
  for (const pageEntry of pages) console.log(`Play page ${pageEntry.host}: ${pageEntry.bytes} bytes`);
  return pages;
}


const PUBLISHED_STYLE_EXTENSIONS = new Set([".css", ".html", ".js"]);

/** 🎨 Rewrites final stylesheet URLs onto the asset page. Script logic keeps its own prefixes. */
function rewritePublishedStyleUrls(directory: string, origins: Readonly<Record<string, string>>): void {
  const prefixes = Object.keys(origins).sort((a, b) => b.length - a.length);
  const visit = (current: string): void => {
    for (const entry of readdirSync(current, { withFileTypes: true })) {
      const path = join(current, entry.name);
      if (entry.isDirectory()) visit(path);
      else if (PUBLISHED_STYLE_EXTENSIONS.has(path.slice(path.lastIndexOf(".")))) rewriteStyleFile(path, prefixes, origins);
    }
  };
  visit(directory);
}

function rewriteStyleFile(file: string, prefixes: readonly string[], origins: Readonly<Record<string, string>>): void {
  const text = readFileSync(file, "utf8");
  let next = text;
  for (const prefix of prefixes) {
    const origin = origins[prefix]!.replace(/\/$/, "");
    const needle = `${prefix}/`;
    for (const form of [`url(${needle}`, `url("${needle}`, `url('${needle}`]) {
      if (next.includes(form)) next = next.split(form).join(form.replace(needle, `${origin}${needle}`));
    }
  }
  if (next !== text) writeFileSync(file, next);
}

/** 🧵 The shard worker is a copied module, so its asset fetches learn the page map here. */
function installShardWorkerAssetFetch(directory: string, origins: Readonly<Record<string, string>>): void {
  const visit = (current: string): void => {
    for (const entry of readdirSync(current, { withFileTypes: true })) {
      const path = join(current, entry.name);
      if (entry.isDirectory()) visit(path);
      else if (entry.name.endsWith("shard-worker.js")) {
        const prelude = `const __semioPageOrigins = ${JSON.stringify(origins)};\nconst __semioNativeFetch = globalThis.fetch.bind(globalThis);\nfunction __semioRelocate(url) {\n  let path = url;\n  if (/^[a-z][a-z0-9+.-]*:/i.test(url)) {\n    let parsed;\n    try { parsed = new URL(url); } catch { return url; }\n    if (parsed.origin !== self.location.origin) return url;\n    path = parsed.pathname + parsed.search + parsed.hash;\n  }\n  const key = Object.keys(__semioPageOrigins).sort((a, b) => b.length - a.length).find((prefix) => path === prefix || path.startsWith(prefix + "/"));\n  return key ? __semioPageOrigins[key].replace(/\\/$/, "") + path : url;\n}\nglobalThis.fetch = (input, init) => {\n  if (typeof Request !== "undefined" && input instanceof Request) {\n    if (input.method !== "GET" && input.method !== "HEAD") return __semioNativeFetch(input, init);\n    const next = __semioRelocate(input.url);\n    return next === input.url ? __semioNativeFetch(input, init) : __semioNativeFetch(new Request(next, input), init);\n  }\n  const raw = typeof input === "string" ? input : input instanceof URL ? input.href : "";\n  const next = raw ? __semioRelocate(raw) : "";\n  return __semioNativeFetch(next || input, init);\n};\n`;
        writeFileSync(path, prelude + readFileSync(path, "utf8"));
      }
    }
  };
  visit(directory);
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../../🧪️tests/🧪️playpages/🟦️.ts");
  await registerTests1(import.meta.vitest, { assignPlayPages, playPageOrigins, PLAY_PAGE_BUDGET_BYTES });
}
