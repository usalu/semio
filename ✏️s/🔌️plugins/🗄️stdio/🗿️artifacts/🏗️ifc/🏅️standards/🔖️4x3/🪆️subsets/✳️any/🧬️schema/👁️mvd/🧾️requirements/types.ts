/** 🧾️ Selected requirement reports retain exact child scope and unresolved released-view authority. */
import type {IfcConceptStatus,IfcConceptDiagnostic} from '../🧩️concepts/types.ts';
import type {IfcBodyConceptReport} from '../🧩️concepts/🧱️body/types.ts';
import type {IfcAlignmentTerminalReport,IfcAlignmentLayoutKind} from '../🧩️concepts/📐️alignment/types.ts';
import type {IfcAlignmentDirectReport} from '../🧩️concepts/📐️alignment/direct-types.ts';
import type {Ifc4x3MvdId} from '../🟦️.ts';
export type IfcBodyRequirementKey='body-tessellation'|'body-swept-solid'|'body-advanced-swept-solid'|'body-surface-model'|'body-clipping'|'body-brep'|'body-advanced-brep';
export type IfcRequirementSelection=Readonly<{applicable:boolean|null}>&(
 Readonly<{contract:IfcBodyRequirementKey;target:Readonly<{representation:string}>}>|
 Readonly<{contract:'alignment-terminal';target:Readonly<{layout:string;kind:IfcAlignmentLayoutKind}>}>|
 Readonly<{contract:'alignment-direct';target:Readonly<{segment:string;kind:IfcAlignmentLayoutKind}>}>);
export type IfcRequirementsRequest=Readonly<{scope:'selected-contracts';obligations:readonly IfcRequirementSelection[]}>|Readonly<{scope:'released-view';view:Ifc4x3MvdId}>;
export type IfcRequirementChild=IfcBodyConceptReport|IfcAlignmentTerminalReport|IfcAlignmentDirectReport;
export interface IfcRequirementResult {readonly selection:IfcRequirementSelection;readonly implementation:'implemented'|'unimplemented';readonly status:IfcConceptStatus;readonly hasFailure:boolean;readonly child?:IfcRequirementChild;readonly diagnostics:readonly IfcConceptDiagnostic[]}
export interface IfcRequirementCoverage {readonly requested:number;readonly evaluated:number;readonly passed:number;readonly failed:number;readonly notApplicable:number;readonly incomplete:number;readonly denominator:number|null;readonly authority:'explicit-selection'|'unresolved-release'}
export interface IfcRequirementsReport {readonly schema:'IFC4X3_ADD2';readonly scope:IfcRequirementsRequest['scope'];readonly view:Ifc4x3MvdId|null;readonly schemaConformance:'not-evaluated';readonly viewConformance:'not-evaluated';readonly complete:boolean;readonly valid:boolean;readonly hasFailure:boolean;readonly status:IfcConceptStatus;readonly coverage:IfcRequirementCoverage;readonly obligations:readonly IfcRequirementResult[];readonly diagnostics:readonly IfcConceptDiagnostic[]}
