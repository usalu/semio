import { expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync, realpathSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import glob from "fast-glob";
import { fixtureFacadeTargets } from "@semio-tech/framework/test/source-text";
import { testBuiltTreeRetirementFixture } from "../../🧬️contract/🏗️builder/♻️retirement/🧪️tests/🔬️built-tree-retirement/🟦️.ts";

import { fixedListStorageSelfTests } from "../../🧬️contract/🧪️tests/🔬️fixed-list-storage/🟦️.ts";
import { surfaceOwnershipSelfTests } from "../../🧠️runtime/🧪️tests/🔬️surface-ownership/🟦️.ts";

const ui = resolve(import.meta.dir, "../..");
const walk = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap(entry => entry.isDirectory() ? walk(join(directory, entry.name)) : entry.isFile() ? [join(directory, entry.name)] : []);
const files = walk(ui);
const parts = (file: string): string[] => file.split(/[\\/]/u);

test("UI testing example folders belong beneath actual tests folders", () => {
  const actual = files.filter(file => parts(file).includes("🧫️fixtures")).sort();
  const oracle = glob.sync("**/🧫️fixtures/**/*", { cwd: ui, absolute: true, onlyFiles: true }).map(file => resolve(file)).sort();
  expect(actual).toEqual(oracle);
  expect(actual.length).toBeGreaterThan(400);
  for (const file of actual) {
    const targets = fixtureFacadeTargets(file);
    if (targets !== undefined && targets.every(target => parts(realpathSync(target)).includes("🧪️tests"))) continue;
    const path = parts(realpathSync(file));
    expect(path.indexOf("🧪️tests")).toBeGreaterThan(-1);
    expect(path.indexOf("🧪️tests")).toBeLessThan(path.indexOf("🧫️fixtures"));
    expect([...file].length).toBeLessThanOrEqual(256);
    expect(file.length).toBeLessThanOrEqual(256);
  }
});

test("UI native testing example includes resolve to physical test owners", () => {
  let count = 0;
  for (const file of files.filter(file => file.endsWith(".rs"))) {
    const source = readFileSync(file, "utf8");
    for (const match of source.matchAll(/include_(?:str|bytes)!\(\s*"([^"]*🧫️fixtures[^"]*)"\s*\)/gu)) {
      const target = resolve(dirname(file), match[1]!);
      expect(existsSync(target)).toBe(true);
      const coordinate = relative(ui, realpathSync(target));
      if (!coordinate.startsWith("..") && !isAbsolute(coordinate)) expect(parts(realpathSync(target))).toContain("🧪️tests");
      expect(parts(file)).toContain("🧪️tests");
      count++;
    }
  }
  expect(count).toBeGreaterThan(50);
});

test("UI moved built retirement examples retain their original behavior", () => {
  testBuiltTreeRetirementFixture();
});

test("UI direct TypeScript example URLs resolve to physical test owners", () => {
  let count = 0;
  for (const file of files.filter(file => file.endsWith(".ts") || file.endsWith(".tsx"))) {
    const source = readFileSync(file, "utf8");
    for (const match of source.matchAll(/new URL\(\s*["']([^"']*🧫️fixtures[^"']*)["']\s*,\s*import.meta.url\s*\)/gu)) {
      const target = resolve(dirname(file), match[1]!);
      expect(existsSync(target)).toBe(true);
      expect(parts(file)).toContain("🧪️tests");
      const coordinate = relative(ui, realpathSync(target));
      if (!coordinate.startsWith("..") && !isAbsolute(coordinate)) expect(parts(realpathSync(target))).toContain("🧪️tests");
      count++;
    }
  }
  expect(count).toBeGreaterThan(20);
});

test("UI moved storage and surface examples retain their authored oracles", () => {
  const list = fixedListStorageSelfTests(), surface = surfaceOwnershipSelfTests();
  expect(list).toBe(75);
  expect(surface).toBeGreaterThan(32);
});
