/** 🧱️ Exact CSG primitive roots share current source owners and finite primitive geometry. */
import type {IfcEntity} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {IfcStepError,type IfcIoSteps} from '../control.ts';
import {ifcNumber} from '../step.ts';
import {ifcIdentity,ifcMultiply,ifcTransform,type IfcGeometryGraph,type IfcMatrix,type IfcTriangleMesh} from '../geometry.ts';
import {ifcMatrixOrientation} from '../orientation.ts';
import {blockPoint,BLOCK_TRIANGLES} from '../../../../../../📐️geometry/🧱️primitive/🟦️.ts';
import {ifcGeometryEntity as exact,ifcGeometryFieldSteps} from './source.ts';
import {ifcArrayMeshWriterFactory,type IfcMeshWriterFactory} from './mesh-writer.ts';
import {ifcCircularSolidWriteSteps} from './circular-solid.ts';
import type {IfcCurveSampling} from './control.ts';

/** 📐️ Required primitive Position precedes represented frames; outward indices reach the central reflection owner. */
export function* ifcCsgSolidSteps(graph:IfcGeometryGraph,solid:IfcEntity,world:IfcMatrix,scale:number,policy:IfcCurveSampling,maxValues:number):IfcIoSteps<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 yield;exact(solid,['IFCCSGSOLID']);const primitive=graph.ref(yield* ifcGeometryFieldSteps(graph,solid,'IFCCSGSOLID','TreeRootExpression'));
 return yield* ifcCsgPrimitiveSteps(graph,primitive,world,scale,policy,maxValues);
}
/** 🧱️ Borrow the actual declared primitive and current graph for both represented roots and Boolean operands. */
export function* ifcCsgPrimitiveSteps(graph:IfcGeometryGraph,primitive:IfcEntity,world:IfcMatrix,scale:number,policy:IfcCurveSampling,maxValues:number):IfcIoSteps<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 return yield* ifcCsgPrimitiveWriteSteps(graph,primitive,world,scale,policy,ifcArrayMeshWriterFactory(maxValues));
}
/** 🧱️ Reserve the endpoint before any primitive coordinate or triangle backing is born. */
export function* ifcCsgPrimitiveWriteSteps<T>(graph:IfcGeometryGraph,primitive:IfcEntity,world:IfcMatrix,scale:number,policy:IfcCurveSampling,make:IfcMeshWriterFactory<T>):IfcIoSteps<T>{
 exact(primitive,['IFCBLOCK','IFCRIGHTCIRCULARCYLINDER','IFCRIGHTCIRCULARCONE']);const field=(entity:IfcEntity,owner:string,name:string)=>ifcGeometryFieldSteps(graph,entity,owner,name);
 const positionValue=yield* field(primitive,'IFCCSGPRIMITIVE3D','Position'),position=graph.ref(positionValue);exact(position,['IFCAXIS2PLACEMENT3D']);const location=graph.ref(yield* field(position,'IFCPLACEMENT','Location'));exact(location,['IFCCARTESIANPOINT']);for(const name of ['Axis','RefDirection']){yield;const value=yield* field(position,'IFCAXIS2PLACEMENT3D',name);if(value.kind!=='unset')exact(graph.ref(value),['IFCDIRECTION']);}
 const frame=ifcMultiply(world,graph.axis(positionValue));
 if(primitive.name!=='IFCBLOCK'){
  const height=ifcNumber(yield* field(primitive,primitive.name,'Height')),radius=ifcNumber(yield* field(primitive,primitive.name,primitive.name==='IFCRIGHTCIRCULARCONE'?'BottomRadius':'Radius'));
  if(!(scale>0)||!Number.isFinite(scale)||[height,radius].some(value=>!(value>0)||!Number.isFinite(value*scale))||!ifcMatrixOrientation(frame))throw new IfcStepError('geometry.csg','Positive finite circular primitive measures and nonsingular finite placement required');
  if(height*scale<2**-1022||radius*scale<2**-1022)throw new IfcStepError('geometry.precision','Circular primitive dimensions require normal binary64 SI measures');
  return yield* ifcCircularSolidWriteSteps({shape:primitive.name==='IFCRIGHTCIRCULARCONE'?'cone':'cylinder',radius,displacement:[0,0,height],profile:ifcIdentity(),frame,scale},policy,make);
 }
 const block=primitive,lengths:[number,number,number]=[ifcNumber(yield* field(block,'IFCBLOCK','XLength')),ifcNumber(yield* field(block,'IFCBLOCK','YLength')),ifcNumber(yield* field(block,'IFCBLOCK','ZLength'))];
 if(!(scale>0)||!Number.isFinite(scale)||lengths.some(n=>!(n>0)||!Number.isFinite(n*scale)||n*scale<2**-1022)||!ifcMatrixOrientation(frame))throw new IfcStepError('geometry.csg','Positive finite normal SI block lengths and nonsingular finite placement required');
 const writer=make(8,12),seen=new Set<string>();let failed=false;
 try{
 for(let at=0;at<8;at++){yield;const point=ifcTransform(frame,blockPoint(lengths,at));for(let axis=0;axis<3;axis++){point[axis]=point[axis]!*scale;if(!Number.isFinite(point[axis]))throw new IfcStepError('geometry.csg','Block coordinates exceed finite SI range');}const key=point.join(',');if(seen.has(key))throw new IfcStepError('geometry.csg','Block placement collapses distinct geometric corners');seen.add(key);writer.point(at,point[0],point[1],point[2]);}
 for(let at=0;at<BLOCK_TRIANGLES.length;at++){yield;const triangle=BLOCK_TRIANGLES[at]!;writer.triangle(at,triangle[0],triangle[1],triangle[2]);}yield;return writer.finish();
 }catch(error){failed=true;throw error;}finally{try{writer.close();}catch(error){if(!failed)throw error;}}
}
