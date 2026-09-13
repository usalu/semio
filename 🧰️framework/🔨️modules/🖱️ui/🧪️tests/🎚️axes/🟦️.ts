import { describe, expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import ts from "typescript";
import { readUiAxes } from "../../🎚️axes/📥️source/🟦️.ts";
import { emitUiAxesRust, emitUiAxesTypeScript } from "../../🎚️axes/📽️projection/🟦️.ts";
import { uiAxesPreview, uiAxesTargets } from "../../🎚️axes/📋️plan/🟦️.ts";
import { publishUiAxes, staleUiAxesTargets } from "../../🎚️axes/📤️publication/🟦️.ts";

const repoRoot = join(import.meta.dir, "../../../../..");

describe("UI axes source and projections", () => {
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
    const parsed = ts.createSourceFile("🟦️.ts", typescript, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    expect(parsed.parseDiagnostics).toEqual([]);
  });

  test("preview, freshness and publication share one exact two-file plan", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-ui-axes-"));
    try {
      const axes = readUiAxes(repoRoot);
      const targets = uiAxesTargets(root, axes);
      expect(targets.map(({ path }) => path)).toEqual([
        join(root, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🤖️generated/🦀️.rs"),
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
    const rustConsumer = readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs"), "utf8");
    const typescriptConsumer = readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🛂️manifest/🟦️.ts"), "utf8");
    expect(rustConsumer).toContain("#[path = \"🤖️generated/🦀️.rs\"]");
    expect(typescriptConsumer).toContain("./🤖️generated/🎚️ui-axes/🟦️.ts");
    const taxonomy = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as { generatorContracts: Record<string, { ownerPath: string; inputPatterns: string[] }> };
    const contract = taxonomy.generatorContracts["ui-axes"]!;
    expect(contract.ownerPath).toBe("🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust");
    for (const suffix of requiredInputs) expect(contract.inputPatterns.some((path) => path.endsWith(suffix))).toBe(true);
  });
});
