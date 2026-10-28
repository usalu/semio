/** 🧪️ Valid IFC source drivers are shared by source-admission tests and independent oracles. */
import {readFile} from 'node:fs/promises';
import {bimSpatialToIfc4x3,ifc4x3ToBimSpatial} from '../bim.ts';
import {fem3dToIfc4x3,ifc4x3ToFem3d} from '../fem.ts';
import {extractIfc4x3Triangles,type IfcExchangeOutcome} from '../geometry.ts';
import {ifcReal,parseIfc4x3Step} from '../step.ts';
import type {IfcSnapshot} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import type {IfcIoControl} from '../control.ts';

function withNestedProperty(snapshot:IfcSnapshot):IfcSnapshot {
 const id=snapshot.entities.reduce((maximum,entity)=>entity.id>maximum?entity.id:maximum,0n)+1n;
 snapshot.entities.push({id,name:'IFCPROPERTYLISTVALUE',args:[{kind:'string',value:'Capture depth'},{kind:'unset'},{kind:'aggregate',value:[{kind:'typedValue',value:{name:'IFCLABEL',items:[{kind:'string',value:'entry'}]}}]},{kind:'unset'}],complex:[]});
 return snapshot;
}

/** 🎯️ Capture a genuine supported source before attaching tested projection controls. */
export async function ifcExchangeSourceFixture(id:string):Promise<{snapshot:IfcSnapshot;project:(source:IfcSnapshot,control?:IfcIoControl)=>Promise<IfcExchangeOutcome<unknown>>}> {
 if(id==='ifc-to-bim'){
  const result=await bimSpatialToIfc4x3({schema:'s.bim.model@1',project:{name:'Source',description:'',author:'',organization:'',phase_names:[]},sites:{site:{name:'Site',latitude:47.5,longitude:8.5,elevation:400,true_north:0,boundary:[]}}});
  if(!result.artifact)throw new Error('Valid BIM source was refused');
  return{snapshot:withNestedProperty(result.artifact),project:ifc4x3ToBimSpatial};
 }
 if(id==='ifc-to-fem'){
  const w=(n:number)=>{const value=ifcReal(n);if(value.kind!=='real')throw Error('real');return value.value;};
  const zero=w(0),result=await fem3dToIfc4x3({nodes:[{id:'a',x:zero,y:zero,z:zero},{id:'b',x:w(2),y:zero,z:zero}],elements:[{kind:'frame',id:'frame',start:'a',end:'b',materialId:'steel',sectionId:'section',roll:zero}],materials:[{id:'steel',name:'Steel',e:w(2e11),g:w(8e10),nu:w(.25),rho:w(7850)}],sections:[{id:'section',name:'Section',area:w(.01),iy:w(1e-5),iz:w(2e-5),j:w(3e-5)}],solids:[],supports:[],loadCases:[],combinations:[],analysis:{modalCount:0n,bucklingCount:0n,deformationScale:w(1)}});
  if(!result.artifact)throw new Error('Valid FEM source was refused');
  return{snapshot:withNestedProperty(result.artifact),project:ifc4x3ToFem3d};
 }
 if(id==='ifc-triangles')return{snapshot:await parseIfc4x3Step(await readFile(new URL('triangulated.ifc',import.meta.url),'utf8')),project:extractIfc4x3Triangles};
 throw new Error(`Unknown IFC source endpoint ${id}`);
}
