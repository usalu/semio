import {expect,test} from "bun:test";
import { nodeGraphSceneFixture as corpus, graphSceneBodyFixture as wire } from "@semio-tech/framework-surface-ts/testing/fixtures";
import {encodePackValue,decodePackValue,packValueToExactJson,packValueToBase64} from "../../🟦️.ts";
import type {PackValue} from "@semio-tech/framework/packs";

test("the actual OS browser producer emits explicit Body vectors for the scene and every embedded slot",()=>{
 const body=encodePackValue(corpus.scene);expect(Buffer.from(body).toString("hex")).toBe(wire.bodyHex);expect(packValueToExactJson(decodePackValue(body))).toEqual(corpus.scene);
 const embeddedRows:{field:string;value:PackValue;nativeJson:string}[]=JSON.parse(JSON.stringify(corpus.embedded));
 const embedded=Object.fromEntries(embeddedRows.map(row=>{const field=packValueToBase64(row.value);expect(field.startsWith("pk:")).toBe(true);const bytes=Uint8Array.from(Buffer.from(field.slice(3),"base64"));expect(bytes).toEqual(encodePackValue(row.value));expect(packValueToExactJson(decodePackValue(bytes))).toEqual(row.value);expect(JSON.parse(row.nativeJson)).toEqual(row.value);return[row.field,field];}));
 const combined=encodePackValue({...corpus.scene,...embedded});expect(embedded).toEqual(wire.embedded);expect(Buffer.from(combined).toString("hex")).toBe(wire.combinedHex);expect(packValueToExactJson(decodePackValue(combined))).toEqual({...corpus.scene,...embedded});
});
