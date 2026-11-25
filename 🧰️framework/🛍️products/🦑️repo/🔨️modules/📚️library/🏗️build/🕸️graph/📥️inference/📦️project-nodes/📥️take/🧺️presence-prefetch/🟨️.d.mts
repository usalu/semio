import type { TakePolicyV1 } from "../🟨️.mjs";
export type TakePrefetchCountsV1 = Readonly<{ records: number; skipped: number; names: number; present: number; asks: number; failed: number; milliseconds: number }>;
/** 🪶️ The result of one stored project record without its rows, or undefined. */
export function takeStoredRecordResultV1(cachePath: string, read?: (path: string) => Uint8Array): unknown;
/** 🧺️ Verifies the inventory names of every stored record of a take pass in a few asks before its looks ask one by one. */
export function prefetchTakePresenceV1(input: Readonly<{ records: Iterable<string>; capture(value: unknown): readonly string[]; verify(names: readonly string[]): Promise<Readonly<{ present: number }> | undefined>; signal?: AbortSignal; progress?(counts: TakePrefetchCountsV1): void; read?(cachePath: string): unknown; policy?: TakePolicyV1; now?(): number }>): Promise<TakePrefetchCountsV1>;
