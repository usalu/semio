/** 🔸️ The actor return result matches phase of registerTests1. */
import type { ActorReturnResult } from "../../../🟦️.ts";
import type { registerActorReturnCodecsLoadTests } from "../🔶️actor-return-codecs-load/🟦️.ts";

/** 🔹 Registers the actor return result matches tests of registerTests1. */
export function registerActorReturnResultMatchesTests({ it, resultOracle, hydrateResult, encodeActorReturnResult, expect, decodeActorReturnResult, ACTOR_RETURN_RESULT_MAXIMUM_BYTES, createActorBytePage, readActorBytePage, decodeActorReturnDrive, vi }: Pick<ReturnType<typeof registerActorReturnCodecsLoadTests>, "it" | "resultOracle" | "hydrateResult" | "encodeActorReturnResult" | "expect" | "decodeActorReturnResult" | "ACTOR_RETURN_RESULT_MAXIMUM_BYTES" | "createActorBytePage" | "readActorBytePage" | "decodeActorReturnDrive" | "vi">) {
  it("ActorReturnResult matches all shared fixed results and the independent LEB128 oracle", async () => {
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const oracle = await resultOracle();
    for (const row of fixture.resultVectors) {
      const value = hydrateResult(row.value);
      const encoded = encodeActorReturnResult(value);
      expect(Buffer.from(encoded).toString("hex")).toBe(row.hex);
      expect(Buffer.from(encoded)).toEqual(oracle(value));
      expect(decodeActorReturnResult(encoded)).toEqual(value);
      expect(Object.isFrozen(decodeActorReturnResult(encoded))).toBe(true);
      for (let length = 0; length < encoded.length; length++) expect(() => decodeActorReturnResult(encoded.subarray(0, length))).toThrow();
      expect(() => decodeActorReturnResult(Uint8Array.from([...encoded, 0]))).toThrow();
    }
  });

  it("ActorReturnResult preserves exact fixed page bytes, lengths and the 4138 byte maximum", async () => {
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const oracle = await resultOracle();
    expect(ACTOR_RETURN_RESULT_MAXIMUM_BYTES).toBe(fixture.maximumResultBytes);
    for (const row of fixture.pageResultVectors) {
      const payload = Buffer.alloc(fixture.maximumPageBytes);
      if (row.pattern === "mod37plus11") for (let index = 0; index < row.pageLength; index++) payload[index] = (index * 37 + 11) % 256;
      const source = Uint8Array.from(payload.subarray(0, row.pageLength));
      const receipt = hydrateResult({ kind: "control", control: { kind: "inputAck", receipt: row.receipt }, outcome: "accepted", fault: "none" });
      if (receipt.kind !== "control" || receipt.control.kind !== "inputAck") throw new Error("invalid fixture");
      const value: ActorReturnResult = { kind: "page", receipt: receipt.control.receipt, page: createActorBytePage(source) };
      const expected = Buffer.concat([Buffer.from(row.prefixHex, "hex"), payload]);
      const bytes = encodeActorReturnResult(value);
      expect(bytes.length).toBe(row.wireBytes); expect(Buffer.from(bytes)).toEqual(expected); expect(oracle(value)).toEqual(expected);
      const backing = new Uint8Array(bytes.length + 7); backing.set(bytes, 3);
      const decoded = decodeActorReturnResult(backing.subarray(3, bytes.length + 3));
      expect(decoded).toEqual(value);
      if (decoded.kind !== "page") throw new Error("missing page result");
      expect(Object.isFrozen(decoded.page)).toBe(true); expect(Object.isFrozen(decoded.receipt.identity.origin)).toBe(true);
      expect(readActorBytePage(decoded.page)).toEqual(source);
      for (let length = 0; length < bytes.length; length++) expect(() => decodeActorReturnResult(bytes.subarray(0, length))).toThrow();
      expect(() => decodeActorReturnResult(Uint8Array.from([...bytes, 0]))).toThrow();
      source.fill(0); expect(Buffer.from(encodeActorReturnResult(value))).toEqual(expected);
      if (row.pageLength === 0) {
        const dirty = bytes.slice(); dirty[dirty.length - 1] = 1;
        expect(() => decodeActorReturnResult(dirty)).toThrow();
        expect(() => encodeActorReturnResult({ ...value, page: createActorBytePage(new Uint8Array(1)) })).toThrow();
      }
    }
  });

  it("ActorReturnResult rejects shared contradictions and every enum boundary in both directions", async () => {
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const { default: schema } = await import("../../../🧬️schema/🔣️.json");
    const { default: lifetimeSchema } = await import("../../../../🚪️lifetime/🧬️schema/🔣️.json");
    const { valueSchema } = await import("@semio-tech/value-ts/schema");
    const { default: pageSchema } = await import("../../../../📃️page/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const validate = new Ajv({ strict: true }).addSchema(valueSchema).addSchema(lifetimeSchema).addSchema(pageSchema).addSchema(schema).getSchema("https://json.schemas.assets.semio-tech.com/framework/actor/return/schema.json#/$defs/Result")!;
    const oracle = await resultOracle();
    for (const row of fixture.resultContradictions) {
      expect(validate(row)).toBe(false);
      const value = hydrateResult(row);
      expect(() => encodeActorReturnResult(value)).toThrow();
      const bytes = Uint8Array.from(oracle(value)); const original = bytes.slice();
      expect(() => decodeActorReturnResult(bytes)).toThrow(); expect(bytes).toEqual(original);
    }
    for (const drive of fixture.wireVectors.slice(1, 5)) {
      for (const outcome of fixture.resultEnums.outcome) for (const fault of fixture.resultEnums.fault) {
        const row = { kind: "control", control: drive.value.control, outcome, fault };
        const value = hydrateResult(row); const bytes = oracle(value);
        if (validate(row)) { expect(Buffer.from(encodeActorReturnResult(value))).toEqual(bytes); expect(decodeActorReturnResult(bytes)).toEqual(value); }
        else { expect(() => encodeActorReturnResult(value)).toThrow(); expect(() => decodeActorReturnResult(bytes)).toThrow(); }
      }
    }
    for (const row of fixture.resultVectors) {
      const bytes = Uint8Array.from(Buffer.from(row.hex, "hex")); bytes[bytes.length - 1] = 255;
      expect(() => decodeActorReturnResult(bytes)).toThrow();
    }
    for (const value of [null, undefined, [], {}, "00070901"]) expect(() => decodeActorReturnResult(value as Uint8Array)).toThrow();
    expect(() => decodeActorReturnResult(new Uint8Array(fixture.maximumResultBytes + 1))).toThrow();
    expect(() => decodeActorReturnResult(Uint8Array.of(5))).toThrow();
    expect(() => decodeActorReturnResult(Uint8Array.of(fixture.resultTags.protocolFault + 1, 12))).toThrow();
  });

  it("ActorReturnResult encodes pre-admission protocol faults without inventing return authority", async () => {
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const oracle = await resultOracle();
    for (const row of fixture.preAdmissionFaults) {
      const bytes = Uint8Array.from(Buffer.from(row.invalidDriveHex, "hex")); const original = bytes.slice();
      expect(() => decodeActorReturnDrive(bytes)).toThrow(); expect(bytes).toEqual(original);
      const result = decodeActorReturnResult(Uint8Array.from(Buffer.from(row.resultHex, "hex")));
      expect(result).toEqual({ kind: "protocolFault", fault: "malformedControl" });
      expect(Object.keys(result)).toEqual(["kind", "fault"]);
      expect(Buffer.from(encodeActorReturnResult(result)).toString("hex")).toBe(row.resultHex);
      expect(oracle(result).toString("hex")).toBe(row.resultHex);
    }
    for (const fault of fixture.resultEnums.fault) {
      const value = hydrateResult({ kind: "protocolFault", fault });
      const bytes = oracle(value);
      if (fault === "malformedControl" || fault === "mixedControl") {
        expect(encodeActorReturnResult(value)).toHaveLength(2);
        expect(Buffer.from(encodeActorReturnResult(value))).toEqual(bytes);
        expect(decodeActorReturnResult(bytes)).toEqual(value);
      } else {
        expect(() => encodeActorReturnResult(value)).toThrow();
        expect(() => decodeActorReturnResult(bytes)).toThrow();
      }
    }
  });

  it("ActorReturnResultFraming validates shared vectors without allocating or exposing page storage", async () => {
    const api = await import("../../../🟦️.ts");
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const { default: law } = await import("../../../🌿️framing/🧪️tests/🧫️fixtures/🔣️.json");
    const { default: schema } = await import("../../../🌿️framing/🧬️schema/🔣️.json");
    const { default: returned } = await import("../../../🧬️schema/🔣️.json");
    const { default: lifetime } = await import("../../../../🚪️lifetime/🧬️schema/🔣️.json");
    const { valueSchema: value } = await import("@semio-tech/value-ts/schema");
    const { default: page } = await import("../../../../📃️page/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true }).addSchema(value).addSchema(lifetime).addSchema(page).addSchema(returned).addSchema(schema);
    expect(ajv.validate(schema, law)).toBe(true); const oracle = await resultOracle();
    for (const row of fixture.resultVectors) {
      const value = hydrateResult(row.value); const bytes = oracle(value);
      expect(bytes.toString("hex")).toBe(row.hex);
      const parser = new api.ActorReturnResultFraming();
      for (const byte of bytes) parser.push(byte);
      expect(parser.finish()).toEqual(value); expect(parser.finish()).toBe(parser.value);
    }
    for (const row of fixture.pageResultVectors) {
      const receiptResult = hydrateResult({ kind: "control", control: { kind: "inputAck", receipt: row.receipt }, outcome: "accepted", fault: "none" });
      if (receiptResult.kind !== "control" || receiptResult.control.kind !== "inputAck") throw new Error("fixture receipt");
      const payload = Uint8Array.from({ length: row.pageLength }, (_, index) => (index * 37 + 11) % 256);
      const bytes = oracle({ kind: "page", receipt: receiptResult.control.receipt, page: createActorBytePage(payload) });
      const allocations: number[] = [];
      for (const name of ["Uint8Array", "BigUint64Array", "ArrayBuffer"] as const) {
        const original = globalThis[name]; vi.stubGlobal(name, new Proxy(original, { construct(target, args, newTarget) { allocations.push(Number(args[0])); return Reflect.construct(target, args, newTarget); } }));
      }
      let projection;
      try {
        const parser = new api.ActorReturnResultFraming();
        for (let index = 0; index < bytes.length; index++) { expect(parser.value).toBeNull(); parser.push(bytes[index]!); }
        projection = parser.finish(); expect(parser.finish()).toBe(projection); expect(allocations).toHaveLength(law.maximumPayloadCopies);
      } finally { vi.unstubAllGlobals(); }
      expect(projection).toEqual({ kind: "page", receipt: receiptResult.control.receipt, payloadOffset: Buffer.from(row.prefixHex, "hex").length });
      expect(ajv.validate({ $ref: schema.$id + "#/definitions/pageProjection" }, JSON.parse(JSON.stringify(projection, (_, value) => typeof value === "bigint" ? value.toString() : value)))).toBe(true);
      if (projection!.kind !== "page") throw new Error("fixture page projection");
      expect(bytes.subarray(projection!.payloadOffset, projection!.payloadOffset + row.pageLength)).toEqual(Buffer.from(payload));
      expect(Object.keys(projection!).sort()).toEqual(["kind", "payloadOffset", "receipt"]);
    }
  });
}
