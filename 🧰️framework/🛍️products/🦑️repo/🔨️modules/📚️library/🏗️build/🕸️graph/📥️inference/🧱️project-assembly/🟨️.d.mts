/** 🧱️ Declares the project-assembly inference component: native target inputs, declared source inputs and closure references of one project. */
export const component: "project-assembly";
import type {GeneratedProjectV1,ScriptInputCacheV1,ScriptInputContextV1} from "../🟨️.mjs";
export function reachesNativeCommandDriverV1(sources: readonly unknown[]): boolean;
export function nativeAuthoredInputsV1(policyInputs: readonly unknown[], commandInputs: readonly unknown[]|undefined): unknown[];
export function nativeTargetRunsTestsV1(name: string, execution: Readonly<{kind:string;scope?:string}>|undefined, reachesTestDriver: boolean): boolean;
export function nativeTargetInputsV1(runsTests: boolean, generated: readonly unknown[], declared: readonly unknown[]): unknown[];
export function wire(registry: Readonly<Record<string, Readonly<Record<string, unknown>>>>): void;
/** 📏️ The input precision rules, read once per process with their facts replayed into every receiving project; undefined without a rules file. */
export function readProjectInputPrecisionV1(): import("../../../📥️inputs/🎯️precision/🟦️.ts").InputPrecisionDocumentV1 | undefined;
/** 👁️ The files, includes and excludes a native plugin watches: ordered path lists of the workspace. */
export type NativeSourceWatchPlanV1=Readonly<{schema:"semio.framework.os.plugin.source-watch-plan/v1";includes:readonly string[];excludes:readonly string[];files:readonly string[]}>;
export function nativeSourceWatchPlanV1(root: string, workspaceRoot: string): Promise<NativeSourceWatchPlanV1>;
export function declaredSourceInputs(json: Readonly<Record<string,unknown>>, workspaceRoot: string, context?: ScriptInputContextV1, inputNames?: readonly string[]): Promise<Record<string,unknown[]>>;
export function projectInputs(json: Readonly<Record<string,unknown>>, root: string, workspaceRoot: string, facts: Map<string,unknown>, scripts?: ScriptInputCacheV1, sourceRepresentation?: "source"|"hash", sourceGroups?: unknown, inputNames?: readonly string[], commandGroups?: Readonly<Record<string,unknown[]>>): Promise<Record<string,unknown[]>>;
export function projectWithDefaults<T extends Readonly<Record<string,unknown>>>(json: T, root: string, projectDir: string, workspaceRoot: string, contracts?: Readonly<Record<string,unknown>>, facts?: Map<string,unknown>, commandInputs?: readonly string[], scripts?: ScriptInputCacheV1, sourceGroups?: Readonly<Record<string,unknown[]>>, sourceRepresentation?: "source"|"hash", selectedTargets?: readonly string[], inputNames?: readonly string[]): Promise<T&GeneratedProjectV1>;
/** 🧩️ The derived closure facts and rows a closure reference resolution adopted. */
export type ClosureRefsAdoptionV1=Readonly<{rows:unknown;derived:readonly Readonly<{row:unknown;support:unknown}>[]}>;
export function resolveClosureRefsV1<P extends Readonly<Record<string,unknown>>>(project: P, workspaceRoot: string, scripts: ScriptInputCacheV1, sourceRepresentation?: "source"|"hash", log?: unknown): Promise<ClosureRefsAdoptionV1&Readonly<{project:P}>>;
export function resolveClosureRefsPlanV1<P extends Readonly<Record<string,unknown>>>(project: P, workspaceRoot: string, scripts: ScriptInputCacheV1, sourceRepresentation?: "source"|"hash"): Promise<Readonly<{project:P;plan:unknown}>|undefined>;
export function adoptClosureRefsV1(plan: unknown, workspaceRoot: string, scripts: ScriptInputCacheV1, log: unknown): ClosureRefsAdoptionV1;
export function inferReadOnlyBuildProjectSourcesV1(json: Readonly<Record<string,unknown>>, root: string, workspaceRoot: string, invocation: import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation, phaseControl: import("@semio-tech/process/leases").ResourceLeaseControl): Promise<GeneratedProjectV1>;

/** 📁️ Groups made from different recipes can hold the same files; the direct inference names one group per distinct content in order of first use, so equal groups are merged and the survivors renumbered the same way. */
export function mergeCommandSourceGroupsV1<P extends Readonly<{namedInputs:Readonly<Record<string,unknown>>;targets?:Readonly<Record<string,unknown>>}>>(project: P): P;
