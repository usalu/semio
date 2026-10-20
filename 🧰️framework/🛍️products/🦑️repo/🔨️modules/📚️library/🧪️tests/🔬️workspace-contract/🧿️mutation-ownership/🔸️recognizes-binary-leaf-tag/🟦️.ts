/** 🔸️ The recognizes binary leaf tag tests of the direct mutation ownership suite. */
import { mutationRoot, mutationFixtureRoot } from "../../../🧬️mutation-authority/🧫️fixture/🟦️.ts";
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { loadTaxonomy, canonicalPrimaryFilenameForKind } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { createRustMutationCodecOwnershipInspector } from "../../../../🔍️discovery/🛡️mutation-codec-ownership/🟦️.ts";
import { inspectRustStructure } from "../../../../🔍️discovery/🏛️rust-structure/🟦️.ts";
import { MUTATION_STRUCTURAL_POLICY_KINDS, policyMutationStructuralBreaches } from "../../../../🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🔹 Registers the recognizes binary leaf tag tests of the direct mutation ownership suite. */
export function registerRecognizesBinaryLeafTagTests(): void {
  test("recognizes a binary leaf tag only when its numeric identity equals the descriptor", () => {
    const golden = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🔭️mutation-scope/🔣️.json"), "utf8")) as {
      mutationRoot: string;
      binaryCases: { source: string; accepted: boolean }[];
    };
    const root = mutationFixtureRoot("semio-direct-mutation-binary-identity-");
    try {
      const leaf = `${golden.mutationRoot}/➕️insert-page`;
      mkdirSync(join(root, leaf, "💾️binary"), { recursive: true });
      mkdirSync(join(root, golden.mutationRoot, "💾️binary"), { recursive: true });
      writeFileSync(join(root, golden.mutationRoot, "🦀️.rs"), "pub enum PageMutation { InsertPage(insert_page::InsertPage) }\n");
      writeFileSync(join(root, golden.mutationRoot, "💾️binary", "🦀️.rs"), "pub const REGISTRY: &[u8] = &[insert_page::binary::BINARY_TAG];\n");
      writeFileSync(join(root, leaf, "🦀️.rs"), "pub struct InsertPage; impl MutationKind for InsertPage {}\n");
      writeFileSync(join(root, leaf, "🔣️.json"), JSON.stringify({ schemaVersion: 1, owner: leaf, semanticKind: "insert-page", displayName: "Insert Page", emoji: "➕️", aggregateVariant: "InsertPage", payloadSchema: "🦀️.rs#InsertPage", textOpcode: null, binaryTag: 3, invertibility: "explicit-mutation", diffParticipation: "detect", outcomeClasses: ["applied"], composition: "atomic", requiredLanguageSurfaces: ["rust", "binary"] }));
      const codec = join(root, leaf, "💾️binary", "🦀️.rs");
      for (const vector of golden.binaryCases) {
        writeFileSync(codec, vector.source);
        const oracle = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-name", "mutation_binary_probe", "--edition", "2021", codec], { encoding: "utf8" });
        expect(oracle.status).toBe(0);
        expect(oracle.stdout.includes('symbol: "3"')).toBe(vector.accepted);
        expect(policyMutationStructuralBreaches(root, [golden.mutationRoot]).filter(({ kind }) => kind === "mutation/language-parity").length === 0).toBe(vector.accepted);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 30_000);
  test("rejects executable root aggregate codecs while preserving generic framing and unrelated syntax", () => {
    const golden = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🔐️mutation-codec-ownership/🔣️.json"), "utf8")) as {
      schemaVersion: number;
      mutationRoot: string;
      aggregate: string;
      cases: { name: string; source: string; expected: string[] }[];
    };
    const root = mutationFixtureRoot("semio-mutation-codec-ownership-");
    try {
      expect(golden.schemaVersion).toBe(1);
      const taxonomy = loadTaxonomy();
      const rustComponent = canonicalPrimaryFilenameForKind(taxonomy.mutationComponentFileKindId, taxonomy);
      const codec = join(root, golden.mutationRoot, "📝️text", rustComponent);
      mkdirSync(dirname(codec), { recursive: true });
      writeFileSync(join(root, golden.mutationRoot, rustComponent), golden.aggregate);
      const compilerOutput = join(root, "📦️compiler-output");
      mkdirSync(compilerOutput, { recursive: true });
      const inspect = createRustMutationCodecOwnershipInspector(golden.aggregate);
      for (const vector of golden.cases) {
        writeFileSync(codec, vector.source);
        const source = `${golden.aggregate}\n${vector.source}`;
        const parserOracle = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-name", "mutation_codec_ownership_probe", "--edition", "2021", "-"], { encoding: "utf8", input: source });
        const compilerOracle = spawnSync("rustc", ["--crate-type=lib", "--crate-name", "mutation_codec_ownership_probe", "--edition", "2021", "--out-dir", compilerOutput, "-"], { encoding: "utf8", input: source });
        expect(parserOracle.status).toBe(0);
        expect(compilerOracle.status === 0).toBe(vector.expected.length === 0);
        expect(inspect(vector.source).map(({ kind }) => kind)).toEqual<readonly string[]>(vector.expected);
        expect(policyMutationStructuralBreaches(root, [golden.mutationRoot]).filter(({ kind }) => kind === "mutation/codec-ownership").length > 0).toBe(vector.expected.length > 0);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
    }  }, 30000);
  test("rejects empty aggregates, inline variants, and orphaned direct folders", () => {
    const golden = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🔭️mutation-scope/🔣️.json"), "utf8")) as {
      mutationRoot: string;
      aggregateCases: { source: string; kind: string; variantCount: number }[];
    };
    const root = mutationFixtureRoot("semio-direct-mutation-aggregate-");
    try {
      mkdirSync(join(root, golden.mutationRoot, "➕️insert-page"), { recursive: true });
      writeFileSync(join(root, golden.mutationRoot, "➕️insert-page", "🦀️.rs"), "pub struct InsertPage;\n");
      const aggregate = join(root, golden.mutationRoot, "🦀️.rs");
      for (const vector of golden.aggregateCases) {
        writeFileSync(aggregate, vector.source);
        const oracle = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-name", "mutation_aggregate_probe", "--edition", "2021", aggregate], { encoding: "utf8" });
        expect(oracle.status).toBe(0);
        expect((oracle.stdout.match(/\bVariant \{/gu) ?? []).length).toBe(vector.variantCount);
        const facts = inspectRustStructure(vector.source);
        expect(facts.enums.flatMap((item) => item.variants)).toHaveLength(vector.variantCount);
        expect(policyMutationStructuralBreaches(root, [golden.mutationRoot]).some(({ kind }) => kind === vector.kind)).toBe(true);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
    }  }, 30000);
  test("rejects aggregate-state mutation inputs transitively while accepting semantic prior values", () => {
    const golden = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/📥️mutation-input-carriers/🔣️.json"), "utf8")) as {
      schemaVersion: number;
      mutationRoot: string;
      aggregate: string;
      compilerPrelude: string;
      compilerAssertion: string;
      cases: { name: string; source: string; aggregate?: string; forbidden: boolean }[];
    };
    const root = mutationFixtureRoot("semio-direct-mutation-input-");
    try {
      expect(golden.schemaVersion).toBe(1);
      const leaf = join(root, golden.mutationRoot, "➕️insert-page");
      mkdirSync(leaf, { recursive: true });
      for (const vector of golden.cases) {
        writeFileSync(join(root, golden.mutationRoot, "🦀️.rs"), vector.aggregate ?? golden.aggregate);
        writeFileSync(join(leaf, "🦀️.rs"), vector.source);
        const oracle = spawnSync("rustc", ["--crate-name", "mutation_input_probe", "--edition", "2021", "--crate-type", "lib", "--emit=metadata", "-o", join(root, "oracle.rmeta"), "-"], { input: golden.compilerPrelude + vector.source + golden.compilerAssertion, encoding: "utf8" });
        expect(oracle.status === 0).toBe(!vector.forbidden);
        if (vector.forbidden) expect(oracle.stderr).toContain("E0277");
        const breaches = policyMutationStructuralBreaches(root, [golden.mutationRoot]).filter(({ kind }) => kind === "mutation/no-generic-snapshot-fallback");
        expect({ name: vector.name, forbidden: breaches.length > 0 }).toEqual({ name: vector.name, forbidden: vector.forbidden });
        expect(breaches.every(({ priority }) => priority === "high")).toBe(true);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
    }  }, 30000);
  test("reports legacy nesting, inline root behavior, sentinels, and snapshot fallbacks at high severity", () => {
    const root = mutationFixtureRoot("semio-direct-mutation-policy-");
    try {
      const mutations = mutationRoot(root);
      const leaf = join(mutations, "➕️insert-page");
      mkdirSync(join(leaf, "🦠️mutation"), { recursive: true });
      writeFileSync(join(leaf, "🦠️mutation", "🦀️.rs"), "pub struct InsertPage;\n");
      writeFileSync(join(mutations, "🦀️.rs"), "pub const KINDS: &[&str] = &[\"insert-page\"]; pub enum PdfMutation { NoMutation, SetSnapshot(Vec<u8>), InsertPage(InsertPage) } pub fn apply(m: PdfMutation) { match m { _ => {} } }\n");
      const findings = policyMutationStructuralBreaches(root);
      const kinds = new Set(findings.map(({ kind }) => kind));
      for (const kind of ["mutation/direct-owner", "mutation/root-purity", "mutation/no-sentinel", "mutation/no-generic-snapshot-fallback"]) expect(kinds.has(kind)).toBe(true);
      expect(findings.every(({ priority }) => priority === "high")).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("registers every architectural invariant as a high-severity structural policy", () => {
    expect(MUTATION_STRUCTURAL_POLICY_KINDS).toEqual([
      "mutation/direct-owner", "mutation/root-purity", "mutation/folder-variant-bijection", "mutation/descriptor-bijection", "mutation/reachability", "mutation/behavior-ownership", "mutation/codec-ownership", "mutation/wire-identity", "mutation/schema-parity", "mutation/language-parity", "mutation/catalog-parity", "mutation/no-hidden-generation", "mutation/no-sentinel", "mutation/no-generic-snapshot-fallback", "mutation/shared-helper-purity", "mutation/test-presence", "mutation/compose-exclusion",
    ]);
    const root = mutationFixtureRoot("semio-direct-mutation-policy-catalog-");
    try {
      const mutations = mutationRoot(root);
      mkdirSync(join(mutations, "➕️insert-page"), { recursive: true });
      writeFileSync(join(mutations, "🦀️.rs"), "pub enum ProbeMutation { InsertPage(InsertPage) }\n");
      expect(policyMutationStructuralBreaches(root).every(({ priority }) => priority === "high")).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 30_000);
}
