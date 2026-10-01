import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { isAbsolute, join } from "node:path";
import Ajv from "ajv";
import { afterEach, describe, expect, it } from "vitest";
import {
  buildTrustedStdioCatalogV1,
  encodeTrustedStdioCatalogV1,
  loadFirstPartyStdioNativeCodecReceipts,
  publishTrustedStdioCatalogV1,
  verifyStdioNativeCodecReceipts,
} from "../../✅️trusted-stdio-catalog/🟦️.ts";

const fixtureRoot = join(import.meta.dirname, "../../🧫️fixtures/🧬️trusted-stdio-catalog");
const schemaRoot = join(import.meta.dirname, "../../🧬️schema/🧬️trusted-stdio-catalog");
const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as {
  readonly schemaVersion: 1;
  readonly multipleStandards: { readonly artifactKind: string; readonly artifactSchemas: readonly string[]; readonly factoryIds: readonly string[] };
  readonly expectedOpenTargetKind: string;
  readonly expectedPublication: string;
  readonly expectedHubBundle: string;
  readonly structuralCodec: { readonly artifactKind: string; readonly packSchemaHash: string; readonly protocolSourceSha256: string };
  readonly hostile: { readonly emptyReceipts: string; readonly digestMismatch: string; readonly identityMismatch: string; readonly missingProtocol: string; readonly missingDefinition: string };
};
const schema = JSON.parse(readFileSync(join(schemaRoot, "🔣️.json"), "utf8"));
const temporaryRoots: string[] = [];
const testRoot = (): string => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output || !isAbsolute(output)) throw new Error("SEMIO_TEST_ARTIFACT_DIR must name an absolute caller-owned test output root");
  mkdirSync(output, { recursive: true });
  const directory = mkdtempSync(join(output, "trusted-stdio-catalog-"));
  temporaryRoots.push(directory);
  return directory;
};

afterEach(() => {
  while (temporaryRoots.length) rmSync(temporaryRoots.pop()!, { recursive: true, force: true });
});

