/** 📐️ A selected business clause leaves geometry, schema and complete view conformance unevaluated. */
import type {IfcConceptStatus,IfcConceptSource,IfcConceptDiagnostic} from '../types.ts';
export type IfcAlignmentLayoutKind='horizontal'|'vertical'|'cant';
export type IfcAlignmentTerminalStatus=IfcConceptStatus;
export interface IfcAlignmentTerminalTarget {readonly layout:string;readonly kind:IfcAlignmentLayoutKind;readonly applicable:boolean|null}
export interface IfcAlignmentTerminalRequest {readonly targets:readonly IfcAlignmentTerminalTarget[]}
export type IfcAlignmentDiagnosticCode='mvd.source'|'mvd.applicability'|'mvd.target.missing'|'mvd.target.ambiguous'|'mvd.target.type'|'mvd.target.unknown'|'mvd.attribute.missing'|'mvd.attribute.ambiguous'|'mvd.attribute.unknown'|'mvd.attribute.shape'|'mvd.nesting.missing'|'mvd.nesting.ambiguous'|'mvd.nesting.unresolved'|'mvd.member.reference'|'mvd.member.missing'|'mvd.member.ambiguous'|'mvd.member.type'|'mvd.parameter.type'|'mvd.length.shape'|'mvd.terminal.nonzero';
export type IfcAlignmentDiagnostic=IfcConceptDiagnostic<IfcAlignmentDiagnosticCode>;
export interface IfcAlignmentTerminalTargetReport extends IfcAlignmentTerminalTarget {readonly status:IfcAlignmentTerminalStatus;readonly diagnostics:readonly IfcAlignmentDiagnostic[]}
export interface IfcAlignmentTerminalReport {readonly concept:string;readonly predicate:'business-terminal-zero';readonly source:IfcConceptSource;readonly schemaConformance:'not-evaluated';readonly viewConformance:'not-evaluated';readonly geometryConformance:'not-evaluated';readonly status:IfcAlignmentTerminalStatus;readonly targets:readonly IfcAlignmentTerminalTargetReport[];readonly diagnostics:readonly IfcAlignmentDiagnostic[]}
