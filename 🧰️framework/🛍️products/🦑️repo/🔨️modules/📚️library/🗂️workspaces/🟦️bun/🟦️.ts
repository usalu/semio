import physicalModule from "./🟨️.cjs";
const{createBunWorkspaceOwnershipV1}=physicalModule;
import {fileURLToPath} from "node:url";
import {capturePhysicalSourceTextV1,physicalSourceOriginV1,type PhysicalSourceModuleMetaV1} from "../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
export type BunWorkspaceDeclarationV1 = Readonly<{ schemaVersion: 1; members: readonly string[]; owners: readonly string[] }>;
export type BunWorkspaceScope = Readonly<{ directory: string; manifest: string; lock: string; declaration: BunWorkspaceDeclarationV1 }>;
export type BunRepositoryMembership = Readonly<{ scopes: readonly BunWorkspaceScope[]; packages: readonly string[] }>;
/** 🎯️ Selects all authored installation scopes or exactly one declared owner. */
export type BunWorkspaceSelection = Readonly<{kind:"all"}> | Readonly<{kind:"scope";owner:string}>;
/** 📣️ Reports actual physical and ownership visits of one fresh verification. */
export type BunWorkspaceProgress = Readonly<{ operation: "state" | "list" | "readText" | "membership" | "frontier" | "selection" | "publication"; path: string; completedOperations: number }>;
/** 🎛️ Receives the original operation signal and continuation without constructing authority. */
export interface BunWorkspaceControl { readonly signal: AbortSignal; advance(progress: BunWorkspaceProgress): Promise<void> }
/** 🧾️ Publishes current native patterns and independently owned source packages after manifest revalidation. */
export type BunWorkspaceOwnershipReport = Readonly<{ version: 1; scopes: readonly Readonly<{ manifest: string; beforeSha256: string; patterns: readonly string[]; packages: readonly string[] }>[]; packages: readonly string[] }>;
/** 🪢️ Retains original fresh Source observations throughout one controlled callback. */
/** 🔭️ Reports the exact directory listings, path states and manifest digests one ownership computation retained. */
export type BunWorkspaceObservedOwnershipV1 = Readonly<{ root: string; paths: readonly Readonly<{ path: string; state: readonly (string | number)[]; list?: readonly Readonly<{ name: string; kind: string }>[] }>[]; texts: readonly Readonly<{ path: string; sha256: string }>[] }>;
export type BunWorkspaceInstallationCustodyV1 = Readonly<{scopes:readonly Readonly<{manifest:string;packages:readonly string[]}>[];recheck:()=>Promise<void>;observed:()=>BunWorkspaceObservedOwnershipV1}>;
export interface BunWorkspacePhysicalV1 {
  readonly withCurrentBunWorkspaceOwnershipV1:<T>(root:string,selection:BunWorkspaceSelection,control:BunWorkspaceControl,body:(custody:BunWorkspaceInstallationCustodyV1)=>Promise<T>)=>Promise<T>;
  readonly withPublishedBunWorkspaceOwnershipV1:<T>(root:string,selection:BunWorkspaceSelection,control:BunWorkspaceControl,body:(custody:BunWorkspaceInstallationCustodyV1)=>Promise<T>)=>Promise<T>;
  readonly discoverCurrentBunWorkspaceScopesV1:(root:string,selection:BunWorkspaceSelection,control:BunWorkspaceControl)=>Promise<readonly BunWorkspaceScope[]>;
  readonly publishBunWorkspaceOwnership: (root: string, selection: BunWorkspaceSelection, control: BunWorkspaceControl) => Promise<BunWorkspaceOwnershipReport>;
  readonly inspectBunWorkspaceOwnership: (root: string, selection: BunWorkspaceSelection, control: BunWorkspaceControl) => Promise<BunWorkspaceOwnershipReport>;
  readonly parseBunWorkspaceDeclaration: (value: unknown) => BunWorkspaceDeclarationV1;
  readonly bunWorkspacePackages: (root: string, scope: BunWorkspaceScope) => readonly string[];
  readonly bunWorkspaceNativePatterns: (root: string, scope: BunWorkspaceScope) => readonly string[];
  readonly bunRepositoryMembership: (root: string) => BunRepositoryMembership;
  readonly discoverBunWorkspaces: (root: string) => readonly BunWorkspaceScope[];
  readonly bunRepositoryPackages: (root: string) => readonly string[];
  readonly readListingProjection: (root: string, path: string) => readonly Readonly<{ name: string; kind: string }>[];
  readonly exportOwnershipIndexV1: (root: string) => readonly Readonly<{ path: string; stamp: string; count: number; entries: readonly Readonly<{ name: string; kind: string }>[] }>[];
  readonly seedOwnershipIndexV1: (root: string, records: readonly Readonly<{ path: string; stamp: string; count: number; entries: readonly Readonly<{ name: string; kind: string }>[] }>[]) => void;
}
/** 📜️ Captured physical policy DATA is bound privately by the owned facade. */
export interface BunWorkspacePhysicalPolicyV1 {readonly ownershipPath:string;readonly ownershipSource:string;}
const ownershipURL=new URL("../🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL));
const physical=createBunWorkspaceOwnershipV1({ownershipPath:fileURLToPath(ownershipURL),ownershipSource:capturePhysicalSourceTextV1(ownershipURL,65536)});

/** 🧬️ Admits the canonical first-party native workspace declaration. */
export const parseBunWorkspaceDeclaration: (value: unknown) => BunWorkspaceDeclarationV1 = physical.parseBunWorkspaceDeclaration;
/** 📦️ Resolves independently owned source packages through concrete authored export namespaces. */
export const bunWorkspacePackages: (root: string, scope: BunWorkspaceScope) => readonly string[] = physical.bunWorkspacePackages;
/** 🧾️ Emits the complete escaped concrete projection of independently owned packages. */
export const bunWorkspaceNativePatterns: (root: string, scope: BunWorkspaceScope) => readonly string[] = physical.bunWorkspaceNativePatterns;
/** 🗂️ Reads fresh source ownership across independent installation scopes. */
export const bunRepositoryMembership: (root: string) => BunRepositoryMembership = physical.bunRepositoryMembership;
/** 🗂️ Reads the current admitted native installation scopes. */
export const discoverBunWorkspaces: (root: string) => readonly BunWorkspaceScope[] = physical.discoverBunWorkspaces;
/** 📦️ Unions current source package documents without fabricated absent membership. */
export const bunRepositoryPackages: (root: string) => readonly string[] = physical.bunRepositoryPackages;

/** 🎛️ Verifies selected native publication with one original controlled physical ownership computation. */
export const inspectBunWorkspaceOwnership: (root: string, selection: BunWorkspaceSelection, control: BunWorkspaceControl) => Promise<BunWorkspaceOwnershipReport> = physical.inspectBunWorkspaceOwnership;

/** 📣️ Refreshes only explicitly selected installation scopes through the original operation control. */
export const publishBunWorkspaceOwnership: (root: string, selection: BunWorkspaceSelection, control: BunWorkspaceControl) => Promise<BunWorkspaceOwnershipReport> = physical.publishBunWorkspaceOwnership;

/** 🧭️ Admits current installation owners without computing package membership twice. */
export const discoverCurrentBunWorkspaceScopesV1:(root:string,selection:BunWorkspaceSelection,control:BunWorkspaceControl)=>Promise<readonly BunWorkspaceScope[]>=physical.discoverCurrentBunWorkspaceScopesV1;

/** 🪢️ Keeps original Source and topology guards alive through installation and receiving publication. */
export const withPublishedBunWorkspaceOwnershipV1:<T>(root:string,selection:BunWorkspaceSelection,control:BunWorkspaceControl,body:(custody:BunWorkspaceInstallationCustodyV1)=>Promise<T>)=>Promise<T>=physical.withPublishedBunWorkspaceOwnershipV1;

/** 👓️ Retains original current membership through a read-only callback without publishing Source. */
export const withCurrentBunWorkspaceOwnershipV1:<T>(root:string,selection:BunWorkspaceSelection,control:BunWorkspaceControl,body:(custody:BunWorkspaceInstallationCustodyV1)=>Promise<T>)=>Promise<T>=physical.withCurrentBunWorkspaceOwnershipV1;

/** 📤️ Exports the settled directory listings retained under one workspace for reuse by a later process. */
export const exportBunWorkspaceOwnershipIndexV1: (root: string) => ReturnType<BunWorkspacePhysicalV1["exportOwnershipIndexV1"]> = physical.exportOwnershipIndexV1;

/** 📥️ Seeds persisted settled directory listings that stay trusted only under their exact live stamps. */
export const seedBunWorkspaceOwnershipIndexV1: BunWorkspacePhysicalV1["seedOwnershipIndexV1"] = physical.seedOwnershipIndexV1;

/** 🔬️ Lists one directory exactly as the ownership walk does and returns the entries that can change ownership. */
export const readBunWorkspaceListingProjectionV1: (root: string, path: string) => readonly Readonly<{ name: string; kind: string }>[] = physical.readListingProjection;
