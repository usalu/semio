/** 🚆️ Follow the selected zero business segment's own direct representation under the released counterpart clause. */
import definition from './direct-definition.json';
import businessDefinition from './definition.json';
import type {IfcAlignmentDirectRequest,IfcAlignmentDirectReport,IfcAlignmentDirectTarget,IfcAlignmentDirectTargetReport,IfcAlignmentDirectDiagnostic,IfcAlignmentDirectDiagnosticCode} from './direct-types.ts';
import type {IfcConceptStatus} from '../types.ts';
import type {IfcSnapshot,IfcValue} from '../../../📸️snapshot/🟦️.ts';
import {ifc4x3Catalog} from '../../../📚️catalog/🟦️.ts';
import {ifcNameSteps} from '../../../../../../../../📚️catalog/🟦️.ts';
import {ifcAttributeQuerySteps,type IfcAttributeMatch} from '../../../../../../../../📚️catalog/🔎️query/🟦️.ts';
import {copyIfcOwnedSteps,runIfcEditAsync,IfcEditError} from '../../../🔺️diff/🟦️.ts';
import {captureIfcIoControlScope,ifcCheckpoint,IfcStepError,type IfcIoControl} from '../../../../🚪️io/control.ts';
import {ifcNumber} from '../../../../🚪️io/step-values.ts';
import {ifcConceptOptions,ifcConceptCancelled,ifcConceptId,captureIfcConceptSource,runIfcConceptSteps,ifcConceptIndexSteps,ifcConceptMembershipSteps,ifcConceptMembershipsSteps,ifcConceptPrimitiveSteps,type IfcConceptSteps as Steps,type IfcConceptIndexed as Indexed} from '../🟦️.ts';
export type * from './direct-types.ts';
export const ifcAlignmentDirectDefinition=Object.freeze({...definition,source:Object.freeze({...definition.source})});
const ranks:Record<IfcConceptStatus,number>={'not-applicable':0,pass:1,fail:2,incomplete:3};
const labels:Record<IfcAlignmentDirectDiagnosticCode,readonly [string,string]>={
 'mvd.direct.source':['The source cannot be admitted for this predicate.','Die Quelle kann für diese Prüfung nicht eingelesen werden.'],
 'mvd.direct.target.missing':['The selected business segment is missing.','Das ausgewählte fachliche Segment fehlt.'],
 'mvd.direct.target.ambiguous':['The selected segment identifier is ambiguous.','Die Kennung des ausgewählten Segments ist mehrdeutig.'],
 'mvd.direct.target.type':['The selected object is not an alignment segment.','Das ausgewählte Objekt ist kein Trassierungssegment.'],
 'mvd.direct.role.unknown':['The declared object role is unresolved.','Die deklarierte Objektrolle ist ungeklärt.'],
 'mvd.direct.attribute.missing':['A declared clause attribute is missing.','Ein deklariertes Prüfungsattribut fehlt.'],
 'mvd.direct.attribute.ambiguous':['Several source slots declare this attribute.','Mehrere Quellpositionen deklarieren dieses Attribut.'],
 'mvd.direct.attribute.unknown':['Attribute ownership is unresolved.','Die Attributzuordnung ist ungeklärt.'],
 'mvd.direct.attribute.shape':['The declared attribute has an unresolved value shape.','Die Wertstruktur des deklarierten Attributs ist ungeklärt.'],
 'mvd.direct.reference.missing':['A directly referenced object is missing.','Ein direkt referenziertes Objekt fehlt.'],
 'mvd.direct.reference.ambiguous':['A directly referenced identifier is ambiguous.','Eine direkt referenzierte Kennung ist mehrdeutig.'],
 'mvd.direct.parameter.type':['The design parameter belongs to another layout kind.','Der Entwurfsparameter gehört zu einer anderen Layoutart.'],
 'mvd.direct.business.shape':['The selected business length is not a finite numeric measure.','Die ausgewählte fachliche Länge ist kein endliches numerisches Maß.'],
 'mvd.direct.business.nonzero':['The selected segment does not meet this predicate\'s zero-length precondition.','Das ausgewählte Segment erfüllt die Nulllängenbedingung dieser Prüfung nicht.'],
 'mvd.direct.path.missing':['No direct geometric counterpart path was resolved.','Kein direkter geometrischer Gegenstückpfad wurde aufgelöst.'],
 'mvd.direct.path.ambiguous':['Several direct paths require an explicit correspondence policy.','Mehrere direkte Pfade erfordern eine explizite Zuordnungsregel.'],
 'mvd.direct.path.unsupported':['The direct representation requires another correspondence evaluator.','Die direkte Repräsentation erfordert eine andere Zuordnungsprüfung.'],
 'mvd.direct.curve.shape':['The curve length requires one finite value in a released measure SELECT branch.','Die Kurvenlänge erfordert einen endlichen Wert in einem freigegebenen Maß-SELECT-Zweig.'],
 'mvd.direct.curve.nonzero':['The selected direct geometric counterpart has nonzero length.','Das ausgewählte direkte geometrische Gegenstück hat eine Länge ungleich null.'],
};
type Attribute=IfcAttributeMatch<IfcValue>;
function diagnostic(code:IfcAlignmentDirectDiagnosticCode,path:readonly string[]=[],pointer?:string):IfcAlignmentDirectDiagnostic{const [en,de]=labels[code];return{code,path,label:{en,de},...(pointer===undefined?{}:{pointer})};}
function invalidRequest():never{throw new IfcStepError('mvd.request','Explicit unique segment IDs, layout kinds and boolean applicability are required');}
function* field(indexed:Indexed,name:string,owner:string,path:readonly string[],diagnostics:IfcAlignmentDirectDiagnostic[]):Steps<Attribute|undefined>{
 const result=yield* ifcAttributeQuerySteps(ifc4x3Catalog,indexed,name,owner),match=result.matches[0];
 if(result.status!=='unique'||match?.value===undefined){diagnostics.push(diagnostic(result.status==='unknown'?'mvd.direct.attribute.unknown':result.status==='ambiguous'?'mvd.direct.attribute.ambiguous':'mvd.direct.attribute.missing',path,match?.pointer));return;}return match;
}
function resolve(index:ReadonlyMap<bigint,Indexed|null>,value:IfcValue|undefined,path:readonly string[],diagnostics:IfcAlignmentDirectDiagnostic[],pointer?:string):Indexed|undefined{
 if(value?.kind!=='reference'){diagnostics.push(diagnostic('mvd.direct.attribute.shape',path,pointer));return;}const row=index.get(value.value);if(!row){diagnostics.push(diagnostic(row===null?'mvd.direct.reference.ambiguous':'mvd.direct.reference.missing',path,pointer));return;}return row;
}
function finite(value:IfcValue|undefined):number|undefined{if(value?.kind!=='integer'&&value?.kind!=='real')return;try{return ifcNumber(value);}catch(error){if(!(error instanceof IfcStepError))throw error;}}
function* pathRole(indexed:Indexed,expected:string,path:readonly string[],diagnostics:IfcAlignmentDirectDiagnostic[]):Steps<boolean>{const role=yield* ifcConceptMembershipSteps(indexed.entity,expected);if(role===true)return true;diagnostics.push(diagnostic(role===undefined?'mvd.direct.role.unknown':'mvd.direct.path.unsupported',path));return false;}
function single(attribute:Attribute,path:readonly string[],diagnostics:IfcAlignmentDirectDiagnostic[]):IfcValue|undefined{
 const value=attribute.value;if(value?.kind!=='aggregate'){diagnostics.push(diagnostic('mvd.direct.attribute.shape',path,attribute.pointer));return;}
 if(value.value.length!==1){diagnostics.push(diagnostic(value.value.length?'mvd.direct.path.ambiguous':'mvd.direct.path.missing',path,attribute.pointer));return;}return value.value[0];
}
function* targetResult(target:IfcAlignmentDirectTarget,index:ReadonlyMap<bigint,Indexed|null>):Steps<IfcAlignmentDirectTargetReport>{
 yield;const diagnostics:IfcAlignmentDirectDiagnostic[]=[],result=(status:IfcConceptStatus,curve?:string):IfcAlignmentDirectTargetReport=>({...target,status,diagnostics,...(curve===undefined?{}:{curve})});
 if(!target.applicable)return result('not-applicable');
 const segment=index.get(BigInt(target.segment));if(!segment){diagnostics.push(diagnostic(segment===null?'mvd.direct.target.ambiguous':'mvd.direct.target.missing'));return result('incomplete');}
 const membership=yield* ifcConceptMembershipSteps(segment.entity,'IFCALIGNMENTSEGMENT');if(membership!==true){diagnostics.push(diagnostic(membership===undefined?'mvd.direct.role.unknown':'mvd.direct.target.type'));return result(membership===false?'fail':'incomplete');}
 const parameterField=yield* field(segment,'DesignParameters','IFCALIGNMENTSEGMENT',['DesignParameters'],diagnostics);if(!parameterField)return result('incomplete');
 const parameters=resolve(index,parameterField.value,['DesignParameters'],diagnostics,parameterField.pointer);if(!parameters)return result('incomplete');
 const layouts=businessDefinition.layouts,roles=yield* ifcConceptMembershipsSteps(parameters.entity,layouts.map(layout=>layout.parameter)),kind=layouts.findIndex(layout=>layout.kind===target.kind),matches=roles.matches.filter(Boolean).length;
 if(roles.unknown||matches!==1){diagnostics.push(diagnostic('mvd.direct.role.unknown',['DesignParameters']));return result('incomplete');}
 if(!roles.matches[kind]){diagnostics.push(diagnostic('mvd.direct.parameter.type',['DesignParameters']));return result('fail');}
 const layout=layouts[kind]!,businessPath=['DesignParameters',layout.length],business=yield* field(parameters,layout.length,layout.parameter,businessPath,diagnostics);if(!business)return result('incomplete');
 const measure=yield* ifcConceptPrimitiveSteps(business.value!,business.type,'REAL'),length=finite(measure);
 if(length===undefined){diagnostics.push(diagnostic('mvd.direct.business.shape',businessPath,business.pointer));return result('incomplete');}
 if(length!==0){diagnostics.push(diagnostic('mvd.direct.business.nonzero',businessPath,business.pointer));return result('fail');}
 const definitionField=yield* field(segment,'Representation','IFCPRODUCT',['Representation'],diagnostics);if(!definitionField)return result('incomplete');
 if(definitionField.value?.kind==='unset'){diagnostics.push(diagnostic('mvd.direct.path.missing',['Representation'],definitionField.pointer));return result('incomplete');}
 const productDefinition=resolve(index,definitionField.value,['Representation'],diagnostics,definitionField.pointer);if(!productDefinition||!(yield* pathRole(productDefinition,'IFCPRODUCTDEFINITIONSHAPE',['Representation'],diagnostics)))return result('incomplete');
 const representationsPath=['Representation','Representations'],representations=yield* field(productDefinition,'Representations','IFCPRODUCTREPRESENTATION',representationsPath,diagnostics);if(!representations)return result('incomplete');
 const representationValue=single(representations,representationsPath,diagnostics);if(!representationValue)return result('incomplete');
 const representation=resolve(index,representationValue,representationsPath,diagnostics,representations.pointer+'/value/0');if(!representation||!(yield* pathRole(representation,'IFCSHAPEREPRESENTATION',representationsPath,diagnostics)))return result('incomplete');
 const itemsPath=[...representationsPath,'0','Items'],items=yield* field(representation,'Items','IFCREPRESENTATION',itemsPath,diagnostics);if(!items)return result('incomplete');
 const itemValue=single(items,itemsPath,diagnostics);if(!itemValue)return result('incomplete');
 const curve=resolve(index,itemValue,itemsPath,diagnostics,items.pointer+'/value/0');if(!curve||!(yield* pathRole(curve,'IFCCURVESEGMENT',itemsPath,diagnostics)))return result('incomplete');
 const curvePath=[...itemsPath,'0','SegmentLength'],curveLength=yield* field(curve,'SegmentLength','IFCCURVESEGMENT',curvePath,diagnostics);if(!curveLength)return result('incomplete',String(curve.entity.id));
 const wrapped=curveLength.value!,branch=wrapped.kind==='typedValue'?yield* ifcNameSteps(wrapped.value.name):undefined;
 const scalar=branch&&['IFCLENGTHMEASURE','IFCPARAMETERVALUE'].includes(branch)&&wrapped.kind==='typedValue'&&wrapped.value.items.length===1?yield* ifcConceptPrimitiveSteps(wrapped,{kind:'named',name:branch},'REAL'):undefined,geometric=finite(scalar);
 if(geometric===undefined){diagnostics.push(diagnostic('mvd.direct.curve.shape',curvePath,curveLength.pointer));return result('incomplete',String(curve.entity.id));}
 if(geometric!==0){diagnostics.push(diagnostic('mvd.direct.curve.nonzero',curvePath,curveLength.pointer));return result('fail',String(curve.entity.id));}
 return result('pass',String(curve.entity.id));
}
function* evaluate(request:IfcAlignmentDirectRequest,index:ReadonlyMap<bigint,Indexed|null>):Steps<IfcAlignmentDirectTargetReport[]>{const rows:IfcAlignmentDirectTargetReport[]=[];for(const target of request.targets)rows.push(yield* targetResult(target,index));return rows;}
function* finish(input:IfcAlignmentDirectRequest,rows?:readonly IfcAlignmentDirectTargetReport[]):Steps<{targets:IfcAlignmentDirectTargetReport[];status:IfcConceptStatus}>{
 const targets:IfcAlignmentDirectTargetReport[]=[];let status:IfcConceptStatus='not-applicable';for(let ordinal=0;ordinal<input.targets.length;ordinal++){yield;const target=input.targets[ordinal]!,row=rows?.[ordinal]??{...target,status:target.applicable?'incomplete':'not-applicable',diagnostics:[]};targets.push(row);if(ranks[row.status]>ranks[status])status=row.status;}return{targets,status};
}
/** 🫴️ Internal selected-clause composition borrows the original shared source index. */
export function* ifcAlignmentDirectPreparedSteps(input:IfcAlignmentDirectRequest,index:ReadonlyMap<bigint,Indexed|null>|undefined):Steps<IfcAlignmentDirectReport>{
 const rows=index?yield* evaluate(input,index):undefined,result=yield* finish(input,rows);
 return{concept:definition.concept,predicate:'selected-direct-zero-counterpart',source:ifcAlignmentDirectDefinition.source,schemaConformance:'not-evaluated',viewConformance:'not-evaluated',geometryConformance:'not-evaluated',layoutTerminalConformance:'not-evaluated',parentCurveConformance:'not-evaluated',status:index?result.status:'incomplete',targets:result.targets,diagnostics:index?[]:[diagnostic('mvd.direct.source')]};
}
/** 🛤️ A direct path identifies correspondence; source admission and this local verdict never certify a whole IFC view. */
export async function validateIfcAlignmentDirectCounterpart(source:IfcSnapshot,request:IfcAlignmentDirectRequest,control:IfcIoControl={}):Promise<IfcAlignmentDirectReport>{
 const scope=captureIfcIoControlScope(control);control=scope.control;const options=ifcConceptOptions(control,'alignment-direct-request-');let input:IfcAlignmentDirectRequest;
 try{
  input=await runIfcEditAsync(copyIfcOwnedSteps(request,options,false,{plainDataOnly:true,maxProperties:control.maxValues,maxContainerDepth:16}),options);
  if(!input||Object.keys(input).length!==1||!Object.hasOwn(input,'targets')||!Array.isArray(input.targets)||!input.targets.length||input.targets.length>control.maxEntities!)invalidRequest();
  const ids=new Set<string>();await ifcCheckpoint(control,'alignment-direct-request',0,input.targets.length);
  for(let ordinal=0;ordinal<input.targets.length;ordinal++){const target=input.targets[ordinal]!;if(!target||Object.keys(target).length!==3||!Object.hasOwn(target,'segment')||!Object.hasOwn(target,'kind')||!Object.hasOwn(target,'applicable')||!ifcConceptId(target.segment)||!businessDefinition.layouts.some(layout=>layout.kind===target.kind)||typeof target.applicable!=='boolean'||ids.has(target.segment))invalidRequest();ids.add(target.segment);if((ordinal+1)%(control.batchSize!)===0)await ifcCheckpoint(control,'alignment-direct-request',ordinal+1,0);}
  await ifcCheckpoint(control,'alignment-direct-request',input.targets.length,input.targets.length);
 }catch(error){scope.rethrowCallerFailure();ifcConceptCancelled(error);if(error instanceof IfcEditError)invalidRequest();throw error;}
 const snapshot=await captureIfcConceptSource(source,control,'alignment-direct'),index=snapshot?await runIfcConceptSteps(ifcConceptIndexSteps(snapshot),control,'alignment-direct-index'):undefined,rows=index?await runIfcConceptSteps(evaluate(input,index),control,'alignment-direct-targets'):undefined,result=await runIfcConceptSteps(finish(input,rows),control,'alignment-direct-result');
 return{concept:definition.concept,predicate:'selected-direct-zero-counterpart',source:ifcAlignmentDirectDefinition.source,schemaConformance:'not-evaluated',viewConformance:'not-evaluated',geometryConformance:'not-evaluated',layoutTerminalConformance:'not-evaluated',parentCurveConformance:'not-evaluated',status:snapshot?result.status:'incomplete',targets:result.targets,diagnostics:snapshot?[]:[diagnostic('mvd.direct.source')]};
}
