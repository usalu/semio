/**
 * 🔤️ Law of the pack key comparator (`packByteCompare`, `💻️os/🟦️.ts`): over the language-agnostic fixture
 * `../../🧫️fixtures/🔤️pack-key-order/🔣️.json` (edge pairs + a seeded fuzz over an alphabet of ASCII, 2/3-byte, BMP private-use,
 * astral and lone-surrogate units) the sign of every comparison equals the sign of Node's own UTF-8 byte comparison
 * (`Buffer.compare(Buffer.from(a), Buffer.from(b))`, the third-party oracle). Bound: a comparison whose strings hold no
 * surrogate at the first difference constructs no `TextEncoder` at all — the allocation that made manifest encoding cost
 * ~0.5 s per boot (ticket 26/09/23 F2).
 */
import { readFileSync } from "node:fs";
import { describe, expect, it, vi } from "vitest";
import { encodePackValue, packByteCompare } from "../../🟦️.ts";

type Fixture = {
  readonly schema: "semio.os.pack-key-order-fixture/v1";
  readonly pairs: readonly { readonly left: string; readonly right: string }[];
  readonly fuzz: { readonly seed: number; readonly pairs: number; readonly maxLength: number; readonly alphabet: readonly string[] };
};

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔤️pack-key-order/🔣️.json", import.meta.url), "utf8")) as Fixture;
const oracle = (left: string, right: string): number => Math.sign(Buffer.compare(Buffer.from(left, "utf8"), Buffer.from(right, "utf8")));

/** 🎲️ xorshift32 — the fuzz is reproducible from the fixture's seed alone. */
function random(seed: number): () => number {
  let state = seed >>> 0 || 1;
  return () => {
    state ^= state << 13;
    state >>>= 0;
    state ^= state >>> 17;
    state ^= state << 5;
    state >>>= 0;
    return state / 0x1_0000_0000;
  };
}

describe("pack key order (UTF-8 byte order)", () => {
  it("reproduces the byte order of every fixture pair", () => {
    expect(fixture.schema).toBe("semio.os.pack-key-order-fixture/v1");
    for (const { left, right } of fixture.pairs) {
      expect({ left, right, sign: Math.sign(packByteCompare(left, right)) }).toEqual({ left, right, sign: oracle(left, right) });
      expect(Math.sign(packByteCompare(right, left))).toBe(oracle(right, left));
    }
  });

  it("reproduces the byte order of the seeded fuzz", () => {
    const next = random(fixture.fuzz.seed);
    const draw = (): string => {
      let text = "";
      const length = Math.floor(next() * (fixture.fuzz.maxLength + 1));
      for (let index = 0; index < length; index++) text += fixture.fuzz.alphabet[Math.floor(next() * fixture.fuzz.alphabet.length)]!;
      return text;
    };
    let disagreements = 0;
    for (let index = 0; index < fixture.fuzz.pairs; index++) {
      const left = draw();
      const right = next() < 0.3 ? left + draw() : draw();
      if (Math.sign(packByteCompare(left, right)) !== oracle(left, right)) disagreements += 1;
    }
    expect(disagreements).toBe(0);
  });

  it("sorts map keys by bytes without constructing an encoder for surrogate-free differences", () => {
    const keys = ["pluginId", "apps", "actions", "appId", "windowKinds", "Ärger", "中文", "", "zeta", "alpha"];
    const Native = globalThis.TextEncoder;
    let constructed = 0;
    const spy = vi.spyOn(globalThis, "TextEncoder").mockImplementation(function (this: unknown) {
      constructed += 1;
      return new Native();
    } as unknown as () => TextEncoder);
    try {
      const sorted = [...keys].sort(packByteCompare);
      expect(constructed).toBe(0);
      expect(sorted).toEqual([...keys].sort((left, right) => oracle(left, right)));
    } finally {
      spy.mockRestore();
    }
    const shuffled = Object.fromEntries([...keys].reverse().map((key) => [key, key.length]));
    const ordered = Object.fromEntries([...keys].sort((left, right) => oracle(left, right)).map((key) => [key, key.length]));
    expect(encodePackValue(shuffled)).toEqual(encodePackValue(ordered));
  });
});
