import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { join, relative, resolve } from "node:path";
import { readFileSync } from "node:fs";
import { canvasRuntimeAssetSources, runtimeAssetStageTrees } from "../🟦️.ts";
import fixture from "./🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

test("neutral Canvas assets have one stable managed tree per mode and no source-tree publication", () => {
  const root = resolve("/products"), validate = new Ajv({ strict: true }).compile(schema), database = new Database(":memory:"), roots = new Set<string>();
  try {
    for (const mode of fixture.modes) {
      expect(validate({ version: fixture.version, mode })).toBe(true);
      const trees = (runtimeAssetStageTrees as (...args: unknown[]) => ReturnType<typeof runtimeAssetStageTrees>)(root, mode);
      expect(trees.map(tree => relative(join(root, fixture.namespace, mode), tree.root).replaceAll("\\", "/"))).toEqual(fixture.trees);
      expect(trees.map(tree => tree.root)).toEqual((canvasRuntimeAssetSources as (...args: unknown[]) => ReturnType<typeof canvasRuntimeAssetSources>)(root, mode).map(tree => tree.root));
      for (const tree of trees) {
        roots.add(tree.root);
        expect((database.query("select instr(?1, ?2)=1 and instr(?1, 'dist')=0 confined").get(tree.root, root) as { confined: number }).confined).toBe(1);
        expect(tree.root.length).toBeLessThanOrEqual(224);
        expect(tree.files.every(file => join(tree.root, file.name).length <= 256)).toBe(true);
      }
    }
    expect(roots.size).toBe(fixture.modes.length * fixture.trees.length);
    for (const mode of fixture.invalidModes) {
      expect(validate({ version: 1, mode })).toBe(false);
      expect(() => (runtimeAssetStageTrees as (...args: unknown[]) => unknown)(root, mode)).toThrow();
    }
    for (const directory of fixture.invalidRoots) expect(() => (runtimeAssetStageTrees as (...args: unknown[]) => unknown)(directory, "prototype")).toThrow();
  } finally { database.close(); }
});
