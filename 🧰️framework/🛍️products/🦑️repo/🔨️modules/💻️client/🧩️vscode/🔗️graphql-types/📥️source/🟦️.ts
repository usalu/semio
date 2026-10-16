/** 📥️ Inputs of the GraphQL type generation: the served schema document and the operations the extension sends. */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join, relative } from "node:path";
import type { GraphqlTypesSource } from "../📽️projection/🟦️.ts";

//#region 📥️Source
/** 📜️ The committed SDL every implementation and every client reads: `🔗️graphql/🧪️tests/🧫️fixtures/📜️served-schema/🔗️.graphql`. */
export const SCHEMA_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🧪️tests/🧫️fixtures/📜️served-schema/🔗️.graphql";

function vscodeRoot(): string {
  return join(dirname(fileURLToPath(import.meta.url)), "..", "..");
}

function sha256(bytes: string | Uint8Array): string {
  return createHash("sha256").update(bytes).digest("hex");
}

/** ⌛️ Reads the schema and the `…Document` string exports of `⌛️queries`; operations are ordered by name so the digest is stable. */
export async function readGraphqlTypesSource(repoRoot: string): Promise<GraphqlTypesSource> {
  const schema = readFileSync(join(repoRoot, SCHEMA_REL), "utf8");
  const queriesPath = join(vscodeRoot(), "⌛️queries", "🟦️.ts");
  const exported = (await import(queriesPath)) as Record<string, unknown>;
  const operations = Object.entries(exported)
    .filter(([name, value]) => name.endsWith("Document") && typeof value === "string")
    .map(([name, value]) => ({ name: name.slice(0, -"Document".length), source: value as string }))
    .sort((left, right) => (left.name < right.name ? -1 : 1));
  if (operations.length === 0) throw new Error("GraphQL types: ⌛️queries exports no …Document operation");
  const digest = sha256(operations.map((operation) => `${operation.name}\0${operation.source}\n`).join(""));
  return {
    schemaRel: SCHEMA_REL,
    schemaSha256: sha256(schema),
    schema,
    operationsRel: relative(repoRoot, queriesPath).replaceAll("\\", "/").normalize("NFC"),
    operationsSha256: digest,
    operations,
  };
}
//#endregion 📥️Source
