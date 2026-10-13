import { buildModePolicy, readBuildMode } from "../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, relative } from "node:path";
import { copyBrowserArtifacts } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/🟦️.ts";
import { COMPONENT_RECEIPT_FILE, readComponentReceiptV1 } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { playRuntimeAssetSources } from "../../🧩️runtime/📦️assets/🟦️.ts";
import runtimeSchema from "../../🧩️runtime/🧬️schema/🔣️.json";
import { MODULE_EXTENSION_ROUTE, MODULE_PLUGIN_ROUTE, publishedPageUrl } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { SEMIO_ASSET_DIRECTORY } from "../../../../../🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🌐️delivery/🟦️.ts";

/** 📦 One CDN deployment of play. Each page stays under {@link PLAY_PAGE_BUDGET_BYTES}. */
export const PLAY_PAGE_BUDGET_BYTES = 1_000_000_000;

/** 🗺️ Satellite directories published on their own host so the app page stays small. */
const routeDirectory = (route: string): string => route.startsWith("/") ? route.slice(1) : route;

export const PLAY_PAGE_FAMILIES = [
  { id: "map", directories: ["osm", "vt", "dem"] },
  { id: "media", directories: ["mesh", "cad-assets", "infinite-assets", SEMIO_ASSET_DIRECTORY] },
  { id: "modules", directories: [routeDirectory(MODULE_PLUGIN_ROUTE), routeDirectory(MODULE_EXTENSION_ROUTE)] },
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

export function publishPlayPages(siteDir: string, pagesDir: string, apex: string, budget = PLAY_PAGE_BUDGET_BYTES): readonly PlayPublishedPage[] {
  const entries: PlayDirectoryEntry[] = readdirSync(siteDir, { withFileTypes: true })
    .filter((entry) => entry.name !== ".DS_Store")
    .map((entry) => ({ name: entry.name, bytes: entry.isDirectory() ? directoryBytes(join(siteDir, entry.name)) : statSync(join(siteDir, entry.name)).size }));
  const { pages, origins } = assignPlayPages(entries, apex, budget);
  rmSync(pagesDir, { recursive: true, force: true });
  for (const pageEntry of pages) {
    const destination = join(pagesDir, pageEntry.name);
    mkdirSync(destination, { recursive: true });
    for (const name of pageEntry.directories) renameSync(join(siteDir, name), join(destination, name));
    writeFileSync(join(destination, "CNAME"), `${pageEntry.host}\n`);
    writeFileSync(join(destination, ".nojekyll"), "");
    writeFileSync(join(destination, "_headers"), `/*\n  Cache-Control: no-cache\n${pageEntry.name === "play" ? "" : "  Access-Control-Allow-Origin: *\n"}`);
  }
  for (const pageEntry of pages) rewritePublishedStyleUrls(join(pagesDir, pageEntry.name), origins);
  const modulesPage = pages.find((entry) => entry.name === "modules");
  const playPage = pages.find((entry) => entry.name === "play");
  if (modulesPage) installShardWorkerAssetFetch(join(pagesDir, modulesPage.name), origins);
  if (modulesPage && playPage) retainSameOriginShardWorker(join(pagesDir, modulesPage.name), join(pagesDir, playPage.name));
  const published = pages.map(entry => ({ ...entry, bytes: directoryBytes(join(pagesDir, entry.name)) }));
  for (const entry of published) if (entry.bytes >= budget) throw new Error(`Play page ${entry.name} is ${entry.bytes} bytes, at or above the ${budget} byte CDN page limit`);
  rmSync(siteDir, { recursive: true, force: true });
  for (const pageEntry of published) console.log(`Play page ${pageEntry.host}: ${pageEntry.bytes} bytes`);
  return published;
}


/** 🧾️ Where the module catalog is published, relative to the plugin route. */
export const PLAY_MODULE_CATALOG_FILE = "🧾️catalog.json";

/** 🧾️ One runtime module bound to its receipt and its published files (`PlayModuleBindingV1`). */
export type PlayModuleBindingV1 = Readonly<{ pluginId: string; route: string; directory: string; receipt: Readonly<{ profile: "wasm-prototype" | "wasm-release"; wasmSha256: string; wasmBytes: number; semanticDescriptorSha256: string; sourceTreeDigest: string; provenanceSha256: string }>; files: readonly Readonly<{ path: string; sha256: string; bytes: number }>[] }>;

/** 🧾️ The runtime module catalog of one deployment (`PlayModuleCatalogV1`). */
export type PlayModuleCatalogV1 = Readonly<{ schemaVersion: 1; modules: readonly PlayModuleBindingV1[] }>;

/** 🌲️ Every regular file below `directory` as a sorted `/` path. */
function filesBelow(directory: string, prefix = ""): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => entry.isDirectory() ? filesBelow(join(directory, entry.name), `${prefix}${entry.name}/`) : entry.isFile() ? [`${prefix}${entry.name}`] : []).sort();
}

