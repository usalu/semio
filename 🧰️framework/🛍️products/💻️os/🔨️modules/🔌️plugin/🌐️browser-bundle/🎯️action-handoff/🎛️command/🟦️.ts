/** 🎛️ Canonical catalog action and command pages for one authenticated document actor. */
import type { ActionInvocation, CommandInvocation, PluginViewState } from "@semio-tech/framework";
import { encodeAppCommand, encodePackValue, type PackValue } from "../../../../../🟦️.ts";
import { parseBrowserActorActionRequestV1, type BrowserActorActionOwnerV1, type BrowserActorActionRequestV1 } from "../🟦️.ts";

/** 📦️ Preserves the complete invocation and view state inside one exact AppCommand::Command frame. */
export function createBrowserActorAppCommandRequestV1(owner: BrowserActorActionOwnerV1, invocation: ActionInvocation | CommandInvocation, viewState: PluginViewState): BrowserActorActionRequestV1 {
  const bytes = encodeAppCommand({
    Command: {
      seq: owner.actionSequence,
      command: Array.from(encodePackValue(invocation as PackValue)),
      view_state: Array.from(encodePackValue(viewState as PackValue)),
    },
  });
  return parseBrowserActorActionRequestV1({
    kind: "browser-actor-action",
    scope: owner.scope,
    verifiedSurfaceId: owner.verifiedSurfaceId,
    appChannelVersion: owner.appChannelVersion,
    activationGeneration: owner.activationGeneration,
    instanceId: owner.instanceId,
    surfaceRevision: owner.surfaceRevision,
    actionSequence: owner.actionSequence,
    payload: { kind: "app-command", bytes: Array.from(bytes) },
  });
}
