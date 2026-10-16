import type {NativeBuildCallerCapabilitiesV1} from "../🟦️.ts";

/** 🎯️ Names one declared neutral build task and its explicit mode. */
export interface BuildOrchestrationTaskV1 {readonly project:string;readonly target:string;readonly configuration?:"prototype"|"release"}
/** 🧭️ Declares the bounded children of an orchestration target. */
export interface BuildOrchestrationDeclarationV1 {readonly version:1;readonly kind:"orchestration";readonly children:readonly BuildOrchestrationTaskV1[]}
/** 📋️ Carries the exact admitted task closure without issuing process authority. */
export interface BuildOrchestrationContextV1 {readonly version:1;readonly task:BuildOrchestrationTaskV1;readonly targets:readonly {readonly project:string;readonly target:string;readonly delegation:BuildOrchestrationDeclarationV1|null}[]}
/** 🗺️ Provides only the graph fields consumed by orchestration projection. */
export interface BuildOrchestrationProjectV1 {readonly targets?:Readonly<Record<string,{readonly metadata?:unknown;readonly configurations?:Readonly<Record<string,unknown>>}>>}
/** 🧭️ Captures and validates the declared bounded task closure from an admitted graph. */
export function buildOrchestrationContextV1(projects:Readonly<Record<string,BuildOrchestrationProjectV1>>,selection:Readonly<{project:string;target:string;configuration?:string}>):BuildOrchestrationContextV1|undefined;
/** 📤️ Validates carried data and projects child capabilities without issuing an Original. */
export function projectBuildOrchestrationCapabilitiesV1(envelope:unknown,workspaceRoot:string,tool:"nx"|"bazel",value:unknown,now:number):NativeBuildCallerCapabilitiesV1;

/** 🔏️ Requires the complete carried closure to match its immutable admitted graph. */
export function requireBuildOrchestrationContextV1(projects:Readonly<Record<string,BuildOrchestrationProjectV1>>,value:unknown):BuildOrchestrationContextV1;