/** 📦️ Binds the runtime into an already bundled site: copies every runtime module, the browser support and the fonts from their producers' owned outputs,
 * then writes the module catalog that binds each module directory to its receipt and to the digest of every file it publishes. The bundle itself never
 * embeds compiled bytes, so it is built before any component links. */
export async function bindPlayRuntimeAssetsV1(siteDir: string, workspace: string, signal?: AbortSignal): Promise<PlayModuleCatalogV1> {
  await copyBrowserArtifacts(siteDir, playRuntimeAssetSources(workspace, readBuildMode(process.env.SEMIO_BUILD_MODE)), signal ? { signal } : {});
  const modules: PlayModuleBindingV1[] = [];
  for (const route of [MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE].map(routeDirectory)) {
    const root = join(siteDir, route);
    if (!existsSync(root)) continue;
    for (const directory of readdirSync(root, { withFileTypes: true }).filter((entry) => entry.isDirectory() && existsSync(join(root, entry.name, COMPONENT_RECEIPT_FILE))).map((entry) => entry.name).sort()) {
      const receipt = readComponentReceiptV1(join(root, directory, COMPONENT_RECEIPT_FILE));
      const profile = buildModePolicy(readBuildMode(process.env.SEMIO_BUILD_MODE)).wasmProfile;
      if (receipt.profile !== profile) throw new Error(`Play ${profile} publication cannot bind ${directory}: ${receipt.profile}`);
      modules.push({ pluginId: receipt.pluginId, route, directory, receipt: { profile, wasmSha256: receipt.wasmSha256, wasmBytes: receipt.wasmBytes, semanticDescriptorSha256: receipt.semanticDescriptorSha256, sourceTreeDigest: receipt.sourceTreeDigest, provenanceSha256: receipt.provenanceSha256 }, files: filesBelow(join(root, directory)).map((path) => { const bytes = readFileSync(join(root, directory, path)); return { path, sha256: createHash("sha256").update(bytes).digest("hex"), bytes: bytes.byteLength }; }) });
    }
  }
  const catalog: PlayModuleCatalogV1 = { schemaVersion: 1, modules };
  const errors = validateJsonSchemaSubset(runtimeSchema.$defs.PlayModuleCatalogV1, catalog, runtimeSchema);
  if (errors.length) throw new Error(`Invalid play module catalog: ${errors.join("; ")}`);
  writeFileSync(join(siteDir, routeDirectory(MODULE_PLUGIN_ROUTE), PLAY_MODULE_CATALOG_FILE), `${JSON.stringify(catalog, null, 1)}\n`);
  return catalog;
}

/** 🔎️ Every way the published pages disagree with their module catalog: a missing catalog, a missing or changed file, or a module whose receipt is not the
 * one the catalog binds. Empty when every module page file is exactly the one the catalog names. */
