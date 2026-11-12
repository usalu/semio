/** ⏱️ The cancellation law of the GLB encoder: `encodeGlb` polls inside every accessor, stops at the first positive answer with a `CancelledError` and returns no bytes; polling does not change the bytes. */
import { describe, it } from "vitest";
import { expect } from "vitest";
import { encodeGlb } from "../../🟦️.ts";
import { caseOf, denseResolved, expectLaw } from "../../../../🧪️tests/🧰️support/⏱️operation/🟦️.ts";

describe("encodeGlb cancellation", () => {
  it("cancels inside a dense primitive and its bytes are unchanged by polling", async () => {
    const resolved = await denseResolved();
    const options = { animations: true, variants: true };
    const reference = encodeGlb(resolved, options, { report: () => undefined, cancelled: () => false });
    const bytes = await expectLaw((operation) => encodeGlb(resolved, options, operation), caseOf("encodeGlb"));
    expect(Buffer.from(bytes).equals(Buffer.from(reference))).toBe(true);
  });
});
