/** 🟠️ The preserves complete browser ui tests of the document backbone worker wire suite. */
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { expect, it } from "vitest";
import { BACKBONE_WORKER_WIRE_MAGIC, decodeBackboneMessage, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, encodeBackboneMessage, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse, parseDocumentBackboneMessage, parseInboundDocumentBackboneMessage } from "@semio-tech/framework/backbone";
import { encodePackValue } from "@semio-tech/framework/packs";
import { source, type BackboneWorkerRequest } from "../../🧮️directory-client-helpers/🟦️.ts";

/** 🟡 Registers the preserves complete browser ui tests of the document backbone worker wire suite. */
export function registerPreservesCompleteBrowserUiTests(): void {
  it("preserves the complete browser UI intent and its unsigned sequence above JSON precision", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧬️schema/🔣️.json", source.url), "utf8"));
    const validRequest = semioSchemaAjvV1({ strict: true }).compile({ ...schema, $ref: "#/definitions/request" });
    const validIntent = semioSchemaAjvV1({ strict: true }).compile({ ...schema, $ref: "#/definitions/uiIntent" });

    const { createBrowserActorUiIntentRequestV1 } = await import("@semio-tech/framework/browser-bundle/action-handoff/intent");
    const { decodePackValue, packUInt } = await import("@semio-tech/framework/packs");
    const intent = { ...fixture.uiIntent, seq: BigInt(fixture.uiIntent.seq) };
    const request = createBrowserActorUiIntentRequestV1(fixture.request, "map", intent);
    expect(request.actionSequence).toBe(9);
    expect(equal(decodePackValue(new Uint8Array(request.payload.bytes)), { ...intent, surface: "0:map", revision: packUInt(3n), node: packUInt(42n), action: { ...intent.action, version: packUInt(1n) }, seq: packUInt(0xffffffffffffffffn) })).toBe(true);
    expect(intent.surface).toBe("map");
    expect(validIntent({ ...fixture.uiIntent, seq: "0" })).toBe(true);
    expect((decodePackValue(new Uint8Array(createBrowserActorUiIntentRequestV1(fixture.request, "map", { ...intent, seq: 0n }).payload.bytes)) as Readonly<Record<string, unknown>>).seq).toEqual(packUInt(0n));
    for (const extra of [{ surface: "other" }, { revision: 4 }, { node: -1 }, { seq: 0x10000000000000000n }]) expect(() => createBrowserActorUiIntentRequestV1(fixture.request, "map", { ...intent, ...extra })).toThrow();
    for (const extra of [{ activationGeneration: "18446744073709551616" }, { actionSequence: 9007199254740992 }, { surfaceRevision: 9007199254740992 }]) {
      expect(validRequest({ ...fixture.request, ...extra })).toBe(false);
      expect(() => createBrowserActorUiIntentRequestV1({ ...fixture.request, ...extra }, "map", intent)).toThrow();
    }
  });
  it("admits only exact direct browser intent owners on the private worker wire", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const request = { ...fixture.request, clientInstanceId: "12345678-1234-4123-8123-123456789abc" };
    const response = { ...fixture.acknowledged, clientInstanceId: "12345678-1234-4123-8123-123456789abc" };
    expect(equal(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request)), request)).toBe(true);
    expect(equal(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response)), response)).toBe(true);
    for (const extra of [{ clientInstanceId: "" }, { authority: "forged" }, { command: [1] }, { surfaceRevision: 0 }, { payload: { kind: "ui-intent", bytes: [] } }]) {
      expect(() => decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ ...request, ...extra }))).toThrow();
    }
    for (const extra of [{ clientInstanceId: "" }, { authority: "forged" }, { outcome: "rejected" }, { actionSequence: 0 }]) {
      expect(() => decodeBackboneWorkerResponse(encodeBackboneWorkerResponse({ ...response, ...extra }))).toThrow();
    }
  });
  it("admits only closed semantically valid directory administration requests", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/📇️directory/🏛️administration-worker-wire-v1.json", source.url), "utf8")) as {
      valid: readonly BackboneWorkerRequest[];
      invalid: readonly { name: string; request: unknown }[];
    };
    const commandFixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🏛️administration/🛂️command-admission/🔣️.json", source.url), "utf8")) as {
      allowed: readonly { command: unknown }[];
      unrelated: readonly unknown[];
      malformed: readonly unknown[];
    };
    const directorySchema = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧬️schema/🔣️.json", source.url), "utf8"));
    const commandOracle = semioSchemaAjvV1({ strict: true }).addSchema(directorySchema).getSchema(`${directorySchema.$id}#/$defs/DirectoryCommand`)!;
    const wire = (request: unknown): Uint8Array => new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...encodePackValue(request)]);

    for (const request of fixture.valid) expect(equal(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request)), request)).toBe(true);
    for (const command of [...commandFixture.allowed.map((row) => row.command), ...commandFixture.unrelated]) {
      expect(commandOracle(command), JSON.stringify(commandOracle.errors)).toBe(true);
      const request = { kind: "directory-administration-submit", operationEpoch: 7, requestId: "9f8e7d6c5b4a39281706f5e4d3c2b1a0", command };
      const decoded = decodeBackboneWorkerRequest(wire(request));
      expect(equal(decoded, request)).toBe(true);
      expect(JSON.stringify((decoded as { command: unknown }).command)).toBe(JSON.stringify(command));
    }
    for (const row of fixture.invalid) expect(() => decodeBackboneWorkerRequest(wire(row.request)), row.name).toThrow();
    for (const command of commandFixture.malformed) {
      if (commandOracle(command)) continue;
      expect(() => decodeBackboneWorkerRequest(wire({ kind: "directory-administration-submit", operationEpoch: 7, requestId: "9f8e7d6c5b4a39281706f5e4d3c2b1a0", command }))).toThrow();
    }
    for (const request of [
      { kind: "directory-administration-open", operationEpoch: 1, spaceId: "s".repeat(257) },
      { kind: "directory-administration-refresh", operationEpoch: 1, cursor: "a".repeat(1025) },
    ]) expect(() => decodeBackboneWorkerRequest(wire(request))).toThrow();
  });
  const fromHex = (hex: string): Uint8Array => new Uint8Array(Buffer.from(hex, "hex"));
  it("retains one exact causal OpBinary through actor send and receive frames", () => {
    const batch = fromHex("01016d01640161000000017301aa016902bbcc03ffffffffffffffffff010500");
    const message = encodeBackboneMessage({ kind: "mutations", envelopes: batch });
    const clientInstanceId = "12345678-1234-4123-8123-123456789abc";
    const request = { kind: "send", documentId: "d", clientInstanceId, message: { kind: "documentBackbone", message } } as const;
    const response = { kind: "event", documentId: "d", clientInstanceId, event: { kind: "documentBackbone", message } } as const;
    const requestRound = decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request));
    const responseRound = decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response));
    expect(requestRound).toEqual(request);
    expect(responseRound).toEqual(response);
    expect(parseDocumentBackboneMessage((requestRound as typeof request).message.message).envelopes[0]?.timestamp.physical_ms).toBe(0xffff_ffff_ffff_ffffn);
  });
  it("encodes a retraction as the Rust store's exact vector and admits it only toward the store", () => {
    const toHex = (bytes: Uint8Array) => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
    for (const [mutationIds, hex] of [[[], "01040001000c00"], [["a"], "010401016101000c010600"]] as const) {
      const encoded = encodeBackboneMessage({ kind: "retract", mutationIds });
      expect(toHex(encoded)).toBe(hex);
      expect(decodeBackboneMessage(encoded)).toEqual({ kind: "retract", mutationIds });
      expect(parseInboundDocumentBackboneMessage(encoded)).toEqual(encoded);
      expect(() => parseDocumentBackboneMessage(encoded)).toThrow("mutations required");
    }
    expect(() => decodeBackboneMessage(fromHex("01030001000c00"))).toThrow("member lanes are Rust-only");
  });
  it("rejects non-mutation, noncanonical and over-cap actor messages", () => {
    const genesis = encodeBackboneMessage({ kind: "genesis", pack: new Uint8Array() });
    const ack = encodeBackboneMessage({ kind: "ack", opIds: [] });
    const trailing = encodeBackboneMessage({ kind: "mutations", envelopes: fromHex("0000") });
    expect(() => parseDocumentBackboneMessage(genesis)).toThrow("mutations required");
    expect(() => parseDocumentBackboneMessage(ack)).toThrow("mutations required");
    expect(() => parseDocumentBackboneMessage(trailing)).toThrow("trailing-bytes");
    expect(() => parseDocumentBackboneMessage(new Uint8Array(262_145))).toThrow("hot byte limit");

    const malformedFields = encodePackValue({ kind: "send", documentId: "d", clientInstanceId: "12345678-1234-4123-8123-123456789abc", message: { kind: "documentBackbone", message: Array.from(encodeBackboneMessage({ kind: "mutations", envelopes: Uint8Array.of(0) })), legacy: [] } });
    expect(() => decodeBackboneWorkerRequest(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...malformedFields]))).toThrow("invalid document backbone message");
  });
}
