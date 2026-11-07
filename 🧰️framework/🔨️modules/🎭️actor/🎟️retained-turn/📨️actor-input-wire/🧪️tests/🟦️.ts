import { expect, test } from "bun:test";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { decodePackValue, encodePackValue, packUInt, type PackValue } from "../../../../🎁️packs/🟦️.ts";
import { createActorInputWireV1 } from "../🟦️.ts";

const fixtures = resolve(import.meta.dirname, "../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🎟️actor-input/🧫️fixtures/🔣️.json");
const corpus = JSON.parse(readFileSync(fixtures, "utf8")) as { valid: { operation: string; wire: unknown; packHex?: string }[] };
const wire = createActorInputWireV1();
const hex = (bytes: Uint8Array) => [...bytes].map((byte) => byte.toString(16).padStart(2, "0")).join("");
const carriers = (value: unknown): PackValue => {
  if (typeof value === "number") return packUInt(BigInt(value));
  if (Array.isArray(value)) return value.map(carriers);
  if (value !== null && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, carriers(item)]));
  return value as PackValue;
};

test("the self-contained encoder writes the same bytes as the repo pack library for every authored wire", () => {
  for (const { wire: value } of corpus.valid) expect(hex(wire.encode(value))).toBe(hex(encodePackValue(carriers(value))));
});

test("every pinned packHex is the encoder output and decodes back to the authored wire", () => {
  if (process.env.SEMIO_PIN_ACTOR_INPUT_HEX === "1") {
    for (const entry of corpus.valid) entry.packHex = hex(wire.encode(entry.wire));
    writeFileSync(fixtures, JSON.stringify(corpus, null, 2) + "\n");
  }
  for (const { wire: value, packHex } of corpus.valid) {
    expect(packHex).toBe(hex(wire.encode(value)));
    expect(decodePackValue(wire.encode(value))).toEqual(carriers(value));
  }
});

test("a jco-lowered event crosses as its own lowered form with absent optionals as null", () => {
  expect(wire.event({ tag: "wake" })).toEqual({ tag: "wake" });
  expect(wire.event({ tag: "patch-ack", val: { surface: { instance: 1, surface: "main" }, revision: 2n, receipt: undefined } })).toEqual({ tag: "patch-ack", val: { surface: { instance: 1, surface: "main" }, revision: 2n, receipt: null } });
  expect(wire.event({ tag: "completed", val: { req: 1n, outcome: { tag: "ok", val: new Uint8Array([1]) } } })).toEqual({ tag: "completed", val: { req: 1n, outcome: { tag: "ok", val: new Uint8Array([1]) } } });
});

test("numbers that are not exact unsigned integers are refused", () => {
  for (const value of [-1, 0.5, 2 ** 53, Number.NaN]) expect(() => wire.encode({ value })).toThrow();
});
