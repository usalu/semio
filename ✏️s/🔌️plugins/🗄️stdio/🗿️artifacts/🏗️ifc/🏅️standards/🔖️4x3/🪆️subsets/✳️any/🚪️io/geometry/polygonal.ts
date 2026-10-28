/** 🔷️ Indexed planar faces retain source identity while sharing exact geometric vertices. */
import type {IfcEntity,IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {triangulateRingsSteps,type RingPoint} from '../../../../../../📐️geometry/🕳️rings/🟦️.ts';
import {validateSolidMeshSteps,type SolidTriangle} from '../../../../../../📐️geometry/🧊️boolean/🟦️.ts';
import {IfcStepError,type IfcIoSteps} from '../control.ts';
import {ifcNumber} from '../step.ts';
import {ifcAggregate,ifcTransform,type IfcGeometryGraph,type IfcMatrix,type IfcTriangleMesh} from '../geometry.ts';
import {ifcGeometryEntity as exact,ifcGeometryFieldSteps} from './source.ts';
function reject(reason:string):never{throw new IfcStepError('geometry.polygonal',reason);}
const integer=(value:IfcValue|undefined,length:number):number=>{if(value?.kind!=='integer'||value.value<1n||value.value>BigInt(length))reject('A positive INTEGER index within its declared source roster is required');return Number(value.value)-1;};

/** 🕳️ Cooperative polygon rings, optional indirection and closed-solid validation precede publication. */
export function* ifcPolygonalFaceSetSteps(graph:IfcGeometryGraph,item:IfcEntity,world:IfcMatrix,scale:number,maxValues:number):IfcIoSteps<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 yield;exact(item,['IFCPOLYGONALFACESET']);if(!(scale>0)||!Number.isFinite(scale)||!Number.isSafeInteger(maxValues)||maxValues<0)reject('Finite positive SI scale and original value grant required');
 const field=(entity:IfcEntity,owner:string,name:string)=>ifcGeometryFieldSteps(graph,entity,owner,name),coordinates=graph.ref(yield* field(item,'IFCTESSELLATEDFACESET','Coordinates'));exact(coordinates,['IFCCARTESIANPOINTLIST3D']);const rows=ifcAggregate(yield* field(coordinates,'IFCCARTESIANPOINTLIST3D','CoordList'));if(!rows.length)reject('A nonempty coordinate roster is required');
 const xyz=(at:number):RingPoint=>{const row=ifcAggregate(rows[at]);if(row.length!==3)reject('Every coordinate row requires three finite values');return[ifcNumber(row[0]),ifcNumber(row[1]),ifcNumber(row[2])];};
 for(let at=0;at<rows.length;at++){yield;xyz(at);}
 const pnValue=yield* field(item,'IFCPOLYGONALFACESET','PnIndex'),pn=pnValue.kind==='unset'?undefined:ifcAggregate(pnValue);if(pn&&!pn.length)reject('Present PnIndex must be nonempty');if(pn)for(const value of pn){yield;integer(value,rows.length);}
 const closed=yield* field(item,'IFCPOLYGONALFACESET','Closed');if(closed.kind!=='unset'&&(closed.kind!=='enum'||!['T','F'].includes(closed.value)))reject('Closed requires BOOLEAN or unset');
 const positions:RingPoint[]=[],sourceRows:number[]=[],sourceVertices=new Map<number,number>(),geometricVertices=new Map<string,number>(),triangles:SolidTriangle[]=[],low:RingPoint=[Infinity,Infinity,Infinity],high:RingPoint=[-Infinity,-Infinity,-Infinity];let values=0;
 const grant=(count:number)=>{if(!Number.isSafeInteger(count)||count<0||count>maxValues-values)throw new IfcStepError('geometry.limit','Polygonal coordinates and triangles exceed the original value grant');values+=count;};
 function* vertex(value:IfcValue):IfcIoSteps<number>{yield;const raw=integer(value,pn?.length??rows.length),ordinal=pn?integer(pn[raw],rows.length):raw,known=sourceVertices.get(ordinal);if(known!==undefined)return known;const point=xyz(ordinal),key=point.join(','),alias=geometricVertices.get(key);if(alias!==undefined){sourceVertices.set(ordinal,alias);return alias;}grant(3);const id=positions.length;sourceVertices.set(ordinal,id);geometricVertices.set(key,id);positions.push(point);sourceRows.push(ordinal);for(let axis=0;axis<3;axis++){low[axis]=Math.min(low[axis]!,point[axis]!);high[axis]=Math.max(high[axis]!,point[axis]!);}return id;}
 function* ring(value:IfcValue,unique:boolean):IfcIoSteps<number[]>{yield;const source=ifcAggregate(value),indices:number[]=[],rawSeen=new Set<bigint>();if(source.length<3)reject('Every polygon ring requires at least three indices');for(const value of source){yield;if(unique){integer(value,pn?.length??rows.length);if(value.kind!=='integer'||rawSeen.has(value.value))reject('Inner ring source indices must be UNIQUE');rawSeen.add(value.value);}const id=yield* vertex(value);if(indices.at(-1)!==id)indices.push(id);}if(indices[0]===indices.at(-1))indices.pop();if(indices.length<3)reject('A ring requires at least three geometric vertices');const seen=new Set<number>();for(const id of indices){yield;if(seen.has(id))reject('Nonconsecutive geometric ring repetition is unsupported');seen.add(id);}return indices;}
 const faces=ifcAggregate(yield* field(item,'IFCPOLYGONALFACESET','Faces')),seenFaces=new Set<bigint>();if(!faces.length)reject('At least one indexed polygonal face is required');
 for(const value of faces){
  yield;const face=graph.ref(value);exact(face,['IFCINDEXEDPOLYGONALFACE','IFCINDEXEDPOLYGONALFACEWITHVOIDS']);if(seenFaces.has(face.id))reject('Faces requires UNIQUE source references');seenFaces.add(face.id);
  const rings=[yield* ring(yield* field(face,'IFCINDEXEDPOLYGONALFACE','CoordIndex'),false)];if(face.name==='IFCINDEXEDPOLYGONALFACEWITHVOIDS'){const holes=ifcAggregate(yield* field(face,'IFCINDEXEDPOLYGONALFACEWITHVOIDS','InnerCoordIndices'));if(!holes.length)reject('A face with voids requires inner rings');for(const hole of holes){yield;rings.push(yield* ring(hole,true));}}
  let expected=-2+2*(rings.length-1);for(const ring of rings){yield;expected+=ring.length;}grant(expected*3);const result=yield* triangulateRingsSteps(positions,rings);if(result.length!==expected)reject('Triangulation must retain each geometric boundary edge');for(const triangle of result){yield;triangles.push(triangle);}
 }
 if(closed.kind==='enum'&&closed.value==='T'){
  const extent=Math.max(high[0]-low[0],high[1]-low[1],high[2]-low[2]);if(!(extent>0)||!Number.isFinite(extent))reject('Finite nondegenerate closed coordinates required');for(let id=0;id<positions.length;id++){yield;const p=positions[id]!;positions[id]=[(p[0]-low[0])/extent,(p[1]-low[1])/extent,(p[2]-low[2])/extent];}yield* validateSolidMeshSteps({vertices:positions,triangles});
 }
 for(let id=0;id<positions.length;id++){yield;const point=ifcTransform(world,xyz(sourceRows[id]!));for(let axis=0;axis<3;axis++){point[axis]=point[axis]!*scale;if(!Number.isFinite(point[axis]))reject('Polygonal coordinates exceed finite SI range');}positions[id]=point;}
 const indices:number[]=[];for(const triangle of triangles){yield;indices.push(triangle[0],triangle[1],triangle[2]);}yield;return{positions,indices};
}
