/** 🌱️ Neutral stable-child coordinates; identity derivation grants no creation authority. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";
import { blake3Hex } from "../../../../🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts";

type Coordinate = { values: string[]; ordinal: number };

/** 🪪️ Validate canonical coordinate frames with independent Buffer/DataView and AJV encoders. */
export function testInitialChildIdentityFixture(): void {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const fixture = read("./🧪️fixtures/🔣️.json");
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(read("./🧬️schema/🔣️.json"));
  assert.equal(fixture.domain, "semio.initial-child.v1");
  assert.equal(fixture.prefix, "initial-child-");
  assert.equal(fixture.hash, "blake3-256");
  assert.equal(fixture.authority, "none");
  assert.deepEqual(fixture.fields, ["scope.spaceId", "parent.artifactId", "parent.artifactKind", "parent.standard", "parent.subset", "slot", "child.artifactKind", "child.standard", "child.subset"]);
  const firstSpace = fixture.cases.find((row: any) => row.id === "document-one");
  const secondSpace = fixture.cases.find((row: any) => row.id === "different-space-same-document");
  assert(secondSpace, "equal document IDs in different authenticated spaces require a separate identity vector");
  assert.deepEqual(firstSpace.values.slice(1), secondSpace.values.slice(1));
  assert.notEqual(firstSpace.values[0], secondSpace.values[0]);
  assert.notEqual(firstSpace.expectedId, secondSpace.expectedId);
  assert.equal(blake3Hex(new TextEncoder().encode("abc")), "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85");
  const admitted = (value: Coordinate): boolean => validate(value) && value.values.every(field => field.isWellFormed() && Buffer.byteLength(field) <= fixture.maximumFieldBytes);
  const encode = (value: Coordinate): Uint8Array => {
    assert(admitted(value));
    const fields = value.values.map(field => new TextEncoder().encode(field));
    const domain = new TextEncoder().encode(fixture.domain);
    const bytes = new Uint8Array(domain.length + 1 + fields.reduce((sum, field) => sum + 4 + field.length, 0) + 4);
    bytes.set(domain);
    const view = new DataView(bytes.buffer);
    let offset = domain.length + 1;
    for (const field of fields) {
      view.setUint32(offset, field.length, true); offset += 4;
      bytes.set(field, offset); offset += field.length;
    }
    view.setUint32(offset, value.ordinal, true);
    return bytes;
  };
  const oracle = (value: Coordinate): Buffer => {
    const chunks = [Buffer.from(fixture.domain), Buffer.from([0])];
    for (const field of value.values) {
      const body = Buffer.from(field);
      const size = Buffer.alloc(4); size.writeUInt32LE(body.length);
      chunks.push(size, body);
    }
    const ordinal = Buffer.alloc(4); ordinal.writeUInt32LE(value.ordinal);
    return Buffer.concat([...chunks, ordinal]);
  };
  const targets = new Set<string>();
  for (const row of fixture.cases) {
    const coordinate = { values: row.values, ordinal: row.ordinal };
    const bytes = encode(coordinate);
    assert.deepEqual(Buffer.from(bytes), oracle(coordinate), row.id);
    assert.equal(bytes.length, row.wireBytes, row.id);
    assert.equal(fixture.prefix + blake3Hex(bytes), row.expectedId, row.id);
    assert(!targets.has(row.expectedId), row.id); targets.add(row.expectedId);
    assert.deepEqual(encode(coordinate), encode(structuredClone(coordinate)));
  }
  const base = fixture.cases[0];
  const sameDocument = new Ajv2020({ strict: true }).compile({ const: base.values[1] });
  for (const row of fixture.scopeAgreementCases) {
    assert.equal(row.scopeDocumentId === base.values[1], row.accepted);
    assert.equal(sameDocument(row.scopeDocumentId), row.accepted);
  }
  let denied = 0;
  for (let index = 0; index < fixture.fields.length; index++) {
    for (const field of fixture.rejectedFields) {
      const values = [...base.values]; values[index] = field.unit.repeat(field.repeat);
      assert(!admitted({ values, ordinal: base.ordinal })); denied++;
    }
    const values = [...base.values]; values[index] = "🌊".repeat(64);
    assert(admitted({ values, ordinal: 0 }));
    assert.deepEqual(Buffer.from(encode({ values, ordinal: 0 })), oracle({ values, ordinal: 0 }));
  }
  for (const ordinal of fixture.rejectedOrdinals) { assert(!admitted({ values: base.values, ordinal })); denied++; }
  for (const extra of fixture.excludedInputs) {
    assert(!validate({ values: base.values, ordinal: base.ordinal, [extra]: "not-coordinate-authority" })); denied++;
  }
  assert(!admitted({ values: ["\ud800", ...base.values.slice(1)], ordinal: 0 })); denied++;
  const native = readFileSync(new URL("./🦀️.rs", import.meta.url), "utf8");
  assert(native.includes("scope: &DocumentScope") && native.includes("parent: &ArtifactRef") && native.includes("child: &ArtifactDialect") && native.includes("scope.document_id != parent.artifact_id"), "native initial identity must use agreeing typed document coordinates");
  assert(native.includes('b"semio.initial-child.v1\\0"') && native.includes("INITIAL_CHILD_FIELD_BYTES: usize = 256") && native.includes("INITIAL_CHILD_ORDINAL_LIMIT: u32 = 64"));
  assert(native.includes('include_str!("🧪️fixtures/🔣️.json")') && native.includes("blake3::hash(&wire)"), "native identity must consume the neutral corpus and independent hash oracle");
  console.log(`[DEBUG] initial-child coordinate source: ${fixture.cases.length} vectors, ${denied} denials, ${fixture.fields.length} UTF-8 boundary frames; native independent BLAKE3/creation/receipt authority unverified`);
}
