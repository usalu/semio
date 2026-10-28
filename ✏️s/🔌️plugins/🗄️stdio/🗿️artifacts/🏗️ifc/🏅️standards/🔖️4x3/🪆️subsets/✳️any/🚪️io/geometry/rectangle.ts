/** 🧊️ Exact rectangular area sweeps produce outward triangles in the admitted product frame. */
import type {IfcEntity,IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {ifcNumber} from '../step.ts';
import {IfcStepError,type IfcIoSteps} from '../control.ts';
import {ifcIdentity,ifcTransform,ifcMultiply,ifcAggregate,ifcNormalizeDirection,type IfcGeometryGraph,type IfcMatrix,type IfcTriangleMesh} from '../geometry.ts';
import {ifcArrayMeshWriterFactory,type IfcMeshWriterFactory} from './mesh-writer.ts';
import {ifcGeometryEntity,ifcGeometryFieldSteps} from './source.ts';
import {rectanglePoint,BLOCK_TRIANGLES} from '../../../../../../📐️geometry/🧱️primitive/🟦️.ts';

/** 📐️ Profile placement affects the base only; normalized extrusion stays in solid coordinates. */
export function* ifcRectangleExtrusionSteps(graph:IfcGeometryGraph,solid:IfcEntity,world:IfcMatrix,scale:number,maxValues:number):IfcIoSteps<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 return yield* ifcRectangleExtrusionWriteSteps(graph,solid,world,scale,ifcArrayMeshWriterFactory(maxValues));
}
/** 🧊️ The rectangular source owns no second positions or index array. */
export function* ifcRectangleExtrusionWriteSteps<T>(graph:IfcGeometryGraph,solid:IfcEntity,world:IfcMatrix,scale:number,make:IfcMeshWriterFactory<T>):IfcIoSteps<T>{
 yield;ifcGeometryEntity(solid,['IFCEXTRUDEDAREASOLID']);
 const profile=graph.ref(yield* ifcGeometryFieldSteps(graph,solid,'IFCSWEPTAREASOLID','SweptArea'));ifcGeometryEntity(profile,['IFCRECTANGLEPROFILEDEF']);
 const profileType=yield* ifcGeometryFieldSteps(graph,profile,'IFCPROFILEDEF','ProfileType');
 if(profileType.kind!=='enum'||profileType.value!=='AREA')throw new IfcStepError('geometry.profile','An exact AREA profile is required');
 const position=(value:IfcValue,name:string):IfcMatrix=>{if(value.kind==='unset')return ifcIdentity();if(graph.ref(value).name!==name)throw new IfcStepError('geometry.placement','Sweep placement must match its declared dimension');return graph.axis(value);};
 const profileFrame=position(yield* ifcGeometryFieldSteps(graph,profile,'IFCPARAMETERIZEDPROFILEDEF','Position'),'IFCAXIS2PLACEMENT2D'),solidFrame=position(yield* ifcGeometryFieldSteps(graph,solid,'IFCSWEPTAREASOLID','Position'),'IFCAXIS2PLACEMENT3D'),frame=ifcMultiply(world,solidFrame);
 const directionValue=yield* ifcGeometryFieldSteps(graph,solid,'IFCEXTRUDEDAREASOLID','ExtrudedDirection'),direction=graph.ref(directionValue);ifcGeometryEntity(direction,['IFCDIRECTION']);
 if(ifcAggregate(yield* ifcGeometryFieldSteps(graph,direction,'IFCDIRECTION','DirectionRatios')).length!==3)throw new IfcStepError('geometry.direction','An explicit three-dimensional extrusion direction is required');
 const vector=ifcNormalizeDirection(graph.vector(directionValue)),x=ifcNumber(yield* ifcGeometryFieldSteps(graph,profile,'IFCRECTANGLEPROFILEDEF','XDim')),y=ifcNumber(yield* ifcGeometryFieldSteps(graph,profile,'IFCRECTANGLEPROFILEDEF','YDim')),depth=ifcNumber(yield* ifcGeometryFieldSteps(graph,solid,'IFCEXTRUDEDAREASOLID','Depth'));
 if(x<=0||y<=0||depth<=0||vector[2]===0||!Number.isFinite(scale)||scale<=0)throw new IfcStepError('geometry.extrusion','Positive rectangular dimensions and depth, transverse direction and finite SI scale required');
 const writer=make(8,12);let failed=false;
 try{
 for(let layer=0;layer<2;layer++)for(let corner=0;corner<4;corner++){yield;const base=ifcTransform(profileFrame,rectanglePoint(x,y,corner)),local=base.map((n,k)=>n+layer*depth*vector[k]!) as [number,number,number],point=ifcTransform(frame,local).map(n=>n*scale) as [number,number,number];if(point.some(n=>!Number.isFinite(n)))throw new IfcStepError('geometry.range','Sweep coordinates exceed finite SI range');writer.point(layer*4+corner,point[0],point[1],point[2]);}
 for(let at=0;at<BLOCK_TRIANGLES.length;at++){yield;const face=BLOCK_TRIANGLES[at]!;writer.triangle(at,face[0],face[vector[2]>0?1:2],face[vector[2]>0?2:1]);}
 return writer.finish();
 }catch(error){failed=true;throw error;}finally{try{writer.close();}catch(error){if(!failed)throw error;}}
}
