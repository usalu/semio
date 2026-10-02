import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import glob from "fast-glob";
import { parse as parseToml } from "@iarna/toml";
import { inspectRustModuleGraph } from "../../../🔍️discovery/🟦️.ts";
import { mutationTaxonomyResolveRustRoute, type MutationTaxonomyRustRoute } from "../../../🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts";

const library = resolve(import.meta.dir, "../../..");
const corpus = JSON.parse(readFileSync(join(library, "🧫️fixtures/🧱️rust-source-direction/🔗️participation/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(library, "🧬️schema/🧱️rust-source-direction/🔗️participation/🔣️.json"), "utf8"));
type Row = { id: string; files: Record<string, string>; alternate?: Record<string, string>; sourcePath: string; sourceScope: readonly string[]; specifier: string; unavailable: string | null; expected: MutationTaxonomyRustRoute };
const rows = corpus.routes as readonly Row[];

test("participation route corpus is closed with unique authored identities", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(rows.map(row => row.id)).size).toBe(rows.length);
});

test("native physical observations cannot restore refused Cargo route authority", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust participation requires caller-owned output");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-participation-")));
  try {
    let next = 0;
    const failures: unknown[] = [];
    await Promise.all(Array.from({ length: 4 }, async () => {
      while (next < rows.length) {
        const row = rows[next++]!, cwd = join(root, row.id);
        try {
          for (const [path, source] of Object.entries(row.files)) { mkdirSync(dirname(join(cwd, path)), { recursive: true }); writeFileSync(join(cwd, path), source); }
          expect(glob.sync("**/*", { cwd, onlyFiles: true, followSymbolicLinks: false }).sort(), row.id).toEqual(Object.keys(row.files).sort());
          const manifests = Object.keys(row.files).filter(path => path.endsWith("Cargo.toml"));
          for (const path of manifests) {
            if (row.id === "malformed-manifest-is-not-orphan") expect(() => parseToml(row.files[path]!)).toThrow();
            else expect(typeof (parseToml(row.files[path]!).lib as { path: string }).path).toBe("string");
          }
          const child = Bun.spawn(["rustc", "--edition=2021", "--crate-type=lib", "--crate-name", "participation_probe", "--emit=dep-info=dependencies.d,link", "-o", "probe.rlib", row.sourcePath], { cwd, stdout: "pipe", stderr: "pipe" });
          const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
          expect(status, row.id + stdout + stderr).toBe(0);
          const inputs = /^probe\.rlib: ([^\r\n]+)$/mu.exec(readFileSync(join(cwd, "dependencies.d"), "utf8"));
          expect(inputs, row.id).not.toBeNull();
          expect([...new Set(inputs![1]!.split(" ").filter(Boolean))].sort(), row.id).toEqual(["leaf.rs", "source.rs"]);
          writeFileSync(join(root, row.id + ".receipt.json"), JSON.stringify({ id: row.id, status, inputs: inputs![1] }));
        } catch (error) { failures.push(error); }
      }
    }));
    if (failures.length) throw new AggregateError(failures, "Independent native participation oracles failed");
    expect(typeof mutationTaxonomyResolveRustRoute).toBe("function");
    for (const row of rows) {
      for (const files of [row.files, ...(row.alternate ? [{ ...row.files, ...row.alternate }] : [])]) {
        const contents = new Map(Object.entries(files).filter(([path]) => path !== row.unavailable));
        const graph = inspectRustModuleGraph(Object.keys(files), path => contents.get(path), { conventionalRoots: true, strictManifests: true });
        expect(mutationTaxonomyResolveRustRoute(row.sourcePath, row.specifier, row.sourceScope, graph, contents), row.id).toEqual(row.expected);
      }
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);
