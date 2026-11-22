import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import Ajv from "ajv";
import enhancedResolve from "enhanced-resolve";
import { observeDetachedSyncV1 } from "../../../../../👁️observation/🟨️.mjs";
import { typescriptCompilerFileV1 } from "../../../../../🚀️bootstrap/🔷️typescript-compiler/🟨️.mjs";
import { readPhysicalSource } from "../../../📍️coordinates/🟨️.mjs";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const sourceAnalyses = await import(new URL("../../🟨️.mjs", import.meta.url).href) as any;

test("compiler source capture follows the live generation after dependency publication", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  const parent = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();
  mkdirSync(parent, { recursive: true });
  for (const row of fixture.cases) {
    const root = mkdtempSync(join(realpathSync(parent), "compiler-swap-"));
    try {
      const parserDirectory = join(root, "parser"), binding = join(root, "node_modules");
      mkdirSync(parserDirectory);
      writeFileSync(join(parserDirectory, "package.json"), JSON.stringify({ devDependencies: { typescript: fixture.compiler.version } }));
      const createGeneration = (id: string) => {
        const generation = join(root, ".cache", "b", "d", "m", "j", id);
        const modules = join(generation, "node_modules"), compiler = join(modules, "typescript");
        const main = join(compiler, fixture.compiler.main);
        mkdirSync(dirname(main), { recursive: true });
        writeFileSync(join(compiler, "package.json"), JSON.stringify({ name: "typescript", version: fixture.compiler.version, main: fixture.compiler.main }));
        writeFileSync(main, fixture.compiler.source);
        return { generation, modules, main };
      };
      const retired = createGeneration(fixture.generations.retired), published = createGeneration(fixture.generations.published);
      symlinkSync(retired.modules, binding, "junction");
      const select = () => typescriptCompilerFileV1({ root, parserDirectory });
      expect(select()).toBe(retired.main);
      const require = createRequire(join(root, "fixture.cjs"));
      expect(require.resolve("typescript")).toBe(retired.main);
      const publish = () => {
        rmSync(binding);
        symlinkSync(published.modules, binding, "junction");
        rmSync(retired.generation, { recursive: true });
      };
      if (row.change === "before-capture") publish();
      const reads: string[] = [];
      const read = (path: string) => {
        reads.push(path);
        if (reads.length === 1) {
          if (row.change === "during-read") publish();
          if (row.change === "remove-entry") rmSync(retired.main);
        }
        return readPhysicalSource(path);
      };
      const capture = () => observeDetachedSyncV1(() => sourceAnalyses.readCurrentCompilerSourceV1(select, read));
      if (row.expected === "ENOENT") {
        expect(capture, row.name).toThrow(expect.objectContaining({ code: "ENOENT", path: retired.main }));
        expect(reads).toEqual([retired.main]);
      } else {
        const result = capture(), rows = result.rows.concat(result.layers.flat());
        expect(result.value.toString(), row.name).toBe(fixture.compiler.source);
        const freshResolve = enhancedResolve.create.sync({ mainFields: ["main"], symlinks: true });
        expect(freshResolve(root, "typescript")).toBe(published.main);
        expect(reads).toEqual(row.change === "during-read" ? [retired.main, published.main] : [published.main]);
        expect(rows.some(row => row.path.includes(fixture.generations.retired))).toBe(false);
        expect(rows.some(row => row.path === published.main)).toBe(true);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }
});
