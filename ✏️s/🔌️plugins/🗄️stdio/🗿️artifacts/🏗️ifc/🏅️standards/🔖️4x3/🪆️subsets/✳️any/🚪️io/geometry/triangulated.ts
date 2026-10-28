/** 🔺️ Staged triangulation admission retains original index domains and optional closure claims. */
import type {IfcEntity,IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {validateTriangleMeshSteps,validateSolidMeshSteps,type SolidPoint,type SolidTriangle} from '../../../../../../📐️geometry/🧊️boolean/🟦️.ts';
import {IfcStepError,type IfcIoSteps} from '../control.ts';
import {ifcNumber} from '../step.ts';
import {ifcAggregate,ifcTransform,type IfcGeometryGraph,type IfcMatrix,type IfcTriangleMesh} from '../geometry.ts';
import {ifcGeometryEntity as exact,ifcGeometryFieldSteps} from './source.ts';
import {IfcNormalTransform} from '../orientation.ts';
function reject(message:string):never{throw new IfcStepError('geometry.triangulation',message);}
const integer=(value:IfcValue|undefined,length:number):number=>{if(value?.kind!=='integer'||value.value<1n||value.value>BigInt(length))reject('Positive INTEGER index within its original source roster required');return Number(value.value)-1;};
const xyz=(value:IfcValue|undefined):SolidPoint=>{const row=ifcAggregate(value);if(row.length!==3)reject('Every coordinate or normal row requires three finite REAL domain values');return[ifcNumber(row[0]),ifcNumber(row[1]),ifcNumber(row[2])];};

/** 🧵️ One captured source work item exposes meaningful cooperative stages before any mesh publication. */
export class IfcTriangulatedFaceWork{
 readonly positions:SolidPoint[]=[];readonly indices:number[]=[];readonly normalVectors:SolidPoint[]|undefined;readonly normalIndices:number[]|undefined;
 private constructor(private readonly rows:IfcValue[],private readonly triangles:IfcValue[],private readonly normals:IfcValue[]|undefined,private readonly pn:IfcValue[]|undefined,private readonly closed:boolean,private readonly world:IfcMatrix,private readonly scale:number){this.normalVectors=normals?[]:undefined;this.normalIndices=normals?[]:undefined;}
 /** 📍️ Resolve original owner-qualified fields and reserve the entire logical output before growth. */
 static *captureSteps(graph:IfcGeometryGraph,item:IfcEntity,world:IfcMatrix,scale:number,maxValues:number):IfcIoSteps<IfcTriangulatedFaceWork>{
  yield;exact(item,['IFCTRIANGULATEDFACESET']);if(!(scale>0)||!Number.isFinite(scale)||!Number.isSafeInteger(maxValues)||maxValues<0)reject('Finite positive SI scale and original value grant required');
  const field=(owner:string,name:string)=>ifcGeometryFieldSteps(graph,item,owner,name),coordinates=graph.ref(yield* field('IFCTESSELLATEDFACESET','Coordinates'));exact(coordinates,['IFCCARTESIANPOINTLIST3D']);const rows=ifcAggregate(yield* ifcGeometryFieldSteps(graph,coordinates,'IFCCARTESIANPOINTLIST3D','CoordList')),triangles=ifcAggregate(yield* field('IFCTRIANGULATEDFACESET','CoordIndex'));if(!rows.length||!triangles.length)reject('Nonempty coordinate and triangle rosters required');
  const optional=(value:IfcValue):IfcValue[]|undefined=>{if(value.kind==='unset')return;const rows=ifcAggregate(value);if(!rows.length)reject('Present optional rosters must be nonempty');return rows;},normals=optional(yield* field('IFCTRIANGULATEDFACESET','Normals')),pn=optional(yield* field('IFCTRIANGULATEDFACESET','PnIndex')),closed=yield* field('IFCTRIANGULATEDFACESET','Closed');if(closed.kind!=='unset'&&(closed.kind!=='enum'||!['T','F'].includes(closed.value)))reject('Closed requires BOOLEAN or unset');
  const values=rows.length*3+triangles.length*3+(normals?normals.length*3+triangles.length*3:0);if(!Number.isSafeInteger(values)||values>maxValues)throw new IfcStepError('geometry.limit','Triangulation coordinates, normals and indices exceed the original value grant');yield;return new IfcTriangulatedFaceWork(rows,triangles,normals,pn,closed.kind==='enum'&&closed.value==='T',world,scale);
 }
 /** 📐️ Validate every original coordinate row while retaining its source occurrence in world SI output. */
 *verticesSteps():IfcIoSteps<void>{yield;for(const row of this.rows){yield;const point=ifcTransform(this.world,xyz(row));for(let axis=0;axis<3;axis++){point[axis]=point[axis]!*this.scale;if(!Number.isFinite(point[axis]))reject('Coordinates exceed finite SI range');}this.positions.push(point);}}
 /** 🧭️ Optional normals retain their raw CoordIndex domain without invented length or unit constraints. */
 *normalsSteps():IfcIoSteps<void>{yield;if(!this.normals)return;const transform=IfcNormalTransform.capture(this.world);for(const normal of this.normals){yield;this.normalVectors!.push(transform.direction(xyz(normal)));}}
 /** 🔢️ Validate even unused original indirection members before any triangle reads them. */
 *pnSteps():IfcIoSteps<void>{yield;for(const value of this.pn??[]){yield;integer(value,this.rows.length);}}
 /** 🗂️ CoordIndex addresses normals directly and coordinates through optional PnIndex exactly once. */
 *trianglesSteps():IfcIoSteps<void>{yield;for(const value of this.triangles){yield;const row=ifcAggregate(value);if(row.length!==3)reject('Every triangle requires exactly three indices');for(const value of row){yield;const raw=integer(value,this.pn?.length??this.rows.length);if(this.normals&&raw>=this.normals.length)reject('Referenced raw normal index is outside the original normal roster');this.indices.push(this.pn?integer(this.pn[raw],this.rows.length):raw);this.normalIndices?.push(raw);}}}
 /** 🛡️ Open triangles receive input checks; explicit closure uses a private exact geometric identity domain. */
 *validationSteps():IfcIoSteps<void>{
  yield;const vertices:SolidPoint[]=[],triangles:SolidTriangle[]=[],keys=new Map<string,number>(),low:SolidPoint=[Infinity,Infinity,Infinity],high:SolidPoint=[-Infinity,-Infinity,-Infinity];
  for(let at=0;at<this.indices.length;at+=3){yield;const points:SolidPoint[]=[],ids:SolidTriangle=[0,0,0];for(let corner=0;corner<3;corner++){yield;const ordinal=this.indices[at+corner]!;points.push(this.positions[ordinal]!);if(this.closed){const point=xyz(this.rows[ordinal]!),key=point.join(',');let id=keys.get(key);if(id===undefined){id=vertices.length;keys.set(key,id);vertices.push(point);for(let axis=0;axis<3;axis++){low[axis]=Math.min(low[axis]!,point[axis]!);high[axis]=Math.max(high[axis]!,point[axis]!);}}ids[corner]=id;}}
   const origin=points[0]!,extent=Math.max(...points.flatMap(p=>p.map((n,axis)=>Math.abs(n-origin[axis]!))));if(!(extent>0)||!Number.isFinite(extent))reject('Finite nondegenerate projected triangle coordinates required');yield* validateTriangleMeshSteps({vertices:points.map(p=>p.map((n,axis)=>(n-origin[axis]!)/extent) as SolidPoint),triangles:[[0,1,2]]});if(this.closed)triangles.push(ids);
  }
  if(this.closed){const extent=Math.max(high[0]-low[0],high[1]-low[1],high[2]-low[2]);if(!(extent>0)||!Number.isFinite(extent))reject('Finite nondegenerate closed coordinates required');for(let at=0;at<vertices.length;at++){yield;const point=vertices[at]!;vertices[at]=[(point[0]-low[0])/extent,(point[1]-low[1])/extent,(point[2]-low[2])/extent];}yield* validateSolidMeshSteps({vertices,triangles});}
 }
 /** 📦️ Expose completed output only after the caller has drained every validation stage. */
 mesh():Pick<IfcTriangleMesh,'positions'|'indices'|'normals'|'normalIndices'>{return{positions:this.positions,indices:this.indices,...this.normalVectors?{normals:this.normalVectors,normalIndices:this.normalIndices!}:{}};}
}
