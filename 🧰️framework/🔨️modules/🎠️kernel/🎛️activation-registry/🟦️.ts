/** 🎛️ The activation registry: manifest-only records seeded from a catalog, activation with LRU and memory-pressure suspension, resume from a checkpoint, capability grant intersection and the default asset fetchers.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import {
  ShardClient,
  type ShardAsset,
  type ShardBudget,
  type ShardCapabilityGrant,
  type ShardEventEnvelope
} from "../../🎭️actor/📦️packages/🟦️typescript/📮️shard-client/🟦️.ts";
import { fetchPackageAssets, type PackageAssetDeclaration } from "../📦️package-assets/🟦️.ts";
import { OwnedResidentLedger } from "../../🌱️value/📦️packages/🟦️typescript/💾️resident/🟦️.ts";
import { TurnScheduler, type Backpressure, type CoalesceKey, type Lane } from "../../🎭️actor/📦️packages/🟦️typescript/🧵️turn-scheduler/🟦️.ts";
import { ON_EXTENSION_REQUEST_ACTIVATION_PREFIX, type PluginCatalog } from "../📚️plugin-catalog/🟦️.ts";
import { activationEventEnvelope, type ActivationReason } from "../🚦️activation-reason/🟦️.ts";
import { DEFAULT_MAX_RESIDENT_ACTORS, defaultMemoryProbe, type MemoryProbe, residentActorCapFromMemory } from "../🧯️memory-pressure/🟦️.ts";
import { RUNTIME_METRICS_PUBLISH_INTERVAL_MS, type RuntimeMetricsActorRow, runtimeMetricsDue, type RuntimeMetricsSnapshot } from "../⏱️runtime-metrics/🟦️.ts";

// #region 🎠️Kernel
//#region 🐚️ActivationRegistry
export interface ActivationManifestEntry {
  readonly pluginId: string;
  readonly moduleUrl: string;
  readonly caps: readonly ShardCapabilityGrant[];
  /** 🪪️ The plugin id the program's own descriptor names, when the program id differs from it (a hub document's
   * catalog-resolved program): what an extension's activation event names as its host. */
  readonly manifestPluginId?: string;
  /** 📦️ The package assets the plugin's descriptor declares, delivered with the activation so they reach `instance-open`. */
  readonly assets?: readonly PackageAssetDeclaration[];
}

/** 🔐️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): the web mirror of
 * `semio_framework_actor::intersect_capabilities` (`🎭️actor/🦀️.rs`) — an extension must
 * never hold a capability its host plugin lacks. Matched by `ShardCapabilityGrant.id` (this file's
 * capability-name field, the web counterpart of the Rust `CapabilityGrant.capability` string); a
 * requested grant survives only when `granted` already carries one with the same `id`. An actual
 * intersection, not "grant what was asked for" — the result is always a subset of `requested`. */
export function intersectCapabilityGrants(granted: readonly ShardCapabilityGrant[], requested: readonly ShardCapabilityGrant[]): readonly ShardCapabilityGrant[] {
  const grantedIds = new Set(granted.map((grant) => grant.id));
  return requested.filter((request) => grantedIds.has(request.id));
}

/** 🪶️ GUESTSLIM (design-runtime.md §3): the typst default font set, fetched ONCE and reused across
 * every actor this registry activates — same fetch-once/reuse contract the deleted
 * `guestSlimAssetsForModule` had (`📇️registry/📜️script.ts`'s `ensureGuestSlimTypstFontsAsset` served
 * layout), just no longer tied to a single worker's lifetime. Delivered as a declared asset attached
 * to the guest's `instance-open` event (see `🟦️.ts`'s `shardWorkerSource`) rather
 * than a worker-bootstrap special case — it must be resident before the first `surface-visible`. */
