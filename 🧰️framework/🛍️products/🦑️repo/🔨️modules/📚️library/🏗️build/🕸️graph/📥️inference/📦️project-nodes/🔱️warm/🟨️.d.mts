export const WARM_POLICY_V1: Readonly<{ directory: string; maximumAgeMilliseconds: number; progressMilliseconds: number; minimumOwners: number; parentMegabytes: number; childMegabytes: number }>;
export const WARM_OUTCOMES_V1: readonly ("settled" | "other" | "absent")[];
export const WARM_TEXTS_V1: Readonly<{ en: string; de: string }>;
export interface WarmRowV1 { v: 1; n: string; k: string; d: string; o: "settled" | "other" | "absent"; t: number; c?: number; m?: number }
export interface WarmIndexV1 { readonly size: number; has(name: string, key: string, stamp: string): boolean }
export interface WarmCountersV1 { total: number; done: number; settled: number; durable: number; failed: number; lanes: number; peak: number; why?: string }
export function warmDigestV1(text: string): string;
export function warmDirectoryV1(workspaceRoot: string): string;
export function readStoredRecordV1(path: string, workspaceRoot: string): { header: { digest: unknown; observations: readonly { kind: string; path: string }[] }; value: unknown } | undefined;
export function warmStampV1(path: string): string | undefined;
export function isWarmRowV1(row: unknown): row is WarmRowV1;
export function appendWarmRowV1(directory: string, row: WarmRowV1, pid?: number): void;
export function readWarmIndexV1(directory: string, options?: { now?: () => number; maximumAgeMilliseconds?: number }): WarmIndexV1;
export function sweepWarmPartsV1(directory: string, options?: { now?: () => number; maximumAgeMilliseconds?: number }): number;
export function planWarmLanesV1(owners: readonly { name: string; key: string; cost?: number }[], lanes: number): string[][];
export function planWarmWidthV1(input: { cores: number; grantedChildren?: number; farmLoops?: number; limitMegabytes?: number; parentMegabytes?: number; childMegabytes?: number; owners?: number }): number;
export function warmLineV1(language: string, fields: Partial<Record<string, number>>): string;
export function runWarmLanesV1(input: { why?: () => string; lanes: readonly (readonly string[])[]; inspect(name: string): { key: string; stamp?: string }; warm(name: string): Promise<{ peakMegabytes?: number } | undefined>; index?: Pick<WarmIndexV1, "has">; signal?: AbortSignal; why?: () => string; tell?(fields: WarmCountersV1): void; progressMilliseconds?: number; now?: () => number }): Promise<WarmCountersV1>;
export function warmOwnerV1(job: { name: string }, input: { directory: string; cachePath: string; ports: { read(path: string): { header: { digest: string; observations: readonly unknown[] }; value: unknown } | undefined; settle(observations: readonly unknown[], value: unknown): Promise<void>; keyText(job: unknown): string }; now?: () => number }): Promise<{ outcome: "settled" | "other" | "absent"; cpuMilliseconds: number }>;
export function warmWaveV1(input: { farm: { warmReasons?(): Readonly<Record<string, number>>; warm?(job: unknown, signal?: AbortSignal): Promise<{ peakMegabytes?: number } | undefined>; engage?(): void; whenReady(milliseconds: number, signal?: AbortSignal): Promise<boolean>; width(): number; workerMegabytes?(): number } | undefined; names: readonly string[]; describe(name: string): { key: string; stamp?: string; cost?: number; job: unknown }; directory: string; cores: number; limitMegabytes?: number; tell?(fields: WarmCountersV1): void; signal?: AbortSignal; environment?: Record<string, string | undefined>; waitMilliseconds?: number }): Promise<WarmCountersV1 | undefined>;
export function runWarmBesideTakeV1(warm: () => Promise<unknown>, take: () => Promise<unknown>): Promise<void>;
export function warmThroughV1<H>(input: { pick(): H | undefined; run(handle: H): Promise<{ reply?: { peakMegabytes?: number }; failure?: unknown; lost?: unknown }>; hopeless(): string | undefined; wait(milliseconds: number): Promise<unknown>; now?: () => number; deadlineMilliseconds?: number; attempts?: number }): Promise<{ reply?: { peakMegabytes?: number; cpuMilliseconds?: number }; why?: string }>;
