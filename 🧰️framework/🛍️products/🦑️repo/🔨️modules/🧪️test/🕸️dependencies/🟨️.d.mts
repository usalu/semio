/** 📦️ Cargo `[package]` table as the tooling parser reads it. */
export interface CargoPackageV1 {
  readonly name?: string;
  readonly [key: string]: unknown;
}

/** 🧾️ The slice of the taxonomy vocabulary that locates an owner's test contribution manifest. */
export interface TestContributionVocabularyV1 {
  readonly testContributionFileKindId: string;
  readonly testOraclesDirName: string;
  readonly fileKinds: Readonly<Record<string, { readonly emoji: string; readonly extensionChains: readonly string[] }>>;
}

/** 🦀️ Reads Cargo package identity through the tooling parser boundary. */
export function cargoPackage(workspaceRoot: string, path: string): CargoPackageV1 | undefined;

/** 📍️ Resolves a repository-owned dependency without accepting traversal outside the workspace. */
export function localPackagePath(workspaceRoot: string, path: string): string;

/** 🌳️ Enumerates an owner's ancestry through the workspace root. */
export function ownerAncestors(owner: string): string[];

/** 🧪️ Selects the same subject package for graph inference and generated host execution. */
export function rustSubjectPackage(workspaceRoot: string, owner: string): { readonly name: string; readonly path: string } | null;

/** 🔖️ Whether an oracle pin names an installable release rather than a lock marker. */
export function isInstallablePin(version: unknown): version is string;

/** 🌱️ The external host distributions one oracle declares, derived from its ecosystem and linked packages. */
export function oracleHostPackagesOf(oracle: {
  readonly ecosystem: string;
  readonly package?: string;
  readonly version?: string;
  readonly module?: string;
  readonly packages?: readonly { readonly package: string; readonly version?: string; readonly module?: string }[];
}): { implementation: string; package: string; version?: string; module?: string }[];

/** 🔮️ Selects every applicable ancestor contribution for one adapter implementation. */
export function packagesForOwner<P extends { readonly implementation: string }>(
  contributions: readonly { readonly owner: string; readonly oracleHostPackages?: readonly P[]; readonly oracles?: readonly { readonly ecosystem: string }[] }[],
  owner: string,
  implementation?: string,
): P[];

/** 🧾️ Reads only the contribution manifests that can supply packages to this owner. */
export function ownerContributions(workspaceRoot: string, vocabulary: TestContributionVocabularyV1, owner: string): (Readonly<Record<string, unknown>> & { readonly owner: string })[];
