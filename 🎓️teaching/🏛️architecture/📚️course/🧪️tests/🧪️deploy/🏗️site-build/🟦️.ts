/** 🏗️ The site build contract that holds on any manifest: the compiler budget is derived from the neutral budget mechanism, the type-check state never names a dependency store, the Vite cache directory comes only from the framework and the artifact never carries the source revision.
 * @see ../../../🏗️builder/⏱️budget/🧬️schema/🔣️.json — the budget fixture contract */
import Ajv from "ajv";
import ts from "typescript";
import { readFileSync, readdirSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { expect, it } from "vitest";
import budgetSchema from "../../../🏗️builder/⏱️budget/🧬️schema/🔣️.json" with { type: "json" };
import budgetFixture from "../../../🏗️builder/⏱️budget/🧫️fixtures/🔣️.json" with { type: "json" };
import processBudget from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🧬️schema/🔣️json/🟦️.ts" with { type: "json" };
import { courseBudgetRequestMs } from "../../../🏗️builder/⏱️budget/🟦️.ts";
import { repoRoot, site } from "../🧰️deploy-fixtures/🟦️.ts";

const bundle = join(site, "📦️packages/🟦️typescript");
const sources = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap(entry => entry.name === "node_modules" || entry.name === "dist" ? [] : entry.isDirectory() ? sources(join(directory, entry.name)) : /\.tsx?$/u.test(entry.name) ? [join(directory, entry.name)] : []);

/** 🧪️ Registers the budget, type-check state, cache directory and revision tests. */
export function registerSiteBuildTests(): void {
  it("validates every native CI case as closed input and expected-result data", () => {
    const directory = join(site, "🧫️fixtures/🚀️readiness/🛗️native");
    const corpus = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(directory, "🧬️schema/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true, allErrors: true });
    const valid = ajv.compile(schema);
    expect(valid(corpus), JSON.stringify(valid.errors)).toBe(true);
    expect(corpus.cases).toHaveLength(17);
    expect(new Set(corpus.cases.map((row: { id: string }) => row.id)).size).toBe(17);
    const validCase = ajv.compile(schema.properties.cases.items);
    for (const row of corpus.cases) {
      expect(validCase(row), row.id).toBe(true);
      expect(validCase({ ...row, unknown: true }), row.id).toBe(false);
      expect(validCase({ ...row, expected: "passed" }), row.id).toBe(false);
      const missing = { ...row };
      delete missing.expected;
      expect(validCase(missing), row.id).toBe(false);
    }
  });
  it("derives every course compiler budget from the neutral budget mechanism", () => {
    const ajv = new Ajv({ strict: true });
    expect(ajv.validate(budgetSchema, budgetFixture), JSON.stringify(ajv.errors)).toBe(true);
    const milliseconds = ajv.compile({ ...processBudget, $ref: "#/$defs/Milliseconds" });
    for (const row of budgetFixture.cases) {
      if (row.requestedMilliseconds === null) expect(() => courseBudgetRequestMs(row.environment), row.id).toThrow();
      else {
        expect(courseBudgetRequestMs(row.environment), row.id).toBe(row.requestedMilliseconds);
        expect(milliseconds(row.requestedMilliseconds), row.id).toBe(true);
      }
    }
    const router = readFileSync(join(bundle, "📜️script.ts"), "utf8");
    expect(router).not.toMatch(/timeoutMs:\s*\d/u);
    expect(router).toContain("timeoutMs: courseBudgetMs(this.invocation)");
  });
  it("keeps the incremental type-check state out of every dependency store", () => {
    const config = ts.readConfigFile(join(bundle, "tsconfig.json"), ts.sys.readFile);
    const parsed = ts.parseJsonConfigFileContent(config.config, ts.sys, bundle);
    expect(parsed.options.tsBuildInfoFile).toBeUndefined();
    expect(JSON.stringify(config.config.compilerOptions)).not.toContain("node_modules");
    const runner = readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🟦️.ts"), "utf8");
    expect(runner).toContain('"--incremental", "true", "--tsBuildInfoFile", join(directory, "state.tsbuildinfo")');
  });
  it("takes the Vite cache directory only from the framework, never from the course or the ambient shell", async () => {
    const owners = sources(site).filter(path => readFileSync(path, "utf8").includes("SEMIO_VITE_CACHE_DIRECTORY")).map(path => relative(site, path).split("\\").join("/"));
    expect(owners).toEqual(["🧪️tests/🧪️deploy/🏗️site-build/🟦️.ts"]);
    const configure = (await import("../../../🏗️builder/🌐️vite/🟦️.ts")).default as unknown as (environment: { command: string; mode: string }) => unknown;
    const before = process.env.SEMIO_VITE_CACHE_DIRECTORY;
    delete process.env.SEMIO_VITE_CACHE_DIRECTORY;
    try {
      expect(() => configure({ command: "build", mode: "prototype" })).toThrow("original bounded compiler cache");
      process.env.SEMIO_VITE_CACHE_DIRECTORY = "/ambient/elsewhere";
      expect(() => configure({ command: "build", mode: "prototype" })).toThrow("Canonical bounded Vite compiler cache path");
    } finally {
      if (before === undefined) delete process.env.SEMIO_VITE_CACHE_DIRECTORY;
      else process.env.SEMIO_VITE_CACHE_DIRECTORY = before;
    }
  });
  it("hints at low priority exactly the lazy chunks of the course document and the quizzes it names, and nothing the entry runs", async () => {
    const { courseMaterialPreloadVitePlugin } = await import("../../../🏗️builder/🌐️vite/🟦️.ts");
    const plugin = courseMaterialPreloadVitePlugin();
    type Tag = { readonly tag: string; readonly attrs: Record<string, string | boolean>; readonly injectTo: string };
    type Output = { readonly type: string; readonly fileName: string; readonly facadeModuleId?: string | null };
    const hook = plugin.transformIndexHtml as unknown as { readonly handler: (html: string, context: { readonly path: string; readonly filename: string; readonly bundle?: Record<string, Output> }) => readonly Tag[] };
    const course = JSON.parse(readFileSync(join(site, "🔣️.json"), "utf8")) as { readonly quizzes: readonly string[] };
    const documents = ["🔣️.json", ...course.quizzes].map((path) => resolve(site, path));
    const chunks = documents.map((id, index) => [`assets/m${index}`, { type: "chunk", fileName: `assets/material-${index}.js`, facadeModuleId: id }] as const);
    const bundle = { "assets/entry": { type: "chunk", fileName: "assets/entry.js", facadeModuleId: join(site, "🟦️.ts") }, ...Object.fromEntries(chunks), "assets/style": { type: "asset", fileName: "assets/style.css" }, "assets/pets": { type: "chunk", fileName: "assets/pets.js", facadeModuleId: join(site, "../🐾️pets/🟦️.ts") } };
    expect(plugin.apply).toBe("build");
    expect(hook.handler("", { path: "/index.html", filename: "index.html", bundle })).toEqual(chunks.map(([, chunk]) => ({ tag: "link", attrs: { rel: "modulepreload", fetchpriority: "low", crossorigin: true, href: `/${chunk.fileName}` }, injectTo: "head" })));
    expect(hook.handler("", { path: "/index.html", filename: "index.html" })).toEqual([]);
  });
  it("keeps the source revision out of the site build and its artifact", () => {
    const readers = sources(site).filter(path => /\bsourceRevision\b/u.test(readFileSync(path, "utf8")) && !path.includes("🧪️tests")).map(path => relative(site, path).split("\\").join("/")).sort();
    expect(readers).toEqual(["🚀️deploy/🐳️docker-image/🟦️.ts", "🚀️deploy/🟦️.ts"]);
  });
}
