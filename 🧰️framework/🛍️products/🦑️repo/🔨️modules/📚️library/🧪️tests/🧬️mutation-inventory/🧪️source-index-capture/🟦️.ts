import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { Taxonomy } from "../../../🔍️discovery/🟦️.ts";
import type { TaxonomySourceInventory, TaxonomySourceObservation } from "../../../🧹️normalization/🟦️.ts";
import { sourceFileFactByteCompare, sourceFileFactOracleKind } from "../../🧬️🔀️🌲️mutation-inventory/🧪️source-file-facts/🟦️.ts";

//#region 🧭️Inputs
const root = (() => {
  let candidate = import.meta.dir;
  while (!existsSync(join(candidate, ".🧬semio"))) {
    const parent = dirname(candidate);
    if (parent === candidate) throw new Error("workspace marker is absent");
    candidate = parent;
  }
  return candidate;
})();
const library = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-capture-66");
const descriptorRelative = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️.schema.json";
const taxonomyRelative = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const rootScriptPath = join(root, "📜️script.ts");
const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️🔀️🌲️mutation-inventory/🧪️🚪️source-index-capture/🧬️schema/🔣️.json"), "utf8"));
const vectors = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️🔀️🌲️mutation-inventory/🧪️🚪️source-index-capture/🔣️.json"), "utf8")) as {
  readonly schemaVersion: 1;
  readonly expectedRoots: readonly string[];
  readonly existingEvidence: readonly string[];
  readonly cases: readonly { readonly id: string; readonly path: string; readonly bytes: string; readonly expectedKind: string | null; readonly expectedRole: "source" | "schema" | "specification" | "documentation" | null; readonly retained: boolean }[];
  readonly unknownPath: string;
  readonly cancelFile: string;
  readonly cancelProbePath: string;
};
const sha256 = (value: string | Uint8Array): string => createHash("sha256").update(value).digest("hex");
const compare = (left: string, right: string): number => sourceFileFactByteCompare(left, right);

/** 🧫️ Creates one ticket-owned physical fixture without collector or Git access. */
function fixture(): string {
  mkdirSync(ticket, { recursive: true });
  return mkdtempSync(join(ticket, "🧫️run-"));
}

/** 🧫️ Writes an owned relative fixture file. */
function writeFixture(rootPath: string, path: string, bytes: string | Uint8Array): void {
  if (!path || path.includes("\\") || path.split("/").some((part) => !part || part === "" || part === "../../🧬️🔀️🌲️mutation-inventory" || part.toLocaleLowerCase("en-US") === "compose")) throw new Error(`unsafe fixture path: ${path}`);
  const target = join(rootPath, ...path.split("/"));
  mkdirSync(dirname(target), { recursive: true });
  writeFileSync(target, bytes, { flag: "wx" });
}

/** 🧫️ Supplies exactly one complete admission; no collector is invoked. */
function admission(rootPath: string, taxonomyBytes: Buffer, cancellationProbe = false): TaxonomySourceInventory {
  const regular = vectors.cases.map((row): TaxonomySourceObservation => ({ sourcePath: row.path, observedKind: "file", worktreeMode: "100644", explicitDirectory: false, origins: ["tracked"], indexEntries: [{ stage: 0, mode: "100644", objectId: sha256(row.bytes).slice(0, 40) }], generatorOutputs: [], repositoryBoundary: null }));
  const nonregular: TaxonomySourceObservation = { sourcePath: "virtual/nonregular.rs", observedKind: "symlink", worktreeMode: "120000", explicitDirectory: false, origins: ["tracked"], indexEntries: [{ stage: 0, mode: "120000", objectId: "0".repeat(40) }], generatorOutputs: [], repositoryBoundary: null };
  const probe: TaxonomySourceObservation[] = cancellationProbe ? [{ sourcePath: vectors.cancelProbePath, observedKind: "file", worktreeMode: "100644", explicitDirectory: false, origins: ["tracked"], indexEntries: [{ stage: 0, mode: "100644", objectId: "f".repeat(40) }], generatorOutputs: [], repositoryBoundary: null }] : [];
  return { schemaVersion: 1, scope: null, status: "complete", observations: [...probe, ...regular, nonregular], diagnostics: [], repoRoot: rootPath, taxonomyPath: taxonomyRelative, taxonomyContentHash: sha256(taxonomyBytes), membershipDigest: "a".repeat(64) };
}

