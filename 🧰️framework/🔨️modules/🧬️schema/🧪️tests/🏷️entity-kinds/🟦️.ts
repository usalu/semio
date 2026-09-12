import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { ENTITY_KINDS, ENTITY_KIND_BY_EMOJI, entityKindByEmoji } from "../../🤖️generated/🏷️entity-kinds/🟦️";
import { entityKindIndexByEmoji, parseEntityKind, parseEntityKindCatalog } from "../../🟦️";

/** 🏷️ Third-party oracle for the `framework.schema` entity-kind catalog: the single source
 * `🔣️entity-kinds.json` must satisfy `🔣️.json#/$defs/EntityKindCatalog` in `ajv` exactly as it does in
 * the owned draft-07 validator and in the hand-written `parseEntityKindCatalog`, and the three
 * generated projections must carry the same 58 entries, the same provenance sha256 and the same
 * FIRST-WINS emoji index. `ajv` is a test-only oracle — no production code in this repo may depend
 * on it.
 * @see https://ajv.js.org/json-schema.html */
const moduleRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const repoRoot = join(moduleRoot, "..", "..", "..");
const SCHEMA_ID = "https://semio.tech/schema/framework/schema/schema.json";
const facet = JSON.parse(readFileSync(join(moduleRoot, "🔣️.json"), "utf8")) as Record<string, unknown>;
const sourceBytes = readFileSync(join(moduleRoot, "🔣️entity-kinds.json"));
const sourceSha256 = createHash("sha256").update(sourceBytes).digest("hex");
const catalog = JSON.parse(sourceBytes.toString("utf8")) as unknown;
const ajvConstructor = (Ajv as unknown as { readonly default?: typeof Ajv }).default ?? Ajv;

function oracle(exportId: "EntityKind" | "EntityKindCatalog"): (value: unknown) => boolean {
  const ajv = new ajvConstructor({ allErrors: true, strict: false });
  ajv.addSchema(facet);
  const validate = ajv.getSchema(`${SCHEMA_ID}#/$defs/${exportId}`);
  if (!validate) throw new Error(`ajv could not resolve ${SCHEMA_ID}#/$defs/${exportId}`);
  return (value) => validate(value) === true;
}

const entry = { id: "definition-implementation", emoji: "🛠️", iconId: "wrench", label: "Implementation", filterable: true };
const rejected: readonly (readonly [string, unknown])[] = [
  ["not an object", "technology-user"],
  ["a missing member", { id: entry.id, emoji: entry.emoji, iconId: entry.iconId, label: entry.label }],
  ["an undeclared property", { ...entry, color: "red" }],
  ["a non-kebab id", { ...entry, id: "Definition-Implementation" }],
  ["a one-character id", { ...entry, id: "ab" }],
  ["an emoji carrying whitespace", { ...entry, emoji: "🛠️ " }],
  ["an empty emoji", { ...entry, emoji: "" }],
  ["a non-kebab iconId", { ...entry, iconId: "Wrench" }],
  ["an untrimmed label", { ...entry, label: " Implementation" }],
  ["a one-character label", { ...entry, label: "I" }],
  ["a non-boolean filterable", { ...entry, filterable: "yes" }],
];

describe("framework.schema entity-kind catalog facet", () => {
  it("declares both serialized documents at its root", () => {
    expect(facet.$schema).toBe("http://json-schema.org/draft-07/schema#");
    expect(facet.$id).toBe(SCHEMA_ID);
    expect(facet.oneOf).toStrictEqual([{ $ref: "#/$defs/SchemaExportEntries" }, { $ref: "#/$defs/EntityKindCatalog" }]);
  });

  it("accepts 🔣️entity-kinds.json in ajv and in the owned parser", () => {
    expect(oracle("EntityKindCatalog")(catalog)).toBe(true);
    expect(parseEntityKindCatalog(catalog)).toStrictEqual(ENTITY_KINDS);
    expect(ENTITY_KINDS).toHaveLength(58);
  });

  it("rejects the same malformed entries in ajv and in the owned parser", () => {
    const validate = oracle("EntityKind");
    for (const [reason, value] of rejected) {
      expect(validate(value), `ajv accepted ${reason}`).toBe(false);
      expect(() => parseEntityKind(value), `the owned parser accepted ${reason}`).toThrow();
    }
    expect(validate(entry)).toBe(true);
    expect(parseEntityKind(entry)).toStrictEqual(entry);
  });

  it("rejects an empty catalog and a duplicated entry in both", () => {
    const validate = oracle("EntityKindCatalog");
    expect(validate([])).toBe(false);
    expect(() => parseEntityKindCatalog([])).toThrow();
    expect(validate([entry, { ...entry }])).toBe(false);
    expect(() => parseEntityKindCatalog([entry, { ...entry }])).toThrow();
    expect(() => parseEntityKindCatalog([entry, { ...entry, emoji: "🔧️", iconId: "hammer" }])).toThrow(/id definition-implementation is already declared/);
  });

  it("indexes emoji FIRST-WINS, shadowing exactly two kinds", () => {
    expect(ENTITY_KIND_BY_EMOJI).toStrictEqual(entityKindIndexByEmoji(ENTITY_KINDS));
    expect(ENTITY_KIND_BY_EMOJI.size).toBe(56);
    expect(entityKindByEmoji("📝️")?.id).toBe("draft");
    expect(entityKindByEmoji("🌱️")?.id).toBe("technology-mono");
    expect(entityKindByEmoji("🦕️")).toBeUndefined();
    expect(ENTITY_KINDS.filter((kind) => entityKindByEmoji(kind.emoji)?.id !== kind.id).map((kind) => kind.id)).toStrictEqual(["todo", "interaction-started"]);
  });

  it("stamps every projection with the same generator and source sha256", () => {
    const projections = ["🧰️framework/🔨️modules/🧬️schema/🤖️generated/🏷️entity-kinds/🟦️.ts", "🧰️framework/🔨️modules/🧬️schema/🤖️generated.rs", "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🏷️entity-kinds/🐹️.go"];
    for (const projection of projections) {
      const head = readFileSync(join(repoRoot, projection), "utf8").split("\n").slice(0, 2).join("\n");
      expect(head, projection).toContain("@generated by `schema-entity-catalog` from `🧰️framework/🔨️modules/🧬️schema/🔣️entity-kinds.json`");
      expect(head, projection).toContain(`(sha256 ${sourceSha256})`);
      expect(head, projection).toContain("bun nx run @semio-tech/framework-schema:generate");
    }
  });
});
