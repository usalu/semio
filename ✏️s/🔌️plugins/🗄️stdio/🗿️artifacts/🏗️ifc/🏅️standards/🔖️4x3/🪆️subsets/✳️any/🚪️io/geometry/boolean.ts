/** 🌳️ Actual IFC expressions share one cumulative owned evaluation across the whole extraction request. */
import type{IfcEntity}from'../../🧬️schema/📸️snapshot/🟦️.ts';
import{IfcStepError,runIfcIoSteps,type IfcIoControl,type IfcIoSteps}from'../control.ts';
import type{IfcGeometryGraph,IfcMatrix,IfcTriangleMesh}from'../geometry.ts';
import{ifcMatrixOrientation}from'../orientation.ts';
import{SolidBooleanEvaluation,type SolidBooleanOperation,type SolidOwnedMesh}from'../../../../../../📐️geometry/🧊️boolean/🟦️.ts';
import type{IfcBooleanEvaluationPolicy,IfcCurveSampling}from'./control.ts';
import{ifcGeometryEntity,ifcGeometryFieldSteps}from'./source.ts';
import type{IfcMeshWriterFactory}from'./mesh-writer.ts';
import{ifcCsgPrimitiveWriteSteps}from'./csg.ts';
import{ifcRectangleExtrusionWriteSteps}from'./rectangle.ts';
import{ifcCircleExtrusionWriteSteps}from'./circle.ts';
import{ifcRoundedRectangleExtrusionWriteSteps}from'./rounded.ts';
type Result={kind:'empty'}|{kind:'mesh';mesh:Pick<IfcTriangleMesh,'positions'|'indices'>};
interface Frame{entity:IfcEntity;stage:number;operation?:SolidBooleanOperation;first?:IfcEntity;second?:IfcEntity;left?:SolidOwnedMesh;}
/** 🧊️ Classification only selects the original evaluator; source authority remains in its declared field queries. */
export function ifcBooleanItem(graph:IfcGeometryGraph,item:IfcEntity):boolean{return item.name==='IFCBOOLEANRESULT'||item.name==='IFCBOOLEANCLIPPINGRESULT'||item.name==='IFCCSGSOLID'&&['IFCBOOLEANRESULT','IFCBOOLEANCLIPPINGRESULT'].includes(graph.ref(item.args[0]).name);}
/** 🧵️ Private request ownership keeps CPU, live backing and represented output as distinct ledgers. */
export class IfcBooleanWork{
 readonly #session:SolidBooleanEvaluation;readonly #control:IfcIoControl;readonly #sampling:IfcCurveSampling;readonly #depth:number;
 constructor(control:IfcIoControl,policy:IfcBooleanEvaluationPolicy,sampling:IfcCurveSampling){
  this.#control=control;this.#sampling=sampling;this.#depth=control.maxDepth!;const observe=control.progress,batch=control.batchSize!;let previous=-1;
  this.#session=new SolidBooleanEvaluation({...policy,tick:()=>{if(control.signal?.aborted)throw new IfcStepError('io.cancelled','Boolean evaluation cancelled');},progress:(work,budget)=>{const repeated=work===previous;previous=work;if(repeated||work%batch===0)observe?.({phase:'boolean-work',completed:work,total:budget});if(control.signal?.aborted)throw new IfcStepError('io.cancelled','Boolean evaluation cancelled');return true;}});
 }
 get work():number{return this.#session.work;}
 get usedValues():number{return this.#session.usedValues;}
 get peakValues():number{return this.#session.peakValues;}
 /** ⏳️ Original IO scheduling surrounds the same explicit source and kernel continuations. */
 evaluate(graph:IfcGeometryGraph,root:IfcEntity,world:IfcMatrix,scale:number,maxValues:number,visited:Set<bigint>):Promise<Result>{return runIfcIoSteps(this.#evaluateSteps(graph,root,world,scale,maxValues,visited),this.#control,'booleans');}
 /** 🧹️ Close only after entered child cursors and builders have retired. */
 close():void{this.#session.close();}
 *#source<T>(steps:IfcIoSteps<T>):IfcIoSteps<T>{let failed=false;try{for(;;){const value=steps.next();if(value.done)return value.value;this.#session.checkpoint();yield;}}catch(error){failed=true;throw error;}finally{try{steps.return(undefined as T);}catch(error){if(!failed)throw error;}}}
 *#evaluateSteps(graph:IfcGeometryGraph,root:IfcEntity,world:IfcMatrix,scale:number,maxValues:number,visited:Set<bigint>):IfcIoSteps<Result>{
  const sign=ifcMatrixOrientation(world);if(!sign)throw new IfcStepError('geometry.placement','Boolean represented frame must be finite and nonsingular');
  const frames:Frame[]=[{entity:root,stage:0}],active=new Set<bigint>(),live=new Set<SolidOwnedMesh>(),session=this.#session;let result:SolidOwnedMesh|undefined,failed=false;
  const make:IfcMeshWriterFactory<SolidOwnedMesh>=(points,triangles)=>{const builder=session.meshBuilder(points,triangles);return{point:(index,x,y,z)=>builder.point(index,x,y,z),triangle:(index,a,b,c)=>builder.triangle(index,a,sign<0?c:b,sign<0?b:c),finish:()=>{const handle=builder.seal();live.add(handle);return handle;},close:()=>builder.close()};};
  const enter=(entity:IfcEntity)=>{if(frames.length>=this.#depth)throw new IfcStepError('geometry.boolean-depth','The original expression depth is exhausted');frames.push({entity,stage:0});};
  const leave=()=>{active.delete(frames.pop()!.entity.id);};
  try{
   while(frames.length){yield;session.checkpoint();const frame=frames.at(-1)!,entity=frame.entity;
    if(frame.stage===0){
     if(active.has(entity.id))throw new IfcStepError('geometry.boolean-cycle','An active-path Boolean expression cycle is unsupported');active.add(entity.id);visited.add(entity.id);
     if(entity.name==='IFCCSGSOLID'){ifcGeometryEntity(entity,['IFCCSGSOLID']);const child=graph.ref(yield* this.#source(ifcGeometryFieldSteps(graph,entity,'IFCCSGSOLID','TreeRootExpression')));frame.stage=3;enter(child);continue;}
     if(entity.name==='IFCBOOLEANRESULT'){
      ifcGeometryEntity(entity,['IFCBOOLEANRESULT']);const operator=yield* this.#source(ifcGeometryFieldSteps(graph,entity,'IFCBOOLEANRESULT','Operator'));
      if(operator.kind!=='enum'||!['DIFFERENCE','UNION','INTERSECTION'].includes(operator.value))throw new IfcStepError('geometry.boolean-operator','An actual declared Boolean operator is required');
      frame.operation=operator.value.toLowerCase()as SolidBooleanOperation;frame.first=graph.ref(yield* this.#source(ifcGeometryFieldSteps(graph,entity,'IFCBOOLEANRESULT','FirstOperand')));frame.second=graph.ref(yield* this.#source(ifcGeometryFieldSteps(graph,entity,'IFCBOOLEANRESULT','SecondOperand')));frame.stage=1;enter(frame.first);continue;
     }
     let steps:IfcIoSteps<SolidOwnedMesh>;
     if(['IFCBLOCK','IFCRIGHTCIRCULARCYLINDER','IFCRIGHTCIRCULARCONE'].includes(entity.name))steps=ifcCsgPrimitiveWriteSteps(graph,entity,world,scale,this.#sampling,make);
     else if(entity.name==='IFCEXTRUDEDAREASOLID'){
      const profile=graph.ref(entity.args[0]);if(profile.name==='IFCRECTANGLEPROFILEDEF')steps=ifcRectangleExtrusionWriteSteps(graph,entity,world,scale,make);else if(profile.name==='IFCCIRCLEPROFILEDEF')steps=ifcCircleExtrusionWriteSteps(graph,entity,world,scale,this.#sampling,make);else if(profile.name==='IFCROUNDEDRECTANGLEPROFILEDEF')steps=ifcRoundedRectangleExtrusionWriteSteps(graph,entity,world,scale,this.#sampling,make);else throw new IfcStepError('geometry.boolean-operand','This actual swept profile requires an explicit owned Boolean writer');
     }else throw new IfcStepError('geometry.boolean-operand',entity.name+' requires an explicit owned Boolean operand evaluator');
     result=yield* this.#source(steps);leave();continue;
    }
    if(!result)throw new IfcStepError('geometry.boolean-state','A complete owned child is required');
    if(frame.stage===1){frame.left=result;result=undefined;frame.stage=2;enter(frame.second!);continue;}
    if(frame.stage===2){const left=frame.left!,right=result;result=yield* session.booleanOwnedSteps(frame.operation!,left,right);live.delete(left);live.delete(right);live.add(result);frame.left=undefined;leave();continue;}
    leave();
   }
   if(!result)throw new IfcStepError('geometry.boolean-state','A completed original expression is required');
   if(result.vertexCount===0&&result.triangleCount===0){session.releaseMesh(result);live.delete(result);return{kind:'empty'};}
   const values=3*(result.vertexCount+result.triangleCount);if(!Number.isSafeInteger(maxValues)||!Number.isSafeInteger(values)||values>maxValues)throw new IfcStepError('geometry.limit','Boolean output exceeds the remaining original value grant');
   const mesh=yield* session.publishIndexedSteps(result);live.delete(result);return{kind:'mesh',mesh:{positions:mesh.vertices,indices:mesh.indices}};
  }catch(error){failed=true;throw error;}finally{for(const handle of live)try{session.releaseMesh(handle);}catch(error){if(!failed)throw error;}live.clear();frames.length=0;active.clear();}
 }
}
