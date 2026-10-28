/** ⭕️ Ordered convex circle chords produce finite outward sweeps under original cooperative control. */
import type {IfcEntity,IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {ifcNumber} from '../step.ts';
import {IfcStepError,type IfcIoSteps} from '../control.ts';
import {ifcIdentity,ifcMultiply,ifcAggregate,ifcNormalizeDirection,type IfcGeometryGraph,type IfcMatrix,type IfcTriangleMesh} from '../geometry.ts';
import type {IfcCurveSampling} from './control.ts';
import {ifcArrayMeshWriterFactory,type IfcMeshWriterFactory} from './mesh-writer.ts';
import {ifcCircularSolidWriteSteps} from './circular-solid.ts';
import {ifcGeometryEntity,ifcGeometryFieldSteps} from './source.ts';
/** 📐️ The curve certificate covers ideal profile chords; transforms and binary64 output must also remain representable. */
export function* ifcCircleExtrusionSteps(graph:IfcGeometryGraph,solid:IfcEntity,world:IfcMatrix,scale:number,policy:IfcCurveSampling,maxValues:number):IfcIoSteps<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 return yield* ifcCircleExtrusionWriteSteps(graph,solid,world,scale,policy,ifcArrayMeshWriterFactory(maxValues));
}
/** ⭕️ Author source circle rows directly into the pre-reserved endpoint. */
export function* ifcCircleExtrusionWriteSteps<T>(graph:IfcGeometryGraph,solid:IfcEntity,world:IfcMatrix,scale:number,policy:IfcCurveSampling,make:IfcMeshWriterFactory<T>):IfcIoSteps<T>{
 yield;ifcGeometryEntity(solid,['IFCEXTRUDEDAREASOLID']);
 const profile=graph.ref(yield* ifcGeometryFieldSteps(graph,solid,'IFCSWEPTAREASOLID','SweptArea'));ifcGeometryEntity(profile,['IFCCIRCLEPROFILEDEF']);
 const profileType=yield* ifcGeometryFieldSteps(graph,profile,'IFCPROFILEDEF','ProfileType');
 if(profileType.kind!=='enum'||profileType.value!=='AREA')throw new IfcStepError('geometry.profile','An exact AREA profile is required');
 const position=(value:IfcValue,name:string):IfcMatrix=>{if(value.kind==='unset')return ifcIdentity();if(graph.ref(value).name!==name)throw new IfcStepError('geometry.placement','Sweep placement must match its declared dimension');return graph.axis(value);};
 const profileFrame=position(yield* ifcGeometryFieldSteps(graph,profile,'IFCPARAMETERIZEDPROFILEDEF','Position'),'IFCAXIS2PLACEMENT2D'),solidFrame=position(yield* ifcGeometryFieldSteps(graph,solid,'IFCSWEPTAREASOLID','Position'),'IFCAXIS2PLACEMENT3D'),frame=ifcMultiply(world,solidFrame);
 const directionValue=yield* ifcGeometryFieldSteps(graph,solid,'IFCEXTRUDEDAREASOLID','ExtrudedDirection'),direction=graph.ref(directionValue);ifcGeometryEntity(direction,['IFCDIRECTION']);
 if(ifcAggregate(yield* ifcGeometryFieldSteps(graph,direction,'IFCDIRECTION','DirectionRatios')).length!==3)throw new IfcStepError('geometry.direction','An explicit three-dimensional extrusion direction is required');
 const vector=ifcNormalizeDirection(graph.vector(directionValue)),radius=ifcNumber(yield* ifcGeometryFieldSteps(graph,profile,'IFCCIRCLEPROFILEDEF','Radius')),depth=ifcNumber(yield* ifcGeometryFieldSteps(graph,solid,'IFCEXTRUDEDAREASOLID','Depth')),radiusMetres=radius*scale;
 if(radius<=0||depth<=0||vector[2]===0||!Number.isFinite(scale)||scale<=0||!Number.isFinite(radiusMetres)||radiusMetres<=0||!Number.isFinite(depth*scale)||depth*scale<=0)throw new IfcStepError('geometry.extrusion','Positive representable circular radius and depth, transverse direction and finite SI scale required');
 if(radiusMetres<2**-1022||depth*scale<2**-1022||Math.abs(depth*vector[2]*scale)<2**-1022)throw new IfcStepError('geometry.precision','Circle radius, depth and transverse height require normal binary64 SI measures');
 return yield* ifcCircularSolidWriteSteps({shape:'cylinder',radius,displacement:vector.map(value=>depth*value) as [number,number,number],profile:profileFrame,frame,scale},policy,make);
}