function defaultGuestSlimAssetFetcher(moduleUrl: string): Promise<readonly ShardAsset[]> {
  const vendorUrl = moduleUrl.split(/[?#]/)[0]!.replace(/\/[^/]+\/[^/]+\.js$/, "/🪞️vendor/🔤️guestslim-typst-fonts.bin");
  return fetch(vendorUrl)
    .then((response) => {
      if (!response.ok) throw new Error(`GuestSlim typst fonts asset fetch failed: ${response.status} ${vendorUrl}`);
      return response.arrayBuffer();
    })
    .then((buffer): readonly ShardAsset[] => [["guestslim-typst-fonts", buffer]]);
}

/** 📦️ The default package asset fetch: the declared files from the page's own origin, verified before delivery; nothing where
 * there is no page location (a worker-less test host delivers through {@link ActivationRegistryOptions.fetchPackageAssets}). */
function defaultPackageAssetFetcher(declared: readonly PackageAssetDeclaration[]): Promise<readonly ShardAsset[]> {
  const base = (globalThis as { readonly location?: { readonly href: string } }).location?.href;
  return base === undefined ? Promise.resolve([]) : fetchPackageAssets(declared, base, url => fetch(url));
}

//#region 🧵️QueuedTurn
const DEFAULT_TURN_MAILBOX_CAPACITY = 32;

/** 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): what {@link ActivationRegistry.enqueueTurn}
 * hands the {@link TurnScheduler}. `generation` is this registry's own mirror of the native
 * `RuntimeActorId`'s bit-packed `generation` field (design-runtime.md §1: "generation makes
 * restart-after-trap addressable without id reuse") — this registry's `actorId` stays the caller-
 * minted string key it always was (see this region's own header doc), so generation lives OUT OF BAND
 * in `ActivationRegistry`'s own `actorGeneration` map instead of inside the id, and is checked at
 * dispatch time in `runQueuedTurn` so a turn queued before a restore can never run against the actor's
 * post-restore instance, even one that slips past the synchronous `cancelQueued` in `restoreActor`. */
interface QueuedTurnPayload {
  readonly events: readonly ShardEventEnvelope[];
  readonly generation: number;
}
//#endregion 🧵️QueuedTurn

interface ActivationResidentEntry {
  readonly actorId: string;
  readonly pluginId: string;
}

export interface ActivationRegistryOptions {
  readonly shardClient: ShardClient;
  readonly defaultBudget: ShardBudget;
  /** ↔️ LRU cap driving memory-pressure suspension (design-runtime.md §1 `FailurePolicy`) — activating
   * beyond this count checkpoints + suspends the least-recently-touched resident actor first. Explicit
   * override; omit to derive the cap from {@link memoryProbe} instead (see that option's own doc). */
  readonly maxResidentActors?: number;
  /** 🧮️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): injectable memory signal
   * {@link residentActorCapFromMemory} derives `maxResidentActors` from when that option itself is
   * omitted — defaults to {@link defaultMemoryProbe} (real `navigator.deviceMemory`/
   * `performance.memory`). Tests inject a fake reading; this is what keeps the derivation testable
   * without a real browser (CLAUDE.md: no direct dependency on an external implementation detail). */
  readonly memoryProbe?: MemoryProbe;
  readonly fetchAssets?: (moduleUrl: string) => Promise<readonly ShardAsset[]>;
  /** 📦️ Fetches, verifies and returns the declared package assets of one plugin; default: from the page's own origin through
   * {@link PACKAGE_ASSET_ROUTE} (nothing without a page location). */
  readonly fetchPackageAssets?: (declared: readonly PackageAssetDeclaration[]) => Promise<readonly ShardAsset[]>;
  /** ⏱️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): clock for `runtimeMetricsSnapshot`'s
   * `sampledAtMs`/`startRuntimeMetricsPublisher`'s cadence gate — injectable so both are testable
   * without real timers, same pattern `ShardClient`'s own `options.now` already uses. */
  readonly now?: () => number;
  /** 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): per-actor `TurnScheduler` mailbox
   * capacity — see `📬️mailbox.ts`'s own doc for the accept/coalesced/dropped/rejected contract
   * {@link ActivationRegistry.enqueueTurn} surfaces verbatim. */
  readonly turnMailboxCapacity?: number;
  /** 🧵️ A turn's `ShardClient.turn` result — no effect/`UiPatch` routing exists on this side of the
   * boundary yet (that belongs to the renderer's `🌉️ProgramBridge`), so the default is a documented
   * no-op rather than a silent drop of something anyone actually reads. */
  readonly onTurnResult?: (actorId: string, result: unknown) => void;
  /** 🧵️ A rejected queued turn — default logs via `console.error`, same "never let one actor's
   * failure wedge the dispatch loop" contract `TurnScheduler.onTurnError` documents on its own. */
  readonly onTurnError?: (actorId: string, error: unknown) => void;
  /** 📈️ Set `true` to auto-start `startRuntimeMetricsPublisher` in the constructor, wired to this
   * registry's own {@link ActivationRegistry.metricsBus}. Defaults to `false` — opt-in, not opt-out —
   * so every OTHER existing/future construction site across the tree (this file's own tests, the
   * `🧵️TaskManager` component's, …) keeps building a plain object with no live `setInterval`, exactly
   * as before this option existed; a real caller (ShellHost, once it mounts the task-manager window)
   * turns this on explicitly. */
  readonly autoStartMetricsPublisher?: boolean;
}

export class ActivationRegistry {
  private readonly manifests = new Map<string, ActivationManifestEntry>();
  private readonly resident = new Map<string, ActivationResidentEntry>();
  private readonly residencyOrder: string[] = [];
  private readonly checkpoints = new Map<string, Uint8Array>();
  private readonly actorPlugin = new Map<string, string>();
  /** 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): see `QueuedTurnPayload`'s own doc —
   * this registry's mirror of the native `RuntimeActorId.generation` field, out of band since this
   * class's `actorId` is a plain caller-minted string. */
  private readonly actorGeneration = new Map<string, number>();
  /** 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): the web mirror of the
   * native `ExtensionIndex` — every registered extension's `pluginId`, grouped by its parent plugin
   * id (`PluginCatalogTarget.dependsOn[0]`, guaranteed == `extends` by the same builder assertion the
   * native descriptor pipeline enforces at build time). Stores IDs, not manifest snapshots —
   * `activateExtensionsOf` looks the manifest up FRESH from `manifests` at activation time, the same
   * way `activate()` itself already resolves the parent's own manifest, so a `registerManifest` call
   * that updates an extension's entry after `registerCatalog` (e.g. once a real capability broker
   * starts populating `caps`) is honoured rather than shadowed by a stale copy. Populated by
   * `registerCatalog`; a bare `registerManifest` call (no catalog) leaves this empty, so a manually-
   * seeded manifest never cascades — matching `registerManifest`'s own pre-existing "manifest-only,
   * no side effects" contract exactly. */
  private readonly extensionsByParent = new Map<string, string[]>();
  /** 🧩️ parent actorId → the child actorIds `activateExtensionsOf` minted for it — the cascade
   * topology `suspend`/`resume`/`cancel` walk (leaves-first for suspend/cancel, parent-first for
   * resume — see each method's own doc). */
  private readonly extensionChildren = new Map<string, string[]>();
  private readonly shardClient: ShardClient;
  private readonly defaultBudget: ShardBudget;
  private readonly maxResidentActors: number;
  private readonly fetchAssets: (moduleUrl: string) => Promise<readonly ShardAsset[]>;
  private assetsPromise: Promise<readonly ShardAsset[]> | null = null;
  private readonly fetchPackageAssets: (declared: readonly PackageAssetDeclaration[]) => Promise<readonly ShardAsset[]>;
  private readonly packageAssets = new Map<string, Promise<readonly ShardAsset[]>>();
  private readonly now: () => number;
  private lastRuntimeMetricsPublishMs: number | null = null;
  private readonly turnScheduler: TurnScheduler<QueuedTurnPayload, ShardBudget>;
  private readonly onTurnResult: (actorId: string, result: unknown) => void;
  private readonly stopMetricsPublisher: () => void;
  /** 📡️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): the platform's own pub-sub
   * primitive, not a bespoke one — no topic-subscriber bus exists anywhere in this codebase yet
   * (native or web; see `🧵️TaskManager/🟦️.tsx`'s own header doc on why a real window mount is
   * still registrar-only work), so `startRuntimeMetricsPublisher`'s sink dispatches a
   * `CustomEvent(topic, { detail: snapshot })` here rather than inventing a second bus. Populated
   * (via `startRuntimeMetricsPublisher`) only when `autoStartMetricsPublisher: true` is passed; a real
   * consumer (once `ShellHost` mounts the task-manager window) subscribes with a plain
   * `registry.metricsBus.addEventListener("os.runtime.metrics", ...)`. */
  readonly metricsBus: EventTarget = new EventTarget();

  constructor(options: ActivationRegistryOptions) {
    this.shardClient = options.shardClient;
    this.defaultBudget = options.defaultBudget;
    this.maxResidentActors = options.maxResidentActors ?? residentActorCapFromMemory((options.memoryProbe ?? defaultMemoryProbe)());
    this.fetchAssets = options.fetchAssets ?? defaultGuestSlimAssetFetcher;
    this.fetchPackageAssets = options.fetchPackageAssets ?? defaultPackageAssetFetcher;
    this.now = options.now ?? (() => Date.now());
    this.onTurnResult = options.onTurnResult ?? (() => {});
    const onTurnError = options.onTurnError ?? ((actorId: string, error: unknown) => undefined);
    this.turnScheduler = new TurnScheduler<QueuedTurnPayload, ShardBudget>({
      mailboxCapacity: options.turnMailboxCapacity ?? DEFAULT_TURN_MAILBOX_CAPACITY,
      budgetFor: () => this.defaultBudget,
      runTurn: (actorId, payload, budget) => this.runQueuedTurn(actorId, payload, budget),
      onTurnError,
    });
    this.stopMetricsPublisher =
      options.autoStartMetricsPublisher === true
        ? this.startRuntimeMetricsPublisher((topic, snapshot) => this.metricsBus.dispatchEvent(new CustomEvent(topic, { detail: snapshot })))
        : () => {};
  }

  //#region 📖️Manifest
  registerManifest(entry: ActivationManifestEntry): void {
    this.manifests.set(entry.pluginId, entry);
  }

  /** 📖️ Seeds every plugin + extension row from a `PluginCatalog` (build-time descriptors) — no
   * worker/module is touched until `activate()` for one of these ids actually runs. Also indexes
   * every extension by its parent (`extensionsByParent`) so `activate()`'s cascade has something to
   * walk — descriptor-driven, zero special-casing per target. */
  registerCatalog(catalog: PluginCatalog): void {
    for (const target of catalog.plugins) this.registerManifest({ pluginId: target.pluginId, moduleUrl: catalog.moduleUrl(target.pluginId), caps: [], ...(target.assets ? { assets: target.assets } : {}) });
    for (const target of catalog.extensions) {
      this.registerManifest({ pluginId: target.pluginId, moduleUrl: catalog.extensionModuleUrl(target.pluginId), caps: [], ...(target.assets ? { assets: target.assets } : {}) });
      const parentId = target.dependsOn?.[0];
      if (!parentId) continue;
      const siblings = this.extensionsByParent.get(parentId) ?? [];
      siblings.push(target.pluginId);
      this.extensionsByParent.set(parentId, siblings);
    }
  }

  /** 🧩️ Indexes one extension program under its parent program — the web mirror of one catalog extension row, for
   * programs outside the build-time catalog (a hub document's catalog-resolved programs). Both ids are program ids whose
   * manifests `registerManifest` records; registering the same pair twice indexes it once. */
  registerExtension(parentPluginId: string, extensionPluginId: string): void {
    const siblings = this.extensionsByParent.get(parentPluginId) ?? [];
    if (!siblings.includes(extensionPluginId)) siblings.push(extensionPluginId);
    this.extensionsByParent.set(parentPluginId, siblings);
  }

  manifestFor(pluginId: string): ActivationManifestEntry | undefined {
    return this.manifests.get(pluginId);
  }
  //#endregion 📖️Manifest

  private loadAssets(moduleUrl: string): Promise<readonly ShardAsset[]> {
    this.assetsPromise ??= this.fetchAssets(moduleUrl).catch((error: unknown) => {
      this.assetsPromise = null;
      return [];
    });
    return this.assetsPromise;
  }

  /** 📦️ The declared package assets of `manifest`'s plugin, fetched and verified once per plugin. A failed fetch or a size/hash
   * mismatch delivers none of them (the plugin still activates, as it would on a host that delivers nothing) and is reported. */
  private loadPackageAssets(manifest: ActivationManifestEntry): Promise<readonly ShardAsset[]> {
    const declared = manifest.assets;
    if (declared === undefined || declared.length === 0) return Promise.resolve([]);
    let pending = this.packageAssets.get(manifest.pluginId);
    if (pending === undefined) {
      pending = this.fetchPackageAssets(declared);
      this.packageAssets.set(manifest.pluginId, pending);
    }
    return pending.catch((error: unknown) => {
      this.packageAssets.delete(manifest.pluginId);
      console.error(`[package-assets] ${manifest.pluginId} activates without its package assets: ${error instanceof Error ? error.message : String(error)}`);
      return [];
    });
  }

  private markResident(actorId: string, pluginId: string): void {
    this.resident.set(actorId, { actorId, pluginId });
    this.actorPlugin.set(actorId, pluginId);
    this.touch(actorId);
  }

  /** ⏱️ Refreshes `actorId`'s LRU position — call on every turn, not just activation, or a
   * long-resident-but-idle actor never yields to memory pressure ahead of one that's actually busy. */
  touch(actorId: string): void {
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1) this.residencyOrder.splice(index, 1);
    this.residencyOrder.push(actorId);
  }

  //#region ▶️Activate
  /** ▶️ `events::activation-event` → `Kernel::activate`. Cascades to every registered extension of
   * `pluginId` — see `activateExtensionsOf`.
   *
   * 🎬️ `reason` is not bookkeeping: the guest's own `activate` event IS this reason
   * ({@link activationEventEnvelope}), queued on `Maintenance` the moment the actor is resident so an
   * `on-artifact-kind:<kind>` install tells the guest WHICH kind woke it before any instance opens.
   * A `manual` activation has no WIT variant and therefore queues nothing — the host's own trigger is
   * not a declared activation event and must not be dressed up as one. The extension cascade below
   * activates each child `on-extension-request:<parent>`, the declared event that names exactly what
   * pulled it up. */
  async activate(pluginId: string, actorId: string, reason: ActivationReason): Promise<void> {
    const manifest = this.manifests.get(pluginId);
    if (!manifest) throw new Error(`ActivationRegistry.activate: no manifest for plugin ${pluginId}`);
    await this.evictForMemoryPressure();
    const assets = [...(await this.loadAssets(manifest.moduleUrl)), ...(await this.loadPackageAssets(manifest))];
    await this.shardClient.activate(actorId, manifest.moduleUrl, manifest.caps, this.defaultBudget, assets);
    this.markResident(actorId, pluginId);
    const activation = activationEventEnvelope(reason);
    if (activation) this.enqueueTurn(actorId, "Maintenance", [activation]);
    await this.activateExtensionsOf(pluginId, actorId);
  }

  /** 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): the cascade half of
   * `activate` — `design-unified.md` §M6's web mirror, "per-extension `shardClient.activate` with
   * parent affinity, local link records, symmetric cascade." Every registered extension whose
   * descriptor names `pluginId` as its parent (`registerCatalog`'s own `extensionsByParent` index)
   * activates alongside it under a deterministic child actorId, `caps` scoped to
   * {@link intersectCapabilityGrants} of the parent's own granted set against the extension's
   * request. Best-effort per extension (one broken extension must not fail the parent's own
   * `activate()` call) — logs and continues, matching this class's existing `evictForMemoryPressure`/
   * `loadAssets` failure posture elsewhere.
   *
   * 🕳️ Honest gap, not worked around: `ShardClient.activate` has no pinned-shard/worker-affinity
   * parameter (its own `assignShard` is a private least-loaded placement, mirroring the native
   * `ShardTable::pin`) — every extension lands on whichever shard `ShardClient` picks, not
   * necessarily the parent's own. A lease-request for a small additive `ShardClient.activate`
   * overload is open against `🎭️actor/📮️shard-client/🟦️.ts` (out of this
   * packet's `path_scope`); see this ticket's report. The cascade LINK (`extensionChildren`), and
   * therefore zero-orphan teardown via `suspend`/`cancel`, is unaffected by this gap. */
  private async activateExtensionsOf(pluginId: string, parentActorId: string): Promise<void> {
    const extensionIds = this.extensionsByParent.get(pluginId);
    if (!extensionIds || extensionIds.length === 0) return;
    const parentCaps = this.manifests.get(pluginId)?.caps ?? [];
    const children: string[] = [];
    for (const extensionId of extensionIds) {
      const manifest = this.manifests.get(extensionId);
      if (!manifest) {
        continue;
      }
      const childActorId = `${parentActorId}::${extensionId}`;
      try {
        const scopedCaps = intersectCapabilityGrants(parentCaps, manifest.caps);
        const assets = [...(await this.loadAssets(manifest.moduleUrl)), ...(await this.loadPackageAssets(manifest))];
        await this.shardClient.activate(childActorId, manifest.moduleUrl, scopedCaps, this.defaultBudget, assets);
        this.markResident(childActorId, extensionId);
        const activation = activationEventEnvelope(`${ON_EXTENSION_REQUEST_ACTIVATION_PREFIX}${this.manifests.get(pluginId)?.manifestPluginId ?? pluginId}`);
        if (activation) this.enqueueTurn(childActorId, "Maintenance", [activation]);
        children.push(childActorId);
      } catch (error) {
      }
    }
    if (children.length > 0) this.extensionChildren.set(parentActorId, children);
  }
  //#endregion ▶️Activate

  //#region 🧵️TurnDispatch
  /** 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): routes one turn through the
   * `TurnScheduler` instead of a caller reaching `ShardClient.turn` directly — lane priority,
   * latest-wins coalescing, and cancellation-on-suspend/teardown/restore all come from the scheduler,
   * not from this method. Returns the same {@link Backpressure} the scheduler itself returns; `rejected`
   * must surface to the UI as busy, same contract `TurnScheduler.enqueue` documents on its own. */
  enqueueTurn(actorId: string, lane: Lane, events: readonly ShardEventEnvelope[], options?: { readonly coalesce?: CoalesceKey }): Backpressure {
    const generation = this.actorGeneration.get(actorId) ?? 0;
    return this.turnScheduler.enqueue(actorId, { lane, coalesce: options?.coalesce, payload: { events, generation } });
  }

  /** 🧵️ The `TurnScheduler`'s `runTurn` seam. Drops (rather than dispatches) a turn whose snapshotted
   * generation no longer matches this actor's current one — it was queued against an instance that has
   * since been restored (see `QueuedTurnPayload`'s own doc and `restoreActor`), so running it now would
   * be exactly the "receives pre-restart queued work" bug this packet exists to prevent. */
  private async runQueuedTurn(actorId: string, payload: QueuedTurnPayload, budget: ShardBudget): Promise<void> {
    const currentGeneration = this.actorGeneration.get(actorId) ?? 0;
    if (payload.generation !== currentGeneration) {
      return;
    }
    this.touch(actorId);
    const result = await this.shardClient.turn(actorId, payload.events, budget);
    this.onTurnResult(actorId, result);
  }
  //#endregion 🧵️TurnDispatch

  //#region 🚑️SuspendResume
  private async evictForMemoryPressure(): Promise<void> {
    while (this.residencyOrder.length >= this.maxResidentActors) {
      await this.suspend(this.residencyOrder[0]!);
    }
  }

  /** 🚑️ Checkpoints and drops `actorId`'s worker-side residency — LRU eviction and an explicit call
   * both go through here. A no-op for an already-suspended (or never-activated) actorId.
   *
   * 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): cancels every turn still queued (not
   * yet dispatched) for `actorId` FIRST, synchronously, before the `checkpoint`/`dispose` round trip
   * even starts — a suspended actor's worker-side instance is about to go away, so anything still
   * queued must be cancelled rather than risk `TurnScheduler`'s pump dispatching it against a
   * dead/disposed instance mid-suspend. */
  async suspend(actorId: string): Promise<void> {
    if (!this.resident.has(actorId)) return;
    this.turnScheduler.cancelQueued(actorId);
    await this.suspendExtensionsOf(actorId);
    const checkpoint = await this.shardClient.checkpoint(actorId);
    this.checkpoints.set(actorId, checkpoint);
    this.shardClient.dispose(actorId);
    this.resident.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1) this.residencyOrder.splice(index, 1);
  }

  /** 🧩️ terra-extension-activation: cascade half of `suspend` — every child `activateExtensionsOf`
   * minted for `parentActorId`, suspended in turn (recursing naturally through each child's own
   * `suspend`, so a deeper cascade would still work correctly if one ever existed — today an
   * extension's `dependsOn[0]` always names a plugin, never another extension, so this is exactly
   * one level). A no-op for a parent with no tracked extensions. */
  private async suspendExtensionsOf(parentActorId: string): Promise<void> {
    const children = this.extensionChildren.get(parentActorId);
    if (!children) return;
    for (const child of children) await this.suspend(child);
  }

  /** 🚑️ Re-activates a suspended actorId and restores its last checkpoint — a plain cold `activate()`
   * (no `restore()` call) if it was never checkpointed. */
  async resume(actorId: string): Promise<void> {
    const pluginId = this.actorPlugin.get(actorId);
    if (!pluginId) throw new Error(`ActivationRegistry.resume: unknown actor ${actorId} (never activated)`);
    const manifest = this.manifests.get(pluginId);
    if (!manifest) throw new Error(`ActivationRegistry.resume: no manifest for plugin ${pluginId}`);
    await this.evictForMemoryPressure();
    const assets = [...(await this.loadAssets(manifest.moduleUrl)), ...(await this.loadPackageAssets(manifest))];
    await this.shardClient.activate(actorId, manifest.moduleUrl, manifest.caps, this.defaultBudget, assets);
    const checkpoint = this.checkpoints.get(actorId);
    if (checkpoint) await this.shardClient.restore(actorId, checkpoint);
    this.markResident(actorId, pluginId);
    await this.resumeExtensionsOf(actorId);
  }

  /** 🧩️ terra-extension-activation: cascade half of `resume` — every tracked child that was
   * suspended (has a checkpoint, not currently resident) resumes after its parent. A child never
   * suspended at all (e.g. it failed to activate in the first place, per `activateExtensionsOf`'s
   * best-effort policy) has no checkpoint and is correctly skipped rather than cold-activated here —
   * `activate()` is the only entry point that MINTS a fresh extension cascade. */
  private async resumeExtensionsOf(parentActorId: string): Promise<void> {
    const children = this.extensionChildren.get(parentActorId);
    if (!children) return;
    for (const child of children) {
      if (this.checkpoints.has(child) && !this.resident.has(child)) await this.resume(child);
    }
  }

  /** 🚑️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): the web mirror of native's
   * "Trapped → drop + re-instantiate (generation++) + restore last checkpoint" (design-runtime.md §1
   * `FailurePolicy`) — called from a `ShardClient`'s own `onShardLost` for every actorId that WAS
   * pinned to the shard that just died (see {@link handleShardLost}). Bumps this actor's generation
   * BEFORE cancelling its queue: the bump is what protects a turn that gets enqueued during the
   * `resume()` await below (since `enqueueTurn` always reads the CURRENT generation at call time), and
   * the immediately-following `cancelQueued` is belt-and-suspenders for anything already queued at the
   * moment loss is detected. A no-op for an actorId this registry never activated or has already fully
   * `cancel()`ed — the shard's own bookkeeping and this registry's can disagree briefly across a
   * teardown race, and only actors this registry still recognizes are ours to restore. */
  private async restoreActor(actorId: string): Promise<void> {
    const pluginId = this.actorPlugin.get(actorId);
    if (!pluginId) return;
    this.actorGeneration.set(actorId, (this.actorGeneration.get(actorId) ?? 0) + 1);
    this.turnScheduler.cancelQueued(actorId);
    this.resident.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1) this.residencyOrder.splice(index, 1);
    await this.resume(actorId);
  }

  /** 🚑️ Restores every actorId the caller reports as lost together — see {@link restoreActor}. Runs
   * every restoration concurrently (independent actors, no ordering dependency between them) and never
   * lets one actor's restore failure block another's, same "one actor's failure never wedges the rest"
   * reasoning `TurnScheduler.onTurnError`'s own doc gives. */
  async restoreActors(actorIds: readonly string[]): Promise<void> {
    await Promise.all(actorIds.map((actorId) => this.restoreActor(actorId).catch((error: unknown) => undefined)));
  }

  /** 🚑️ Bound convenience handler for `ShardClientOptions.onShardLost` — pass this directly, e.g.
   * `new ShardClient({ …, onShardLost: registry.handleShardLost })`. `ShardClient`'s own callback
   * contract is synchronous `void` (the shard transport only reports loss; restoration is the
   * kernel-side registry's job, per that option's own doc comment), so the restore promise is
   * deliberately fire-and-forget here — failures are still visible via `restoreActors`' own
   * `console.error`, nothing new swallows them. */
  readonly handleShardLost = (_shardIndex: number, actorIds: readonly string[]): void => {
    void this.restoreActors(actorIds);
  };

  /** 🛑️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1, wiring the task manager's "cancel" action to a
   * REAL dispatch path): the web mirror of `Payload::Cancel`'s now-landed native semantics
   * (`🧵️shard/🦀️.rs`'s `ShardLoop::pump`, K1 — "cancels the actor's running jobs +
   * unregisters the instance"). Unlike `suspend()`, this is NOT resumable: no checkpoint is taken,
   * and every bookkeeping entry (including `actorPlugin`) is dropped, so a later `resume(actorId)`
   * correctly throws "unknown actor" rather than silently reviving it. A no-op for an actorId this
   * registry has never heard of.
   *
   * 🚧️ Honest gap, NOT fixed here (would need a file outside this packet's `path_scope`): this
   * class has no per-actor job-id bookkeeping (`ShardClient.cancelJob` needs a specific job id,
   * tracked by whoever calls `startJob`/`stepJob` — not `ActivationRegistry`/`ShardClient`), so
   * "cancels the actor's running jobs" is only reachable here via `dispose()` tearing down the whole
   * worker-side instance (which implicitly ends any in-flight job), not via an explicit per-job
   * cancel message. See `📓️terra-T1-report.md` `## honest gaps`. */
  cancel(actorId: string): void {
    if (!this.actorPlugin.has(actorId)) return;
    const children = this.extensionChildren.get(actorId);
    if (children) {
      for (const child of children) this.cancel(child);
      this.extensionChildren.delete(actorId);
    }
    this.turnScheduler.teardownActor(actorId);
    this.actorGeneration.delete(actorId);
    this.shardClient.dispose(actorId);
    this.resident.delete(actorId);
    this.checkpoints.delete(actorId);
    this.actorPlugin.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1) this.residencyOrder.splice(index, 1);
  }

  isResident(actorId: string): boolean {
    return this.resident.has(actorId);
  }

  /** ⏏️ Stops the constructor's own auto-started metrics-publish loop (a no-op if
   * `autoStartMetricsPublisher: false` was passed) — call once on full teardown, mirroring
   * `ShardClient.disposeAll`'s own real-`setInterval` cleanup. Does not touch `shardClient` itself
   * (this registry doesn't own its lifecycle — it was handed one already built). */
  dispose(): void {
    this.stopMetricsPublisher();
  }
  //#endregion 🚑️SuspendResume

  //#region 📈️RuntimeMetrics
  /** 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): one row per actor this registry has ever
   * activated (`actorPlugin` — populated on `markResident`, never cleared on `suspend`, so a
   * suspended-but-not-forgotten actor still gets a row with `resident: false`). Field-compatible
   * with the Rust `ActorMetricsSample` the native host publishes, minus the fields only a live
   * `Kernel`/guest turn can produce (`turns`/`traps`/`wallUsP95`/…) — this registry never held a
   * `Kernel` (it delegates straight to `ShardClient`, see this file's own header doc), so those are
   * an honest gap here, not a silent zero-fill. */
  runtimeMetricsActorRows(): readonly RuntimeMetricsActorRow[] {
    return [...this.actorPlugin.entries()].map(([actorId, pluginId]) => ({ actorId, pluginId, resident: this.resident.has(actorId), shard: this.shardClient.shardIndexFor(actorId) ?? null }));
  }

  /** 📈️ The `os.runtime.metrics` payload this side of the boundary can build: per-actor residency
   * rows plus `ShardClient.shardMetricsSamples` (see that method's own doc comment). `sampledAtMs`
   * defaults to this registry's own injected clock, same convention as `runtime_metrics_snapshot`'s
   * `sampled_at_ms` on the Rust side. */
  runtimeMetricsSnapshot(sampledAtMs: number = this.now()): RuntimeMetricsSnapshot {
    return { actors: this.runtimeMetricsActorRows(), shards: this.shardClient.shardMetricsSamples(sampledAtMs), sampledAtMs };
  }

  /** ⏱️ Starts a 2Hz (`RUNTIME_METRICS_PUBLISH_INTERVAL_MS`) publish loop calling `sink(topic,
   * snapshot)` — `topic` is always `"os.runtime.metrics"`, matching the Rust side's bus topic name.
   * Returns a `stop()` disposer. Real `setInterval`, not the injected `now` (browser timer loops are
   * not something the pure-crate clock-injection discipline applies to — only `🎭️actor` itself must
   * never read a clock); `runtimeMetricsDue` below is what stays unit-testable without real timers.
   *
   * ✅️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): this now HAS a real caller and sink —
   * pass `autoStartMetricsPublisher: true` and the constructor starts this itself, with a sink that
   * dispatches on `this.metricsBus`; see that field's own doc. Calling this method directly (with a
   * different sink) is still supported for a caller that wants its own delivery instead of the bus.
   *
   * 🚧️ Honest remaining gap: no real CONSUMER subscribes to `metricsBus` yet anywhere in this codebase
   * (native or web) — mounting the task-manager window that would (`🧵️TaskManager/🟦️.tsx`'s own
   * header doc) is registrar-only, lease-requested work outside this packet's `path_scope`. */
  startRuntimeMetricsPublisher(sink: (topic: string, snapshot: RuntimeMetricsSnapshot) => void): () => void {
    const interval = setInterval(() => {
      const nowMs = this.now();
      if (!runtimeMetricsDue(this.lastRuntimeMetricsPublishMs, nowMs)) return;
      this.lastRuntimeMetricsPublishMs = nowMs;
      sink("os.runtime.metrics", this.runtimeMetricsSnapshot(nowMs));
    }, RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
    return () => clearInterval(interval);
  }
  //#endregion 📈️RuntimeMetrics
}
//#endregion 🐚️ActivationRegistry
// #endregion 🎠️Kernel
