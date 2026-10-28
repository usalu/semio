/** 🧱️ Shared exact planar face work retains private geometric and original source identities. */
import type {IfcEntity,IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {triangulateRingsSteps,type RingPoint} from '../../../../../../📐️geometry/🕳️rings/🟦️.ts';
import type {SolidTriangle} from '../../../../../../📐️geometry/🧊️boolean/🟦️.ts';
import {IfcStepError,type IfcIoSteps} from '../control.ts';
import {ifcNumber} from '../step.ts';
import {ifcAggregate,ifcTransform,type IfcGeometryGraph,type IfcMatrix} from '../geometry.ts';
import {ifcGeometryEntity as exact,ifcGeometryFieldSteps} from './source.ts';
function reject(reason:string):never{throw new IfcStepError('geometry.faces',reason);}

/** 📐️ Bounded point/ring work is shared by solid shells and planar surface members. */
export class IfcPlanarFaceWork{
 readonly positions:RingPoint[]=[];
 readonly low:RingPoint=[Infinity,Infinity,Infinity];
 readonly high:RingPoint=[-Infinity,-Infinity,-Infinity];
 readonly #pointIds=new Map<bigint,number>();
 readonly #geometricPoints=new Map<string,number>();
 readonly #sourcePointIds:bigint[]=[];
 #values=0;
 constructor(readonly graph:IfcGeometryGraph,readonly maxValues:number){if(!Number.isSafeInteger(maxValues)||maxValues<0)reject('An original finite value grant is required');}
 private reserve(count:number):void{if(!Number.isSafeInteger(count)||count<0||count>this.maxValues-this.#values)throw new IfcStepError('geometry.limit','Planar coordinates and triangles exceed the original value grant');this.#values+=count;}
 private field(entity:IfcEntity,owner:string,name:string){return ifcGeometryFieldSteps(this.graph,entity,owner,name);}
 private *point(value:IfcValue):IfcIoSteps<number>{yield;const entity=this.graph.ref(value);exact(entity,['IFCCARTESIANPOINT']);const known=this.#pointIds.get(entity.id);if(known!==undefined)return known;const coordinates=ifcAggregate(yield* this.field(entity,'IFCCARTESIANPOINT','Coordinates'));if(coordinates.length!==3)reject('Planar points require exactly three finite coordinates');const xyz:RingPoint=[ifcNumber(coordinates[0]),ifcNumber(coordinates[1]),ifcNumber(coordinates[2])],key=xyz.join(','),alias=this.#geometricPoints.get(key);if(alias!==undefined){this.#pointIds.set(entity.id,alias);return alias;}this.reserve(3);const id=this.positions.length;this.#pointIds.set(entity.id,id);this.#geometricPoints.set(key,id);this.#sourcePointIds.push(entity.id);this.positions.push(xyz);for(let axis=0;axis<3;axis++){this.low[axis]=Math.min(this.low[axis]!,xyz[axis]!);this.high[axis]=Math.max(this.high[axis]!,xyz[axis]!);}return id;}
 /** 🕳️ Preserve declared bound direction and every geometric boundary through one ring triangulator. */
 *faceSteps(face:IfcEntity):IfcIoSteps<{rings:number[][];triangles:SolidTriangle[]}>{
  yield;exact(face,['IFCFACE']);const bounds=ifcAggregate(yield* this.field(face,'IFCFACE','Bounds')),boundIds=new Set<bigint>(),rings:number[][]=[],boxes:{low:RingPoint;high:RingPoint}[]=[],allLow:RingPoint=[Infinity,Infinity,Infinity],allHigh:RingPoint=[-Infinity,-Infinity,-Infinity];let outerAt=-1;if(!bounds.length)reject('Every face requires bounds');
  for(const value of bounds){
   yield;const bound=this.graph.ref(value);exact(bound,['IFCFACEBOUND','IFCFACEOUTERBOUND']);if(boundIds.has(bound.id))reject('A face bound may occur only once');boundIds.add(bound.id);if(bound.name==='IFCFACEOUTERBOUND'){if(outerAt!==-1)reject('A face has at most one explicit outer bound');outerAt=rings.length;}
   const direction=yield* this.field(bound,'IFCFACEBOUND','Orientation');if(direction.kind!=='enum'||!['T','F'].includes(direction.value))reject('Bound orientation requires an explicit BOOLEAN');
   const loop=this.graph.ref(yield* this.field(bound,'IFCFACEBOUND','Bound'));exact(loop,['IFCPOLYLOOP']);const polygon=ifcAggregate(yield* this.field(loop,'IFCPOLYLOOP','Polygon')),ring:number[]=[],seen=new Set<number>(),box={low:[Infinity,Infinity,Infinity] as RingPoint,high:[-Infinity,-Infinity,-Infinity] as RingPoint};if(polygon.length<3)reject('A polyloop requires three distinct points');
   for(let at=0;at<polygon.length;at++){yield;const id=yield* this.point(polygon[direction.value==='T'?at:polygon.length-at-1]!);if(seen.has(id))reject('A polyloop point may occur only once');seen.add(id);ring.push(id);const xyz=this.positions[id]!;for(let axis=0;axis<3;axis++){box.low[axis]=Math.min(box.low[axis]!,xyz[axis]!);box.high[axis]=Math.max(box.high[axis]!,xyz[axis]!);allLow[axis]=Math.min(allLow[axis]!,xyz[axis]!);allHigh[axis]=Math.max(allHigh[axis]!,xyz[axis]!);}}
   rings.push(ring);boxes.push(box);
  }
  if(outerAt===-1)for(let at=0;at<boxes.length;at++){yield;const box=boxes[at]!;if([0,1,2].every(axis=>box.low[axis]===allLow[axis]&&box.high[axis]===allHigh[axis])){if(outerAt!==-1)reject('Unmarked face requires a unique geometric outer bound');outerAt=at;}}
  if(outerAt===-1)reject('A geometric outer bound is required');if(outerAt!==0)[rings[0],rings[outerAt]]=[rings[outerAt]!,rings[0]!];let expected=-2+2*(rings.length-1);for(const ring of rings){yield;expected+=ring.length;}this.reserve(expected*3);const triangles=yield* triangulateRingsSteps(this.positions,rings);if(triangles.length!==expected)reject('Planar triangulation must retain every declared boundary edge');return{rings,triangles};
 }
 /** 📏️ Normalize only private work coordinates before the original solid predicates. */
 *normalizeSteps():IfcIoSteps<void>{yield;const extent=Math.max(this.high[0]-this.low[0],this.high[1]-this.low[1],this.high[2]-this.low[2]);if(!(extent>0)||!Number.isFinite(extent))reject('Finite nondegenerate planar coordinate range required');for(let id=0;id<this.positions.length;id++){yield;const p=this.positions[id]!;this.positions[id]=[(p[0]-this.low[0])/extent,(p[1]-this.low[1])/extent,(p[2]-this.low[2])/extent];}}
 /** 🗺️ Reread original representative coordinates before finite SI/world emission. */
 *restoreSteps(world:IfcMatrix,scale:number):IfcIoSteps<void>{yield;for(let index=0;index<this.#sourcePointIds.length;index++){yield;const entity=this.graph.lookup(this.#sourcePointIds[index]!);if(!entity)reject('Original source point required');const coordinates=ifcAggregate(yield* this.field(entity,'IFCCARTESIANPOINT','Coordinates')),local:RingPoint=[ifcNumber(coordinates[0]),ifcNumber(coordinates[1]),ifcNumber(coordinates[2])],point=ifcTransform(world,local);for(let axis=0;axis<3;axis++){point[axis]=point[axis]!*scale;if(!Number.isFinite(point[axis]))reject('Planar coordinates exceed finite SI range');}this.positions[index]=point;}}
}
