/** 📜️ The name under which the generator contracts travel as a `global` row. */
export const GENERATOR_CONTRACTS_FACT_V1: string;
/** 🔭️ A view of the generator contracts that remembers what an inference reads of them. */
export function trackGeneratorContractsV1<Contracts extends object>(contracts: Contracts, onRead?: (segments: readonly string[], fact: unknown) => void): Readonly<{ view: Contracts; tree(): Record<string, unknown> }>;
/** ✅️ Whether one read of the contracts has the same answer in `contracts`. */
export function generatorContractReadHoldsV1(contracts: unknown, segments: readonly string[], fact: unknown): boolean;
/** ✅️ Whether everything an inference read of the contracts has the same answer in `contracts`. */
export function generatorContractReadsHoldV1(contracts: unknown, tree: unknown): boolean;
/** 🧭️ Runs `work` with `contracts` as the contracts of the asynchronous scope. */
export function withGeneratorContractsScopeV1<T>(contracts: unknown, work: () => T): T;
/** 🧭️ The contracts of the running scope, `fallback` outside any. */
export function generatorContractsInScopeV1<T>(fallback: T): T;
/** 🔑️ The head of an owner record key. */
export function ownerRecordHeadV1(globals: Readonly<{ registryDigest: string; commandRevision: string | null; late: boolean }>, owner: Readonly<{ native: boolean; pipeline: boolean; assemblyAuthority: string | null | undefined }>): (string | null)[];
