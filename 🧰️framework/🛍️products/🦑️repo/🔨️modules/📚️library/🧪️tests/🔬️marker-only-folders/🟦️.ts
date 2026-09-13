import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import Ajv from "ajv";
import { CLEAN_CACHE_DIR_NAME } from "../../🧼️workspace-cleanup/🛡️protection/🟦️.ts";
import { cleanCollectMarkerOnlyFolderRemovals, cleanDirectoryTreeIsOnlyEmptyMarkers, cleanEmptyMarkerFilenames } from "../../🧼️workspace-cleanup/🔍️marker-only-folders/🟦️.ts";

type Fixture = Readonly<{
  version: number;
  trees: readonly Readonly<{ id: string; expectRemoval: boolean; root: string; paths: readonly Readonly<{ path: string; bytes: string }>[] }>;
}>;

const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧼️marker-only-folders/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🧼️marker-only-folders/🔣️.json"), "utf8"));

function materialize(tree: Fixture["trees"][number]): string {
  const root = mkdtempSync(join(tmpdir(), "semio-marker-only-folders-"));
  for (const leaf of tree.paths) {
    const abs = join(root, leaf.path);
    mkdirSync(dirname(abs), { recursive: true });
    writeFileSync(abs, leaf.bytes);
  }
  return root;
}

describe("marker-only folder clean discovery", () => {
  test("validates the portable fixture contract", () => {
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    expect(validate({ ...fixture, version: 2 })).toBe(false);
  });

  test("collects ticket important folders under repo meta when only the cache subtree is protected", () => {
    const tree = {
      root: ".🧬semio/🦑️repo/🎫️tickets/TICKET/📌️important",
      paths: [
        { path: ".🧬semio/🦑️repo/⚡️cache/📌️.empty.md", bytes: "" },
        { path: ".🧬semio/🦑️repo/🎫️tickets/TICKET/📌️important/📝️.md", bytes: "" },
        { path: ".🧬semio/🦑️repo/🧷️anchor.txt", bytes: "keep" },
        { path: ".🧬semio/🦑️repo/🎫️tickets/TICKET/🧷️ticket.txt", bytes: "keep" },
      ],
    };
    const root = mkdtempSync(join(tmpdir(), "semio-marker-only-folders-"));
    try {
      for (const leaf of tree.paths) {
        const abs = join(root, leaf.path);
        mkdirSync(dirname(abs), { recursive: true });
        writeFileSync(abs, leaf.bytes);
      }
      const protectedPrefixes = [join(root, ".🧬semio/🦑️repo", CLEAN_CACHE_DIR_NAME)];
      const removals = cleanCollectMarkerOnlyFolderRemovals(root, protectedPrefixes).map((row) => row.path);
      expect(removals).toContain(tree.root);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("matches an independent oracle for every fixture tree", () => {
    const names = cleanEmptyMarkerFilenames();
    for (const tree of fixture.trees) {
      const root = materialize(tree);
      try {
        const target = join(root, tree.root);
        const oracle = cleanDirectoryTreeIsOnlyEmptyMarkers(target, names);
        const removals = new Set(cleanCollectMarkerOnlyFolderRemovals(root, []).map((row) => row.path));
        const removed = removals.has(tree.root) || [...removals].some((path) => tree.root.startsWith(`${path}/`));
        expect(oracle).toBe(tree.expectRemoval);
        expect(removed).toBe(tree.expectRemoval);
        console.info("[DEBUG] marker-only folder vector", JSON.stringify({ id: tree.id, oracle, removed, removals: [...removals] }));
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
    }
  });
});
