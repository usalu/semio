/** 🧩️ The history and tool-run partition law of the plugin SDK: every named item has exactly one home among the files of its crate, the seams of the monolith mount each moved file once, and no home file names an item whose crate ranks above it (command below the panel, the panel below tool-run, tool-run below time-travel). */
import { test, expect } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import fixture from "../🧫️fixtures/🧩️history-partition.json" with { type: "json" };
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

const pluginRoot = join(import.meta.dir, "../../..");
type Item = { name: string; kind: string; crate: string };
const strip = (text: string): string => text.replace(/\/\*[\s\S]*?\*\//gu, "").replace(/\/\/[^\n]*/gu, "").replace(/"(?:[^"\\\n]|\\.)*"/gu, '""');
const trees = new Map<string, string>();
const read = (rel: string): string => {
  let text = trees.get(rel);
  if (text === undefined) {
    text = readModuleTreeSource(join(pluginRoot, rel));
    trees.set(rel, text);
  }
  return text;
};
const defines = (text: string, item: Item): boolean => new RegExp(`^[ \\t]*(?:pub(?:\\([a-z:]+\\))?\\s+)?(?:const\\s+|unsafe\\s+|async\\s+)*${item.kind}\\s+${item.name}\\b`, "mu").test(strip(text));
const mentions = (text: string, name: string): boolean => new RegExp(`(?<![A-Za-z0-9_])${name}(?![A-Za-z0-9_])`, "u").test(strip(text));

const homesOf = (items: Item[], homes: Record<string, string[]>): Map<string, string[]> => {
  const found = new Map<string, string[]>();
  for (const item of items) {
    const where: string[] = [];
    for (const [crate, files] of Object.entries(homes)) for (const file of files) if (defines(read(file), item)) where.push(crate + ":" + file);
    found.set(item.name, where);
  }
  return found;
};

const upwardReferences = (items: Item[], homes: Record<string, string[]>, rank: Record<string, number>, source: (file: string) => string = read): string[] => {
  const bad: string[] = [];
  for (const [crate, files] of Object.entries(homes)) {
    for (const file of files) {
      const text = source(file);
      for (const item of items) if (rank[item.crate]! > rank[crate]! && mentions(text, item.name)) bad.push(`${crate} (${file}) names ${item.name} of ${item.crate}`);
    }
  }
  return bad;
};

const misplaced = (items: Item[], homes: Record<string, string[]>): string[] => {
  const found = homesOf(items, homes);
  return items.filter((item) => found.get(item.name)!.length !== 1 || !found.get(item.name)![0]!.startsWith(item.crate + ":")).map((item) => item.name);
};

test("every named item is defined exactly once, in a file of its crate", () => {
  expect(misplaced(fixture.items, fixture.homes)).toEqual([]);
});

test("each crate root mounts each moved file once and publishes it as a public module", () => {
  for (const [parent, children] of Object.entries(fixture.mounts)) {
    const text = read(parent);
    for (const child of children) {
      const literal = `#[path = "${child}"]`;
      expect(text.split(literal).length - 1, `${parent} mounts ${child}`).toBe(1);
      expect(existsSync(normalize(join(pluginRoot, dirname(parent), child))), child).toBe(true);
    }
    expect((text.match(/^pub mod [a-z_]+;$/gmu) ?? []).length).toBeGreaterThanOrEqual(children.length);
  }
});

test("no home file names an item of a higher-ranked crate", () => {
  expect(upwardReferences(fixture.items, fixture.homes, fixture.rank)).toEqual([]);
});

test("the checker sees a seeded upward reference, a seeded misplaced item and a seeded duplicate home", () => {
  const command = fixture.homes.command![0]!;
  const seeded = (file: string): string => (file === command ? read(file) + "\nfn seeded() { time_travel_band_section(); }\n" : read(file));
  expect(upwardReferences(fixture.items, fixture.homes, fixture.rank, seeded)).toEqual([`command (${command}) names time_travel_band_section of time-travel-panel`]);
  const lowered = fixture.items.map((item) => (item.name === "history_mutation_entry" ? { ...item, crate: "time-travel-panel" } : item));
  expect(misplaced(lowered, fixture.homes)).toEqual(["history_mutation_entry"]);
  const doubled = { ...fixture.homes, "time-travel-panel": [...fixture.homes["time-travel-panel"]!, command] };
  expect(misplaced(fixture.items, doubled).length).toBeGreaterThan(5);
  expect(upwardReferences(fixture.items, fixture.homes, { ...fixture.rank, command: 99 }).length).toBeGreaterThan(0);
});
