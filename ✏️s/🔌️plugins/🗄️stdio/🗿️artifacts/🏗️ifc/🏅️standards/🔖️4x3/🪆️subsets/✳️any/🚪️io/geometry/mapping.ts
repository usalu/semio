/** 🗺️ Retained source frames expand mapped leaves without flattening or rebuilding graph indexes. */
import type {IfcEntity,IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {IfcStepError,type IfcIoSteps} from '../control.ts';
import {ifcNumber} from '../step.ts';
import {ifcAggregate,ifcMultiply,ifcNormalizeDirection,type IfcGeometryGraph,type IfcMatrix} from '../geometry.ts';
import {ifcMatrixOrientation} from '../orientation.ts';
import {ifcGeometryEntity as exact,ifcGeometryFieldSteps} from './source.ts';
type Vec=[number,number,number];
const dot=(a:Vec,b:Vec)=>a[0]*b[0]+a[1]*b[1]+a[2]*b[2];
function reject(reason:string):never{throw new IfcStepError('geometry.mapping',reason);}

/** 🧭️ Apply released IfcBaseAxis projections and optional positive scale declarations in three dimensions. */
function* operatorSteps(graph:IfcGeometryGraph,entity:IfcEntity):IfcIoSteps<IfcMatrix>{
 yield;exact(entity,['IFCCARTESIANTRANSFORMATIONOPERATOR3D','IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM']);const field=(owner:string,name:string)=>ifcGeometryFieldSteps(graph,entity,owner,name),owner='IFCCARTESIANTRANSFORMATIONOPERATOR';
 function* coordinate(value:IfcValue,name:'IFCCARTESIANPOINT'|'IFCDIRECTION'):IfcIoSteps<Vec>{yield;const row=graph.ref(value);exact(row,[name]);const items=ifcAggregate(yield* ifcGeometryFieldSteps(graph,row,name,name==='IFCCARTESIANPOINT'?'Coordinates':'DirectionRatios'));if(items.length!==3)reject('Mapping requires three-dimensional points and directions');const point:Vec=[ifcNumber(items[0]),ifcNumber(items[1]),ifcNumber(items[2])];return name==='IFCDIRECTION'?ifcNormalizeDirection(point):point;}
 const axis1=yield* field(owner,'Axis1'),axis2=yield* field(owner,'Axis2'),axis3=yield* field('IFCCARTESIANTRANSFORMATIONOPERATOR3D','Axis3'),z:Vec=axis3.kind==='unset'?[0,0,1]:yield* coordinate(axis3,'IFCDIRECTION'),hx:Vec=axis1.kind==='unset'?(z[0]===1&&z[1]===0&&z[2]===0?[0,1,0]:[1,0,0]):yield* coordinate(axis1,'IFCDIRECTION'),hy:Vec=axis2.kind==='unset'?[0,1,0]:yield* coordinate(axis2,'IFCDIRECTION'),x=ifcNormalizeDirection(hx.map((n,k)=>n-dot(hx,z)*z[k]!) as Vec),y=ifcNormalizeDirection(hy.map((n,k)=>n-dot(hy,z)*z[k]!-dot(hy,x)*x[k]!) as Vec);
 if(Math.max(Math.abs(dot(x,y)),Math.abs(dot(x,z)),Math.abs(dot(y,z)))>128*Number.EPSILON)reject('Mapping axes require representable orthogonal projections');
 const scale=(value:IfcValue,fallback:number)=>{const result=value.kind==='unset'?fallback:ifcNumber(value);if(!(result>0)||!Number.isFinite(result))reject('Mapping scales must be finite and positive');return result;},s=scale(yield* field(owner,'Scale'),1),nonuniform=entity.name==='IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM',sy=nonuniform?scale(yield* field(entity.name,'Scale2'),s):s,sz=nonuniform?scale(yield* field(entity.name,'Scale3'),s):s,origin=yield* coordinate(yield* field(owner,'LocalOrigin'),'IFCCARTESIANPOINT'),matrix:IfcMatrix=[x[0]*s,x[1]*s,x[2]*s,0,y[0]*sy,y[1]*sy,y[2]*sy,0,z[0]*sz,z[1]*sz,z[2]*sz,0,...origin,1];if(!ifcMatrixOrientation(matrix))reject('A finite nonsingular represented mapping is required');return matrix;
}
interface Frame{items:readonly IfcValue[];index:number;world:IfcMatrix;mapping?:bigint}
/** 🧵️ One controlled leaf request preserves active-map paths, source order and repeated shared definitions. */
export class IfcRepresentationCursor{
 readonly #frames:Frame[];readonly #active=new Set<bigint>();#work=0;
 constructor(private readonly graph:IfcGeometryGraph,items:readonly IfcValue[],world:IfcMatrix,private readonly depth:number,private readonly maxItems:number){this.#frames=[{items,index:0,world}];}
 *nextSteps():IfcIoSteps<{item:IfcEntity;world:IfcMatrix}|undefined>{
  while(this.#frames.length){
   yield;const frame=this.#frames.at(-1)!;if(frame.index===frame.items.length){this.#frames.pop();if(frame.mapping!==undefined)this.#active.delete(frame.mapping);continue;}if(++this.#work>this.maxItems)throw new IfcStepError('geometry.limit','Mapped source traversal exceeds the original value grant');const item=this.graph.ref(frame.items[frame.index++]);if(item.name!=='IFCMAPPEDITEM')return{item,world:frame.world};
   exact(item,['IFCMAPPEDITEM']);if(this.#active.has(item.id)||this.#frames.length>=this.depth)reject('Mapped representations require an acyclic path within the original depth grant');const source=this.graph.ref(yield* ifcGeometryFieldSteps(this.graph,item,'IFCMAPPEDITEM','MappingSource'));exact(source,['IFCREPRESENTATIONMAP']);const origin=yield* ifcGeometryFieldSteps(this.graph,source,'IFCREPRESENTATIONMAP','MappingOrigin');exact(this.graph.ref(origin),['IFCAXIS2PLACEMENT3D']);const target=this.graph.ref(yield* ifcGeometryFieldSteps(this.graph,item,'IFCMAPPEDITEM','MappingTarget')),matrix=ifcMultiply(ifcMultiply(frame.world,yield* operatorSteps(this.graph,target)),this.graph.axis(origin));if(!ifcMatrixOrientation(matrix))reject('Composed mapping must remain finite and nonsingular');
   const representation=this.graph.ref(yield* ifcGeometryFieldSteps(this.graph,source,'IFCREPRESENTATIONMAP','MappedRepresentation'));exact(representation,['IFCSHAPEREPRESENTATION']);const items=ifcAggregate(yield* ifcGeometryFieldSteps(this.graph,representation,'IFCREPRESENTATION','Items'));if(!items.length)reject('Mapped representations require nonempty Items');this.#active.add(item.id);this.#frames.push({items,index:0,world:matrix,mapping:item.id});
  }
  return undefined;
 }
}
