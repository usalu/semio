import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { SchemaScript } from "../../../../../../../../📜️script.ts";
import { createScriptProcessEnvelope, withScriptProcessEnvelope } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import Ajv from "ajv";
import { loadProductionSchemaEntriesV1 } from "../🟦️.ts";

const schema=JSON.parse(readFileSync(new URL("../../../../../../../🔨️modules/🧬️schema/🔣️.json",import.meta.url),"utf8"));
const examples=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
const oracle=new Ajv({strict:false}).compile(schema);

test("production schema registry consumes fresh producer output with independent JSON and Ajv",async()=>{
 for(const row of examples.cases){
  const dump={contractId:row.contractId??"schema-export-registry-entries-v1",generator:row.generator??"semio-framework-schema schema-export-entries",entries:[{scope:"framework.schema",export:"SchemaFormat",format:row.format??"rust"}]};
  expect(oracle(dump),row.name).toBe(row.expectedValid);
  let completed=false,calls=0;
  const producer={async produce(){calls++;await Promise.resolve();completed=true;return JSON.stringify(dump);}};
  if(row.expectedValid){
   expect(await loadProductionSchemaEntriesV1(producer,schema)).toEqual(JSON.parse(JSON.stringify(dump)));
   expect(completed).toBe(true);expect(calls).toBe(1);
   await loadProductionSchemaEntriesV1(producer,schema);expect(calls).toBe(2);
  }else await expect(loadProductionSchemaEntriesV1(producer,schema)).rejects.toThrow();
 }
 console.log("[DEBUG] production registry fresh routing / original canonical Schema / independent JSON + Ajv examples="+examples.cases.length);
});

test("production schema registry preserves failed and cancelled producer outcomes",async()=>{
 for(const reason of ["native failed","cancelled"]){
  const failure=Error(reason);
  await expect(loadProductionSchemaEntriesV1({async produce(){throw failure;}},schema)).rejects.toBe(failure);
 }
});

test("production schema registry refuses the genuine alternate entity catalog root",async()=>{
 const text=readFileSync(new URL("../../../../../../../🔨️modules/🧬️schema/🏷️entity-kinds/🔣️.json",import.meta.url),"utf8");
 const value=JSON.parse(text);
 expect(oracle(value)).toBe(true);
 const registryOracle=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/SchemaExportEntries"});
 expect(registryOracle(value)).toBe(false);
 await expect(loadProductionSchemaEntriesV1({async produce(){return text;}},schema)).rejects.toThrow();
});


test("original schema routes refuse test inputs and protect existing output before acquisition",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"schema-registry-test",maximumElapsedMilliseconds:0},{},Date.now()),async invocation=>{
 const root=resolve(import.meta.dir,"../../../../../../../..");
 const script=new SchemaScript(root,root,invocation);
 for(const route of ["verify","entries"])
  await expect(script.run([route,"--rust-entries","testing-input.json"])).rejects.toThrow("Registry inputs are produced fresh");
 await expect(script.run(["entries","--out"])).rejects.toThrow("--out requires a path");
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!base)throw Error("Original schema route test requires its declared test artifact directory");
 const directory=mkdtempSync(join(base,"registry-refusal-")), output=join(directory,"existing.json");
 try{
  const bytes=Buffer.from("original output \0 🧫️", "utf8");
  writeFileSync(output,bytes);
  await expect(script.run(["entries","--out",output])).rejects.toThrow("--out requires a new output file");
  expect(readFileSync(output)).toEqual(bytes);
  console.log("[DEBUG] original SchemaScript rejects two test-input routes / missing output / preserves real existing UTF8 bytes before acquisition");
 }finally{rmSync(directory,{recursive:true,force:true});}
}));
