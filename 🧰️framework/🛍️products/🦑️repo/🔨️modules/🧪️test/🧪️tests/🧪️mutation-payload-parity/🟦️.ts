import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import Ajv2019 from "ajv/dist/2019";
import { type MutationFeatureRow, type MutationLeafWrapper, mutationAggregateBranch, mutationFeatureRows, mutationFixtureOutcome, mutationInputPayload, mutationPayloadChecker, mutationPayloadParityReport, mutationVariantWireName, rustMutationAggregates, rustValueEnums } from "../../🧬️schema/📋️orchestration/🟦️.ts";

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
type Case = {
  readonly id: string;
  readonly rust: string;
  readonly leafRust?: string;
  readonly aggregate: string;
  readonly variant: string;
  readonly layout: { readonly tag: string | null; readonly content: string | null; readonly renameAll: string | null };
  readonly wireName: string;
  readonly leaf: { readonly $id: string; readonly properties?: Record<string, { readonly const?: Json }> } & Record<string, Json>;
  readonly input: { readonly aggregateSchema: ({ readonly $id: string; readonly oneOf: readonly Json[] } & Record<string, Json>) | null };
  readonly fixture: Json;
  readonly outcome?: Json;
  readonly invariants?: readonly string[];
  readonly payload: Json;
  readonly findings: readonly string[];
};

const corpus = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧫️mutation-payload-parity/🔣️.json"), "utf8")) as { readonly cases: readonly Case[]; readonly features: readonly { readonly id: string; readonly source: string; readonly rows: readonly MutationFeatureRow[] }[] };
const isObject = (value: unknown): value is Record<string, Json> => value !== null && typeof value === "object" && !Array.isArray(value);

/** 🔒️ The leaf schema with `unevaluatedProperties: false` on every object node that declares no `additionalProperties` — the Ajv 2019 reading of "every payload member is described". */
function closed(node: Json): Json {
  if (Array.isArray(node)) return node.map(closed);
  if (!isObject(node)) return node;
  const entries = Object.fromEntries(Object.entries(node).filter(([key]) => key !== "$schema").map(([key, value]) => [key, key === "const" || key === "enum" ? value : closed(value)]));
  const objectNode = node.type === "object" || node.properties !== undefined;
  return objectNode && node.additionalProperties === undefined ? { ...entries, unevaluatedProperties: false } : entries;
}

describe("schema-mutation-payload-parity corpus", () => {
  test("the corpus covers every finding class the lint reports on a fixture", () => {
    const classes = new Set(corpus.cases.flatMap((entry) => entry.findings.map((finding) => finding.split("@")[0])));
    for (const name of ["invalid", "undescribed", "opaque", "layout", "aggregate", "negative"]) expect(classes.has(name)).toBe(true);
  });

  for (const entry of corpus.cases)
    test(entry.id, () => {
      const aggregate = rustMutationAggregates("case.rs", entry.rust).find((candidate) => candidate.name === entry.aggregate);
      expect(aggregate?.layout).toEqual(entry.layout);
      expect(aggregate?.variants.has(entry.variant)).toBe(true);
      const wireName = mutationVariantWireName(entry.variant, aggregate!.variants.get(entry.variant) ?? null, aggregate!.layout.renameAll);
      expect(wireName).toBe(entry.wireName);
      const leafEnum = rustValueEnums("leaf.rs", entry.leafRust ?? "").find((candidate) => candidate.name === aggregate!.payloadTypes.get(entry.variant) && candidate.payloadVariant !== null);
      if (entry.input.aggregateSchema !== null) expect(mutationAggregateBranch(aggregate!.layout, wireName, entry.leaf.$id, leafEnum?.name ?? null)).toEqual(entry.input.aggregateSchema.oneOf[0] as Record<string, unknown>);
      const wrapper: MutationLeafWrapper | null = leafEnum === undefined ? null : { layout: leafEnum.layout, wireName: mutationVariantWireName(leafEnum.payloadVariant!, leafEnum.variants.get(leafEnum.payloadVariant!) ?? null, leafEnum.layout.renameAll) };
      expect(wrapper !== null).toBe(entry.leafRust !== undefined);
      const cut = mutationInputPayload(aggregate!.layout, wireName, entry.fixture, wrapper);
      expect("payload" in cut ? cut.payload : null).toEqual(entry.payload);
      const documents = new Map<string, Record<string, unknown>>([[entry.leaf.$id, entry.leaf], ...(entry.input.aggregateSchema === null ? [] : [[entry.input.aggregateSchema.$id, entry.input.aggregateSchema] as const])]);
      const checker = mutationPayloadChecker(documents);
      if (entry.invariants !== undefined) expect([...checker.invariants(entry.leaf.$id)].sort()).toEqual([...entry.invariants].sort());
      const findings = [...checker.opaque(entry.leaf.$id), ...checker.fixture(aggregate!.layout, wireName, entry.leaf.$id, entry.input.aggregateSchema?.$id ?? null, entry.fixture, wrapper, mutationFixtureOutcome(entry.outcome ?? null))].map((finding) => `${finding.class}@${finding.pointer}`);
      expect(findings.sort()).toEqual([...entry.findings].sort());
    });
});

