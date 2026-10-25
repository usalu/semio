import { resolve, join, dirname, basename } from "node:path";
import { existsSync, statSync, lstatSync, mkdirSync, cpSync, readdirSync, readFileSync, copyFileSync } from "node:fs";
import type { OwnedBuildPlugin } from "../../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import { SEMIO_ASSET_DIRECTORY, SEMIO_ASSET_ROUTE, assetPathFromRequest } from "../../../../../🖼️assets/📦️packages/🟦️typescript/🔍️resolver/🌐️delivery/🟦️.ts";
import { createUiAssetsMiddleware } from "../🗺️mesh-collection/🟦️.ts";

/** 🗂️ Canonical repo-relative root of the asset-owned public namespace. */
export const SEMIO_ASSET_ROOT = "🧰️framework/🔨️modules/🖼️assets";

/** 📂 Resolves and validates the merged Semio asset package root (fonts required). */
export function resolveSemioAssetRoot(repoRoot: string): string {
  const assetsRoot = resolve(repoRoot, SEMIO_ASSET_ROOT);
  const fontDir = resolve(assetsRoot, "🔤️fonts");
  if (!existsSync(assetsRoot) || !statSync(assetsRoot).isDirectory() || !existsSync(fontDir)) {
    throw new Error(`Missing Semio asset root at ${assetsRoot} (expected ${SEMIO_ASSET_ROOT} with 🔤️fonts)`);
  }
  return assetsRoot;
}

function uiAssetsVitePluginsForRoot(assetsRoot: string): OwnedBuildPlugin[] {
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  const serveAssets = createUiAssetsMiddleware(assetsRoot);
  return [
    {
      name: "ui-assets-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveAssets);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveAssets);
      },
    },
    {
      name: "ui-assets-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput) return;
        if (!existsSync(assetsRoot)) {
          return;
        }
        const dest = resolve(outDir, SEMIO_ASSET_DIRECTORY);
        mkdirSync(outDir, { recursive: true });
        cpSync(assetsRoot, dest, { recursive: true, filter: (source) => basename(source) !== "node_modules" && !lstatSync(source).isSymbolicLink() });
      },
    },
  ];
}

/** 🌐️ Serves and copies shared fonts and cursors at `/🖼️assets/*`. */
export function semioAssetsVitePlugin(repoRoot: string): OwnedBuildPlugin[] {
  return uiAssetsVitePluginsForRoot(resolveSemioAssetRoot(repoRoot));
}

/** 📦️ Serves `/🖼️assets/*` like {@link semioAssetsVitePlugin}, but a build copies only the asset files the written bundle
 * references (the fonts of an imported palette, an icon named in CSS) instead of the whole asset root — for a site that
 * ships a handful of files rather than the library's every font, mesh and image. A reference is a route that opens a
 * token (after a quote, bracket, `=` or whitespace), so a repository path that merely contains `/🖼️assets/` never counts. */
export function semioReferencedAssetsVitePlugin(repoRoot: string): OwnedBuildPlugin[] {
  const assetsRoot = resolveSemioAssetRoot(repoRoot);
  const serveAssets = createUiAssetsMiddleware(assetsRoot);
  const reference = new RegExp(`(?<![^\\s"'\`(=,:\\[{>|;])${SEMIO_ASSET_ROUTE}/[^"'()\\s?#]+`, "gu");
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  const referenced = (directory: string, found: Set<string>): Set<string> => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isDirectory() && entry.name !== SEMIO_ASSET_DIRECTORY) referenced(path, found);
      else if (entry.isFile() && /\.(?:css|html|js|mjs)$/u.test(entry.name)) for (const [match] of readFileSync(path, "utf8").matchAll(reference)) found.add(assetPathFromRequest(match) ?? "");
    }
    return found;
  };
  return [
    {
      name: "semio-referenced-assets",
      configureServer(server) {
        server.middlewares.use(serveAssets);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveAssets);
      },
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput || !existsSync(outDir)) return;
        for (const asset of referenced(outDir, new Set())) {
          if (!asset) continue;
          const target = resolve(outDir, SEMIO_ASSET_DIRECTORY, asset);
          mkdirSync(dirname(target), { recursive: true });
          copyFileSync(resolve(assetsRoot, asset), target);
        }
      },
    },
  ];
}

/** 🛝️ Playground app kind for Vite play harness config (validated against manifest scan). */
export type PlaygroundRendererPuzzleKind = string;

function namedImportSpecifiersForModule(source: string, moduleId: string): string[] {
  const escaped = moduleId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const re = new RegExp(`import\\s*\\{([^}]+)\\}\\s*from\\s*["']${escaped}["']`, "gs");
  const names: string[] = [];
  let match: RegExpExecArray | null;
  while ((match = re.exec(source))) {
    const imports=match[1];
    if(imports===undefined)throw Error("Matched named import clause required");
    for (const part of imports.split(",")) {
      const trimmed = part.trim();
      if (!trimmed) continue;
      const name = trimmed
        .replace(/^type\s+/, "")
        .split(/\s+as\s+/)[0]
        ?.trim();
      if (name) names.push(name);
    }
  }
  return names;
}

/** 🔁️ Named import specifiers duplicated within the same module import block(s). */
export function duplicateNamedImportsForModule(source: string, moduleId: string): string[] {
  const names = namedImportSpecifiersForModule(source, moduleId);
  const seen = new Set<string>();
  const dupes: string[] = [];
  for (const name of names) {
    if (seen.has(name)) dupes.push(name);
    else seen.add(name);
  }
  return dupes;
}
