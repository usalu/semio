import { describe, expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { basename, dirname, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { classifyPackageSource, loadCatalogTaxonomy, semanticDirectoryKindId } from "../../🔍️discovery/🟦️.ts";

type Entry = Readonly<{
  legacy: string;
  owner: string;
  kind: "dotnet" | "rust" | "typescript" | "typescript-declaration" | "typescript-jsx";
  boundary: "cargo-bin" | "cargo-lib" | "declaration" | "dotnet-compile" | "next-layout" | "next-page" | "next-route" | "none" | "package-reexport";
  anchor: string;
  glue?: string;
  exports?: readonly string[];
}>;
type OwnerDirectoryChain = Readonly<{
  owner: string;
  parentKind: string;
  members: readonly Readonly<{ name: string; kind: string }>[];
}>;
type Fixture = Readonly<{ version: 1; entries: readonly Entry[]; ownerDirectoryChains: readonly OwnerDirectoryChain[] }>;

const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(libraryRoot, "../../../../..");
const fixture = JSON.parse(readFileSync(resolve(libraryRoot, "🧫️fixtures/🦑️repo-source-ownership/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(libraryRoot, "🧬️schema/🦑️repo-source-ownership/🔣️.json"), "utf8"));
const basenames: Readonly<Record<Entry["kind"], string>> = {
  dotnet: "🔷️.cs",
  rust: "🦀️.rs",
  typescript: "🟦️.ts",
  "typescript-declaration": "🟦️.d.mts",
  "typescript-jsx": "🟦️.tsx",
};
const wrappers = new Set<Entry["boundary"]>(["package-reexport", "next-route", "next-layout", "next-page"]);

describe("repository source ownership", () => {
  test("validates the exact portable 22-entry projection", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.entries).toHaveLength(22);
    expect(new Set(fixture.entries.map(({ legacy }) => legacy))).toHaveProperty("size", 22);
    expect(new Set(fixture.entries.map(({ owner }) => owner))).toHaveProperty("size", 22);
  });

  test("owns implementation bytes at anonymous semantic leaves and only retains required glue", () => {
    for (const row of fixture.entries) {
      const owner = resolve(repoRoot, row.owner);
      const legacy = resolve(repoRoot, row.legacy);
      expect(existsSync(owner), row.owner).toBe(true);
      expect(basename(owner), row.owner).toBe(basenames[row.kind]);
      expect(readFileSync(owner, "utf8"), row.owner).toContain(row.anchor);
      if (!wrappers.has(row.boundary)) {
        expect(existsSync(legacy), row.legacy).toBe(false);
        continue;
      }
      expect(existsSync(legacy), row.legacy).toBe(true);
      const glue = readFileSync(legacy, "utf8");
      expect(glue.trimEnd().split("\n").length, row.legacy).toBeLessThanOrEqual(2);
      expect(glue, row.legacy).toContain(row.glue!);
      expect(resolve(dirname(legacy), row.glue!), row.legacy).toBe(owner);
      for (const name of row.exports ?? []) expect(glue, name).toContain(name);
    }
  });

  test("resolves the complete registered ancestry of both library source owners", () => {
    const taxonomy = loadCatalogTaxonomy(repoRoot);
    expect(fixture.ownerDirectoryChains).toHaveLength(2);
    for (const chain of fixture.ownerDirectoryChains) {
      expect(existsSync(resolve(repoRoot, chain.owner)), chain.owner).toBe(true);
      let parentKind = chain.parentKind;
      for (const member of chain.members) {
        expect(semanticDirectoryKindId(member.name, taxonomy, { parentKindId: parentKind }), `${parentKind}/${member.name}`).toBe(member.kind);
        parentKind = member.kind;
      }
    }
  });

  test("parses all TypeScript owners and delegates with the installed compiler oracle", () => {
    for (const row of fixture.entries) {
      if (!row.kind.startsWith("typescript")) continue;
      for (const path of [row.owner, ...(wrappers.has(row.boundary) ? [row.legacy] : [])]) {
        const source = readFileSync(resolve(repoRoot, path), "utf8");
        const kind = path.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
        const parsed = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true, kind);
        expect(parsed.parseDiagnostics, path).toEqual([]);
        if (path === row.legacy) expect(parsed.statements.every((statement) => ts.isExportDeclaration(statement)), path).toBe(true);
      }
    }
  });

  test("binds native manifests and preserves the library package-directory anchor", async () => {
    const cliManifest = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/Cargo.toml"), "utf8");
    expect(cliManifest).toContain('path = "../../🦀️.rs"');
    expect(cliManifest).toContain('path = "../../🚪️entrypoint/🦀️.rs"');
    const rustTestManifest = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml"), "utf8");
    expect(rustTestManifest).toContain('path = "../../🦀️.rs"');
    const dotnetProject = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🔷️dotnet/🧪️Semio.Repo.Test.csproj"), "utf8");
    expect(dotnetProject).toContain('Compile Include="../../🖥️host/🔷️.cs"');
    const library = await import("../../📦️packages/🟦️typescript/🟦️.ts");
    expect(library.getLibRoot()).toBe(resolve(libraryRoot, "📦️packages"));
  });

  test("classifies retained TypeScript package entries as declarations", () => {
    const taxonomy = loadCatalogTaxonomy(repoRoot);
    const packageEntries = fixture.entries.filter(({ boundary, kind }) => boundary === "package-reexport" && kind === "typescript");
    expect(packageEntries).toHaveLength(3);
    for (const row of packageEntries) {
      const source = readFileSync(resolve(repoRoot, row.legacy), "utf8");
      expect(classifyPackageSource(source, taxonomy.packageGlueGrammar.typescript!).role, row.legacy).toBe("declaration");
    }
  });

  test("registers the portable check in both editor launch authorities", () => {
    const name = "🧹clean🦑️repo🧪️source-ownership";
    const command = "bun nx run @semio-tech/repo-lib:test-repo-source-ownership";
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const launch = Bun.JSONC.parse(readFileSync(resolve(repoRoot, path), "utf8")) as { readonly configurations: readonly { readonly name?: string; readonly command?: string }[] };
      expect(launch.configurations.filter((row) => row.name === name && row.command === command), path).toHaveLength(1);
    }
  });
});
