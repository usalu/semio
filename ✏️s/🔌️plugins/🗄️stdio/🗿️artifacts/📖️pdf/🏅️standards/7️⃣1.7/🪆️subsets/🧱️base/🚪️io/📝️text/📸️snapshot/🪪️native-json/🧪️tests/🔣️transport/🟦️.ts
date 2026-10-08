/** 🧪️ Artifact-owned physical transport corpus and independent oracles. */
import {test,expect,afterAll} from "bun:test";
import Ajv from "ajv";
import {Buffer} from "node:buffer";
import fixture from "./🧫️fixtures/🔣️.json";
import {pdfCosFromNativeJson,pdfFunctionFromNativeJson} from "./../../🟦️.ts";
import {parsePdfObject,parsePdfFunction} from "./../../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {schema as pdfNativeSchema} from "./../../🧬️schema/🟦️.ts";
const oracle=new Ajv({strict:false});
const word=(number:number):bigint=>{const bytes=Buffer.alloc(8);bytes.writeDoubleBE(number);return bytes.readBigUInt64BE()};
test("PDF native JSON converts only at IO and canonical guards require owned words",()=>{
 const cos=pdfCosFromNativeJson(fixture.pdf.cos),fn=pdfFunctionFromNativeJson(fixture.pdf.function);
 expect(parsePdfObject(cos)).toEqual(cos);
 expect(parsePdfFunction(fn)).toEqual(fn);
 expect(()=>parsePdfObject(fixture.pdf.cos)).toThrow();
 expect(()=>parsePdfFunction(fixture.pdf.function)).toThrow();
 expect(fn.kind).toBe("exponential");if(fn.kind==="exponential")expect(fn.n.bits).toBe(word(fixture.word.number));
 const native={$defs:pdfNativeSchema.$defs,$ref:"#/$defs/PdfFunction"};expect(oracle.compile(native)(fixture.pdf.function)).toBe(true);
});
afterAll(()=>console.log("[DEBUG] Owned transport corpus completed: @semio-tech/stdio-pdf"));
