/** 🧩️ Explicit language-neutral polygon inputs produce genuine current source declarations. */
import type {IfcEntity,IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {createIfc4x3ExchangeGraph} from '../gltf.ts';
import {ifcReal} from '../step.ts';
export interface IfcPolygonalCase {id:string;coordinates:number[][];faces:{outer:number[];holes:number[][]}[];pnIndex:number[]|null;closed:boolean|null;expected:string;bounds:number[][];volume:number|null;damage:string;qualification:string}
/** 🏗️ Preserve source indices, coordinate aliases, optional closure and exact face roles. */
export function ifcPolygonalFixture(row:IfcPolygonalCase){
 const graph=createIfc4x3ExchangeGraph(row.id),unset:IfcValue={kind:'unset'},a=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value}),s=(value:string):IfcValue=>({kind:'string',value}),ref=(entity:IfcEntity):IfcValue=>({kind:'reference',value:entity.id}),index=(n:number):IfcValue=>Number.isSafeInteger(n)?{kind:'integer',value:BigInt(n)}:ifcReal(n),put=(name:string,args:IfcValue[])=>{graph.add(name,args);return graph.snapshot.entities.at(-1)!;};
 const wrongCoordinates=row.damage==='wrong-coordinate-role',coordinates=put(wrongCoordinates?'IFCCARTESIANPOINTLIST2D':'IFCCARTESIANPOINTLIST3D',[a(row.coordinates.map(p=>a((wrongCoordinates?p.slice(0,2):p).map(ifcReal)))),unset]),faces=row.faces.map(face=>row.damage==='wrong-face-role'?put('IFCPOLYLINE',[a(row.coordinates.slice(0,2).map(p=>ref(put('IFCCARTESIANPOINT',[a(p.map(ifcReal))]))))]):put(face.holes.length?'IFCINDEXEDPOLYGONALFACEWITHVOIDS':'IFCINDEXEDPOLYGONALFACE',[a(face.outer.map(index)),...(face.holes.length?[a(face.holes.map(r=>a(r.map(index))))]:[])])),members=faces.map(ref);
 if(row.damage==='duplicate-face-reference')members.push(ref(faces[0]!));
 const faceSet=put('IFCPOLYGONALFACESET',[ref(coordinates),row.closed===null?unset:{kind:'enum',value:row.closed?'T':'F'},a(members),row.pnIndex===null?unset:a(row.pnIndex.map(index))]),representation=put('IFCSHAPEREPRESENTATION',[graph.context,s('Body'),s('Tessellation'),a([ref(faceSet)])]),shape=put('IFCPRODUCTDEFINITIONSHAPE',[unset,unset,a([ref(representation)])]),placement=put('IFCLOCALPLACEMENT',[unset,graph.worldCoordinateSystem]),product=put('IFCBUILDINGELEMENTPROXY',[s('0000000000000000000002'),unset,s(row.id),unset,unset,ref(placement),ref(shape),unset,unset]);
 return{...graph,coordinates,faces,faceSet,representation,product};
}
