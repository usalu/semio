/** 🟠️ The actor return result framing phase of registerTests1. */
import type { registerActorReturnCodecsLoadTests } from "../🔶️actor-return-codecs-load/🟦️.ts";

/** 🟡 Registers the actor return result framing tests of registerTests1. */
export function registerActorReturnResultFramingTests({ it, resultOracle, hydrateResult, expect, vi, encodeActorReturnResult, decodeActorReturnResult }: Pick<ReturnType<typeof registerActorReturnCodecsLoadTests>, "it" | "resultOracle" | "hydrateResult" | "expect" | "vi" | "encodeActorReturnResult" | "decodeActorReturnResult">) {
  it("ActorReturnResultFraming retains failure across malformed, trailing and truncated input", async () => {
    const api = await import("../../../🟦️.ts");
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json"); const oracle = await resultOracle();
    const malformed = fixture.resultContradictions.map(row => oracle(hydrateResult(row)));
    malformed.push(Buffer.of(6), Buffer.of(1, 0, 1, 1, 0), Buffer.of(1, 0x81, 0, 1, 1, 0));
    for (const row of fixture.resultVectors) { const bytes = Buffer.from(row.hex, "hex"); malformed.push(bytes.subarray(0, bytes.length - 1), Buffer.concat([bytes, Buffer.of(0)])); }
    const emptyPage = fixture.pageResultVectors[0]!; const padding = Buffer.alloc(4096); padding[4095] = 1; malformed.push(Buffer.concat([Buffer.from(emptyPage.prefixHex, "hex"), padding]));
    for (const bytes of malformed) {
      const parser = new api.ActorReturnResultFraming();
      expect(() => { for (const byte of bytes) parser.push(byte); parser.finish(); }).toThrow();
      expect(() => parser.push(0)).toThrow(); expect(() => parser.finish()).toThrow(); expect(parser.value).toBeNull();
    }
    for (const byte of [-1, 256, 0.5, NaN]) { const parser = new api.ActorReturnResultFraming(); expect(() => parser.push(byte)).toThrow(); expect(() => parser.finish()).toThrow(); }
  });

  it("ActorReturnResult does not allocate a page for a fixed control or any whole semantic result", async () => {
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const input = hydrateResult(fixture.resultVectors[0]!.value);
    const allocations: number[] = []; const original = Uint8Array;
    vi.stubGlobal("Uint8Array", new Proxy(original, { construct(target, args, newTarget) {
      if (typeof args[0] !== "number") throw new Error("unexpected whole-buffer constructor");
      allocations.push(args[0]); return Reflect.construct(target, args, newTarget);
    } }));
    let bytes: Uint8Array;
    try { bytes = encodeActorReturnResult(input); expect(decodeActorReturnResult(bytes)).toEqual(input); }
    finally { vi.unstubAllGlobals(); }
    expect(allocations).toHaveLength(1); expect(allocations[0]).toBeLessThanOrEqual(fixture.maximumControlBytes + 3);
    expect(bytes!).toHaveLength(4);
  });
}
