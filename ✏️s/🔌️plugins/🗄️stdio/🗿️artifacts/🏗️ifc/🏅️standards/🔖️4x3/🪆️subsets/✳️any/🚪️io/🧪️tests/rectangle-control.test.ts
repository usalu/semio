/** 🚦️ Rectangular geometry publishes only after original evaluator completion. */
import {expect,test} from 'bun:test';
import {ifcRectangleFixture} from './rectangle-extrusion-fixtures.ts';
import cases from './rectangle-extrusion-cases.json';
import {extractIfc4x3Triangles} from '../geometry.ts';
import {ifc4x3ToGltf,writeIfcGltfGlb} from '../gltf.ts';
import {IfcStepError,type IfcIoControl} from '../control.ts';
import {IfcEditError} from '../../🧬️schema/🔺️diff/🟦️.ts';

for(const endpoint of ['triangles','glb'])for(const frontier of ['initial','interior','completion'])for(const fault of ['abort','error','step-error','edit-error','undefined','number'])test('rectangle '+endpoint+' '+frontier+' '+fault,async()=>{
 const source=ifcRectangleFixture(cases.cases[0]!).snapshot,before=structuredClone(source),events:number[]=[];
 const run=async(control:IfcIoControl)=>{if(endpoint==='triangles')return extractIfc4x3Triangles(source,control);const result=await ifc4x3ToGltf(source,control);return writeIfcGltfGlb(result.artifact!,control);};
 await run({batchSize:4,progress:event=>{if(event.phase==='extrusions')events.push(event.completed);}});expect(events.length).toBeGreaterThan(2);
 const target=frontier==='initial'?0:frontier==='interior'?1:events.length-1,original=new AbortController(),replacement=new AbortController(),failure=fault==='error'?new Error('caller'):fault==='step-error'?new IfcStepError('geometry.caller','caller'):fault==='edit-error'?new IfcEditError('invalidValue','caller'):fault==='number'?17:undefined;
 let calls=0,replacementCalls=0,caught=false;const control:IfcIoControl={signal:original.signal,batchSize:4,progress(event){if(event.phase!=='extrusions'||calls++!==target)return;control.signal=replacement.signal;control.progress=()=>replacementCalls++;if(fault==='abort')original.abort();else throw failure;}};
 try{await run(control);}catch(error){caught=true;if(fault==='abort')expect(error).toMatchObject({code:'io.cancelled'});else expect(error).toBe(failure);}
 expect(caught).toBe(true);expect(calls).toBe(target+1);expect(replacementCalls).toBe(0);expect(source).toEqual(before);
});
test('rectangle semantic callbacks cannot replace admitted original profile geometry',async()=>{
 const source=ifcRectangleFixture(cases.cases[0]!).snapshot,profile=source.entities.find(row=>row.name==='IFCRECTANGLEPROFILEDEF')!;
 let changed=false;const result=await extractIfc4x3Triangles(source,{batchSize:4,progress(event){if(event.phase==='extrusions'&&!changed){changed=true;profile.args[3]={kind:'integer',value:1000n};}}});
 expect(changed).toBe(true);expect(result.diagnostics).toEqual([]);expect(result.artifact![0]!.positions.map(p=>p[0])).toEqual([-1,1,1,-1,-1,1,1,-1]);expect(profile.args[3]).toEqual({kind:'integer',value:1000n});
});
