import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import Ajv from "ajv";
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import contract from "../../🛡️untrusted-content-schemas/🧬️schema/capability-output.json" with { type: "json" };
import mirror from "../../🔣️.json" with { type: "json" };
import cases from "../../../🔀️dispatch/🧪️tests/🧫️fixtures/command-output.json" with { type: "json" };

test("capability results have exact content hashes and independently validated untrusted provenance", () => {
  const oracle = new Ajv({ strict: false, allErrors: true }), schema = contract.$defs.Envelope;
  for (const row of cases) {
    const value = {
      schema: schema.properties.schema.const,
      notice: schema.properties.notice.const,
      provenance: { source: "capability-output", artifactId: "a", artifactKind: null, spaceId: null, authors: { kind: "unattributed" }, revision: { contentSha256: row.sha256, headEditId: "h", commitSeq: null } },
      content: row.expected,
    };
    expect(createHash("sha256").update(JSON.stringify(row.expected)).digest("hex")).toBe(row.sha256);
    expect(validateJsonSchemaSubset(schema, value)).toEqual([]);
    expect(oracle.validate(schema, value), JSON.stringify(oracle.errors)).toBe(true);
    const invalid = { ...value, provenance: { ...value.provenance, authors: { kind: "local-principal", principal: "guessed" } } };
    expect(validateJsonSchemaSubset(schema, invalid).length).toBeGreaterThan(0);
    expect(oracle.validate(schema, invalid)).toBe(false);
  }
});

test("public invocation and preparation mirrors declare the actual untrusted result lane", () => {
  const definitions = mirror.$defs as Record<string, any>;
  expect(definitions.InvocationReport.properties.untrusted).toBeDefined();
  expect(definitions.PreparedActionReport.properties.preview.properties.untrusted).toBeDefined();
  expect(definitions.PreparedActionReport.properties.preview.properties.opsCount).toBeDefined();
  expect(definitions.UntrustedProvenanceV1.properties.source.enum).toContain("capability-output");
});
