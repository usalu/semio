/** 🔍️ The scoped inventory, symlink, nested repository and census parity tests. */
import { expect, test } from "bun:test";
import fastGlob from "fast-glob";
import { lstatSync, mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { inventoryTaxonomy } from "../../../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { type TaxonomyInventoryOptions } from "../../../../🧹️normalization/📜️plan-contracts/🟦️.ts";
import { canonicalJson } from "../../../../🧾️serialization/🔣️json/🟦️.ts";
import { taxonomyScopedGitPathspec } from "../../../../🧹️normalization/🚪️source-admission/🟦️.ts";
import { type TaxonomyProgress } from "../../../../🧹️normalization/🏃️operation/🟦️.ts";
import { ownedPathByteSort } from "../../../🔍️filesystem/🟦️.ts";
import { NORMALIZATION_SCHEMA_REL, SCOPED_INVENTORY_PATHSPEC_GOLDEN, inScopeForTest, normalizationFixture, normalizationGit, normalizationInventoryCensus, normalizationWriteFiles } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🔗 Registers the scoped inventory, exact symlink leaf, symlink ancestor fallback, mode 160000 refusal and census parity tests. */
export function registerScopedInventoryAndSymlinkTests(): void {
  test("scoped inventory preserves literal Unicode scope, third-party census, and closed phases", () => {
    const literalRoot = "🧪️literal [x]*?!:";
    const decomposedRoot = "🧪️cafe\u0301";
    const fixture = normalizationFixture("scoped-pathspec", {
      ".gitignore": "ignored/\n",
      "🟦️.ts": "export const tracked = true;\n",
      [`${literalRoot}/🟦️.ts`]: "export const literal = true;\n",
      [`${decomposedRoot}/🟦️.ts`]: "export const unicode = true;\n",
    });
    try {
      normalizationWriteFiles(fixture.workspace, {
        "🧪️untracked/🟦️.ts": "export const untracked = true;\n",
        "ignored/🟦️.ts": "export const ignored = true;\n",
      });
      normalizationWriteFiles(fixture.repoRoot, { "🧪️unrelated/🟦️.ts": "export const unrelated = true;\n" });
      const events: TaxonomyProgress[] = [];
      const scopedOptions: TaxonomyInventoryOptions = fixture.options;
      const inventory = inventoryTaxonomy({ ...scopedOptions, progress: (event) => events.push(event) });
      const files = inventory.entries.filter((entry) => entry.nodeKind === "file").map((entry) => entry.sourcePath);
      expect(files).toContain(`${fixture.scope}/🧪️untracked/🟦️.ts`);
      expect(files.some((path) => path.startsWith("🧪️unrelated/"))).toBe(false);
      const thirdParty = fastGlob.sync("**/*", { cwd: fixture.workspace, dot: true, followSymbolicLinks: false, onlyFiles: true })
        .map((path) => `${fixture.scope}/${path.replaceAll("\\", "/")}`)
        .sort(ownedPathByteSort);
      expect([...files].sort(ownedPathByteSort)).toEqual(thirdParty);
      const phases = events.map((event) => event.phase).filter((phase, index, rows) => index === 0 || phase !== rows[index - 1]);
      expect(phases).toEqual(["tracked-enumeration", "setup", "untracked-enumeration", "ignored-generator-admission", "explicit-ticket-admission", "source-observation", "directories", "files", "references", "complete"]);
      for (const phase of ["tracked-enumeration", "untracked-enumeration", "ignored-generator-admission", "explicit-ticket-admission"]) expect(events.filter((event) => event.phase === phase).map((event) => [event.current, event.total])).toEqual([[0, 1], [1, 1]]);
      for (const phase of ["directories", "files", "references"]) expect(events.find((event) => event.phase === phase)?.current).toBe(0);
      const unicode = inventoryTaxonomy({ ...scopedOptions, scope: `${fixture.scope}/🧪️café` });
      expect(unicode.entries.some((entry) => entry.sourcePath === `${fixture.scope}/${decomposedRoot}/🟦️.ts`)).toBe(true);
      const literal = inventoryTaxonomy({ ...scopedOptions, scope: `${fixture.scope}/${literalRoot}` });
      expect(literal.entries.filter((entry) => entry.nodeKind === "file").map((entry) => entry.sourcePath)).toEqual([`${fixture.scope}/${literalRoot}/🟦️.ts`]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
  test("an exact symlink leaf is inventoried without following target content", () => {
    const fixture = normalizationFixture("scoped-symlink-leaf", { "🧪️target/📝️.md": "first\n" }, ({ workspace }) => symlinkSync("🧪️target/📝️.md", join(workspace, "🧪️link"), "file"));
    try {
      const scope = `${fixture.scope}/🧪️link`;
      normalizationGit(fixture.repoRoot, ["add", "-f", "--", scope]);
      expect(lstatSync(join(fixture.workspace, "🧪️link")).isSymbolicLink()).toBe(true);
      const first = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      const link = first.entries.find(({ sourcePath }) => sourcePath === scope);
      expect(link?.nodeKind).toBe("symlink");
      expect(link?.symlinkTarget).toBe("🧪️target/📝️.md");
      writeFileSync(join(fixture.workspace, "🧪️target", "📝️.md"), "second and deliberately different\n");
      const second = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      expect(second.entries.find(({ sourcePath }) => sourcePath === scope)?.contentHash).toBe(link?.contentHash);
      expect(second.entries.some(({ sourcePath }) => sourcePath.startsWith(`${scope}/`))).toBe(false);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
  test("a scope below a symlink ancestor falls back to full Git enumeration and preserves the ancestor-only census", () => {
    const fixture = normalizationFixture("scoped-symlink-ancestor", { "🧪️seed/📝️.md": "seed\n" }, ({ repoRoot, workspace }) => {
      const target = join(repoRoot, "🧪️outside-target");
      normalizationWriteFiles(target, { "🧪️child/📝️.md": "must not be followed\n" });
      symlinkSync(process.platform === "win32" ? target : relative(workspace, target), join(workspace, "🧪️alias"), process.platform === "win32" ? "junction" : "dir");
    });
    try {
      const scope = `${fixture.scope}/🧪️alias/🧪️child`;
      normalizationGit(fixture.repoRoot, ["add", "-f", "--", `${fixture.scope}/🧪️alias`]);
      const scoped = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      const full = inventoryTaxonomy({ repoRoot: fixture.repoRoot, workers: 1 });
      const expected = normalizationInventoryCensus({ ...full, entries: full.entries.filter(({ sourcePath }) => inScopeForTest(sourcePath, scope)) });
      expect(normalizationInventoryCensus(scoped)).toEqual(expected);
      expect(scoped.entries.filter(({ nodeKind }) => nodeKind !== "directory").map(({ sourcePath, nodeKind }) => ({ sourcePath, nodeKind }))).toEqual([{ sourcePath: `${fixture.scope}/🧪️alias`, nodeKind: "symlink" }]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
  test("an exact mode-160000 ancestor refuses scoped admission without reading nested-repository content", () => {
    const fixture = normalizationFixture("scoped-gitlink-ancestor", { "🧪️seed/📝️.md": "seed\n" });
    const gitlinkRelative = `${fixture.scope}/🧪️gitlink`;
    const gitlink = join(fixture.workspace, "🧪️gitlink");
    try {
      mkdirSync(gitlink, { recursive: true });
      normalizationGit(gitlink, ["init", "--quiet"]);
      normalizationWriteFiles(gitlink, { "🧪️child/📝️.md": "nested sentinel\n" });
      normalizationGit(fixture.repoRoot, ["update-index", "--add", "--cacheinfo", `160000,${fixture.baselineCommit},${gitlinkRelative}`]);
      const reads: string[] = [];
      expect(() => inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope: `${gitlinkRelative}/🧪️child`, workers: 1, progress: (event) => { if (event.path) reads.push(event.path); } })).toThrow("Source admission scope crosses an index-owned repository boundary");
      expect(reads.some((path) => path.startsWith(`${gitlinkRelative}/`) && path !== `${gitlinkRelative}/🧪️child`)).toBe(false);
      expect(() => inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope: gitlinkRelative, workers: 1 })).toThrow("explicit repository-boundary decision");
      expect(() => inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope: fixture.scope, workers: 1 })).toThrow("explicit repository-boundary decision");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
  test("standalone ignored-generator admission preserves scoped versus full census parity", () => {
    const fixture = normalizationFixture("scoped-generator-admission", { "🧪️generator/🟦️.ts": "export const input = true;\n" }, ({ repoRoot, workspace }) => {
      const owner = relative(repoRoot, join(workspace, "🧪️generator")).replaceAll("\\", "/");
      const outputRoot = `${owner}/🤖️generated`;
      const schemaPath = join(repoRoot, NORMALIZATION_SCHEMA_REL);
      const taxonomy = JSON.parse(readFileSync(schemaPath, "utf8")) as { generatorContracts: Record<string, unknown> };
      taxonomy.generatorContracts = Object.fromEntries(Object.entries({ ...taxonomy.generatorContracts, "fixture-scoped-generator": { ownership: "owned", ownerPath: owner, target: "@fixture/scoped:generate", previewTarget: "@fixture/scoped:preview-generated", checkTarget: "@fixture/scoped:check", inputPatterns: [`${owner}/🟦️.ts`], outputRoots: [{ path: outputRoot, inclusion: "ignored" }], reason: "Scoped inventory admission fixture" } }).sort(([left], [right]) => left.localeCompare(right)));
      writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
      writeFileSync(join(repoRoot, ".gitignore"), `${outputRoot}\n`);
      normalizationWriteFiles(repoRoot, { [`${outputRoot}/📝️.md`]: "owned ignored output\n" });
    });
    try {
      const scope = `${fixture.scope}/🧪️generator/🤖️generated`;
      const scoped = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      const full = inventoryTaxonomy({ repoRoot: fixture.repoRoot, workers: 1 });
      const expected = normalizationInventoryCensus({ ...full, entries: full.entries.filter(({ sourcePath }) => inScopeForTest(sourcePath, scope)) });
      expect(normalizationInventoryCensus(scoped)).toEqual(expected);
      expect(scoped.entries.some(({ sourcePath }) => sourcePath === `${scope}/📝️.md`)).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
  test("standalone explicit-ticket admission preserves scoped versus full census parity", () => {
    const fixture = normalizationFixture("scoped-ticket-admission", { ".gitignore": "🧪️explicit/\n", "🟦️.ts": "export const input = true;\n" }, ({ workspace }) => normalizationWriteFiles(workspace, { "🧪️explicit/📝️.md": "explicit ignored evidence\n" }));
    try {
      const scope = `${fixture.scope}/🧪️explicit`;
      const scoped = inventoryTaxonomy({ ...fixture.options, scope });
      const full = inventoryTaxonomy({ ...fixture.options, scope: undefined });
      const withoutTicketAuthority = inventoryTaxonomy({ repoRoot: fixture.repoRoot, scope, workers: 1 });
      const expected = normalizationInventoryCensus({ ...full, entries: full.entries.filter(({ sourcePath }) => inScopeForTest(sourcePath, scope)) });
      expect(normalizationInventoryCensus(scoped)).toEqual(expected);
      expect(scoped.entries.some(({ sourcePath }) => sourcePath === `${scope}/📝️.md`)).toBe(true);
      expect(withoutTicketAuthority.entries.some(({ sourcePath }) => sourcePath === `${scope}/📝️.md`)).toBe(false);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
  test("inventory cancellation is observed from every nonterminal frozen phase boundary", () => {
    const fixture = normalizationFixture("scoped-phase-cancellation", { "🧪️subject/🟦️.ts": "export const value = true;\n" });
    const cancelFile = join(fixture.root, "🛑️cancel");
    try {
      for (const phase of ["tracked-enumeration", "setup", "untracked-enumeration", "ignored-generator-admission", "explicit-ticket-admission", "source-observation", "directories", "files", "references"]) {
        rmSync(cancelFile, { force: true });
        expect(() => inventoryTaxonomy({ ...fixture.options, cancelFile, progress: (event) => { if (event.phase === phase && event.current === 0) writeFileSync(cancelFile, `${phase}\n`); } })).toThrow("Taxonomy operation cancelled");
      }
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
  test("unscoped inventory retains the frozen Git pathspec and canonical inventory bytes", () => {
    const fixture = normalizationFixture("unscoped-regression", { "🧪️subject/🟦️.ts": "export const value = true;\n" });
    try {
      const options: TaxonomyInventoryOptions = { repoRoot: fixture.repoRoot, workers: 1 };
      const first = inventoryTaxonomy(options);
      const second = inventoryTaxonomy(options);
      expect(canonicalJson(first)).toBe(canonicalJson(second));
      expect(taxonomyScopedGitPathspec(undefined, SCOPED_INVENTORY_PATHSPEC_GOLDEN.opaqueExclusions)).toEqual({ normalizedScope: null, conservativePrefix: ".", positivePathspec: ".", exclusionPathspecs: [":(exclude,top,literal)compose", ":(exclude,top,literal)temp/compose"] });
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
}