describe("trusted stdio catalog", () => {
  it("matches the language-neutral fixture schema and publishes all contributed first-party codecs", () => {
    const ajv = new Ajv({ strict: true, allErrors: true });
    ajv.addSchema(schema);
    const validateFixture = ajv.compile(schema);
    expect(validateFixture(fixture)).toBe(true);
    const receiptSchema = JSON.parse(readFileSync(join(import.meta.dirname, "../../../📇️catalog/🧬️schema/🔣️.json"), "utf8"));
    ajv.addSchema(receiptSchema);
    const validateReceipts = ajv.getSchema(`${receiptSchema.$id}#/$defs/NativeCodecFactories`)!;
    const source = JSON.parse(readFileSync(join(import.meta.dirname, "../../../🔌️plugin/📇️catalog/📜️native-codec-factories.json"), "utf8"));
    expect(validateReceipts(source), JSON.stringify(validateReceipts.errors)).toBe(true);
    const extra = structuredClone(source);
    extra.receipts[0].pack_schema_sha256 = extra.receipts[0].pack_schema_hash;
    expect(validateReceipts(extra)).toBe(false);
    const catalog = buildTrustedStdioCatalogV1();
    const validateCatalog = ajv.getSchema(`${schema.$id}#/$defs/TrustedStdioCatalogV1`);
    expect(validateCatalog).toBeTruthy();
    expect(validateCatalog!(catalog), JSON.stringify(validateCatalog!.errors)).toBe(true);
    expect(catalog.nativeCodecs).toHaveLength(loadFirstPartyStdioNativeCodecReceipts().length);
    expect(catalog.openTargets).toHaveLength(1);
    expect(catalog.openTargets[0]?.artifactKind).toBe(fixture.expectedOpenTargetKind);
    expect(catalog.publication).toBe(fixture.expectedPublication);
    expect(catalog.hubBundle).toBe(fixture.expectedHubBundle);
    expect(catalog.pluginId).toBe("stdio");
    expect(catalog.packageId).toBe("semio:stdio");
    const json = catalog.nativeCodecs.find((row) => row.artifactKind === "s.stdio.json");
    expect(json?.factoryId).toBe("stdio.native.json.v1");
    expect(catalog.openTargets[0]?.packSchemaHash).toBe(json?.packSchemaHash);
  });

  it("separates declared pack identities from Node crypto SHA-256 of every contributed protocol source", () => {
    const receipts = loadFirstPartyStdioNativeCodecReceipts();
    const codecs = verifyStdioNativeCodecReceipts(receipts);
    expect(codecs).toHaveLength(loadFirstPartyStdioNativeCodecReceipts().length);
    const kinds = codecs.map((row) => [row.artifactKind, row.artifactSchema, row.factoryId].join("\0"));
    expect(new Set(kinds).size).toBe(loadFirstPartyStdioNativeCodecReceipts().length);
    expect(kinds).toEqual([...kinds].sort((left, right) => (left < right ? -1 : left > right ? 1 : 0)));
    const multiple = codecs.filter(row => row.artifactKind === fixture.multipleStandards.artifactKind);
    expect(multiple.map(row => row.artifactSchema).sort()).toEqual([...fixture.multipleStandards.artifactSchemas].sort());
    expect(multiple.map(row => row.factoryId).sort()).toEqual([...fixture.multipleStandards.factoryIds].sort());
    expect(() => verifyStdioNativeCodecReceipts([receipts[0]!, receipts[0]!])).toThrow(/duplicate native codec/);
    for (const receipt of receipts) {
      expect(createHash("sha256").update(readFileSync(receipt.protocolPath)).digest("hex")).toBe(receipt.protocolSourceSha256);
    }
    const las = codecs.find((row) => row.artifactKind === fixture.structuralCodec.artifactKind)!;
    expect(las.packSchemaHash).toBe(fixture.structuralCodec.packSchemaHash);
    expect(las.protocolSourceSha256).toBe(fixture.structuralCodec.protocolSourceSha256);
    expect(las.packSchemaHash).not.toBe(las.protocolSourceSha256);
    for (const row of codecs) {
      expect(row.packSchemaHash).toMatch(/^[0-9a-f]{64}$/);
      expect(row.packSchemaHash).not.toBe("0".repeat(64));
    }
  });

  it("writes a committed catalog whose bytes hash under the Node crypto oracle", () => {
    const outDir = testRoot();
    const receipt = publishTrustedStdioCatalogV1({ outDir });
    expect(receipt.publication).toBe("committed");
    expect(receipt.catalog.nativeCodecs).toHaveLength(loadFirstPartyStdioNativeCodecReceipts().length);
    const written = readFileSync(join(outDir, "trusted-stdio-catalog.json"));
    expect(createHash("sha256").update(written).digest("hex")).toBe(receipt.catalogSha256);
    expect(written.toString("utf8")).toBe(encodeTrustedStdioCatalogV1(receipt.catalog));
    expect(JSON.parse(written.toString("utf8")).hubBundle).toBe("withheld");
  });

  it("fails closed on empty, digest-mismatched, and missing protocol receipts", () => {
    const receipts = loadFirstPartyStdioNativeCodecReceipts();
    const first = receipts[0]!;
    expect(() => verifyStdioNativeCodecReceipts([])).toThrow(new RegExp(fixture.hostile.emptyReceipts));
    expect(() => verifyStdioNativeCodecReceipts([{ ...first, protocolSourceSha256: "ab".repeat(32) }])).toThrow(new RegExp(fixture.hostile.digestMismatch));
    const outDir = testRoot();
    expect(() => verifyStdioNativeCodecReceipts([{ ...first, packSchemaHash: "ab".repeat(32) }])).toThrow(new RegExp(fixture.hostile.identityMismatch));
    expect(() => verifyStdioNativeCodecReceipts([{ ...first, artifactDefinitionPath: join(outDir, "absent.definition.json") }])).toThrow(new RegExp(fixture.hostile.missingDefinition));
    expect(() => verifyStdioNativeCodecReceipts([{ ...first, protocolPath: join(outDir, "absent.protocol.semio") }])).toThrow(new RegExp(fixture.hostile.missingProtocol));
    expect(() => publishTrustedStdioCatalogV1({ outDir, receipts: [] })).toThrow(new RegExp(fixture.hostile.emptyReceipts));
  });
});
