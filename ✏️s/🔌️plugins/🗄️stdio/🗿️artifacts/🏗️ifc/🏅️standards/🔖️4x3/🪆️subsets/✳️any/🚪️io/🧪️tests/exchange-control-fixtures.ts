/** 🧪️ Reusable actual exchange drivers for neutral cancellation and independent-oracle checks. */
import {readFile} from 'node:fs/promises';
import {bimSpatialToIfc4x3,ifc4x3ToBimSpatial} from '../bim.ts';
import {fem3dToIfc4x3,ifc4x3ToFem3d} from '../fem.ts';
import {extractIfc4x3Triangles} from '../geometry.ts';
import {parseIfc4x3Step} from '../step.ts';
import type {IfcIoControl} from '../control.ts';

const bim=()=>({schema:'s.bim.model@1',project:{name:'Controls',description:'',author:'',organization:'',phase_names:[]},sites:{site:{name:'Site',latitude:0,longitude:0,elevation:0,true_north:0,boundary:[]}}});
const fem=()=>({nodes:[],elements:[],materials:[],sections:[],solids:[],supports:[],loadCases:[],combinations:[],analysis:{modalCount:0n,bucklingCount:0n,deformationScale:{bits:0x3ff0000000000000n}}});
const fixture=async(file:string)=>parseIfc4x3Step(await readFile(new URL(file,import.meta.url),'utf8'));

/** 🎯️ Resolve one real endpoint before attaching the operation's cancellation authority. */
export async function ifcExchangeControlEndpoint(name:string):Promise<(control:IfcIoControl)=>Promise<unknown>> {
 if(name==='bim-to-ifc')return control=>bimSpatialToIfc4x3(bim(),control);
 if(name==='fem-to-ifc')return control=>fem3dToIfc4x3(fem(),control);
 if(name==='ifc-to-bim'){const source=(await bimSpatialToIfc4x3(bim())).artifact!;return control=>ifc4x3ToBimSpatial(source,control);}
 const source=await fixture(name==='ifc-to-fem'?'structural.ifc':'triangulated.ifc');
 if(name==='ifc-to-fem')return control=>ifc4x3ToFem3d(source,control);
 if(name==='ifc-triangles')return control=>extractIfc4x3Triangles(source,control);
 throw new Error(`Unknown endpoint ${name}`);
}
