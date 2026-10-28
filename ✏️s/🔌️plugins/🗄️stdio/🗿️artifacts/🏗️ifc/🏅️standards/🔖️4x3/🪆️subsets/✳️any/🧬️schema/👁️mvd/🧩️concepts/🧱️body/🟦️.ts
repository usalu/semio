/** 🧱️ Share controlled selected body representation clauses without inferring view applicability. */
import type {IfcBodyConceptDefinition,IfcBodyConceptRequest,IfcBodyConceptReport,IfcConceptDiagnostic,IfcConceptDiagnosticCode,IfcConceptStatus,IfcConceptTargetReport} from './types.ts';
import type {IfcSnapshot,IfcValue} from '../../../📸️snapshot/🟦️.ts';
import type {IfcTypeExpression} from '../../../📚️catalog/🟦️.ts';
import {ifc4x3Catalog} from '../../../📚️catalog/🟦️.ts';
import {ifcAttributeQuerySteps} from '../../../../../../../../📚️catalog/🔎️query/🟦️.ts';
import {copyIfcOwnedSteps,runIfcEditAsync,IfcEditError} from '../../../🔺️diff/🟦️.ts';
import {captureIfcIoControlScope,ifcCheckpoint,IfcStepError,type IfcIoControl} from '../../../../🚪️io/control.ts';
import {ifcConceptOptions as options,ifcConceptCancelled as cancelled,runIfcConceptSteps as run,ifcConceptMembershipSteps as membership,ifcConceptPrimitiveSteps,captureIfcConceptSource,ifcConceptIndexSteps,type IfcConceptSteps as Steps,type IfcConceptIndexed as Indexed} from '../🟦️.ts';
const labels:Record<IfcConceptDiagnosticCode,readonly [string,string]>={
 'mvd.source':['The source cannot be admitted for this concept.','Die Quelle kann für dieses Konzept nicht eingelesen werden.'],
 'mvd.applicability':['Concept applicability is unresolved.','Die Anwendbarkeit des Konzepts ist ungeklärt.'],
 'mvd.target.missing':['The representation target is missing.','Die angegebene Repräsentation fehlt.'],
 'mvd.target.ambiguous':['The representation identifier is ambiguous.','Die Kennung der Repräsentation ist mehrdeutig.'],
 'mvd.target.type':['The target is not a shape representation.','Das Ziel ist keine Formrepräsentation.'],
 'mvd.target.unknown':['The representation declaration is unresolved.','Die Deklaration der Repräsentation ist ungeklärt.'],
 'mvd.attribute.missing':['A required concept attribute cannot be resolved.','Ein benötigtes Konzeptattribut kann nicht aufgelöst werden.'],
 'mvd.attribute.unknown':['An attribute declaration is unresolved.','Eine Attributdeklaration ist ungeklärt.'],
 'mvd.attribute.ambiguous':['Several constituents declare the concept attribute.','Mehrere Bestandteile deklarieren das Konzeptattribut.'],
 'mvd.attribute.shape':['The concept attribute has an unresolved value shape.','Die Wertstruktur des Konzeptattributs ist ungeklärt.'],
 'mvd.equals':['The attribute differs from the released concept value.','Das Attribut weicht vom veröffentlichten Konzeptwert ab.'],
 'mvd.items.type':['A representation item is not a tessellated face set.','Ein Repräsentationselement ist kein tessellierter Flächensatz.'],
 'mvd.items.reference':['A representation item is not an entity reference.','Ein Repräsentationselement ist keine Entitätsreferenz.'],
 'mvd.items.missing':['A referenced representation item is missing.','Ein referenziertes Repräsentationselement fehlt.'],
 'mvd.items.ambiguous':['A referenced representation item is ambiguous.','Ein referenziertes Repräsentationselement ist mehrdeutig.'],
 'mvd.items.unknown':['An item declaration is unresolved.','Die Deklaration eines Elements ist ungeklärt.'],
};
function diagnostic(code:IfcConceptDiagnosticCode,path:readonly string[]=[],pointer?:string,itemLabel?:readonly [string,string]):IfcConceptDiagnostic{const [en,de]=code==='mvd.items.type'&&itemLabel?itemLabel:labels[code];return {code,path,label:{en,de},...(pointer===undefined?{}:{pointer})};}
const ranks:Record<IfcConceptStatus,number>={'not-applicable':0,pass:1,fail:2,incomplete:3};
function report(definition:IfcBodyConceptDefinition,targets:readonly IfcConceptTargetReport[],status:IfcConceptStatus,diagnostics:readonly IfcConceptDiagnostic[]=[]):IfcBodyConceptReport{return {concept:definition.id,source:definition.source,schemaConformance:'not-evaluated',viewConformance:'not-evaluated',status:diagnostics.length?'incomplete':status,targets,diagnostics};}
function invalidRequest():never{throw new IfcStepError('mvd.request','Explicit unique representation IDs and applicability decisions are required');}
function* unavailable(input:IfcBodyConceptRequest):Steps<IfcConceptTargetReport[]>{const rows:IfcConceptTargetReport[]=[];for(const target of input.targets){yield;rows.push({...target,status:target.applicable===false?'not-applicable':'incomplete',diagnostics:[]});}return rows;}
function* label(source:IfcValue,expression:IfcTypeExpression):Steps<string|undefined>{
 const value=yield* ifcConceptPrimitiveSteps(source,expression,'STRING');return value?.kind==='string'?value.value:undefined;
}
function* evaluate(definition:IfcBodyConceptDefinition,input:IfcBodyConceptRequest,index:ReadonlyMap<bigint,Indexed|null>,itemLabel:readonly [string,string]):Steps<{rows:IfcConceptTargetReport[];status:IfcConceptStatus}>{
 const rows:IfcConceptTargetReport[]=[];let status:IfcConceptStatus='not-applicable';const append=(row:IfcConceptTargetReport)=>{rows.push(row);if(ranks[row.status]>ranks[status])status=row.status;};
 for(const target of input.targets){yield;const diagnostics:IfcConceptDiagnostic[]=[];let failed=false,incomplete=false;
  if(target.applicable!==true){append({...target,status:target.applicable===false?'not-applicable':'incomplete',diagnostics:target.applicable===false?[]:[diagnostic('mvd.applicability')]});continue;}
  const indexed=index.get(BigInt(target.representation));if(!indexed){append({...target,status:'incomplete',diagnostics:[diagnostic(indexed===null?'mvd.target.ambiguous':'mvd.target.missing')]});continue;}
  const shape=yield* membership(indexed.entity,'IFCSHAPEREPRESENTATION');if(shape!==true){append({...target,status:shape===false?'fail':'incomplete',diagnostics:[diagnostic(shape===false?'mvd.target.type':'mvd.target.unknown',[],`/entities/${indexed.ordinal}/name`)]});continue;}
  for(const predicate of definition.predicates){yield;const query=yield* ifcAttributeQuerySteps(ifc4x3Catalog,indexed,predicate.attribute,'IFCREPRESENTATION'),found=query.matches,path=[predicate.attribute];
   if(query.status!=='unique'||found[0]?.value===undefined){incomplete=true;diagnostics.push(diagnostic(query.status==='unknown'?'mvd.attribute.unknown':query.status==='ambiguous'?'mvd.attribute.ambiguous':'mvd.attribute.missing',path,found[0]?.pointer));continue;}
   const attributeValue=found[0]!,value=attributeValue.value!;
   if(predicate.kind==='equals-string'){if((yield* label(value,attributeValue.type))!==predicate.value){failed=true;diagnostics.push(diagnostic('mvd.equals',path,attributeValue.pointer));}continue;}
   if(value.kind!=='aggregate'){incomplete=true;diagnostics.push(diagnostic('mvd.attribute.shape',path,attributeValue.pointer));continue;}
   for(let item=0;item<value.value.length;item++){yield;const reference=value.value[item]!,itemPath=[...path,String(item)],pointer=attributeValue.pointer+'/value/'+item;if(reference.kind!=='reference'){incomplete=true;diagnostics.push(diagnostic('mvd.items.reference',itemPath,pointer));continue;}
    const resolved=index.get(reference.value);if(!resolved){incomplete=true;diagnostics.push(diagnostic(resolved===null?'mvd.items.ambiguous':'mvd.items.missing',itemPath,pointer));continue;}
    let matched=false,excluded=false,unresolved=false;
    for(const name of typeof predicate.value==='string'?[predicate.value]:predicate.value){const value=yield* membership(resolved.entity,name);matched ||= value===true;unresolved ||= value===undefined;}
    for(const name of predicate.exclude??[]){const value=yield* membership(resolved.entity,name);excluded ||= value===true;unresolved ||= value===undefined;}
    const accepted=unresolved?undefined:matched&&!excluded;if(accepted!==true){if(accepted===undefined)incomplete=true;else failed=true;diagnostics.push(diagnostic(accepted===undefined?'mvd.items.unknown':'mvd.items.type',itemPath,pointer,itemLabel));}
   }
  }
  append({...target,status:incomplete?'incomplete':failed?'fail':'pass',diagnostics});
 }return {rows,status};
}

