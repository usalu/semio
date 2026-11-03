/** 🧵️ What the edges of one owner read from the environment: a closed record, empty lists left out. */
export interface EdgeReadsV1{readonly owners?:readonly string[];readonly paths?:readonly string[];readonly packages?:readonly string[];readonly modules?:readonly string[];readonly workspaces?:readonly string[];readonly lock?:true}
/** 🧵️ The handle one owner records its lookups through. */
export interface EdgeReadsHandleV1{owner(target:string):void;path(path:string):void;package(name:string):void;module(name:string):void;workspace(path:string):void;lock():void}
export interface EdgeReadsRecorderV1{owner(name:string):EdgeReadsHandleV1;finish(name:string):EdgeReadsV1|undefined}
export declare function createEdgeReadsRecorderV1():EdgeReadsRecorderV1;
export declare const NO_EDGE_READS_V1:EdgeReadsHandleV1;
export declare function readEdgeReadsV1(value:unknown,items?:number,length?:number):EdgeReadsV1;
/** 🧭️ The stored parts of an environment: everything but the owner table, which is the owner rows of the manifest. */
export interface EnvironmentStoredPartsV1{readonly packages:readonly (readonly [string,string])[];readonly modules:readonly (readonly [string,string])[];readonly workspaces:readonly (readonly [string,string])[];readonly lock:string|null;readonly taxonomy:string|null}
export interface EnvironmentPartsV1 extends EnvironmentStoredPartsV1{readonly owners:readonly (readonly [string,string,string|null])[]}
export declare function readEnvironmentPartsV1(value:unknown):EnvironmentStoredPartsV1;
export interface EnvironmentDiffV1{readonly all:boolean;readonly names:ReadonlySet<string>;readonly roots:readonly string[];readonly packages:ReadonlySet<string>;readonly modules:ReadonlySet<string>;readonly workspaces:readonly string[];readonly lock:boolean}
export declare function diffEnvironmentV1(previous:EnvironmentPartsV1|undefined,current:EnvironmentPartsV1):EnvironmentDiffV1;
export declare function edgeConsumersV1(input:{diff:EnvironmentDiffV1;rows:ReadonlyMap<string,{root:string;nativeRoot?:string;reads?:EdgeReadsV1}>;tables:readonly ReadonlyMap<string,readonly (string|null)[]>[]}):Set<string>;
