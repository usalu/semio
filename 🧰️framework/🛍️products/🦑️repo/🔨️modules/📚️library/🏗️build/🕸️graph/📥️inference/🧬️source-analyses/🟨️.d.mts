/** 🧭️ Types of the source-analyses component: parsed source facts stay structural data (their rows are untrusted until a producer admits them). */
export type SourceAnalysesValueV1 = any;
export const component: "source-analyses";
export let TAXONOMY_PROJECTION: SourceAnalysesValueV1;
export let TAXONOMY: SourceAnalysesValueV1;
export function wire(registry: SourceAnalysesValueV1): void;
export function readTaxonomyProjectionV1(path: string): SourceAnalysesValueV1;
export function createRustSourceCache(limit?: number): SourceAnalysesValueV1;
export const RUST_SOURCE_CACHE: SourceAnalysesValueV1;
export const RUST_INPUT_SNAPSHOTS: WeakMap<object, SourceAnalysesValueV1>;
export function sourceAnalysisOwnerIdentityV1(url: string, revision: string): string;
export function readCurrentCompilerSourceV1(select: SourceAnalysesValueV1, read: SourceAnalysesValueV1): SourceAnalysesValueV1;
export function prepareSourceAnalysesV1(): Readonly<Record<string, { id: string }>>;
