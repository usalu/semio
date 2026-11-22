/** 🩹️ Removes the records that name a vanished build input closure leaf. */
export function purgeClosureReferencesV1(options: { root: string; key: string; dryRun?: boolean; cancelled?: () => boolean }): Promise<number>;
export const CLOSURE_NAMESPACES_V1: Readonly<Record<"SOURCE_CLOSURE" | "CARGO_LOCK", "build-input-closures" | "build-lock-closures">>;
export const CLOSURE_POLICY_V1: Readonly<{ maximumLeafBytes: number; concurrency: number; fragments: string }>;
export type ClosureNamespaceV1 = "build-input-closures" | "build-lock-closures";
export type ClosureObtainV1 = (namespace: ClosureNamespaceV1, key: string) => Promise<"present" | "refetched" | "missing">;
/** 🧭️ The closure leaves a text names, key to namespace. */
export function closureReferencesOfTextV1(text: string): Map<string, ClosureNamespaceV1>;
export function closureReferencePairsOfTextV1(text: string): IterableIterator<[string, ClosureNamespaceV1]>;
/** 🚫️ The typed failure of an absent closure leaf (code `closure-missing`, never ENOENT). */
export function closureMissingErrorV1(input: { namespace: string; key: string; projects?: readonly string[]; cause?: unknown }): Error & { code: "closure-missing"; missingClosure: string; missingNamespace: string; missingProjects: readonly string[] };
/** 🗄️ Whether the leaf is on this machine and its bytes match its sha256 name. */
export function closureLeafPresentV1(directory: string, key: string, maximumBytes?: number): boolean;
/** 🧲️ Obtains one closure leaf: present, refetched from the fleet cache, or missing. */
export function obtainClosureLeafV1(input: { directory: (namespace: ClosureNamespaceV1) => string; namespace: ClosureNamespaceV1; key: string; fetch?: (namespace: ClosureNamespaceV1, key: string, signal?: AbortSignal) => Promise<Uint8Array | undefined>; put?: (namespace: ClosureNamespaceV1, key: string, bytes: Uint8Array) => Promise<void>; signal?: AbortSignal; verify?: (namespace: ClosureNamespaceV1, key: string) => Promise<boolean>; maximumBytes?: number }): Promise<"present" | "refetched" | "missing">;
/** 📖️ Reads one closure leaf, obtaining it when absent. */
export function readClosureLeafV1<T>(input: { namespace: string; key: string; read: () => T; obtain?: ClosureObtainV1; projects?: readonly string[] | (() => readonly string[]) }): Promise<T>;
/** 🩹️ The closure leaves the named fragments name. */
export function closuresOfFragmentsV1(input: { names: readonly string[]; read: (name: string) => Promise<Uint8Array | undefined>; memo?: Map<string, [string, ClosureNamespaceV1][]>; concurrency?: number; cancelled?: () => boolean }): Promise<Map<string, { namespace: ClosureNamespaceV1; fragments: Set<string> }>>;
/** 👥️ The owners that name each leaf. */
export function closureOwnersV1(named: ReadonlyMap<string, { namespace: ClosureNamespaceV1; fragments: ReadonlySet<string> }>, owners: Readonly<Record<string, { fragment: string }>>): Map<string, { namespace: ClosureNamespaceV1; projects: string[] }>;
/** 📣️ The log line of an absent leaf. */
export function closureMissingLineV1(input: { key: string; count: number; outcome: string }): string;
/** 🧰️ Settles one absent leaf: refetched, re-derived or asked of the fleet. */
export function settleClosureLeafV1(input: { key: string; namespace: ClosureNamespaceV1; projects?: readonly string[]; obtain: ClosureObtainV1; purge: (key: string) => Promise<unknown>; heavy?: (projects: readonly string[]) => Promise<boolean> | boolean; say?: (line: string) => void }): Promise<"refetched" | "re-derived" | "asked of the fleet">;
/** 💾️ Writes a verified leaf under the writer custody of the graph store. */
export function closureLeafPutterV1(workspaceRoot: string): (namespace: ClosureNamespaceV1, key: string, bytes: Uint8Array) => Promise<void>;
