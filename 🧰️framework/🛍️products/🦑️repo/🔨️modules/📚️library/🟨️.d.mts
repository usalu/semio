/** 🕸️ Actual JavaScript projection exports retain dynamic graph DATA as unknown. */
export function nxBuildDependencyImplementationV1():string;
export function nxBuildEngineReferencesV1(source:string):readonly Readonly<{kind:"module"|"directory-url"|"file-url";specifier:string}>[];
export function nxBuildEngineSourceFileV1(owner:string,specifier:string):string;
export function nxBuildEngineInputsV1():readonly unknown[];
export function readPrivateNxBuildGraphV1(workspaceRoot:string,environment:Readonly<Record<string,string|undefined>>):unknown;
export function projectBuildTargetForNxV1(target:unknown):unknown;
export function projectBuildGraphForNxV1(graph:unknown):unknown;
export function projectBuildDependenciesForNxV1(graph:unknown,context:unknown):unknown;
export function createDependencies(options:unknown,context:unknown):unknown;
export function dependencyResolutionAuthority(workspaceRoot:string,projects:unknown):string;
declare const plugin:unknown;
export default plugin;
