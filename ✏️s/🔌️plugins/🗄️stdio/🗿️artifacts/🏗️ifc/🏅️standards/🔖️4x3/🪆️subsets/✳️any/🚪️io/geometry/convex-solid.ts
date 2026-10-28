/** 🔲️ Indexed convex boundaries share ordered sweep vertices, finite triangles and cooperative publication. */
import{IfcStepError,type IfcIoSteps}from'../control.ts';
import{ifcTransform,type IfcMatrix,type IfcTriangleMesh}from'../geometry.ts';
import{ifcArrayMeshWriterFactory,type IfcMeshWriterFactory}from'./mesh-writer.ts';
import{convexProfilePoint,convexProfileSize,type ConvexProfileBoundary}from'../../../../../../📐️geometry/🔲️convex/🟦️.ts';
export type IfcConvexSolid={shape:'cylinder';boundary:ConvexProfileBoundary;displacement:[number,number,number];profile:IfcMatrix;frame:IfcMatrix;scale:number}|{shape:'cone';boundary:Extract<ConvexProfileBoundary,{kind:'circle'}>;displacement:[number,number,number];profile:IfcMatrix;frame:IfcMatrix;scale:number};
/** 🧭️ Apply the profile placement before displacement in the enclosing solid coordinate system. */
export function* ifcConvexSolidSteps(source:IfcConvexSolid,maxValues:number):IfcIoSteps<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 return yield* ifcConvexSolidWriteSteps(source,ifcArrayMeshWriterFactory(maxValues));
}
/** 🧭️ Stream the same indexed sweep into a pre-reserved endpoint using fixed precision scratch. */
export function* ifcConvexSolidWriteSteps<T>(source:IfcConvexSolid,make:IfcMeshWriterFactory<T>):IfcIoSteps<T>{
 const{shape,boundary,displacement,profile,frame,scale}=source,n=convexProfileSize(boundary),cone=shape==='cone';
 if(n===undefined||shape!=='cylinder'&&shape!=='cone'||cone&&boundary.kind!=='circle')throw new IfcStepError('geometry.profile','An explicit supported convex sweep boundary is required');
 const vertexCount=cone?n+1:2*n,triangleCount=cone?2*n-2:4*n-4;
 const point=(ordinal:number):[number,number,number]=>{
  const layer=ordinal<n?0:1,index=ordinal<n?ordinal:ordinal-n,apex=cone&&layer===1,sample=apex?[0,0]:convexProfilePoint(boundary,index)!;
  const base=ifcTransform(profile,[sample[0]!,sample[1]!,0]),local=base.map((value,k)=>value+layer*displacement[k]!)as[number,number,number],result=ifcTransform(frame,local).map(value=>value*scale)as[number,number,number];
  if(result.some(value=>!Number.isFinite(value)))throw new IfcStepError('geometry.range','Convex solid coordinates exceed finite SI range');return result;
 };
 const writer=make(vertexCount,triangleCount);let failed=false,face=0;
 try{
 for(let index=0;index<vertexCount;index++){yield;const p=point(index);writer.point(index,p[0],p[1],p[2]);}
 const triangle=(a:number,b:number,c:number)=>{
  if(displacement[2]<0)[b,c]=[c,b];const origin=point(a),edges=[b,c].map(index=>point(index).map((value,k)=>value-origin[k]!));
  if(edges.some(edge=>edge.some(value=>!Number.isFinite(value))))throw new IfcStepError('geometry.range','Convex solid edges exceed finite coordinate differences');
  const lengths=edges.map(edge=>Math.max(...edge.map(Math.abs)));
  if(lengths.some(length=>length===0))throw new IfcStepError('geometry.precision','Convex solid vertices collapse at the represented coordinate precision');
  const[u,v]=edges.map((edge,index)=>edge.map(value=>value/lengths[index]!)),normal=[u![1]!*v![2]!-u![2]!*v![1]!,u![2]!*v![0]!-u![0]!*v![2]!,u![0]!*v![1]!-u![1]!*v![0]!];
  if(normal.every(value=>value===0))throw new IfcStepError('geometry.precision','Convex solid triangles collapse at the represented coordinate precision');writer.triangle(face++,a,b,c);
 };
 for(let index=1;index<n-1;index++){yield;triangle(0,index+1,index);if(!cone){yield;triangle(n,n+index,n+index+1);}}
 for(let index=0;index<n;index++){const next=(index+1)%n;yield;if(cone)triangle(index,next,n);else{triangle(index,next,n+next);yield;triangle(index,n+next,n+index);}}
 return writer.finish();
 }catch(error){failed=true;throw error;}finally{try{writer.close();}catch(error){if(!failed)throw error;}}
}
