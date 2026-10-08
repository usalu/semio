import {expect,test} from "bun:test";
import corpus from "../../../../🔨️modules/🗺️surface/🕸️node-graph/📡️scene/🧫️fixtures/🔣️.json";
import wire from "../../../../🔨️modules/🗺️surface/🕸️node-graph/📡️scene/🧫️fixtures/📦️body/🔣️.json";
import {encodePackValue,decodePackValue,packValueToExactJson,packValueToBase64} from "../../🟦️.ts";

test("the actual OS browser producer emits explicit Body vectors for the scene and every embedded slot",()=>{
 const body=encodePackValue(corpus.scene);expect(Buffer.from(body).toString("hex")).toBe(wire.bodyHex);expect(packValueToExactJson(decodePackValue(body))).toEqual(corpus.scene);
 const embedded=Object.fromEntries(corpus.embedded.map(row=>{const field=packValueToBase64(row.value);expect(field.startsWith("pk:")).toBe(true);const bytes=Uint8Array.from(Buffer.from(field.slice(3),"base64"));expect(bytes).toEqual(encodePackValue(row.value));expect(packValueToExactJson(decodePackValue(bytes))).toEqual(row.value);expect(JSON.parse(row.nativeJson)).toEqual(row.value);return[row.field,field];}));
 const combined=encodePackValue({...corpus.scene,...embedded});expect(embedded).toEqual(wire.embedded);expect(Buffer.from(combined).toString("hex")).toBe(wire.combinedHex);expect(packValueToExactJson(decodePackValue(combined))).toEqual({...corpus.scene,...embedded});
});
