/** 🟠️ The requires one enabled reachable tests of the direct mutation ownership suite. */
import { mutationRoot, prepareMutationFixtureRoot, mutationFixtureRoot } from "../../../🧬️mutation-authority/🧫️fixture/🟦️.ts";
import Ajv from "ajv";
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import { newScaffoldMutationTree } from "../../../../🏗️authoring/🧬️mutation-tree/🟦️.ts";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { policyMutationStructuralBreaches } from "../../../../🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts";
import { planMutationTaxonomy } from "../../../../🧹️normalization/🧬️mutation/🔁️workflow/🟦️.ts";
import { inventoryMutationTaxonomy } from "../../../../🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🟡 Registers the requires one enabled reachable tests of the direct mutation ownership suite. */
export function registerRequiresOneEnabledReachableTests(): void {
  test("requires one enabled reachable non-ignored leaf test proven by parsed Rust items", () => {
    const fixturePath = join(contractRoot, "../🧫️fixtures/✅️mutation-test-presence/🔣️.json");
    
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { schemaVersion: 1; mutationRoot: string; leaf: string; cases: readonly { name: string; source: string; files: Readonly<Record<string, string>>; links: Readonly<Record<string, string>>; compilerAccepted: boolean; runnableTests: number; policyAccepted: boolean }[] };
    
    const artifactDirectory = process.env.SEMIO_TEST_ARTIFACT_DIR?.trim();
    const base = artifactDirectory ? resolve(getWorkspaceRoot(), artifactDirectory) : realpathSync(tmpdir());
    const oracleTimeoutMs = 30_000;
    mkdirSync(base, { recursive: true });
    const root = mkdtempSync(join(base, "semio-mutation-test-presence-"));
    try {
      for (const vector of fixture.cases) {
        const caseRoot = join(root, vector.name);
        mkdirSync(caseRoot, { recursive: true });
        prepareMutationFixtureRoot(caseRoot);
        const mutations = join(caseRoot, fixture.mutationRoot);
        const leaf = join(mutations, fixture.leaf);
        mkdirSync(leaf, { recursive: true });
        writeFileSync(join(leaf, "🦀️.rs"), vector.source);
        for (const [path, source] of Object.entries(vector.files)) {
          const target = resolve(leaf, path);
          mkdirSync(dirname(target), { recursive: true });
          writeFileSync(target, source);
        }
        for (const [path, target] of Object.entries(vector.links)) {
          const link = resolve(leaf, path);
          mkdirSync(dirname(link), { recursive: true });
          symlinkSync(resolve(leaf, target), link, process.platform === "win32" ? "junction" : "dir");
        }
        const aggregate = join(mutations, "🦀️.rs");
        writeFileSync(aggregate, `#[path = "${fixture.leaf}/🦀️.rs"] pub mod insert_page; pub enum ProbeMutation { InsertPage(insert_page::InsertPage) }\n`);
        const executable = join(caseRoot, process.platform === "win32" ? "oracle.exe" : "oracle");
        const compile = spawnSync("rustc", ["--test", "--edition", "2021", "--crate-name", "mutation_test_presence_oracle", aggregate, "-o", executable], { encoding: "utf8", timeout: oracleTimeoutMs });
        writeFileSync(join(caseRoot, "compiler.stdout.log"), compile.stdout ?? "");
        writeFileSync(join(caseRoot, "compiler.stderr.log"), compile.stderr ?? "");
        expect(compile.signal).toBeNull();
        expect(compile.status === 0).toBe(vector.compilerAccepted);
        if (!vector.compilerAccepted) {
          expect(policyMutationStructuralBreaches(caseRoot, [fixture.mutationRoot]).some(({ kind }) => kind === "mutation/test-presence")).toBe(true);
          continue;
        }
        const execution = spawnSync(executable, ["--nocapture"], { encoding: "utf8", timeout: oracleTimeoutMs });
        writeFileSync(join(caseRoot, "runtime.stdout.log"), execution.stdout ?? "");
        writeFileSync(join(caseRoot, "runtime.stderr.log"), execution.stderr ?? "");
        expect(execution.signal).toBeNull();
        expect(execution.status).toBe(0);
        expect(Number(/test result: ok\. (\d+) passed;/u.exec(execution.stdout)?.[1])).toBe(vector.runnableTests);
        const accepted = policyMutationStructuralBreaches(caseRoot, [fixture.mutationRoot]).every(({ kind }) => kind !== "mutation/test-presence");
        expect({ name: vector.name, runnable: vector.runnableTests, accepted }).toEqual({ name: vector.name, runnable: vector.runnableTests, accepted: vector.policyAccepted });
      }
    } finally {
      if (!artifactDirectory) rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);
  test("accepts a direct authoritative leaf with optional facets and ignores the same violation under compose", () => {
    const root = mutationFixtureRoot("semio-direct-mutation-policy-");
    try {
      const mutations = mutationRoot(root);
      const leaf = join(mutations, "➕️insert-page");
      mkdirSync(join(leaf, "🔺️diff"), { recursive: true });
      mkdirSync(join(leaf, "↩️inverse"), { recursive: true });
      mkdirSync(join(leaf, "🧪️tests"), { recursive: true });
      writeFileSync(join(leaf, "🦀️.rs"), "pub struct InsertPage; pub const DESCRIPTOR: MutationDescriptor = descriptor(); impl MutationKind for InsertPage {}\n");
      writeFileSync(join(leaf, "🔣️.json"), `${JSON.stringify({ schemaVersion: 1, owner: relative(root, leaf).replaceAll("\\", "/"), semanticKind: "insert-page", displayName: "Insert Page", emoji: "➕️", aggregateVariant: "InsertPage", payloadSchema: "🦀️.rs#InsertPage", textOpcode: null, binaryTag: null, invertibility: "explicit-mutation", diffParticipation: "detect", outcomeClasses: ["applied"], composition: "atomic", requiredLanguageSurfaces: ["rust"] }, null, 2)}\n`);
      writeFileSync(join(leaf, "🔺️diff", "🦀️.rs"), "pub fn diff() {}\n");
      writeFileSync(join(leaf, "↩️inverse", "🦀️.rs"), "pub fn inverse() {}\n");
      writeFileSync(join(leaf, "🧪️tests", "🦀️.rs"), "#[test] fn applies() {}\n");
      writeFileSync(join(mutations, "🦀️.rs"), "pub use super::insert_page::*; #[derive(Mutations)] pub enum PdfMutation { InsertPage(insert_page::InsertPage) }\n");
      const opaque = mutationRoot(root, "compose");
      mkdirSync(join(opaque, "❌️broken", "🦠️mutation"), { recursive: true });
      writeFileSync(join(opaque, "❌️broken", "🦠️mutation", "🦀️.rs"), "pub struct Broken;\n");
      expect(policyMutationStructuralBreaches(root).filter(({ kind }) => ["mutation/direct-owner", "mutation/root-purity", "mutation/descriptor-bijection", "mutation/no-sentinel", "mutation/no-generic-snapshot-fallback"].includes(kind))).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 30_000);
  test("scaffolds one direct mutation idempotently without overwriting implementation", () => {
    const root = mutationFixtureRoot("semio-direct-mutation-scaffold-");
    try {
      const mutations = mutationRoot(root);
      mkdirSync(mutations, { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation {}\n");
      const first = newScaffoldMutationTree(root, relative(root, mutations).replaceAll("\\", "/"), "➕️insert-page", { text: true, binary: true, jsonSchema: true }, false);
      expect(first.created).toContain(`${relative(root, mutations).replaceAll("\\", "/")}/➕️insert-page/🦀️.rs`);
      const descriptor = JSON.parse(readFileSync(join(mutations, "➕️insert-page", "🔣️.json"), "utf8"));
      expect(descriptor).toMatchObject({ schemaVersion: 1, owner: `${relative(root, mutations).replaceAll("\\", "/")}/➕️insert-page`, semanticKind: "insert-page", payloadSchema: "🧬️schema/🔣️.json", textOpcode: "insert-page", invertibility: "explicit-mutation", diffParticipation: "detect", outcomeClasses: ["applied"], requiredLanguageSurfaces: ["rust", "json-schema", "text", "binary"] });
      const descriptorSchema = JSON.parse(readFileSync(join(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️.json"), "utf8"));
      const validateDescriptor = new Ajv({ allErrors: true, strict: true }).compile(descriptorSchema);
      expect(validateDescriptor(descriptor)).toBe(true);
      expect(validateJsonSchemaSubset(descriptorSchema, descriptor)).toEqual([]);
      for (const candidate of [
        { ...descriptor, binaryTag: -1 },
        { ...descriptor, requiredLanguageSurfaces: ["typescript"] },
        { ...descriptor, outcomeClasses: [] },
        { ...descriptor, invertibility: "unclassified" },
        { ...descriptor, compatibilityAlias: "legacy" },
      ]) {
        expect(validateJsonSchemaSubset(descriptorSchema, candidate).length === 0).toBe(validateDescriptor(candidate));
      }
      const payloadSchema = JSON.parse(readFileSync(join(mutations, "➕️insert-page", "🧬️schema/🔣️.json"), "utf8"));
      expect(new Ajv({ allErrors: true, strict: true }).compile(payloadSchema)({})).toBe(true);
      writeFileSync(join(mutations, "➕️insert-page", "🦀️.rs"), "pub struct Handwritten;\n");
      const second = newScaffoldMutationTree(root, relative(root, mutations).replaceAll("\\", "/"), "➕️insert-page", { text: true, binary: true, jsonSchema: true }, false);
      expect(second.created).toEqual([]);
      expect(readFileSync(join(mutations, "➕️insert-page", "🦀️.rs"), "utf8")).toBe("pub struct Handwritten;\n");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 30_000);
  test("inventories direct and legacy records as stable language-neutral JSON validated by Ajv", () => {
    const root = mutationFixtureRoot("semio-direct-mutation-inventory-");
    try {
      const mutations = mutationRoot(root);
      mkdirSync(join(mutations, "➕️insert-page"), { recursive: true });
      mkdirSync(join(mutations, "➖️remove-page", "🦠️mutation"), { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation { InsertPage(InsertPage), RemovePage(RemovePage) }\n");
      writeFileSync(join(mutations, "➕️insert-page", "🦀️.rs"), 'pub struct InsertPage; impl MutationKind for InsertPage { const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "page", kind: "insert-page", record: "InsertedPage" }; }\n');
      writeFileSync(join(mutations, "➖️remove-page", "🦠️mutation", "🦀️.rs"), "pub struct RemovePage;\n");
      const inventory = inventoryMutationTaxonomy(root);
      expect(inventory.records.map(({ state }) => state).sort()).toEqual(["direct", "legacy"]);
      const validate = new Ajv({ strict: true }).compile({ type: "object", required: ["schemaVersion", "kind", "sourceTreeDigest", "roots", "sourceRoster", "records", "unresolved", "violations"], properties: { schemaVersion: { const: 2 }, kind: { const: "mutation" }, sourceTreeDigest: { type: "string", pattern: "^[a-f0-9]{64}$" }, roots: { type: "array" }, sourceRoster: { type: "array" }, records: { type: "array" }, unresolved: { type: "array" }, violations: { type: "array" } }, additionalProperties: false });
      expect(validate(JSON.parse(JSON.stringify(inventory)))).toBe(true);
      const plan = planMutationTaxonomy(inventory, "d03b1fdb6da7c4ea97043e5618d8f4098a43dff7");
      expect(plan.moves).toEqual([{ source: `${relative(root, mutations).replaceAll("\\", "/")}/➖️remove-page/🦠️mutation/🦀️.rs`, destination: `${relative(root, mutations).replaceAll("\\", "/")}/➖️remove-page/🦀️.rs` }]);
      expect(plan.unresolved).toEqual(expect.arrayContaining([expect.objectContaining({ path: `${relative(root, mutations).replaceAll("\\", "/")}/➖️remove-page` })]));
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 30_000);
}
