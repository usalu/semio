/** 🧪️ Framework source-text access law: the fixture cases (`🧫️fixtures/🔣️.json`, language-agnostic) resolve to exactly the entries `fast-glob` finds below the framework root, refused paths throw, and reads return the bytes `node:fs` returns.
 * @see https://github.com/mrmlnc/fast-glob */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import fg from "fast-glob";
import { frameworkSourcePath, frameworkSourceUrl, readFrameworkSourceJson, readFrameworkSourceText } from "../🟦️.ts";

const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8")) as { root: string; accepted: string[]; refused: { id: string; path: string }[]; json: { path: string; field: string; value: string } };
const frameworkRoot = resolve(import.meta.dir, "../../../..");

test("the fixture root is the framework tree the resolver anchors on", () => {
  expect(frameworkRoot.endsWith("/" + fixture.root)).toBe(true);
});

test("accepted paths resolve inside the framework tree exactly where the oracle puts them", () => {
  for (const path of fixture.accepted) {
    const actual = frameworkSourcePath(path);
    const found = fg.sync(path.replace(/[\\*?[\]{}()!@+]/g, "\\$&"), { cwd: frameworkRoot, onlyFiles: false, absolute: true, dot: true, unique: true });
    expect({ path, found }).toEqual({ path, found: [actual] });
    expect({ path, actual }).toEqual({ path, actual: resolve(frameworkRoot, path) });
    expect({ path, url: fileURLToPath(frameworkSourceUrl(path)) }).toEqual({ path, url: actual });
  }
});

test("refused paths throw", () => {
  for (const row of fixture.refused) expect({ id: row.id, refused: (() => { try { frameworkSourcePath(row.path); return false; } catch { return true; } })() }).toEqual({ id: row.id, refused: true });
});

test("reads return the bytes node:fs returns", () => {
  const path = fixture.json.path;
  expect(readFrameworkSourceText(path)).toBe(readFileSync(resolve(frameworkRoot, path), "utf8"));
  expect((readFrameworkSourceJson(path) as Record<string, unknown>)[fixture.json.field]).toBe(fixture.json.value);
});
