import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import ts from "typescript";
import { createRequire } from "node:module";
import { readUiAxes } from "../../🎚️axes/📥️source/🟦️.ts";
import { emitUiAxesRust, emitUiAxesTypeScript } from "../../🎚️axes/📽️projection/🟦️.ts";
import { uiAxesPreview, uiAxesTargets } from "../../🎚️axes/📋️plan/🟦️.ts";
import { publishUiAxes, staleUiAxesTargets } from "../../🎚️axes/📤️publication/🟦️.ts";

const repoRoot = join(import.meta.dir, "../../../../..");

describe("UI axes source and projections", () => {
  test("the axes command producer has only neutral runtime inputs", async () => {
    const source = join(import.meta.dir, "../../🎚️axes/🏃️execution/🟦️.ts");
    const syntax = ts.createSourceFile(source, readFileSync(source, "utf8"), ts.ScriptTarget.Latest, true);
    const imports = syntax.statements.filter(ts.isImportDeclaration).map(row => ts.isStringLiteral(row.moduleSpecifier) ? row.moduleSpecifier.text : "");
    expect(imports.every(path => !path.includes("🛍️products"))).toBe(true);
    const projection = await createRequire(import.meta.url)("esbuild").build({ entryPoints: [source], bundle: true, write: false, metafile: true, platform: "node", format: "esm" });
    expect(Object.keys(projection.metafile.inputs).every(path => !path.includes("🛍️products"))).toBe(true);
    expect(uiAxesTargets(repoRoot, readUiAxes(repoRoot)).every(target => !target.path.includes("🛍️products"))).toBe(true);
  });
  test("the neutral source defines the complete 2×2 locale and terminology axes", () => {
    const axes = readUiAxes(repoRoot);
    expect(axes.locales.map(({ id, variant }) => [id, variant])).toEqual([["en", "En"], ["de", "De"]]);
    expect(axes.terminologies.map(({ id, variant }) => [id, variant])).toEqual([["native", "Native"], ["reuse", "Reuse"]]);
    expect(new Set(axes.locales.map(({ id }) => id)).size).toBe(2);
    expect(new Set(axes.terminologies.map(({ id }) => id)).size).toBe(2);
  });

  test("Rust and TypeScript projections preserve the same ordered axes", () => {
    const axes = readUiAxes(repoRoot);
    const rust = emitUiAxesRust(axes);
    const typescript = emitUiAxesTypeScript(axes);
    for (const value of ["en", "de", "native", "reuse"]) {
      expect(rust).toContain(JSON.stringify(value));
      expect(typescript).toContain(JSON.stringify(value));
    }
    const transpiled = ts.transpileModule(typescript, { compilerOptions: { target: ts.ScriptTarget.Latest }, fileName: "🟦️.ts", reportDiagnostics: true });
    expect(transpiled.diagnostics ?? []).toEqual([]);
  });

  test("preview, freshness and publication share one exact two-file plan", () => {
    const output = process.env.SEMIO_TEST_ARTIFACT_DIR!;
    expect(output).toBeTruthy();
    mkdirSync(output, { recursive: true });
    const root = mkdtempSync(join(output, "ui-axes-"));
    try {
      const axes = readUiAxes(repoRoot);
      const targets = uiAxesTargets(root, axes);
      expect(targets.map(({ path }) => path)).toEqual([
        join(root, "🧰️framework/🔨️modules/🖱️ui/🎚️axes/🤖️generated/🦀️.rs"),
        join(root, "🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts"),
      ]);
      expect(uiAxesPreview(root, targets).nodes).toHaveLength(2);
      expect(staleUiAxesTargets(targets)).toEqual(targets.map(({ path }) => path));
      publishUiAxes(targets);
      expect(staleUiAxesTargets(targets)).toEqual([]);
      expect(readFileSync(targets[0]!.path, "utf8")).toBe(targets[0]!.content);
      expect(readFileSync(targets[1]!.path, "utf8")).toBe(targets[1]!.content);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("the registered producer and both implementation consumers name the semantic owners", () => {
    const project = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📋️project.json"), "utf8")) as { namedInputs: { default: string[] } };
    const requiredInputs = ["🎚️axes/🔣️.json", "🎚️axes/📥️source/🟦️.ts", "🎚️axes/📽️projection/🟦️.ts", "🎚️axes/📋️plan/🟦️.ts", "🎚️axes/📤️publication/🟦️.ts", "🎚️axes/🏃️execution/🟦️.ts", "🧪️tests/🎚️axes/🟦️.ts"];
    for (const suffix of requiredInputs) expect(project.namedInputs.default.some((path) => path.endsWith(suffix))).toBe(true);
    const rustConsumer = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌐️locale/🦀️.rs"), "utf8");
    const rustFacade = readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs"), "utf8");
    const typescriptConsumer = readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🛂️manifest/🟦️.ts"), "utf8");
    expect(rustConsumer).toContain("../../../../🔨️modules/🖱️ui/🎚️axes/🤖️generated/🦀️.rs");
    expect(rustFacade).toContain("pub use dsl::{AppLabels, Label, LabelText, Locale, LocalizedLabel, Terminology};");
    expect(typescriptConsumer).toContain("./🤖️generated/🎚️ui-axes/🟦️.ts");
    const taxonomy = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as { generatorContracts: Record<string, { ownerPath: string; inputPatterns: string[] }> };
    const contract = taxonomy.generatorContracts["ui-axes"]!;
    expect(contract.ownerPath).toBe("🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust");
    for (const suffix of requiredInputs) expect(contract.inputPatterns.some((path) => path.endsWith(suffix))).toBe(true);
  });
});
