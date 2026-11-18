export type LookPolicyV1 = Readonly<{ maximumOwnersPerJob: number; jobsPerChild: number; leadJobsPerChild: number; assumedChildren: number; freshMilliseconds: number }>;
export type LookReplyV1 = Readonly<{ name: string; native: boolean; pipeline: boolean; head: readonly unknown[]; verdict?: string; headerDigest?: string; refs?: unknown; failed?: string }>;
/** ⚙️ The authored numbers of the owner looks. */
export const LOOK_POLICY_V1: LookPolicyV1;
/** 🔭️ How many owners one look job holds. */
export function lookBatchSizeV1(owners: number, children: number, policy?: LookPolicyV1): number;
/** 🔭️ The batches of the lanes of a take pass, one of each lane in turn. */
export function lookBatchesV1(lanes: readonly (readonly string[])[], size: number): string[][];
/** 🔭️ The line a read says when it queues its looks. */
export function lookQueuedTextV1(language: string, fields: Readonly<Record<string, number>>): string;
/** 🔭️ The queue of owner looks of a read. */
export function createLookQueueV1(options: { farm: { width(): number; observe(job: object): Promise<{ looks?: readonly object[] } | undefined> }; jobOf: (name: string) => object; shared?: object | (() => object); refuse: (name: string, reason: string) => Error; say?: (text: (language: string) => string) => void; policy?: LookPolicyV1; now?: () => number }): Readonly<{ start(lanes: readonly (readonly string[])[]): void; kick(): void; ask(name: string): Promise<LookReplyV1 | undefined>; stats(): Readonly<Record<string, number>> }>;
