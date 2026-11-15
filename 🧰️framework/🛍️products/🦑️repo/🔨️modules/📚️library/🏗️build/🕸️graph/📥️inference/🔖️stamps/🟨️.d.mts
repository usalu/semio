/** 🔖️ Types of the inference stamp component. */
export type InferenceScopeV1 = "project" | "native" | "environment" | "all";
export interface InferrerV1 { readonly stamp: string; readonly scope: InferenceScopeV1; readonly sources: readonly string[] }
export interface InferenceProtocolV1 { readonly version: 1; readonly scopes: readonly InferenceScopeV1[]; readonly inferrers: Readonly<Record<string, InferrerV1>>; readonly derived: readonly string[]; readonly none: readonly string[] }
export interface ConformanceSettlementV1 { readonly status: "stored" | "fleet" | "computed" | "unavailable"; readonly id: string; readonly stamp?: string; readonly reason?: string }
export interface ConformanceFleetPortV1 { get?(id: string, key: string): Promise<unknown> | unknown; put?(id: string, key: string, record: unknown): Promise<unknown> | unknown }
export interface ImplementationRowV1 { readonly kind: string; readonly path: string; readonly fact: unknown; readonly scope?: InferenceScopeV1 }
export const component: "stamps";
export function wire(): void;
export function inferenceFragmentsV1(document: InferenceProtocolV1, stamps?: ReadonlyMap<string, string>): string[];
export function inferrerActiveV1(id: string, stamps?: ReadonlyMap<string, string>, document?: InferenceProtocolV1): boolean;
export function bindConformanceFleetPortV1(port: ConformanceFleetPortV1 | undefined): void;
export function conformanceDirectoryV1(environment?: Readonly<Record<string, string | undefined>>, platform?: string, home?: string): string;
export function settleConformanceStampsV1(options?: { libraryRoot: string; directory?: string; runners?: Readonly<Record<string, (fixtures: never) => Promise<unknown>>>; fleet?: ConformanceFleetPortV1; signal?: AbortSignal; say?: (text: string) => void; language?: "en" | "de"; timeoutMilliseconds?: number }): Promise<ConformanceSettlementV1[]>;
export function admitInferenceProtocolV1(value: unknown): InferenceProtocolV1;
export function inferenceStampEntriesV1(document: InferenceProtocolV1, stamps?: ReadonlyMap<string, string>): [string, string][];
export function inferenceProtocolV1(): InferenceProtocolV1;
export function inferrerStampPathV1(id: string): string;
export function inferrerStampV1(id: string): string;
export function projectStampPathsV1(flags: { native: boolean }): string[];
export function matchesFragmentsV1(path: string, fragments: readonly string[]): boolean;
export function classifyImplementationRowsV1(rows: readonly ImplementationRowV1[], document?: InferenceProtocolV1, stamps?: ReadonlyMap<string, string>): ImplementationRowV1[];
export function unclassifiedCodeV1(rows: readonly ImplementationRowV1[], document?: InferenceProtocolV1): string[];
