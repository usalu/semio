/** 💾️ The durable store of closure entries. */
export type StoredClosureEntryV1 = Readonly<{ recipe: unknown; value: unknown; rows: unknown[]; digests: string[]; sidecar: unknown; bytes: number }>;
export type ClosureStoreV1 = Readonly<{ save(id: string, entry: { recipe: unknown; value: unknown; rows: unknown[]; layers: readonly unknown[][]; sidecar?: unknown }): Promise<{ bytes: number; digests: string[] | undefined }>; layers(digests: readonly string[]): unknown[][] | undefined; load(id: string): Promise<StoredClosureEntryV1 | undefined>; settled(): Promise<void> }>;
export function createClosureStoreV1(options: { directory: string; limits?: Record<string, number>; readOnly?: boolean }): ClosureStoreV1;
