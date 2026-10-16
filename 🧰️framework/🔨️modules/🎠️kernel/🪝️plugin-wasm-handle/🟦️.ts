/** 🪝️ The kernel's plugin handle: the dispatch hint and the `PluginWasmHandle` type every host adapts a loaded plugin to.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import type { PluginManifest } from "../../🛂️manifest/🟦️.ts";
import { type TurnOutcome } from "../📡️turn-outcome-broadcast/🟦️.ts";

// #region 🎠️Kernel
/** 🔗️ Optional steering for ONE dispatch into a plugin instance (INPUT-CAUSALITY-LEDGER §2 B, law
 * L2). `order` is the input ledger's causal key (`causalOrderKeyV1(provenance)` =
 * `causedBy ?? inputSeq`, `🏛️ShellHost/🎯️input-ledger/🟦️.ts`): a root input carries its own `inputSeq`,
 * a guest follow-up caused by input N carries N. The runtime hands it verbatim to the per-actor
 * mailbox as `MailboxEnvelope.order` (`🎭️actor/📬️mailbox/🟦️.ts` `## causal order`), so a follow-up of
 * input N dequeues before the already-queued input N+1 on the same actor. Absent ⇒ plain arrival
 * order, byte-for-byte the behaviour before this type existed. Every field is optional and every
 * consumer must accept `undefined`: the hint never changes WHAT is dispatched, only WHERE in one
 * actor's queue it lands. */
export type PluginDispatchHintV1 = Readonly<{ order?: number }>;

export type PluginWasmHandle = {
  /** 📇️ The admitted descriptor manifest of the plugin, as the loader read it — a value, never re-encoded per call. */
  readonly manifest: PluginManifest;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  /** 🧵 Takes one capped operation-owned export chunk; `undefined` is the exact terminal option. */
  readonly takeSegmentedDownloadChunk: (instanceId: number, operationId: bigint) => Promise<Uint8Array | undefined>;
  /** 📤️ Fire-and-forget: queues `events` (encoded `AppCommand` frames) for `instanceId`'s next turn
   * and returns immediately. The turn's result arrives later on {@link outcomes}, never as this call's
   * return value — replaces the old handle's synchronous per-call method, whose `Promise<Uint8Array[]>`
   * return shape wrongly assumed a reply always lands on the turn it was sent on (R2). `dispatch`
   * ({@link PluginDispatchHintV1}) is optional and only steers the queue position of this turn. */
  readonly enqueue: (instanceId: number, events: readonly Uint8Array[], dispatch?: PluginDispatchHintV1) => void;
  /** 📥️ Every live instance's turn outcomes, multicast (see {@link createTurnOutcomeBroadcast}) —
   * a caller filters to the `instanceId`(s) it owns. */
  readonly outcomes: AsyncIterable<TurnOutcome>;
  readonly dispose: () => Promise<void>;
};
// #endregion 🎠️Kernel
