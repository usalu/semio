/** 🔖️ Types of the inference stamp component. */
export type InferenceScopeV1 = "project" | "native" | "environment" | "all";
export interface InferrerV1 { readonly stamp: string; readonly scope: InferenceScopeV1; readonly sources: readonly string[] }
export interface InferenceProtocolV1 { readonly version: 1; readonly scopes: readonly InferenceScopeV1[]; readonly inferrers: Readonly<Record<string, InferrerV1>>; readonly none: readonly string[] }
export interface ImplementationRowV1 { readonly kind: string; readonly path: string; readonly fact: unknown; readonly scope?: InferenceScopeV1 }
export const component: "stamps";
export function wire(): void;
export function inferenceFragmentsV1(document: InferenceProtocolV1): string[];
export function admitInferenceProtocolV1(value: unknown): InferenceProtocolV1;
export function inferenceStampEntriesV1(document: InferenceProtocolV1): [string, string][];
export function inferenceProtocolV1(): InferenceProtocolV1;
export function inferrerStampPathV1(id: string): string;
export function inferrerStampV1(id: string): string;
export function projectStampPathsV1(flags: { native: boolean }): string[];
export function matchesFragmentsV1(path: string, fragments: readonly string[]): boolean;
export function classifyImplementationRowsV1(rows: readonly ImplementationRowV1[], document?: InferenceProtocolV1): ImplementationRowV1[];
export function unclassifiedCodeV1(rows: readonly ImplementationRowV1[], document?: InferenceProtocolV1): string[];
