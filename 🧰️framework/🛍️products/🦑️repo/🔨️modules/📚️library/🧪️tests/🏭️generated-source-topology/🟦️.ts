import { describe, expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { generatorContractIdsForOutputPath, getWorkspaceRoot, implementationLeafBasenameFinding, loadCatalogTaxonomy, semanticDirectoryKindId } from "../../📦️packages/🟦️typescript/🟦️.ts";

type NativePackage = { readonly root: string; readonly metadataPath: string; readonly sourceReference: string };
type SourceCase = { readonly contractId: string; readonly previousPath: string; readonly supersededPaths?: readonly string[]; readonly canonicalPath: string; readonly fileKindId: string; readonly semanticOwnerKindId: string; readonly inclusion: "ignored" | "tracked"; readonly nativePackage?: NativePackage };
type Fixture = { readonly schemaVersion: 1; readonly kindLeaves: Readonly<Record<string, string>>; readonly cases: readonly SourceCase[] };

const repoRoot = getWorkspaceRoot();
const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🏭️generated-source-topology/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🏭️generated-source-topology/🔣️.json"), "utf8"));

describe("generated source topology", () => {
  test("portable fixture is schema-valid and exact", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(new Set(fixture.cases.map(({ previousPath }) => previousPath)).size).toBe(30);
    expect(new Set(fixture.cases.map(({ canonicalPath }) => canonicalPath)).size).toBe(30);
    const retiredPaths = fixture.cases.flatMap(({ previousPath, supersededPaths = [] }) => [previousPath, ...supersededPaths]);
    expect(new Set(retiredPaths).size).toBe(retiredPaths.length);
  });

  test("every persistent source is semantic, kind-only, owned and regenerated", () => {
    const taxonomy = loadCatalogTaxonomy();
    for (const row of fixture.cases) {
      expect(basename(row.canonicalPath)).toBe(fixture.kindLeaves[row.fileKindId]);
      expect(semanticDirectoryKindId(basename(dirname(row.canonicalPath)), taxonomy, { parentKindId: row.semanticOwnerKindId })).not.toBeNull();
      expect(generatorContractIdsForOutputPath(row.canonicalPath, taxonomy)).toContain(row.contractId);
      const contract = taxonomy.generatorContracts[row.contractId]!;
      expect(contract.outputRoots.some((root) => root.inclusion === row.inclusion && (root.path === row.canonicalPath || row.canonicalPath.startsWith(`${root.path}/`)))).toBe(true);
      expect(implementationLeafBasenameFinding(row.canonicalPath, taxonomy)).toBeNull();
      expect(existsSync(join(repoRoot, row.previousPath))).toBe(false);
      for (const path of row.supersededPaths ?? []) expect(existsSync(join(repoRoot, path))).toBe(false);
      expect(existsSync(join(repoRoot, row.canonicalPath))).toBe(true);
      if (row.nativePackage) {
        expect(row.canonicalPath.startsWith(`${row.nativePackage.root}/`)).toBe(false);
        expect(row.nativePackage.root).toContain("/📦️packages/");
        expect(resolve(repoRoot, row.nativePackage.root, row.nativePackage.sourceReference)).toBe(resolve(repoRoot, row.canonicalPath));
        expect(readFileSync(join(repoRoot, row.nativePackage.metadataPath), "utf8")).toContain(row.nativePackage.sourceReference);
      }
    }
  }, 15_000);
});
