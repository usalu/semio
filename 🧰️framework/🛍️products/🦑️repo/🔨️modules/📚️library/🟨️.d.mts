import type{BuildGraphV1,BuildTargetV1,BuildProjectV1}from"./🏗️build/🕸️graph/🟦️.ts";
/** 🕸️ Actual JavaScript projection exports retain dynamic graph DATA as unknown. */
export function nxBuildDependencyImplementationV1():string;
export function nxBuildEngineReferencesV1(source:string):readonly Readonly<{kind:"module"|"directory-url"|"file-url";specifier:string}>[];
export function nxBuildEngineSourceFileV1(owner:string,specifier:string):string;
export function nxBuildEngineInputsV1():readonly unknown[];
export function readPrivateNxBuildGraphV1(workspaceRoot:string,environment:Readonly<Record<string,string|undefined>>):Promise<BuildGraphV1>;
export function projectBuildTargetForNxV1(target:BuildTargetV1):NxProjectedTargetV1;
export function projectBuildGraphForNxV1(graph:BuildGraphV1):NxProjectedGraphV1;
export function projectBuildDependenciesForNxV1(graph:BuildGraphV1,context:NxDependencyContextV1):BuildGraphV1["dependencies"];
export function createDependencies(options:unknown,context:unknown):Promise<BuildGraphV1["dependencies"]>;
export function projectBuildDependencyAuthorityV1(workspaceRoot:string,graph:BuildGraphV1,digest:string,owners:readonly(readonly[string,string])[],identity:string):string;
export function dependencyResolutionAuthority(workspaceRoot:string,projects:unknown):Promise<string>;
declare const plugin:Readonly<{name:string;createNodesV2:readonly[string,(files:readonly string[],options:unknown,context:{workspaceRoot:string})=>Promise<readonly(readonly[string,NxProjectedGraphV1])[]>]}>;
export default plugin;

/** 🔌️ Owns the private backend DATA projection without exporting task-runner package types. */
export interface NxProjectedTargetV1{executor:string;inputs?:unknown[];outputs?:string[];dependsOn?:BuildTargetV1["dependsOn"];cache?:boolean;continuous?:boolean;options?:Record<string,unknown>;configurations?:Record<string,Record<string,unknown>>;[key:string]:unknown}
export interface NxProjectedProjectV1{name:string;root:string;targets:Record<string,NxProjectedTargetV1>;namedInputs:Record<string,unknown[]>;[key:string]:unknown}
export interface NxProjectedGraphV1{projects:Record<string,NxProjectedProjectV1>;externalNodes:Record<string,unknown>}
export interface NxDependencyContextV1{workspaceRoot:string;projects:Record<string,BuildProjectV1>;externalNodes?:Record<string,unknown>;fileMap?:{projectFileMap:Record<string,readonly{file:string}[]>;nonProjectFiles:readonly{file:string}[]}}
