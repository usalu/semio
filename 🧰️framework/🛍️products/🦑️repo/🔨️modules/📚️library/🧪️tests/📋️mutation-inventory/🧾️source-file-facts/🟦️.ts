import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { fileKindIdForSourcePath, type Taxonomy } from "../../../🔍️discovery/🟦️.ts";
import { projectTaxonomySourceAdmission, type TaxonomySourceCandidateObservation, type TaxonomySourceInventory, type TaxonomySourceObservation } from "../../../🧹️normalization/🟦️.ts";
import { strictSourceDiagnostics } from "../../📣️typescript-declaration-facts/🔮️oracle/🟦️.ts";
import { sourceFileFactByteCompare, sourceFileFactCatalog, sourceFileFactReference, type SourceFileFactCase as Case, type SourceFileFactExpected as Expected } from "./🔮️oracle/🟦️.ts";

//#region 🧭️Inputs
const root = resolve(import.meta.dir, "../../../../../../../../");
const schemaPath = resolve(import.meta.dir, "../../📋️mutation-inventory/🧾️source-file-facts/🧬️schema/🔣️.json");
const vectorsPath = resolve(import.meta.dir, "../../📋️mutation-inventory/🧾️source-file-facts/🔣️.json");
const rootScriptPath = resolve(root, "📜️script.ts");
const taxonomyPath = resolve(import.meta.dir, "../../../🔣️taxonomy.json");
const schema = JSON.parse(readFileSync(schemaPath, "utf8"));
const vectors = JSON.parse(readFileSync(vectorsPath, "utf8")) as { readonly schemaVersion: 1; readonly cases: readonly Case[] };
const taxonomy = JSON.parse(readFileSync(taxonomyPath, "utf8")) as Taxonomy;

/** 🧫️ Supplies exact observation tuples; no fixture path is ever probed. */
function suppliedObservation(row: Case): TaxonomySourceObservation {
  const regular = row.observedKind === "file" && (row.mode === "100644" || row.mode === "100755"), gitlink = row.repositoryBoundary === "gitlink";
  const indexEntries = gitlink ? [{ stage: 0, mode: "160000", objectId: "c".repeat(40) }] : regular || row.observedKind === "symlink" ? [{ stage: 0, mode: row.mode!, objectId: "d".repeat(40) }] : [];
  return { sourcePath: row.sourcePath, observedKind: row.observedKind, worktreeMode: row.mode, explicitDirectory: row.observedKind === "directory", origins: indexEntries.length ? ["tracked"] : [], indexEntries, generatorOutputs: [], repositoryBoundary: row.repositoryBoundary };
}

/** 🧫️ Builds an explicit supplied-observation inventory; no collector claim is made. */
function admission(status: "complete" | "rejected", rows: readonly Case[]): TaxonomySourceInventory {
  return {
    schemaVersion: 1,
    scope: null,
    status,
    observations: rows.map(suppliedObservation),
    diagnostics: status === "complete" ? [] : [{ code: "opaque-path", path: "CoMpOsE", message: "virtual opaque path" }],
    repoRoot: "/virtual/workspace",
    taxonomyPath: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
    taxonomyContentHash: "a".repeat(64),
    membershipDigest: "b".repeat(64),
  };
}

/** 🧫️ Supplies a closed pure-projector candidate with explicit physical/index/provenance facts. */
function candidate(row: Case): TaxonomySourceCandidateObservation {
  const observation = suppliedObservation(row);
  return { sourcePath: observation.sourcePath, observedKind: observation.observedKind, worktreeMode: observation.worktreeMode, explicitDirectory: observation.explicitDirectory, origins: observation.origins, indexEntries: observation.indexEntries, unsafeAncestor: false };
}

