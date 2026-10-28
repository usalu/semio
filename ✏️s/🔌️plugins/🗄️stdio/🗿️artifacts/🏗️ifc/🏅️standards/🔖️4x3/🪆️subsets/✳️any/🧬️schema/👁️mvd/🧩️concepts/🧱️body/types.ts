/** 🧩️ Explicit concept reports never stand in for schema or complete exchange-view validation. */
import type {IfcConceptStatus,IfcConceptSource,IfcConceptDiagnostic as ConceptDiagnostic} from '../types.ts';
export type {IfcConceptStatus,IfcConceptSource} from '../types.ts';
export interface IfcConceptTarget {readonly representation:string;readonly applicable:boolean|null}
export interface IfcBodyConceptRequest {readonly targets:readonly IfcConceptTarget[]}
export type IfcConceptDiagnosticCode='mvd.source'|'mvd.applicability'|'mvd.target.missing'|'mvd.target.ambiguous'|'mvd.target.type'|'mvd.target.unknown'|'mvd.attribute.unknown'|'mvd.attribute.missing'|'mvd.attribute.ambiguous'|'mvd.attribute.shape'|'mvd.equals'|'mvd.items.type'|'mvd.items.reference'|'mvd.items.missing'|'mvd.items.ambiguous'|'mvd.items.unknown';
export type IfcConceptDiagnostic=ConceptDiagnostic<IfcConceptDiagnosticCode>;
export interface IfcConceptTargetReport extends IfcConceptTarget {readonly status:IfcConceptStatus;readonly diagnostics:readonly IfcConceptDiagnostic[]}
export interface IfcBodyConceptReport {readonly concept:string;readonly source:IfcConceptSource;readonly schemaConformance:'not-evaluated';readonly viewConformance:'not-evaluated';readonly status:IfcConceptStatus;readonly targets:readonly IfcConceptTargetReport[];readonly diagnostics:readonly IfcConceptDiagnostic[]}

/** 📜 Repository-authored released body clauses retain their exact inclusion and exclusion families. */
export interface IfcBodyConceptDefinition {readonly id:string;readonly source:IfcConceptSource;readonly predicates:readonly Readonly<{attribute:string;kind:'equals-string'|'every-subtype'|'every-subtype-except';value:string|readonly string[];exclude?:readonly string[]}>[]}
