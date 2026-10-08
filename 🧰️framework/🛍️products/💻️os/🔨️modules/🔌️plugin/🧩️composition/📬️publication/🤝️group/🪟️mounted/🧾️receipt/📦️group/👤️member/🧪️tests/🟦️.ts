import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv/dist/2020";
const root = new URL("../", import.meta.url);
const fixture = JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json", root), "utf8"));
const schema = JSON.parse(readFileSync(new URL("📐️schema.json", root), "utf8"));
function integer(value: number) { const bytes: number[] = []; do { const byte=value%128;value=Math.floor(value/128);bytes.push(byte|(value?128:0)); } while(value);return Buffer.from(bytes); }
function operation(text: string) { return Buffer.concat([Buffer.from([1,7]),Buffer.from(JSON.stringify(text))]); }
test("member receipt neutral ordered causal corpus agrees with independent Node framing and UTF8", () => {
 expect(new Ajv({strict:false}).compile(schema)(fixture)).toBe(true);
 const inverse=fixture.inverseTexts.map(operation);
 const framed=Buffer.concat([integer(inverse.length),...inverse.flatMap((bytes:Buffer)=>[integer(bytes.length),bytes])]);
 expect(Buffer.byteLength(fixture.operations[0].text)).toBe(8194);expect(fixture.maximumCopyBytes).toBe(64);
 expect(framed.toString("hex")).toBe(fixture.inverseWireHex);expect(integer(0).toString("hex")).toBe(fixture.zeroInverseWireHex);
 expect(framed[0]).toBe(2);expect(Buffer.from(new TextEncoder().encode(fixture.editId))).toEqual(Buffer.from(fixture.editId));
 expect(fixture.operations.map((row:any)=>row.schema+row.suffix)).toEqual(["fixture.first.operation","fixture.second"]);
 for(const row of fixture.operations){expect(operation(row.text).subarray(2).toString()).toBe(JSON.stringify(row.text));expect(row.author??fixture.fallbackAuthor).toBe(row.id==="mutation:one"?"actor:fallback":"actor:explicit");}
 expect(Buffer.from([0])).toEqual(integer(0));expect(framed.includes(Buffer.from("\\u0000"))).toBe(true);
});
test("member receipt source keeps original publication codec and complete causal owners before acknowledgment", () => {
 const source=readFileSync(new URL("🦀️.rs",root),"utf8");
 for(const contract of ["prepared_edit_id", "prepared_operation_count", "prepared_operation_metadata", "prepared_operation_wire_source", "MountedPreparedOperationsBytes", "MountedKernelMutationReceipt", "MountedGroupReceipt::clear_mutation", "operation_index", "next_close_byte_demand"]){expect(source).toContain(contract);}
 expect(source).not.toContain("encode_ops_vec");expect(source).not.toContain(".clone()");
});
