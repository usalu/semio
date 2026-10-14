/** 📦️ Language-neutral native archive witnesses with independent system and Ajv oracles. */
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {decodeJackArchive,encodeJackArchive} from "../🟦️.ts";
const fixture=await Bun.file(new URL("./🧫️fixtures/📦️archive.json",import.meta.url)).json();
const schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();
const validate=new Ajv().compile(schema);
const control=()=>({admit:(bytes:number)=>{expect(bytes).toBeLessThanOrEqual(fixture.maximumBytes*2+128);},checkpoint:(_completed:number,_total:number)=>{},yield:async()=>{}});
test("specific Jack archive carriers preserve every original octet against independent codecs",async()=>{
 for(const row of fixture.cases){const bytes=new Uint8Array(row.bytes);for(const format of ["json","txt"] as const){const text=encodeJackArchive(bytes,format,control());expect(text).toBe(row[format]);expect([...await decodeJackArchive(text,format,control())]).toEqual(row.bytes);expect(new TextEncoder().encode(text)).toEqual(new TextEncoder().encode(row[format]));if(format==="json"){const oracle=JSON.parse(text);expect(validate(oracle)).toBe(true);expect([...Buffer.from(oracle.archive,"hex")]).toEqual(row.bytes);}}}
});
test("specific Jack archive carriers refuse altered closed schema and incomplete bytes",async()=>{for(const row of fixture.refusals)await expect(decodeJackArchive(row.text,row.format,control())).rejects.toThrow();expect(()=>encodeJackArchive(new Uint8Array([0]),"txt",control())).toThrow();});
test("specific Jack archive carrier obeys original admission and cancellation before copies",async()=>{const bytes=new Uint8Array(fixture.cases[2].bytes);let admitted=false;expect(()=>encodeJackArchive(bytes,"json",{admit:()=>{throw Error("original capacity refusal");},checkpoint:()=>{admitted=true;},yield:async()=>{}})).toThrow("original capacity refusal");expect(admitted).toBe(false);let ticks=0;await expect(decodeJackArchive(fixture.cases[2].txt,"txt",{admit:()=>{},checkpoint:()=>{if(++ticks===2)throw Error("original cancellation");},yield:async()=>{}})).rejects.toThrow("original cancellation");});
