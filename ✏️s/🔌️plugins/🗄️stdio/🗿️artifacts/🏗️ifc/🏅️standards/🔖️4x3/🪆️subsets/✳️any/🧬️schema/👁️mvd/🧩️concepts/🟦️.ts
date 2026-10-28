/** 🧩️ Cooperative declaration and original source access shared by executable IFC4x3 clauses. */
import {ifc4x3Catalog,type IfcTypeExpression} from '../../📚️catalog/🟦️.ts';
import {ifcNameSteps,ifcSubtypeSteps} from '../../../../../../../📚️catalog/🟦️.ts';
import type {IfcEntity,IfcSnapshot,IfcValue} from '../../📸️snapshot/🟦️.ts';
import {copyIfcOwnedSteps,validateIfcNativeShapeSteps,runIfcEditAsync,IfcEditError,type IfcApplyOptions} from '../../🔺️diff/🟦️.ts';
import {captureIfcIoControlScope,ifcCheckpoint,runIfcIoSteps,IfcStepError,type IfcIoControl} from '../../../🚪️io/control.ts';

export type IfcConceptSteps<T>=Generator<void,T,void>;
export interface IfcConceptIndexed {readonly entity:IfcEntity;readonly ordinal:number}
export function ifcConceptOptions(control:IfcIoControl,prefix:string):IfcApplyOptions{return {signal:control.signal,maxEntities:control.maxEntities,maxValues:control.maxValues,maxDepth:control.maxDepth,yieldEvery:control.batchSize,onProgress:event=>control.progress?.({...event,phase:prefix+event.phase})};}
export function ifcConceptCancelled(error:unknown):void{if(error instanceof IfcStepError&&error.code==='io.cancelled')throw error;if(error instanceof IfcEditError&&error.code==='cancelled')throw new IfcStepError('io.cancelled',error.message);}
export async function runIfcConceptSteps<T>(steps:IfcConceptSteps<T>,control:IfcIoControl,phase:string):Promise<T>{
 return runIfcIoSteps(steps,control,phase);
}
export function ifcConceptId(value:unknown):value is string{return typeof value==='string'&&/^[1-9][0-9]{0,19}$/u.test(value)&&BigInt(value)<=(1n<<64n)-1n;}
export async function captureIfcConceptSource(source:IfcSnapshot,control:IfcIoControl,prefix:string):Promise<IfcSnapshot|undefined>{
 const scope=captureIfcIoControlScope(control);control=scope.control;
 const options=ifcConceptOptions(control,prefix+'-capture-');let snapshot:IfcSnapshot;
 try{snapshot=await runIfcEditAsync(copyIfcOwnedSteps(source,options,true),options);await runIfcEditAsync(validateIfcNativeShapeSteps(snapshot,'snapshot',options),options);const schemas=snapshot.header.fileSchema;if(snapshot.schema!=='stdio.ifc'||schemas.length!==1||schemas[0]?.kind!=='aggregate'||schemas[0].value.length!==1||schemas[0].value[0]?.kind!=='string'||schemas[0].value[0].value!=='IFC4X3_ADD2')return undefined;}
 catch(error){scope.rethrowCallerFailure();ifcConceptCancelled(error);if(error instanceof IfcEditError)return undefined;throw error;}
 await ifcCheckpoint(control,prefix+'-capture',1,1);return snapshot;
}
export function* ifcConceptIndexSteps(source:IfcSnapshot):IfcConceptSteps<Map<bigint,IfcConceptIndexed|null>>{
 const index=new Map<bigint,IfcConceptIndexed|null>();for(let ordinal=0;ordinal<source.entities.length;ordinal++){yield;const entity=source.entities[ordinal]!;index.set(entity.id,index.has(entity.id)?null:{entity,ordinal});}return index;
}
export function* ifcConceptMembershipsSteps(entity:IfcEntity,expected:readonly string[]):IfcConceptSteps<{matches:boolean[];unknown:boolean}>{
 const complex=entity.complex??[],matches=expected.map(()=>false);let unknown=false;
 for(let part=0;part<=complex.length;part++){yield;const row=part?complex[part-1]!:entity,name=yield* ifcNameSteps(row.name);if(!name||!ifc4x3Catalog.entities[name]){unknown=true;continue;}for(let i=0;i<expected.length;i++){yield;if(yield* ifcSubtypeSteps(ifc4x3Catalog,name,expected[i]!))matches[i]=true;}}
 return {matches,unknown};
}
export function* ifcConceptMembershipSteps(entity:IfcEntity,expected:string):IfcConceptSteps<boolean|undefined>{const result=yield* ifcConceptMembershipsSteps(entity,[expected]);return result.unknown?undefined:result.matches[0]!;}
export function* ifcConceptPrimitiveSteps(source:IfcValue,expression:IfcTypeExpression,primitive:'STRING'|'REAL'):IfcConceptSteps<IfcValue|undefined>{
 let value=source,type=expression;const seen=new Set<string>();
 for(let depth=0;depth<64;depth++){yield;if(type.kind!=='named')return undefined;const name=yield* ifcNameSteps(type.name??'');if(!name||seen.has(name))return undefined;if(name===primitive)return value;seen.add(name);const declaration=ifc4x3Catalog.types[name];if(declaration?.kind!=='defined'||!declaration.typeExpr)return undefined;if(value.kind==='typedValue'){if((yield* ifcNameSteps(value.value.name))!==name||value.value.items.length!==1)return undefined;value=value.value.items[0]!;}type=declaration.typeExpr;}return undefined;
}
