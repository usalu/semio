//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { parseTechnologyCatalog, parseTechnologyCatalogEntry, VSCODE_PARSERS, VSCODE_SCHEMA_ID, type TechnologyCatalogEntry } from "../../🧬️schema/🟦️";
//#endregion 🔌️Adapters

//#region 🧫️Fixtures
const ownerRoot = join(import.meta.dirname, "../..");
const document = JSON.parse(readFileSync(join(ownerRoot, "🧬️schema/🔣️.json"), "utf-8")) as {
  $schema: string;
  $id: string;
  $ref: string;
  $defs: Record<string, Record<string, unknown>>;
};
const catalog = JSON.parse(readFileSync(join(ownerRoot, "🗂️technologies.json"), "utf-8")) as unknown;

/** ⚖️ Third-party draft-07 oracle every case is cross-checked against. */
function oracle(exportId: string): (data: unknown) => boolean {
  const AjvConstructor = createRequire(import.meta.url)("ajv") as new (options: Record<string, unknown>) => { compile(schema: unknown): (data: unknown) => boolean };
  return new AjvConstructor({ strict: false }).compile({ ...document, $id: `${document.$id}#${exportId}`, $ref: `#/$defs/${exportId}` });
}

const entry: TechnologyCatalogEntry = { id: "definition-implementation", emoji: "🛠️", iconId: "wrench", label: "Implementation", filterable: true };

/** 🚫️ Entries every implementation must reject, with the keyword that rejects them. */
const rejectedEntries: readonly (readonly [string, unknown])[] = [
  ["unknown property", { ...entry, extra: 1 }],
  ["missing filterable", { id: entry.id, emoji: entry.emoji, iconId: entry.iconId, label: entry.label }],
  ["non-kebab id", { ...entry, id: "Definition-Implementation" }],
  ["id too short", { ...entry, id: "ab" }],
  ["empty emoji", { ...entry, emoji: "" }],
  ["emoji with whitespace", { ...entry, emoji: "🛠️ " }],
  ["non-kebab icon", { ...entry, iconId: "check_circle_2" }],
  ["untrimmed label", { ...entry, label: "Implementation " }],
  ["label too short", { ...entry, label: "I" }],
  ["filterable not boolean", { ...entry, filterable: "true" }],
  ["array instead of object", [entry]],
  ["null", null],
];
//#endregion 🧫️Fixtures

//#region 🧪️Module
describe("repo client vscode technology catalog module", () => {
  it("declares the canonical draft-07 identity", () => {
    expect(document.$schema).toBe("http://json-schema.org/draft-07/schema#");
    expect(document.$id).toBe("https://semio.tech/schema/repo/client/vscode/schema.json");
    expect(VSCODE_SCHEMA_ID).toBe(document.$id);
    expect(document.$ref).toBe("#/$defs/TechnologyCatalog");
  });

  it("publishes one parser per export", () => {
    expect(Object.keys(VSCODE_PARSERS).sort()).toEqual(Object.keys(document.$defs).sort());
    expect(Object.keys(document.$defs).sort()).toEqual(["TechnologyCatalog", "TechnologyCatalogEntry"]);
  });
});
//#endregion 🧪️Module

//#region 🧪️Data
describe("🗂️technologies.json is an instance of the module", () => {
  it("passes our parser", () => {
    const parsed = parseTechnologyCatalog(catalog);
    expect(parsed.success ? "" : parsed.error.message).toBe("");
  });

  it("passes the ajv draft-07 oracle", () => {
    expect(oracle("TechnologyCatalog")(catalog)).toBe(true);
  });

  it("declares every entity kind id and every label exactly once", () => {
    const parsed = parseTechnologyCatalog(catalog);
    expect(parsed.success).toBe(true);
    if (!parsed.success) return;
    expect(new Set(parsed.data.map((row) => row.id)).size).toBe(parsed.data.length);
    expect(new Set(parsed.data.map((row) => row.label)).size).toBe(parsed.data.length);
  });

  it("carries the six non-filterable aggregate kinds", () => {
    const parsed = parseTechnologyCatalog(catalog);
    expect(parsed.success).toBe(true);
    if (!parsed.success) return;
    expect(parsed.data.filter((row) => !row.filterable).map((row) => row.id)).toEqual(["codebase", "technologies", "bundles", "folders", "files", "definitions"]);
  });

  it("keeps the first declaration of every shared emoji", () => {
    const parsed = parseTechnologyCatalog(catalog);
    expect(parsed.success).toBe(true);
    if (!parsed.success) return;
    const index = new Map<string, string>();
    for (const row of parsed.data) if (!index.has(row.emoji)) index.set(row.emoji, row.id);
    expect(index.get("🌱️")).toBe("technology-mono");
    expect(index.get("📝️")).toBe("draft");
    expect(parsed.data.length - index.size).toBe(2);
  });
});
//#endregion 🧪️Data

//#region 🧪️Parity
describe("parser and ajv agree on every case", () => {
  it.each(rejectedEntries.map(([name, value]) => [name, value] as const))("rejects an entry with %s", (_name, value) => {
    expect(parseTechnologyCatalogEntry(value).success).toBe(false);
    expect(oracle("TechnologyCatalogEntry")(value)).toBe(false);
  });

  it("accepts a well-formed entry in both", () => {
    expect(parseTechnologyCatalogEntry(entry).success).toBe(true);
    expect(oracle("TechnologyCatalogEntry")(entry)).toBe(true);
  });

  it("rejects a repeated entry in both", () => {
    expect(parseTechnologyCatalog([entry, { ...entry }]).success).toBe(false);
    expect(oracle("TechnologyCatalog")([entry, { ...entry }])).toBe(false);
  });

  it("rejects an empty catalog in both", () => {
    expect(parseTechnologyCatalog([]).success).toBe(false);
    expect(oracle("TechnologyCatalog")([])).toBe(false);
  });
});
//#endregion 🧪️Parity
