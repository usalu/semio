import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {inflateRawSync} from "node:zlib";
import {decodeJsonPack,decodeJsonPayload,encodeJsonPack,type PackJson} from "../../../../../📚️catalog/🛠️extract/📦️json-pack.ts";
import {compiledProgramJson,tableFiles} from "../🧬️schema/🏭️pack-tables.ts";
const LIMIT=262144;
for(const[name,files]of Object.entries(tableFiles))test(`IFC4x3 ${name} resource is the exact pack of its neutral JSON and fits one native compilation unit`,()=>{
 const pack=new Uint8Array(readFileSync(files.pack)),neutral=JSON.parse(readFileSync(files.json,"utf8")) as PackJson;
 expect(decodeJsonPack(pack)).toEqual(neutral);
 expect(Buffer.from(encodeJsonPack(neutral))).toEqual(Buffer.from(pack));
 expect(decodeJsonPayload(inflateRawSync(pack))).toEqual(neutral);
 expect(pack.length).toBeLessThan(name==="templates"?LIMIT-12000:LIMIT/2);
});
test("IFC4x3 program.json is the compiled canonical program of the retained catalog",async()=>{expect(readFileSync(tableFiles.program.json,"utf8")).toBe(await compiledProgramJson()+"\n");});
test("IFC4x3 mvd scope ids are unique so the sorted native lookup is exact",()=>{const neutral=JSON.parse(readFileSync(tableFiles.mvd.json,"utf8")) as {entities:{id:string}[];concepts:{id:string}[]};for(const rows of [neutral.entities,neutral.concepts])expect(new Set(rows.map(row=>row.id)).size).toBe(rows.length);});
