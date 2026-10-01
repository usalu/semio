import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import {semioSchemaAjvV1} from "../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import * as geometry from "../../🟦️.ts";
import {point3Fixture,point2Fixture,uvFixture,rgbaFixture,quaternionFixture,transformFixture} from "../🧫️fixtures/🟦️.ts";

/** 🧫️ Shared geometry admission agrees with an independent JSON Schema validator. */
export function testSemioGeometryContract(): void {
  const schema = JSON.parse(readFileSync(new URL("../../🔣️.json", import.meta.url), "utf8"));
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const ajv = semioSchemaAjvV1({ allErrors: true });
  ajv.addSchema(schema);
  for (const entry of fixture.cases as { type: string; input: unknown; valid: boolean }[]) {
    const validate = ajv.compile({ $ref: schema.$id + "#/$defs/" + entry.type });
    assert.equal(validate(entry.input), entry.valid, entry.type + ": schema oracle");
    let parser:(value:unknown)=>unknown,owned:unknown;
    switch(entry.type){case "SemioPoint3":parser=geometry.parseSemioPoint3;owned=point3Fixture(entry.input);break;case "SemioPoint2":parser=geometry.parseSemioPoint2;owned=point2Fixture(entry.input);break;case "SemioUv":parser=geometry.parseSemioUv;owned=uvFixture(entry.input);break;case "SemioRgba":parser=geometry.parseSemioRgba;owned=rgbaFixture(entry.input);break;case "SemioQuaternion":parser=geometry.parseSemioQuaternion;owned=quaternionFixture(entry.input);break;case "SemioTransform":parser=geometry.parseSemioTransform;owned=transformFixture(entry.input);break;default:throw Error("unknown neutral geometry fixture");}
    if (entry.valid) assert.deepEqual(parser(owned), owned, entry.type);
    else assert.throws(() => parser(owned), entry.type);
  }
}