/** 🫴️ Internal composition consumes one already captured source index and the exact static definition. */
export function* ifcBodyPreparedSteps(definition:IfcBodyConceptDefinition,input:IfcBodyConceptRequest,index:ReadonlyMap<bigint,Indexed|null>|undefined,itemLabel:readonly [string,string]):Steps<IfcBodyConceptReport>{
 if(!index)return report(definition,yield* unavailable(input),'incomplete',[diagnostic('mvd.source')]);
 const result=yield* evaluate(definition,input,index,itemLabel);return report(definition,result.rows,result.status);
}
/** 🧭️ Check three released clauses against privately captured data. Keep source and request stable through capture for exact start-state semantics. */
export async function validateIfcBodyConcept(source:IfcSnapshot,request:IfcBodyConceptRequest,control:IfcIoControl,definition:IfcBodyConceptDefinition,itemLabel:readonly [string,string]):Promise<IfcBodyConceptReport>{
 const scope=captureIfcIoControlScope(control);control=scope.control;const requestOptions=options(control,'mvd-request-');let input:IfcBodyConceptRequest;
 try{input=await runIfcEditAsync(copyIfcOwnedSteps(request,requestOptions,false,{plainDataOnly:true,maxProperties:control.maxValues,maxContainerDepth:16}),requestOptions);if(!input||Object.keys(input).length!==1||!Object.hasOwn(input,'targets')||!Array.isArray(input.targets)||input.targets.length<1||input.targets.length>control.maxEntities!)invalidRequest();const ids=new Set<string>();for(let index=0;index<input.targets.length;index++){if(index%(control.batchSize!)===0)await ifcCheckpoint(control,'mvd-request',index,input.targets.length);const target=input.targets[index]!;if(!target||Object.keys(target).length!==2||!Object.hasOwn(target,'representation')||!Object.hasOwn(target,'applicable')||typeof target.representation!=='string'||!/^[1-9][0-9]{0,19}$/u.test(target.representation)||BigInt(target.representation)>(1n<<64n)-1n||target.applicable!==null&&typeof target.applicable!=='boolean'||ids.has(target.representation))invalidRequest();ids.add(target.representation);}await ifcCheckpoint(control,'mvd-request',input.targets.length,input.targets.length);}
 catch(error){scope.rethrowCallerFailure();cancelled(error);if(error instanceof IfcEditError)invalidRequest();throw error;}
 const snapshot=await captureIfcConceptSource(source,control,'mvd'),index=snapshot?await run(ifcConceptIndexSteps(snapshot),control,'mvd-index'):undefined;
 return run(ifcBodyPreparedSteps(definition,input,index,itemLabel),control,snapshot?'mvd-concept':'mvd-result');
}

/** 🧊 Freeze repository-authored static definitions before exposing their source and family policy. */
export function freezeIfcBodyDefinition(input:IfcBodyConceptDefinition):IfcBodyConceptDefinition{
 return Object.freeze({...input,source:Object.freeze({...input.source}),predicates:Object.freeze(input.predicates.map(row=>Object.freeze({...row,...(typeof row.value==='string'?{}:{value:Object.freeze(row.value)}),...(row.exclude?{exclude:Object.freeze(row.exclude)}:{})})))});
}
