/** 🧷️ Embedded child slot: unbound refusal, idempotent rebinding and refused replacement, agreeing with a hand-written truth table. */
import { expect, test } from "bun:test";
import { embeddedChildSlotV1, type EmbeddedChildCodecV1 } from "../🟦️.ts";

const codec = (tag: string): EmbeddedChildCodecV1<string> => ({
  parse: (value, at) => { if (typeof value !== "string") throw new Error(`${at}: ${tag} text required`); return value; },
  nativeJson: { from: (value) => String(value), to: (content) => content },
  sqlite: { schema: `-- ${tag}`, project: async () => {}, reconstruct: async () => tag },
});

test("an unbound slot refuses to hand out a codec", () => {
  expect(() => embeddedChildSlotV1<string>("test.unbound").codec()).toThrow("test.unbound has no bound codec");
});

test("a slot returns exactly the codec bound to it and tolerates identical rebinding", () => {
  const slot = embeddedChildSlotV1<string>("test.bound"), first = codec("first");
  slot.bind(first);
  slot.bind(first);
  expect(slot.codec()).toBe(first);
  expect(slot.codec().parse("x", "$")).toBe("x");
  expect(() => slot.codec().parse(1, "$.row")).toThrow("$.row: first text required");
});

test("a bound slot refuses a different codec and slots stay independent", () => {
  const left = embeddedChildSlotV1<string>("test.left"), right = embeddedChildSlotV1<string>("test.right");
  left.bind(codec("left"));
  expect(() => left.bind(codec("other"))).toThrow("test.left is already bound to a different codec");
  expect(() => right.codec()).toThrow("test.right has no bound codec");
  right.bind(codec("right"));
  expect(left.codec().sqlite.schema).toBe("-- left");
  expect(right.codec().sqlite.schema).toBe("-- right");
});
