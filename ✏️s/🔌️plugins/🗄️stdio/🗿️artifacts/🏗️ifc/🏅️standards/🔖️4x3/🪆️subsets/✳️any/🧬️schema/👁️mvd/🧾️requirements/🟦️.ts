/** 🧾️ Execute closed selected predicates over one original private source without inferring whole-view authority. */
import type {IfcSnapshot} from '../../📸️snapshot/🟦️.ts';
import {copyIfcOwnedSteps,runIfcEditAsync,IfcEditError} from '../../🔺️diff/🟦️.ts';
import {captureIfcIoControlScope,IfcStepError,ifcCheckpoint,type IfcIoControl} from '../../../🚪️io/control.ts';
import {captureIfcConceptSource,ifcConceptIndexSteps,ifcConceptOptions,ifcConceptCancelled,ifcConceptId,runIfcConceptSteps,type IfcConceptSteps as Steps,type IfcConceptIndexed} from '../🧩️concepts/🟦️.ts';
import {ifcBodyPreparedSteps} from '../🧩️concepts/🧱️body/🟦️.ts';
import {ifcAlignmentTerminalPreparedSteps} from '../🧩️concepts/📐️alignment/🟦️.ts';
import {ifcAlignmentDirectPreparedSteps} from '../🧩️concepts/📐️alignment/direct.ts';
import {ifcBodyTessellationDefinition,ifcBodyTessellationItemLabel} from '../🧩️concepts/🔺️tessellation/🟦️.ts';
import {ifcBodySweptSolidDefinition,ifcBodySweptSolidItemLabel} from '../🧩️concepts/🌊️swept-solid/🟦️.ts';
import {ifcBodyAdvancedSweptSolidDefinition,ifcBodyAdvancedSweptSolidItemLabel} from '../🧩️concepts/🌪️advanced-swept-solid/🟦️.ts';
import {ifcBodySurfaceModelDefinition,ifcBodySurfaceModelItemLabel} from '../🧩️concepts/🫧️surface-model/🟦️.ts';
import {ifcBodyClippingDefinition,ifcBodyClippingItemLabel} from '../🧩️concepts/✂️clipping/🟦️.ts';
import {ifcBodyBrepDefinition,ifcBodyBrepItemLabel} from '../🧩️concepts/🧊️brep/🟦️.ts';
import {ifcBodyAdvancedBrepDefinition,ifcBodyAdvancedBrepItemLabel} from '../🧩️concepts/💎️advanced-brep/🟦️.ts';
import type {IfcBodyRequirementKey,IfcRequirementsRequest,IfcRequirementsReport,IfcRequirementSelection,IfcRequirementChild,IfcRequirementResult,IfcRequirementCoverage} from './types.ts';
import {ifcRequirementCoverageSteps} from './coverage.ts';
export type * from './types.ts';
const body=Object.freeze({
 'body-tessellation':[ifcBodyTessellationDefinition,ifcBodyTessellationItemLabel],
 'body-swept-solid':[ifcBodySweptSolidDefinition,ifcBodySweptSolidItemLabel],
 'body-advanced-swept-solid':[ifcBodyAdvancedSweptSolidDefinition,ifcBodyAdvancedSweptSolidItemLabel],
 'body-surface-model':[ifcBodySurfaceModelDefinition,ifcBodySurfaceModelItemLabel],
 'body-clipping':[ifcBodyClippingDefinition,ifcBodyClippingItemLabel],
 'body-brep':[ifcBodyBrepDefinition,ifcBodyBrepItemLabel],
 'body-advanced-brep':[ifcBodyAdvancedBrepDefinition,ifcBodyAdvancedBrepItemLabel],
}as const);
const failedCodes=new Set(['mvd.target.type','mvd.equals','mvd.items.type','mvd.terminal.nonzero','mvd.direct.target.type','mvd.direct.business.nonzero','mvd.direct.curve.nonzero']);
const diagnostic=(code:string,en:string,de:string)=>({code,path:[],label:{en,de}});
function invalid():never{throw new IfcStepError('mvd.request','Closed unique requirement selections with exact typed targets are required');}
function record(value:unknown):value is Record<string,unknown>{return value!==null&&typeof value==='object'&&!Array.isArray(value);}
function keys(value:Record<string,unknown>,expected:readonly string[]):boolean{return Object.keys(value).length===expected.length&&expected.every(key=>Object.hasOwn(value,key));}
function* admit(input:unknown,maximum:number):Steps<IfcRequirementsRequest>{
 yield;if(!record(input))invalid();
 if(input.scope==='released-view'){if(!keys(input,['scope','view'])||input.view!=='ReferenceView'&&input.view!=='Alignment-basedReferenceView')invalid();return input as unknown as IfcRequirementsRequest;}
 if(input.scope!=='selected-contracts'||!keys(input,['scope','obligations'])||!Array.isArray(input.obligations)||!input.obligations.length||input.obligations.length>Math.min(maximum,64000))invalid();
 const seen=new Set<string>();
 for(const row of input.obligations){yield;if(!record(row)||!keys(row,['contract','target','applicable'])||row.applicable!==null&&typeof row.applicable!=='boolean'||!record(row.target)||typeof row.contract!=='string')invalid();let id:unknown,kind='';
  if(Object.hasOwn(body,row.contract)){if(!keys(row.target,['representation']))invalid();id=row.target.representation;}
  else if(row.contract==='alignment-terminal'||row.contract==='alignment-direct'){const name=row.contract==='alignment-terminal'?'layout':'segment';if(!keys(row.target,[name,'kind'])||!['horizontal','vertical','cant'].includes(String(row.target.kind)))invalid();id=row.target[name];kind=String(row.target.kind);}
  else invalid();
  if(!ifcConceptId(id))invalid();const identity=row.contract+':'+id+':'+kind;if(seen.has(identity))invalid();seen.add(identity);
 }
 return input as unknown as IfcRequirementsRequest;
}
class Population {
 #source:IfcSnapshot|undefined;
 #index:ReadonlyMap<bigint,IfcConceptIndexed|null>|undefined;
 private constructor(source:IfcSnapshot|undefined,index:ReadonlyMap<bigint,IfcConceptIndexed|null>|undefined){this.#source=source;this.#index=index;}
 static async capture(source:IfcSnapshot,control:IfcIoControl){const snapshot=await captureIfcConceptSource(source,control,'requirements');return new Population(snapshot,snapshot?await runIfcConceptSteps(ifcConceptIndexSteps(snapshot),control,'requirements-index'):undefined);}
 *evaluate(row:IfcRequirementSelection):Steps<IfcRequirementChild>{
  if(row.contract==='alignment-terminal')return yield* ifcAlignmentTerminalPreparedSteps({targets:[{...row.target,applicable:true}]},this.#source,this.#index);
  if(row.contract==='alignment-direct')return yield* ifcAlignmentDirectPreparedSteps({targets:[{...row.target,applicable:true}]},this.#index);
  const [definition,label]=body[row.contract];return yield* ifcBodyPreparedSteps(definition,{targets:[{...row.target,applicable:true}]},this.#index,label);
 }
}
function* result(input:IfcRequirementsRequest,rows:IfcRequirementResult[]):Steps<IfcRequirementsReport>{
 const summary=yield* ifcRequirementCoverageSteps(rows,input.scope);
 return{schema:'IFC4X3_ADD2',scope:input.scope,view:input.scope==='released-view'?input.view:null,schemaConformance:'not-evaluated',viewConformance:'not-evaluated',...summary,obligations:rows,diagnostics:input.scope==='released-view'?[diagnostic('mvd.requirements.unresolved','The complete released view requirement set is unresolved.','Der vollständige Anforderungssatz der veröffentlichten Ansicht ist ungeklärt.')]:[]};
}
/** 🧭️ Select explicit clauses; released view intent cannot grant complete requirement authority. */
export async function executeIfc4x3Requirements(source:IfcSnapshot,request:IfcRequirementsRequest,control:IfcIoControl={}):Promise<IfcRequirementsReport>{
 const scope=captureIfcIoControlScope(control);control=scope.control;const options=ifcConceptOptions(control,'requirements-request-');let input:IfcRequirementsRequest;
 try{const copy=await runIfcEditAsync(copyIfcOwnedSteps(request,options,false,{plainDataOnly:true,maxProperties:control.maxValues,maxContainerDepth:16}),options);input=await runIfcConceptSteps(admit(copy,control.maxEntities!),control,'requirements-request');}
 catch(error){scope.rethrowCallerFailure();ifcConceptCancelled(error);if(error instanceof IfcEditError)invalid();throw error;}
 const rows:IfcRequirementResult[]=[];
 if(input.scope==='selected-contracts'){
  const population=await Population.capture(source,control);
  for(const row of input.obligations){await ifcCheckpoint(control,'requirements-obligation',rows.length,input.obligations.length);
   if(row.applicable!==true){rows.push({selection:row,implementation:'implemented',status:row.applicable===false?'not-applicable':'incomplete',hasFailure:false,diagnostics:row.applicable===false?[]:[diagnostic('mvd.applicability','Requirement applicability is unresolved.','Die Anwendbarkeit der Anforderung ist ungeklärt.')]});continue;}
   const child=await runIfcConceptSteps(population.evaluate(row),control,'requirements-child');let failure=child.status==='fail';
   for(const target of child.targets){await ifcCheckpoint(control,'requirements-evidence',0,0);for(const item of target.diagnostics){await ifcCheckpoint(control,'requirements-evidence',0,0);failure ||= failedCodes.has(item.code);}}
   rows.push({selection:row,implementation:'implemented',status:child.status,hasFailure:failure,child,diagnostics:[]});
  }
 }
 const report=await runIfcConceptSteps(result(input,rows),control,'requirements-report');await ifcCheckpoint(control,'requirements-result',1,1);return report;
}
