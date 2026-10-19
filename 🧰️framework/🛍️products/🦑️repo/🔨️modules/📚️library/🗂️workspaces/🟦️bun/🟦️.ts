import { createRequire } from "node:module";
export type BunWorkspaceDeclarationV1 = Readonly<{ schemaVersion: 1; members: readonly string[]; owners: readonly string[] }>;
export type BunWorkspaceScope = Readonly<{ directory: string; manifest: string; lock: string; declaration: BunWorkspaceDeclarationV1 }>;
export type BunRepositoryMembership = Readonly<{ scopes: readonly BunWorkspaceScope[]; packages: readonly string[] }>;
/** 🎯️ Selects all authored installation scopes or exactly one declared owner. */
export type BunWorkspaceSelection = Readonly<{kind:"all"}> | Readonly<{kind:"scope";owner:string}>;
/** 📣️ Reports actual physical and ownership visits of one fresh verification. */
export type BunWorkspaceProgress = Readonly<{ operation: "state" | "list" | "readText" | "membership" | "selection" | "publication"; path: string; completedOperations: number }>;
/** 🎛️ Receives the original operation signal and continuation without constructing authority. */
export interface BunWorkspaceControl { readonly signal: AbortSignal; advance(progress: BunWorkspaceProgress): Promise<void> }
/** 🧾️ Publishes current native patterns and independently owned source packages after manifest revalidation. */
export type BunWorkspaceOwnershipReport = Readonly<{ version: 1; scopes: readonly Readonly<{ manifest: string; beforeSha256: string; patterns: readonly string[]; packages: readonly string[] }>[]; packages: readonly string[] }>;
/** 🪢️ Retains original fresh Source observations throughout one controlled callback. */
export type BunWorkspaceInstallationCustodyV1 = Readonly<{scopes:readonly Readonly<{manifest:string;packages:readonly string[]}>[];recheck:()=>Promise<void>}>;
const physical: {
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
} = createRequire(import.meta.url)("./🟨️.cjs");

/** 🧬️ Admits the canonical first-party native workspace declaration. */
export const parseBunWorkspaceDeclaration: (value: unknown) => BunWorkspaceDeclarationV1 = physical.parseBunWorkspaceDeclaration;
/** 📦️ Resolves independently owned source packages through concrete physical exports. */
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
