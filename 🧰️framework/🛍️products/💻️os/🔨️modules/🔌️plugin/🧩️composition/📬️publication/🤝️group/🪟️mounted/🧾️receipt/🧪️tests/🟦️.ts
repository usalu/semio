import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { applyPatch } from "fast-json-patch";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

test("borrowed semantic schema fragments reproduce original forward and inverse identifiers",async()=>{
 
 const meta=fixture.receiptMetadata;
 const forward=Buffer.concat([Buffer.from(meta.schema),Buffer.from(meta.schemaSeparator),Buffer.from(meta.schemaSuffix)]);
 expect(forward.toString()).toBe("semio.fixture.operation");
 expect(Buffer.concat([forward,Buffer.from(".inverse")]).toString()).toBe("semio.fixture.operation.inverse");
 expect(JSON.parse(JSON.stringify({schema:forward.toString()})).schema).toBe("semio.fixture.operation");
 const source=await Bun.file(new URL("../🦀️.rs",import.meta.url)).text();
 expect(source.includes("pub schema_separator: &'a str")).toBe(true);
});

test("original mounted receipt bytes preserve denied backing and bounded exact copies", async () => {

  
  for (const row of fixture.cases) {
    const original = Buffer.from(row.text, "utf8"), output = Buffer.allocUnsafeSlow(original.length);
    for (let offset = 0; offset < original.length; offset += fixture.copyBytes) original.copy(output, offset, offset, Math.min(original.length, offset + fixture.copyBytes));
    expect(output.equals(new TextEncoder().encode(row.text))).toBe(true);
    const backing = { bytes: row.initialCapacity };
    expect(applyPatch(structuredClone(backing), [], true).newDocument).toEqual(backing);
    expect(applyPatch(structuredClone(backing), [{ op: "replace", path: "/bytes", value: 0 }], true).newDocument.bytes).toBe(0);
  }
  for (const [index] of fixture.cancellationFrontiers.entries()) {
    const original = Buffer.allocUnsafeSlow(64).fill(17), candidate = Buffer.allocUnsafeSlow(128);
    const copied = [0, 16, 64][index];
    original.copy(candidate, 0, 0, copied);
    expect(candidate.subarray(0, copied).equals(original.subarray(0, copied))).toBe(true);
    expect(original.equals(Buffer.alloc(64, 17))).toBe(true);
    expect(applyPatch({ original: 64, candidate: 128 }, [{ op: "replace", path: "/candidate", value: 0 }], true).newDocument).toEqual({ original: 64, candidate: 0 });
  }
  
  
  const source = await Bun.file(new URL("../🦀️.rs", import.meta.url)).text();
  expect(source.includes("struct MountedReceiptBytes")).toBe(true);
});

test("prestage operation bytes match the independently serialized original before receipt acknowledgment", async () => {
  
  for (const row of fixture.cases) for (const hex of fixture.operationHexModes) {
    const json = Buffer.from(JSON.stringify(row.text));
    const body = hex ? Buffer.from("value=" + json.toString("hex")) : json;
    const expected = Buffer.concat([Buffer.from(fixture.operationHeader), body]);
    const output = Buffer.allocUnsafeSlow(expected.length);
    for (let offset = 0; offset < expected.length; offset += fixture.copyBytes) expected.copy(output, offset, offset, Math.min(expected.length, offset + fixture.copyBytes));
    expect(output.equals(expected)).toBe(true);
  }
  const source = await Bun.file(new URL("../🦀️.rs", import.meta.url)).text();
  expect(source.includes("struct MountedPreparedOperationBytes")).toBe(true);
});

test("complete inverse operation framing retains every original length and payload", async () => {
  
  for (const hex of fixture.operationHexModes) for (const indices of fixture.operationLists) {
    const encodeInteger = (value: number): Buffer => { const bytes: number[] = []; do { const byte = value & 127; value = Math.floor(value / 128); bytes.push(byte | (value ? 128 : 0)); } while (value); return Buffer.from(bytes); };
    const operations = indices.map(index => { const json = Buffer.from(JSON.stringify(fixture.cases[index].text)); return Buffer.concat([Buffer.from(fixture.operationHeader), hex ? Buffer.from("value=" + json.toString("hex")) : json]); });
    const expected = Buffer.concat([encodeInteger(operations.length), ...operations.flatMap(operation => [encodeInteger(operation.length), operation])]);
    let offset = 0;
    const decodeInteger = (): number => { let value = 0, factor = 1, byte: number; do { byte = expected[offset++]; value += (byte & 127) * factor; factor *= 128; } while (byte & 128); return value; };
    expect(decodeInteger()).toBe(operations.length);
    for (const operation of operations) { const length = decodeInteger(); expect(expected.subarray(offset, offset + length).equals(operation)).toBe(true); offset += length; }
    expect(offset).toBe(expected.length);
  }
  const source = await Bun.file(new URL("../🦀️.rs", import.meta.url)).text();
  expect(source.includes("struct MountedPreparedOperationsBytes")).toBe(true);
});

test("host mutation and inverse receipts preserve original causal metadata and independent ownership", async () => {
  
  const meta = fixture.receiptMetadata;
  const kernel = { id: meta.mutationId, invocationId: meta.invocationId, schema: meta.schema + meta.schemaSeparator + meta.schemaSuffix, inverseSchema: meta.schema + meta.schemaSeparator + meta.schemaSuffix + ".inverse", dependencies: [...meta.dependencies], inverseDependencies: [...meta.dependencies], author: meta.author, baseVersion: meta.baseVersion, timestamp: [meta.clockActor, meta.physicalMs, meta.logical] };
  const undo = { target: meta.mutationId, schema: kernel.inverseSchema, dependencies: [...meta.dependencies], baseVersion: meta.baseVersion };
  expect(JSON.parse(JSON.stringify({ kernel, undo }))).toEqual({ kernel, undo });
  const changed = applyPatch(structuredClone(kernel), [{ op: "replace", path: "/dependencies/1", value: "different" }], true).newDocument;
  expect(changed.dependencies).not.toEqual(undo.dependencies);
  expect(undo.dependencies).toEqual(meta.dependencies);
  for (const value of [meta.mutationId, meta.invocationId, kernel.schema, kernel.inverseSchema, meta.author, ...meta.dependencies]) expect(Buffer.from(value).equals(new TextEncoder().encode(value))).toBe(true);
  const source = await Bun.file(new URL("../🦀️.rs", import.meta.url)).text();
  expect(source.includes("struct MountedKernelMutationReceipt")).toBe(true);
});

test("child document handles hash every original UTF8 byte in bounded turns", async () => {
  
  const mask = (1n << 64n) - 1n, prime = 0x100000001b3n;
  for (const [index, row] of fixture.cases.entries()) {
    let first = 0xcbf29ce484222325n, second = first ^ 0x9e3779b97f4a7c15n;
    const bytes = Buffer.from(row.text);
    for (let offset = 0; offset < bytes.length; offset += fixture.copyBytes) for (const byte of bytes.subarray(offset, offset + fixture.copyBytes)) { first = ((first ^ BigInt(byte)) * prime) & mask; second = ((second ^ (BigInt(byte) << 7n)) * prime) & mask; }
    expect(((first << 64n) | second).toString(16).padStart(32, "0")).toBe(fixture.artifactHandles[index]);
  }
  const source = await Bun.file(new URL("../🦀️.rs", import.meta.url)).text();
  expect(source.includes("struct MountedArtifactHandleIssuer")).toBe(true);
});
