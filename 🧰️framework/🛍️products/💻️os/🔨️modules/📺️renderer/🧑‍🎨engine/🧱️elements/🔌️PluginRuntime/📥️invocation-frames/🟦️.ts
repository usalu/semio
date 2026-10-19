import { TYPED_OPERATION_DIAGNOSTIC } from "../📬️typed-result/🟦️.ts";
import type { Diagnostic as OwnedOperationDiagnostic } from "../../../../../../../../🔨️modules/⚠️diagnostic/🎛️controlled/🟦️.ts";
/** 📥️ Invocation reassembly: command inspection, `performInvocation`, `invocationFromFrames`, context-menu calls and the guest-ingress generation counter.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  type ContextMenuItemSpec,
  type Effect,
  type InvocationResponse,
  type PluginContextMenuRequest,
  SemioFaultError
} from "@semio-tech/framework";
// #endregion 🔌️Adapters
import {
  decodePackValue,
  encodePackValue
} from "@semio-tech/framework/packs";
import {
  AppChannelClient,
  decodeAppCommand,
  decodeAppFrame,
  decodeFaultFromWire,
  faultDisplayMessage
} from "@semio-tech/framework/app-channel";
import {
  type AppFrameValue
} from "@semio-tech/framework/wire-contracts";
import {
  decodeInvocationResultPacks,
  decodePackWire,
  viewContextWireValue
} from "@semio-tech/framework/packs";
import { type PluginDispatchHintV1 } from "@semio-tech/framework";
import { hopTrace } from "@semio-tech/framework/trace";
import type { ViewModel } from "../🧷️handle-contract/🟦️.ts";
import { leftoverShellInvocationFrames, type WireVariant } from "../🪝️wire-codec/🟦️.ts";
import { wireEffectToFriendly } from "../🪟️retained-ui-patch/🟦️.ts";
import { TYPED_OPERATION_PENDING_OUTPUT, TYPED_OPERATION_TERMINAL_OUTPUT, TYPED_OPERATION_TERMINAL_SEEN } from "../📬️typed-result/🟦️.ts";
import { commandIngressLaneForActionV1 } from "../🔁️turn-settle/🟦️.ts";
import { pendingTurnEffects } from "../🏁️operation-completion/🟦️.ts";
import { admitCrossingViewContext } from "../🖼️surface-refresh/🟦️.ts";

//#region 🔖️plugin-runtime
//#region 🔖️ActorAdapter
//#region 🔖️GenericThunkQueue
export function inspectEncodedAppCommand(events: readonly Uint8Array[]): { actionId: string | null; seq: number | null; lane: "Interactive" | "Background" } {
  try {
    const command = decodeAppCommand(events[0]!);
    const seq = "Command" in command ? command.Command.seq : "ConfigCommand" in command ? command.ConfigCommand.seq : null;
    if (!("Command" in command)) return { actionId: null, seq, lane: "Interactive" };
    const invocation = decodePackValue(new Uint8Array(command.Command.command)) as { readonly address?: { readonly actionId?: unknown } } | null;
    const actionId = typeof invocation?.address?.actionId === "string" ? invocation.address.actionId : null;
    return { actionId, seq, lane: actionId === null ? "Interactive" : commandIngressLaneForActionV1(actionId) };
  } catch {
    return { actionId: null, seq: null, lane: "Interactive" };
  }
}

export function encodedFrameReplySequence(bytes: Uint8Array): number | null {
  try {
    const value = Object.values(decodeAppFrame(bytes))[0] as { readonly in_reply_to?: unknown } | undefined;
    return value && typeof value.in_reply_to === "number" ? value.in_reply_to : null;
  } catch {
    return null;
  }
}
//#endregion 🔖️GenericThunkQueue
//#endregion 🔖️ActorAdapter

//#region 🔖️ChannelAdapter
/** 📋️ Passes a job completion's leftover effects through unchanged, tracing the `Invocation` frames it
 * carries. A `clipboard-write` is NOT implied by a `send-message`: a reserved job publishes its result
 * frame on `send-message` whatever it did, so a "clipboard-write missing" warning keyed on that tag
 * fired on EVERY reserved-job completion (measured 2026-09-12 on every `interactionSelect`/
 * `interactionHover` of the generation3d preview — ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️selection-dedupe-2026-09-12.md`). Nothing in the leftover list declares that a clipboard write was
 * expected, so the check had no predicate to test and is gone rather than silenced. */
export function promoteShellSendMessages(instanceId: number, leftover: readonly WireVariant[], _outFrames: Uint8Array[]): WireVariant[] {
  return [...leftover];
}

