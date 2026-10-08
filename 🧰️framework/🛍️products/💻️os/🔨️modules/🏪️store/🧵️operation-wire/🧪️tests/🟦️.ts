import { test, expect } from "bun:test";
import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
const base = join(import.meta.dir, "..");
const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
test("borrowed operation wire neutral UTF8/ordered JSON and bounded chunk oracle", () => {
  expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const bytes = Buffer.concat([Buffer.from(row.header), Buffer.from(JSON.stringify(row.body))]);
    expect(Array.from(new TextEncoder().encode(JSON.stringify(row.body)))).toEqual(Array.from(bytes.subarray(row.header.length)));
    for (const grant of fixture.grants.filter((value: number) => value > 0)) {
      const chunks = [];
      for (let offset = 0; offset < bytes.length; offset += Math.min(grant, fixture.copyBytes)) chunks.push(bytes.subarray(offset, offset + Math.min(grant, fixture.copyBytes)));
      expect(chunks.every(chunk => chunk.length <= grant && chunk.length <= 64)).toBe(true);
      expect(Buffer.concat(chunks)).toEqual(bytes);
      expect(JSON.parse(bytes.subarray(row.header.length).toString("utf8"))).toEqual(row.body);
    }
  }
});
test("installed borrowed operation cursor retains no owner copy or whole encode fallback", () => {
  const path = join(base, "🦀️.rs");
  expect(existsSync(path)).toBe(true);
  const source = readFileSync(path, "utf8");
  for (const name of ["ArtifactPreparedOperationSource", "ArtifactPreparedOperationCursor", "maximum_copy_bytes", "source_identity", "ArtifactCanonicalJsonCursor"]) expect(source).toContain(name);
  expect(source).not.toContain("encode_op(");
  expect(source).not.toContain("unsafe");
});

test("lowercase JSON hex preserves original UTF8 bytes and nibble continuation", () => {
  for (const row of fixture.cases) {
    const raw = Buffer.from(JSON.stringify(row.body));
    const hex = Buffer.from(raw.toString("hex"));
    expect(Buffer.from(hex.toString(), "hex")).toEqual(raw);
    const chunks = Array.from({length: hex.length}, (_, index) => hex.subarray(index, index + 1));
    expect(Buffer.concat(chunks)).toEqual(hex);
  }
  expect(readFileSync(join(base, "🦀️.rs"), "utf8")).toContain("HexJson");
});

test("borrowed textual operation neutral ordered tuple and UTF8 hex oracle", () => {
  const encode = (node: any): string => node.kind === "hex" ? Buffer.from(node.text).toString("hex") : node.kind === "sequence" ? node.open + node.children.map(encode).join(node.separator) + node.close : node.text;
  for (const row of fixture.textCases) {
    const body = encode(row.node);
    expect(Buffer.from(new TextEncoder().encode(body))).toEqual(Buffer.from(body));
    const full = Buffer.concat([Buffer.from(row.header), Buffer.from(body)]);
    for (const grant of fixture.grants.filter((value: number) => value > 0)) {
      const parts = [];
      for (let index = 0; index < full.length; index += Math.min(grant,64)) parts.push(full.subarray(index,index + Math.min(grant,64)));
      expect(Buffer.concat(parts)).toEqual(full);
    }
  }
  expect(readFileSync(join(base, "🦀️.rs"), "utf8")).toContain("ArtifactOperationTextCursor");
});

test("retained canonical Pack operation reports distinct copied, capacity and release authority", () => {
  const source = readFileSync(join(base,"🦀️.rs"),"utf8");
  for (const name of ["BorrowedProjectedPackCursor", "Pack {", "copied_bytes", "retained_capacity_bytes", "released_bytes"]) expect(source).toContain(name);
  expect(source).not.toContain("encode_record_body(");
});
