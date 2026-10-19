import { defineScriptProviderV1, loadScriptProviderV1 } from "../🔌️providers/🟦️.ts";
import type { ScriptRouter } from "../🧭️routing/🟦️.ts";
import type { ScriptInvocation } from "../🧭️routing/📥️invocation/🟦️.ts";

/** 🔬️ One artifact's TypeScript twin: the second, independent implementation of a law whose fixture the Rust suite also answers; it returns how many assertions it carried. */
export type ArtifactTwinSelfTest = { readonly name: string; readonly run: () => number };
/** 📦️ Declares the packages, twins and extra commands of an independently owned Rust artifact package. */
export type ArtifactRustPackageOptions = { readonly packages?: readonly string[]; readonly testPackages?: readonly string[]; readonly twins?: readonly ArtifactTwinSelfTest[]; readonly commands?: ScriptRouter; readonly testCommands?: ScriptRouter; readonly snapshotSqliteTests?: readonly string[]; readonly snapshotSqliteTestBudgetMs?: number; readonly snapshotSqliteTestGroups?: readonly (readonly string[])[] };
/** 🧪️ Declares artifact-owned suites and commands under the actual package Source owner. */
export interface ArtifactTypeScriptPackageOptions { readonly suites?: readonly string[]; readonly bundleRuntime?: "javascript" | "bun"; readonly afterTest?: (invocation: ScriptInvocation) => Promise<void>; readonly commands?: ScriptRouter }

/** 🔌️ Runs the build, check and test commands of independently owned artifact packages under the repository's compiler admission. */
export interface ArtifactPackageProviderV1 {
  runRustPackageMain(packageRoot: string, cargoName: string, options?: ArtifactRustPackageOptions): Promise<void>;
  runRustPackagesTests(cargoNames: readonly string[], repoRoot: string, segments: string[], invocation: ScriptInvocation): Promise<void>;
  runTypeScriptPackageMain(packageRoot: string, packageName: string, options?: ArtifactTypeScriptPackageOptions): Promise<void>;
}

/** 🧷️ Names the port and the exact functions its implementation must expose. */
export const ARTIFACT_PACKAGE_PROVIDER_V1 = defineScriptProviderV1<ArtifactPackageProviderV1>("artifact-package", ["runRustPackageMain", "runRustPackagesTests", "runTypeScriptPackageMain"]);
const provider = (): Promise<ArtifactPackageProviderV1> => loadScriptProviderV1(ARTIFACT_PACKAGE_PROVIDER_V1);

/** 📦️ Runs an independently owned Rust artifact package through the shared Nx-native contract. */
export async function runArtifactRustPackageMain(packageRoot: string, cargoName: string, options: ArtifactRustPackageOptions = {}): Promise<void> { return (await provider()).runRustPackageMain(packageRoot, cargoName, options); }
/** 🧪️ Runs one artifact's suites across every Cargo package it is cut into, in one invocation. */
export async function runArtifactRustPackagesTests(cargoNames: readonly string[], repoRoot: string, segments: string[], invocation: ScriptInvocation): Promise<void> { return (await provider()).runRustPackagesTests(cargoNames, repoRoot, segments, invocation); }
/** 🧪️ Runs an artifact crate through the repository's level filters, Nextest profile and assertion budget. */
export async function runArtifactRustTests(cargoName: string, repoRoot: string, segments: string[], invocation: ScriptInvocation): Promise<void> { return runArtifactRustPackagesTests([cargoName], repoRoot, segments, invocation); }
/** 🟦️ Runs all default package compiler commands of a TypeScript artifact package inside fixed products and diagnostic lifetimes. */
export async function runArtifactTypeScriptPackageMain(packageRoot: string, packageName: string, options: ArtifactTypeScriptPackageOptions = {}): Promise<void> { return (await provider()).runTypeScriptPackageMain(packageRoot, packageName, options); }
