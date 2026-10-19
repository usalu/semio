import { defineScriptProviderV1, loadScriptProviderSyncV1, loadScriptProviderV1 } from "../🔌️providers/🟦️.ts";
import type { OwnedCommandOptions } from "../🎛️owned-execution/🟦️.ts";
import type { CompilerAdmissionControl } from "../🏗️build/🎟️capacity/🟦️.ts";
import type { ScriptInvocation } from "../🧭️routing/📥️invocation/🟦️.ts";

/** 🏃️ Declares one subprocess run: working directory, environment, wall-clock budget and shell resolution. */
export interface RunCmdOpts {
  cwd?: string;
  env?: NodeJS.ProcessEnv;
  budgetMs?: number;
  onTimeoutHint?: string;
  shell?: boolean;
}
/** 🔍️ Carries the captured outcome of one probe subprocess. */
export interface RunProbeResult { status: number | null; stdout: string; stderr: string; signal: NodeJS.Signals | null }
/** 📋️ Declares original child time and independent bounded or explicitly ordinary output authority. */
export type RepositoryCommandPolicyV1 = Readonly<{ version: 1; maximumElapsedMilliseconds: number; output: Readonly<{ maximumBytes: number; maximumLines: number }> | null }>;
/** 📤️ Supplies passive transport choices without overriding original cancellation or publication. */
export type RepositoryCommandOptionsV1 = Pick<OwnedCommandOptions, "stdout" | "env" | "onLine">;
/** 🧪️ Supplies generic test command options without creating compiler authority. */
export type RepositoryTestCommandOptionsV1 = Readonly<{ cwd?: string; env?: Readonly<Record<string, string | undefined>>; budgetMs?: number; signal?: AbortSignal; onTimeoutHint?: string; throwOnFailure?: boolean }>;
/** 🎛️ Bounds runtime inventory work inside the same Original compiler operation. */
export type RepositoryMutationCollectorV1 = Readonly<{ version: 1; maximumUnits: number; maximumOwnedBytes: number; maximumMilliseconds: number }>;
/** 🧪️ Carries the options of a test command; a Cargo command also carries the actual owner invocation and phase control. */
export type RepositoryTestCommandArgumentsV1<Command extends string> = Command extends "cargo" ? [options: RepositoryTestCommandOptionsV1 & Readonly<{ invocation: ScriptInvocation; control: CompilerAdmissionControl }>] : [options?: RepositoryTestCommandOptionsV1];

/** 🔌️ Runs the repository's budgeted commands and resolves its cache, metadata and Cargo locations for scripts that must stay independent of the repository product. */
export interface RepositoryToolingProviderV1 {
  runCmd(cmd: string, args: string[], opts?: RunCmdOpts): void;
  runProbe(cmd: string, args: string[], opts?: RunCmdOpts): RunProbeResult;
  runBun(args: string[], cwd: string, env?: NodeJS.ProcessEnv): void;
  runCargo(args: string[], cwd: string, env?: NodeJS.ProcessEnv): void;
  orchestratorBudgetOpts(extra?: Partial<NodeJS.ProcessEnv>): RunCmdOpts;
  devToolingEnv(extra?: Partial<NodeJS.ProcessEnv>): NodeJS.ProcessEnv;
  repoCacheDirectory(repoRoot: string, ...segments: string[]): string;
  cargoTargetDirectory(repoRoot: string, env?: NodeJS.ProcessEnv): string;
  repoMetaDirectory(repoRoot: string): string;
  testArtifactEnvironment(repoRoot: string, route: string, env?: NodeJS.ProcessEnv): NodeJS.ProcessEnv;
  runVitest(bundleRoot: string, segments: string[], config: string): Promise<void>;
  runCommand(invocation: ScriptInvocation, command: string, args: string[], cwd: string, label: string, policy: RepositoryCommandPolicyV1, options?: RepositoryCommandOptionsV1): Promise<void>;
  runTestCommand(command: string, args: string[], options?: RepositoryTestCommandOptionsV1 & Partial<{ invocation: ScriptInvocation; control: CompilerAdmissionControl }>): Promise<void>;
  runMutationBridgeScript(packageRoot: string, binary: string, collector?: RepositoryMutationCollectorV1): Promise<void>;
  dispatchOwnedRoute(invocation: ScriptInvocation, repoRoot: string, segments: readonly string[]): Promise<boolean>;
  runCmdStatus(cmd: string, args: string[], opts?: RunCmdOpts): number;
  runNodeBinStatus(args: string[], cwd?: string, env?: NodeJS.ProcessEnv): number;
  resolveWorkspaceBin(binName: string, cwd?: string): string | null;
  daemonBudgetOpts(extra?: Partial<NodeJS.ProcessEnv>): RunCmdOpts;
  cargoProfileDir(profile: string): string;
  cargoBuildDirectory(repoRoot: string, env?: NodeJS.ProcessEnv): string;
  cargoDirectories(repoRoot: string, env?: NodeJS.ProcessEnv): Readonly<{ target: string; build: string }>;
  semioBuildParallel(): number;
  repoToolCacheEnv(repoRoot: string, extra?: NodeJS.ProcessEnv): NodeJS.ProcessEnv;
  generateCollectionCatalog(schemaPath: string, mode: "write" | "check"): readonly string[];
}

