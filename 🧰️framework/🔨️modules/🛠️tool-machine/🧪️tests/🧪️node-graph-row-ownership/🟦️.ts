import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";

type RowCase = Readonly<{ id: string; row: Readonly<Record<string, unknown>> }>;
type Cases = Readonly<{ accepted: readonly RowCase[]; refused: readonly RowCase[] }>;
const owner = resolve(import.meta.dir, "../.."), read = (path: string): unknown => JSON.parse(readFileSync(resolve(owner, path), "utf8"));
const cases = (): Cases => read("🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json") as Cases;
const validator = () => new Ajv({ strict: true, allErrors: true }).compile(read("🧬️schema/🔣️node-graph-edit-rows/🔣️.json") as object);

test("the neutral corpus retains every original accepted and refused row", () => {
  const fixture = cases();
  expect(Object.keys(fixture)).toEqual(["accepted", "refused"]);
  expect(fixture.accepted.length).toBe(8);
  expect(fixture.refused.length).toBe(14);
  expect(new Set([...fixture.accepted, ...fixture.refused].map(row => row.id)).size).toBe(22);
  expect(createHash("sha256").update(JSON.stringify(fixture)).digest("hex")).toBe("01322c8975adc09ced12d9c9eef1cc2d45d66f8b607c2c925d04507087217a31");
});

test("the closed schema agrees with the original decoder row and batch laws", () => {
  const fixture = cases(), validate = validator(), accepted = fixture.accepted.map(({ row }) => row);
  for (const { id, row } of fixture.accepted) expect(validate({ operations: [row] }), `${id}: ${JSON.stringify(validate.errors)}`).toBe(true);
  for (const { id, row } of fixture.refused) {
    expect(validate({ operations: [row] }), id).toBe(false);
    expect(validate({ operations: [...accepted, row] }), id).toBe(false);
  }
  expect(validate({ operations: accepted })).toBe(true);
  expect(validate({ operations: [], gesture: "slider:7", abort: "blur" })).toBe(true);
  for (const field of ["gesture", "commit", "abort"]) expect(validate({ operations: [], [field]: { opaque: true } }), field).toBe(true);
  expect(validate({ operations: accepted, hostSnapshotChanged: true })).toBe(false);
  expect(validate({ gesture: "slider:7" })).toBe(false);
  expect(validate({ operations: Array.from({ length: 256 }, () => accepted[1]) })).toBe(true);
  expect(validate({ operations: Array.from({ length: 257 }, () => accepted[1]) })).toBe(false);
});
