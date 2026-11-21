import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { createHash, webcrypto } from "node:crypto";
import { actorPreparationReceiptV1, assertActorPreparationBindingV1 } from "../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

test("published preparation binds exact image and component bytes", async () => {
  const bytes = new TextEncoder().encode(fixture.source);
  const sha256 = createHash("sha256").update(bytes).digest("hex");
  const independent = Buffer.from(await webcrypto.subtle.digest("SHA-256", bytes)).toString("hex");
  const validate = new Ajv().compile(schema.definitions.ActorPreparationReceipt);
  const database = new Database(":memory:");
  try {
    const bind = database.query<{ admitted: number }, [string, string, string, number]>(
      "select json_extract(?1,'$.componentSha256') = ?2 and json_extract(?1,'$.imageSha256') = ?3 and json_extract(?1,'$.imageBytes') = ?4 as admitted",
    );
    for (const row of fixture.cases) {
      const value = { version: 1, componentSha256: fixture.componentSha256, imageSha256: sha256, imageBytes: bytes.length, ...row.changes };
      const actual = () => actorPreparationReceiptV1(value, fixture.componentSha256, sha256, bytes.length);
      const expected = validate(value) && bind.get(JSON.stringify(value), fixture.componentSha256, independent, bytes.length)?.admitted === 1;
      expect(expected).toBe(row.accepted);
      if (row.accepted) expect(actual()).toEqual(value);
      else expect(actual).toThrow();
    }
  } finally {
    database.close();
  }
});

test("the embedded full component identity must agree independently of the receipt", () => {
  const image = Buffer.concat([Buffer.from("SEMIOPA\0\x01"), Buffer.from(fixture.componentSha256, "hex"), Buffer.alloc(8)]);
  expect(() => assertActorPreparationBindingV1(image, fixture.componentSha256)).not.toThrow();
  expect(() => assertActorPreparationBindingV1(image.subarray(0, 40), fixture.componentSha256)).toThrow();
  expect(() => assertActorPreparationBindingV1(image, "b".repeat(64))).toThrow();
  image[8] = 2;
  expect(() => assertActorPreparationBindingV1(image, fixture.componentSha256)).toThrow();
});
