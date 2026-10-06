/** 🧫️ Verifies exact source projection against an independent lodash object overlay. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { omit } from "lodash";
import Ajv from "ajv/dist/2020.js";
import { SourceProjection, SourceProjectionError } from "../🟦️.ts";
const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));

function oracle(row: any) {
  const seen = new Set<string>();
  const refuse = (code: string, path: string) => ({ refusal: { code, path } });
  for (const change of row.changes) {
    if (!change.path || /[\\:\u0000]/u.test(change.path) || change.path.split("/").some((part: string) => !part || part === "." || part === "..")) return refuse("invalid-path", change.path);
    if (seen.has(change.path)) return refuse("duplicate-change", change.path);
    seen.add(change.path);
    if (change.before === null && change.after === null) return refuse("empty-change", change.path);
  }
  for (const change of row.changes) if ((Object.hasOwn(row.files, change.path) ? row.files[change.path] : null) !== change.before) return refuse("predecessor-mismatch", change.path);
  const deleted = row.changes.filter((change: any) => change.after === null).map((change: any) => change.path);
  const overlay = Object.assign(omit(row.files, deleted), Object.fromEntries(row.changes.filter((change: any) => change.after !== null).map((change: any) => [change.path, change.after])));
  for (const reference of row.references) {
    if (deleted.includes(reference.target)) return refuse("deleted-reference", reference.target);
    if (!Object.hasOwn(overlay, reference.target)) return refuse("missing-reference", reference.target);
  }
  return { files: Object.fromEntries(row.requests.map((path: string) => [path, Object.hasOwn(overlay, path) ? overlay[path] : null])), inverse: row.changes.map((change: any) => ({ path: change.path, before: change.after, after: change.before })) };
}

test("source projection preserves explicit deletion, predecessors and inverse bodies", () => {
  for (const row of fixture.cases) {
    let actual: unknown;
    try {
      const reads: string[] = [];
      const read = (path: string): string | null => { reads.push(path); return Object.hasOwn(row.files, path) ? row.files[path] : null; };
      const projection = new SourceProjection(row.changes, read);
      for (const reference of row.references) projection.require(reference.target);
      actual = { files: Object.fromEntries(row.requests.map((path: string) => [path, projection.resolve(path)])), inverse: projection.inverse() };
      for (const change of row.changes) expect(reads.filter(path => path === change.path)).toHaveLength(1);
    } catch (error) {
      if (!(error instanceof SourceProjectionError)) throw error;
      actual = { refusal: { code: error.code, path: error.path } };
    }
    expect(actual).toEqual(row.expected);
    expect(oracle(row)).toEqual(row.expected);
    console.log("[DEBUG] source projection case " + JSON.stringify({ id: row.id, expected: row.expected, own: actual, independent: oracle(row) }));
  }
  const change: { path: string; before: string | null; after: string | null } = { path: "a.rs", before: "old", after: null };
  const projection = new SourceProjection([change], () => "old");
  change.after = "foreign";
  expect(projection.resolve("a.rs")).toBeNull();
  console.log(`[DEBUG] source projection: ${fixture.cases.length} shared cases retained; deletion never reads physical fallback`);
});
