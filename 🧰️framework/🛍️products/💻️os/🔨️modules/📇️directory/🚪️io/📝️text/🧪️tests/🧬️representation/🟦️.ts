/** 🧪️ Neutral Directory IO vectors with independent JSON Schema and SHA witnesses. */
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import { semioSchemaAjvV1 } from "../../../../../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import * as wire from "../../🟦️.ts";
import { parseDirectorySessionAuthorityJsonV1 } from "../../🪪️session-authority-v1/🟦️.ts";
import * as creation from "../../🌱️space-artifact-creation-v1/🟦️.ts";
import * as checkin from "../../📌️document-check-in-v1/🟦️.ts";
import { documentCheckInStatusFromValueV1 } from "../../../../🧬️schema/📌️document-check-in-v1/🟦️.ts";

const directory = resolve(import.meta.dir, "../../../..");
const os = resolve(directory, "../..");
const read = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
const schema = read(resolve(directory, "🧬️schema/🔣️.json"));
const validator = (name: string) => semioSchemaAjvV1({ strict: false, allErrors: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/" + name });
const accepts = (run: () => unknown) => { try { return run() !== null; } catch { return false; } };
const sha = (source: string) => createHash("sha256").update(source).digest("hex");

test("event page keeps the neutral canonical receipt and refuses padding and duplicates", async () => {
  const fixture = read(resolve(os, "🧫️fixtures/📇️directory/📃️event-page-v1.json"));
  expect(validator("DirectoryEventPageV1")(fixture.valid)).toBe(true);
  expect(sha(fixture.canonicalUnsigned)).toBe(fixture.expectedReceiptSha256);
  const source = JSON.stringify(fixture.valid);
  expect(await wire.parseDirectoryEventPageV1(source)).toEqual(fixture.valid);
  await expect(wire.parseDirectoryEventPageV1(source + " ")).rejects.toThrow();
  await expect(wire.parseDirectoryEventPageV1(source.replace('{"schema":', '{"schema":"duplicate","schema":'))).rejects.toThrow();
  console.log("[DEBUG] Directory IO page: independentAjv=true independentNodeCrypto=true");
});

test("command request and receipt retain all neutral digests and refusal cases", async () => {
  const fixture = read(resolve(os, "🧫️fixtures/📇️directory/🧾️command-receipt-v1.json"));
  const requests = new Map<string, ReturnType<typeof wire.parseDirectoryCommandRequestV1>>();
  const requestCheck = validator("DirectoryCommandRequestV1"), receiptCheck = validator("DirectoryCommandReceiptV1");
  for (const row of fixture.requests) {
    const request = wire.parseDirectoryCommandRequestV1(row.canonical);
    expect(requestCheck(request)).toBe(true);
    expect(wire.directoryCommandRequestJson(request)).toBe(row.canonical);
    expect(sha(JSON.stringify(request.command))).toBe(row.commandSha256);
    expect(await wire.directoryCommandSha256(request.command)).toBe(row.commandSha256);
    requests.set(row.name, request);
  }
  for (const row of fixture.receipts) {
    const receipt = await wire.parseDirectoryCommandReceiptV1(row.canonical, requests.get(row.requestName)!);
    expect(receiptCheck(receipt)).toBe(true);
    const { receiptSha256, ...unsigned } = receipt;
    expect(sha(JSON.stringify(unsigned))).toBe(receiptSha256);
  }
  for (const row of fixture.rejectedRequests) expect(accepts(() => wire.parseDirectoryCommandRequestV1(row.source))).toBe(false);
  for (const row of fixture.rejectedReceipts) await expect(wire.parseDirectoryCommandReceiptV1(row.source, requests.get(row.requestName)!)).rejects.toThrow();
  console.log(`[DEBUG] Directory IO command: requests=${fixture.requests.length} receipts=${fixture.receipts.length} independentAjv=true independentNodeCrypto=true`);
});

test("session authority retains the independent schema and exact wire grammar", () => {
  const fixture = read(resolve(directory, "🧫️fixtures/🪪️session-authority-v1/🔣️.json"));
  const schema = read(resolve(directory, "🧬️schema/🪪️session-authority-v1/🧬️.schema.json"));
  const check = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  for (const row of fixture.rows) {
    expect(check(row.value)).toBe(row.accepted);
    expect(accepts(() => parseDirectorySessionAuthorityJsonV1(JSON.stringify(row.value)))).toBe(row.accepted);
  }
  for (const row of fixture.raw) expect(accepts(() => parseDirectorySessionAuthorityJsonV1(row.source))).toBe(row.accepted);
  console.log(`[DEBUG] Directory IO session: rows=${fixture.rows.length} raw=${fixture.raw.length} independentAjv=true`);
});

test("artifact creation uses the existing neutral request status and catalogue vectors", () => {
  const fixture = read(resolve(directory, "🧫️fixtures/🌱️space-artifact-creation-v1/🔣️.json"));
  const lanes = [["requests", creation.parseSpaceArtifactCreateJsonV1], ["statuses", creation.parseSpaceArtifactCreationStatusJsonV1], ["catalogs", creation.parseSpaceArtifactCreationCatalogJsonV1]] as const;
  const check = validator("SpaceArtifactCreationV1");
  let cases = 0;
  for (const [name, parse] of lanes) for (const row of fixture[name]) {
    const value = row.value;
    const independent = Boolean(check(value))
      && (value.progress === undefined || value.progress.completedUnits <= value.progress.totalUnits)
      && (value.kinds === undefined || value.kinds.every((kind: {kindId: string}, index: number) => index === 0 || value.kinds[index - 1].kindId < kind.kindId));
    expect(independent).toBe(row.accepted);
    expect(accepts(() => parse(JSON.stringify(value)))).toBe(row.accepted);
    cases++;
  }
  for (const row of fixture.rawJson) {
    const parse = row.type === "request" ? creation.parseSpaceArtifactCreateJsonV1 : row.type === "status" ? creation.parseSpaceArtifactCreationStatusJsonV1 : creation.parseSpaceArtifactCreationCatalogJsonV1;
    expect(accepts(() => parse(row.source))).toBe(row.accepted);
    cases++;
  }
  console.log(`[DEBUG] Directory IO creation: neutralCases=${cases} independentAjv=true`);
});

test("check-in wire and direct typed projection preserve the neutral corpus against Ajv", () => {
  const fixture = read(resolve(directory, "🧬️schema/📌️document-check-in-v1/🧫️fixtures/🔣️.json"));
  const requestCheck = validator("DocumentCheckInV1"), statusCheck = validator("DocumentCheckInStatusV1");
  for (const row of fixture.valid.requests) {
    const value = checkin.parseDocumentCheckInV1(row.source);
    expect(value).not.toBeNull();
    expect(requestCheck(value)).toBe(true);
    expect(checkin.documentCheckInCanonicalJson(value!)).toBe(row.source);
  }
  for (const row of fixture.valid.statuses) {
    const value = checkin.parseDocumentCheckInStatusV1(row.source);
    expect(value).not.toBeNull();
    expect(statusCheck(value)).toBe(true);
    expect(documentCheckInStatusFromValueV1(JSON.parse(row.source))).toEqual(value);
  }
  for (const row of fixture.invalid.requests) expect(checkin.parseDocumentCheckInV1(row.source)).toBeNull();
  for (const row of fixture.invalid.statuses) expect(checkin.parseDocumentCheckInStatusV1(row.source)).toBeNull();
  console.log("[DEBUG] Directory IO check-in: directTypedProjection=true independentAjv=true");
});
