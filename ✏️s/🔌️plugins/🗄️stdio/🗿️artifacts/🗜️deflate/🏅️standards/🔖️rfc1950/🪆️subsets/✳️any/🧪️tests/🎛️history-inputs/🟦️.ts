import { expect, test } from "bun:test";
import Ajv from "ajv";
import { deflateSync, inflateSync } from "node:zlib";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

const root=resolve(import.meta.dir,"../..");
const read=(path:string)=>JSON.parse(readFileSync(resolve(root,path),"utf8"));
test("Deflate committed header and payload intents agree with independent zlib",()=>{
  const before={schema:"stdio.deflate",compressionMethod:8,windowBits:7,compressionLevelHint:"default",payload:[104,105]};
  const cases=[
    {id:"header", mutation:{mutation:"setCompressionParams",method:8,window_bits:6,level_hint:"maximum"},after:{...before,windowBits:6,compressionLevelHint:"maximum"}},
    {id:"payload", mutation:{mutation:"setPayload",payload:[104,105,33]},after:{...before,payload:[104,105,33]}}
  ];
  const ajv=new Ajv({strict:false});
  const snapshot=ajv.compile(read("🧬️schema/📸️snapshot/🔣️.json"));
  expect(snapshot(before)).toBe(true);
  for(const row of cases){
    expect(snapshot(row.after)).toBe(true);
    const leaf=row.id==="header"?"🧮set-compression-params":"📦set-payload";
    expect(ajv.compile(read(`🧬️schema/🧬️mutations/${leaf}/🧬️schema/🔣️.json`))(row.mutation)).toBe(true);
    const output=deflateSync(Buffer.from(row.after.payload),{windowBits:row.after.windowBits+8,level:row.after.compressionLevelHint==="maximum"?9:6});
    expect([...inflateSync(output)]).toEqual(row.after.payload);
    expect(output[0]>>4).toBe(row.after.windowBits);
    expect(output[1]>>6).toBe(row.after.compressionLevelHint==="maximum"?3:2);
    const path=resolve(root,`🧫️fixtures/🧬️history-edits/${row.id}/🦠️mutation/🔣️.json`);
    expect(existsSync(path)).toBe(true);
    expect(JSON.parse(readFileSync(path,"utf8"))).toEqual({mutation:row.mutation,before,after:row.after});
    console.log("[DEBUG] Deflate committed native intent agrees with independent zlib",row.id,output[0],output[1],row.after.payload);
  }
});
