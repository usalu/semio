/** 🧭️ Actual geometry queries enforce normative placement dimensions and finite directions. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './axis-cases.json';
import schema from './axis.schema.json';
import {ifcAxisFixture} from './axis-fixtures.ts';
import {IfcGeometryGraph,extractIfc4x3Triangles} from '../geometry.ts';
import {IfcStepError,type IfcIoControl} from '../control.ts';
import {ifcTfsFixture} from './tfs-admission-fixtures.ts';
import tfsCases from './tfs-admission-cases.json';
import {ifc4x3ToGltf,writeIfcGltfGlb} from '../gltf.ts';

test('Cartesian axis cases have a closed language-neutral schema',()=>expect(new Ajv({strict:true,allErrors:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test(`Cartesian placement ${row.id}`,async()=>{
 const {snapshot,placement}=ifcAxisFixture(row),result=await IfcGeometryGraph.capture(snapshot);expect(result.artifact).toBeDefined();const graph=result.artifact!;
 if(row.error){expect(()=>graph.axis(placement)).toThrow(expect.objectContaining({code:row.error}));return;}
 const actual=graph.axis(placement);for(let i=0;i<16;i++)expect(actual[i]).toBeCloseTo(row.matrix![i]!,14);
});


test('placement boundaries preserve same-class original observer identity',async()=>{
 const source=ifcTfsFixture(tfsCases.cases[0]!).snapshot,before=structuredClone(source);
 for(const endpoint of ['triangles','glb'])for(const code of ['geometry.declaration','exchange.vector','geometry.domain'])for(const frontier of ['initial','interior','completion']){
  const events:number[]=[],run=async(control:IfcIoControl)=>{if(endpoint==='triangles')return extractIfc4x3Triangles(source,control);const out=await ifc4x3ToGltf(source,control);return writeIfcGltfGlb(out.artifact!,control);};await run({batchSize:1,progress:event=>{if(event.phase==='placements')events.push(event.completed);}});expect(events.length).toBeGreaterThan(2);const target=frontier==='initial'?0:frontier==='completion'?events.length-1:1,original=new IfcStepError(code,'exact original placement observer');let calls=0,redirected=0,caught=false;const control:IfcIoControl={batchSize:1,progress:event=>{if(event.phase==='placements'&&calls++===target){control.progress=()=>{redirected++;};throw original;}}};
  try{await run(control);}catch(error){caught=true;expect(error).toBe(original);}expect(caught).toBe(true);expect(calls).toBe(target+1);expect(redirected).toBe(0);expect(source).toEqual(before);
 }
});
