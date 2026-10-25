import { defineScriptProviderV1, loadScriptProviderV1 } from "../../../🔌️providers/🟦️.ts";
import type { ScriptInvocation } from "../../../🧭️routing/📥️invocation/🟦️.ts";
import type { ResourceLeaseControl } from "../../../🔒️leases/🟦️.ts";
import type { CargoArtifactBuildOptionsV1 } from "../../../📦️artifacts/🏗️native-build/🟦️.ts";
import type { NativeStorageReservationV1 } from "../../../📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import type { CargoSourceBoundaryPolicyV1 } from "../../../📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts";
import type { BuildMode } from "../../../🏗️build/🎚️mode/🟦️.ts";
import type { CargoTestPolicyV1, CargoTestRequestV1 } from "../🟦️.ts";
import type { ExactCargoLawOptions, ExactCargoLawReceipt } from "../🎯️exact/🟦️.ts";

/** 📨️ Carries one exact Cargo law request; the provider completes it with its selected preparation, storage and source authority. */
export type ExactCargoLawRequestV1 = Omit<ExactCargoLawOptions, "storage" | "sourceBoundary" | "leaseDirectory" | "manifestPaths" | "cargoTargetDir" | "artifactDir" | "cancelled"> & Readonly<{ manifestPath?: string; manifestPaths?: Readonly<Record<string, string>> }>;
/** 🔨️ Carries one Cargo artifact production request; the provider completes it with its selected storage, source and network authority. */
export type CargoArtifactBuildRequestV1 = Omit<CargoArtifactBuildOptionsV1, "storage" | "sourceBoundary" | "maximumObservationBytes" | "withCargoInputs"> & Readonly<{ invocation: ScriptInvocation; producer: "artifact" | "descriptor-emitter" }>;

/** 🏪️ Exposes the bounded compiler store a provider holds open for one operation. */
export type CompilerStoreViewV1 = Readonly<{ control: ResourceLeaseControl; root: string; buildDirectory: string; captureDirectory: string; storage: NativeStorageReservationV1; sourceBoundary: CargoSourceBoundaryPolicyV1; environment: Readonly<Record<string, string | undefined>> }>;

/** 🔌️ Admits and runs Cargo compiler work under the authority an issuing parent selected for the whole script process. */
export interface CargoAdmissionProviderV1 {
  runTests(request: CargoTestRequestV1, policy: CargoTestPolicyV1, workspace: string, invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void>;
  runLint(request: CargoTestRequestV1, policy: CargoTestPolicyV1, workspace: string, invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void>;
  runCommand(invocation: ScriptInvocation, phaseControl: ResourceLeaseControl, workspace: string, args: readonly string[], cwd: string, environment: Readonly<Record<string, string | undefined>>, maximumMilliseconds: number, emit?: boolean, store?: "shared" | "isolated"): Promise<void>;
  runPackageTests(packages: string[], cwd: string, invocation: ScriptInvocation, extraArgs?: string[], environment?: Readonly<Record<string, string | undefined>>): Promise<void>;
  testPolicy(manifestPath: string, cwd: string, environment?: Readonly<Record<string, string | undefined>>): CargoTestPolicyV1;
  runExactLaws(request: ExactCargoLawRequestV1): Promise<readonly ExactCargoLawReceipt[]>;
  buildArtifacts(manifest: string, args: string[], root: string, request: CargoArtifactBuildRequestV1): Promise<void>;
  withCompilerStore<T>(workspace: string, mode: BuildMode, control: ResourceLeaseControl, environment: Readonly<Record<string, string | undefined>>, body: (store: CompilerStoreViewV1, control: ResourceLeaseControl) => Promise<T>): Promise<T>;
}

/** 🧷️ Names the port and the exact functions its implementation must expose. */
export const CARGO_ADMISSION_PROVIDER_V1 = defineScriptProviderV1<CargoAdmissionProviderV1>("cargo-admission", ["runTests", "runLint", "runCommand", "runPackageTests", "testPolicy", "runExactLaws", "buildArtifacts", "withCompilerStore"]);
const provider = (): Promise<CargoAdmissionProviderV1> => loadScriptProviderV1(CARGO_ADMISSION_PROVIDER_V1);

/** 🧪️ Runs Cargo tests of one manifest under the issued compiler admission. */
export async function runAdmittedCargoTestsV1(request: CargoTestRequestV1, policy: CargoTestPolicyV1, workspace: string, invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void> {
  return (await provider()).runTests(request, policy, workspace, invocation, phaseControl);
}
/** 🧹️ Runs Clippy of one manifest under the issued compiler admission. */
export async function runAdmittedCargoLintV1(request: CargoTestRequestV1, policy: CargoTestPolicyV1, workspace: string, invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void> {
  return (await provider()).runLint(request, policy, workspace, invocation, phaseControl);
}
/** 🦀️ Runs one exact Cargo command under the issued compiler admission, failing on a nonzero status. */
export async function runAdmittedCargoCommandV1(invocation: ScriptInvocation, phaseControl: ResourceLeaseControl, workspace: string, args: readonly string[], cwd: string, environment: Readonly<Record<string, string | undefined>>, maximumMilliseconds: number, emit?: boolean, store?: "shared" | "isolated"): Promise<void> {
  return (await provider()).runCommand(invocation, phaseControl, workspace, args, cwd, environment, maximumMilliseconds, emit, store);
}
/** 📦️ Runs the tests of named packages after the provider selected and prepared their workspaces. */
export async function runAdmittedCargoPackageTestsV1(packages: string[], cwd: string, invocation: ScriptInvocation, extraArgs?: string[], environment?: Readonly<Record<string, string | undefined>>): Promise<void> {
  return (await provider()).runPackageTests(packages, cwd, invocation, extraArgs, environment);
}
/** 🔐️ Authors the Cargo test policy the provider selected for one manifest. */
export async function admittedCargoTestPolicyV1(manifestPath: string, cwd: string, environment?: Readonly<Record<string, string | undefined>>): Promise<CargoTestPolicyV1> {
  return (await provider()).testPolicy(manifestPath, cwd, environment);
}
/** 🎯️ Runs exact Cargo laws under the issued compiler admission. */
export async function runAdmittedExactCargoLawsV1(request: ExactCargoLawRequestV1): Promise<readonly ExactCargoLawReceipt[]> {
  return (await provider()).runExactLaws(request);
}
/** 🏭️ Produces and captures Cargo artifacts under the issued compiler admission. */
export async function buildAdmittedCargoArtifactsV1(manifest: string, args: string[], root: string, request: CargoArtifactBuildRequestV1): Promise<void> {
  return (await provider()).buildArtifacts(manifest, args, root, request);
}
/** 🗄️ Holds the bounded compiler store open while the body captures one owned compiler operation. */
export async function withAdmittedCompilerStoreV1<T>(workspace: string, mode: BuildMode, control: ResourceLeaseControl, environment: Readonly<Record<string, string | undefined>>, body: (store: CompilerStoreViewV1, control: ResourceLeaseControl) => Promise<T>): Promise<T> {
  return (await provider()).withCompilerStore(workspace, mode, control, environment, body);
}
