/** 📣️ App channel unsolicited feed: what a guest pushes without being asked, delivered to subscribers (operation completion, mid-operation progress and the latest ephemeral publication). */

import { decodePackWire } from "../../🎁️packs/🟦️.ts";
import type { AppFrameValue, OperationCompletionV1 } from "../../🪢️wire-contracts/🟦️.ts";
import type { ChannelLifecycle } from "../🔌️channel-lifecycle/🟦️.ts";

//#region 🔖️AppChannelClient
/** 🎞️ One unsolicited mid-operation `Invocation` frame as {@link UnsolicitedFeed.onProgress} hands it on: the
 * decoded `UiDirtyScope` and `HistoryPatch` it carries, each absent when the frame carried none. */
export type AppChannelOperationProgressV1 = { readonly uiScope?: unknown; readonly historyPatch?: unknown };

/** 🫧️ The presence pack, interaction slice and tool-run and history-edit summaries of the latest ephemeral publication, as {@link UnsolicitedFeed.ephemeral} hands them on. */
export type AppChannelEphemeralV1 = { readonly presence: readonly number[]; readonly presenceGeneration: number; readonly transientGeneration: number; readonly interaction: readonly number[]; readonly toolRun: readonly number[]; readonly historyEdit: readonly number[] };

/** 🪞️ The latest `AppFrame::Ephemeral` payload as the guest published it. */
export type EphemeralFrame = Extract<AppFrameValue, { readonly Ephemeral: unknown }>["Ephemeral"];

/** 🔔️ Owns the subscriber sets and the last ephemeral publication of one app instance. A throwing subscriber never starves its siblings nor the outcome pump. */
export class UnsolicitedFeed {
  private readonly instanceId: number;
  private readonly lifecycle: ChannelLifecycle;
  private readonly completionListeners = new Set<(completion: OperationCompletionV1) => void>();
  private readonly progressListeners = new Set<(progress: AppChannelOperationProgressV1) => void>();
  /** 🫂️ The last `AppFrame::Ephemeral` any outcome of this instance carried (every guest exchange appends one,
   * contract-freeze §C7.6) — the presence heartbeat's pack and interaction slice, read without a round trip (the
   * native twin: `ProgramBridge::observe_ephemeral`). */
  private lastEphemeral: EphemeralFrame | null = null;

  constructor(instanceId: number, lifecycle: ChannelLifecycle) {
    this.instanceId = instanceId;
    this.lifecycle = lifecycle;
  }

  /** 👥️ This instance's last published presence pack, interaction slice and tool-run and history-edit summaries (each
   * summary empty while none is open), `null` until the guest published one. */
  ephemeral(): AppChannelEphemeralV1 | null {
    const frame = this.lastEphemeral;
    return frame === null ? null : { presence: frame.presence, presenceGeneration: frame.presence_generation, transientGeneration: frame.transient_generation, interaction: frame.interaction, toolRun: frame.tool_run, historyEdit: frame.history_edit };
  }

  /** 👁️ Remembers the newest ephemeral publication. */
  observeEphemeral(frame: EphemeralFrame): void {
    this.lastEphemeral = frame;
  }

  /** 🏁️ Subscribes to this instance's unsolicited typed-operation completions. A mounted operation
   * finishes turns after the command that started it already resolved, so its terminal
   * `AppFrame::OperationCompleted` matches no pending waiter and would otherwise be dropped — this is
   * the ONLY delivery path for it. Returns the unsubscribe. */
  onCompleted(listener: (completion: OperationCompletionV1) => void): () => void {
    this.completionListeners.add(listener);
    return () => this.completionListeners.delete(listener);
  }

  /** 📈️ Subscribes to this instance's unsolicited progress: the `UiDirtyScope` a running operation (a tool run's trace,
   * provisional pieces and progress) and the `HistoryPatch` a running history replay (its stage and `done/total`) hand
   * the shell mid-operation as an `Invocation` frame answering no command. It is best effort — the shell refreshes as
   * often as it keeps up with — and the ONLY delivery path for either once the operation runs on the background drain.
   * Returns the unsubscribe. */
  onProgress(listener: (progress: AppChannelOperationProgressV1) => void): () => void {
    this.progressListeners.add(listener);
    return () => this.progressListeners.delete(listener);
  }

  /** 🧹️ Forgets every completion subscriber (the retiring client no longer reports completions). */
  clearCompletions(): void {
    this.completionListeners.clear();
  }

  /** 📡️ Decodes one progress frame's pack-encoded carriers and fans it out to every subscriber. */
  publishProgress(frame: { readonly ui_scope: readonly number[]; readonly history_patch: readonly number[] }): void {
    if (this.lifecycle.disposed || this.progressListeners.size === 0) return;
    const progress: AppChannelOperationProgressV1 = {
      ...(frame.ui_scope.length > 0 ? { uiScope: decodePackWire(new Uint8Array(frame.ui_scope), "$.uiScope") } : {}),
      ...(frame.history_patch.length > 0 ? { historyPatch: decodePackWire(new Uint8Array(frame.history_patch), "$.historyPatch") } : {}),
    };
    for (const listener of [...this.progressListeners]) {
      try { listener(progress); }
      catch (error) { console.error("[TRACE] operation progress subscriber failed", error); }
    }
  }

  /** 🔀️ Decodes one completion frame's pack-encoded carriers and fans it out to every subscriber.
   * A throwing subscriber never starves its siblings nor the outcome pump. */
  publishCompletion(frame: Extract<AppFrameValue, { readonly OperationCompleted: unknown }>["OperationCompleted"]): void {
    if (this.lifecycle.disposed || this.completionListeners.size === 0) return;
    const completion: OperationCompletionV1 = {
      instanceId: this.instanceId,
      operation: frame.operation,
      revision: frame.revision,
      uiScope: frame.ui_scope.length ? decodePackWire(new Uint8Array(frame.ui_scope), "$.uiScope") : undefined,
      historyPatch: frame.history_patch.length ? decodePackWire(new Uint8Array(frame.history_patch), "$.historyPatch") : undefined,
    };
    for (const listener of [...this.completionListeners]) {
      try { listener(completion); }
      catch (error) { console.error("[TRACE] operation completion subscriber failed", error); }
    }
  }
}
//#endregion 🔖️AppChannelClient
