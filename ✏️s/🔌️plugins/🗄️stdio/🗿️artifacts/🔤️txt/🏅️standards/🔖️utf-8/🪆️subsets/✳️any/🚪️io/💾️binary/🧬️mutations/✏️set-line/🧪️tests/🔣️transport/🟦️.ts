/** 🧪️ Artifact-owned physical transport corpus and independent oracles. */
import {test,expect,afterAll} from "bun:test";
import Ajv from "ajv";
import {Buffer} from "node:buffer";
import fixture from "./🧫️fixtures/🔣️.json";
import {decodeSetLineProtobuf} from "./../../🟦️.ts";
const oracle=new Ajv({strict:false});
test("TXT physical protobuf field decoding preserves Unicode and rejects truncated frames",()=>{
 const bytes=Uint8Array.from(fixture.txt.bytes),value=decodeSetLineProtobuf(bytes);
 expect(oracle.compile({const:fixture.txt.expected})(value)).toBe(true);
 expect(Buffer.from(bytes.subarray(4)).toString("utf8")).toBe(fixture.txt.expected.text);
 expect(()=>decodeSetLineProtobuf(bytes.subarray(0,bytes.length-1))).toThrow();
});
afterAll(()=>console.log("[DEBUG] Owned transport corpus completed: @semio-tech/stdio-txt"));
