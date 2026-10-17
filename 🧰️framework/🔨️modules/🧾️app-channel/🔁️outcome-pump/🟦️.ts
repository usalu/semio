/** 🔁️ App channel outcome pump: the one background loop that reads this instance's turn outcomes and routes every frame to its waiter, subscriber or query. */

import type { TurnOutcome } from "@semio-tech/framework";
import type { AppFrameValue } from "../../🪢️wire-contracts/🟦️.ts";
import { decodeAppFrame } from "../🎞️frame-codec/🟦️.ts";
import type { ChannelLifecycle } from "../🔌️channel-lifecycle/🟦️.ts";
import type { DocumentPackCache } from "../📦️document-pack-cache/🟦️.ts";
import type { UnsolicitedFeed } from "../📣️unsolicited-feed/🟦️.ts";
import type { ReplyLedger } from "../📮️reply-ledger/🟦️.ts";
import type { LocalInteractionQuery } from "../🏠️local-interaction-query/🟦️.ts";
import type { AppChannelHandle } from "../🔢️request-sequence/🟦️.ts";

//#region 🔖️AppChannelClient
/** 🌊️ Owns this client's subscription against the handle-wide {@link PluginWasmHandle.outcomes} stream for its whole lifetime. */
export class OutcomePump {
  private readonly instanceId: number;
  private readonly lifecycle: ChannelLifecycle;
  private readonly ledger: ReplyLedger;
  private readonly localQuery: LocalInteractionQuery;
  private readonly cache: DocumentPackCache;
  private readonly feed: UnsolicitedFeed;
  private readonly outcomeIterator: AsyncIterator<TurnOutcome>;

  constructor(handle: AppChannelHandle, instanceId: number, lifecycle: ChannelLifecycle, ledger: ReplyLedger, localQuery: LocalInteractionQuery, cache: DocumentPackCache, feed: UnsolicitedFeed) {
    this.instanceId = instanceId;
    this.lifecycle = lifecycle;
    this.ledger = ledger;
    this.localQuery = localQuery;
    this.cache = cache;
    this.feed = feed;
    this.outcomeIterator = handle.outcomes[Symbol.asyncIterator]();
  }

  /** ▶️ Starts the loop; it runs until {@link finishDisposal} ends the iterator or the handle completes it. */
  start(): void {
    void this.run();
  }

  /** 🏁️ Ends the subscription once the client is disposed and nothing it still owes a reply to is left — the iterator's own `return()` is what breaks the loop. */
  finishDisposal(): void {
    if (this.lifecycle.disposed && this.localQuery.idle && this.ledger.size === 0) void this.outcomeIterator.return?.();
  }

  /** 🔂️ Every outcome for a DIFFERENT
   * `instanceId` is silently skipped (it belongs to a sibling `AppChannelClient` on the same handle);
   * stale or duplicate numeric receipts do not consume another command's waiter. Query pages retain
   * their separate ACK-owned lifecycle even when coalesced with ordinary replies or notifications. */
  private async run(): Promise<void> {
    for (;;) {
      const step = await this.outcomeIterator.next();
      if (step.done) {
        this.cache.forget();
        this.localQuery.finish(new Error("local-interaction.channel-closed"));
        this.ledger.closeAll();
        return;
      }
      const outcome = step.value;
      if (outcome.instanceId !== this.instanceId) continue;
      if ("error" in outcome) {
        this.localQuery.finish(outcome.error);
        this.ledger.rejectOldest(outcome.error);
        continue;
      }
      const frames: AppFrameValue[] = [];
      for (const encoded of outcome.frames) {
        try { frames.push(decodeAppFrame(encoded)); }
        catch (error) {
          this.localQuery.cancel(error);
          this.ledger.rejectCommands(error);
        }
      }
      const ordinary: AppFrameValue[] = [];
      for (const frame of frames) {
        if ("Ephemeral" in frame) this.feed.observeEphemeral(frame.Ephemeral);
        if ("LocalInteractionQuery" in frame) this.localQuery.receive(frame.LocalInteractionQuery.reply);
        else if ("OperationCompleted" in frame) this.feed.publishCompletion(frame.OperationCompleted);
        else if ("Invocation" in frame && frame.Invocation.in_reply_to === 0 && (frame.Invocation.ui_scope.length > 0 || frame.Invocation.history_patch.length > 0)) this.feed.publishProgress(frame.Invocation);
        else ordinary.push(frame);
      }
      this.ledger.settle(ordinary, (reply) => this.cache.capture(reply));
      this.finishDisposal();
    }
  }
}
//#endregion 🔖️AppChannelClient
