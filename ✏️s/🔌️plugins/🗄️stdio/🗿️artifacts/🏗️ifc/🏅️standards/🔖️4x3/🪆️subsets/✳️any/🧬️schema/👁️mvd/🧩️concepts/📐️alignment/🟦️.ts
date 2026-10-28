/** 🚆️ Execute only the released ordered business terminal clause under explicit applicability. */
import definition from './definition.json';
import type {IfcAlignmentTerminalRequest,IfcAlignmentTerminalReport,IfcAlignmentTerminalTargetReport,IfcAlignmentTerminalStatus,IfcAlignmentDiagnostic,IfcAlignmentDiagnosticCode} from './types.ts';
import type {IfcSnapshot,IfcValue} from '../../../📸️snapshot/🟦️.ts';
import {ifc4x3Catalog} from '../../../📚️catalog/🟦️.ts';
import {ifcAttributeQuerySteps,type IfcAttributeMatch} from '../../../../../../../../📚️catalog/🔎️query/🟦️.ts';
import {copyIfcOwnedSteps,runIfcEditAsync,IfcEditError} from '../../../🔺️diff/🟦️.ts';
import {captureIfcIoControlScope,ifcCheckpoint,IfcStepError,type IfcIoControl} from '../../../../🚪️io/control.ts';
import {ifcNumber} from '../../../../🚪️io/step-values.ts';
import {ifcConceptOptions,ifcConceptCancelled,ifcConceptId,captureIfcConceptSource,runIfcConceptSteps,ifcConceptIndexSteps,ifcConceptMembershipSteps,ifcConceptMembershipsSteps,ifcConceptPrimitiveSteps,type IfcConceptSteps as Steps,type IfcConceptIndexed as Indexed} from '../🟦️.ts';
export type * from './types.ts';
export const ifcAlignmentTerminalDefinition=Object.freeze({...definition,source:Object.freeze({...definition.source}),layouts:Object.freeze(definition.layouts.map(row=>Object.freeze({...row})))});
const layouts=ifcAlignmentTerminalDefinition.layouts;
const ranks:Record<IfcAlignmentTerminalStatus,number>={'not-applicable':0,pass:1,fail:2,incomplete:3};
const labels:Record<IfcAlignmentDiagnosticCode,readonly [string,string]>={
 'mvd.source':['The source cannot be admitted for this predicate.','Die Quelle kann für diese Prüfung nicht eingelesen werden.'],
 'mvd.applicability':['Predicate applicability is unresolved.','Die Anwendbarkeit der Prüfung ist ungeklärt.'],
 'mvd.target.missing':['The selected layout is missing.','Das ausgewählte Layout fehlt.'],
 'mvd.target.ambiguous':['The selected layout identifier is ambiguous.','Die Kennung des ausgewählten Layouts ist mehrdeutig.'],
 'mvd.target.type':['The target has another layout kind.','Das Ziel hat eine andere Layoutart.'],
 'mvd.target.unknown':['The layout declaration or kind is unresolved.','Die Layoutdeklaration oder Layoutart ist ungeklärt.'],
 'mvd.attribute.missing':['A required clause attribute is missing.','Ein erforderliches Prüfungsattribut fehlt.'],
 'mvd.attribute.ambiguous':['Several constituents declare the required attribute.','Mehrere Bestandteile deklarieren das erforderliche Attribut.'],
 'mvd.attribute.unknown':['The required attribute declaration is unresolved.','Die erforderliche Attributdeklaration ist ungeklärt.'],
 'mvd.attribute.shape':['A required clause attribute has an unresolved shape.','Die Wertstruktur eines erforderlichen Prüfungsattributs ist ungeklärt.'],
 'mvd.nesting.missing':['No ordered segment nesting was resolved.','Keine geordnete Segmentverschachtelung wurde aufgelöst.'],
 'mvd.nesting.ambiguous':['Several nesting lists have no unambiguous shared order.','Mehrere Verschachtelungslisten haben keine eindeutige gemeinsame Reihenfolge.'],
 'mvd.nesting.unresolved':['Nesting ownership is unresolved.','Die Zuordnung der Verschachtelung ist ungeklärt.'],
 'mvd.member.reference':['A segment member is not a reference.','Ein Segmenteintrag ist keine Referenz.'],
 'mvd.member.missing':['A referenced segment or design parameter is missing.','Ein referenziertes Segment oder ein Entwurfsparameter fehlt.'],
 'mvd.member.ambiguous':['A referenced segment or design parameter is ambiguous.','Ein referenziertes Segment oder ein Entwurfsparameter ist mehrdeutig.'],
 'mvd.member.type':['A list member is not an authenticated alignment segment.','Ein Listeneintrag ist kein bestätigtes Trassierungssegment.'],
 'mvd.parameter.type':['The design parameter has an unresolved layout domain.','Der Layoutbereich des Entwurfsparameters ist ungeklärt.'],
 'mvd.length.shape':['The declared segment length is not a finite numeric value.','Die deklarierte Segmentlänge ist kein endlicher Zahlenwert.'],
 'mvd.terminal.nonzero':['The last business segment has nonzero length.','Das letzte fachliche Segment hat eine Länge ungleich null.'],
};
function diagnostic(code:IfcAlignmentDiagnosticCode,path:readonly string[]=[],pointer?:string):IfcAlignmentDiagnostic{const [en,de]=labels[code];return {code,path,label:{en,de},...(pointer===undefined?{}:{pointer})};}
function report(targets:readonly IfcAlignmentTerminalTargetReport[],status:IfcAlignmentTerminalStatus,diagnostics:readonly IfcAlignmentDiagnostic[]=[]):IfcAlignmentTerminalReport{return {concept:definition.concept,predicate:'business-terminal-zero',source:ifcAlignmentTerminalDefinition.source,schemaConformance:'not-evaluated',viewConformance:'not-evaluated',geometryConformance:'not-evaluated',status:diagnostics.length?'incomplete':status,targets,diagnostics};}
function invalidRequest():never{throw new IfcStepError('mvd.request','Explicit unique layout IDs, layout kinds and applicability are required');}
interface Nesting {readonly rows:Indexed[];unresolved:boolean}
interface Relations {readonly byOwner:ReadonlyMap<bigint,Nesting>;readonly unresolved:boolean}
function* relations(snapshot:IfcSnapshot,index:ReadonlyMap<bigint,Indexed|null>,targets:ReadonlySet<bigint>):Steps<Relations>{
 const byOwner=new Map<bigint,Nesting>();let unresolved=false;
 for(let ordinal=0;ordinal<snapshot.entities.length;ordinal++){yield;const entity=snapshot.entities[ordinal]!,role=yield* ifcConceptMembershipsSteps(entity,['IFCRELNESTS']);if(role.unknown)unresolved=true;if(!role.matches[0])continue;
  const indexed={entity,ordinal},owners=yield* ifcAttributeQuerySteps(ifc4x3Catalog,indexed,'RelatingObject','IFCRELNESTS'),owner=owners.matches[0];
  if(owners.status!=='unique'||owner?.value?.kind!=='reference'){unresolved=true;continue;}
  const id=owner.value.value;if(!targets.has(id))continue;let entry=byOwner.get(id);if(!entry){entry={rows:[],unresolved:false};byOwner.set(id,entry);}entry.rows.push(indexed);if(index.get(entity.id)?.ordinal!==ordinal)entry.unresolved=true;
 }return {byOwner,unresolved};
}
function* required(indexed:Indexed,name:string,owner:string,path:readonly string[],diagnostics:IfcAlignmentDiagnostic[]):Steps<IfcAttributeMatch<IfcValue>|undefined>{
 const query=yield* ifcAttributeQuerySteps(ifc4x3Catalog,indexed,name,owner),attribute=query.matches[0];if(query.status!=='unique'||attribute?.value===undefined){diagnostics.push(diagnostic(query.status==='unknown'?'mvd.attribute.unknown':query.status==='ambiguous'?'mvd.attribute.ambiguous':'mvd.attribute.missing',path,attribute?.pointer));return undefined;}return attribute;
}
function resolve(index:ReadonlyMap<bigint,Indexed|null>,id:bigint,path:readonly string[],pointer:string,diagnostics:IfcAlignmentDiagnostic[]):Indexed|undefined{
 const indexed=index.get(id);if(!indexed){diagnostics.push(diagnostic(indexed===null?'mvd.member.ambiguous':'mvd.member.missing',path,pointer));return undefined;}return indexed;
}
function* members(target:IfcAlignmentTerminalRequest['targets'][number],nesting:Indexed,index:ReadonlyMap<bigint,Indexed|null>,diagnostics:IfcAlignmentDiagnostic[]):Steps<boolean>{
 const objects=yield* required(nesting,'RelatedObjects','IFCRELNESTS',['RelatedObjects'],diagnostics);if(!objects)return false;const value=objects.value!;
 if(value.kind!=='aggregate'||value.value.length===0){diagnostics.push(diagnostic('mvd.attribute.shape',['RelatedObjects'],objects.pointer));return false;}
 const declaration=layouts.find(row=>row.kind===target.kind)!;let failed=false;
 for(let ordinal=0;ordinal<value.value.length;ordinal++){yield;const member=value.value[ordinal]!,path=['RelatedObjects',String(ordinal)],pointer=objects.pointer+'/value/'+ordinal;
  if(member.kind!=='reference'){diagnostics.push(diagnostic('mvd.member.reference',path,pointer));continue;}
  const segment=resolve(index,member.value,path,pointer,diagnostics);if(!segment)continue;if((yield* ifcConceptMembershipSteps(segment.entity,'IFCALIGNMENTSEGMENT'))!==true){diagnostics.push(diagnostic('mvd.member.type',path,pointer));continue;}
  const designPath=[...path,'DesignParameters'],design=yield* required(segment,'DesignParameters','IFCALIGNMENTSEGMENT',designPath,diagnostics);if(!design)continue;if(design.value!.kind!=='reference'){diagnostics.push(diagnostic('mvd.attribute.shape',designPath,design.pointer));continue;}
  const parameter=resolve(index,design.value!.value,designPath,design.pointer,diagnostics);if(!parameter)continue;const roles=yield* ifcConceptMembershipsSteps(parameter.entity,layouts.map(row=>row.parameter)),matches=roles.matches.filter(Boolean).length;
  if(roles.unknown||matches!==1||!roles.matches[layouts.indexOf(declaration)]){diagnostics.push(diagnostic('mvd.parameter.type',designPath,design.pointer));continue;}
  const lengthPath=[...designPath,declaration.length],length=yield* required(parameter,declaration.length,declaration.parameter,lengthPath,diagnostics);if(!length)continue;
  const scalar=yield* ifcConceptPrimitiveSteps(length.value!,length.type,'REAL');let zero:boolean|undefined;
  if(scalar?.kind==='integer')zero=scalar.value===0n;else if(scalar?.kind==='real'){try{zero=ifcNumber(scalar)===0;}catch(error){if(!(error instanceof IfcStepError))throw error;}}
  if(zero===undefined){diagnostics.push(diagnostic('mvd.length.shape',lengthPath,length.pointer));continue;}if(ordinal===value.value.length-1&&!zero){failed=true;diagnostics.push(diagnostic('mvd.terminal.nonzero',lengthPath,length.pointer));}
 }return failed;
}
function* evaluate(input:IfcAlignmentTerminalRequest,index:ReadonlyMap<bigint,Indexed|null>,nested:Relations):Steps<IfcAlignmentTerminalTargetReport[]>{
 const rows:IfcAlignmentTerminalTargetReport[]=[];
 for(const target of input.targets){yield;const diagnostics:IfcAlignmentDiagnostic[]=[];
  if(target.applicable!==true){rows.push({...target,status:target.applicable===false?'not-applicable':'incomplete',diagnostics:target.applicable===false?[]:[diagnostic('mvd.applicability')]});continue;}
  const id=BigInt(target.layout),indexed=index.get(id);if(!indexed){rows.push({...target,status:'incomplete',diagnostics:[diagnostic(indexed===null?'mvd.target.ambiguous':'mvd.target.missing')]});continue;}
  const roles=yield* ifcConceptMembershipsSteps(indexed.entity,layouts.map(row=>row.layout)),matches=roles.matches.filter(Boolean).length,kind=layouts.findIndex(row=>row.kind===target.kind);
  if(roles.unknown||matches>1){rows.push({...target,status:'incomplete',diagnostics:[diagnostic('mvd.target.unknown',[], '/entities/'+indexed.ordinal+'/name')]});continue;}
  if(!roles.matches[kind]){rows.push({...target,status:'fail',diagnostics:[diagnostic('mvd.target.type',[], '/entities/'+indexed.ordinal+'/name')]});continue;}
  const entry=nested.byOwner.get(id);if(nested.unresolved||entry?.unresolved)diagnostics.push(diagnostic('mvd.nesting.unresolved',['RelatedObjects']));
  if(!entry||entry.rows.length!==1){diagnostics.push(diagnostic(entry?'mvd.nesting.ambiguous':'mvd.nesting.missing',['RelatedObjects']));rows.push({...target,status:'incomplete',diagnostics});continue;}
  const failed=yield* members(target,entry.rows[0]!,index,diagnostics),incomplete=diagnostics.length>(failed?1:0);rows.push({...target,status:incomplete?'incomplete':failed?'fail':'pass',diagnostics});
 }return rows;
}
function* finish(input:IfcAlignmentTerminalRequest,rows?:readonly IfcAlignmentTerminalTargetReport[]):Steps<{rows:IfcAlignmentTerminalTargetReport[];status:IfcAlignmentTerminalStatus}>{
 const result:IfcAlignmentTerminalTargetReport[]=[];let status:IfcAlignmentTerminalStatus='not-applicable';
 for(let ordinal=0;ordinal<input.targets.length;ordinal++){yield;const target=input.targets[ordinal]!,row=rows?.[ordinal]??{...target,status:target.applicable===false?'not-applicable':'incomplete',diagnostics:[]};result.push(row);if(ranks[row.status]>ranks[status])status=row.status;}return {rows:result,status};
}
/** 🫴️ Internal composition prepares only clause relationships against the original shared source index. */
export function* ifcAlignmentTerminalPreparedSteps(input:IfcAlignmentTerminalRequest,snapshot:IfcSnapshot|undefined,index:ReadonlyMap<bigint,Indexed|null>|undefined):Steps<IfcAlignmentTerminalReport>{
 if(!snapshot||!index){const result=yield* finish(input);return report(result.rows,'incomplete',[diagnostic('mvd.source')]);}
 const targets=new Set<bigint>();for(const target of input.targets){yield;if(target.applicable===true)targets.add(BigInt(target.layout));}
 const nested=yield* relations(snapshot,index,targets),rows=yield* evaluate(input,index,nested),result=yield* finish(input,rows);return report(result.rows,result.status);
}
/** 🛤️ Check the business clause on privately captured source and request; no complete child or view verdict is produced. */
export async function validateIfcAlignmentTerminal(source:IfcSnapshot,request:IfcAlignmentTerminalRequest,control:IfcIoControl={}):Promise<IfcAlignmentTerminalReport>{
 const scope=captureIfcIoControlScope(control);control=scope.control;const options=ifcConceptOptions(control,'alignment-request-'),targets=new Set<bigint>();let input:IfcAlignmentTerminalRequest;
 try{input=await runIfcEditAsync(copyIfcOwnedSteps(request,options,false,{plainDataOnly:true,maxProperties:control.maxValues,maxContainerDepth:16}),options);if(!input||Object.keys(input).length!==1||!Object.hasOwn(input,'targets')||!Array.isArray(input.targets)||input.targets.length<1||input.targets.length>control.maxEntities!)invalidRequest();const ids=new Set<string>();
  await ifcCheckpoint(control,'alignment-request',0,input.targets.length);for(let ordinal=0;ordinal<input.targets.length;ordinal++){const target=input.targets[ordinal]!;if(!target||Object.keys(target).length!==3||!Object.hasOwn(target,'layout')||!Object.hasOwn(target,'kind')||!Object.hasOwn(target,'applicable')||!ifcConceptId(target.layout)||!layouts.some(row=>row.kind===target.kind)||target.applicable!==null&&typeof target.applicable!=='boolean'||ids.has(target.layout))invalidRequest();ids.add(target.layout);if(target.applicable===true)targets.add(BigInt(target.layout));if((ordinal+1)%(control.batchSize!)===0)await ifcCheckpoint(control,'alignment-request',ordinal+1,0);}await ifcCheckpoint(control,'alignment-request',input.targets.length,input.targets.length);
 }catch(error){scope.rethrowCallerFailure();ifcConceptCancelled(error);if(error instanceof IfcEditError)invalidRequest();throw error;}
 const snapshot=await captureIfcConceptSource(source,control,'alignment');if(!snapshot)return runIfcConceptSteps(ifcAlignmentTerminalPreparedSteps(input,undefined,undefined),control,'alignment-result');
 const index=await runIfcConceptSteps(ifcConceptIndexSteps(snapshot),control,'alignment-index');
 const nested=await runIfcConceptSteps(relations(snapshot,index,targets),control,'alignment-relations'),rows=await runIfcConceptSteps(evaluate(input,index,nested),control,'alignment-members'),result=await runIfcConceptSteps(finish(input,rows),control,'alignment-result');return report(result.rows,result.status);
}
