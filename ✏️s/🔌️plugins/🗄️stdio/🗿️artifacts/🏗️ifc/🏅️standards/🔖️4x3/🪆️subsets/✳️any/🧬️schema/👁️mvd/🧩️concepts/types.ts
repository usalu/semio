/** 🧭️ Partial concept checks preserve source provenance and explicit conformance boundaries. */
export type IfcConceptStatus='pass'|'fail'|'not-applicable'|'incomplete';
export interface IfcConceptSource {readonly commit:string;readonly url:string;readonly sha256:string}
export interface IfcConceptDiagnostic<Code extends string=string> {readonly code:Code;readonly path:readonly string[];readonly pointer?:string;readonly label:Readonly<{en:string;de:string}>}