/** 🔐️ Reconstructs the documented source-tree digest with explicit sorted JSON keys. */
function expectedDigest(roots: readonly string[], roster: readonly { readonly path: string; readonly sha256: string; readonly role: string }[], membershipDigest: string, taxonomyContentHash: string, mutationDescriptorSchemaHash: string): string {
  const quoted = (value: unknown): string => JSON.stringify(value);
  const canonicalRoots = [...roots];
  const canonicalRoster = [...roster].sort((left, right) => compare(left.path, right.path)).map(({ path, role, sha256: digest }) => `{"path":${quoted(path)},"role":${quoted(role)},"sha256":${quoted(digest)}}`).join(",");
  const body = `{"membershipDigest":${quoted(membershipDigest)},"mutationDescriptorSchemaHash":${quoted(mutationDescriptorSchemaHash)},"roots":${quoted(canonicalRoots)},"sourceRoster":[${canonicalRoster}],"taxonomyContentHash":${quoted(taxonomyContentHash)}}`;
  return sha256(body);
}

/** 🧭️ Resolves the actual root function using its current captured bytes. */
async function subject(): Promise<{ readonly mutationTaxonomySourceIndex: (repoRoot: string, options: Record<string, unknown>, injected: TaxonomySourceInventory) => { readonly files: readonly string[]; readonly bytes: ReadonlyMap<string, Buffer>; readonly contents: ReadonlyMap<string, string>; readonly sourceRoster: readonly { readonly path: string; readonly sha256: string; readonly role: string }[]; readonly sourceTreeDigest: string } }> {
  const module = await import(`${pathToFileURL(rootScriptPath).href}?source-index-capture=${sha256(readFileSync(rootScriptPath))}`);
  if (typeof module.mutationTaxonomySourceIndex !== "function") throw new Error("missing mutationTaxonomySourceIndex export");
  return module as Awaited<ReturnType<typeof subject>>;
}
//#endregion 🧭️Inputs

