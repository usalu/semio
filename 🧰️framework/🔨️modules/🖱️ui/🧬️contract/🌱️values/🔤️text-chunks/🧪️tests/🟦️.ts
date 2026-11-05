import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Buffer} from "node:buffer";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {splitUiTextChunks,joinUiTextChunks} from "../🟦️.ts";

test("bounded text chunks preserve multilingual data with independent UTF8 and schema oracles",()=>{
 const validate=new Ajv({strict:true}).compile(schema);
 for(const row of fixture.cases){
  const source=row.unit.repeat(row.repeat),chunks=splitUiTextChunks(source,row.maximumBytes);
  expect(validate(chunks)).toBe(true);
  expect(chunks.join("")).toBe(source);
  expect(joinUiTextChunks(chunks,row.maximumBytes)).toBe(source);
  expect(Buffer.concat(chunks.map(chunk=>Buffer.from(chunk)))).toEqual(Buffer.from(source));
  expect(chunks.every(chunk=>Buffer.byteLength(chunk)<=512)).toBe(true);
  if(row.bytes!==undefined)expect(chunks.map(chunk=>Buffer.byteLength(chunk))).toEqual(row.bytes);
 }
 for(const row of fixture.refusals)expect(()=>splitUiTextChunks(row.unit.repeat(row.repeat),row.maximumBytes)).toThrow();
 expect(()=>splitUiTextChunks(String.fromCharCode(0xd800),512)).toThrow();
 for(const chunks of [[],[5],["é".repeat(257)],[String.fromCharCode(0xd800)],Array(257).fill("")])expect(()=>joinUiTextChunks(chunks,65536)).toThrow();
});
