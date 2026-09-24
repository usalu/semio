import { cpSync, mkdirSync, readdirSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 📦 One CDN deployment of play. Each page stays under {@link PLAY_PAGE_BUDGET_BYTES}. */
export const PLAY_PAGE_BUDGET_BYTES = 1_000_000_000;

/** 🗺️ Satellite directories published on their own host so the app page stays small. */
export const PLAY_PAGE_FAMILIES = [
  { id: "map", directories: ["osm", "vt", "dem"] },
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

const SHED_EXTENSIONS = new Set([".gif", ".glb", ".3dm"]);

/** 🗑️ Drops illustrative media from the app page so plugins fit under the CDN limit. Map tiles stay. */
function shedIllustrativeMedia(siteDir: string): void {
  const keep = new Set(["assets", "osm", "vt", "dem"]);
  const walk = (directory: string, top: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isDirectory()) {
        if (top === "" && (keep.has(entry.name) || entry.name.endsWith("plugin-modules") || entry.name.endsWith("extension-modules"))) continue;
        walk(path, top || entry.name);
      } else if (SHED_EXTENSIONS.has(entry.name.slice(entry.name.lastIndexOf(".")))) {
        rmSync(path);
      }
    }
  };
  walk(siteDir, "");
}

export function publishPlayPages(siteDir: string, pagesDir: string, apex: string): readonly PlayPublishedPage[] {
  shedIllustrativeMedia(siteDir);
  const entries: PlayDirectoryEntry[] = readdirSync(siteDir, { withFileTypes: true })
    .filter((entry) => entry.name !== ".DS_Store")
    .map((entry) => ({ name: entry.name, bytes: entry.isDirectory() ? directoryBytes(join(siteDir, entry.name)) : statSync(join(siteDir, entry.name)).size }));
  const { pages } = assignPlayPages(entries, apex);
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
  rmSync(siteDir, { recursive: true, force: true });
  for (const pageEntry of pages) console.log(`Play page ${pageEntry.host}: ${pageEntry.bytes} bytes`);
  return pages;
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../../🧪️tests/🧪️playpages/🟦️.ts");
  await registerTests1(import.meta.vitest, { assignPlayPages, playPageOrigins, PLAY_PAGE_BUDGET_BYTES });
}
