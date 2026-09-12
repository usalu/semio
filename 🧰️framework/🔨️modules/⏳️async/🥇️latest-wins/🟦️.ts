//#region 🥇️LatestWins
/**
 * @emoji 🥇️ Single-flight with a trailing coalescer: the first call launches `run()` and hands the
 * caller that exact run's promise. Every subsequent call that arrives while a run is still in
 * flight collapses into at most one queued follow-up run — they all share that one follow-up's
 * promise, so N concurrent callers never produce more than one extra `run()` call, and every one of
 * them observes the latest result. Used for folder revalidation, presence, and refresh, where firing
 * a fresh request per caller would be wasteful and stale-by-the-time-it-lands anyway.
 * Einzelflug mit nachlaufender Zusammenführung: gleichzeitige Aufrufe während eines laufenden `run()`
 * teilen sich höchstens einen einzigen Folgeauf-ruf und dessen Ergebnis.
 */
export function latestWins<T>(run: () => Promise<T>): () => Promise<T> {
  let current: Promise<T> | null = null;
  let queued: Promise<T> | null = null;

  function launch(): Promise<T> {
    let promise: Promise<T>;
    try {
      promise = Promise.resolve(run());
    } catch (error) {
      promise = Promise.reject(error);
    }
    current = promise;
    const advance = (): void => {
      if (current === promise) current = null;
      if (queued !== null) queued = null;
    };
    promise.then(advance, advance);
    return promise;
  }

  return function trigger(): Promise<T> {
    if (current === null) return launch();
    if (queued === null) queued = current.then(launch, launch);
    return queued;
  };
}
//#endregion 🥇️LatestWins
