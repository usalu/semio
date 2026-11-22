/** 🧾️ The name under which what a read asked of the owner registry travels as a `global` row. */
export const OWNER_REGISTRY_FACT_V1: string;
/** 🔑️ The digest of one owner row, the order of its lists never counting. */
export function ownerRowDigestV1(owner: Readonly<{ name: string; root: string; manifest: string; packages: readonly string[]; nativeRoots?: readonly string[] }>): string;
/** 🗝️ The digest of a whole registry: the row digests by name and the catalog providers. */
export function ownerRegistryDigestV1(registry: Readonly<{ owners: readonly { name: string; root: string; manifest: string; packages: readonly string[]; nativeRoots?: readonly string[] }[]; catalogProviders: readonly { ownerRoot: string; sourcePath: string; consumerRoots: readonly string[] }[] }>): string;
/** 🗝️ The digest of `registry` computed once per registry object; null for no registry. */
export function ownerRegistryDigestOnceV1(registry: any): string | null;
/** 📖️ The admitted registry of the checkout as it is now. */
export function currentOwnerRegistryV1(workspaceRoot: string): any;
/** 🧩️ Registers the answer of one more question kind. */
export function registerOwnerRegistryQuestionV1(kind: string, answer: (registry: any, args: readonly any[], workspaceRoot: string) => unknown): void;
/** ❓️ What `registry` answers to one question, `segments` = [kind, ...arguments]. */
export function answerOwnerRegistryQuestionV1(registry: any, segments: readonly unknown[], workspaceRoot?: string): unknown;
/** 🖊️ Asks `registry` one question and records the question and its answer as a row of the running observation. */
export function askOwnerRegistryV1(workspaceRoot: string, registry: any, segments: readonly unknown[]): unknown;
/** 🔭️ The lookups of the owner discovery over `registry`, each answer recorded. */
export function observedOwnerRegistryViewV1(workspaceRoot: string, registry: any): Readonly<{ byName: { has(name: string): boolean }; byPackage: { get(name: string): string | undefined }; byRoot: { get(root: string): string | undefined; readonly size: number }; relativeProject(path: string): string | undefined }>;
/** ✍️ Records an answer the caller already holds as a row of the running observation. */
export function observeOwnerRegistryAnswerV1(workspaceRoot: string, segments: readonly unknown[], answer: unknown): void;
