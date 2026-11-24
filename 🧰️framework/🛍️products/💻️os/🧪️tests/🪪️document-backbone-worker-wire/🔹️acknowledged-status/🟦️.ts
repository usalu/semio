/** 🔹️ The acknowledged status tests of the document backbone worker wire suite. */
import { expect, it } from "vitest";
import { decodeBackboneWorkerResponse, encodeBackboneWorkerResponse } from "@semio-tech/framework/backbone";
import { encodePackValue } from "@semio-tech/framework/packs";

/** 💾️ Registers the tests that keep an acknowledged folder write reading as saved across the worker wire. */
export function registerAcknowledgedStatusTests(): void {
  const status = { persisted: true, pendingMutations: 0, remote: { kind: "detached" } } as const;
  const response = (event: unknown) => ({ kind: "event", documentId: "doc", clientInstanceId: "6f1c2a84-3b7d-4e19-9a52-0c8d7e4b1f36", event }) as never;

  it("never turns an absent write failure into a failure across the wire", () => {
    const decoded = decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response({ kind: "status", ...status, writeFailure: undefined }))) as unknown as { event: Record<string, unknown> };
    expect(Object.hasOwn(decoded.event, "writeFailure")).toBe(false);
    expect(decoded.event).toEqual({ kind: "status", ...status });
  });

  it("reads a null write failure from a twin as absent and keeps a real one", () => {
    const wire = (event: unknown) => { const packed = encodePackValue(response(event)); const framed = new Uint8Array(packed.length + 1); framed[0] = 0x01; framed.set(packed, 1); return framed; };
    const nulled = decodeBackboneWorkerResponse(wire({ kind: "status", ...status, writeFailure: null })) as unknown as { event: Record<string, unknown> };
    expect(Object.hasOwn(nulled.event, "writeFailure")).toBe(false);
    const failed = decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response({ kind: "status", ...status, persisted: false, writeFailure: { code: "folder-write-failed" } }))) as unknown as { event: Record<string, unknown> };
    expect(failed.event.writeFailure).toEqual({ code: "folder-write-failed" });
  });
}
