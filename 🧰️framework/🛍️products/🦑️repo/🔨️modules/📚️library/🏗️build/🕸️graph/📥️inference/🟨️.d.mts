import type{ScriptInvocation}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
/** 🕸️ Declares the actual JavaScript inference API while leaving dynamic result DATA untrusted. */
export function prepareSourceAnalysesV1():Readonly<Record<string,unknown>>;
export function createScriptInputCache(context?:unknown):unknown;
export function nativeSourceWatchPlanV1(root:string,workspaceRoot:string):Promise<unknown>;
export function inferReadOnlyBuildProjectSourcesV1(json:unknown,root:string,workspaceRoot:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<unknown>;
export function discoverNativeBuildOwnersV1(configFiles:unknown,workspaceRoot:string,context:unknown):Promise<unknown>;
export function discoverBuildOwnerAdmissionFactsV1(owner:unknown,targets:unknown,registry:unknown,context:unknown,inputNames?:unknown):Promise<unknown>;
export function readBuildOwnerRegistrySourceV1(workspaceRoot:string):string;
export function dependencyResolutionAuthority(...args:readonly unknown[]):unknown;
export function createDependencies(...args:readonly unknown[]):unknown;
export function createMetadata(...args:readonly unknown[]):unknown;
declare const plugin:unknown;
export default plugin;
export const libraryBootstrap:Promise<unknown>;
export const cacheInternals:Readonly<Record<string,unknown>>;
export function discoverBuildProjectsV1(files:unknown,options:unknown,context:unknown):Promise<unknown>;
export function discoverBuildDependenciesV1(options:unknown,context:unknown):Promise<unknown>;
