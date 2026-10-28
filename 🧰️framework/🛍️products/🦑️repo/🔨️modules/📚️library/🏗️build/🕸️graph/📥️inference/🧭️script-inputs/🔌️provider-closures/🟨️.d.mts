/** 🔌️ Types of the shared closure references (`./🟨️.mjs`). */
export type ProviderViewV1 = "scheduling" | "native";
export interface ProviderClosurePolicyV1 { readonly version: 1; readonly group: { readonly prefix: string; readonly separator: ":" }; readonly views: Readonly<Record<ProviderViewV1, { readonly packages: boolean; readonly contract: { readonly en: string; readonly de: string } }>>; readonly tools: { readonly prefix: string; readonly closures: Readonly<Record<string, { readonly entries: readonly string[]; readonly resources: readonly string[]; readonly view: ProviderViewV1; readonly contract: { readonly en: string; readonly de: string } }>> }; readonly project: { readonly basename: string; readonly metadata: string }; readonly contract: { readonly en: string; readonly de: string } }
export type SharedRecipeV1 = { readonly fn: "script-provider"; readonly port: string; readonly view: ProviderViewV1 } | { readonly fn: "tool-closure"; readonly name: string; readonly view: ProviderViewV1 };
export interface SharedReferenceV1 { readonly input: string; readonly projects: readonly [string] }
export const PROVIDER_CLOSURE_POLICY_V1: ProviderClosurePolicyV1;
export function providerGroupNameV1(port: string, view: ProviderViewV1): string;
export function toolGroupNameV1(name: string): string;
export function sharedGroupOfV1(name: unknown): SharedRecipeV1 | undefined;
export function providerReferenceV1(project: string, port: string, view: ProviderViewV1): SharedReferenceV1;
export function toolReferenceV1(project: string, name: string): SharedReferenceV1;
export function sharedReferenceOfV1(item: unknown): { readonly project: string; readonly recipe: SharedRecipeV1 } | undefined;
export function sharedReferencesOfV1(value: unknown): readonly { readonly project: string; readonly recipe: SharedRecipeV1 }[];
export function joinWithProviderViewV1(lists: readonly (readonly unknown[])[], view: ProviderViewV1): unknown[];
export function expandClosureValueV1<T>(value: T, files: readonly string[]): T;
export function providerProjectNameV1(json: unknown, designation: string): string;
