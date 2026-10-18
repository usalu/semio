/** ✂️ The generated shard worker is an ES module that imports the WASI monotonic clock from its vendored preview2 shim; a `node:vm` classic script cannot import, so the suites that run it in a context bind a fixed clock instead of the import.
 * @see ../../🌐️browser-bundle/🏗️materialization/🟦️.ts — `shardWorkerSource` */
export function classicShardWorkerSource(source: string): string {
  const clockImport = /^import \{ monotonicClock \} from "[^"]*";$/mu;
  if (!clockImport.test(source)) throw new Error("shard-worker.clock-import-missing");
  return source.replace(clockImport, "const monotonicClock = { now: () => 5_000_000n };");
}

/** ✂️ The original operation source a generated worker's turn drive reads from its actor's api; a fake api that answers `poll` supplies this beside it.
 * @see ../../🌐️browser-bundle/🏗️materialization/🟦️.ts — `shardWorkerSource`'s `turn` case */
export const vmOriginalSource = Object.freeze({ deadlineUs: 10_000_000n, operationClockUs: (): bigint => 5_000_000n, fuelRemaining: 1 });
