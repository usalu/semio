/** 🧪️ FEM export refuses orphan analytical nodes before returning any IFC artifact. */
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import cases from "./fem-isolated-cases.json";
import schema from "./fem-isolated.schema.json";
import {femConnectivityFixture} from "./fem-isolated-fixtures.ts";
import {fem3dToIfc4x3,ifc4x3ToFem3d} from "../fem.ts";
import {validateIfc4x3Snapshot} from "../../🧬️schema/📚️catalog/🟦️.ts";
import type {IfcIoControl} from "../control.ts";

test("FEM isolated-node vectors have a closed language-neutral schema",()=>{
 expect(new Ajv({strict:true,allErrors:true}).compile(schema)(cases)).toBe(true);
});
for(const row of cases.cases)test("FEM analytical connectivity "+row.id,async()=>{
 const model=femConnectivityFixture(row),before=structuredClone(model),outcome=await fem3dToIfc4x3(model);
 expect(model).toEqual(before);
 if(row.code){
  expect(outcome.artifact).toBeUndefined();expect(outcome.fidelity).toBe("unsupported");expect(outcome.diagnostics[0]?.code).toBe(row.code);expect(outcome.diagnostics[0]?.severity).toBe("error");
 }else{
  expect(outcome.artifact).toBeDefined();expect(validateIfc4x3Snapshot(outcome.artifact!)).toEqual([]);
  const imported=await ifc4x3ToFem3d(outcome.artifact!);expect(imported.artifact?.nodes.length).toBe(row.nodes.length);expect(imported.artifact?.elements.length).toBe(row.members.length);
 }
});
for(const cut of [0,1,2,3])test("FEM topology cancellation retains original controls at prefix "+cut,async()=>{
 const row=cases.cases.find(row=>row.id==="connected-frame")!,model=femConnectivityFixture(row),before=structuredClone(model),original=new AbortController(),replacement=new AbortController();let callbacks=0,replacementCallbacks=0,published=false;
 const control:IfcIoControl={signal:original.signal,batchSize:1,progress:progress=>{
  if(progress.phase!==cases.phase)return;
  if(callbacks++===cut){control.signal=replacement.signal;control.progress=()=>replacementCallbacks++;original.abort();}
 }};
 await expect(fem3dToIfc4x3(model,control).then(outcome=>{published=outcome.artifact!==undefined;})).rejects.toHaveProperty("code","io.cancelled");
 expect(callbacks).toBe(cut+1);expect(replacementCallbacks).toBe(0);expect(published).toBe(false);expect(model).toEqual(before);
});
