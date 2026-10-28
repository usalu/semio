/** 📚 Exact official source files exercise independent conformance baselines and lossless editing. */
import {test,expect} from "bun:test";
import {readFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import {resolve} from "node:path";
import Ajv from "ajv/dist/2020";
import manifest from "../🧪️fixtures/official/cases.json";
import schema from "../🧪️fixtures/official/schema.json";
import {parseIfc4x3Step,writeIfc4x3Step} from "../🚪️io/step.ts";
import {applyIfcDiff} from "../🧬️schema/🔺️diff/🟦️.ts";
import {compileIfc4x3Express} from "../🧬️schema/⚖️validation/compiler.ts";
import {validateIfc4x3Express} from "../🧬️schema/⚖️validation/population.ts";
const directory=resolve(import.meta.dir,"../🧪️fixtures/official"),compilation=compileIfc4x3Express();
test("official corpus metadata has a closed schema and one exact source identity per case",()=>{
 const admit=new Ajv({strict:true,allErrors:true}).compile(schema);
 expect(admit(manifest)).toBe(true);
 expect(new Set(manifest.cases.map(row=>row.file)).size).toBe(46);
 expect(manifest.cases.reduce((count,row)=>count+row.entities,0)).toBe(8172);
});
for(const row of manifest.cases)test(`official ADD2 model ${row.file}`,async()=>{
 const bytes=await readFile(resolve(directory,row.file));
 expect(bytes.length).toBe(row.bytes);
 expect(createHash("sha256").update(bytes).digest("hex")).toBe(row.sha256);
 expect(createHash("sha1").update(`blob ${bytes.length}\0`).update(bytes).digest("hex")).toBe(row.blob);
 const before=await parseIfc4x3Step(bytes.toString("utf8"),{batchSize:4096});
 expect(before.entities.length).toBe(row.entities);
 expect(applyIfcDiff(before,{})).toEqual(before);
 expect(await parseIfc4x3Step(await writeIfc4x3Step(before,{batchSize:4096}),{batchSize:4096})).toEqual(before);
 const compiled=await compilation;
 expect(compiled.complete).toBe(true);
 const semantic=await validateIfc4x3Express(before,{maximumWork:20_000_000,batchSize:4096},compiled.program);
 if(!row.diagnostics.length){expect(semantic.complete).toBe(true);expect(semantic.valid).toBe(true);expect(semantic.diagnostics).toEqual([]);}
 else{expect(semantic.valid).toBe(false);expect(semantic.diagnostics.map(value=>({entityId:value.entityId?.toString(),attribute:value.message.split(" ")[0]}))).toEqual(row.diagnostics.map(value=>({entityId:value.entityId,attribute:value.attribute})));}
},30_000);
