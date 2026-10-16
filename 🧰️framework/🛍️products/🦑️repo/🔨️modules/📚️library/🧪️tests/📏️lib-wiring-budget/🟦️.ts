import { describe, expect, test } from "bun:test";
import fastGlob from "fast-glob";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { loadTaxonomy } from "../../🔍️discovery/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

type Ratchet = { schema: string; budget: number; entries: Record<string, number> };

let memo: Map<string, number> | null = null;

/** 📏️ Every plugin crate lib root with its physical line count. */
export function pluginLibRoots(root: string): Map<string, number> {
  if (memo) return memo;
  const roots = new Map<string, number>();
  const manifests = fastGlob.sync("✏️s/🔌️plugins/**/Cargo.toml", { cwd: root, ignore: ["**/node_modules/**", "**/target/**", "**/dist/**"], onlyFiles: true, followSymbolicLinks: false });
  for (const manifest of manifests) {
    const parsed = Bun.TOML.parse(readFileSync(join(root, manifest), "utf8")) as { lib?: { path?: string } };
    if (!parsed.lib?.path) continue;
    const lib = resolve(dirname(join(root, manifest)), parsed.lib.path).slice(root.length + 1);
    try { roots.set(lib, readFileSync(join(root, lib), "utf8").split("\n").length); } catch { continue; }
  }
  memo = roots;
  return roots;
}

describe("library wiring budget of plugin crates", () => {
  const root = getWorkspaceRoot();
  const ratchet = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json"), "utf8")) as Ratchet;
  const budget = loadTaxonomy().libWiringLineBudget;

  test("the ratchet is written for the taxonomy budget and sorted by byte order", () => {
    expect(ratchet.schema).toBe("semio.repo.lib-wiring-ratchet/v1");
    expect(ratchet.budget).toBe(budget);
    const keys = Object.keys(ratchet.entries);
    expect(keys).toEqual([...keys].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b))));
  });

  test("no plugin crate lib root exceeds the wiring budget unless the ratchet pins it, and pinned roots only shrink", () => {
    const roots = pluginLibRoots(root);
    const breaches: string[] = [];
    for (const [path, lines] of roots) {
      const pinned = ratchet.entries[path];
      if (lines <= budget) continue;
      if (pinned === undefined) breaches.push(`${path}: ${lines} lines exceed the ${budget}-line wiring budget; carve it with \`bun ./📜️script.ts new crates\` instead of growing the root`);
      else if (lines > pinned) breaches.push(`${path}: grew from ${pinned} to ${lines} lines`);
    }
    expect(breaches).toEqual([]);
  }, 120_000);

  test("the ratchet lists no root that complies, vanished or was not measured", () => {
    const roots = pluginLibRoots(root);
    const stale = Object.keys(ratchet.entries).filter((path) => !roots.has(path) || roots.get(path)! <= budget);
    expect(stale).toEqual([]);
  }, 120_000);
});
