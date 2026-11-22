import { describe, expect, it } from "bun:test";
import { Buffer } from "node:buffer";
import { decodeBackboneWorkerRequest, encodeBackboneWorkerResponse, type BackboneWorkerRequest, type BackboneWorkerResponse } from "@semio-tech/framework/backbone";
import { readSignedDocumentArchiveV1 } from "../../../🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/📦️signed-archive/🟦️.ts";
import { settleRoleDocumentV1 } from "../../../🧱️elements/🏛️ShellHost/🛫️spawned-session/👁️switch-spawned-role/🚦️settle-document/🟦️.ts";
import fixture from "../🧫️fixtures/📦️signed-archive/🔣️.json";

class TestWorker extends EventTarget {
  readonly requests: BackboneWorkerRequest[] = [];
  postMessage(message: { readonly wire: Uint8Array }) {
    this.requests.push(decodeBackboneWorkerRequest(message.wire));
  }
  reply(reply: BackboneWorkerResponse): void {
    this.dispatchEvent(new MessageEvent("message", { data: { wire: encodeBackboneWorkerResponse(reply) } }));
  }
}

describe("signed archive and role transport", () => {
  it("reads the exact actor's archive in order", async () => {
    const worker = new TestWorker();
    const progress: number[][] = [];
    const result = readSignedDocumentArchiveV1(worker as unknown as Worker, fixture.scope, fixture.client, () => true,
      new AbortController().signal, (done, total) => progress.push([done, total ?? 0]));
    const request = worker.requests[0];
    if (request?.kind !== "document-archive-read") throw new Error("archive request missing");
    const whole = Buffer.concat([Buffer.alloc(fixture.pageBytes, fixture.pageValue), Buffer.from(fixture.tail)]);
    const first = { ...request, kind: "document-archive-page" as const, offset: 0, total: whole.length, bytes: whole.subarray(0, fixture.pageBytes) };
    worker.reply({ ...first, clientInstanceId: fixture.foreignClient });
    worker.reply(first);
    worker.reply({ ...first, offset: fixture.pageBytes, bytes: Uint8Array.from(fixture.tail) });
    expect(Buffer.from(await result)).toEqual(whole);
    expect(progress).toEqual([[fixture.pageBytes, whole.length], [whole.length, whole.length]]);
  });

  it("rejects a validly encoded page delivered before its predecessor", async () => {
    const worker = new TestWorker();
    const result = readSignedDocumentArchiveV1(worker as unknown as Worker, fixture.scope, fixture.client, () => true,
      new AbortController().signal, () => {});
    const request = worker.requests[0];
    if (request?.kind !== "document-archive-read") throw new Error("archive request missing");
    worker.reply({ ...request, kind: "document-archive-page", offset: fixture.pageBytes, total: fixture.pageBytes + fixture.tail.length, bytes: Uint8Array.from(fixture.tail) });
    await expect(result).rejects.toThrow("document-archive.invalid-page");
    expect(worker.requests.at(-1)).toEqual({ ...request, kind: "document-archive-read-cancel" });
  });

  it("does not accept role settlement under another scope or client", async () => {
    const worker = new TestWorker();
    const result = settleRoleDocumentV1(worker as unknown as Worker, fixture.scope, fixture.client, () => true);
    const request = worker.requests[0];
    if (request?.kind !== "document-settle") throw new Error("settle request missing");
    worker.reply({ ...request, kind: "document-settled", scope: { ...request.scope, spaceId: fixture.foreignSpace }, settled: false });
    worker.reply({ ...request, kind: "document-settled", clientInstanceId: fixture.foreignClient, settled: false });
    worker.reply({ ...request, kind: "document-settled", settled: true });
    expect(await result).toBe(true);
  });

  it("cancels the exact in-flight archive transfer", async () => {
    const worker = new TestWorker();
    const abort = new AbortController();
    const result = readSignedDocumentArchiveV1(worker as unknown as Worker, fixture.scope, fixture.client, () => true, abort.signal, () => {});
    abort.abort(new Error("cancelled by user"));
    await expect(result).rejects.toThrow("cancelled by user");
    expect(worker.requests[1]).toEqual({ ...worker.requests[0], kind: "document-archive-read-cancel" });
  });
});
