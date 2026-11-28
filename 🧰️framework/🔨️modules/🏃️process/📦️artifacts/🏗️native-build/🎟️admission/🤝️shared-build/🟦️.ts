import type { ChildProcess } from "node:child_process";
import { createSharedChildRegistryV1, sharedChildNodeHostV1 } from "../../../../🪓️termination/🤝️shared-child/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 📐️ The label under which the stops of a shared build are recorded.
 * @see ./🧬️schema/🔣️.json */
export const SHARED_BUILD_POLICY_V1 = schema.definitions.Policy.const;

/** 🎫️ One requester's ask: the key of the build, who asks, when it leaves, how a child is started (called only for the first requester) and how its outcome is read (called once, for that child). */
export interface SharedBuildAskV1<Outcome> {
  readonly key: string;
  readonly requester: string;
  readonly signal?: AbortSignal;
  readonly start: () => ChildProcess;
  readonly collect: (child: ChildProcess) => Promise<Outcome>;
}

/** 🤝️ The builds in flight of one process. */
export interface SharedBuildsV1 {
  readonly run: <Outcome>(ask: SharedBuildAskV1<Outcome>) => Promise<Outcome>;
  readonly requesters: (key: string) => readonly string[];
  readonly size: () => number;
}

/** 🤝️ Builds shared by requesters: the first requester of a key starts the child, later ones join it; a requester that leaves is refused with its signal's reason and never stops a build another requester still needs; the last one out stops the tree, every signal recorded through `write`.
 * @see ./🧬️schema/🔣️.json */
export function createSharedBuildsV1(options: Readonly<{ label?: string; write?: (line: string) => void }> = {}): SharedBuildsV1 {
  const registry = createSharedChildRegistryV1<ChildProcess>(sharedChildNodeHostV1(options.label ?? SHARED_BUILD_POLICY_V1.label, options.write));
  const outcomes = new WeakMap<ChildProcess, Promise<unknown>>();
  return {
    run<Outcome>(ask: SharedBuildAskV1<Outcome>): Promise<Outcome> {
      const lease = registry.acquire(ask.key, ask.requester, () => {
        const child = ask.start();
        const outcome = ask.collect(child);
        outcome.catch(() => undefined);
        outcomes.set(child, outcome);
        return child;
      });
      const outcome = outcomes.get(lease.child) as Promise<Outcome>;
      return new Promise<Outcome>((accept, reject) => {
        let settled = false;
        const finish = (settle: () => void): void => {
          if (settled) return;
          settled = true;
          ask.signal?.removeEventListener("abort", leave);
          settle();
          void lease.release();
        };
        const leave = (): void => finish(() => reject(ask.signal?.reason ?? new Error("cancelled")));
        if (ask.signal?.aborted) return leave();
        ask.signal?.addEventListener("abort", leave, { once: true });
        outcome.then((value) => finish(() => accept(value)), (error) => finish(() => reject(error)));
      });
    },
    requesters: (key) => registry.requesters(key),
    size: () => registry.size(),
  };
}
