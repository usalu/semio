import { dirname, join, resolve } from "node:path";
import { existsSync, readFileSync } from "node:fs";
import { type RustSourceItem, useLeafNames } from "../../🌳️module-tree/🟦️.ts";
import { runtimeParseTomlV1 } from "../../../../../../../../🔨️modules/⚙️runtime/📄️parse/🟦️.ts";

/** 🔍️ Finds the Cargo package whose library owns a directory: the nearest ancestor package, or the artifact package of the nearest workspace root. */
export function resolveOwningCrate(repoRoot: string, directoryRel: string): { manifest: string; name: string; entry: string } | null {
  let cursor = resolve(repoRoot, directoryRel);
  const root = resolve(repoRoot);
  while (cursor.startsWith(root) && cursor !== dirname(root)) {
    const manifest = join(cursor, "Cargo.toml");
    if (existsSync(manifest)) {
      const text = readFileSync(manifest, "utf8");
      if (/^\[package\]/mu.test(text)) { const lib = readCargoLib(manifest); return { manifest, ...lib }; }
      const packageManifest = join(cursor, "📦️packages/🦀️rust/Cargo.toml");
      if (/^\[workspace\]/mu.test(text) && existsSync(packageManifest)) { const lib = readCargoLib(packageManifest); return { manifest: packageManifest, ...lib }; }
    }
    const parent = dirname(cursor);
    if (parent === cursor) break;
    cursor = parent;
  }
  return null;
}

/** 🔍️ Reads the lib package name and entry file of a Cargo manifest. */
export function readCargoLib(manifestPath: string): { name: string; entry: string } {
  const parsed = runtimeParseTomlV1(readFileSync(manifestPath, "utf8")) as { package?: { name?: string }; lib?: { path?: string } };
  if (!parsed.package?.name) throw new Error(`Cargo manifest has no package name: ${manifestPath}`);
  return { name: parsed.package.name, entry: resolve(dirname(manifestPath), parsed.lib?.path ?? "src/lib.rs") };
}

/** 📝️ Collects `extern crate X as Y;` rows of a root source. */
export function externCrateRows(source: string, items: readonly RustSourceItem[]): { crate: string; alias: string; start: number; end: number }[] {
  const rows: { crate: string; alias: string; start: number; end: number }[] = [];
  for (const item of items) {
    if (item.kind !== "extern-crate") continue;
    const match = /extern\s+crate\s+([A-Za-z0-9_]+)(?:\s+as\s+([A-Za-z0-9_]+))?\s*;/u.exec(source.slice(item.declStart, item.end));
    if (match) rows.push({ crate: match[1]!, alias: match[2] ?? match[1]!, start: item.start, end: item.end });
  }
  return rows;
}

/** 🏷️ Names defined or imported at the top level of a module body, with their items. */
export function topLevelNames(source: string, items: readonly RustSourceItem[]): Map<string, RustSourceItem> {
  const names = new Map<string, RustSourceItem>();
  for (const item of items) {
    if (item.name && item.kind !== "use" && item.kind !== "extern-crate" && item.kind !== "impl") names.set(item.name, item);
    if (item.kind === "use") for (const leaf of useLeafNames(source.slice(item.declStart, item.end))) names.set(leaf, item);
  }
  return names;
}
