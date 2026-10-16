/** 📡️ The multicast queue of one instance's turn replies: the outcome type and `createTurnOutcomeBroadcast` with its own and force-close contract.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

// #region 🎠️Kernel
//#region 🔖️TurnOutcomeBroadcast
/** 📨️ One instance's reply to whatever {@link PluginWasmHandle.enqueue} most recently queued for
 * it — the async-stream replacement for the old handle's synchronous `(instanceId, frames) ->
 * Promise<frames>` per-call RPC shape (`📌️important.md`'s "Replace, never wrap" list — the removed
 * method's name is deliberately not repeated here, see that list). That old shape assumed a command's
 * reply always lands on the SAME call that sent it; under
 * the turn model a reply may arrive N turns later, so `enqueue` returns nothing and a caller
 * correlates against this stream instead — `AppChannelClient` (`💻️os/🟦️.ts`) is the
 * host-side correlator (FIFO per `instanceId`, matching every real call site's own sequential-await
 * usage today). `frames` mirrors what the old method used to resolve with directly; `error` covers
 * what used to REJECT that promise (a turn submission failure, e.g. a trapped actor — an
 * `AppFrame::Error` frame is still an ordinary `frames` entry, decoded by the caller exactly as
 * before). */
export type TurnOutcome = { readonly instanceId: number; readonly frames: readonly Uint8Array[] } | { readonly instanceId: number; readonly error: unknown };

/** 📡️ Multicast queue backing {@link PluginWasmHandle.outcomes}: every independent
 * `[Symbol.asyncIterator]()` call (one per live `AppChannelClient`) gets its OWN subscription fed
 * every {@link push}ed value, rather than several callers racing to drain one shared FIFO — required
 * because more than one live instance's client iterates the SAME handle-wide stream at once, each
 * filtering to its own `instanceId`. A subscriber unregisters itself the instant its iterator's
 * `return()` is called (what `for await...of`'s `break`/an uncaught throw triggers automatically, and
 * what `AppChannelClient.dispose()` calls explicitly on teardown); {@link complete} force-closes every
 * still-live subscriber at once, for {@link PluginWasmHandle.dispose}. */
export function createTurnOutcomeBroadcast<T>(): { readonly push: (value: T) => void; readonly complete: () => void; readonly stream: AsyncIterable<T> } {
  const subscribers = new Set<{ queue: T[]; resolve: ((result: IteratorResult<T>) => void) | null }>();
  return {
    push: (value) => {
      for (const subscriber of subscribers) {
        if (subscriber.resolve) {
          const resolve = subscriber.resolve;
          subscriber.resolve = null;
          resolve({ value, done: false });
        } else {
          subscriber.queue.push(value);
        }
      }
    },
    complete: () => {
      for (const subscriber of subscribers) subscriber.resolve?.({ value: undefined as unknown as T, done: true });
      subscribers.clear();
    },
    stream: {
      [Symbol.asyncIterator](): AsyncIterator<T> {
        const subscriber: { queue: T[]; resolve: ((result: IteratorResult<T>) => void) | null } = { queue: [], resolve: null };
        subscribers.add(subscriber);
        return {
          next: (): Promise<IteratorResult<T>> => {
            if (subscriber.queue.length > 0) return Promise.resolve({ value: subscriber.queue.shift() as T, done: false });
            return new Promise<IteratorResult<T>>((resolve) => {
              subscriber.resolve = resolve;
            });
          },
          return: (): Promise<IteratorResult<T>> => {
            subscribers.delete(subscriber);
            return Promise.resolve({ value: undefined as unknown as T, done: true });
          },
        };
      },
    },
  };
}
//#endregion 🔖️TurnOutcomeBroadcast
// #endregion 🎠️Kernel
