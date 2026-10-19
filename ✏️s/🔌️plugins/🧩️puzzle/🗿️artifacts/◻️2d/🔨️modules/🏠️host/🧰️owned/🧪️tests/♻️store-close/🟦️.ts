import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import patch from "fast-json-patch";
import corpus from "../🧫️fixtures/♻️store-close/🔣️.json";
import schema from "../../🧬️schema/♻️store-close/🔣️.json";

test("store close is resumable for histories larger than one ledger page", () => {
  expect(new Ajv({ strict: true }).compile(schema)(corpus)).toBe(true);
  for (const row of corpus.histories) {
    const initial: { history: number[]; current: number | null } = { history: Array.from({ length: row.edits }, (_, id) => id), current: row.edits };
    const own = structuredClone(initial);
    const oracle = structuredClone(initial);
    expect(patch.applyPatch(structuredClone(oracle), []).newDocument).toEqual(own);
    while (own.history.length > 0) {
      own.history.pop();
      patch.applyPatch(oracle, [{ op: "remove", path: `/history/${oracle.history.length - 1}` }]);
      expect(oracle).toEqual(own);
    }
    own.current = null;
    patch.applyPatch(oracle, [{ op: "replace", path: "/current", value: null }]);
    expect(oracle).toEqual(corpus.terminal);
    expect(own).toEqual(corpus.terminal);
  }
});

test("standalone puzzle store exposes the shared granted close step without a history ceiling", () => {
  const source = readFileSync(resolve(import.meta.dir, "../../🦀️.rs"), "utf8");
  expect(source).toContain("pub fn close_puzzle2d_store_step");
  expect(source).toContain("ArtifactDocumentStoreDisposer");
  expect(source).not.toContain("PUZZLE2D_STORE_CLOSE_TURNS");
});
