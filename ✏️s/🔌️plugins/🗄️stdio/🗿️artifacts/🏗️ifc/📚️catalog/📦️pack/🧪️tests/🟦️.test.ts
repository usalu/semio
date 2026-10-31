import {test} from "node:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {inflateRawSync} from "node:zlib";
import {extractExpressCatalog} from "../../🛠️extract/🟦️.ts";
import {encodeCatalogPack,decodeCatalogPack,decodeCatalogPayload} from "../../🛠️extract/📦️pack.ts";
const bytes=(url:string):Uint8Array=>new Uint8Array(readFileSync(new URL(url,import.meta.url))),retained=(url:string)=>{const{validation:_validation,...rest}=JSON.parse(readFileSync(new URL(url,import.meta.url),"utf8"));return rest;};
const standards=[["2x3","../../../🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/"],["4","../../../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/"],["4x3","../../../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/"]] as const;
test("minimal fixture decodes to its neutral JSON and regenerates byte for byte",()=>{const pack=bytes("./🧫️fixtures/minimal/catalog.ifcpack"),expected=retained("./🧫️fixtures/minimal/🔣️.json");assert.deepEqual(decodeCatalogPack(pack),expected);assert.deepEqual(encodeCatalogPack(expected),pack);});
for(const[id,dir]of standards)test(`IFC ${id} committed resource is the exact pack of its neutral JSON and stays inside the source boundary`,()=>{const pack=bytes(dir+"catalog.ifcpack"),expected=retained(dir+"🔣️.json");assert.deepEqual(decodeCatalogPack(pack),expected);assert.deepEqual(encodeCatalogPack(expected),pack);assert.ok(pack.length<131072);assert.deepEqual(decodeCatalogPayload(inflateRawSync(pack)),expected);});
test("hostile payloads are refused",()=>{const payload=inflateRawSync(bytes("./🧫️fixtures/minimal/catalog.ifcpack"));assert.throws(()=>decodeCatalogPayload(payload.subarray(0,payload.length-1)),/truncated|index/);assert.throws(()=>decodeCatalogPayload(Uint8Array.from([...payload,0])),/trailing/);assert.throws(()=>decodeCatalogPayload(Uint8Array.from([0,...payload.subarray(1)])),/magic/);});
const sources=[["2x3","IFC2X3_TC1.exp"],["4","IFC4.exp"],["4x3","IFC4X3_ADD2.exp"]] as const;
for(const[id,exp]of sources){const dir=standards.find(row=>row[0]===id)![1];test(`IFC ${id} extractor reproduces the committed neutral JSON from the pinned EXPRESS source`,()=>{const text=readFileSync(new URL(dir+exp,import.meta.url),"utf8"),manifest=JSON.parse(readFileSync(new URL(dir+"source.json",import.meta.url),"utf8")),{validation:_validation,...expected}=JSON.parse(readFileSync(new URL(dir+"🔣️.json",import.meta.url),"utf8")),{validation:_extracted,...extracted}=extractExpressCatalog(text,manifest);assert.deepEqual(extracted,expected);});}
