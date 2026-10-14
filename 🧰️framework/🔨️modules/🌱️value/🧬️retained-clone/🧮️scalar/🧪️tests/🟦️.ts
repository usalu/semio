import Ajv from "ajv";
import {expect,test} from "bun:test";
import {applyPatch} from "fast-json-patch";
import law from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("original scalar source and exact payload agree with independent codecs",()=>{
  const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,deniedLeaseCountChanges:1})).toBe(false);expect(validate({...law,depth:0})).toBe(false);
  for(const value of law.values){const original={value},copy=applyPatch({},[{op:"add",path:"/scalar",value:original}],true,false).newDocument.scalar;expect(copy).toEqual(original);const bytes=Buffer.alloc(law.payloadBytes);bytes.writeBigUInt64LE(BigInt(value));const independent=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength);expect(independent.getBigUint64(0,true).toString()).toBe(value);expect(bytes.byteLength).toBe(BigUint64Array.BYTES_PER_ELEMENT);expect(original.value).toBe(value);}
  console.log("[DEBUG] Strict scalar lease-stutter policy and original u64 payload agree with JSON Patch and independent 64-bit codecs");
});

test("scalar binding admission quotes the original atom while its metadata receipt remains zero",()=>{expect(law.bindingAdmissionBytes).toBe(Math.max(law.bindingCopyBytes,BigUint64Array.BYTES_PER_ELEMENT));expect(law.bindingCopyBytes).toBe(0);for(const budget of[0,7,8]){const admitted=budget>=law.bindingAdmissionBytes;expect(admitted).toBe(budget>=Buffer.alloc(law.payloadBytes).byteLength);expect(budget<law.bindingAdmissionBytes).toBe(!admitted);}});