export function playModuleCatalogFindingsV1(pagesDir: string): readonly string[] {
  const page = join(pagesDir, "modules"), path = join(page, routeDirectory(MODULE_PLUGIN_ROUTE), PLAY_MODULE_CATALOG_FILE);
  if (!existsSync(path)) return [`Missing module catalog ${relative(pagesDir, path)}`];
  const catalog = JSON.parse(readFileSync(path, "utf8")) as PlayModuleCatalogV1, findings = validateJsonSchemaSubset(runtimeSchema.$defs.PlayModuleCatalogV1, catalog, runtimeSchema);
  for (const module of catalog.modules ?? []) {
    const directory = join(page, module.route, module.directory);
    for (const file of module.files) {
      const target = join(directory, file.path);
      if (!existsSync(target)) { findings.push(`${module.pluginId}: missing ${file.path}`); continue; }
      const bytes = readFileSync(target);
      if (bytes.byteLength !== file.bytes || createHash("sha256").update(bytes).digest("hex") !== file.sha256) findings.push(`${module.pluginId}: ${file.path} changed after binding`);
    }
    const receipt = existsSync(join(directory, COMPONENT_RECEIPT_FILE)) ? readComponentReceiptV1(join(directory, COMPONENT_RECEIPT_FILE)) : undefined;
    if (!receipt || receipt.wasmSha256 !== module.receipt.wasmSha256 || receipt.provenanceSha256 !== module.receipt.provenanceSha256 || receipt.semanticDescriptorSha256 !== module.receipt.semanticDescriptorSha256) findings.push(`${module.pluginId}: published receipt is not the bound receipt`);
  }
  return findings;
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
        const prelude = `const __semioPageOrigins = ${JSON.stringify(origins)};\nconst __semioNativeFetch = globalThis.fetch.bind(globalThis);\nfunction __semioRelocate(url) {\n  let path = url;\n  if (/^[a-z][a-z0-9+.-]*:/i.test(url)) {\n    let parsed;\n    try { parsed = new URL(url); } catch { return url; }\n    if (parsed.origin !== self.location.origin) return url;\n    path = parsed.pathname + parsed.search + parsed.hash;\n  }\n  if (!path.startsWith("/") || path.startsWith("//")) return url;\n  const encoded = path.split(/[?#]/, 1)[0];\n  if (/%(?:2f|5c)/iu.test(encoded)) return url;\n  let pathname;\n  try { pathname = decodeURIComponent(encoded); } catch { return url; }\n  const key = Object.keys(__semioPageOrigins).sort((a, b) => b.length - a.length).find((prefix) => pathname === prefix || pathname.startsWith(prefix + "/"));\n  return key ? __semioPageOrigins[key].replace(/\\/$/, "") + path : url;\n}\nglobalThis.fetch = (input, init) => {\n  if (typeof Request !== "undefined" && input instanceof Request) {\n    if (input.method !== "GET" && input.method !== "HEAD") return __semioNativeFetch(input, init);\n    const next = __semioRelocate(input.url);\n    return next === input.url ? __semioNativeFetch(input, init) : __semioNativeFetch(new Request(next, input), init);\n  }\n  const raw = typeof input === "string" ? input : input instanceof URL ? input.href : "";\n  const next = raw ? __semioRelocate(raw) : "";\n  return __semioNativeFetch(next || input, init);\n};\n`;
        const workerUrl = publishedPageUrl(`/${relative(directory, path).replaceAll("\\", "/")}`, origins);
        const source = readFileSync(path, "utf8").replace(/(\bimport\s*\(\s*(?:\/\*[\s\S]*?\*\/\s*)?)(["'])(\.\.?\/[^"']+)\2/gu, (_, prefix: string, quote: string, specifier: string) => `${prefix}${quote}${new URL(specifier, workerUrl).href}${quote}`);
        writeFileSync(path, prelude + source);
      }
    }
  };
  visit(directory);
}



/** 🧵 Browsers refuse a worker loaded from another host, so the play page keeps this one script. */
function retainSameOriginShardWorker(modulesDir: string, playDir: string): void {
  const visit = (current: string): void => {
    for (const entry of readdirSync(current, { withFileTypes: true })) {
      const path = join(current, entry.name);
      if (entry.isDirectory()) visit(path);
      else if (entry.name.endsWith("shard-worker.js")) {
        const destination = join(playDir, relative(modulesDir, path));
        mkdirSync(dirname(destination), { recursive: true });
        cpSync(path, destination);
      }
    }
  };
  visit(modulesDir);
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../../🧪️tests/🧪️playpages/🟦️.ts");
  await registerTests1(import.meta.vitest, { assignPlayPages, playPageOrigins, publishPlayPages, PLAY_PAGE_BUDGET_BYTES });
}
