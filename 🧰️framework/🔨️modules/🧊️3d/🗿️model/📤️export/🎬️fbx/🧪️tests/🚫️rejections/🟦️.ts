import { describe, expect, it } from "vitest";
import type { ModelDocument } from "../../../../🧬️schema/🟦️.ts";
import { FbxCancelledError, FbxEncodeError, encodeFbx, encodeFbxDetailed } from "../../🟦️.ts";
import { NO_OPERATION, fixture } from "../🧰️support/🟦️.ts";
import type { FbxFixture } from "../🧰️support/🟦️.ts";

/** 🚫️ Procedural documents, unknown references, cancellation and progress. */
describe("FBX encoder contract", () => {
  const procedural = fixture<{ document: ModelDocument; expectedError: string }>("procedural");
  const { document, cases } = fixture<FbxFixture>("rigged-arm");

  it("rejects a procedural document with a clear bilingual error", () => {
    try {
      encodeFbx(procedural.document, { format: "binary" }, NO_OPERATION);
      expect.unreachable();
    } catch (error) {
      expect(error).toBeInstanceOf(FbxEncodeError);
      const failure = error as FbxEncodeError;
      expect(failure.code).toBe(procedural.expectedError);
      expect(failure.message).toContain("procedural");
      expect(failure.german).toContain("prozedural");
    }
  });

  it("rejects unknown animations, variants and formats", () => {
    const code = (run: () => unknown): string => {
      try {
        run();
      } catch (error) {
        return (error as FbxEncodeError).code;
      }
      return "none";
    };
    expect(code(() => encodeFbx(document, { animation: "nope", format: "binary" }, NO_OPERATION))).toBe("unknown-animation");
    expect(code(() => encodeFbx(document, { variant: "nope", format: "binary" }, NO_OPERATION))).toBe("unknown-variant");
    expect(code(() => encodeFbx(document, { format: "ascii" as "binary" }, NO_OPERATION))).toBe("unsupported-format");
  });

  it("reports monotonic progress from 0 to 1", () => {
    const fractions: number[] = [];
    encodeFbx(document, cases[1].options, { report: (fraction) => fractions.push(fraction), cancelled: () => false });
    expect(fractions[0]).toBe(0);
    expect(fractions[fractions.length - 1]).toBe(1);
    for (let index = 1; index < fractions.length; index++) expect(fractions[index]).toBeGreaterThanOrEqual(fractions[index - 1]);
  });

  it("cancels before, during and produces no bytes", () => {
    expect(() => encodeFbx(document, cases[1].options, { report() {}, cancelled: () => true })).toThrow(FbxCancelledError);
    for (const after of [1, 5, 20, 60]) {
      let polls = 0;
      expect(() => encodeFbx(document, cases[1].options, { report() {}, cancelled: () => ++polls > after })).toThrow(FbxCancelledError);
    }
  });

  it("accepts a custom compressor behind the owned interface", () => {
    let calls = 0;
    const { bytes } = encodeFbxDetailed(document, cases[0].options, NO_OPERATION, {
      zlib: (data) => {
        calls++;
        return data.slice();
      },
    });
    expect(calls).toBeGreaterThan(0);
    expect(bytes.length).toBeGreaterThan(0);
  });
});
