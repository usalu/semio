/** 🛢️ Finite circular solids share ideal world-chord admission, ordered vertices and cooperative publication. */
import{IfcStepError,type IfcIoSteps}from'../control.ts';
import{ifcMultiply,type IfcMatrix,type IfcTriangleMesh}from'../geometry.ts';
import{ifcCircleSegments,type IfcCurveSampling}from'./control.ts';
import{circleBasisRadius}from'../../../../../../📐️geometry/⭕️circle/🟦️.ts';
import{ifcArrayMeshWriterFactory,type IfcMeshWriterFactory}from'./mesh-writer.ts';
import{ifcConvexSolidWriteSteps}from'./convex-solid.ts';
export interface IfcCircularSolid{shape:'cylinder'|'cone';radius:number;displacement:[number,number,number];profile:IfcMatrix;frame:IfcMatrix;scale:number}
/** ⭕️ Profile placement affects each base point; displacement remains in the enclosing solid coordinate system. */
export function* ifcCircularSolidSteps(source:IfcCircularSolid,policy:IfcCurveSampling,maxValues:number):IfcIoSteps<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 return yield* ifcCircularSolidWriteSteps(source,policy,ifcArrayMeshWriterFactory(maxValues));
}
/** ⭕️ Delegate the admitted circle boundary directly to its original endpoint. */
export function* ifcCircularSolidWriteSteps<T>(source:IfcCircularSolid,policy:IfcCurveSampling,make:IfcMeshWriterFactory<T>):IfcIoSteps<T>{
 const{shape,radius,displacement,profile,frame,scale}=source;
 if(shape!=='cylinder'&&shape!=='cone')throw new IfcStepError('geometry.csg','An explicit circular solid shape is required');
 const basis=ifcMultiply(frame,profile),worldRadius=circleBasisRadius(radius*scale,[basis[0],basis[1],basis[2]],[basis[4],basis[5],basis[6]]);
 if(worldRadius===undefined)throw new IfcStepError('geometry.range','Circle world radius requires a finite normal represented basis bound');
 const chords=ifcCircleSegments(worldRadius,policy);
 return yield* ifcConvexSolidWriteSteps({shape,boundary:{kind:'circle',radius,chords},displacement,profile,frame,scale},make);
}
