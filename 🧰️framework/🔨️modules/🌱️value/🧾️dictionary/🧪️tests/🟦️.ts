/** 🧾️ Question dictionaries agree with their independent JSON Schema oracle and round-trip through the native JSON limbs. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import { parseQuestionDictionary } from "../🟦️.ts";
import { questionDictionaryFromNativeJson, questionDictionaryNativeJson } from "../🪪️native-json/🟦️.ts";

const oracle = new Ajv({ strict: false }).compile(schema);

for (const [index, row] of fixture.valid.entries()) test(`valid question dictionary ${index} is admitted and reproduced exactly`, () => {
  expect(oracle(row)).toBe(true);
  const owned = questionDictionaryFromNativeJson(row);
  expect(parseQuestionDictionary(owned)).toEqual(owned);
  expect(questionDictionaryNativeJson(owned)).toEqual(row);
});

for (const [index, row] of fixture.structurallyInvalid.entries()) test(`structurally invalid question dictionary ${index} is refused by both`, () => {
  expect(oracle(row)).toBe(false);
  expect(() => questionDictionaryFromNativeJson(row)).toThrow();
});

for (const [index, row] of fixture.semanticallyInvalid.entries()) test(`duplicate question identity ${index} is a semantic refusal the schema cannot express`, () => {
  expect(oracle(row)).toBe(true);
  expect(() => questionDictionaryFromNativeJson(row)).toThrow("duplicate or invalid question identity");
});
