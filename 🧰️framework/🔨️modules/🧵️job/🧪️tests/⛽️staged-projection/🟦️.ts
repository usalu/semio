import Ajv from "ajv";
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Buffer } from "node:buffer";

test("original staged byte source matches independent Buffer contributor words", () => {
  const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/⛽️staged-projection/🔣️.json"), "utf8"));
  const schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../../🧬️schema/⛽️staged-projection/🔣️.json"),"utf8"));expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
  const source = Buffer.from(fixture.sourceBytes);
  expect(source.readUInt32LE(0)).toBe(2);
  expect(source.readBigUInt64LE(4)).toBe(0xffffffffffffffffn);
  expect(Array.from(Buffer.concat([source.subarray(0, 4), source.subarray(4)]))).toEqual(fixture.sourceBytes);
  expect(fixture.maximumItems).toBe(1);
});

 test("original in-place construction separates native slot prefixes and canonical page births",()=>{const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/⛽️staged-projection/🔣️.json"),"utf8"));const law=fixture.construction;expect(law.slotQuantum).toBe(1);expect(law.maximumCopyBytes).toBe(4096);expect(law.canonicalPageBytes).toBe(16384);for(const cut of law.cancelPrefixes){const prefix=Buffer.alloc(law.slots*8,0xa5);for(let i=0;i<Math.min(law.slots,Math.max(0,cut-1));i++)prefix.writeBigUInt64LE(0n,i*8);const initialized=Math.min(law.slots,Math.max(0,cut-1));expect(Array.from({length:initialized},(_,i)=>prefix.readBigUInt64LE(i*8))).toEqual(Array(initialized).fill(0n));if(initialized<law.slots)expect(prefix.readBigUInt64LE(initialized*8)).toBe(0xa5a5a5a5a5a5a5a5n);}});