/** 🛡️ Uses the actual pure admission projector for the virtual opaque path. */
function opaqueComposeAdmission(): TaxonomySourceInventory {
  const row = vectors.cases.find((entry) => entry.id === "rejected-compose")!;
  const projected = projectTaxonomySourceAdmission({ scope: null, opaquePrefixes: ["CoMpOsE"], generatorOutputRoots: [], candidates: [candidate(row)] });
  expect(projected.status).toBe("rejected");
  expect(projected.diagnostics.map((diagnostic) => diagnostic.code)).toContain("opaque-path");
  return { ...projected, repoRoot: "/virtual/workspace", taxonomyPath: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", taxonomyContentHash: "a".repeat(64), membershipDigest: "b".repeat(64) };
}

/** 🧭️ Resolves the proposed root projector dynamically so absence is a real test failure. */
async function subject(): Promise<(admission: TaxonomySourceInventory, taxonomy: Taxonomy) => readonly Omit<Expected, "extensionChain">[]> {
  const module = await import(`${pathToFileURL(rootScriptPath).href}?source-file-facts=${createHash("sha256").update(readFileSync(rootScriptPath)).digest("hex")}`);
  const projector = Reflect.get(module, "mutationTaxonomySourceFileFacts");
  if (typeof projector !== "function") throw new Error("missing mutationTaxonomySourceFileFacts export");
  return projector as (admission: TaxonomySourceInventory, taxonomy: Taxonomy) => readonly Omit<Expected, "extensionChain">[];
}
//#endregion 🧭️Inputs

//#region 🧪️Reference
test("mutation source-file facts vectors are closed and cover the registered source chains", () => {
  const ajv = new Ajv2020({ strict: true, allErrors: true }), validate = ajv.compile(schema);
  expect(validate(vectors), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(vectors.cases.map((row) => row.id)).size).toBe(vectors.cases.length);
  expect(validate({ schemaVersion: 1, cases: [] })).toBe(false);
  const registeredSourceChains = Object.values(taxonomy.fileKinds).filter((spec) => spec.role === "source").flatMap((spec) => spec.extensionChains).sort();
  const vectorSourceChains = vectors.cases.filter((row) => row.catalog === "current" && row.expected?.fileRole === "source").map((row) => row.expected!.extensionChain!).sort();
  expect(vectorSourceChains).toEqual(registeredSourceChains);
  const allRoles = new Set(Object.values(taxonomy.fileKinds).map((spec) => spec.role));
  const vectorRoles = new Set(vectors.cases.flatMap((row) => row.expected === null || row.expected.fileRole === null ? [] : [row.expected.fileRole]));
  expect(vectorRoles).toEqual(new Set([...allRoles, "generated"]));
  const nfd = vectors.cases.find((row) => row.id === "documentation-nfd")!;
  expect(nfd.sourcePath).not.toBe(nfd.sourcePath.normalize("NFC"));
  expect(sourceFileFactReference([nfd], taxonomy)).toEqual([nfd.expected]);
});

test("mutation source-file facts reference oracle has strict standalone types", () => {
  const path = resolve(import.meta.dir, "🔮️oracle/🟦️.ts");
  expect(strictSourceDiagnostics(readFileSync(path, "utf8"), path)).toEqual([]);
});

for (const catalog of ["current", "synthetic-generated", "synthetic-tie"] as const) test(`mutation source-file facts independent suffix reference: ${catalog}`, () => {
  const rows = vectors.cases.filter((row) => row.catalog === catalog && row.admissionStatus === "complete"), actual = sourceFileFactReference(rows, sourceFileFactCatalog(catalog, taxonomy));
  expect(actual).toEqual(rows.flatMap((row) => row.expected === null ? [] : [row.expected]).sort((left, right) => sourceFileFactByteCompare(left.sourcePath, right.sourcePath)));
  for (const row of rows) if (row.expected) expect(fileKindIdForSourcePath(row.sourcePath, sourceFileFactCatalog(catalog, taxonomy))).toBe(row.expected.fileKindId);
});
//#endregion 🧪️Reference

//#region 🧪️Subject
test("mutation source-file facts subject rejects projected opaque Compose before poisoned observations", async () => {
  const projector = await subject();
  const rejected = opaqueComposeAdmission();
  Object.defineProperty(rejected, "observations", { get: () => { throw new Error("rejected observations were read"); } });
  expect(() => projector(rejected, taxonomy)).toThrow(/source admission is rejected/u);
});

for (const catalog of ["current", "synthetic-generated", "synthetic-tie"] as const) test(`mutation source-file facts subject matches reference: ${catalog}`, async () => {
  const rows = vectors.cases.filter((row) => row.catalog === catalog && row.admissionStatus === "complete"), projector = await subject();
  const actual = projector(admission("complete", rows), sourceFileFactCatalog(catalog, taxonomy));
  expect(actual).toEqual(sourceFileFactReference(rows, sourceFileFactCatalog(catalog, taxonomy)).map(({ extensionChain: _, ...row }) => row));
});
//#endregion 🧪️Subject
