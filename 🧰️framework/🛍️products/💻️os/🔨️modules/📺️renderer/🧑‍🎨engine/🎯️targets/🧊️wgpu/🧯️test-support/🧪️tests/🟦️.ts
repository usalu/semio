/** 🧪️ Proves the closed fixture of the Rust `test-support` module tree reader against the independent TypeScript `readModuleTreeSource`: the expectations the Rust tests assert are what the TypeScript helper computes. */
import { afterAll, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { inspectRustModuleChildren, listModuleTree, readModuleTreeSource } from "../../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";

const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../../🧪️tests/🧫️fixtures/🧊️wgpu/🧯️test-support/🔣️.json"), "utf8"));
const scratch = mkdtempSync(join(tmpdir(), "semio-test-support-"));
afterAll(() => rmSync(scratch, { recursive: true, force: true }));

function materialize(id: string, files: Record<string, string>): string {
  const base = join(scratch, id);
  for (const [path, text] of Object.entries(files)) {
    mkdirSync(dirname(join(base, path)), { recursive: true });
    writeFileSync(join(base, path), text);
  }
  return base;
}

test("children equal the independent TypeScript inspection", () => {
  for (const row of fixture.children) expect(inspectRustModuleChildren(row.source), row.id).toEqual(row.expected);
});

test("ambiguous path attributes are refused by the TypeScript helper too", () => {
  for (const row of fixture.refusals) expect(() => inspectRustModuleChildren(row.source), row.id).toThrow(row.error);
});

test("trees list the same files and read the same text", () => {
  for (const row of fixture.trees) {
    const base = materialize(row.id, row.files), root = join(base, row.root);
    const listed = listModuleTree(root).map(path => path.slice(base.length + 1));
    expect(listed, row.id).toEqual(row.expected);
    expect(readModuleTreeSource(root), row.id).toBe(row.expected.map((path: string) => readFileSync(join(base, path), "utf8")).map((text: string) => text.endsWith("\n") ? text : text + "\n").join(""));
  }
});

test("missing mounts are refused by the TypeScript helper too", () => {
  for (const row of fixture.treeRefusals) expect(() => listModuleTree(join(materialize(row.id, row.files), row.root)), row.id).toThrow(row.error);
});
