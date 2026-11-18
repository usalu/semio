export type TakeOutcomeV1 = "present" | "fetched" | "stale" | "miss" | "fault";
export type TakePolicyV1 = Readonly<{ concurrency: number; pollFirstMilliseconds: number; pollGrowthPercent: number; pollMaximumMilliseconds: number; pollBatch: number; faultRetries: number; faultWaitMilliseconds: number; progressMilliseconds: number; passBoundMilliseconds: number; releaseMilliseconds: number; noProgressMilliseconds: number; tailReportMilliseconds: number; watchMilliseconds: number; strongMemoryMegabytes: number; strongCores: number; cpuProgressPercent: number }>;
export type TakeCountsV1 = Readonly<{ total: number; present: number; fetched: number; here: number; stale: number; running: number; wanted: number; done: number; missing: number }>;
/** ⚙️ The authored numbers of the take pass. */
export const TAKE_POLICY_V1: TakePolicyV1;
/** ⏱️ The pause before look number `round` at the cache. */
export function takePollWaitV1(round: number, policy?: TakePolicyV1): number;
/** 📒️ What a reader knows about its owners while it resolves them. */
export function createTakeLedgerV1(total: number): Readonly<{ mark(name: string, outcome: TakeOutcomeV1 | "inferred"): void; want(names: Iterable<string>): void; run(delta: number): void; state(name: string): string | undefined; counts(): TakeCountsV1; line(language: string): string; waveLine(language: string): string }>;
/** 💪️ Whether a machine is large enough never to wait for another. */
export function takeStrongMachineV1(input: Readonly<{ memoryMegabytes: number; cores: number }>, policy?: TakePolicyV1): boolean;
/** 🚦️ A watch over the progress of a read. */
export function createProgressWatchV1(input?: Readonly<{ now?(): number; windowMilliseconds?: number }>): Readonly<{ check(sign: unknown): Readonly<{ sinceMilliseconds: number }> | undefined }>;
export function takeCpuMillisecondsV1(text: string): number;
export function takeTreeCpuMillisecondsV1(rows: readonly Readonly<{ pid: number; parent: number; milliseconds: number }>[], root: number): number;
export function takeChildrenCpuV1(root?: number, run?: (...input: any[]) => unknown): Promise<number | undefined>;
export function createCpuProgressV1(input: Readonly<{ sample(): Promise<number | undefined> | number | undefined; now?(): number; policy?: Readonly<{ cpuProgressPercent: number }> }>): Readonly<{ tick(): Promise<number> }>;
/** 🗣️ The plain text of a read that stopped for lack of progress. */
export function takeNoProgressTextV1(language: string, fields: Readonly<Record<string, string | number>>): string;
/** 🔢 The looks a lane runs at once under the cut of the process. */
export function takeConcurrencyV1(cutConcurrency?: number, policy?: TakePolicyV1): number;
/** 🚦 A look admitted by the footprint gate of the process. */
export function takeAdmittedV1<Name, Outcome>(attempt: (name: Name) => Promise<Outcome>, gate: () => Readonly<{ admit(signal?: AbortSignal): Promise<unknown> }> | undefined, signal?: AbortSignal, state?: { flying: number }): (name: Name) => Promise<Outcome>;
/** 🔍️ Looks at every name once, and again those the cache did not answer. */
export function takeLanesV1(names: readonly string[], isPresent: (name: string) => boolean): string[][];
/** 🐘️ The names of a pass in the light group and the heavy group (looked at one at a time, after the light ones). */
export function takeSplitV1(names: readonly string[], isHeavy: (name: string) => boolean): Readonly<{ light: string[]; heavy: string[] }>;
export function takePassV1(input: Readonly<{ names: readonly string[]; attempt(name: string): Promise<TakeOutcomeV1>; concurrency?: number; faultRetries?: number; faultWaitMilliseconds?: number; sleep?(milliseconds: number): Promise<void>; signal?: AbortSignal; mark?(name: string, outcome: TakeOutcomeV1): void }>): Promise<ReadonlyMap<string, TakeOutcomeV1>>;
/** 🔁️ Looks again at the cache for the owners that are still missing, with a growing pause. */
export function takePollV1(input: Readonly<{ concurrency?: number; queue(): readonly string[]; attempt(name: string): Promise<TakeOutcomeV1>; arrived(name: string, outcome: TakeOutcomeV1): void | Promise<void>; stop(): boolean; signal?: AbortSignal; sleep?(milliseconds: number, signal?: AbortSignal): Promise<void>; policy?: TakePolicyV1 }>): Promise<Readonly<{ looks: number; found: number }>>;
/** 🚀️ Runs the take pass and the inference of the owners beside each other: the inference is released when the pass ends or after `releaseMilliseconds`, never later, and the warm pass is stopped then. */
export function runTakeBesideInferV1(input: Readonly<{ take(): Promise<unknown>; warm?(): Promise<unknown>; stopWarm?(): void; release(): void; infer(): Promise<unknown>; end?(): void; releaseMilliseconds?: number; sleep?(milliseconds: number, signal?: AbortSignal): Promise<void> }>): Promise<void>;

export const TAKE_PROGRESS_KEYS_V1: readonly string[];
/** 🧮️ The sign of the progress watch, made of the declared progress facts only. */
export function takeProgressSignV1(row: Readonly<Record<string, unknown>>): string;
/** 🪧️ A reporter that speaks at most once per window of no change. */
export function createTailReporterV1(input?: Readonly<{ now?(): number; windowMilliseconds?: number }>): Readonly<{ check(sign: unknown): Readonly<{ sinceMilliseconds: number }> | undefined }>;
export type TakeMissingReasonV1 = "heavy" | "running" | "parked" | "fleet";
export type TakeMissingRowV1 = Readonly<{ name: string; reason: TakeMissingReasonV1; megabytes?: number; limitMegabytes?: number; seconds?: number }>;
/** 📋️ The reason of every missing owner. */
export function takeMissingRowsV1(names: readonly string[], input: Readonly<{ never: ReadonlySet<string>; running: ReadonlyMap<string, number>; deferred: ReadonlySet<string>; megabytes?(name: string): number | undefined; limitMegabytes?: number }>): readonly TakeMissingRowV1[];
/** ⚖️ Whether a stall is the typed heavy refusal or a generic stall. */
export function takeStallKindV1(rows: readonly TakeMissingRowV1[]): "heavy" | "stalled";
/** 🗣️ The plain text of the tail of a read, in one language. */
export function takeTailTextV1(language: string, input: Readonly<{ minutes: number; done: number; total: number; rows: readonly TakeMissingRowV1[]; pool?: Readonly<{ state: string; children: number; running: number; ownersPerMinute: number; text: Readonly<{ en: string; de: string }> | null }> }>, limit?: number): string;
