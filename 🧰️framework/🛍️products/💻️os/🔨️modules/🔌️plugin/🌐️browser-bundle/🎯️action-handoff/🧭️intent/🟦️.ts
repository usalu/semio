/** 🧭️ Complete retained-surface intents for the exact authenticated document actor. */
import type { UiIntent } from "@semio-tech/framework";
import { encodePackValue, packUInt, type PackValue } from "../../../../../🟦️.ts";
import { parseBrowserActorActionRequestV1, type BrowserActorActionOwnerV1, type BrowserActorActionRequestV1 } from "../🟦️.ts";

function unsigned(value: number, path: string, maximum = Number.MAX_SAFE_INTEGER): PackValue {
  if (!Number.isSafeInteger(value) || value < 0 || value > maximum) throw new Error(`browser-actor-intent: invalid ${path}`);
  return packUInt(BigInt(value));
}

/** 🎯️ Preserves every intent field while binding the renderer surface to its native instance. */
export function createBrowserActorUiIntentRequestV1(owner: BrowserActorActionOwnerV1, windowKindId: string, intent: UiIntent): BrowserActorActionRequestV1 {
  if (windowKindId.length === 0 || intent.surface !== windowKindId || intent.revision !== owner.surfaceRevision) throw new Error("browser-actor-intent: stale surface");
  if (typeof intent.seq !== "bigint" || intent.seq < 0n || intent.seq > 0xffffffffffffffffn) throw new Error("browser-actor-intent: invalid sequence");
  const payload = encodePackValue({
    ...intent,
    surface: `${owner.instanceId}:${windowKindId}`,
    revision: unsigned(intent.revision, "revision"),
    node: unsigned(intent.node, "node"),
    action: { ...intent.action, version: unsigned(intent.action.version, "action version", 0xffff) },
    seq: packUInt(intent.seq),
  } as PackValue);
  return parseBrowserActorActionRequestV1({
    kind: "browser-actor-action",
    scope: owner.scope,
    verifiedSurfaceId: owner.verifiedSurfaceId,
    appChannelVersion: owner.appChannelVersion,
    activationGeneration: owner.activationGeneration,
    instanceId: owner.instanceId,
    surfaceRevision: owner.surfaceRevision,
    actionSequence: owner.actionSequence,
    payload: { kind: "ui-intent", bytes: Array.from(payload) },
  });
}
