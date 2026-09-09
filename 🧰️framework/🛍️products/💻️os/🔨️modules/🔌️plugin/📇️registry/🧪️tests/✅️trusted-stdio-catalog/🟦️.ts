import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import Ajv from "ajv";
import { afterEach, describe, expect, it } from "vitest";
import {
  buildTrustedStdioCatalogV1,
  encodeTrustedStdioCatalogV1,
  loadFirstPartyStdioNativeCodecReceipts,
  publishTrustedStdioCatalogV1,
  verifyStdioNativeCodecReceipts,
} from "../../📜️script.ts";

const fixtureRoot = join(import.meta.dirname, "../../🧫️fixtures/🧬️trusted-stdio-catalog");
const schemaRoot = join(import.meta.dirname, "../../🧬️schema/🧬️trusted-stdio-catalog");
const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as {
  readonly schemaVersion: 1;
  readonly expectedCodecCount: number;
  readonly expectedOpenTargetKind: string;
  readonly expectedPublication: string;
  readonly expectedHubBundle: string;
  readonly hostile: { readonly emptyReceipts: string; readonly digestMismatch: string; readonly missingProtocol: string };
};
const schema = JSON.parse(readFileSync(join(schemaRoot, "🔣️.json"), "utf8"));
const temporaryRoots: string[] = [];

afterEach(() => {
  while (temporaryRoots.length) rmSync(temporaryRoots.pop()!, { recursive: true, force: true });
});

describe("trusted stdio catalog", () => {
  it("matches the language-neutral fixture schema and publishes 26 first-party codecs", () => {
    const ajv = new Ajv({ strict: true, allErrors: true });
    ajv.addSchema(schema);
    const validateFixture = ajv.compile(schema);
    expect(validateFixture(fixture)).toBe(true);
    const catalog = buildTrustedStdioCatalogV1();
    const validateCatalog = ajv.getSchema(`${schema.$id}#/$defs/TrustedStdioCatalogV1`);
    expect(validateCatalog).toBeTruthy();
    expect(validateCatalog!(catalog), JSON.stringify(validateCatalog!.errors)).toBe(true);
    expect(catalog.nativeCodecs).toHaveLength(fixture.expectedCodecCount);
    expect(catalog.openTargets).toHaveLength(1);
    expect(catalog.openTargets[0]?.artifactKind).toBe(fixture.expectedOpenTargetKind);
    expect(catalog.publication).toBe(fixture.expectedPublication);
    expect(catalog.hubBundle).toBe(fixture.expectedHubBundle);
    expect(catalog.pluginId).toBe("stdio");
    expect(catalog.packageId).toBe("semio:stdio");
    const json = catalog.nativeCodecs.find((row) => row.artifactKind === "stdio.json");
    expect(json?.factoryId).toBe("stdio.native.json.v1");
    expect(catalog.openTargets[0]?.packSchemaHash).toBe(json?.packSchemaHash);
  });

  it("matches Node crypto SHA-256 of every first-party protocol file", () => {
    const receipts = loadFirstPartyStdioNativeCodecReceipts();
    const codecs = verifyStdioNativeCodecReceipts(receipts);
    expect(codecs).toHaveLength(fixture.expectedCodecCount);
    const kinds = codecs.map((row) => row.artifactKind);
    expect(new Set(kinds).size).toBe(fixture.expectedCodecCount);
    expect(kinds).toEqual([...kinds].sort((left, right) => (left < right ? -1 : left > right ? 1 : 0)));
    for (const row of codecs) {
      expect(row.packSchemaHash).toMatch(/^[0-9a-f]{64}$/);
      expect(row.packSchemaHash).not.toBe("0".repeat(64));
    }
  });

  it("writes a committed catalog whose bytes hash under the Node crypto oracle", () => {
    const outDir = mkdtempSync(join(tmpdir(), "semio-trusted-stdio-catalog-"));
    temporaryRoots.push(outDir);
    const receipt = publishTrustedStdioCatalogV1({ outDir });
    expect(receipt.publication).toBe("committed");
    expect(receipt.catalog.nativeCodecs).toHaveLength(fixture.expectedCodecCount);
    const written = readFileSync(join(outDir, "trusted-stdio-catalog.json"));
    expect(createHash("sha256").update(written).digest("hex")).toBe(receipt.catalogSha256);
    expect(written.toString("utf8")).toBe(encodeTrustedStdioCatalogV1(receipt.catalog));
    expect(JSON.parse(written.toString("utf8")).hubBundle).toBe("withheld");
  });

  it("fails closed on empty, digest-mismatched, and missing protocol receipts", () => {
    const receipts = loadFirstPartyStdioNativeCodecReceipts();
    const first = receipts[0]!;
    expect(() => verifyStdioNativeCodecReceipts([])).toThrow(new RegExp(fixture.hostile.emptyReceipts));
    expect(() => verifyStdioNativeCodecReceipts([{ ...first, packSchemaHash: "ab".repeat(32) }])).toThrow(new RegExp(fixture.hostile.digestMismatch));
    expect(() => verifyStdioNativeCodecReceipts([{ ...first, protocolPath: join(tmpdir(), "semio-absent-stdio.protocol.semio") }])).toThrow(new RegExp(fixture.hostile.missingProtocol));
    const outDir = mkdtempSync(join(tmpdir(), "semio-trusted-stdio-hostile-"));
    temporaryRoots.push(outDir);
    expect(() => publishTrustedStdioCatalogV1({ outDir, receipts: [] })).toThrow(new RegExp(fixture.hostile.emptyReceipts));
  });
});