/** 🎯️ Shared by `handleAction`/`handleCommand`: encodes `envelope` + `viewState`, sends one
 * `AppCommand::Command` frame, and reassembles the `Invocation` frame plus this SAME turn's leftover
 * `TurnResult.effects` (`pendingTurnEffects`, H1-react — replaces the deleted `AppFrame::Effects`/
 * `Events` frames) back into the `InvocationResponse` shape the rest of this file already consumes.
 * `events` has no wire counterpart in this wave (an honest gap `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s
 * native `invocation_from_frames` already flags identically). */
/** 🚪️ Per-instance count of GUEST-MUTATING ingress this renderer has submitted — a dispatch, or the
 * publication of an extension completion's answer. It is deliberately NOT bumped by a `refresh-ui`
 * turn: a refresh asks the guest to re-render what it already holds, so one refresh pass cannot
 * invalidate another refresh pass's answer.
 *
 * 🤝️ This is the ONE thing the shell's ui-refresh lane cannot know on its own
 * (`📓️react-hop-latency-2026-09-14.md` §4). A converging `flowEvalTick` arms one chain per flow
 * window, so two completions demand a refresh per hop; the second arrives a few milliseconds after
 * the first pass has already submitted its own guest turn, and BOTH are host-side callbacks of turns
 * that had already settled — nothing crossed into the guest in between. Read against the generation
 * the in-flight pass submitted under, that second pass is provably already answered, and a hop pays
 * two serialized guest turns instead of three. See `uiRefreshAlreadyAnsweredV1`.
 */
export const guestIngressGenerationByInstance = new Map<number, number>();

export function guestIngressGenerationV1(instanceId: number): number {
  return guestIngressGenerationByInstance.get(instanceId) ?? 0;
}

/** 🚪️ Records one guest-mutating ingress. Called BEFORE the crossing, never after: a pass that starts
 * while a dispatch is in flight must see the bump, or it would hold an answer taken before it. */
export function noteGuestIngressV1(instanceId: number): void {
  guestIngressGenerationByInstance.set(instanceId, guestIngressGenerationV1(instanceId) + 1);
}

export async function performInvocation(client: AppChannelClient, instanceId: number, invocation: unknown, invocationKind: "action" | "command", viewState: unknown, dispatch?: PluginDispatchHintV1): Promise<InvocationResponse> {
  assertAddressedInvocation(invocation, invocationKind, instanceId);
  const invocationRecord = invocation as { readonly address?: { readonly actionId?: unknown; readonly commandId?: unknown }; readonly arguments?: Record<string, unknown> } | null;
  const address = invocationRecord?.address;
  const actionId = address?.actionId ?? address?.commandId ?? null;
  if (actionId === "importFixture") {
    const args = invocationRecord?.arguments;
    const payload = args?.payload;
  }
  noteGuestIngressV1(instanceId);
  const hopDetail = { instanceId, actionId: String(actionId), invocationKind };
  const closeInvoke = hopTrace.open("invoke", hopDetail);
  let response: InvocationResponse;
  let frames: readonly AppFrameValue[];
  try {
    const wire = hopTrace.time("encode", hopDetail, () => ({ command: encodePackValue(invocation), view: admitCrossingViewContext(`${invocationKind} ${String(actionId)}`, viewState) }));
    frames = await hopTrace.timeAsync("channel", hopDetail, () => client.command(wire.command, wire.view, dispatch));
    const leftover = pendingTurnEffects.get(instanceId) ?? [];
    pendingTurnEffects.delete(instanceId);
    response = hopTrace.time("decode", { ...hopDetail, frames: frames.length }, () => invocationFromFrames(frames, leftover, invocationKind));
  } catch (error) {
    closeInvoke({ failed: true });
    throw error;
  }
  closeInvoke({ frames: frames.length, effects: (response.requestedEffects ?? []).length });
  return response;
}

/** 🚫️ Refuses an unaddressed invocation at the renderer edge, naming the window kind and instance it
 * was aimed at. Without this the empty id travels to the plugin, which can only answer with the
 * anonymous `window kind <kind> does not own action ""` — a `plugin.internal` fault that names neither
 * the renderer frame that produced it nor the instance it belongs to
 * (`📓️runtime-verification-2026-09-09.md` boot #3, two unhandled rejections per boot). */
export function assertAddressedInvocation(invocation: unknown, invocationKind: "action" | "command", instanceId: number): void {
  const address = (invocation as { readonly address?: Record<string, unknown> } | null)?.address;
  const id = invocationKind === "action" ? address?.actionId : address?.commandId;
  if (typeof id === "string" && id.length > 0) return;
  const windowKindId = typeof address?.windowKindId === "string" ? address.windowKindId : "<unknown>";
  throw new SemioFaultError({
    origin: "renderer",
    code: "renderer.invocation.unaddressed",
    severity: "error",
    message: `${invocationKind} invocation on window kind ${windowKindId} carries no ${invocationKind}Id`,
    scope: {
      pluginId: typeof address?.pluginId === "string" ? address.pluginId : undefined,
      appId: typeof address?.appId === "string" ? address.appId : undefined,
      instanceId: String(instanceId),
    },
    retryable: false,
  });
}

