import { createRequire } from "node:module";
export type BunWorkspaceDeclarationV1 = Readonly<{ schemaVersion: 1; members: readonly string[]; owners: readonly string[] }>;
export type BunWorkspaceScope = Readonly<{ directory: string; manifest: string; lock: string; declaration: BunWorkspaceDeclarationV1 }>;
export type BunRepositoryMembership = Readonly<{ scopes: readonly BunWorkspaceScope[]; packages: readonly string[] }>;
/** 📣️ Reports actual physical and ownership visits of one fresh verification. */
export type BunWorkspaceProgress = Readonly<{ operation: "state" | "list" | "readText" | "selection" | "publication"; path: string; completedOperations: number }>;
/** 🎛️ Receives the original operation signal and continuation without constructing authority. */
export interface BunWorkspaceControl { readonly signal: AbortSignal; advance(progress: BunWorkspaceProgress): Promise<void> }
/** 🧾️ Publishes current native patterns and independently owned source packages after manifest revalidation. */
export type BunWorkspaceOwnershipReport = Readonly<{ version: 1; scopes: readonly Readonly<{ manifest: string; beforeSha256: string; patterns: readonly string[]; packages: readonly string[] }>[]; packages: readonly string[] }>;
const physical: {
  readonly publishBunWorkspaceOwnership: (root: string, control: BunWorkspaceControl) => Promise<BunWorkspaceOwnershipReport>;
  readonly inspectBunWorkspaceOwnership: (root: string, control: BunWorkspaceControl) => Promise<BunWorkspaceOwnershipReport>;
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

/** 🎛️ Verifies complete native publication with one controlled shared physical ownership computation. */
export const inspectBunWorkspaceOwnership: (root: string, control: BunWorkspaceControl) => Promise<BunWorkspaceOwnershipReport> = physical.inspectBunWorkspaceOwnership;

/** 📣️ Refreshes all current installation scopes through the original operation control. */
export const publishBunWorkspaceOwnership: (root: string, control: BunWorkspaceControl) => Promise<BunWorkspaceOwnershipReport> = physical.publishBunWorkspaceOwnership;
