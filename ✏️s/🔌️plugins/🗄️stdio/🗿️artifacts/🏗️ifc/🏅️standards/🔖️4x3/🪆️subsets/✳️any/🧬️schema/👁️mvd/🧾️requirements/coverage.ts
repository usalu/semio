/** 🔢️ Pure coverage arithmetic does not issue source validation or released-view authority. */
import type {IfcConceptStatus} from '../🧩️concepts/types.ts';
import type {IfcRequirementCoverage,IfcRequirementsRequest} from './types.ts';
export interface IfcRequirementEvidence {readonly selection:Readonly<{applicable:boolean|null}>;readonly implementation:'implemented'|'unimplemented';readonly status:IfcConceptStatus;readonly hasFailure:boolean;readonly child?:unknown}
export function* ifcRequirementCoverageSteps(rows:readonly IfcRequirementEvidence[],scope:IfcRequirementsRequest['scope']){
 const coverage:IfcRequirementCoverage={requested:rows.length,evaluated:0,passed:0,failed:0,notApplicable:0,incomplete:0,denominator:scope==='selected-contracts'?rows.length:null,authority:scope==='selected-contracts'?'explicit-selection':'unresolved-release'};
 const count={...coverage};let complete=scope==='selected-contracts'&&rows.length>0;
 for(const row of rows){yield;if(row.child!==undefined)count.evaluated++;if(row.status==='pass')count.passed++;if(row.hasFailure)count.failed++;if(row.status==='not-applicable')count.notApplicable++;if(row.selection.applicable===null||row.status==='incomplete'||row.implementation==='unimplemented'&&row.selection.applicable!==false){count.incomplete++;complete=false;}}
 const hasFailure=count.failed>0,status:IfcConceptStatus=hasFailure?'fail':!complete?'incomplete':count.passed?'pass':'not-applicable';
 return{coverage:count,complete,valid:complete&&!hasFailure&&count.passed>0,hasFailure,status};
}
