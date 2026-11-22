export const ANALYSIS_STORE_POLICY_V1: Readonly<{ version: 1; domain: string; rootToken: string; shardCharacters: number; minimumSourceBytes: number; maximumBytes: number; lowWaterPercent: number; entryMaximumBytes: number; collectEveryWrites: number; brotliQuality: number }>;
/** 🔑️ The key of the analysis of one source, no machine path. */
export function analysisStoreKeyV1(root: string, providerId: string, providerRevision: string, path: string, contentSha256: string, contentBytes?: number): string | undefined;
export const neutralAnalysisTextV1: (text: string, root: string) => string | undefined;
export const localAnalysisTextV1: (text: string, root: string) => string;
/** 📖️ The analysis stored under a key, or undefined. */
export function readStoredAnalysisV1(directory: string, root: string, key: string): { image: string; nestedObservations: unknown[] } | undefined;
/** 💾️ Stores one analysis atomically. */
export function writeStoredAnalysisV1(directory: string, root: string, key: string, result: { image: string; nestedObservations: readonly unknown[] }): boolean;
/** 🧹️ Brings the directory under its budget. */
export function collectAnalysisStoreV1(directory: string, maximumBytes?: number, now?: number): number;
