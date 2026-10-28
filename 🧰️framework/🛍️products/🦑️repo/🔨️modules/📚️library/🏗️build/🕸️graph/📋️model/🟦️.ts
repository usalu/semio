import type { BuildGraphSelectionV1 } from "../🚪️admission/🟦️.ts";

/** 🧾️ Binds loaded graph behavior and its publication declaration to their original bytes. */
export interface BuildGraphModuleAuthorityV1{implementation:string;publication:string}
/** ⚙️ Describes portable process execution independently of any build implementation. */
export interface BuildExecutionV1 { kind: "command" | "noop"; program?:string; command?: string; commands?: (string | {command:string;forwardAllArgs?:boolean})[]; args?:string[]; cwd?:string; env?:Record<string,string>; parallel?:boolean; forwardAllArgs?:boolean; [key:string]:unknown }
/** 🎯️ Declares one build operation and its configuration, inputs and products. */
export interface BuildTargetV1 { execution:BuildExecutionV1; configurations?:Record<string,Record<string,unknown>>; dependsOn?: (string | {target:string;projects?:string[]|string;dependencies?:boolean;params?:string})[]; inputs?:unknown[]; outputs?:string[]; cache?:boolean; continuous?:boolean; [key:string]:unknown }
/** 🧩️ Describes an owned project with its complete inferred operations. */
export interface BuildProjectV1 { name:string; root:string; targets:Record<string,BuildTargetV1>; tags?:string[]; namedInputs?:Record<string,unknown[]>; [key:string]:unknown }
/** 🕸️ Defines the authoritative portable workspace graph. */
export interface BuildGraphV1 { version:1; projects:Record<string,BuildProjectV1>; dependencies:{source:string;target:string;type?:string;sourceFile?:string}[]; namedInputs?:Record<string,unknown[]>; externalNodes?:Record<string,unknown>; unavailable?:BuildUnavailableProjectV1[]; admission?:{selection:BuildGraphSelectionV1;registry:string;closure:string} }
/** 🚧️ Names one project withheld from the snapshot because a source its inference consumed does not parse. */
export interface BuildUnavailableProjectV1 { root:string; project?:string; source:string; sha256:string; reason:string; missing?:string[] }
/** 📋️ Describes observed root entries without granting mutation or storage custody. */
export interface BuildGraphInvalidationInventoryV1 {device:number;entries:{name:string;kind:"file"|"directory"|"symlink"|"other";links:number;bytes:number;device:number}[]}
/** 🏗️ What one partial inference produced, in repository terms: fully inferred owners with their own facts, owners whose edges alone were recomputed, every edge of the processed owners, the environment and every fact the inference consumed. */
export interface BuildGraphPartV1 {owners:{name:string;root:string;nativeRoot?:string;config?:string;tag?:string;node?:any;unavailable?:BuildUnavailableProjectV1;nodeRows?:BuildGraphRowV1[]}[];edgeOnly:{name:string;streamed?:true}[];reEdge?:(name:string,fragment:{node?:any})=>Promise<unknown[]>|unknown[];derived?:{readonly row:BuildGraphRowV1;readonly support:readonly BuildGraphRowV1[]}[];edges:{source:string;target:string;[key:string]:unknown}[];externalNodes:Record<string,unknown>;parseUnavailable:BuildUnavailableProjectV1[];rows:BuildGraphRowV1[];env:{digest:string;rows:BuildGraphRowV1[]};replaceTags:string[]}
/** 🔨️ One consumed fact of an inference. */
export interface BuildGraphRowV1 {kind:string;path:string;fact:unknown}
/** 🧭️ The stored summary of one published owner: enough for edges, ownership and generator validation without its node. */
export interface BuildGraphPartStubV1 {root:string;kind:string;targets:readonly string[];nativeRoot?:string;config?:string;tag?:string}
