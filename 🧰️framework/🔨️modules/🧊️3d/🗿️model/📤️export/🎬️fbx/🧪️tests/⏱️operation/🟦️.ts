/** ⏱️ The cancellation law of the FBX side: `encodeFbx`, `deflateZlib`, `inflateZlib` and `readFbx` poll inside their hot loops, stop at the first positive answer with a `CancelledError` (the encoder's `FbxCancelledError` is one) and return nothing; polling does not change the bytes. fflate produces the stream `inflateZlib` is cancelled on. */
import { deflateSync as fflateDeflate } from "fflate";
import { describe, expect, it } from "vitest";
import { CancelledError } from "../../../../⏱️operation/🟦️.ts";
import { caseOf, bytesOf, denseResolved, expectLaw, expectCancelledAt } from "../../../../🧪️tests/🧰️support/⏱️operation/🟦️.ts";
import { FbxCancelledError, encodeFbx } from "../../🟦️.ts";
import { deflateZlib, inflateZlib } from "../../🗜️deflate/🟦️.ts";
import { readFbx } from "../../📖️reader/🟦️.ts";
import { NO_OPERATION } from "../🧰️support/🟦️.ts";

describe("fbx cancellation", () => {
  it("encodeFbx cancels inside the vertex loops and the array serialisation, with an FbxCancelledError", async () => {
    const resolved = await denseResolved();
    const reference = encodeFbx(resolved, { format: "binary" }, NO_OPERATION);
    const bytes = await expectLaw((operation) => encodeFbx(resolved, { format: "binary" }, operation), caseOf("encodeFbx"));
    expect(Buffer.from(bytes).equals(Buffer.from(reference))).toBe(true);
    expect(new FbxCancelledError()).toBeInstanceOf(CancelledError);
    await expectCancelledAt((operation) => {
      try {
        return encodeFbx(resolved, { format: "binary" }, operation);
      } catch (error) {
        expect(error).toBeInstanceOf(FbxCancelledError);
        throw error;
      }
    }, 3);
  });
  it("deflateZlib cancels inside tokenisation and the stream still inflates with fflate", async () => {
    const input = bytesOf();
    const packed = await expectLaw((operation) => deflateZlib(input, operation), caseOf("deflateZlib"));
    expect(Buffer.from(inflateZlib(packed)).equals(Buffer.from(input))).toBe(true);
  });
  it("inflateZlib cancels inside the symbol loop on a stream fflate wrote", async () => {
    const input = bytesOf();
    const stream = fflateDeflate(input);
    const zlib = new Uint8Array(stream.length + 6);
    zlib.set([0x78, 0x9c]);
    zlib.set(stream, 2);
    const view = new DataView(zlib.buffer);
    let a = 1, b = 0;
    for (const byte of input) { a = (a + byte) % 65521; b = (b + a) % 65521; }
    view.setUint32(zlib.length - 4, ((b << 16) | a) >>> 0, false);
    const out = await expectLaw((operation) => inflateZlib(zlib, operation), caseOf("inflateZlib"));
    expect(Buffer.from(out).equals(Buffer.from(input))).toBe(true);
  });
  it("readFbx cancels inside the array decoding", async () => {
    const bytes = encodeFbx(await denseResolved(), { format: "binary" }, NO_OPERATION);
    const file = await expectLaw((operation) => readFbx(bytes, operation), caseOf("readFbx"));
    expect(file.version).toBe(7400);
  });
});
