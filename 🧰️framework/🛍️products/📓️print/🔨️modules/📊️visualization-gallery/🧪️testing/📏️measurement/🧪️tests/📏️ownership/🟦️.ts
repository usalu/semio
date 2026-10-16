import { test, expect } from "bun:test";
import { deflateSync, deflateRawSync } from "node:zlib";
import { sha256 } from "@noble/hashes/sha2.js";
import Ajv from "ajv";
import { printPdfMeasurementHashV1 } from "../../📥️pdf/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import cases from "../🧫️fixtures/📏️measurement/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

test("PDF measurement uses the same canonical bytes across metadata and compression", () => {
    const expected = Buffer.from(sha256(new TextEncoder().encode(cases.canonicalPdf))).toString("hex");
    for (const metadata of cases.metadata) for (const encode of [(bytes: Buffer) => bytes, deflateSync, deflateRawSync]) {
      const bytes = Buffer.concat([Buffer.from(`%PDF-1.7\n/CreationDate (${metadata})\nstream\n`), encode(Buffer.from(cases.body)), Buffer.from("\nendstream\n")]);
      expect(printPdfMeasurementHashV1(bytes)).toBe(expected);
    }
    console.log("[DEBUG] PDF measurement canonical-hash comparisons=6 noble-sha256=agreed");
});

test("actual gallery measurement reports agree with an independent validator", () => {
  const contract = { $defs: schema.$defs, $ref: "#/$defs/GalleryRenderEvidence" };
  const independent = new Ajv({ strict: true }).compile(contract);
  const firstParty = (value: unknown) => validateJsonSchemaSubset(contract, value).length === 0;
  expect(firstParty(cases.report)).toBe(true);
  expect(independent(cases.report)).toBe(true);
  const hostile = structuredClone(cases.report);
  hostile.variants["bars/light/en"].kinds.bars.page = 0;
  expect(firstParty(hostile)).toBe(false);
  expect(independent(hostile)).toBe(false);
});
