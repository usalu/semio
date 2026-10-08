/** 🧪️ Artifact-owned physical transport corpus and independent oracles. */
import {test,expect,afterAll} from "bun:test";
import Ajv from "ajv";
import {Buffer} from "node:buffer";
import fixture from "./🧫️fixtures/🔣️.json";
import {block3dSnapshotFromJsonText,block3dSnapshotToJsonText} from "./../../🟦️.ts";
const oracle=new Ajv({strict:false});
const word=(number:number):bigint=>{const bytes=Buffer.alloc(8);bytes.writeDoubleBE(number);return bytes.readBigUInt64BE()};
test("Block 3D physical JSON resolves words before canonical snapshot admission",()=>{
 const owned=block3dSnapshotFromJsonText(JSON.stringify(fixture.block)),json=JSON.parse(block3dSnapshotToJsonText(owned));
 expect(json.vortices[0].radius.bits).toBe(word(fixture.block.vortices[0]!.radius).toString(16).padStart(16,"0"));
 expect(block3dSnapshotFromJsonText(JSON.stringify(json))).toEqual(owned);
 expect(oracle.compile({const:json})(JSON.parse(block3dSnapshotToJsonText(owned)))).toBe(true);
});
afterAll(()=>console.log("[DEBUG] Owned transport corpus completed: @semio-tech/block-3d"));
