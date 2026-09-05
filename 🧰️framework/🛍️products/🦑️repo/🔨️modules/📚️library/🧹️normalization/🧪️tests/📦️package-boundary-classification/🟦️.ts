//#region 🔌️Adapters
import Ajv from "ajv/dist/2020";
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { classifyPackageGlueContent, fixedContractScopeSpecificityRank, type FixedContractScopeKind, type PackageGlueAnalyzer, type TaxonomyPackageRole } from "../../🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const vectors = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as {
  glueRoleCases: readonly { id: string; analyzer: PackageGlueAnalyzer; maxDelegationStatements: number; content: string; expectedRole: TaxonomyPackageRole }[];
  scopeSpecificityOrder: readonly FixedContractScopeKind[];
};

/** 🧬️ Independent structural oracle for the fixture shape itself (a third-party JSON Schema
 * validator, matching the sibling `🚪️source-admission` suite's own convention), kept inline since
 * this narrow fixture does not warrant a standalone schema file. */
const schema = {
  $id: "https://semio.local/package-boundary-classification-vectors",
  type: "object",
  required: ["glueRoleCases", "scopeSpecificityOrder"],
  additionalProperties: false,
  properties: {
    glueRoleCases: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        required: ["id", "analyzer", "maxDelegationStatements", "content", "expectedRole"],
        additionalProperties: false,
        properties: {
          id: { type: "string", minLength: 1 },
          analyzer: { enum: ["rust", "typescript", "javascript", "go", "python", "dotnet", "c-cpp"] },
          maxDelegationStatements: { type: "integer", minimum: 0 },
          content: { type: "string" },
          expectedRole: { enum: ["configuration", "declaration", "registration", "bootstrap", "thin-delegation", "implementation", "unresolved", "not-package"] },
        },
      },
    },
    scopeSpecificityOrder: {
      type: "array",
      minItems: 2,
      items: { enum: ["exact-path", "repository-root", "package-root", "directory-kind", "fixed-directory-contract", "sibling-fixed-filename-contract", "path-pattern"] },
    },
  },
};
const validator = new Ajv({ strict: true, allErrors: true });
const validateVectors = validator.compile(schema);
//#endregion 🧬️Contract

//#region 🧪️Classification
describe("package boundary glue-content classification", () => {
  test("fixture vectors satisfy the independent schema implementation", () => {
    expect(validateVectors(vectors), JSON.stringify(validateVectors.errors)).toBe(true);
    const ids = vectors.glueRoleCases.map((row) => row.id);
    expect(new Set(ids).size).toBe(ids.length);
  });

  for (const row of vectors.glueRoleCases) test(row.id, () => {
    expect(classifyPackageGlueContent(row.analyzer, row.content, row.maxDelegationStatements)).toBe(row.expectedRole);
  });

  test("scope-kind specificity is strictly increasing in the documented, narrowest-wins order", () => {
    const ranks = vectors.scopeSpecificityOrder.map((kind) => fixedContractScopeSpecificityRank(kind));
    for (let index = 1; index < ranks.length; index++) expect(ranks[index]).toBeGreaterThan(ranks[index - 1]);
  });

  test("sibling-fixed-filename-contract outranks package-root (the package.json/tsconfig.json ambiguity fix)", () => {
    expect(fixedContractScopeSpecificityRank("sibling-fixed-filename-contract")).toBeGreaterThan(fixedContractScopeSpecificityRank("package-root"));
  });
});
//#endregion 🧪️Classification
