import Ajv from "ajv";
import { sha256 } from "@noble/hashes/sha2.js";
import { createHash } from "node:crypto";
import { describe, expect, it } from "vitest";
import { courseRollbackTagV1 } from "../../🚀️deploy/🏷️release/🟦️.ts";
import schema from "../../🚀️deploy/🏷️release/🧬️schema/🔣️.json" with { type: "json" };
import fixtures from "../../🚀️deploy/🏷️release/🧫️fixtures/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

describe("content-addressed course rollback tags", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  const validTag = new Ajv({ strict: true }).compile(schema.definitions.Tag);

  for (const row of fixtures.cases) it(row.id, () => {
    expect(validate(row.request)).toBe(true);
    expect(validateJsonSchemaSubset(schema, row.request)).toEqual([]);
    const digest = Buffer.from(sha256(new TextEncoder().encode(row.config))).toString("hex");
    expect(createHash("sha256").update(row.config).digest("hex")).toBe(digest);
    expect(row.request.imageId).toBe(`sha256:${digest}`);
    const actual = courseRollbackTagV1(row.request);
    expect(actual).toBe(`sha-${row.request.revision.slice(0, 12)}-${digest}`);
    expect(actual).toBe(row.tag);
    expect(validTag(actual)).toBe(true);
  });

  it("changes the rollback tag when a rebuild changes the image and preserves the same image's tag", () => {
    const tags = fixtures.cases.map(row => courseRollbackTagV1(row.request));
    expect(tags[0]).not.toBe(tags[1]);
    expect(tags[0]).toBe(tags[2]);
  });

  for (const [index, request] of fixtures.rejected.entries()) it(`refuses malformed identity ${index}`, () => {
    expect(validate(request)).toBe(false);
    expect(validateJsonSchemaSubset(schema, request).length).toBeGreaterThan(0);
    expect(() => courseRollbackTagV1(request)).toThrow();
  });
});