//#region 🧪️Capture
test("mutation source index captures admitted registered role files without a second collector", async () => {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  expect(validate(vectors), JSON.stringify(validate.errors)).toBe(true);
  const run = fixture(), taxonomyBytes = readFileSync(join(library, "🔣️taxonomy.json")), descriptorBytes = readFileSync(join(library, "🔣️.schema.json"));
  writeFixture(run, taxonomyRelative, taxonomyBytes);
  writeFixture(run, descriptorRelative, descriptorBytes);
  for (const row of vectors.cases) writeFixture(run, row.path, row.bytes);
  const actualTaxonomy = JSON.parse(taxonomyBytes.toString("utf8")) as Taxonomy, injected = admission(run, taxonomyBytes);
  const oracle = vectors.cases.map((row) => {
    const resolved = sourceFileFactOracleKind(row.path, actualTaxonomy), role = resolved.fileKindId === null ? null : actualTaxonomy.fileKinds[resolved.fileKindId]!.role;
    return { sourcePath: row.path, fileKindId: resolved.fileKindId, fileRole: role };
  }).sort((left, right) => compare(left.sourcePath, right.sourcePath));
  expect(oracle).toEqual(vectors.cases.map(({ path, expectedKind, expectedRole }) => ({ sourcePath: path, fileKindId: expectedKind, fileRole: expectedRole })).sort((left, right) => compare(left.sourcePath, right.sourcePath)));
  const expectedFiles = vectors.cases.filter((row) => row.retained).map((row) => row.path).sort(compare);
  const expectedRoster = (paths: readonly string[], python: string, wit: string): readonly { readonly path: string; readonly sha256: string; readonly role: string }[] => [
    ...paths.map((path) => ({ path, sha256: sha256(path === "languages/worker.py" ? python : path === "contracts/world.wit" ? wit : vectors.cases.find((row) => row.path === path)!.bytes), role: "source" })),
    { path: taxonomyRelative, sha256: sha256(taxonomyBytes), role: "taxonomy-schema" },
    { path: descriptorRelative, sha256: sha256(descriptorBytes), role: "mutation-descriptor-schema" },
  ].sort((left, right) => compare(`${left.role}\0${left.path}`, `${right.role}\0${right.path}`));
  const rootApi = await subject(), first = rootApi.mutationTaxonomySourceIndex(run, {}, injected), initialRoster = expectedRoster(expectedFiles, "value = 1\n", "package fixture:world;\n");
  expect(first.files).toEqual(expectedFiles);
  expect([...first.bytes].map(([path, bytes]) => [path, bytes.toString("utf8")]).sort(([left], [right]) => compare(left, right))).toEqual(expectedFiles.map((path) => [path, vectors.cases.find((row) => row.path === path)!.bytes] as const));
  expect([...first.contents].sort(([left], [right]) => compare(left, right))).toEqual(expectedFiles.map((path) => [path, vectors.cases.find((row) => row.path === path)!.bytes] as const));
  expect(first.sourceRoster).toEqual(initialRoster);
  expect(first.sourceTreeDigest).toBe(expectedDigest(vectors.expectedRoots, initialRoster, injected.membershipDigest, injected.taxonomyContentHash, sha256(descriptorBytes)));
  const unknown = oracle.find((row) => row.sourcePath === vectors.unknownPath);
  expect(unknown).toEqual({ sourcePath: vectors.unknownPath, fileKindId: null, fileRole: null });
  expect(first.files).not.toContain(vectors.unknownPath);
  const changedPython = "value = 2\n";
  writeFileSync(join(run, "languages/worker.py"), changedPython);
  const second = rootApi.mutationTaxonomySourceIndex(run, {}, injected);
  const pythonRoster = expectedRoster(expectedFiles, changedPython, "package fixture:world;\n");
  expect(second.sourceRoster).toEqual(pythonRoster);
  expect(second.sourceTreeDigest).toBe(expectedDigest(vectors.expectedRoots, pythonRoster, injected.membershipDigest, injected.taxonomyContentHash, sha256(descriptorBytes)));
  expect(second.sourceTreeDigest).not.toBe(first.sourceTreeDigest);
  const changedWit = "package fixture:next;\n";
  writeFileSync(join(run, "contracts/world.wit"), changedWit);
  const third = rootApi.mutationTaxonomySourceIndex(run, {}, injected), witRoster = expectedRoster(expectedFiles, changedPython, changedWit);
  expect(third.sourceRoster).toEqual(witRoster);
  expect(third.sourceTreeDigest).toBe(expectedDigest(vectors.expectedRoots, witRoster, injected.membershipDigest, injected.taxonomyContentHash, sha256(descriptorBytes)));
  expect(third.sourceTreeDigest).not.toBe(second.sourceTreeDigest);
  const rejected = { ...injected, status: "rejected" as const } as TaxonomySourceInventory;
  Object.defineProperty(rejected, "observations", { get: () => { throw new Error("rejected observations were read"); } });
  expect(() => rootApi.mutationTaxonomySourceIndex(run, {}, rejected)).toThrow(/source admission is rejected/u);
  writeFixture(run, vectors.cancelFile, "cancel\n");
  const progress: unknown[] = [];
  expect(compare(vectors.cancelProbePath, expectedFiles[0]!) < 0).toBe(true);
  expect(() => rootApi.mutationTaxonomySourceIndex(run, { cancelFile: vectors.cancelFile, progress: (entry: unknown) => progress.push(entry) }, admission(run, taxonomyBytes, true))).toThrow(/cancelled during inventory/u);
  expect(progress).toEqual([]);
  console.info(`[DEBUG] mutation source index capture fixture=${run} files=${first.files.length} digest=${first.sourceTreeDigest}`);
});
//#endregion 🧪️Capture
