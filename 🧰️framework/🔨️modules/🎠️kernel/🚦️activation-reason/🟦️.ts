/** 🚦️ Why an actor is activated: the activation reason, the WIT activation event it justifies and the reason an app id implies.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { parseSurfaceAppId } from "../../🛂️manifest/🧬️schema/🟦️.ts";
import { type ShardEventEnvelope } from "@semio-tech/framework-actor/shard-client";
import { ON_ARTIFACT_KIND_ACTIVATION_PREFIX } from "../📚️plugin-catalog/🟦️.ts";

// #region 🎠️Kernel
//#region 🗂️PluginCatalog
/** 🎬️ The {@link ActivationReason} an app id justifies: a role-suffixed surface app id names the
 * artifact kind the actor is being activated for, so it activates `on-artifact-kind:<kind>` with the
 * kind the id itself carries; anything else (a bare landing/host app id, an extension's request
 * actor) stays `manual`. */
export function activationReasonForAppId(appId: string): ActivationReason {
  try {
    return `${ON_ARTIFACT_KIND_ACTIVATION_PREFIX}${parseSurfaceAppId(appId).dialect.artifactKind}`;
  } catch {
    return "manual";
  }
}
//#endregion 🗂️PluginCatalog

// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H2): `LeasePool`/`createLeasePool` RELOCATE unchanged
// to `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` under `//#region 🪶️LeasePool` — its non-plugin
// consumers (the renderer's engine-session cache and others; see `📓️luna-consumers-audit.md`) keep
// working from there. `PluginModuleLease`/`acquirePluginModule`/`evictPluginModule` and the trailing
// `loadPluginModuleUncached`/`pluginHandleForBridge` are DELETED outright (no relocation — they were
// plugin-specific, per `📌️important.md`'s "must not exist" list), replaced by `ActivationRegistry`.

//#region 🐚️ActivationRegistry
/**
 * 🐚️ Replaces the deleted `LeasePool`/`PluginModuleLease`/`acquirePluginModule` trio
 * (design-runtime.md §3). Manifest-only records seeded from a `PluginCatalog` (build-time descriptors
 * — no worker/module is touched until an actor actually activates); `events::activation-event` maps
 * onto `activate()`, which calls `ShardClient.activate` (design's `Kernel::activate`, on the web
 * transport); LRU + memory-pressure suspension checkpoints a resident actor and drops it before a new
 * activation would exceed `maxResidentActors`; `resume()` re-activates and restores that checkpoint.
 *
 * `actorId` is a caller-minted string key here, not the real bit-packed `RuntimeActorId` (that
 * encoding lives in the pure `semio-framework-actor` crate — packet A1); this registry only needs a
 * stable key for shard routing and residency bookkeeping, same as `ShardClient` itself.
 */
/** 🧠️ Why an actor is being activated, in `📓️design-abi.md` §2's own dash-separated declaration
 * grammar — the SAME spelling a plugin's `activationEvents` row carries, so a declared
 * `on-artifact-kind:s.cad.cad` row and the reason an open of that kind activates its owner with are
 * one string, not two vocabularies. The payload-bearing forms mirror the WIT `activation-event`
 * variant's single-string payload exactly; `manual` is the host's own trigger and has no WIT variant,
 * so it reaches no guest (see {@link activationEventEnvelope}). */
export type ActivationReason =
  | `on-command:${string}`
  | `on-view-visible:${string}`
  | `on-file-type:${string}`
  | `on-artifact-kind:${string}`
  | `on-extension-request:${string}`
  | "on-startup-finished"
  | "manual";

/** ✉️ The WIT `event::activate` an {@link ActivationReason} justifies, in the `{kind, payload}`
 * envelope shape `🟨️shard-worker.js` lifts into the guest's own `{tag, val}` variant — this is the
 * whole bridge from a declared activation event to what the guest's `activate` actually receives.
 * `manual` and an unparseable reason answer `undefined`: the WIT variant has no case for them, so the
 * honest delivery is no event at all rather than an invented one. `instance` is `0` at activation
 * time by construction — no instance is open yet, and the guest routes `activate` per actor, not per
 * instance. */
export function activationEventEnvelope(reason: ActivationReason, instance = 0): ShardEventEnvelope | undefined {
  if (reason === "on-startup-finished") return { kind: "activate", payload: { instance, reason: { tag: reason } } };
  const separator = reason.indexOf(":");
  if (separator <= 0) return undefined;
  return { kind: "activate", payload: { instance, reason: { tag: reason.slice(0, separator), val: reason.slice(separator + 1) } } };
}
//#endregion 🐚️ActivationRegistry
// #endregion 🎠️Kernel
