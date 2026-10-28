/** 🔲️ Exact rounded AREA declarations share the owned convex sweep without fabricated source entities. */
import type{IfcEntity,IfcValue}from'../../🧬️schema/📸️snapshot/🟦️.ts';
import{ifcNumber}from'../step.ts';
import{IfcStepError,type IfcIoSteps}from'../control.ts';
import{ifcIdentity,ifcMultiply,ifcAggregate,ifcNormalizeDirection,type IfcGeometryGraph,type IfcMatrix,type IfcTriangleMesh}from'../geometry.ts';
import{ifcCircleSegments,type IfcCurveSampling}from'./control.ts';
import{circleBasisRadius}from'../../../../../../📐️geometry/⭕️circle/🟦️.ts';
import{ifcGeometryEntity,ifcGeometryFieldSteps}from'./source.ts';
import{ifcArrayMeshWriterFactory,type IfcMeshWriterFactory}from'./mesh-writer.ts';
import{ifcConvexSolidWriteSteps}from'./convex-solid.ts';
/** 📏️ Admit the entire boundary against the original segment cap, including nonzero straight joins. */
export function* ifcRoundedRectangleExtrusionSteps(graph:IfcGeometryGraph,solid:IfcEntity,world:IfcMatrix,scale:number,policy:IfcCurveSampling,maxValues:number):IfcIoSteps<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 return yield* ifcRoundedRectangleExtrusionWriteSteps(graph,solid,world,scale,policy,ifcArrayMeshWriterFactory(maxValues));
}
/** 🔲️ Keep original rounded source admission while streaming directly into its endpoint. */
export function* ifcRoundedRectangleExtrusionWriteSteps<T>(graph:IfcGeometryGraph,solid:IfcEntity,world:IfcMatrix,scale:number,policy:IfcCurveSampling,make:IfcMeshWriterFactory<T>):IfcIoSteps<T>{
 ifcGeometryEntity(solid,['IFCEXTRUDEDAREASOLID']);
 const profile=graph.ref(yield* ifcGeometryFieldSteps(graph,solid,'IFCSWEPTAREASOLID','SweptArea'));ifcGeometryEntity(profile,['IFCROUNDEDRECTANGLEPROFILEDEF']);
 const type=yield* ifcGeometryFieldSteps(graph,profile,'IFCPROFILEDEF','ProfileType');
 if(type.kind!=='enum'||type.value!=='AREA')throw new IfcStepError('geometry.profile','A rounded rectangle AREA profile is required');
 const position=(value:IfcValue,name:string):IfcMatrix=>{if(value.kind==='unset')return ifcIdentity();if(graph.ref(value).name!==name)throw new IfcStepError('geometry.placement','Sweep placement must match its declared dimension');return graph.axis(value);};
 const profileFrame=position(yield* ifcGeometryFieldSteps(graph,profile,'IFCPARAMETERIZEDPROFILEDEF','Position'),'IFCAXIS2PLACEMENT2D'),solidFrame=position(yield* ifcGeometryFieldSteps(graph,solid,'IFCSWEPTAREASOLID','Position'),'IFCAXIS2PLACEMENT3D'),frame=ifcMultiply(world,solidFrame);
 const directionValue=yield* ifcGeometryFieldSteps(graph,solid,'IFCEXTRUDEDAREASOLID','ExtrudedDirection'),direction=graph.ref(directionValue);ifcGeometryEntity(direction,['IFCDIRECTION']);
 if(ifcAggregate(direction.args[0]).length!==3)throw new IfcStepError('geometry.direction','An explicit three-dimensional extrusion direction is required');
 const vector=ifcNormalizeDirection(graph.vector(directionValue)),width=ifcNumber(yield* ifcGeometryFieldSteps(graph,profile,'IFCRECTANGLEPROFILEDEF','XDim')),height=ifcNumber(yield* ifcGeometryFieldSteps(graph,profile,'IFCRECTANGLEPROFILEDEF','YDim')),radius=ifcNumber(yield* ifcGeometryFieldSteps(graph,profile,'IFCROUNDEDRECTANGLEPROFILEDEF','RoundingRadius')),depth=ifcNumber(yield* ifcGeometryFieldSteps(graph,solid,'IFCEXTRUDEDAREASOLID','Depth'));
 if(!Number.isFinite(scale)||scale<=0||[width,height,radius,depth].some(value=>!Number.isFinite(value*scale)||value*scale<=0)||radius>width/2||radius>height/2||vector[2]===0)throw new IfcStepError('geometry.extrusion','Positive represented rounded dimensions, bounded radius and transverse extrusion direction are required');
 if([width,height,radius,depth,Math.abs(depth*vector[2])].some(value=>value*scale<2**-1022))throw new IfcStepError('geometry.precision','Rounded dimensions and transverse height require normal binary64 SI measures');
 const basis=ifcMultiply(frame,profileFrame),worldRadius=circleBasisRadius(radius*scale,[basis[0],basis[1],basis[2]],[basis[4],basis[5],basis[6]]);
 if(worldRadius===undefined)throw new IfcStepError('geometry.range','Rounded arcs require a finite normal represented basis bound');
 const joins=(radius<width/2?2:0)+(radius<height/2?2:0),maximum=policy.maxSegments-joins;
 if(maximum<4)throw new IfcStepError('geometry.accuracy','The original segment grant cannot contain the rounded boundary');
 const chords=ifcCircleSegments(worldRadius,{...policy,maxSegments:maximum});
 return yield* ifcConvexSolidWriteSteps({shape:'cylinder',boundary:{kind:'rounded-rectangle',width,height,radius,chords},displacement:vector.map(value=>depth*value)as[number,number,number],profile:profileFrame,frame,scale},make);
}