/** 🧷️ Names the port and the exact functions its implementation must expose. */
export const REPOSITORY_TOOLING_PROVIDER_V1 = defineScriptProviderV1<RepositoryToolingProviderV1>("repository-tooling", ["runCmd", "runProbe", "runBun", "runCargo", "orchestratorBudgetOpts", "devToolingEnv", "repoCacheDirectory", "cargoTargetDirectory", "repoMetaDirectory", "testArtifactEnvironment", "runVitest", "runCommand", "runTestCommand", "runMutationBridgeScript", "dispatchOwnedRoute", "generateCollectionCatalog", "runCmdStatus", "runNodeBinStatus", "resolveWorkspaceBin", "daemonBudgetOpts", "cargoProfileDir", "cargoBuildDirectory", "cargoDirectories", "semioBuildParallel", "repoToolCacheEnv"]);
const sync = (): RepositoryToolingProviderV1 => loadScriptProviderSyncV1(REPOSITORY_TOOLING_PROVIDER_V1);
const provider = (): Promise<RepositoryToolingProviderV1> => loadScriptProviderV1(REPOSITORY_TOOLING_PROVIDER_V1);

/** 🏃️ Runs a subprocess with inherited stdio and a wall-clock budget; throws on a nonzero exit, a signal or an exceeded budget. */
export function runCmd(cmd: string, args: string[], opts: RunCmdOpts = {}): void { sync().runCmd(cmd, args, opts); }
/** 🔍️ Runs a subprocess and captures its outcome without throwing on a nonzero exit. */
export function runProbe(cmd: string, args: string[], opts: RunCmdOpts = {}): RunProbeResult { return sync().runProbe(cmd, args, opts); }
/** 🍞️ Runs Bun with inherited stdio in a working directory. */
export function runBun(args: string[], cwd: string, env?: NodeJS.ProcessEnv): void { sync().runBun(args, cwd, env); }
/** 🦀️ Runs Cargo through the repository's relay with inherited output semantics. */
export function runCargo(args: string[], cwd: string, env?: NodeJS.ProcessEnv): void { sync().runCargo(args, cwd, env); }
/** ⏱️ Presets the command options of an orchestrator script. */
export function orchestratorBudgetOpts(extra: Partial<NodeJS.ProcessEnv> = {}): RunCmdOpts { return sync().orchestratorBudgetOpts(extra); }
/** 🧰️ Carries neutral build policy without IDE-injected node options. */
export function devToolingEnv(extra: Partial<NodeJS.ProcessEnv> = {}): NodeJS.ProcessEnv { return sync().devToolingEnv(extra); }
/** ⚡️ Resolves a location inside the shared repository cache. */
export function repoCacheDirectory(repoRoot: string, ...segments: string[]): string { return sync().repoCacheDirectory(repoRoot, ...segments); }
/** 🎯️ Resolves Cargo's uplifted deliverable root with Cargo's own precedence of environment over the repository configuration. */
export function cargoTargetDirectory(repoRoot: string, env?: NodeJS.ProcessEnv): string { return sync().cargoTargetDirectory(repoRoot, env); }
/** 🦑️ Resolves the repository metadata owned by a workspace. */
export function getRepoMetaDir(repoRoot: string): string { return sync().repoMetaDirectory(repoRoot); }
/** 🧾️ Resolves caller-owned test output or a neutral cache route. */
export function repoTestArtifactEnvironment(repoRoot: string, route: string, env?: NodeJS.ProcessEnv): NodeJS.ProcessEnv { return sync().testArtifactEnvironment(repoRoot, route, env); }
/** 🧪️ Runs the declared Vitest policy of a bundle through its owned process. */
export async function runVitest(bundleRoot: string, segments: string[], config: string): Promise<void> { return (await provider()).runVitest(bundleRoot, segments, config); }
/** 🦀️ Executes preparation and command under the inherited signal, phase and awaited publication. */
export async function runRepositoryCommand(invocation: ScriptInvocation, command: string, args: string[], cwd: string, label: string, policy: RepositoryCommandPolicyV1, options: RepositoryCommandOptionsV1 = {}): Promise<void> { return (await provider()).runCommand(invocation, command, args, cwd, label, policy, options); }
/** 🧪️ Runs a budgeted test command; a Cargo command carries its actual owner custody. */
export async function runRepositoryTestCommand<Command extends string>(command: Command, args: string[], ...provided: RepositoryTestCommandArgumentsV1<Command>): Promise<void> { return (await provider()).runTestCommand(command, args, provided[0]); }
/** 🚪️ Receives the real parent invocation and delivers the production mutation inventory of a package. */
export async function runRepositoryMutationBridgeScriptV1(packageRoot: string, binary: string, collector?: RepositoryMutationCollectorV1): Promise<void> { return (await provider()).runMutationBridgeScript(packageRoot, binary, collector); }
/** ▶️ Executes an admitted owner's route with progress, cancellation and inherited output ownership. */
export async function dispatchOwnedScriptRoute(invocation: ScriptInvocation, repoRoot: string, segments: readonly string[]): Promise<boolean> { return (await provider()).dispatchOwnedRoute(invocation, repoRoot, segments); }
/** 🏭️ Writes (or, in check mode, compares) the Rust and TypeScript register catalog a snapshot schema declares and returns the paths that differ. */
export function generateCollectionCatalogV1(schemaPath: string, mode: "write" | "check"): readonly string[] { return sync().generateCollectionCatalog(schemaPath, mode); }
/** 🔢️ Runs a subprocess like [[runCmd]] and returns its exit status instead of throwing on a nonzero exit. */
export function runCmdStatus(cmd: string, args: string[], opts: RunCmdOpts = {}): number { return sync().runCmdStatus(cmd, args, opts); }
/** 🟢️ Runs a Node tool from the workspace or caller installation and returns its exit status. */
export function runNodeBinStatus(args: string[], cwd: string = process.cwd(), env: NodeJS.ProcessEnv = process.env): number { return sync().runNodeBinStatus(args, cwd, env); }
/** 🔎️ Resolves a Node tool from installation launchers or its package bin declaration in the caller and workspace scopes. */
export function resolveWorkspaceBin(binName: string, cwd: string = process.cwd()): string | null { return sync().resolveWorkspaceBin(binName, cwd); }
/** ⏲️ Presets the command options of a daemon script. */
export function daemonBudgetOpts(extra: Partial<NodeJS.ProcessEnv> = {}): RunCmdOpts { return sync().daemonBudgetOpts(extra); }
/** 📂️ Names the Cargo output directory of a profile (`dev` is `debug`). */
export function cargoProfileDir(profile: string): string { return sync().cargoProfileDir(profile); }
/** 🏗️ Resolves Cargo's shared intermediate root with Cargo's own precedence of environment over the repository configuration. */
export function cargoBuildDirectory(repoRoot: string, env?: NodeJS.ProcessEnv): string { return sync().cargoBuildDirectory(repoRoot, env); }
/** 🗂️ Resolves both Cargo directories under the same precedence. */
export function cargoDirectories(repoRoot: string, env?: NodeJS.ProcessEnv): Readonly<{ target: string; build: string }> { return sync().cargoDirectories(repoRoot, env); }
/** 🧵️ Resolves the shared build task and worker capacity for this machine. */
export function semioBuildParallel(): number { return sync().semioBuildParallel(); }
/** 🧳️ Routes Go's build cache and Playwright's browser downloads into the shared cache root without overriding explicit values. */
export function repoToolCacheEnv(repoRoot: string, extra: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv { return sync().repoToolCacheEnv(repoRoot, extra); }
