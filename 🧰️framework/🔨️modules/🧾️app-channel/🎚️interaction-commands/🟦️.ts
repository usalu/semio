/** 🎚️ App channel interaction commands: the app-specific command, its configuration, context menus, envelope ingest and the artifact-opening defaults. */

import type { PluginDispatchHintV1 } from "@semio-tech/framework";
import type { MutationEnvelope } from "@semio-tech/framework-replication";
import { decodePackValue, encodePackValue, viewContextWireValue } from "../../🎁️packs/🟦️.ts";
import type { AppFrameValue } from "../../🪢️wire-contracts/🟦️.ts";
import { channelLabel, throwOnFault } from "../🧯️reply-faults/🟦️.ts";
import type { ReplyLedger } from "../📮️reply-ledger/🟦️.ts";

//#region 🔖️AppChannelClient
/** 🕹️ The commands a person's gestures and the shell's configuration turn into: one send, one reply. */
export class InteractionCommands {
  private readonly ledger: ReplyLedger;
  private readonly appId: string;

  constructor(ledger: ReplyLedger, appId: string) {
    this.ledger = ledger;
    this.appId = appId;
  }

  /** 🎛️ Forwards one opaque app-specific command (already encoded by the caller's own command
   * grammar) plus the current view state; may return several frames (`Invocation` + any dirtied
   * `UiPatch`es) — routing them is the caller's job. `dispatch` ({@link PluginDispatchHintV1}) carries
   * the input ledger's causal `order` for this one command; it steers only the command's position in
   * the actor's queue, never its content. */
  async command(commandBytes: Uint8Array, viewState: unknown, dispatch?: PluginDispatchHintV1): Promise<AppFrameValue[]> {
    return this.ledger.send({
      Command: { seq: this.ledger.nextSeq(), command: Array.from(commandBytes), view_state: Array.from(encodePackValue(viewContextWireValue(viewState))) },
    }, dispatch);
  }

  /** 🔧️ Forwards one configuration command; the config value is pack-encoded here. */
  async configure(config: unknown): Promise<AppFrameValue[]> {
    return this.ledger.send({ ConfigCommand: { seq: this.ledger.nextSeq(), command: Array.from(encodePackValue(config)) } });
  }

  /** 🖱️ On-demand context menu — one `ContextMenu` reply whose `in_reply_to` matches this call's `seq`. */
  async contextMenu(request: unknown): Promise<unknown> {
    const seq = this.ledger.nextSeq();
    const frames = await this.ledger.send({
      ContextMenu: { seq, request: Array.from(encodePackValue(request)) },
    });
    const label = channelLabel("contextMenu", this.appId);
    throwOnFault(frames, label);
    const menuFrame = frames.find(
      (frame): frame is Extract<AppFrameValue, { readonly ContextMenu: { readonly in_reply_to: number; readonly items: readonly number[] } }> =>
        "ContextMenu" in frame && frame.ContextMenu.in_reply_to === seq,
    );
    if (!menuFrame) {
      throw new Error(`${label}: missing ContextMenu frame for seq ${seq}`);
    }
    return decodePackValue(new Uint8Array(menuFrame.ContextMenu.items));
  }

  /** 📥️ Force-applies remote `MutationEnvelope`s through `AppCommand::ApplyEnvelopes`. */
  async applyEnvelopes(envelopes: readonly MutationEnvelope[]): Promise<AppFrameValue[]> {
    return this.ledger.send({ ApplyEnvelopes: { seq: this.ledger.nextSeq(), envelopes } });
  }

  /** 📂️ Opens an artifact in its resolved (or explicitly named) viewer/editor surface —
   * `os.open-artifact` (contract-freeze §3 of
   * `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`). Empty
   * `pluginId`/`appId` means "resolve via the `OpeningResolver`". `role` is `0` Viewer, `1`
   * Editor — declaration order of `AppRole` (kernel `🔖️AppRouter` region). */
  async openArtifact(artifactRef: string, role: number, pluginId = "", appId = ""): Promise<AppFrameValue[]> {
    return this.ledger.send({ openArtifact: { seq: this.ledger.nextSeq(), artifact_ref: artifactRef, role, plugin_id: pluginId, app_id: appId } });
  }

  /** 📌️ Pins a viewer/editor default for one `(artifactKind, standard, subset, role)` coordinate,
   * persisted event-sourced in the OS `os.config.opening` facet — `os.set-default-viewer`/
   * `os.set-default-editor`. */
  async setDefaultApp(artifactKind: string, standard: string, subset: string, role: number, pluginId: string, appId: string): Promise<AppFrameValue[]> {
    return this.ledger.send({ setDefaultApp: { seq: this.ledger.nextSeq(), artifact_kind: artifactKind, standard, subset, role, plugin_id: pluginId, app_id: appId } });
  }

  /** 🧽️ Clears a previously pinned default, falling back to the `OpeningResolver`'s owner/router
   * order — `os.clear-default-app`. */
  async clearDefaultApp(artifactKind: string, standard: string, subset: string, role: number): Promise<AppFrameValue[]> {
    return this.ledger.send({ clearDefaultApp: { seq: this.ledger.nextSeq(), artifact_kind: artifactKind, standard, subset, role } });
  }
}
//#endregion 🔖️AppChannelClient
