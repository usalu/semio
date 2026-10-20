/** 🧮️ The invocation response a guest answers a command with, its empty value and its parser.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { type AppEvent, type KernelMutation, type UndoGroup } from "../🧬️mutation-vocabulary/🟦️.ts";
import { type Diagnostic } from "../🚨️fault/🟦️.ts";
import { type Effect } from "@semio-tech/framework-effect";
import { type UiDirtyScope } from "../🎯️ui-dirty-scope/🟦️.ts";
import { type HistoryPatch } from "../🕰️history/🟦️.ts";

// #region 🎠️Kernel
//#region InvocationResponse
/**
 * 📤️ Typed result of a plugin `handle-action`/`handle-command` call — mirrors the Rust
 * `InvocationResult`. Replaces the legacy `string[]` JSON-patch shape: operations are now typed
 * `KernelMutation`s with true inverses, and the shell applies `requestedEffects` through
 * `applyHostEffects` (WS-E).
 */
export type InvocationResponse = {
  readonly output: unknown;
  readonly mutations: readonly KernelMutation[];
  readonly inverseGroup: UndoGroup;
  readonly diagnostics?: readonly Diagnostic[];
  readonly requestedEffects?: readonly Effect[];
  readonly events?: readonly AppEvent[];
  readonly uiScope?: UiDirtyScope;
  readonly historyPatch?: HistoryPatch;
};

// 🐢️ `uiScope` deliberately left unset here (not `{kind: "none"}`) — `resolveUiDirtyScope` treats a
// missing scope as `full`, the safe default for the rare failure paths that return this constant
// (unparseable response, stub module missing `handleAction`/`handleCommand`).
const EMPTY_INVOCATION_RESPONSE: InvocationResponse = {
  output: null,
  mutations: [],
  inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] },
};

/** 📥️ Parses a raw program `handle-action`/`handle-command` response string into a typed {@link InvocationResponse}. */
export function parseInvocationResponse(raw: string): InvocationResponse {
  try {
    const parsed = JSON.parse(raw) as Partial<InvocationResponse> | null;
    if (parsed && typeof parsed === "object" && Array.isArray(parsed.mutations)) {
      return parsed as InvocationResponse;
    }
  } catch {}
  return EMPTY_INVOCATION_RESPONSE;
}
//#endregion InvocationResponse
// #endregion 🎠️Kernel