describe("fixture-to-leaf pairing", () => {
  test("an orphaned fixture stays unmapped instead of borrowing a foreign artifact's aggregate that declares the same variant", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-payload-scope-"));
    try {
      const write = (path: string, body: string): void => {
        mkdirSync(join(root, path, ".."), { recursive: true });
        writeFileSync(join(root, path), body);
      };
      const donor = "✏️s/🔌️plugins/donor/🗿️artifacts/donor/🧬️schema/🧬️mutations";
      const orphan = "✏️s/🔌️plugins/orphan/🗿️artifacts/orphan";
      const aggregate = (name: string, variant: string): string => `#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]\n#[value(tag = "mutation", rename_all = "camelCase")]\n#[mutations(snapshot = S, diff = D, schema = "planted")]\npub enum ${name} {\n    ${variant}(super::leaf::${variant}),\n}\n`;
      const leaf = (directory: string, variant: string, id: string, wire: string): void => {
        write(`${directory}/🔣️.json`, JSON.stringify({ aggregateVariant: variant, payloadSchema: "🧬️schema/🔣️.json", semanticKind: id }));
        write(`${directory}/🧬️schema/🔣️.json`, JSON.stringify({ $id: `https://json.schemas.assets.semio-tech.com/test/${id}/schema.json`, type: "object", additionalProperties: false, required: ["mutation"], properties: { mutation: { const: wire }, id: { type: "string" } } }));
      };
      write("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", "{}");
      write(`${donor}/🦀️.rs`, aggregate("DonorMutation", "MoveBlock"));
      leaf(`${donor}/🔀move-block`, "MoveBlock", "donor-move-block", "moveBlock");
      const fixture = `${orphan}/🧫️fixtures/🧬️mutations/🔀move-block/🧪️rejects/🦠️mutation/🔣️.json`;
      write(fixture, JSON.stringify({ mutation: "moveBlock", id: "block-1" }));
      const findings = mutationPayloadParityReport(root).diagnostics.filter((diagnostic) => diagnostic.path === fixture);
      expect(findings.map((diagnostic) => diagnostic.detail.split(" at ")[0])).toEqual(["unmapped"]);
      expect(findings.every((diagnostic) => !diagnostic.detail.includes("donor"))).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});

describe("wire-form feature rows", () => {
  for (const entry of corpus.features) test(entry.id, () => expect(mutationFeatureRows(entry.source)).toEqual([...entry.rows]));
});

describe("third-party oracle (Ajv draft-07 and Ajv 2019 unevaluatedProperties)", () => {
  for (const entry of corpus.cases.filter((candidate) => isObject(candidate.payload)))
    test(entry.id, () => {
      const instance = { ...Object.fromEntries(Object.entries(entry.leaf.properties ?? {}).flatMap(([key, node]) => (node.const !== undefined ? [[key, node.const]] : []))), ...(entry.payload as Record<string, Json>) };
      const { $schema: _, ...leaf } = entry.leaf;
      const valid = new Ajv({ strict: false }).validate(leaf, instance);
      const outcome = mutationFixtureOutcome(entry.outcome ?? null);
      const declared = outcome.invariant !== null && (entry.invariants ?? []).includes(outcome.invariant);
      expect(valid).toBe(outcome.direction === "negative" ? entry.findings.includes("negative@") || declared : !entry.findings.includes("invalid@"));
      const ajv = new Ajv2019({ strict: false, allErrors: true });
      ajv.validate(closed(leaf as Json) as object, instance);
      const undescribed = (ajv.errors ?? []).filter((error) => error.keyword === "unevaluatedProperties").map((error) => `undescribed@${error.instancePath}/${(error.params as { unevaluatedProperty: string }).unevaluatedProperty}`);
      expect(undescribed.sort()).toEqual(entry.findings.filter((finding) => finding.startsWith("undescribed@")).sort());
    });
});
