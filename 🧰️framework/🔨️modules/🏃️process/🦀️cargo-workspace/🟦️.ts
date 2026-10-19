import { SCRIPT_PROVIDERS_ENV, defineScriptProviderV1, loadScriptProviderSyncV1, loadScriptProviderV1 } from "../🔌️providers/🟦️.ts";
import type { ResourceLeaseControl } from "../🔒️leases/🟦️.ts";

/** 🧭️ Names the Cargo workspace that owns a package manifest. */
export type CargoWorkspaceScopeV1 = Readonly<{ directory: string; manifest: string; lock: string; memberManifests: readonly string[] }>;
/** 📦️ Names one current member package of a Cargo workspace. */
export type CargoWorkspacePackageV1 = Readonly<{ directory: string; manifest: string; name: string; workspace: string }>;
/** 🔎️ Names the kind of one input a preparation program observed. */
export type CargoPreparationInputKindV1 = "file" | "directory" | "presence";

/** 🔌️ Prepares Cargo workspaces and records the inputs and outputs a preparation program observes, under the repository's custody. */
export interface CargoWorkspaceProviderV1 {
  observeSource(...paths: string[]): void;
  observeInput(path: string, kind?: CargoPreparationInputKindV1, bytes?: string | Buffer): void;
  observeOutput(path: string, bytes: string | Buffer): void;
  completeObservation(): void;
  prepareCapabilityLinks(repoRoot: string, ownerRoot: string, manifestRelative: string, linksRelative: string): number;
  runCapabilityContributionChecks(): number;
  repositoryPackages(root: string): readonly CargoWorkspacePackageV1[];
  workspaceForManifest(root: string, manifest: string): CargoWorkspaceScopeV1;
  prepareInvocation(root: string, args: readonly string[], cwd: string, environment: Readonly<Record<string, string | undefined>>, control: ResourceLeaseControl): Promise<void>;
}

/** 🧷️ Names the port and the exact functions its implementation must expose. */
export const CARGO_WORKSPACE_PROVIDER_V1 = defineScriptProviderV1<CargoWorkspaceProviderV1>("cargo-workspace", ["observeSource", "observeInput", "observeOutput", "completeObservation", "prepareCapabilityLinks", "runCapabilityContributionChecks", "repositoryPackages", "workspaceForManifest", "prepareInvocation"]);
const sync = (): CargoWorkspaceProviderV1 => loadScriptProviderSyncV1(CARGO_WORKSPACE_PROVIDER_V1);
const observing = (): boolean => process.env[SCRIPT_PROVIDERS_ENV] !== undefined;

/** 👁️ Records source programs of a running preparation; a process without a designated repository observes nothing. */
export function observeCargoPreparationSourceV1(...paths: string[]): void { if (observing()) sync().observeSource(...paths); }
/** 👁️ Records one input of a running preparation; a process without a designated repository observes nothing. */
export function observeCargoPreparationInputV1(path: string, kind: CargoPreparationInputKindV1 = "file", bytes?: string | Buffer): void { if (observing()) sync().observeInput(path, kind, bytes); }
/** 👁️ Records one output of a running preparation; a process without a designated repository observes nothing. */
export function observeCargoPreparationOutputV1(path: string, bytes: string | Buffer): void { if (observing()) sync().observeOutput(path, bytes); }
/** ✅️ Completes the observation of a running preparation. */
export function completeCargoPreparationObservationV1(): void { if (observing()) sync().completeObservation(); }
/** 🧩️ Links the Cargo capabilities an owner declares and returns how many targets it linked. */
export function prepareCargoCapabilityLinksV1(repoRoot: string, ownerRoot: string, manifestRelative: string, linksRelative: string): number { return sync().prepareCapabilityLinks(repoRoot, ownerRoot, manifestRelative, linksRelative); }
/** 🧪️ Checks the capability contributions and returns how many checks passed. */
export function runCargoCapabilityContributionChecks(): number { return sync().runCapabilityContributionChecks(); }
/** 📦️ Inventories all current repository member packages. */
export function cargoRepositoryPackages(root: string): readonly CargoWorkspacePackageV1[] { return sync().repositoryPackages(root); }
/** 🧭️ Selects the nearest Cargo workspace for a package manifest. */
export function cargoWorkspaceForManifest(root: string, manifest: string): CargoWorkspaceScopeV1 { return sync().workspaceForManifest(root, manifest); }
/** 🛠️ Runs the preparation programs a Cargo invocation needs before it starts. */
export async function prepareCargoWorkspaceInvocation(root: string, args: readonly string[], cwd: string, environment: Readonly<Record<string, string | undefined>>, control: ResourceLeaseControl): Promise<void> { return (await loadScriptProviderV1(CARGO_WORKSPACE_PROVIDER_V1)).prepareInvocation(root, args, cwd, environment, control); }