/** 📬️ Decodes the shared invocation publication without losing its host effects or typed fault. */
export function invocationFromFrames(frames: readonly AppFrameValue[], leftover: readonly WireVariant[], invocationKind: string): InvocationResponse {
  let output: unknown = null;
  let diagnostics: InvocationResponse["diagnostics"] = [];
  let mutations: InvocationResponse["mutations"] = [];
  let inverseGroup: InvocationResponse["inverseGroup"] = { invocationId: "", mutations: [], inverseMutations: [] };
  let uiScope: InvocationResponse["uiScope"];
  let historyPatch: InvocationResponse["historyPatch"];
  const applyInvocationFrame = (frame: Extract<AppFrameValue, { readonly Invocation: unknown }>): void => {
    if (frame.Invocation.output.length) output = decodePackValue(new Uint8Array(frame.Invocation.output));
    if (frame.Invocation.diagnostics.length) {
      const decodedDiagnostics = decodePackValue(new Uint8Array(frame.Invocation.diagnostics));
      diagnostics = Array.isArray(decodedDiagnostics) ? (decodedDiagnostics as InvocationResponse["diagnostics"]) : [];
    }
    if (frame.Invocation.ui_scope.length) uiScope = decodePackValue(new Uint8Array(frame.Invocation.ui_scope)) as InvocationResponse["uiScope"];
    if (frame.Invocation.history_patch.length) {
      const decodedHistoryPatch = decodePackWire(new Uint8Array(frame.Invocation.history_patch), "$.historyPatch");
      historyPatch = decodedHistoryPatch && typeof decodedHistoryPatch === "object" ? (decodedHistoryPatch as InvocationResponse["historyPatch"]) : undefined;
    }
    // 🧾️ Per-field, like every carrier above it: a reserved tool job's completion frame carries no
    // mutations, and folding it wholesale blanked the admission's own (`wgpuInvocationFromFrames`,
    // `🐚️plugin-bridge.ts`, states the same rule for the other target).
    if (frame.Invocation.mutations.length || frame.Invocation.inverse_group.length) ({ mutations, inverseGroup } = decodeInvocationResultPacks(frame.Invocation));
  };
  for (const frame of frames) {
    if ("Invocation" in frame) {
      applyInvocationFrame(frame);
    } else if ("Error" in frame) {
      const fault = decodeFaultFromWire(frame.Error.fault, decodePackValue);
      if (fault) throw new SemioFaultError(fault);
      throw new Error(`${invocationKind} failed: ${faultDisplayMessage(frame.Error.fault, decodePackValue)}`);
    }
  }
  for (const frame of leftoverShellInvocationFrames(leftover)) {
    if ("Invocation" in frame) applyInvocationFrame(frame);
  }
  if (leftover.some((effect) => effect.tag === TYPED_OPERATION_PENDING_OUTPUT)) throw new Error("directory projection receipt was exposed before typed-operation terminal publication");
  const terminalOutputs = leftover.filter((effect) => effect.tag === TYPED_OPERATION_TERMINAL_OUTPUT);
  if (terminalOutputs.length > 1) throw new Error("typed-operation returned more than one terminal output");
  if (terminalOutputs.length === 1) output = terminalOutputs[0]!.val;
  diagnostics = [...diagnostics, ...leftover.filter(effect => effect.tag === TYPED_OPERATION_DIAGNOSTIC).map(effect => effect.val as OwnedOperationDiagnostic)];
  const requestedEffects = leftover.filter((effect) => effect.tag !== TYPED_OPERATION_DIAGNOSTIC && effect.tag !== TYPED_OPERATION_TERMINAL_OUTPUT && effect.tag !== TYPED_OPERATION_TERMINAL_SEEN).map(wireEffectToFriendly).filter((effect): effect is Effect => effect !== null);
  return {
    output,
    mutations,
    inverseGroup,
    diagnostics,
    requestedEffects,
    events: [],
    uiScope,
    historyPatch,
  };
}

export async function performContextMenu(client: Pick<AppChannelClient, "contextMenu">, request: PluginContextMenuRequest, viewState: ViewModel): Promise<readonly ContextMenuItemSpec[]> {
  const items = await client.contextMenu({ ...request, viewState: viewContextWireValue(admitCrossingViewContext("context-menu", viewState)) });
  return Array.isArray(items) ? (items as ContextMenuItemSpec[]) : [];
}
//#endregion 🔖️ChannelAdapter
//#endregion 🔖️plugin-runtime
