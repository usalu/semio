import {test,expect} from "bun:test";
import {readFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import {parseIfc4x3Step,ifcReal} from "../step.ts";
import {extractIfc4x3Triangles} from "../geometry.ts";
import {ifc4x3ToGltf,gltfToIfc4x3,writeIfcGltfGlb,readIfcGltfGlb} from "../gltf.ts";
import {validateIfc4x3Snapshot} from "../../🧬️schema/📚️catalog/🟦️.ts";
import {applyIfcDiff,inverseIfcDiff} from "../../🧬️schema/🔺️diff/🟦️.ts";
import {ifcMutationDiff} from "../../🧬️schema/🧬️mutations/🟦️.ts";
import productCases from './geometry-product-cases.json';
import productSchema from './geometry-product.schema.json';
import Ajv from 'ajv/dist/2020';

const fixture=async()=>parseIfc4x3Step(await readFile(new URL("triangulated.ifc",import.meta.url),"utf8"));
test('represented-product eligibility comes from complete IFC declarations',async()=>{
 expect(new Ajv({strict:true,allErrors:true}).compile(productSchema)(productCases)).toBe(true);
 const original=await fixture();
 for(const row of productCases.cases){
  const source=structuredClone(original),product=source.entities.find(entity=>entity.id===1000n)!;product.name=row.entity;
  const before=structuredClone(source),outcome=await extractIfc4x3Triangles(source);
  expect(outcome.artifact?.length).toBe(row.isProduct?1:0);
  expect(outcome.diagnostics.some(diagnostic=>diagnostic.code==='geometry.unbound-item')).toBe(!row.isProduct);
  expect(source).toEqual(before);
 }
});
test("official IFC editing central diff and inverse preserve exchange graph",async()=>{
 const before=await fixture(),diff=ifcMutationDiff(before,{mutation:"setEntityArg",id:1000n,index:2,value:{kind:"string",value:"Renamed proxy"}}),after=applyIfcDiff(before,diff);
 expect(after.entities.find(e=>e.id===1000n)?.args[2]).toEqual({kind:"string",value:"Renamed proxy"});
 expect(applyIfcDiff(after,inverseIfcDiff(before,diff))).toEqual(before);
});
test("official IFC geometry applies nested placement and millimetre units",async()=>{
 const outcome=await extractIfc4x3Triangles(await fixture());
 expect(outcome.fidelity).toBe("geometry");expect(outcome.diagnostics).toEqual([]);
 const mesh=outcome.artifact![0]!;expect(mesh.positions.length).toBe(8);expect(mesh.indices.length).toBe(36);
 expect([0,1,2].map(k=>Math.min(...mesh.positions.map(p=>p[k]!)))).toEqual([.5,-.5,0]);
 expect([0,1,2].map(k=>Math.max(...mesh.positions.map(p=>p[k]!)))).toEqual([1.5,.5,2]);
});
test("actual glTF artifact and GLB roundtrip produce schema-valid IFC proxies",async()=>{
 const exported=await ifc4x3ToGltf(await fixture());expect(exported.diagnostics.some(d=>d.code==="exchange.loss")).toBe(true);
 const glb=await writeIfcGltfGlb(exported.artifact!),parsed=await readIfcGltfGlb(glb),imported=await gltfToIfc4x3(parsed);
 expect(parsed.document.meshes.length).toBe(1);expect(imported.fidelity).toBe("geometry");expect(imported.artifact).toBeDefined();
 expect(validateIfc4x3Snapshot(imported.artifact!)).toEqual([]);
 const mesh=(await extractIfc4x3Triangles(imported.artifact!)).artifact![0]!;
 expect(mesh.positions.length).toBe(8);expect(mesh.indices.length).toBe(36);
 expect([0,1,2].map(k=>Math.min(...mesh.positions.map(p=>p[k]!)))).toEqual([.5,-.5,0]);
 expect([0,1,2].map(k=>Math.max(...mesh.positions.map(p=>p[k]!)))).toEqual([1.5,.5,2]);
});
test("geometry unsupported items and cancellation are explicit",async()=>{
 const snapshot=await fixture(),item=snapshot.entities.find(e=>e.id===1021n)!;item.name="IFCPOLYGONALFACESET";
 const malformed=await extractIfc4x3Triangles(snapshot);expect(malformed.artifact).toEqual([]);expect(malformed.diagnostics[0]?.code).toBe("geometry.unsupported-product");expect(malformed.diagnostics[0]?.severity).toBe('error');
 item.name='IFCSPHERE';item.args=[{kind:'reference',value:snapshot.entities.find(e=>e.name==='IFCAXIS2PLACEMENT3D')!.id},ifcReal(1)];snapshot.entities.find(e=>e.name==='IFCSHAPEREPRESENTATION')!.args[2]={kind:'string',value:'CSG'};
 const extracted=await extractIfc4x3Triangles(snapshot);expect(extracted.artifact).toEqual([]);expect(extracted.diagnostics[0]?.code).toBe("geometry.unsupported-item");
 const abort=new AbortController();abort.abort();await expect(ifc4x3ToGltf(await fixture(),{signal:abort.signal})).rejects.toHaveProperty("code","io.cancelled");
});
test("official Khronos GLB imports actual indexed triangle data",async()=>{
 const bytes=await readFile(new URL("box.glb",import.meta.url)),provenance=JSON.parse(await readFile(new URL("box.provenance.json",import.meta.url),"utf8"));
 expect(createHash("sha256").update(bytes).digest("hex")).toBe(provenance.sha256);
 const gltf=await readIfcGltfGlb(bytes),outcome=await gltfToIfc4x3(gltf);expect(outcome.artifact).toBeDefined();
 expect(validateIfc4x3Snapshot(outcome.artifact!)).toEqual([]);
 const mesh=(await extractIfc4x3Triangles(outcome.artifact!)).artifact![0]!;expect(mesh.positions.length).toBe(24);expect(mesh.indices.length).toBe(36);
 expect([0,1,2].map(k=>Math.min(...mesh.positions.map(p=>p[k]!)))).toEqual([-.5,-.5,-.5]);
 expect([0,1,2].map(k=>Math.max(...mesh.positions.map(p=>p[k]!)))).toEqual([.5,.5,.5]);
 expect(outcome.diagnostics.some(d=>d.code==="exchange.loss")).toBe(true);
});
