/** 🪪️ Neutral strict value admission checked independently by Ajv. */
import { test, expect } from "bun:test";
import Ajv from "ajv";
import schema from "../🔣️.json";
import fixture from "./🧫️fixtures/🪪️identity/🔣️.json";
import { parseArtifactRef } from "../🟦️.ts";
const oracle = new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/ArtifactRef"});
for (const [index,value] of fixture.valid.entries()) test(`owned artifact identity ${index}`,()=>{
  expect(oracle(value)).toBe(true);
  expect(parseArtifactRef(value)).toEqual(value);
});
for (const [index,value] of fixture.invalid.entries()) test(`refuse malformed artifact identity ${index}`,()=>{
  expect(oracle(value)).toBe(false);
  expect(()=>parseArtifactRef(value)).toThrow();
});
import { requireArtifactDialect } from "../🟦️.ts";
const dialectOracle = (expected: readonly {artifactKind:string;standard:string;subset:string}[]) => new Ajv({strict:false}).compile({
  anyOf: expected.map((row) => ({ type: "object", properties: { artifactKind: { const: row.artifactKind }, standard: { const: row.standard }, subset: { const: row.subset } }, required: ["artifactKind", "standard", "subset"] })),
});
for (const [index,row] of fixture.dialectExpectations.entries()) test(`declared dialect expectation ${index} agrees with the independent const oracle`,()=>{
  const independent = dialectOracle(row.expected)(row.actual);
  expect(independent).toBe(row.accepted);
  if (row.accepted) expect(requireArtifactDialect(row.actual, row.expected, "$.child")).toEqual(row.actual);
  else expect(()=>requireArtifactDialect(row.actual, row.expected, "$.child")).toThrow("$.child: artifact dialect mismatch");
});
