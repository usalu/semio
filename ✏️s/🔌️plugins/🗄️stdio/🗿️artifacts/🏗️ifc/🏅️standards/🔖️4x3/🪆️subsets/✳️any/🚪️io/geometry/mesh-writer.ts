import{IfcStepError}from'../control.ts';
import type{IfcTriangleMesh}from'../geometry.ts';
/** 🏗️ An entered endpoint receives exact indexed source rows without a second mesh backing. */
export interface IfcMeshWriter<T>{point(index:number,x:number,y:number,z:number):void;triangle(index:number,a:number,b:number,c:number):void;finish():T;close():void;}
/** 🧮️ The endpoint reserves genuine point/index capacity before any emitted numeric backing is born. */
export type IfcMeshWriterFactory<T>=(vertexCount:number,triangleCount:number)=>IfcMeshWriter<T>;
/** 📦️ Raw publication owns its actual positions and flat indices under the original remaining value grant. */
export function ifcArrayMeshWriterFactory(maxValues:number):IfcMeshWriterFactory<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 return(vertexCount,triangleCount)=>{
  const values=3*(vertexCount+triangleCount);
  if(!Number.isSafeInteger(maxValues)||maxValues<0||!Number.isSafeInteger(vertexCount)||vertexCount<0||vertexCount>0xffff_ffff||!Number.isSafeInteger(triangleCount)||triangleCount<0||3*triangleCount>0xffff_ffff||!Number.isSafeInteger(values)||values>maxValues)throw new IfcStepError('geometry.limit','Source mesh coordinates and indices exceed the original value grant');
  let positions:IfcTriangleMesh['positions']|undefined=[],indices:number[]|undefined=[],points=0,faces=0,closed=false,failure:IfcStepError|undefined;
  const refuse=():never=>{failure??=new IfcStepError('geometry.mesh-writer','An exact complete source mesh writer is required');throw failure;},ready=()=>{if(failure)throw failure;if(closed)refuse();};
  return Object.freeze({
   point:(index:number,x:number,y:number,z:number)=>{ready();if(!Number.isSafeInteger(index)||index<0||index>=vertexCount||Object.hasOwn(positions!,index)||![x,y,z].every(Number.isFinite))refuse();positions![index]=[x,y,z];points++;},
   triangle:(index:number,a:number,b:number,c:number)=>{ready();if(!Number.isSafeInteger(index)||index<0||index>=triangleCount||Object.hasOwn(indices!,3*index)||[a,b,c].some(value=>!Number.isSafeInteger(value)||value<0||value>=vertexCount))refuse();indices![3*index]=a;indices![3*index+1]=b;indices![3*index+2]=c;faces++;},
   finish:()=>{ready();if(points!==vertexCount||faces!==triangleCount)refuse();const result={positions:positions!,indices:indices!};positions=undefined;indices=undefined;closed=true;return result;},
   close:()=>{if(closed)return;positions=undefined;indices=undefined;closed=true;}
  });
 };
}
